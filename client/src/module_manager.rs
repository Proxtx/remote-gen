use std::collections::HashMap;

pub trait Function {
    fn function(&self, module: Box<&mut dyn Module>, args: Vec<&str>);
}

pub trait Module {
    fn get_functions(&self) -> HashMap<String, Box<dyn Function>>;

    fn run_function(&mut self, function: String, args: Vec<&str>) {
        let functions = self.get_functions();

        match functions.get(&function) {
            Some(entry) => {
                entry.function(Box::from(self), args);
            }
            None => {
                print!("Function {} not found.", function)
            }
        }
    }
}

pub struct ModuleManager {
    modules: HashMap<String, Box<dyn Module>>,
}

impl ModuleManager {
    pub fn new(modules: HashMap<String, Box<dyn Module>>) -> ModuleManager {
        ModuleManager { modules: modules }
    }

    pub fn run_function(&self, module: String, function: String, args: Vec<&str>) {
        match self.modules.get_mut(&module) {
            Some(module) => {
                module.run_function(function, args);
            }
            None => {
                print!("Module {} not found", module)
            }
        }
    }

    pub fn new_filled() -> ModuleManager {
        let map: HashMap<String, Box<dyn Module>> = HashMap::from([
            #[cfg(feature = "display")]
            ("display",),
        ]);

        ModuleManager::new(map)
    }
}
