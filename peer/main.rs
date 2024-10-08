/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{
    env,
    io::{self, BufRead},
    net::{Ipv4Addr, UdpSocket},
    thread,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} rvhost", args[0]);
        std::process::exit(1);
    }
    let rvhost = args[1].clone() + &":55555";
    println!("Specified rendezvous server host: {}", &rvhost);

    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    sock.send_to(&[0], rvhost)?;

    // wait for rendezvous to be ready
    let mut ready_buf = [0; 1];
    loop {
        let (_, src) = sock.recv_from(&mut ready_buf)?;
        println!("Recieved signal from {} (presumed SIP proxy)", &src,);

        if char::from(ready_buf[0]) == 'r' {
            println!("Checked in with the SIP rendezvous server, waiting");
            break;
        }
    }

    let mut data_buf = [0; 32];
    sock.recv_from(&mut data_buf)?;

    // split recieved peer information string into ip and ports
    let data = String::from_utf8_lossy(&data_buf);
    let mut data = data.splitn(3, ";");
    let peer_ip = data.next().unwrap().trim_matches('\0');
    let src_port = data
        .next()
        .unwrap()
        .trim_matches('\0')
        .parse::<u16>()
        .unwrap();
    let dst_port = data
        .next()
        .unwrap()
        .trim_matches('\0')
        .parse::<u16>()
        .unwrap();

    // UDP punching: connect to peer
    let p2p_sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, src_port))?;
    p2p_sock.connect((peer_ip, dst_port))?;
    p2p_sock.send(&[0])?;

    println!("Connected to peer; ready.");

    // spawn listener thread
    let p2p_sock_cln = p2p_sock.try_clone()?;
    thread::spawn(move || -> std::io::Result<()> {
        let mut data = [0; 1024];
        loop {
            p2p_sock_cln.recv(&mut data)?;
            println!(
                "Peer: {}",
                String::from_utf8_lossy(&data).trim_matches('\0')
            );
        }
    });

    let stdin = io::stdin();
    loop {
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            p2p_sock.send(line.as_bytes())?;
        }
    }
}
