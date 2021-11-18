use std::thread::spawn;

fn main() {
    let t = spawn(||{});
    t.join().unwrap();
}