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

//[binary]
// target = "js"
// arguments={
//     ursylaArgument="binaryArgument"
// }
// flags=[
//     "flagsForSlynx"
//]
