use std::net::{TcpListener, TcpStream};
use std::io::{prelude, Read, Write};
use std::fs;
use std::thread::{self, JoinHandle};
use server::ThreadPool;
fn main() {

    let p = error_test(std::io::Error::new(std::io::ErrorKind::Other, "Oh NO"));
    print!("{:?}", p);
    fn error_test(s: impl std::error::Error) -> impl std::error::Error {
        s
    }
    println!("Sever Activated, listening on: 127.0.0.1:6090");
    let thread_pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:6090").unwrap();
    for stream in listener.incoming() {
        // A tcp stream is just a connection between host and peer!
        let s = stream.unwrap();
        thread_pool.execute( || {
            handle_stream(s);
        });
    }
    
}
// Chars.count is a better alternative to string.len --> latter is bytes so may not be what is expected.
fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let req = b"GET / HTTP/1.1\r\n";
    let num_of_bytes = stream.read(&mut buffer).unwrap();

    let (response, file_name) = if  buffer.starts_with(req) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(std::time::Duration::from_secs(12));
        ("HTTP/1.1/ 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };


    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",response, contents.len(), contents);
    stream.write(&(response).as_bytes()).unwrap();
    stream.flush().unwrap();
}
    // Not all sequences of bytes are strings, so use from_lossy to safely convert from bytes to string.
/*
Listen for a tcp connection, Read http request, return a http response
*/
