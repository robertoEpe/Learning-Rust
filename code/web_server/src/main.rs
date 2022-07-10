use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};


fn main() {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).unwrap();
    println!("Servidor iniciado en http:\\{}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Stream recibido!");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / ";

    if buffer.starts_with(get) {
        response_to_client(stream);
    } else {
        response_page_not_found(stream);
    }


}

fn response_page_not_found(mut stream: TcpStream)  {
    let contents = std::fs::read_to_string("404.html").unwrap();

    let response = format!(
        "HTTP/1.1 404 OK\r\nContent-Lenght: {}\r\n\r\n{}", contents.len(), contents
    );
    println!("\n\n{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn response_to_client(mut stream: TcpStream)  {
    let contents = std::fs::read_to_string("index.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Lenght: {}\r\n\r\n{}", contents.len(), contents
    );
    println!("\n\n{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}