//

use crate::iex::Quote;

use anyhow::Result;
use num_format::{Locale, ToFormattedString};
use serenity::{client::Context, model::channel::Message};

pub async fn send_quotes(
    ctx: &Context,
    msg: &Message,
    quotes: Vec<Quote>,
    errors: Vec<String>,
) -> Result<()> {
    if !quotes.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    for quote in quotes {
                        let change = if quote.change > 0.0 {
                            format!(
                                ":chart_with_upwards_trend: {:.2}",
                                quote.change
                            )
                        } else {
                            format!(
                                ":chart_with_downwards_trend: {:.2}",
                                quote.change
                            )
                        };

                        let percent = if quote.change_percent > 0.0 {
                            format!(
                                ":arrow_up: {:+.0}%",
                                quote.change_percent * 100.0
                            )
                        } else {
                            format!(
                                ":arrow_down: {:+.0}%",
                                quote.change_percent * 100.0
                            )
                        };

                        let price = format!(":dollar: {}", quote.price);
                        let volume = format!(
                            ":loudspeaker: {}",
                            quote.volume.to_formatted_string(&Locale::en)
                        );

                        e.field(
                            format!("**{}**", quote.symbol),
                            format!(
                                "*{}*\n{}  {}\n{}  {}",
                                quote.name, price, volume, change, percent
                            ),
                            true,
                        );
                    }
                    e
                })
            })
            .await?;
    }

    if !errors.is_empty() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    for error in errors {
                        e.field(error, "Error fetching quote", true);
                    }
                    e
                })
            })
            .await?;
    }
    Ok(())
}
