extern crate irc;

use std::env;
use irc::client::prelude::*;
use std::path::Path;

fn main() {
    let mut args = env::args().take(3);

    // TODO: fix relative pathing to the config
    let config_path = Path::new("../config/config.json");

    let irc_config = Config::load(&config_path).unwrap();

    let irc_server = IrcServer::from_config(irc_config).unwrap();
    irc_server.identify().unwrap();

    loop {
        irc_server.iter()
                  .map(|m| print!("{}", m.unwrap()))
                  .count();
    }
}
