use std::collections::HashMap;

use crate::{features, FetchCommand};
use combine_rust::combine::Combine;

#[async_trait::async_trait]
pub trait Function {
    async fn function(&self, args: serde_json::Value, api: &Combine);
}

pub struct Module {
    functions: HashMap<String, Box<dyn Function>>,
}

impl Module {
    pub async fn run_function(&self, function: String, args: serde_json::Value, api: &Combine) {
        match self.functions.get(&function) {
            Some(entry) => {
                entry.function(args, api).await;
            }
            None => {
                println!("Function {} not found.", function)
            }
        }
    }

    pub fn new(functions: HashMap<String, Box<dyn Function>>) -> Module {
        Module {
            functions,
        }
    }
}

pub struct ModuleManager {
    modules: HashMap<String, Box<Module>>,
}

impl ModuleManager {
    pub fn new(modules: HashMap<String, Box<Module>>) -> ModuleManager {
        ModuleManager { modules }
    }

    pub async fn run_function(
        &self,
        module: String,
        function: String,
        args: serde_json::Value,
        api: &Combine,
    ) {
        match self.modules.get(&module) {
            Some(module) => {
                module.run_function(function, args, api).await;
            }
            None => {
                println!("Module {} not found", module)
            }
        }
    }

    pub async fn run_command(&self, command: FetchCommand, api: &Combine) {
        self.run_function(command.module, command.function, command.args, api)
            .await
    }

    pub fn new_filled() -> ModuleManager {
        let mut map: HashMap<String, Box<Module>> = HashMap::new();
        #[cfg(feature = "display")]
        map.insert(
            String::from("display"),
            Box::new(Module::new(features::display::get_functions())),
        );

        #[cfg(feature = "command")]
        map.insert(String::from("command"), Box::new(Module::new(features::command::get_functions())));

        ModuleManager::new(map)
    }
}
