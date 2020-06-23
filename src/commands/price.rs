//

use super::utils;

use crate::iex;
use crate::iex::Quote;
use crate::iex::Stock;
use crate::ChannelLastStocks;
use crate::IEXClient;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use log::error;
use std::collections::HashSet;

#[command]
async fn price(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let stocks: Vec<String>;

    if args.is_empty() {
        let data = ctx.data.read().await;
        match data.get::<ChannelLastStocks>() {
            Some(last) => match last.get(&msg.channel_id) {
                Some(s) => stocks = s.iter().cloned().collect(),
                None => stocks = Vec::new(),
            },
            None => {
                error!("Could not get last stock cache");
                stocks = Vec::new();
            }
        }
    } else {
        stocks = args
            .trimmed()
            .quoted()
            .iter::<String>()
            .filter_map(|x| match x {
                Ok(a) => Some(a.trim_start_matches('$').to_uppercase()),
                Err(_) => None,
            })
            .collect();
    }

    let mut quotes: Vec<Quote> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    if stocks.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| m.content("No symbols found"))
            .await?;
    } else {
        let data = ctx.data.read().await;

        if let Some(client) = data.get::<IEXClient>() {
            if let Some(symbols) = iex::cache::symbols(client).await {
                let symbols: HashSet<String> =
                    symbols.iter().cloned().collect();

                for stock in stocks {
                    if !symbols.contains(&stock) {
                        errors.push(stock.clone());
                        continue;
                    }

                    match client.quote(&stock).await {
                        Ok(q) => quotes.push(q),
                        Err(why) => {
                            error!(
                                "Could not get quote for {}: {:?}",
                                stock, why
                            );
                            errors.push(stock.clone());
                        }
                    }
                }
            } else {
                error!("Could not get list of symbols");
                msg.channel_id
                    .send_message(&ctx.http, |m| m.content("Command Error"))
                    .await?;
            }
        } else {
            error!("Could not get iex client");
            msg.channel_id
                .send_message(&ctx.http, |m| m.content("Command Error"))
                .await?;
        }
    }

    utils::send_quotes(ctx, msg, quotes, errors).await?;
    Ok(())
}
