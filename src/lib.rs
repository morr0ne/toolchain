use std::{fmt::Display, str::FromStr};

use target_lexicon::Triple;
use thiserror::Error;
use time::Date;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Toolchain {
    pub channel: Channel,
    pub date: Option<Date>,
    pub host: Option<Triple>,
}

// TODO: Implement FromStr for Toolchain

#[derive(Debug, PartialEq, Eq)]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
    Version {
        major: usize,
        minor: usize,
        patch: usize,
    },
}

#[derive(Debug, Error)]
#[error("Unknown channel \"{0}\"")]
pub struct ChannelFromStrError(String);

impl FromStr for Channel {
    type Err = ChannelFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stable" => Ok(Self::Stable),
            "beta" => Ok(Self::Beta),
            "nightly" => Ok(Self::Nightly),
            // FIXME: This isn't parsing the Version variant
            invalid => Err(ChannelFromStrError(invalid.to_string())),
        }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Self::Stable
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::Stable => write!(f, "stable"),
            Channel::Beta => write!(f, "beta"),
            Channel::Nightly => write!(f, "nightly"),
            Channel::Version {
                major,
                minor,
                patch,
            } => {
                write!(f, "{major}.{minor}.{patch}")
            }
        }
    }
}

impl Display for Toolchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME: This is not the most efficient way, far from it. But it's better than no displat impl at all

        let date = self.date.map_or("".to_string(), |date| {
            format!("-{date}") // NOTE: I'm pretty sure the display impl of Date is the one we want.
        });
        let host = self
            .host
            .as_ref()
            .map_or("".to_string(), |triple| format!("-{triple}"));

        write!(f, "{channel}{date}{host}", channel = self.channel)
    }
}
