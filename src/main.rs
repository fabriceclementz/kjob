fn main() {
    // init logs
    // init CLI
    println!("Hello, world!");
}

pub struct Job {
    name: String,
    image: String,
    command: String,
    namespace: Option<String>,
}
