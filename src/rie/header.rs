use std::str::FromStr;
use thiserror::Error;

use crate::helpers::get_tokens;

#[derive(Error, Debug)]
pub enum HeaderErr {
    #[error(
        "Not enough headers; Missing the {0:?} header.\n\
        The {} primary headers must be in the order {}.",
        HeaderFormat::PRIMARY_HEADERS.len(),
        HeaderFormat::PRIMARY_HEADERS.join(", "),
    )]
    MissingHeaders(String),

    #[error(
        "Expected the {0:?} header, but got a header named {1:?}.\n\
        The {} primary headers must be in the order {}.",
        HeaderFormat::PRIMARY_HEADERS.len(),
        HeaderFormat::PRIMARY_HEADERS.join(", "),
    )]
    ExpectedHeader(String, String),

    #[error(
        "This version of rieasm does not support the {0} header.\n\
        The only valid headers are {}, and {}.",
        HeaderFormat::PRIMARY_HEADERS.join(", "),
        HeaderFormat::REGISTER_HEADER,
    )]
    UnsupportedHeader(String),
}

pub struct HeaderFormat {
    pub register_count: usize,
}

impl HeaderFormat {
    pub const PRIMARY_HEADERS: [&'static str; 4] = ["state", "arg", "jump", "read"];
    pub const REGISTER_HEADER: &'static str = "reg";
}

impl FromStr for HeaderFormat {
    type Err = HeaderErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HeaderErr::*;

        let mut tokens = get_tokens(s);

        // Verify if the primary headers exist and are in correct order
        for expected in Self::PRIMARY_HEADERS {
            let actual = tokens.next();
            if actual != Some(expected) {
                return Err(actual.map_or_else(
                    || MissingHeaders(expected.to_owned()),
                    |actual| ExpectedHeader(expected.to_owned(), actual.to_owned()),
                ));
            }
        }

        // Count register headers
        let mut register_count = 0;
        for token in tokens {
            if token != Self::REGISTER_HEADER {
                return Err(UnsupportedHeader(token.to_owned()));
            }
            register_count += 1;
        }

        Ok(HeaderFormat { register_count })
    }
}
