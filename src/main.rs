use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
  println!("Starting up...");

  let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {

  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();

  //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  parse_request(buffer);

  // TODO: parse request method
  // TODO: parse request URI
  // TODO: generate has of URI
  let get = b"GET / HTTP/1.1\r\n";

  // TODO: handle GET request
  // TODO: load inode
  // TODO: iterate over blocks
  // TODO: return block to client

  // TODO: handle POST request
  // TODO: initialize inode
  // TODO: iterate over request body
  // TODO: hash body block
  // TODO: write block to disk
  // TODO: update inode
  // TODO: return result to client

  let (status_line, filename) = if buffer.starts_with(get){
    ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();
  let response = format!("{}{}", status_line, contents);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();

}

fn parse_request(req: &[u64]) -> String{
  println!("Request: {}", req);

  let result = "";

  result
}
