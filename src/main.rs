







                                        const
                                        PAGE:
                                 &str =include_str!("index.html");
                                       const PAYLOAD: &str = include_str!("index.html");
                                      use std::io::{BufRead, Write};use std::{thread, time};

fn main() {
    let listener = std::net::TcpListener::bind("0.0.0.0:3000").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();

        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", _, "HTTP/1.1"] => {
                send_html(&mut stream)
            }
            ["POST", resource, "HTTP/1.1"] => match resource {
_=>todo!()

            },

            _ => {}
        }
    }
}

fn send_html(stream: &mut std::net::TcpStream,) {
    stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream.write_all(b"\n").unwrap();


for page_byte in PAGE.as_bytes(){

  // stream.write_all(page_byte);

}

    stream.shutdown(std::net::Shutdown::Both).unwrap()
}

