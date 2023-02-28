use std::ffi::{c_uint, c_void};
use lazy_static::lazy_static;

struct DllLoader {
  lib_dll_function: unsafe extern "C" fn(arg1: c_uint, arg2: c_uint) -> bool,
}

type HModule = *const c_void;
type FarProc = *const c_void;
extern "C" {
  fn LoadLibraryA(name: *const u8) -> HModule;
  fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

lazy_static! {
  static ref DLL_LOADER: DllLoader = {
    let h = unsafe { LoadLibraryA("libbug64.dll.so\0".as_ptr()) };
    // println!("----- DLL hmodule: {:?}", h);

    let lib_dll_function_fn: unsafe extern "C" fn(config: c_uint, interface: c_uint) -> bool = unsafe {
      let p = GetProcAddress(h, "lib_dll_function\0".as_ptr());
      // println!("----- lib_dll_function_fn: {:?}", p);
      std::mem::transmute(p)
    };
    DllLoader { lib_dll_function: lib_dll_function_fn }
  };
}

pub fn lib_function(arg1: c_uint, arg2: c_uint) -> bool {
  println!("----- inside lib_function({}, {})", arg1, arg2);
  unsafe { (DLL_LOADER.lib_dll_function)(arg1, arg2) }
}
