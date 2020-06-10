use std::collections::HashMap;

fn _mean2<T>(nums: T) -> Option<f64>
where
    T: Iterator<Item = i32>,
{
    let mut sum = 0;
    let mut len = 0usize;
    for n in nums {
        sum += n;
        len += 1;
    }

    if len != 0 {
        Some(sum as f64 / len as f64)
    } else {
        None
    }
}

pub fn mean(nums: &[i32]) -> Option<f64> {
    if nums.is_empty() {
        None
    } else {
        let sum: i32 = nums.iter().sum();
        Some(sum as f64 / nums.len() as f64)
    }
}

pub fn median(nums: &[i32]) -> Option<i32> {
    if nums.is_empty() {
        None
    } else {
        let mut buf = nums.to_vec(); // only difference!
        buf.sort_unstable();
        Some(buf[buf.len() / 2])
    }
}

pub fn mode(nums: &[i32]) -> Option<i32> {
    let mut freqs = HashMap::new();
    for &n in nums {
        *freqs.entry(n).or_insert(0u32) += 1;
    }

    freqs.into_iter().max_by_key(|&(_n, f)| f).map(|(n, _f)| n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        // Also the functions are more ergonomic to use since you don't have to clone the vector
        // every time.
        let vec = Vec::new();
        assert_eq!(mean(&vec), None);
        assert_eq!(median(&vec), None);
        assert_eq!(mode(&vec), None);
    }

    #[test]
    fn test_singleton() {
        let vec = vec![-5];
        assert_eq!(mean(&vec), Some(-5.));
        assert_eq!(median(&vec), Some(-5));
        assert_eq!(mode(&vec), Some(-5));
    }

    #[test]
    fn test_general_case() {
        let vec = vec![-5, 5, 5, 100, 54];
        assert_eq!(mean(&vec), Some(31.8));
        assert_eq!(median(&vec), Some(5));
        assert_eq!(mode(&vec), Some(5));
    }
}
