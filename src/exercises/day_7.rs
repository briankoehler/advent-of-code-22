use crate::utils::{Solution, read_lines, solution_check_test};

#[derive(Debug, Clone)]
enum Node {
    File(File),
    Directory(Directory),
}

impl Node {
    fn to_file_ref(&self) -> Result<&File, Box<dyn std::error::Error>> {
        match &self {
            Node::File(f) => Ok(f),
            _ => Err("Not a file".into()),
        }
    }

    fn to_directory_ref(&self) -> Result<&Directory, Box<dyn std::error::Error>> {
        match &self {
            Node::Directory(d) => Ok(d),
            _ => Err("Not a Directory".into()),
        }
    }
    
    fn to_directory_mut(&mut self) -> Result<&mut Directory, Box<dyn std::error::Error>> {
        match self {
            Node::Directory(d) => Ok(d),
            _ => Err("Not a Directory".into()),
        }
    }

    fn get_name(&self) -> String {
        match &self {
            Node::File(f) => f.name.clone(),
            Node::Directory(d) => d.name.clone(),
        }
    }

    fn get_index(&self) -> usize {
        match &self {
            Node::File(f) => f.index,
            Node::Directory(d) => d.index,
        }
    }

    fn get_parent_index(&self) -> Option<usize> {
        match &self {
            Node::File(f) => Some(f.parent_index),
            Node::Directory(d) => d.parent_index,
        }
    }
}

#[derive(Debug, Clone)]
struct File {
    pub name: String,
    pub index: usize,
    pub parent_index: usize,
    pub size: u32,
}

#[derive(Debug, Clone)]
struct Directory {
    pub name: String,
    pub index: usize,
    pub parent_index: Option<usize>,
    pub children_indices: Vec<usize>,
}

impl Directory {
    fn get_child_index(&self, name: &str, container: &Vec<Node>) -> Option<usize> {
        for index in &self.children_indices {
            if container[*index].get_name() == name.to_string() {
                return Some(*index)
            }
        }
        None
    }
}

fn dfs(directory: &Directory, sum: &mut u32, nodes_container: &Vec<Node>) -> u32 {
    let mut size: u32 = 0;
    for child_index in &directory.children_indices {
        let node = &nodes_container[*child_index];
        size += match node {
            Node::File(f) => f.size,
            Node::Directory(d) => dfs(d, sum, nodes_container),
        }
    }
    if size <= 100000 {
        *sum += size;
    }
    size
}

fn dfs_2(directory: &Directory, nodes_container: &Vec<Node>, sizes: &mut Vec<u32>) -> u32 {
    let mut size: u32 = 0;
    for child_index in &directory.children_indices {
        let node = &nodes_container[*child_index];
        size += match node {
            Node::File(f) => f.size,
            Node::Directory(d) => dfs_2(d, nodes_container, sizes),
        }
    }
    sizes.push(size);
    size
}

pub struct Day7 {
    input_path: String
}

impl Solution for Day7 {
    fn new(input_path: String) -> Self {
        Self { input_path }
    }

    fn part_1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let lines: Vec<_> = read_lines(&self.input_path)?
            .map(|l| l.expect("Could not parse line"))
            .collect();

        let root = Node::Directory(Directory {
            name: "/".into(),
            index: 0,
            parent_index: None,
            children_indices: vec![],
        });
        let mut nodes_container: Vec<Node> = vec![root];
        
        let mut current_node: usize = 0;
        let mut i = 1;
        while i < lines.len() {
            let tokens: Vec<_> = lines[i].split(' ').collect();

            match tokens[1] {
                "cd" => {
                    if tokens[2] == ".." {
                        current_node = nodes_container[current_node].get_parent_index().unwrap();
                    }
                    else {
                        current_node = nodes_container[current_node].to_directory_ref()?
                            .get_child_index(tokens[2], &nodes_container).unwrap();
                    }
                    i += 1;
                },
                "ls" => {
                    i += 1;
                    while i < lines.len() && !lines[i].starts_with('$') {
                        let tokens: Vec<_> = lines[i].split(' ').collect();
                        
                        if tokens[0] == "dir" {
                            let new_directory = Directory {
                                name: tokens[1].into(),
                                index: nodes_container.len(),
                                parent_index: Some(nodes_container[current_node].get_index()),
                                children_indices: vec![],
                            };
                            nodes_container[current_node].to_directory_mut()?.children_indices.push(new_directory.index);
                            nodes_container.push(Node::Directory(new_directory));
                        }
                        else {
                            let new_file = File {
                                name: tokens[1].into(),
                                index: nodes_container.len(),
                                parent_index: nodes_container[current_node].get_index(),
                                size: tokens[0].parse()?,
                            };
                            nodes_container[current_node].to_directory_mut()?.children_indices.push(new_file.index);
                            nodes_container.push(Node::File(new_file));
                        }
                        i += 1;
                    }
                },
                _ => panic!("Unexpected command token"),
            }
        }

        let root = nodes_container[0].to_directory_ref()?;

        let mut sum: u32 = 0;
        dfs(root, &mut sum, &nodes_container);
        Ok(sum.to_string())
    }

    fn part_2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let lines: Vec<_> = read_lines(&self.input_path)?
            .map(|l| l.expect("Could not parse line"))
            .collect();

        let root = Node::Directory(Directory {
            name: "/".into(),
            index: 0,
            parent_index: None,
            children_indices: vec![],
        });
        let mut nodes_container: Vec<Node> = vec![root];
        
        let mut current_node: usize = 0;
        let mut i = 1;
        while i < lines.len() {
            let tokens: Vec<_> = lines[i].split(' ').collect();

            match tokens[1] {
                "cd" => {
                    if tokens[2] == ".." {
                        current_node = nodes_container[current_node].get_parent_index().unwrap();
                    }
                    else {
                        current_node = nodes_container[current_node].to_directory_ref()?
                            .get_child_index(tokens[2], &nodes_container).unwrap();
                    }
                    i += 1;
                },
                "ls" => {
                    i += 1;
                    while i < lines.len() && !lines[i].starts_with('$') {
                        let tokens: Vec<_> = lines[i].split(' ').collect();
                        
                        if tokens[0] == "dir" {
                            let new_directory = Directory {
                                name: tokens[1].into(),
                                index: nodes_container.len(),
                                parent_index: Some(nodes_container[current_node].get_index()),
                                children_indices: vec![],
                            };
                            nodes_container[current_node].to_directory_mut()?.children_indices.push(new_directory.index);
                            nodes_container.push(Node::Directory(new_directory));
                        }
                        else {
                            let new_file = File {
                                name: tokens[1].into(),
                                index: nodes_container.len(),
                                parent_index: nodes_container[current_node].get_index(),
                                size: tokens[0].parse()?,
                            };
                            nodes_container[current_node].to_directory_mut()?.children_indices.push(new_file.index);
                            nodes_container.push(Node::File(new_file));
                        }
                        i += 1;
                    }
                },
                _ => panic!("Unexpected command token"),
            }
        }

        let root = nodes_container[0].to_directory_ref()?;
        let mut sizes: Vec<u32> = vec![];
        let utilization = dfs_2(root, &nodes_container, &mut sizes);

        let needed_space = i32::abs(70000000 - utilization as i32 - 30000000);
        sizes.sort();
        for size in sizes {
            if size > needed_space as u32 {
                return Ok(size.to_string())
            }
        }
        Ok("".into())
    }
}

solution_check_test!(Day7, "day_7.txt", "95437".to_string(), "24933642".to_string());
