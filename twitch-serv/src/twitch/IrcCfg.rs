extern crate serde_yaml;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

use std::vec;

//#[derive(Debug, PartialEq, Serialize, Deserialize)]
//pub struct IrcConf {
//    nick:String,
//    pass:String,
//    channels:Vec<String>
//}
//
//fn dump_defualt_conf() {
//    let path = Path::new("twitch.yml");
//    let channels = vec!["#dwangoac"];
//    let conf = IrcConf { nick: "justinfan6454654654".to_string(), pass: "example".to_string(), channels };
//
//    let serial = serde_yaml::to_string(&conf).unwrap();
//
//    if !path.exists() {
//        let mut file = match File::create(&path) {
//            Err(why) => panic!("Couldn't create {} file!", path.display()),
//            Ok(file) => file,
//        };
//
//        match file.write_all(serial.as_bytes()) {
//            Err(why) => {
//                panic!("Couldn't write into {}!", path.display())
//            }
//        };
//    }
//}
//
//impl IrcConf {
//
//    pub fn new() -> IrcConf {
//        dump_defualt_conf();
//
//        let path = Path::new("twitch.yml");
//
//        let mut file = File::open(&path).expect("Unable to open twitch.yml");
//        let mut f2s = String::new();
//        file.read_to_string(&mut f2s).expect("Somthing went wrong reading file");
//
//        let dec:IrcConf = serde_yaml::from_str(&f2s).unwrap();
//
//        return dec;
//    }
//}