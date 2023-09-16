const SERVER: &str = env!("REMOTE_SERVER");

fn main() {
    let v = env!("TEST");
    println!("Hello, world! {}", v);
}
