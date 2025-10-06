
                                                use std::io::{BufRead
                                        , Write};fn main(){let listener = 
                                    std::net::TcpListener::bind("0.0.0.0:3001")
                                  .unwrap();for mut stream in listener.incoming().
                                  flatten()         {let mut rdr        = std::io::
                                  BufReader         ::new(&mut         stream);let
                                  mut list           = String ::        new();rdr.
                                  read_line         (&mut list)        .unwrap();
                                  match list.trim().split(' ').collect::<Vec<_>>()
                                     .as_slice(){["GET",      _, "HTTP/1.1"] =>
                                   { stream.write_all            (b"HTTP/1.1 200
                                   OK\r\n").unwrap();           stream.write_all
                                    (b"Content-Type: text/plain; charset=utf-8
                                    \r\n\r\n").unwrap();stream.write_all(b"\n").
                                      unwrap (); stream.write_all(b"\n").unwrap();
                                            stream.write_all(b"BOO!!!")
                                      .unwrap()  ;let source =    std::fs::
                                read_to_string    (file!()).          unwrap();
                                                 stream.
                                write_all(    source
                                   .as_bytes()).
                                      unwrap();
                                   ;} _ => {}}}}
