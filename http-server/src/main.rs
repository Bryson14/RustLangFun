use std::net::TcpListener;

fn main() {
    let port = "127.0.0.1:80";
    let listener = match TcpListener::bind(port) {
        Ok(TcpListener) => TcpListener,
        Err(e) => {
            println!("Error happened binding to port {} {}", port, e);
            panic!();
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("connection established!");
    }
}
