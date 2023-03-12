// nums = { 0, 1, 2, 2, 2, 3, 4, 5 }
//                   X: Remaining elements in the array are copied here.
// nums = { 0, 1, 2, 3, 4, 5, ?, ? }
#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut count = nums.len();
    let mut begin_index = 0;
    let mut end_index = 1;
    while end_index < count {
        if nums[begin_index] != nums[end_index] {
            let length = end_index - begin_index;
            if length == 1 {
                begin_index = end_index;
                end_index += 1;
            } else {
                // Copy the remaining array elements to begin_index + 1
                let target_begin_index = begin_index + 1;
                copy(end_index, count, target_begin_index, nums);
                let duplicate_count = length - 1;
                count -= duplicate_count;
                begin_index = target_begin_index;
                end_index = begin_index + 1;
            }
        } else {
            end_index += 1;
        }
    }

    let length = count - begin_index;
    if length > 1 {
        // For example,
        //                    *     *
        // nums =   [1, 2, 3, 4, 4, X]
        // Indexes:  0  1  2  3  4, 5
        count = begin_index + 1;
    }

    count as i32
}

fn copy(
    source_begin_index: usize,
    source_end_index: usize,
    mut target_begin_index: usize,
    v: &mut [i32],
) {
    for index in source_begin_index..source_end_index {
        v[target_begin_index] = v[index];
        target_begin_index += 1;
    }
}
