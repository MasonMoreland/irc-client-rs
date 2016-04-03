extern crate irc;

use std::env;
use irc::client::prelude::*;

fn main() {
    let mut args = env::args().take(3);
    let (_, server, nickname) =
        (args.next().unwrap(),
        args.next().unwrap(),
        args.next().unwrap());

    let irc_config = Config {
        nickname: Some(String::from(nickname)),
        server: Some(String::from(server)),
        channels: Some(vec![String::from("#GtAixzOf")]),
        .. Default::default()
    };

    let irc_server = IrcServer::from_config(irc_config).unwrap();
    irc_server.identify().unwrap();

    loop {
        irc_server.iter().map(|m| print!("{}",
                                         m.unwrap())).count();
    }
}
