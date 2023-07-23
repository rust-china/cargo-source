use cargo_source::*;

#[test]
fn test_pad_end() {
  let s = pad_end("cargo_source", 15, '*');
  assert_eq!("cargo_source***", s)
}