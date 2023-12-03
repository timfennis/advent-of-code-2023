use std::fmt::Display;

#[macro_export]
macro_rules! create_solution {
    ($name:ident,$year:literal,$day:literal) => {
        pub struct $name {
            answer: $crate::puzzle::Answer,
            pub year: i32,
            pub day: u8,
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    answer: $crate::puzzle::Answer {
                        part_1: None,
                        part_2: None,
                    },
                    year: $year,
                    day: $day,
                }
            }
        }
        impl $crate::puzzle::Answerable for $name {
            fn answer(&self) -> &$crate::puzzle::Answer {
                &self.answer
            }

            fn answer_mut(&mut self) -> &mut $crate::puzzle::Answer {
                &mut self.answer
            }
        }
    };
}
pub trait Solution {
    fn handle_input(&mut self, _input: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
pub trait Answerable {
    fn answer(&self) -> &Answer;
    fn answer_mut(&mut self) -> &mut Answer;

    fn submit_part1<T: Display>(&mut self, val: T) {
        self.answer_mut().part1(format!("{}", val))
    }

    fn submit_part2<T: Display>(&mut self, val: T) {
        self.answer_mut().part2(format!("{}", val))
    }
}

#[derive(Default)]
pub struct Answer {
    pub part_1: Option<String>,
    pub part_2: Option<String>,
}

impl Answer {
    pub fn part1(&mut self, answer: String) {
        self.part_1 = Some(answer);
    }

    pub fn part2(&mut self, answer: String) {
        self.part_2 = Some(answer)
    }
    pub fn get_part1(&self) -> Option<&str> {
        self.part_1.as_deref()
    }

    pub fn get_part2(&self) -> Option<&str> {
        self.part_2.as_deref()
    }
}
