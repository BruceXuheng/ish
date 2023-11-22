
use std::fmt::Debug;
use std::io::Cursor;
use prost::Message;
use crate::wap::client_request;

pub mod wap {
	include!(concat!(env!("OUT_DIR"), "/com.mainjar.wap.rs"));
}


pub fn create_server_response() -> wap::ServerResponse {
	let mut response = wap::ServerResponse::default();
	response.res_code = 0;
	response.res_msg = String::from("OK");
	response
}

pub fn serialize_server_response(server_response: &wap::ServerResponse) -> Vec<u8> {
	let mut buf = Vec::new();
	buf.reserve(server_response.encoded_len());
	server_response.encode_length_delimited(&mut buf).unwrap();
	buf
}

pub fn deserialize_client_request(buf: &[u8]) -> Result<wap::ClientRequest, prost::DecodeError> {
	wap::ClientRequest::decode_length_delimited(&mut Cursor::new(buf))
}

pub fn handle_action(request: wap::ClientRequest) {
	let message = request.message.expect("message Invalid");

	match message {
		client_request::Message::FrpRequest(frp_request) => {
			println!("action_frp => {}:{}:{}", frp_request.frp_remote_port, frp_request.tcp_local_port, frp_request.use_time_minute);
		}
		client_request::Message::ShellRequest(shell_request) => {
			println!("action_cmd => {}", shell_request.cmd_str);
		}
	};
}





