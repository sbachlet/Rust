extern crate communicator;

fn main() {
  communicator::client::connect();
  let mut v = vec![1,2,3];
  v.push(4);
  v.pop();
  println!("{:?}", v);
  let mut s = "string".to_string();
}