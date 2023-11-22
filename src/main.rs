use std::{io::{prelude::*, BufReader}, io, net::{TcpListener, TcpStream}, thread};

use ish::*;

fn main() {
	server_socket();
}


fn server_socket() {
	let listener = TcpListener::bind("127.0.0.1:5139").unwrap();
	for stream in listener.incoming() {
		println!("Connection established!");
		thread::spawn(move || {
			let stream = stream.unwrap();
			handle_connection(stream);
		});
	}
}


fn handle_connection(mut stream: TcpStream) {
	let mut request_buf = vec![0u8; 0];
	let mut buffer = [0u8; 1024];

	// 循环读取
	loop {
		if if_data_read_end(&request_buf) {
			break;
		}
		let n = stream.read(&mut buffer).unwrap();
		if n == 0 {
			break;
		} else {
			request_buf.append(&mut buffer[..n].to_vec())
		}
	}

	// 解析请求
	let request = deserialize_client_request(&request_buf).expect("Failed to deserialize");
	println!("client_request: {:#?}", request);

	handle_action(request);

	// 拼组响应
	let response = create_server_response();

	// 返回响应
	stream.write_all(serialize_server_response(&response).as_ref()).unwrap();

	println!("Response: {:#?}", response);
	println!("===========>");
}

/// 判断数据结束标识
fn if_data_read_end(buf: &Vec<u8>) -> bool {
	let len = buf.len();
	return if len <= 2 {
		false
	} else {
		//because it always is the '\r\n\r\n' to end the request.
		buf[len - 1] == b'\n' && buf[len - 2] == b'\r'
	};
}


