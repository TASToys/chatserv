extern crate libchatserv;

#[macro_use]
extern crate serde_derive;

mod twitch;

use libchatserv::bot::ChatBot;

//use
fn main() {
    println!("Hello, world!");
    twitch::TwitchBot::connect_to_twitch();
//    let twit: ChatBot;
//    let twit = IrcCfg;


}
