#[no_mangle]
pub extern "C" fn add_two(a:u32, b:u32) -> u32 {
    rs_add_two(a,b)
}

pub fn rs_add_two(a: u32, b: u32) -> u32 {
    a + b
}
