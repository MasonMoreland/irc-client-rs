extern crate irc;

use std::env;
use std::io;
use std::io::Write;
use std::thread::spawn;
use irc::client::prelude::*;

fn usage(argv0: &str) {
    let mut stderr = std::io::stderr();
    writeln!(&mut stderr, "Usage: {} server nick channel", argv0).unwrap();
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        usage(&args[0]);
        return;
    }

    let (server, nick, channel) = (args[1].clone(), args[2].clone(), args[3].clone());

    let irc_server_config = Config {
        server: Some(server),
        nickname: Some(nick),
        channels: Some(vec![channel.clone()]),
        .. Default::default()
    };

    let irc_server = IrcServer::from_config(irc_server_config).unwrap();
    irc_server.identify().unwrap();

    let irc_server_wr = irc_server.clone();

    let _ = spawn(move || {
        let mut input_line = String::new();

        io::stdin().read_line(&mut input_line);

        irc_server_wr.send_privmsg(&channel, &input_line).unwrap();
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
