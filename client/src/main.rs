use clap::Parser;
use tracing_subscriber::EnvFilter;

// Audio streamer - capture and send audio
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Example: subcommand placeholder (eg: "play", "connect")
    #[arg(default_value = "help")]
    command: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    tracing::info!("receiver starting..");
    tracing::info!("Args: {:?}", std::env::args().collect::<Vec<_>>());
    Ok(())
}
