/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::net::{Ipv4Addr, UdpSocket};

fn main() -> std::io::Result<()> {
    let rvhost = "0.0.0.0:55555";

    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    sock.connect(rvhost)?;

    sock.send_to(&[0], rvhost)?;

    // wait for rendezvous to be ready
    let mut buf = [0; 10];
    loop {
        let (_, src) = sock.recv_from(&mut buf)?;
        println!(
            "Connection from {} (recieved: '{:?}')",
            &src,
            String::from_utf8_lossy(&buf)
        );

        if String::from_utf8_lossy(&buf) == "ready" {
            println!("Checked in with the rendezvous server, waiting");
            break;
        }
    }

    Ok(())
}
