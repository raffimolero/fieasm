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
        "Found a {0} header when only {} or {} were expected.",
        HeaderFormat::REGISTER_HEADER,
        HeaderFormat::ARM_HEADER
    )]
    ExpectedRegister(String),

    #[error(
        "You cannot put any registers after the {} header.",
        HeaderFormat::ARM_HEADER
    )]
    RegisterAfterArm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderFormat {
    pub register_count: usize,
    pub has_arm: bool,
}

impl HeaderFormat {
    pub const PRIMARY_HEADERS: [&'static str; 4] = ["STATE", "ARG", "GOTO", "READ"];
    pub const REGISTER_HEADER: &'static str = "REG";
    pub const ARM_HEADER: &'static str = "ARM";

    pub fn column_height(&self, state_bits: usize) -> usize {
        state_bits as usize + 2 + self.register_count * 4 + self.has_arm as usize * 4
    }
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
        let mut this = Self {
            register_count: 0,
            has_arm: false,
        };
        while let Some(token) = tokens.next() {
            match token {
                Self::REGISTER_HEADER => this.register_count += 1,
                Self::ARM_HEADER => this.has_arm = true,
                _ => return Err(ExpectedRegister(token.to_owned())),
            }
        }
        if tokens.next().is_some() {
            return Err(RegisterAfterArm);
        }

        Ok(this)
    }
}
