pub fn parse_text(input: &str) -> Option<(Vec<Vec<char>>, Vec<Vec<usize>>)> {
//fuck only knows if that return will work
//TODO clean, add question marks
let mut a = input.split("\r\n\r\n");
    let (mut stacks, mut instructions) = (a.next()?.lines().rev(), a.next()?);
    let stackamt = stacks.next()?.replace(" ", "").len();
    let mut vecstacks = Vec::new();
    for i in 0..stackamt {
        vecstacks.push(Vec::new());
    }
    for line in stacks {
        let linevec: Vec<_> = line.chars().collect();
        for i in 0..stackamt {
            let char = linevec[4*i+1];
            if char != ' ' {
                vecstacks[i].push(char);
            }
        }
    }

    let binding = String::from(instructions.replace("move ", "")
        .replace("from ", "")
        .replace("to ", ""));
    let instructioniter: Vec<Vec<usize>> = binding.split("\r\n")
        .map(|x| x.split(" ")
            .map(|y| y.parse::<usize>()
                .unwrap())
            .collect::<Vec<_>>())
        .collect();

    Some((vecstacks, instructioniter))
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, mut instructions) = parse_text(input)?;

    for x in instructions.iter() {
        for i in 0..x[0] {
            let value = stacks[x[1]-1].pop()?;
            stacks[x[2]-1].push(value);
        }
    }

    let mut answer = String::from("");
    for mut a in stacks.into_iter() {
        answer.push(a.pop()?);
    }
    Some(answer)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, mut instructions) = parse_text(input)?;

    for x in instructions.iter() {
        let mut crates = Vec::new();
        for i in 0..x[0] {
            crates.push(stacks[x[1]-1].pop()?);
        }
        let mut crateslist = crates.into_iter().rev().collect();
        stacks[x[2]-1].append(&mut crateslist);
    }
    let mut answer = String::from("");
    for mut a in stacks.into_iter() {
        answer.push(a.pop()?);
    }
    Some(answer)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
