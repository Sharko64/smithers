use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 3000;
const HTML_FILE: &str = "public/index.html";

fn main() -> std::io::Result<()> {
    let address = format!("{HOST}:{PORT}");
    let listener = TcpListener::bind(&address)?;

    println!("Server running at http://{address}");
    println!("Serving: {HTML_FILE}");
    println!("Press Ctrl+C to stop.");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_request(stream) {
                    eprintln!("Request error: {err}");
                }
            }
            Err(err) => {
                eprintln!("Connection error: {err}");
            }
        }
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 8192];

    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let mut lines = request.lines();

    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    println!("{method} {path}");

    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            serve_file(&mut stream, HTML_FILE)
        }

        ("GET", "/health") => {
            let body = "OK\n";
            send_response(
                &mut stream,
                200,
                "text/plain; charset=utf-8",
                body.as_bytes(),
            )
        }

        _ => {
            let body = b"404 Not Found\n";
            send_response(
                &mut stream,
                404,
                "text/plain; charset=utf-8",
                body,
            )
        }
    }
}

fn serve_file(stream: &mut TcpStream, filename: &str) -> std::io::Result<()> {
    let path = Path::new(filename);

    if !path.exists() {
        let body = b"500 Internal Server Error\n";

        return send_response(
            stream,
            500,
            "text/plain; charset=utf-8",
            body,
        );
    }

    let contents = fs::read(path)?;

    send_response(
        stream,
        200,
        "text/html; charset=utf-8",
        &contents,
    )
}

fn send_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let status_text = match status {
        200 => "OK",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    };

    let headers = format!(
        "HTTP/1.1 {status} {status_text}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    );

    stream.write_all(headers.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;

    Ok(())
}
