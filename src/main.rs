use std::collections::HashMap;

fn main() {
    println!("the answer for part 1 is {}", calculate(&vec![14, 1, 17, 0, 3, 20], 2020));
    println!("the answer for part 2 is {}", calculate(&vec![14, 1, 17, 0, 3, 20], 30_000_000));
}

fn calculate(start: &[u64], target: u64) -> u64 {
    let mut round = 0u64;
    let mut spoken: HashMap<u64, u64> = HashMap::new();
    let mut last: u64;
    while round < target -1 && round < start.len() as u64 -1 {
        last = start[round as usize];
        spoken.insert(last, round as u64);
        round += 1;
    }
    last = *start.last().unwrap();
    while round < target -1 {
        if spoken.contains_key(&last) {
            let diff = round - spoken[&last];
            spoken.insert(last, round);
            last = diff;
        } else {
            spoken.insert(last, round);
            last = 0;
        }
        round +=1;
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_is_correct() {
        assert_eq!(0, calculate(&vec![0, 3, 6], 10));
        assert_eq!(436, calculate(&vec![0, 3, 6], 2020));
        assert_eq!(1, calculate(&vec![1, 3, 2], 2020));
        assert_eq!(1836, calculate(&vec![3, 1, 2], 2020));
    }
}
