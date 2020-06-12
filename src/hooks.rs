//

use crate::utils;
use crate::ChannelLastStocks;

use itertools::Itertools;
use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandError, DispatchError},
    model::channel::Message,
};

use log::{debug, error, warn};

#[hook]
pub(crate) async fn after(
    _: &Context,
    _: &Message,
    cmd_name: &str,
    result: Result<(), CommandError>,
) {
    if let Err(why) = result {
        error!("Command `{}` returned with an error: {:?}", cmd_name, why);
    }
}

#[hook]
pub(crate) async fn dispatch_error(
    context: &Context,
    msg: &Message,
    err: DispatchError,
) {
    match err {
        DispatchError::NotEnoughArguments { min, given } => {
            let s = format!("Need {} arguments, but only got {}.", min, given);

            let _ = msg.channel_id.say(&context, &s).await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let s = format!(
                "Max arguments allowed is {}, but got {}.",
                max, given
            );

            let _ = msg.channel_id.say(&context, &s).await;
        }
        _ => error!("Unhandled dispatch error: {:?}", err),
    }
}

#[hook]
pub(crate) async fn unrecognised_command(
    _: &Context,
    msg: &Message,
    command: &str,
) {
    warn!(
        "A user named {:?} tried to executute an unknown command: {}",
        msg.author.name, command
    );
}

#[hook]
pub(crate) async fn normal_message(ctx: &Context, msg: &Message) {
    if msg.author.bot {
        return;
    }

    match utils::extract_stocks(ctx, msg).await {
        Ok(stocks) => {
            debug!("Found stonks in message: {}", stocks.iter().format(", "));
            {
                let mut data = ctx.data.write().await;
                if let Some(last) = data.get_mut::<ChannelLastStocks>() {
                    last.insert(msg.channel_id, stocks);
                } else {
                    error!("Could not get last stock cache");
                }
            }
        }
        Err(why) => error!("Could not extract symbols: {:?}", why),
    }
}
