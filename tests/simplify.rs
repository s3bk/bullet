extern crate bullet;
use bullet::builder::Builder;

#[test]
fn text_simplify() {
    let pairs = [
        ("1 + 2", "3"),
        ("a / a", "1"),
        ("a ^ b", "exp(b * log(a))")
    ];
    let builder = Builder::new();
    for &(a, b) in &pairs {
        assert_eq!(builder.parse(a).unwrap(), builder.parse(b).unwrap());
    }
}
