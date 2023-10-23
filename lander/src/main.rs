use clap::{Parser, Subcommand};

mod app;

/// Lander, a landing/start page server
#[derive(Parser)]
#[command(author, version , about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Run lander server with default settings
    #[arg(short = 'd', long)]
    default: bool,

    #[command(subcommand)]
    command: Option<Commands>,

    /// Open server config
    #[arg(short = 'c', long)]
    config: bool, 
}

#[derive(Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// does testing things
    Run {
        /// Server domain
        #[arg(short = 'd', long)]
        domain: String,

        /// Server port
        #[arg(short = 'p', long)]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.default {
        app::run("127.0.0.1", &0b101110111000u16); // localhost:3000
    }
     // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Run { domain, port }) => {
            println!("domain enterd: {}", domain);
            println!("port enterd: {}", port);
            app::run(domain, port);
        }
        None => {}
    }

}
