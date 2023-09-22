use combine_rust::combine::Combine;
use std::collections::HashMap;

use crate::module_manager;

use std::process::Command;

struct RunCommand;

#[async_trait::async_trait]
impl module_manager::Function for RunCommand {
    async fn function(&self, args: serde_json::Value, _api: &Combine) {
        let args_parsed: Vec<String> = match serde_json::from_value(args) {
            Ok(v) => v,
            Err(_) => return,
        };

        if args_parsed.len() != 2 {
            return;
        };

        Command::new(args_parsed.get(0).unwrap()).args(args_parsed.get(2).unwrap().split(" "));
    }
}

#[allow(dead_code)]
pub fn get_functions() -> HashMap<String, Box<dyn module_manager::Function>> {
    let mut map: HashMap<String, Box<dyn module_manager::Function>> = HashMap::new();

    map.insert(String::from("Run command"), Box::from(RunCommand {}));

    map
}
