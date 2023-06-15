use std::collections::HashMap;

const FILESYSTEM: usize = 70_000_000;
const REQUIRED: usize = 30_000_000;

#[derive(Clone, Debug)]
enum Node {
    Directory(HashMap<String, Node>),
    File(usize),
}

impl Node {
    fn size(&self) -> usize {
        match self {
            Node::Directory(children) => children.values().map(Node::size).sum(),
            Node::File(size) => *size,
        }
    }

    fn walk<'a>(&'a mut self, path: &'a [&'a str]) -> &'a mut Self {
        match path {
            [] => self,

            [head, tail @ ..] => {
                if let Node::Directory(children) = self {
                    children.get_mut(head.to_owned()).unwrap().walk(tail)
                } else {
                    panic!("Invalid path: {:?}", path)
                }
            }
        }
    }

    fn flat_children(&self) -> Option<Vec<(&String, &Node)>> {
        match self {
            Node::Directory(children) => Some(
                children
                    .iter()
                    .flat_map(|(name, node)| {
                        let mut children = node.flat_children().unwrap_or(vec![]);
                        children.push((name, node));
                        children
                    })
                    .collect(),
            ),
            Node::File(_) => None,
        }
    }
}

fn main() {
    let mut path = vec![];
    let mut tree = Node::Directory(HashMap::new());

    let input = std::fs::read_to_string("input").unwrap();

    input.trim().lines().for_each(|line| {
        let new_child = match line.split_whitespace().collect::<Vec<_>>()[..] {
            ["$", "ls"] => None,

            ["$", "cd", dir] => match dir {
                "/" => {
                    path.clear();
                    None
                }

                ".." => {
                    path.pop();
                    None
                }

                dir => {
                    path.push(dir);
                    None
                }
            },

            ["dir", name] => Some((name, Node::Directory(HashMap::new()))),

            [size, name] => Some((name, Node::File(size.parse().unwrap()))),

            _ => panic!("Invalid command: {}", &line),
        };

        if let (Some((name, node)), Node::Directory(children)) = (new_child, tree.walk(&path)) {
            children.insert(name.to_owned(), node.to_owned());
        };
    });

    let current_free = FILESYSTEM - tree.size();
    let additional_required = REQUIRED - current_free;

    let result = tree.flat_children().unwrap();
    let mut result = result
        .iter()
        .filter(|(_, node)| {
            matches!(node, Node::Directory(_)) && node.size() >= additional_required
        })
        .map(|(_, node)| node.size())
        .collect::<Vec<_>>();

    result.sort_unstable();

    println!("{:?}", result.first().unwrap());
}
