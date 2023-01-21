pub struct Node {
    parent: Option<usize>,
    children: Vec<usize>,
    files: usize,
    name: String
}

impl Node {
    pub fn new(name: String, parent: Option<usize>) -> Node {
         Node {
             parent: parent,
             children: Vec::new(),
             files: 0,
             name: name
         }
    }
    pub fn get_nested_filesize(&self, nodes: &Vec<Node>) -> usize {
        if self.children.len() == 0 {
            self.files
        }
        else {
            self.children.iter().fold(self.files, |acc, x| acc + nodes[*x].get_nested_filesize(nodes))
        }
    }
}

pub fn build_tree(input: &str) -> Vec<Node> {
    let mut lines = input.lines();
    let mut arena: Vec<Node> = Vec::new();
    arena.push(Node::new(String::from("/"), None)); //root node
    lines.next();
    let mut cur_node = 0;
    for line in lines {
        let spline: Vec<&str> = line.split(" ").collect();
        match spline[0] {
            "$" => {
                if spline[1] == "cd" {
                    if spline[2] == ".." { //go up a directory
                        cur_node = arena[cur_node].parent.unwrap();
                    }
                    else { //find the child directory in the current directory's list of children, then go there
                        cur_node = *arena[cur_node].children.iter()
                        .filter(|x| arena[**x].name == spline[2]).next().unwrap(); //we have entered pointer hell
                        //I think this does it? idk
                    }
                }
            },
            "dir" => { //add a new directory to the child
                let new_addr = arena.len();
                arena.push(Node::new(String::from(spline[1]), Some(cur_node)));
                arena[cur_node].children.push(new_addr);
            },
            _ => {//In this case the line must be a file
                arena[cur_node].files += spline[0].parse::<usize>().unwrap();
            }
        }
    }
    arena
}

pub fn part_one(input: &str) -> Option<usize> {
    let tree = build_tree(input);
    let mut output = 0;
    for node in &tree {
        let nodesize = node.get_nested_filesize(&tree);
        if nodesize < 100000 {output += nodesize;}
    }
    Some(output)

}

pub fn part_two(input: &str) -> Option<usize> {
    let tree = build_tree(input);
    let mut sizes: Vec<usize> = tree.iter().map(|x| x.get_nested_filesize(&tree)).collect();
    let total_space = 70000000;
    let required_space = 30000000;
    let used_space = sizes[0];
    let to_clear = required_space - (total_space - used_space);
    println!("To clear: {}", to_clear);
    sizes.sort();
    for a in sizes {
        if a > to_clear {
            return Some(a);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
