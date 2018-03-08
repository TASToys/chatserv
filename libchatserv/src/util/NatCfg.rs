extern crate serde_yaml;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NatsConf {
    pub name:String,
    pub user:String,
    pub port:i32,
    pub password:String,
}


fn dump_default_conf() {

    let path = Path::new("config.yml");
    let conf = NatsConf { name: "inst".to_string(), user:"user".to_string(), port: 4244,
        password:"pass".to_string() };

    let serial = serde_yaml::to_string(&conf).unwrap();

    if !path.exists() {
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {} file!", path.display()),
            Ok(file) => file,
        };

        match file.write_all(serial.as_bytes()) {
            Err(why) => {
                panic!("couldn't write into {}!")
            },
            Ok(_) => println!("created defualt config")
        }
    }
}

impl NatsConf {

    pub fn new() -> NatsConf {
        dump_default_conf();

        let path = Path::new("config.yml");
        let mut file = File::open(&path).expect("unable to open config.yml");
        let mut f2s = String::new();
        file.read_to_string(&mut f2s).expect("somthing when wrong reading file");

        let dec: NatsConf = serde_yaml::from_str(&f2s).unwrap();

        return dec;
    }
}

