use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BinaryConfig {
    pub target: String,
    pub arguments: HashMap<String, String>,
    pub flags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    pub binary: BinaryConfig,
}

impl ProfileConfig {
    pub fn argument_replacement_for(&self, name: &str) -> String {
        self.binary
            .arguments
            .get(name)
            .cloned()
            .unwrap_or(name.to_string())
    }
}

//[binary]
// target = "js"
// arguments={
//     ursylaArgument="binaryArgument"
// }
// flags=[
//     "flagsForSlynx"
//]
