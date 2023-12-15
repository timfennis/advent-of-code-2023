use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use std::collections::VecDeque;
create_solution!(Day15, 2023, 15);

impl Solution for Day15 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        self.submit_part1(solve(input));
        self.submit_part2(solve_part2(input));
        Ok(())
    }
}

fn solve(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn solve_part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    for part in input.split(',') {
        if let Some((label, _)) = part.split_once('-') {
            let num = hash(label);
            let b = boxes.get_mut(num).expect("box must exist");
            if let Some(existing_pos) = b.iter().position(|(l, _)| *l == label) {
                b.remove(existing_pos);
            }
        } else if let Some((label, fl)) = part.split_once('=') {
            let num = hash(label);
            let b = boxes.get_mut(num).expect("box must exist");
            let fl = fl
                .parse::<usize>()
                .expect("focal length must be a valid usize");

            if let Some(pos) = b.iter().position(|(lab, _)| *lab == label) {
                b[pos] = (label, fl)
            } else {
                b.push((label, fl));
            }
        } else {
            panic!("no = or -");
        }
    }

    let mut sum = 0;
    for (bi, b) in boxes.iter().enumerate() {
        let bi = bi + 1;
        for (li, (_, fl)) in b.iter().enumerate() {
            sum += bi * (li + 1) * fl;
        }
    }

    sum
}
fn hash(input: &str) -> usize {
    let mut cur = 0usize;
    for char in input.chars() {
        cur += char as usize;
        cur *= 17;
        cur %= 256;
    }

    cur
}

#[cfg(test)]
mod test {
    use crate::year2023::day15::{hash, solve, solve_part2};

    #[test]
    fn foobar() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
    }

    #[test]
    fn foobar_example() {
        assert_eq!(
            solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );

        assert_eq!(
            solve_part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            145
        );
    }
}
