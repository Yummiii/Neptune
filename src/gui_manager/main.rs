use std::{env, ffi::CString, os::raw::c_char};

extern "C" {
    fn top_nep(path: *const c_char);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    unsafe {
        let block_img = CString::new(if args.len() >= 2 { args[1].clone() } else { "".to_string() }).unwrap();
        top_nep(block_img.as_ptr());
    }
}
