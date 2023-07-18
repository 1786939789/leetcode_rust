pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    fn merge_sort(nums: &mut Vec<i32>, tmps: &mut Vec<i32>, left: usize, right: usize) -> i32 {
        if left >= right {
            return 0;
        }
        let mid = (left + right) / 2;
        let mut count = merge_sort(nums, tmps, left, mid) + merge_sort(nums, tmps, mid + 1, right);

        let mut i = left;
        let mut j = mid + 1;
        for k in left..=right {
            tmps[k] = nums[k];
        }

        for k in left..=right {
            if i == mid + 1 {
                nums[k] = tmps[j];
                j += 1;
            } else if j == right + 1 || tmps[i] <= tmps[j] {
                nums[k] = tmps[i];
                i += 1;
            } else {
                nums[k] = tmps[j];
                j += 1;
                count += (mid - i + 1) as i32;
            }
        }
        count
    }

    let mut nums = nums.clone();
    let len = nums.len();
    let right = if len == 0 { 0 } else { len - 1 };
    let mut tmps: Vec<i32> = vec![0; len];

    return merge_sort(&mut nums, &mut tmps, 0, right);
}