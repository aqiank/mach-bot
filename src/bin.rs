extern crate irc;

use irc::client::prelude::*;
use irc::proto::command::Command::*;

fn main() {
    let server = IrcServer::new("config.toml").unwrap();

    server.identify().unwrap();

    server.for_each_incoming(|message| {
        println!("{}", &message);

        match message.command {
            JOIN(_, _, _) => {
                server.send(Message{
                    command: PRIVMSG(String::from("#mach11726"), String::from("LUL")),
                    prefix: None,
                    tags: None,
                }).unwrap();
            }
            PING(_, _) => {
                server.send(PONG(String::from("tmi.twitch.tv"), None)).unwrap();
            },
            _ => {}
        };
    }).unwrap()
}
