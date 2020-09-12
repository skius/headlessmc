use headlessmc::connection::Connection;
use headlessmc::connection::packets::{serverbound, clientbound};
use headlessmc::connection::packets::types::VarInt;
use serverbound::handshaking::Handshake as HandshakeSb;
use serverbound::status::Request as RequestSb;
use serverbound::status::Ping as PingSb;

fn main() {
    let mut connection = Connection::new("127.0.0.1:25565");

    let handshake = serverbound::handshaking::Data::Handshake(HandshakeSb::new("127.0.0.1".to_string(), 25565, VarInt { val: 1 }));
    connection.write_data(handshake);

    let req = serverbound::status::Data::Request(RequestSb {});
    connection.write_data(req);
    println!("{:?}", connection.read_data::<clientbound::status::Data>());

    let ping = serverbound::status::Data::Ping(PingSb { payload: 42 });
    connection.write_data(ping);
    println!("{:?}", connection.read_data::<clientbound::status::Data>());
}