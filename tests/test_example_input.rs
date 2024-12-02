use day_1::*;

#[test]
fn test_list_distance() {
    let distance = list_distance_sorted(
        vec![3, 4, 2, 1, 3, 3],
        vec![4, 3, 5, 3, 9, 3]
    );

    assert_eq!(distance, 11);
}
