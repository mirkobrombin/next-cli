use bottles_core::proto::bottles::{management_client::ManagementClient, CreateBottleRequest, DeleteBottleRequest, ListBottlesRequest};
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Create {
        name: String,
        #[arg(short, long, default_value = "Gaming")]
        r#type: String,
    },
    Delete {
        name: String,
    },
    List,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();
    // Connect to Server
    let url = "http://[::1]:50052";
    let mut client = ManagementClient::connect(url).await?;

    match args.command {
        Command::Create { name, r#type } => {
            let request = CreateBottleRequest {
                name,
                r#type,
                runner: String::new(),
            };
            let response = client.create_bottle(request).await?;
            let bottle = response.get_ref();
            println!("Created bottle: {} ({}) at {}", bottle.name, bottle.r#type, bottle.path);
        }
        Command::Delete { name } => {
            let request = DeleteBottleRequest { name };
            let response = client.delete_bottle(request).await?;
            if response.get_ref().success {
                println!("Deleted bottle successfully");
            } else {
                eprintln!("Failed to delete bottle: {}", response.get_ref().error_message);
            }
        }
        Command::List => {
            let request = ListBottlesRequest {};
            let response = client.list_bottles(request).await?;
            let list = response.get_ref();
            println!("Bottles:");
            for bottle in &list.bottles {
                println!("- {} ({}) [{}]", bottle.name, bottle.r#type, if bottle.active { "Running" } else { "Stopped" });
            }
        }
    }
    Ok(())
}
