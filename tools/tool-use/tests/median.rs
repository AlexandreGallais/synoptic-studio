//! The one median of the workspace's reports (AUD-035, F6): `bench`, `metrics` and
//! `context-size` take it from here.

use tool_use::median;

#[test]
fn takes_the_middle_whole_number_or_the_mean_of_the_two_middle_ones_rounded_down() {
    assert_eq!(median::<u64>(&[]), None);
    assert_eq!(median(&[7_u64]), Some(7));
    // Sorted 1, 3, 5: the middle one.
    assert_eq!(median(&[5_u64, 1, 3]), Some(3));
    // Sorted 1, 2, 3, 4: (2 + 3) / 2 = 2.5, rounded down to 2.
    assert_eq!(median(&[4_u64, 1, 3, 2]), Some(2));
    // Sorted 2, 6: (2 + 6) / 2 = 4.
    assert_eq!(median(&[6_u64, 2]), Some(4));
    // Sorted 10, 20, 30, 40: (20 + 30) / 2 = 25, where `context-size` took the lower middle, 20.
    assert_eq!(median(&[40_u64, 10, 30, 20]), Some(25));
    // Two of the largest: their mean, without overflowing their sum.
    assert_eq!(median(&[u64::MAX, u64::MAX]), Some(u64::MAX));
    assert_eq!(median(&[u64::MAX - 1, u64::MAX]), Some(u64::MAX - 1));
}

#[test]
fn takes_the_middle_decimal_or_the_mean_of_the_two_middle_ones() {
    let near = |values: &[f64], expected: f64| {
        median(values).is_some_and(|found| (found - expected).abs() < f64::EPSILON)
    };

    assert_eq!(median::<f64>(&[]), None);
    assert!(near(&[5.0, 1.0, 3.0], 3.0));
    // Sorted 1, 2, 4, 9: (2 + 4) / 2 = 3.
    assert!(near(&[9.0, 1.0, 4.0, 2.0], 3.0));
    // Sorted 1, 2: (1 + 2) / 2 = 1.5, not rounded.
    assert!(near(&[2.0, 1.0], 1.5));
}

#[test]
fn takes_the_same_median_in_any_order() {
    // Every rotation of 1 to 6, forward and backward: (3 + 4) / 2 = 3.5, rounded down to 3.
    let values = [1_u64, 2, 3, 4, 5, 6];

    for start in 0..values.len() {
        let mut rotated = values.to_vec();

        rotated.rotate_left(start);
        assert_eq!(median(&rotated), Some(3), "{rotated:?}");
        rotated.reverse();
        assert_eq!(median(&rotated), Some(3), "{rotated:?}");
    }
}
