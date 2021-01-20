mod vector;

fn main() {
    let v = vector::Vector::new(String::from("toby"));
    assert_eq!(v.pop(), "toby");
}
