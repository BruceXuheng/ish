extern crate prost_build;

fn main() {
	prost_build::compile_protos(&["src/mainjar_phone.proto"],
	                            &["src/"]).unwrap();
}
