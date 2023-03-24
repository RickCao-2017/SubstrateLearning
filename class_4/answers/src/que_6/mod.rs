
pub fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in nums {
        match sum.checked_add(num) {
            Some(val) => sum = val,
            None => return None,
        }
    }
    Some(sum)
}