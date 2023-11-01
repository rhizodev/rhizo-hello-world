use rhizosdk::fn_main;
use rhizosdk::container::*;

fn_main!(|buffer: Vec<u8>|{
    b"Helo, world!".to_vec()
});
