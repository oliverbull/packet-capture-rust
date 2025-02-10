use std::fs::File;
use std::io::Write;
use libc::{socket, AF_PACKET, SOCK_RAW, ETH_P_ALL};
use libc::{setsockopt, SOL_SOCKET, SO_BINDTODEVICE};
use libc::{recvfrom, sockaddr};

fn main() {

    // prepare the logfile
    let log_file_res = File::create("log.txt");
    
    let mut log_file = match log_file_res {
        Ok(file) => file,
        Err(error) => panic!("error opening the file: {error:?}"),
    };

    // test the logger
    logger_file("test 1", &mut log_file, true);
    logger("test 2");

    // prepare the socket and bind to the target interface
    let sock = unsafe { socket(AF_PACKET, SOCK_RAW, (ETH_P_ALL as u16).to_be() as i32) };
    if sock < 0 {
        panic!("failed to create raw socket: {}", std::io::Error::last_os_error());
    }
    let if_name = String::from("enp0s3");
    let b = unsafe {
        setsockopt(
            sock,
            SOL_SOCKET,
            SO_BINDTODEVICE,
            if_name.as_ptr() as *const libc::c_void,
            libc::IFNAMSIZ as libc::socklen_t,
        )
    };
    if b < 0 {
        panic!("failed to bind socket to interface: {}", std::io::Error::last_os_error());
    }

    // prepare the data store
    let mut buf = [0u8; 65535];
    let mut peer_addr = sockaddr{sa_family: 0, sa_data: [0i8; 14]};
    let mut peer_addr_len = 0u32;

    // get one packet
    let len = unsafe { recvfrom(sock, buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0, &mut peer_addr, &mut peer_addr_len) };
    if len < 0 {
        panic!("error receiving packet: {}", std::io::Error::last_os_error());
    }

    // Convert the buf to a packet vector
    let packet: Vec<u8> = buf[..len as usize].to_vec();

    // Print the packet in hexadecimal format
    println!("received packet: {:02X?}", packet);

}


fn logger(log: &str) {
    // print to terminal
    println!("{}", log);
}

fn logger_file(log: &str, fp: &mut File, term: bool) {
    // print to terminal
    if term {
        println!("{}", log);
    }
    // print to file
    let write_res = fp.write_all(log.as_bytes());
    match write_res {
        Ok(_) => (),
        Err(error) => println!("error writing the file: {error:?}"),
    };
}