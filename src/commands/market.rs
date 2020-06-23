//

use super::utils;

use crate::iex::Market;
use crate::iex::Quote;
use crate::IEXClient;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use std::cmp::Ordering;

use log::error;

enum MarketFunc {
    Losers,
    Gainers,
    Movers,
}

async fn market(
    ctx: &Context,
    msg: &Message,
    func: &MarketFunc,
) -> CommandResult {
    let data = ctx.data.read().await;
    let mut quotes: Vec<Quote>;

    if let Some(client) = data.get::<IEXClient>() {
        match func {
            MarketFunc::Losers => {
                quotes = client.losers().await?;
                quotes.sort_by(|a, b| {
                    a.change_percent
                        .partial_cmp(&b.change_percent)
                        .unwrap_or(Ordering::Equal)
                });
            }
            MarketFunc::Gainers => {
                quotes = client.gainers().await?;
                quotes.sort_by(|a, b| {
                    b.change_percent
                        .partial_cmp(&a.change_percent)
                        .unwrap_or(Ordering::Equal)
                });
            }
            MarketFunc::Movers => {
                quotes = client.most_active().await?;
                quotes.sort_by(|a, b| {
                    b.volume.partial_cmp(&a.volume).unwrap_or(Ordering::Equal)
                });
            }
        }
    } else {
        error!("Could not get iex client");
        msg.channel_id
            .send_message(&ctx.http, |m| m.content("Command Error"))
            .await?;
        quotes = Vec::new();
    }

    utils::send_quotes(ctx, msg, quotes, Vec::new()).await?;
    Ok(())
}

#[command]
async fn losers(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    market(ctx, msg, &MarketFunc::Losers).await
}

#[command]
async fn gainers(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    market(ctx, msg, &MarketFunc::Gainers).await
}

#[command]
async fn movers(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    market(ctx, msg, &MarketFunc::Movers).await
}
