#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter {
            count: 0,
            length,
        }
    }
}

fn main() {
}