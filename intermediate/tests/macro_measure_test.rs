#[test]
fn test_basic() {
    use intermediate::measure_time;
    let result: i32 = measure_time!(2 + 2);
    assert_eq!(result, 4);
}

#[test]
fn test_loop() {
    use intermediate::measure_time;
    let result: i32 = measure_time!({
        let mut sum = 0;
        for i in 0..100 { sum += i; }
        sum
    });
    assert_eq!(result, 4950);
}

#[test]
fn test_string() {
    use intermediate::measure_time;
    let s: String = measure_time!("hello".to_string());
    assert_eq!(s, "hello");
}

#[test]
fn test_bool() {
    use intermediate::measure_time;
    let b: bool = measure_time!(true);
    assert!(b);
}

#[test]
fn test_negative() {
    use intermediate::measure_time;
    let result: i32 = measure_time!(-42);
    assert_eq!(result, -42);
}

#[test]
fn test_multiplication() {
    use intermediate::measure_time;
    let result: i32 = measure_time!(6 * 7);
    assert_eq!(result, 42);
}

#[test]
fn test_zero() {
    use intermediate::measure_time;
    let result: i32 = measure_time!(0);
    assert_eq!(result, 0);
}

#[test]
fn test_float() {
    use intermediate::measure_time;
    let result: f64 = measure_time!(3.14);
    assert!((result - 3.14).abs() < f64::EPSILON);
}

#[test]
fn test_vec() {
    use intermediate::measure_time;
    let v: Vec<i32> = measure_time!(vec![1, 2, 3]);
    assert_eq!(v, vec![1, 2, 3]);
}
