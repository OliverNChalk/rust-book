use testing;

#[test]
fn test_rectangle() {
    let smaller = testing::Rectangle::new(15, 20);
    let larger = testing::Rectangle::new(30, 30);

    assert!(larger.can_hold(&smaller));
}
