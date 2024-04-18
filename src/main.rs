struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 25, y: true };
}