use std::collections::HashMap;

use crate::module_manager;
use combine_rust::combine::Combine;

struct GetDisplayInfo;

#[async_trait::async_trait]
impl module_manager::Function for GetDisplayInfo {
    async fn function(&self, _args: serde_json::Value, api: &Combine) {
        let screen = Screen::new();
        let info = screen.screens();
        let _ = api
            .run_combine_function::<crate::FunctionResult>(
                "run_function",
                combine_rust::combine::CombineArguments::new()
                    .push(crate::CLIENT_ID)
                    .push(crate::AUTH_KEY)
                    .push(info),
            )
            .await;
    }
}

struct Screenshot;

#[async_trait::async_trait]
impl module_manager::Function for Screenshot {
    async fn function(&self, args: serde_json::Value, api: &Combine) {
        let screen_index: usize = match serde_json::from_value::<Vec<usize>>(args) {
            Ok(parsed) => match parsed.get(0) {
                Some(index) => *index,
                None => {
                    println!("Not enough arguments");
                    return;
                }
            },
            Err(err) => {
                println!("Unable to parse args in screenshot {:?}", err);
                return;
            }
        };

        let screen = Screen::new();
        let screenshot = screen.screenshot(screen_index);
        let _ = api
            .run_combine_function::<crate::FunctionResult>(
                "run_function",
                combine_rust::combine::CombineArguments::new()
                    .push(crate::CLIENT_ID)
                    .push(crate::AUTH_KEY)
                    .push(screenshot),
            )
            .await;
    }
}

pub fn get_functions() -> HashMap<String, Box<dyn module_manager::Function>> {
    let mut map: HashMap<String, Box<dyn module_manager::Function>> = HashMap::new();
    map.insert(
        String::from("get_display_info"),
        Box::from(GetDisplayInfo {}),
    );
    map.insert(String::from("screenshot"), Box::from(Screenshot {}));
    map
}

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        Screen {}
    }

    pub fn screenshot(&self, screen_index: usize) -> Option<Vec<u8>> {
        let screens = screenshots::Screen::all().unwrap();

        if screens.len() <= screen_index {
            return Option::None;
        }

        let mut t = 0;
        let mut screen_option: Option<screenshots::Screen> = Option::None;
        for local_screen in screens {
            if t == screen_index {
                screen_option = Option::Some(local_screen);
                break;
            }

            t += 1;
        }

        let screen;

        match screen_option {
            Some(local_screen) => screen = local_screen,
            None => return Option::None,
        }

        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        return Option::Some(buffer.to_owned());
    }

    pub fn screens(&self) -> Vec<String> {
        let screens = screenshots::Screen::all().unwrap();
        let mut res: Vec<String> = Vec::new();
        for screen in screens {
            res.push(format!("{:?}", screen.display_info));
        }
        return res;
    }
}
