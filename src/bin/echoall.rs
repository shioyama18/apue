use std::env;

fn main() {
    for (i, arg) in env::args().enumerate() {
        println!("argv[{}] = {}", i, arg);
    }

    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }
}
