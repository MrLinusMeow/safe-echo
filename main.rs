fn main(){
    // localhost on port 42069
    let listener = std::net::TcpListener::bind("127.0.0.1:42069").expect("Could not bind :(");
    listener.set_nonblocking(true).expect("Could not set NON-BLOCKING");
    loop{
        match listener.accept() {
            Ok((mut socket, addr)) => {
                println!("New client: {addr:?}");
                loop{
                    let mut buffer: [u8;128] = [0;128];
                    std::io::Read::read(&mut socket, &mut buffer).unwrap();
                    match std::io::Write::write(&mut socket, b"Message recieved.\n") {
                        Ok(_) => {
                            std::io::Write::write(&mut std::io::stdout(), &buffer).unwrap();
                        },
                        Err(_) => {
                            println!("Closed client: {addr:?}");
                            break;
                        },
                    };
                };
            },
            Err(_) => continue,
        };
    }
}
