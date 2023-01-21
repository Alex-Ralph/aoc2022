pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    Some(lines.filter(|x| {
        let elves: Vec<_> = x.split(",").map(|x|
            x.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()).collect();
        (elves[0][0] >= elves[1][0] && elves[0][1] <= elves[1][1]) //if elves[0] is within elves[1]
        || (elves[0][0] <= elves[1][0] && elves[0][1] >= elves[1][1]) //or the inverse
    }
).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    Some(lines.filter(|x| {
        let elves: Vec<_> = x.split(",").map(|x|
            x.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()).collect();
        (elves[0][0] >= elves[1][0] && elves[0][0] <= elves[1][1]) //if elves[0][0] is within elves[1]
        || (elves[0][1] >= elves[1][0] && elves[0][1] <= elves[1][1]) //or if elves[0][1] is
        || (elves[1][0] >= elves[0][0] && elves[1][0] <= elves[0][1]) //or the other way around
        || (elves[1][1] >= elves[0][0] && elves[1][1] <= elves[0][1])
    }
).count())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_04_pt1() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_04_pt2() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
