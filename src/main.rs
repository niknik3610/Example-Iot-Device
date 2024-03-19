use std::error::Error;

use NOSHP_Client::{
    client::{ClientState, NoshpClient, Request, UserDefinedState},
    client_config::{ClientConfig, ParsedConfig},
};

#[derive(Default)]
struct ExampleState {
    pub text: String,
    pub current_brightness_lvl: u32
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

    println!("Device: {}", config.device_name);

    let client_handler = NoshpClient::new();
    client_handler
        .set_state(ExampleState {
            text: String::from("hello world"),
            current_brightness_lvl: 50,
        })
        .add_callback("Turn On", Box::new(turn_on_led))
        .add_callback("Turn Off", Box::new(turn_off_led))
        .add_callback("Brightness", Box::new(handle_brightness))
        .run(config)
        .await
        .unwrap();

    return Ok(());
}

fn turn_on_led(state: &mut ClientState<ExampleState>, _req: Request) {
    state.update_capability_availabillity("Turn On", false).unwrap();
    state.update_capability_availabillity("Turn Off", true).unwrap();
    state.user_state.text = String::from("Turned On");

    println!("State: {}", state.user_state.text);
}

fn turn_off_led(state: &mut ClientState<ExampleState>, _req: Request) {
    state.update_capability_availabillity("Turn On", true).unwrap();
    state.update_capability_availabillity("Turn Off", false).unwrap();
    state.user_state.text = String::from("Turned Off");
    println!("State: {}", state.user_state.text);
}

fn handle_brightness(state: &mut ClientState<ExampleState>, req: Request) {
    let new_brightness = req.value.unwrap();
    state.user_state.current_brightness_lvl = new_brightness as u32;
}
