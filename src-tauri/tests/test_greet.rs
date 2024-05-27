use app;

#[test]
fn test_greet2() {
  let result = app::greet::greet("world");
  assert_eq!(result, "Hello, world!");
}