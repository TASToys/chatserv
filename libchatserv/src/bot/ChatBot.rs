extern crate nats;

use nats::*;
use nats::Client;

use util::NatCfg::NatsConf;
pub trait ChatBot {
    fn new(name: &'static str) -> Self;

    fn onEnable();

    fn onDisable();

    fn getClient() -> nats::Client {

        let cfg = NatsConf::new();

        let mut na = nats::Client::new(format!("nats://{}:{}@{}", cfg.user,
                                   cfg.password, cfg.port)).unwrap();

        na.set_name(cfg.name.as_str());

        return na;
    }
}