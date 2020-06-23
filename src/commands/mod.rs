//

use serenity::framework::{standard::macros::group, StandardFramework};

mod price;
mod utils;

use price::PRICE_COMMAND;

#[group]
#[commands(price)]
struct Stonks;

pub fn configure_framework(f: StandardFramework) -> StandardFramework {
    f.group(&STONKS_GROUP)
}
