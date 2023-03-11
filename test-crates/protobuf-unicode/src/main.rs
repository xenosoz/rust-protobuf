
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use library::Book;
use protobuf::Message;
use protobuf;

fn main() {
    let textproto_a = r#"title: "나를 사랑하지 않는 나에게"
binary_content: "{'current_count': 3}"}"#;
    let message_a: Book = protobuf::text_format::parse_from_str(&textproto_a).unwrap();
    let buffer_a: Vec<u8> = message_a.write_to_bytes().unwrap();

    let mut message_b = Book::new();
    message_b.title = "나를 사랑하지 않는 나에게".to_string();
    message_b.binary_content = b"{'current_count': 3}".to_vec();
    let buffer_b: Vec<u8> = message_b.write_to_bytes().unwrap();

    assert_eq!(buffer_a, buffer_b);
}
