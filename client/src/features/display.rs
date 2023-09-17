use crate::module_manager;

pub struct DisplayModule {
  functions: 
}

impl module_manager::Module for DisplayModule {
  
}

struct ReadDisplayContent {}

impl module_manager::Function for ReadDisplayContent {
    fn function(&self, module: &&mut dyn module_manager::Module, args: Vec<&str>) {}
}
