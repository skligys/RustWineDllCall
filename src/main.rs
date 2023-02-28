use bug_lib::lib_function;

fn main() {
  println!("----- inside main()");
  let res = lib_function(1, 0);
  println!("----- lib_function() returned {}", res);
}
