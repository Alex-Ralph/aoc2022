pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut total = 0;

    for line in lines {
        let mut item = 0;
        let pockets = line.split_at(line.len()/2);
        let pocket_one = pockets.0.chars().map(|x| x as u32); //iterator
        let pocket_two: Vec<u32> = pockets.1.chars().map(|x| x as u32).collect(); //vector
        for a in pocket_one {
            if pocket_two.contains(&a) {
                item = a;
                break;
            }
        }
        total += match item {
            97..=122 => item - 96, //lowercase
            65..=90 => item - 38,
            _ => panic!()
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().peekable();
    let mut total = 0;
    while lines.peek().is_some() {
        let mut elf_one = lines.next().unwrap().chars().map(|x| x as u32);
        let elf_two: Vec<u32> = lines.next().unwrap().chars().map(|x| x as u32).collect();
        let elf_three: Vec<u32> = lines.next().unwrap().chars().map(|x| x as u32).collect();
        for a in elf_one {
            if elf_two.contains(&a) && elf_three.contains(&a) {
                total += match a {
                    97..=122 => a - 96, //lowercase
                    65..=90 => a - 38,
                    _ => panic!()
                };
                break;
            }
        }
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_03_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
