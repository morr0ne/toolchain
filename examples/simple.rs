use target_lexicon_macros::triple;
use time_macros::date;
use toolchain::{Channel, Toolchain};

fn main() {
    let toolchain = Toolchain {
        channel: Channel::Nightly,
        date: Some(date!(2022 - 02 - 16)),
        host: Some(triple!("x86_64-unknown-linux-gnu")),
    };

    println!("{toolchain}");
}
