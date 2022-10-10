use crate::http::{Request, Response, ParseError, request};
use std::convert::TryFrom;
use std::{net::TcpListener, io::Read, io::Write};


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(crate::http::StatusCode::BadRequest, None)
    }
}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("I'm running on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    let read_result = stream.read(&mut buf);
                    match read_result {
                        Ok(_) => {
                            println!("It's ok :) : {}", String::from_utf8_lossy(&buf));
                            // Request::try_from(&buf[..]); Request::try_from(&buf as &[u8]); let result: &Result<Request, _> = &buf[..].try_into();
                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(err) => handler.handle_bad_request(&err),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                        },
                        Err(err) => println!("Error :/ : {}", err),
                    }
                },
                Err(_) => continue,
            }
        }
    }
}
