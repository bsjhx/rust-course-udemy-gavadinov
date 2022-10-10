use std::{fmt::format, fs};

use crate::{http::Response, server::Handler};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> Response {
        match request.method() {
            crate::http::Method::GET => match request.path() {
                "/" => Response::new(crate::http::StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(
                    crate::http::StatusCode::Ok,
                    Some("<h1>Hello :/</h1>".to_string()),
                ),
                path => match self.read_file(path) {
                    Some(content) => Response::new(crate::http::StatusCode::Ok, Some(content)),
                    None => Response::new(crate::http::StatusCode::NotFound, None),
                },
            },
            _ => Response::new(crate::http::StatusCode::NotFound, None),
        }
    }
}
