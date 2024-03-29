#[allow(dead_code)]
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut sorted_numbers = nums;
    sorted_numbers.sort();
    let mut closest = sorted_numbers[0] + sorted_numbers[1] + sorted_numbers[2];
    for index in 0..sorted_numbers.len() {
        let mut j = index + 1;
        let mut k = sorted_numbers.len() - 1;
        while j < k {
            let sum = sorted_numbers[index] + sorted_numbers[j] + sorted_numbers[k];
            if target == sum {
                return target;
            }

            if sum < target {
                closest = get_closest(sum, closest, target);
                j += 1;
            } else {
                closest = get_closest(sum, closest, target);
                k -= 1;
            }
        }
    }

    closest
}

fn get_closest(candidate: i32, current: i32, target: i32) -> i32 {
    if candidate == target {
        return target;
    }

    let difference_candidate = i32::abs(target - candidate);
    let difference_current = i32::abs(target - current);
    if difference_candidate < difference_current {
        candidate
    } else {
        current
    }
}

#[allow(dead_code)]
pub fn brute_force(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    let mut closest_number = nums[0] + nums[1] + nums[2];
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            for k in 0..nums.len() {
                if (i != j) && (i != k) && (j != k) {
                    let sum = nums[i] + nums[j] + nums[k];
                    closest_number = get_closest(sum, closest_number, target);
                }
            }
        }
    }

    closest_number
}
