//

use serenity::framework::{standard::macros::group, StandardFramework};

mod market;
mod price;
mod utils;

use market::GAINERS_COMMAND;
use market::LOSERS_COMMAND;
use market::MOVERS_COMMAND;
use price::PRICE_COMMAND;

#[group]
#[commands(losers, gainers, movers, price)]
struct Stonks;

pub fn configure_framework(f: StandardFramework) -> StandardFramework {
    f.group(&STONKS_GROUP)
}
