#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let left_index = find_equal(nums, 0, val);
    if left_index.is_none() {
        return nums.len() as i32;
    }

    let right_index = find_reverse_not_equal(nums, nums.len() - 1, val);
    if right_index.is_none() {
        return 0;
    }

    // For example,
    // val = 5
    // Indexes: 0 1 2 3 4 5 6 7 8 9
    // nums:    1 2 3 5 5 6 5 1 1 2
    //                L           R
    // For example,
    // val = 5
    // Indexes: 0 1 2 3 4 5 6 7 8 9
    // nums:    2 2 3 0 0 6 0 1 5 1
    //                          L R
    // For example,
    // val = 5
    // Indexes: 0 1 2 3 4 5 6 7 8 9
    // nums:    2 2 3 0 0 6 0 1 1 5
    //                          R L
    let mut left_index = left_index.unwrap();
    let mut right_index = right_index.unwrap();
    while left_index < right_index {
        nums.swap(left_index, right_index);
        let li = find_equal(nums, left_index + 1, val);
        left_index = li.unwrap();
        let ri = find_reverse_not_equal(nums, right_index - 1, val);
        right_index = ri.unwrap();
    }

    left_index as i32
}

fn find_equal(v: &[i32], begin_index: usize, value: i32) -> Option<usize> {
    for (index, x) in v.iter().enumerate().skip(begin_index) {
        if *x == value {
            return Some(index);
        }
    }

    None
}

fn find_reverse_not_equal(v: &[i32], begin_index: usize, value: i32) -> Option<usize> {
    let mut index = begin_index;
    loop {
        if v[index] != value {
            return Some(index);
        }

        let previous_index = index;
        index = index.saturating_sub(1);
        if previous_index == index {
            return None;
        }
    }
}
