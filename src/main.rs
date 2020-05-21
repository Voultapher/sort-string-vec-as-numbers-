#![feature(test)]

extern crate test;

use rand::{Rng, thread_rng};

fn make_string_vec(len: usize) -> Vec<String> {
    let mut rng = thread_rng();
    (0..len).map(|_| (rng.gen::<i32>().to_string())).collect()
}

fn sort_by_key(vec: &mut Vec<String>) -> usize {
    vec.sort_by_key(|e| e.parse::<i32>().unwrap());

    // Side effects are there only for benchmarking.
    let side_effect = vec[0].len();
    side_effect
}

fn sort_by_cached_key(vec: &mut Vec<String>) -> usize {
    vec.sort_by_cached_key(|e| e.parse::<i32>().unwrap());

    let side_effect = vec[0].len();
    side_effect
}

fn parse_then_sort(input: &[String]) -> i32 {
    let mut parsed_vec: Vec<i32> = input.iter()
        .map(|e| e.parse().unwrap())
        .collect();

    parsed_vec.sort();

    let side_effect = parsed_vec[0];
    side_effect
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn make_vec() {
        assert_eq!(make_string_vec(10).len(), 10);
        assert!(make_string_vec(1)[0].parse::<i32>().is_ok());
    }

    #[test]
    fn sort_equality() {
        let input = make_string_vec(100);

        let mut by_key = input.clone();
        let _ = sort_by_key(&mut by_key);

        let mut by_key_cached = input.clone();
        let _ = sort_by_cached_key(&mut by_key_cached);

        let first_elem = parse_then_sort(input.as_slice());

        assert_eq!(by_key, by_key_cached);
        // This is not an exhaustive test, only here as sanity check.
        assert_eq!(by_key[0], first_elem.to_string());
    }

    const SMALL_SIZE: usize = 10;
    const MEDIUM_SIZE: usize = 1000;
    const LARGE_SIZE: usize = 100_000;

    // Could be done dryer via macro.

    #[bench]
    fn only_clone_small(b: &mut Bencher) {
        let input = make_string_vec(SMALL_SIZE);
        b.iter(|| input.clone());
    }

    #[bench]
    fn sort_by_key_small(b: &mut Bencher) {
        let input = make_string_vec(SMALL_SIZE);
        b.iter(|| sort_by_key(&mut input.clone()));
    }

    #[bench]
    fn sort_by_cached_key_small(b: &mut Bencher) {
        let input = make_string_vec(SMALL_SIZE);
        b.iter(|| sort_by_cached_key(&mut input.clone()));
    }

    #[bench]
    fn parse_then_sort_small(b: &mut Bencher) {
        let input = make_string_vec(SMALL_SIZE);
        b.iter(|| parse_then_sort(&mut input.as_slice()));
    }

    #[bench]
    fn only_clone_medium(b: &mut Bencher) {
        let input = make_string_vec(MEDIUM_SIZE);
        b.iter(|| input.clone());
    }

    #[bench]
    fn sort_by_key_medium(b: &mut Bencher) {
        let input = make_string_vec(MEDIUM_SIZE);
        b.iter(|| sort_by_key(&mut input.clone()));
    }

    #[bench]
    fn sort_by_cached_key_medium(b: &mut Bencher) {
        let input = make_string_vec(MEDIUM_SIZE);
        b.iter(|| sort_by_cached_key(&mut input.clone()));
    }

    #[bench]
    fn parse_then_sort_medium(b: &mut Bencher) {
        let input = make_string_vec(MEDIUM_SIZE);
        b.iter(|| parse_then_sort(&mut input.as_slice()));
    }

    #[bench]
    fn only_clone_large(b: &mut Bencher) {
        let input = make_string_vec(LARGE_SIZE);
        b.iter(|| input.clone());
    }

    #[bench]
    fn sort_by_key_large(b: &mut Bencher) {
        let input = make_string_vec(LARGE_SIZE);
        b.iter(|| sort_by_key(&mut input.clone()));
    }

    #[bench]
    fn sort_by_cached_key_large(b: &mut Bencher) {
        let input = make_string_vec(LARGE_SIZE);
        b.iter(|| sort_by_cached_key(&mut input.clone()));
    }

    #[bench]
    fn parse_then_sort_large(b: &mut Bencher) {
        let input = make_string_vec(LARGE_SIZE);
        b.iter(|| parse_then_sort(&mut input.as_slice()));
    }
}
