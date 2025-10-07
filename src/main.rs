//<pre>
                                              use std::io::{BufRead
                                        , Write};fn main(){let listener = 
                                    std::net::TcpListener::bind("0.0.0.0:8001")      
                                  .unwrap();for mut stream in listener.incoming().
                                  flatten()         {let mut rdr        = std::io::
                                  BufReader         ::new(&mut           stream);let  
                                  mut list           = String ::          new();rdr.
                                  read_line         (&mut list)         .unwrap();   
                                  let lsl = list.trim(); let _= 0;match lsl.split
                (' ').            collect::<Vec<_>>().as_slice(){["GET",  _,"HTTP/1.1"]       =>{stream
              .write_all          (b"HTTP/1.1 200 OK           \r\n").unwrap();        let _8001=1;
                   stream         .write_all(b"Content-Type: text/html; charset=utf-8\r\n\r\n")
                .unwrap();stream.write_all(b"\n").unwrap();stream.write_all(b"\n").unwrap();
                      const S: &str = include_str!("main.rs");stream.write_all(S.as_bytes())
                                     .unwrap();let x = r#"/*<style>html, body {margin: 0;
                                       background-color:black;color: #f8f1e5;
                                       font-family: "Courier New", monospace;
                                          animation-name:myAnimation;
                                           animation-duration:4s;}
                                          @keyframes myAnimation {
                                        from {color: #f8f1e5;}
                                        to {color: black;}}
                                          </style> */"#;
                                stream.write_all("<pre>".as_bytes()).unwrap();
                                stream.write_all("<ayayayaya".as_bytes()).unwrap();
                                stream.write_all(x.as_bytes()).unwrap();
                                stream.write_all("</pre>".as_bytes()).unwrap();
                                        } _ => {}}}}
