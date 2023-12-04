pub trait StringTools {
    fn nums(self) -> Vec<u32>;
}

//TODO: implement this as iterator instead of returning a vec
impl StringTools for &str {
    fn nums(self) -> Vec<u32> {
        let mut current_num: Option<u32> = None;
        let mut result = Vec::new();

        for c in self.chars() {
            if let Some(digit) = c.to_digit(10) {
                if let Some(num) = current_num.as_mut() {
                    *num *= 10;
                    *num += digit;
                } else {
                    current_num = Some(digit);
                }
            } else if let Some(num) = current_num {
                result.push(num);
                current_num = None
            }
        }

        if let Some(num) = current_num {
            result.push(num);
        }

        result
    }
}
