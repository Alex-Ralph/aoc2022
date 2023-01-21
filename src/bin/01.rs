pub fn part_one(input: &str) -> Option<u32> {
    let elves = input.split("\r\n\r\n");
    let mut highest_cals = 0;
    for elf in elves {
        let snacks = elf.lines();
        let mut calories = 0;
        for snack in snacks {
            calories += snack.parse::<i32>().unwrap();
        }
        if calories > highest_cals {
            highest_cals = calories;
        }
    }
    Some(highest_cals as u32)

}

//make list of all calorie totals and then sort through the list for the top three
pub fn part_two(input: &str) -> Option<u32> {
    let elves = input.split("\r\n\r\n");
    let mut calorielist: Vec<i32> = vec![];
    for elf in elves {
        let snacks = elf.lines();
        let mut calories = 0;
        for snack in snacks {
            calories += snack.parse::<i32>().unwrap();
        }
        calorielist.push(calories);
    }
    calorielist.sort_unstable();
    Some((calorielist.pop().unwrap() + calorielist.pop().unwrap() + calorielist.pop().unwrap()) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
