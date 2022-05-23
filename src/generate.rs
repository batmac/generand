#![forbid(unsafe_code)]

use rand::{seq::SliceRandom, seq::IteratorRandom};
use std::iter::repeat_with;

#[inline]
pub fn generate(dictionary: &str, size: usize) -> String {
    generate_vec(&dictionary.chars().collect(), size)
        .iter()
        .collect()
}
#[inline]
pub fn generate_vec<T: Clone>(dictionary: &Vec<T>, size: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();

    repeat_with(|| dictionary.choose(&mut rng).unwrap().clone())
        .take(size)
        .collect()
}
#[inline]
pub fn generate_vec_clone_lately<T: Clone>(dictionary: &Vec<T>, size: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();

    repeat_with(|| dictionary.choose(&mut rng).unwrap())
        .take(size)
        .cloned()
        .collect()
}

#[inline]
pub fn generate_vec_for<T: Clone>(dictionary: &Vec<T>, size: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut v = vec![];

    for _ in 0..size {
        v.push(dictionary.choose(&mut rng).unwrap().clone());
    }

    v
}

#[inline]
pub fn generate_generic<T: Clone + IteratorRandom>(symbols: T, size: usize) -> Vec<T::Item> {
    let mut rng = rand::thread_rng();
    repeat_with(|| symbols.clone().choose(&mut rng).unwrap())
        .take(size)
        .collect()
}

#[inline]
pub fn generate_old<T: AsRef<str>>(symbols: T, size: usize) -> String {
    let symbols = symbols.as_ref();

    let mut rng = rand::thread_rng();
    repeat_with(|| symbols.chars().choose(&mut rng).unwrap())
        .take(size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn test_generate_empty() {
        generate("", 256);
    }
    #[test]
    fn test_generate_trivial() {
        let s = generate("x", 5);
        assert_eq!(s, "xxxxx")
    }
    #[test]
    fn test_generate_simple() {
        let s = generate("abc", 10);
        assert_eq!(s.len(), 10);
        assert_eq!(s.chars().count(), 10);
    }
    #[test]
    fn test_generate_zero() {
        let s = generate("abc", 0);
        assert_eq!(s, "")
    }
    #[test]
    fn test_generate_emojis_trivial() {
        let s = generate("👍", 10);
        assert_eq!(s, "👍👍👍👍👍👍👍👍👍👍")
    }
    #[test]
    fn test_generate_emojis() {
        let s = generate("🏉👍🌱", 10);
        assert_eq!(s.len(), 40);
        assert_eq!(s.chars().count(), 10)
    }

    #[test]
    #[should_panic]
    fn test_generate2_empty() {
        generate_generic("".chars(), 256);
    }
    #[test]
    fn test_generate2_trivial() {
        let s = generate_generic("x".chars(), 5);
        assert_eq!(s, "xxxxx".chars().collect::<Vec<char>>())
    }
    #[test]
    fn test_generate2_simple() {
        let s = generate_generic("abc".chars(), 10);
        assert_eq!(s.len(), 10);
    }
    #[test]
    fn test_generate2_zero() {
        let s = generate_generic("abc".chars(), 0);
        assert_eq!(s, Vec::<char>::new())
    }
    #[test]
    fn test_generate2_emojis_trivial() {
        let s = generate_generic("👍".chars(), 10);
        assert_eq!(s, "👍👍👍👍👍👍👍👍👍👍".chars().collect::<Vec<char>>())
    }
    #[test]
    fn test_generate2_emojis() {
        let s = generate_generic("🏉👍🌱".chars(), 10);
        assert_eq!(s.len(), 10);
    }
}
