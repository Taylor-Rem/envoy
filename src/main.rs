mod client;
mod config;
mod ui;

use anyhow::Result;
use client::ApiClient;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let mut config = Config::load()?;
    
    // Parse args
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    // Create API client
    let client = ApiClient::new(config.server_url.clone());

    // Register device if needed
    let device_id = match config.device_id {
        Some(id) => id,
        None => {
            println!("Registering device '{}'...", config.device_name);
            let id = client.register_device(config.device_name.clone()).await?;
            config.set_device_id(id)?;
            println!("Device registered with ID: {}\n", id);
            id
        }
    };

    // Handle commands
    match args[1].as_str() {
        "chat" => {
            ui::interactive_chat(client, device_id).await?;
        }
        "config" => {
            if args.len() < 3 {
                println!("Current config:");
                println!("  Server URL: {}", config.server_url);
                println!("  Device Name: {}", config.device_name);
                println!("  Device ID: {:?}", config.device_id);
            } else if args[2] == "set" && args.len() >= 5 {
                match args[3].as_str() {
                    "server" => {
                        config.server_url = args[4].clone();
                        config.save()?;
                        println!("Server URL updated to: {}", config.server_url);
                    }
                    "device" => {
                        config.device_name = args[4].clone();
                        config.device_id = None; // Reset device_id, will re-register
                        config.save()?;
                        println!("Device name updated to: {}", config.device_name);
                    }
                    _ => print_usage(),
                }
            } else {
                print_usage();
            }
        }
        message => {
            // Treat any other argument as a message
            ui::single_message(client, device_id, message.to_string()).await?;
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Envoy - Client for Artificer AI");
    println!("\nUsage:");
    println!("  envoy chat                    Start interactive chat");
    println!("  envoy \"your message\"          Send a single message");
    println!("  envoy config                  Show current configuration");
    println!("  envoy config set server URL   Set server URL");
    println!("  envoy config set device NAME  Set device name");
}