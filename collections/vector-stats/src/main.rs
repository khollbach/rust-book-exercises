use std::collections::HashMap;

mod slices;

fn main() {
    slices::mean(&vec![]);
    slices::median(&vec![]);
    slices::mode(&vec![]);
}

pub fn mean(nums: Vec<i32>) -> Option<f64> {
    if nums.is_empty() {
        None
    } else {
        let sum: i32 = nums.iter().sum();
        Some(sum as f64 / nums.len() as f64)
    }
}

pub fn median(nums: Vec<i32>) -> Option<i32> {
    if nums.is_empty() {
        None
    } else {
        let mut buf = nums.clone();
        buf.sort_unstable();
        Some(buf[buf.len() / 2])
    }
}

pub fn mode(nums: Vec<i32>) -> Option<i32> {
    let mut freqs: HashMap<i32, u32> = HashMap::new();
    for n in nums {
        freqs.insert(n, freqs.get(&n).unwrap_or(&0) + 1);
    }

    freqs.into_iter().max_by_key(|&(_n, f)| f).map(|(n, _f)| n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let vec = Vec::new();
        assert_eq!(mean(vec.clone()), None);
        assert_eq!(median(vec.clone()), None);
        assert_eq!(mode(vec.clone()), None);
    }

    #[test]
    fn test_singleton() {
        let vec = vec![-5];
        assert_eq!(mean(vec.clone()), Some(-5.));
        assert_eq!(median(vec.clone()), Some(-5));
        assert_eq!(mode(vec.clone()), Some(-5));
    }

    #[test]
    fn test_general_case() {
        let vec = vec![-5, 5, 5, 100, 54];
        assert_eq!(mean(vec.clone()), Some(31.8));
        assert_eq!(median(vec.clone()), Some(5));
        assert_eq!(mode(vec.clone()), Some(5));
    }
}
