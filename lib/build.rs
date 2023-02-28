use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let dll_dir = Path::new(&manifest_dir).join("../dll");
  Command::new("make")
    .current_dir(&dll_dir)
    .args(&["-f", "Makefile"])
    .status()
    .expect("Failed to make DLL");

  let out_dir = env::var_os("OUT_DIR").unwrap();
  let bin_dir = Path::new(&out_dir).join("../../..");
  for f in ["libbug64.dll.so", "libbug64.dll.fake"] {
    let src = Path::new(&dll_dir).join(f);
    let dst = Path::new(&bin_dir).join(f);
    fs::copy(src, dst).unwrap();
  }
}
