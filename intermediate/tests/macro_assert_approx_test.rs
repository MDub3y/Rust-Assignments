#[test]
fn test_equal() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.0, 1.0);
}

#[test]
fn test_close() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);
}

#[test]
#[should_panic]
fn test_not_close() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.0, 2.0);
}

#[test]
fn test_custom_epsilon() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.0, 1.05, 0.1);
}

#[test]
fn test_zero_equal() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(0.0, 0.0);
}

#[test]
fn test_negative_equal() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(-1.0, -1.0);
}

#[test]
#[should_panic]
fn test_far_apart_panics() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(0.0, 100.0);
}

#[test]
fn test_tight_epsilon() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.000001, 1.000002, 0.001);
}

#[test]
fn test_large_values_equal() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1000.0, 1000.0);
}

#[test]
#[should_panic]
fn test_slightly_off_default_epsilon_panics() {
    use intermediate::assert_approx_eq;
    assert_approx_eq!(1.0, 1.0 + 1e-5);
}
