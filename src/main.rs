use rhizosdk::{fn_main, read_argument_bytes, write_return};

fn_main!(|_|{
    b"Hello, World!".to_vec()
});

#[no_mangle]
pub fn test() {
    rhizosdk::test::assert_eq(true, true);
}
