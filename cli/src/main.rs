use clap::{Parser, Subcommand};
use ethers::types::{H256, U256};

pub mod cli;

#[derive(Parser)]
#[clap(name = "intmax2_cli")]
#[clap(about = "Intmax2 CLI tool")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tx {
        #[clap(long)]
        private_key: H256,
        #[clap(long)]
        to: H256,
        #[clap(long)]
        amount: U256,
        #[clap(long)]
        token_index: u64,
    },
    Deposit {
        #[clap(long)]
        rpc_url: String,
        #[clap(long)]
        eth_private_key: H256,
        #[clap(long)]
        to: H256,
        #[clap(long)]
        amount: U256,
        #[clap(long)]
        token_index: u64,
    },
    Sync {
        #[clap(long)]
        private_key: H256,
    },
    Balance {
        #[clap(long)]
        private_key: H256,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Tx {
            private_key,
            to,
            amount,
            token_index,
        } => {
            println!("Executing tx command");
            // Implement tx logic here
        }
        Commands::Deposit {
            rpc_url,
            eth_private_key,
            to,
            amount,
            token_index,
        } => {
            println!("Executing deposit command");
            // Implement deposit logic here
        }
        Commands::Sync { private_key } => {
            println!("Executing sync command");
            // Implement sync logic here
        }
        Commands::Balance { private_key } => {
            println!("Executing balance command");
            // Implement balance logic here
        }
    }
}
