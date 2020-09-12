use headlessmc::connection::packets::types::*;
use headlessmc::connection::packets::serverbound;
use serverbound::login::LoginStart as LoginStartSb;
use serverbound::handshaking::Handshake as HandshakeSb;
use serverbound::status::Request as RequestSb;
use serverbound::status::Ping as PingSb;
use serverbound::play::KeepAlive as KeepAliveSb;
use serverbound::play::ChatMessage as ChatMessageSb;
use headlessmc::connection::packets::clientbound;
use headlessmc::connection::Connection;

use std::thread;

pub fn main() {
    let mut connection = Connection::new("127.0.0.1:25565");

    let mut other_connection = connection.clone();

    thread::spawn(move || {
        loop {
            // println!("reading message");
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer);
            let message = ChatMessageSb { message: McString::new(&buffer)};
            other_connection.write_data(serverbound::play::Data::ChatMessage(message));
        }
    });


    let handshake = serverbound::handshaking::Data::Handshake(HandshakeSb::new("127.0.0.1".to_string(), 25565, VarInt { val: 2 }));
    connection.write_data(handshake);

    let inner = LoginStartSb { name: McString::new("Herobrine") };
    let login_start = serverbound::login::Data::LoginStart(inner);
    connection.write_data(login_start);

    let first_login = connection.read_data::<clientbound::login::Data>();
    match first_login {
        clientbound::login::Data::SetCompression(set_compression) => {
            println!("{:?}", set_compression);
            // Update connection's threshold
            connection.update_compression_threshold(set_compression.threshold.val);
            // Now read LoginSuccess
            println!("{:?}", connection.read_data::<clientbound::login::Data>());
        },
        clientbound::login::Data::LoginSuccess(login_success) => {
            println!("{:?}", login_success);
        },
        data => println!("Got login data I didn't expect: {:?}", data)
    }



    loop {
        match connection.read_data() {
            clientbound::play::Data::KeepAlive(keep_alive) => {
                // println!("Got keep alive: {:?}", keep_alive);

                let inner = KeepAliveSb { keep_alive_id: keep_alive.keep_alive_id };
                connection.write_data(serverbound::play::Data::KeepAlive(inner));
            },
            clientbound::play::Data::ChatMessage(chat_message) => {
                // println!("Got chat: {}", &chat_message.json_data.0.str );
                let chat = chat_message.json_data.content();
                match chat {
                    ChatType::Text { username, message} => {
                        println!("<{}> {}", username, message);
                    }
                    ChatType::Announcement { from, message } => {
                        println!("[{}] {}", from, message);
                    }
                    ChatType::PlayerJoin { player } => {
                        println!("{} joined the game", player);
                    }
                    ChatType::PlayerLeft { player } => {
                        println!("{} left the game", player);
                    }
                    other => println!("{:?}", other)
                }
            },
            clientbound::play::Data::Disconnect(disconnect) => {
                println!("Was disconnected: {:?}", disconnect);
                break;
            }
            _ => (), // I'm not handling all Play packets
        }
    }
}