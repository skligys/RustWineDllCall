use std::ffi::c_uint;

pub fn lib_function(arg1: c_uint, arg2: c_uint) -> bool {
  println!("----- inside lib_function({}, {})", arg1, arg2);
  true
}
