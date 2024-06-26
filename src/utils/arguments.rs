use std::collections::HashMap;
use std::env;

use crate::utils::display_help::display_help;

pub fn parse_arguments() -> HashMap<String, String> {
    let args: Vec<String> = env::args().collect();
    let mut params_map = HashMap::new();

    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        if arg == "--help" || arg == "-h" {
            display_help();
        }
        if let Some(value) = iter.next() {
            params_map.insert(arg.clone(), value.clone());
        }
    }
    params_map
}
