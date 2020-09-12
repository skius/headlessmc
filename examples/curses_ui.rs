use headlessmc::connection::packets::types::*;
use headlessmc::connection::packets::serverbound;
use serverbound::login::LoginStart as LoginStartSb;
use serverbound::handshaking::Handshake as HandshakeSb;
use serverbound::play::KeepAlive as KeepAliveSb;
use serverbound::play::ChatMessage as ChatMessageSb;
use headlessmc::connection::packets::clientbound;
use headlessmc::connection::{Connection, InMcStream, OutMcStream};

use std::sync::mpsc::{channel, Sender};

use std::thread;

use pancurses::{initscr, endwin, Input, noecho};
use std::time::Duration;

fn main() {
    let (in_stream, mut out_stream) = Connection::new("127.0.0.1:25565", false);

    let keep_alive_out = out_stream.clone();

    let (sender, receiver) = channel::<String>();

    thread::spawn(move || {
        handle_normal(in_stream, keep_alive_out, sender);
    });

    let window = initscr();

    let (height, width) = window.get_max_yx();

    let chat_subwin = window.subwin(height - 2, width, 0, 0).unwrap();
    chat_subwin.refresh();

    window.mv(height - 2, 0);
    window.addstr("Send chat messages:");

    let input_subwin = window.subwin(1, width, height - 1, 0).unwrap();
    input_subwin.refresh();

    window.refresh();
    window.keypad(true);

    noecho();

    let mut input = String::new();
    input_subwin.timeout(5);
    loop {
        match receiver.recv_timeout(Duration::from_millis(1)) {
            Ok(s) => {
                chat_subwin.addstr(s);
                chat_subwin.addch('\n');
                chat_subwin.refresh();
                window.touch();
            },
            Err(_) => (),
        }

        match input_subwin.getch() {
            Some(Input::Character(c)) if c == '\n' => {
                input.push('\n');
                send_mc_message(&mut out_stream, &input);

                input_subwin.mv(0, 0);
                input_subwin.clrtoeol();
                input.clear();
                window.touch();

            },
            Some(Input::Character(c)) => {
                input.push(c);
                input_subwin.addch(c);
            },
            Some(Input::KeyDC) => break,
            Some(input) => { input_subwin.addstr(&format!("{:?}", input)); },
            None => ()
        }
    }
    endwin();
}

fn send_mc_message(out: &mut OutMcStream, msg: &str) {
    let message = ChatMessageSb { message: McString::new(msg)};
    out.write_data(serverbound::play::Data::ChatMessage(message));
}

fn handle_normal(mut in_stream: InMcStream, mut out_stream: OutMcStream, sender: Sender<String>) {
    let handshake = serverbound::handshaking::Data::Handshake(HandshakeSb::new("127.0.0.1".to_string(), 25565, VarInt { val: 2 }));
    out_stream.write_data(handshake);

    let inner = LoginStartSb { name: McString::new("Herobrine") };
    let login_start = serverbound::login::Data::LoginStart(inner);
    out_stream.write_data(login_start);

    let first_login = in_stream.read_data::<clientbound::login::Data>();
    match first_login {
        clientbound::login::Data::SetCompression(set_compression) => {
            println!("{:?}", set_compression);
            // Update connection's threshold
            in_stream.update_compression_threshold(set_compression.threshold.val);
            // Now read LoginSuccess
            println!("{:?}", in_stream.read_data::<clientbound::login::Data>());
        },
        clientbound::login::Data::LoginSuccess(login_success) => {
            println!("{:?}", login_success);
        },
        data => println!("Got login data I didn't expect: {:?}", data)
    }

    loop {
        match in_stream.read_data() {
            clientbound::play::Data::KeepAlive(keep_alive) => {
                // println!("Got keep alive: {:?}", keep_alive);

                let inner = KeepAliveSb { keep_alive_id: keep_alive.keep_alive_id };
                out_stream.write_data(serverbound::play::Data::KeepAlive(inner));
            },
            clientbound::play::Data::ChatMessage(chat_message) => {
                // println!("Got chat: {}", &chat_message.json_data.0.str );
                let chat = chat_message.json_data.content();
                match chat {
                    ChatType::Text { username, message} => {
                        sender.send(format!("<{}> {}", username, message)).unwrap();
                    }
                    ChatType::Announcement { from, message } => {
                        sender.send(format!("[{}] {}", from, message)).unwrap();
                    }
                    ChatType::PlayerJoin { player } => {
                        sender.send(format!("{} joined the game", player)).unwrap();
                    }
                    ChatType::PlayerLeft { player } => {
                        sender.send(format!("{} left the game", player)).unwrap();
                    }
                    other => sender.send(format!("{:?}", other)).unwrap()
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