use std::os::unix::net::UnixDatagram;

fn main() {
    let sock = UnixDatagram::bind("./sock").unwrap();
    let addr = sock.local_addr().unwrap();

    sock.connect_addr(&addr).unwrap();
    sock.send(b"test").unwrap();
    sock.recv(&mut [0; 4]).unwrap();
}
