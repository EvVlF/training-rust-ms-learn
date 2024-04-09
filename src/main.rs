fn main() {
    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    let b: Option<&str> = None;
    b.expect("fruits are healthy"); // panics with `fruits are healthy`
}