use rhizosdk::fn_main;
use rhizosdk::container::*;

fn_main!(|buffer: Vec<u8>|{
    b"Hello, world!".to_vec()
});

#[no_mangle]
pub fn test() {
    assert_eq(true, true);
}
