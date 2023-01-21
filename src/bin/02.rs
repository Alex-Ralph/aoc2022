pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines();
    let output = games.map(|x| {
        let mut choices: Vec<u32> = x.split(' ').map(|x| (x.chars().next().unwrap()) as u32).collect();
        choices[1] = choices[1] - 23;

        let mut winloss = 0;
        if choices[0] == choices[1] {winloss = 3;}
        else {
            if choices[0] == 65{
                if choices[1] == 66 {winloss = 6;}
                else {winloss = 0};
            }
            if choices[0] == 66 {
                if choices[1] == 65 {winloss = 0;}
                else {winloss = 6;}
            }
            if choices[0] == 67 {
                if choices[1] == 65 {winloss = 0;}
                else {winloss = 6;}
            }
        }
        match choices[0] {
            65 => winloss + 1,
            66 => winloss + 2,
            67 => winloss + 3,
            _ => panic!(),
        }
    });
    let sum: u32 = output.sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines();
    let output = games.map(|x| {
        let mut data: Vec<&str> = x.split(' ').collect();
        let mut handpos;
        if data[1] == "X" {
            match data[0] {
                "A" => handpos = 3,
                "B" => handpos = 1,
                "C" => handpos = 2,
                _ => panic!()
            }
        }
        else if data[1] == "Y" {
            match data[0] {
                "A" => handpos = 1,
                "B" => handpos = 2,
                "C" => handpos = 3,
                _ => panic!()
            }
        }
        else {
            match data[0] {
                "A" => handpos = 2,
                "B" => handpos = 3,
                "C" => handpos = 1,
                _ => panic!()
            }
        }
        let mut victory = 0;
        match data[1] {
            "X" => victory = 0,
            "Y" => victory = 3,
            "Z" => victory = 6,
            _ => panic!()
        }
        handpos + victory
    });
    Some(output.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
