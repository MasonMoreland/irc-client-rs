extern crate irc;

use std::env;
use std::io;
use std::thread::spawn;
use irc::client::prelude::*;

fn get_irc_server() -> IrcServer {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        // TODO: fix relative pathing to the config
        IrcServer::new("../config/config.json").unwrap()
    }
    else {
        let (server, nick, channel) = (args[1].clone(), args[2].clone(), args[3].clone());

        let irc_server_config = Config {
            server: Some(server),
            nickname: Some(nick),
            channels: Some(vec![channel.clone()]),
            .. Default::default()
        };

        IrcServer::from_config(irc_server_config).unwrap()
    }

}

fn main() {
    let irc_server = get_irc_server();
    irc_server.identify().unwrap();

    let irc_server_wr = irc_server.clone();

    let _ = spawn(move || {
        loop {
            let mut input_line = String::new();

            io::stdin().read_line(&mut input_line).unwrap();

            irc_server_wr.config().channels().iter().map(
                |chan| irc_server_wr.send_privmsg(chan, &input_line).unwrap()).count();
        }
    });

    for message in irc_server.iter() {
        let message = message.unwrap();
        match message.command {
            Command::PRIVMSG(ref target, ref msg) =>
                println!("Received from {}: {}", target, msg),
            _ => (),
        }
    }
}
