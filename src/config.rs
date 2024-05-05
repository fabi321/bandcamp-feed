use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about = "RSS Server providing bandcamp feeds")]
pub struct Config {
    /// Own url, for self references
    #[arg(short, long)]
    pub own_url: String,

    /// Maximum cache age, default: 12h
    #[arg(short, long, value_parser = humantime::parse_duration, default_value = "12h")]
    pub cache_age: Duration,
}

