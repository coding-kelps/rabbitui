use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "rabbitui")]
#[command(about = "A TUI application for RabbitMQ management")]
pub struct Cli {
    #[arg(short, long)]
    #[arg(default_value="http://localhost:15672")]
    #[arg(help = "Http(s) address of the API. Excludes trailing slash")]
    pub addr: String,

    #[arg(short, long)]
    #[arg(default_value="guest")]
    #[arg(help = "Username for the API auth")]
    pub user: String,

    #[arg(short, long)]
    #[arg(default_value="guest")]
    #[arg(help = "Password for the API auth")]
    pub pass: String,
}
