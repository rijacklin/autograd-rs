mod value;
use value::Value;

fn main() {
    let a = Value::new(2.0);
    let b = Value::new(-3.0);
    let c = a + b;
    println!("{c}");
}
