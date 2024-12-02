// https://adventofcode.com/2024/day/1

// Compare the sum distance between two sorted lists
pub fn list_distance_sorted(mut first: Vec<i32>, mut second: Vec<i32>) -> i32 {
    // Sort lists
    first.sort();
    second.sort();

    // TODO: Make lists of equal length by padding with zeros.

    first.iter()
        // Use the index
        .enumerate()
        // Form a tuple of each value from both lists
        .map(|(index, &value)| (value, second[index]))
        // Calculate the distance between the numbers
        .map(|(a, b)| a - b)
        // Distance is expressed as a positive value
        .map(|a| a.abs())
        // Snag the total of all distances
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_distance() {
        let distance = list_distance_sorted(
            vec![3, 4, 2, 1, 3, 3],
            vec![4, 3, 5, 3, 9, 3]
        );

        assert_eq!(distance, 11);
    }
}
