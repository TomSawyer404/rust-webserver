use std::net::TcpListener;

fn main() {
    let listenser = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listenser.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);
    }
}
