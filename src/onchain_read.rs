use rhizosdk::{async_runtime, fn_main, read_argument_bytes, read_oncb, write_return};

fn_main!(|_|{
    async_runtime({
        read_oncb!("YOUR BYTE'S PDA GOES HERE")
    });
    vec![]
});

#[no_mangle]
pub fn test() {
    rhizosdk::test::assert_eq(true, true);
}
