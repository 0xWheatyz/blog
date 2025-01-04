use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

mod html_parser;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000")
        .unwrap();
    
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);

    }
}

fn handle_connection(mut stream: TcpStream) {
    // Init buffer
    let mut buffer = [0; 1024];
    // Read tcp stream into buffer
    stream.read(&mut buffer).unwrap();
    // Convert buffer to string for easier parsing
    let request = String::from_utf8_lossy(&buffer[..]).to_string();

    // handle request, parse for page and verify page is 👍
    let endpoint = handle_request(&request);

    // handle response, return content TcpStream
    stream
        .write(&handle_response(&endpoint))
        .unwrap();
}

fn handle_request(request: &str) -> String {
    // Split the headers into a vector, at index 1 should be the requested path
    let temp_path: Vec<&str> = request
        .split(' ')
        .collect();
    // Set filename to be equal to the path
    let requested_path = temp_path[1];
    //println!("{:?}", temp_path);
    // Raise error if directory traversal is detected 
    if requested_path.contains("..") || requested_path.contains(".env") {
        return "/var/www/error.html".to_string()
    }

    // Verify filename is in ACL
    let raw_acl: String = fs::read_to_string("/var/www/.acl.env")
        .expect("String");
    let acl: Vec<&str>  = raw_acl
        .split("\n")
        .collect();

    for approved_domain in acl {
        let approved_domain_part: Vec<&str> = approved_domain.split("=>").collect();
        
        if approved_domain_part.len() == 1 {
            return "/var/www/error.html".to_string()
        }

        let approved_req_path = approved_domain_part[0];
        let approved_file_path = approved_domain_part[1];
        if approved_req_path == requested_path {
            let absloute_path = format!("/var/www/{}", approved_file_path);
            return absloute_path;
        }
    }
    "/var/www/error.html".to_string()
}

fn handle_response(file: &str) -> Vec<u8> {
    let contents = if file.ends_with(".wmd") {
        html_parser::convert_wmd_to_html(
            file.to_string())
    } else {
        fs::read_to_string(file).expect("Read string from error")
    };

    let response_head: &str = "HTTP/1.1 200 OK\r\n\r\n";
    
    let new_response: String = format!("{}{}", response_head.to_owned(), contents);
    
    new_response
        .into_bytes()
}

fn handle_errors(err_msg: String) -> String {
    let error_message = format!("An error has occured: {}", err_msg);
    error_message
}
