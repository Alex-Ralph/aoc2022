pub fn part_one(input: &str) -> Option<u32> {
    let a = String::from(input);
    let b = a.chars().map(|x| x as u32).collect::<Vec<_>>();
    let mut input_windows = b.windows(4);
    let mut counter = 0;
    for a in input_windows {
        let mut b = a.to_owned();
        b.sort();
        b.dedup();
        if b.len() == 4 {
            break;
        }
        else {
            counter += 1;
        }
    }
    Some(counter + 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let a = String::from(input);
    let b = a.chars().map(|x| x as u32).collect::<Vec<_>>();
    let mut input_windows = b.windows(14);
    let mut counter = 0;
    for a in input_windows {
        let mut b = a.to_owned();
        b.sort();
        b.dedup();
        if b.len() == 14 {
            break;
        }
        else {
            counter += 1;
        }
    }
    Some(counter + 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
