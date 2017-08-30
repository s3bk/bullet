extern crate math;
use math::builder::Builder;

#[test]
fn text_simplify() {
    let pairs = [
        ("1 + 2", "3"),
        ("a / a", "1")
    ];
    let builder = Builder::new();
    for &(a, b) in &pairs {
        assert_eq!(builder.parse(a).unwrap(), builder.parse(b).unwrap());
    }
}
