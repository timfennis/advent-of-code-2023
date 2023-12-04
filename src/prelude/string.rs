use std::str::Chars;

pub trait StringTools {
    fn nums(&self) -> Nums;
}

pub struct Nums<'a> {
    iter: Chars<'a>,
}

impl<'a> Iterator for Nums<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_num: Option<u32> = None;

        for c in self.iter.by_ref() {
            if let Some(digit) = c.to_digit(10) {
                if let Some(num) = current_num.as_mut() {
                    *num *= 10;
                    *num += digit;
                } else {
                    current_num = Some(digit);
                }
            } else if current_num.is_some() {
                return current_num;
            }
        }

        current_num
    }
}

impl StringTools for &str {
    fn nums(&self) -> Nums<'_> {
        Nums { iter: self.chars() }
    }
}
