use std::marker::PhantomData;
use std::ops;
use std::str::Chars;

pub trait StringTools {
    fn nums<T>(&self) -> Nums<T>;
}

pub struct Nums<'a, T> {
    phantom: PhantomData<T>,
    iter: Chars<'a>,
}

impl<'a, T> Iterator for Nums<'a, T>
where
    T: ops::Add<Output = T> + ops::Mul<Output = T> + From<u32> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_num: Option<T> = None;

        for c in self.iter.by_ref() {
            if let Some(digit) = c.to_digit(10) {
                if let Some(num) = current_num.as_mut() {
                    *num = *num * T::from(10);
                    *num = *num + T::from(digit);
                } else {
                    current_num = Some(T::from(digit));
                }
            } else if current_num.is_some() {
                return current_num;
            }
        }

        current_num
    }
}

impl StringTools for &str {
    fn nums<T>(&self) -> Nums<'_, T> {
        Nums {
            phantom: PhantomData,
            iter: self.chars(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::StringTools;
    use itertools::Itertools;

    #[test]
    fn nums() {
        let str = "this is 1 string with a bunch of numbers 123 234 345    128$5\n\n8";
        let nums = str.nums::<u32>().collect_vec();

        assert_eq!(nums, vec![1, 123, 234, 345, 128, 5, 8]);
    }
}
