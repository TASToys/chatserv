extern crate serde_yaml;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

use std::vec;

#[derive(Debug, PartialEq, Serialize, Deserialize)]


pub struct IrcConf {
    nick:String,
    pass:String,
    channels:Vec<String>
}
