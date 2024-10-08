/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::net::UdpSocket;

static COMMON_PORT: u16 = 50002;

fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:55555")?;

    let mut clients = Vec::with_capacity(2);
    let mut buf = [0; 10];

    loop {
        loop {
            let (_, src) = sock.recv_from(&mut buf)?;

            println!(
                "Connection from {} (recieved: '{:?}')",
                &src,
                String::from_utf8_lossy(&buf)
            );
            clients.push(src);

            sock.send_to("ready".as_bytes(), src)?;

            if clients.len() >= 2 {
                println!("Got 2 clients, sending details to each");
                break;
            }
        }

        let c1 = clients.pop().expect("Not enough clients");
        let c2 = clients.pop().expect("Not enough clients");

        sock.send_to(
            format!("{};{};{}", &c1.ip(), &c1.port(), COMMON_PORT).as_bytes(),
            c2,
        )?;
        sock.send_to(
            format!("{};{};{}", &c2.ip(), &c2.port(), COMMON_PORT + 1).as_bytes(),
            c1,
        )?;
    }
}
