//! Tiny local Wi-Fi server for the compiled Rust/WebAssembly game.
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    path::PathBuf,
    thread,
};
fn serve(mut stream: TcpStream, root: PathBuf) {
    let mut request = [0u8; 4096];
    let Ok(n) = stream.read(&mut request) else {
        return;
    };
    let request = String::from_utf8_lossy(&request[..n]);
    let file = request
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .split('?')
        .next()
        .unwrap_or("/");
    let (name, kind) = match file {
        "/" | "/index.html" => ("index.html", "text/html; charset=utf-8"),
        "/mq_js_bundle.js" => ("mq_js_bundle.js", "text/javascript"),
        "/starfighter.wasm" => ("starfighter.wasm", "application/wasm"),
        _ => {
            let _ = stream.write_all(
                b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            );
            return;
        }
    };
    if let Ok(bytes) = std::fs::read(root.join(name)) {
        let h=format!("HTTP/1.1 200 OK\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",bytes.len());
        let _ = stream.write_all(h.as_bytes());
        let _ = stream.write_all(&bytes);
    } else {
        let _ = stream
            .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    }
}
fn main() {
    let port = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8787);
    let root = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("web")))
        .filter(|p| p.join("index.html").exists())
        .unwrap_or_else(|| PathBuf::from("web"));
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let _ = socket.connect("192.0.2.1:80");
    let ip = socket
        .local_addr()
        .map(|a| a.ip().to_string())
        .unwrap_or_else(|_| "YOUR-PC-IP".into());
    println!("STARFIGHTER / REBEL COMMAND\n\nComputer: http://localhost:{port}\nPhone: http://{ip}:{port}\n\nConnect your phone to the same Wi-Fi and open the Phone link.\nKeep this window open while playing. Press Ctrl+C to stop.\n");
    let listener = TcpListener::bind(("0.0.0.0", port)).unwrap_or_else(|e| {
        eprintln!("Cannot use port {port}: {e}");
        std::process::exit(1)
    });
    for stream in listener.incoming().flatten() {
        let root = root.clone();
        thread::spawn(move || serve(stream, root));
    }
}
