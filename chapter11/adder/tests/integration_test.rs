extern crate adder;

#[test]
fn it_creates(){
  assert_eq!(
    adder::rectangle_factory(5, 6),
    adder::Rectangle {length: 5, width: 6});
}