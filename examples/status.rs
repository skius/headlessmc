use headlessmc::connection::Connection;
use headlessmc::connection::packets::{serverbound, clientbound};
use headlessmc::connection::packets::types::VarInt;
use serverbound::handshaking::Handshake as HandshakeSb;
use serverbound::status::Request as RequestSb;
use serverbound::status::Ping as PingSb;

fn main() {
    let (mut in_stream, mut out_stream) = Connection::new("127.0.0.1:25565", false);

    let handshake = serverbound::handshaking::Data::Handshake(HandshakeSb::new("127.0.0.1".to_string(), 25565, VarInt { val: 1 }));
    out_stream.write_data(handshake);

    let req = serverbound::status::Data::Request(RequestSb {});
    out_stream.write_data(req);
    println!("{:?}", in_stream.read_data::<clientbound::status::Data>());

    let ping = serverbound::status::Data::Ping(PingSb { payload: 42 });
    out_stream.write_data(ping);
    println!("{:?}", in_stream.read_data::<clientbound::status::Data>());
}