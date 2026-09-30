#[path = "support/rpc_json.rs"]
mod rpc_json;
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

#[test]
fn loopback_rpc_decodes_real_chunked_http_body_and_rejects_http_errors() {
    for (status, expected) in [("200 OK", true), ("500 Server Error", false)] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            let received = socket.read(&mut request).unwrap();
            assert!(received > 0 && request[..received].starts_with(b"GET "));
            let body = b"{\"result\":{\"height\":7}}";
            write!(socket, "HTTP/1.1 {status}\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n", body.len()).unwrap();
            if expected {
                socket.write_all(body).unwrap();
                socket.write_all(b"\r\n0\r\n\r\n").unwrap();
            }
        });
        let result = rpc_json::rpc_json(address, "/fixture");
        assert_eq!(result.is_ok(), expected);
        if expected {
            assert_eq!(result.unwrap()["height"], 7);
        }
        server.join().unwrap();
    }
    assert!(rpc_json::rpc_json("192.0.2.1:80".parse().unwrap(), "/fixture").is_err());
}
