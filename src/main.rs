use std::error::Error;

use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::{ClientConfig, ParsedConfig},
};

#[derive(Default)]
struct ExampleState {
    text: String,
}
impl UserDefinedState for ExampleState {}

const CONFIG_PATH: &str = "./example_config.toml";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = ClientConfig::load_config(CONFIG_PATH);
    let config = match config {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading config: {}", e.to_string());
            println!("Loading default config...");
            ParsedConfig::default()
        }
    };

    let client_handler = NoshpClient::new();
    client_handler
        .set_state(ExampleState {
            text: String::from("hello world"),
        })
        .add_callback("Turn On", Box::new(turn_on_led))
        .add_callback("Turn Off", Box::new(turn_off_led))
        .run(config)
        .await
        .unwrap();

    return Ok(());
}

fn turn_on_led(state: &mut ClientState<ExampleState>, req: Request) {
    println!("turned on led")
}

fn turn_off_led(state: &mut ClientState<ExampleState>, req: Request) {
    println!("turned off led")
}
