use std::{ffi::CString, os::raw::c_char};
use clap::Parser;

extern "C" {
    fn top_nep(path: *const c_char, hide_cursor: bool);
}

#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "")]
    image: String,
    #[clap(short, long, value_parser, default_value_t = false)]
    hide_cursor: bool,
}

fn main() {
    let args = Args::parse();
    unsafe {
        let block_img = CString::new(args.image).unwrap();
        top_nep(block_img.as_ptr(), args.hide_cursor);
    }
}
