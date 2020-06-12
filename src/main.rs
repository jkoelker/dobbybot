//

mod commands;
mod hooks;
mod iex;
mod utils;

use async_trait::async_trait;
use log::{error, info};
use serenity::{
    client::{bridge::gateway::ShardManager, Client, Context, EventHandler},
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult,
        HelpOptions, StandardFramework,
    },
    http::Http,
    model::{
        channel::Message,
        id::{ChannelId, UserId},
    },
    model::{event::ResumedEvent, gateway::Ready},
    utils::TypeMapKey,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::Mutex;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct IEXClient;

impl TypeMapKey for IEXClient {
    type Value = iex::Client;
}

struct ChannelLastStocks;

impl TypeMapKey for ChannelLastStocks {
    type Value = HashMap<ChannelId, HashSet<String>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    /*
    async fn message(&self, _: Context, msg: serenity::model::channel::Message) {
        error!("Got {}", msg.content);
    }
    */
}

#[help]
#[strikethrough_commands_tip_in_guild(" ")]
#[strikethrough_commands_tip_in_dm(" ")]
#[individual_command_tip = " "]
#[max_levenshtein_distance(3)]
#[embed_success_colour(DARK_TEAL)]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(
        context,
        msg,
        args,
        help_options,
        groups,
        owners,
    )
    .await
}

macro_rules! env_require {
    ($var:expr) => {
        std::env::var($var)
            .expect(&std::format!("Expected {} in the environment", $var))
    };
}

macro_rules! env_default {
    ($var:expr, $default:expr) => {
        match std::env::var($var) {
            Ok(v) => v,
            Err(_) => $default.to_string(),
        }
    };
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let iex_token = env_require!("IEX_TOKEN");
    let discord_token = env_require!("DISCORD_TOKEN");
    let symbol_cache_ttl = env_default!("SYMBOL_CACHE_TTL", "604800")
        .parse::<u64>()
        .unwrap();
    iex::cache::set_symbols_lifetime(symbol_cache_ttl).await;

    let http = Http::new_with_token(&discord_token);
    let iex_client = iex::Client::new(iex_token);

    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            owners
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .help(&MY_HELP)
        .after(hooks::after)
        .unrecognised_command(hooks::unrecognised_command)
        .normal_message(hooks::normal_message)
        .on_dispatch_error(hooks::dispatch_error);
    let framework = commands::configure_framework(framework);

    let mut client = Client::new(&discord_token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(
            &client.shard_manager,
        ));
        data.insert::<IEXClient>(iex_client);
        data.insert::<ChannelLastStocks>(HashMap::new());
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
