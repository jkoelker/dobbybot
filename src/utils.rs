//

use crate::iex;
use crate::IEXClient;

use anyhow::Result;
use serenity::{client::Context, model::channel::Message};

use std::collections::HashSet;

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub async fn extract_stocks(
    ctx: &Context,
    msg: &Message,
) -> Result<HashSet<String>> {
    let symbols: HashSet<String>;

    {
        let data = ctx.data.read().await;
        match data.get::<IEXClient>() {
            Some(client) => match iex::cache::symbols(client).await {
                Some(s) => symbols = s.iter().cloned().collect(),
                None => symbols = HashSet::new(),
            },
            None => symbols = HashSet::new(),
        }
    }

    _extract_stocks(&symbols, &msg.content)
}

fn _extract_stocks(
    symbols: &HashSet<String>,
    content: &str,
) -> Result<HashSet<String>> {
    let mut stocks: HashSet<String> = HashSet::new();
    let mut words: HashSet<String> = HashSet::new();

    for word in content.split_whitespace() {
        if word.starts_with('$') {
            stocks.insert(
                word.to_uppercase()
                    .chars()
                    .filter(|&c| LETTERS.contains(c))
                    .collect(),
            );
            continue;
        }

        words.insert(
            word.to_uppercase()
                .chars()
                .filter(|&c| LETTERS.contains(c))
                .collect(),
        );
    }

    if !stocks.is_empty() {
        return Ok(symbols.intersection(&stocks).cloned().collect());
    }

    let mut discovered: Vec<String> =
        symbols.intersection(&words).cloned().collect();
    discovered.sort_by(|a, b| b.len().cmp(&a.len()));

    if discovered.is_empty() {
        return Ok(HashSet::new());
    }

    if discovered[0].len() == 1 {
        return Ok(discovered.iter().cloned().collect());
    }

    Ok(discovered.iter().filter(|x| x.len() > 1).cloned().collect())
}

#[cfg(test)]
mod test {
    use super::_extract_stocks;

    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const TICKERS: &'static [&'static str] = &["AAPL", "TSLA"];

    #[rstest(
        tickers, content, result,
        case::tagged(TICKERS, "$AAPL", &["AAPL"]),
        case::tagged_multiple(TICKERS, "$AAPL $TSLA", &["AAPL", "TSLA"]),
        case::tagged_one(TICKERS, "$AAPL TSLA", &["AAPL"]),

        case::punctuation(TICKERS, "$AAPL,", &["AAPL"]),
        case::punctuation_multiple(
            TICKERS, "$AAPL, $TSLA;", &["AAPL", "TSLA"]),
        case::punctuation_one(TICKERS, "$AAPL, $TSLA", &["AAPL", "TSLA"]),

        case::sentance(TICKERS, "Just got my new aapl 5.", &["AAPL"]),
        case::sentance_punctuation(
            TICKERS, "What do you think about aapl?", &["AAPL"]),
        case::sentance_multiple(TICKERS, "TSLA or aapl?", &["AAPL", "TSLA"]),

        case::sentance_tagged(TICKERS, "Just got my new $aapl 5.", &["AAPL"]),
        case::sentance_tagged_punctuation(
            TICKERS, "What do you think about $aapl?", &["AAPL"]),
        case::sentance_tagged_multiple(
            TICKERS, "$TSLA or $aapl?", &["AAPL", "TSLA"]),
        case::sentance_tagged_one(TICKERS, "TSLA or $aapl?", &["AAPL"]),
    )]
    fn extract_stocks(
        tickers: &[&str],
        content: &str,
        result: &[&str],
    ) -> Result<()> {
        let symbols = tickers.iter().map(|s| s.to_string()).collect();
        let result = result.iter().map(|s| s.to_string()).collect();

        assert_eq!(_extract_stocks(&symbols, content)?, result);

        Ok(())
    }
}
