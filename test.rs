use std::net::TcpListener;

fn listen_available_port() {
    for port in 3007..3008 {
        println!("port");
         match TcpListener::bind(("127.0.0.1", port)) {
             Ok(l) => println!("{:?}", l),
             _ => println!("None"),
         }
    }
}

fn main() {
    listen_available_port();
}