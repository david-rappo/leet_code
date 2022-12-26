mod list_node;
mod solution;
mod test_utility;

#[cfg(test)]
mod tests {
    use crate::solution;
    use crate::test_utility;
    
    const MERGE_TWO_LISTS_PARAMETER_1: [&[i32]; 6] = [&[1, 2, 4],
        &[],
        &[],
        &[0],
        &[10, 11],
        &[]];
    const MERGE_TWO_LISTS_PARAMETER_2: [&[i32]; 6] = [&[1, 3, 4],
        &[],
        &[0],
        &[],
        &[],
        &[10, 11]];
    const MERGE_TWO_LISTS_EXPECTED_RESULT: [&[i32]; 6] = [&[1, 1, 2, 3, 4, 4],
        &[],
        &[0],
        &[0],
        &[10, 11],
        &[10, 11]];

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(MERGE_TWO_LISTS_PARAMETER_1.len(), MERGE_TWO_LISTS_PARAMETER_2.len());
        assert_eq!(MERGE_TWO_LISTS_PARAMETER_2.len(), MERGE_TWO_LISTS_EXPECTED_RESULT.len());
        for index in 0..MERGE_TWO_LISTS_PARAMETER_1.len() {
            let parameter_1 = MERGE_TWO_LISTS_PARAMETER_1[index];
            let parameter_2 = MERGE_TWO_LISTS_PARAMETER_2[index];
            let list_1 = test_utility::create_list(parameter_1.to_vec());
            let list_2 = test_utility::create_list(parameter_2.to_vec());
            let result = solution::merge_two_lists(list_1, list_2);
            let result = test_utility::get_list_values(&result);
            let expected_result = MERGE_TWO_LISTS_EXPECTED_RESULT[index];
            assert_eq!(result, expected_result);
        }
    }
}