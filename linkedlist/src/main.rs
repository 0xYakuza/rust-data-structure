use std::process::ExitCode;

#[derive(Debug)]
struct LinkedList<'a>{
    nodes: &'a Node,
}

impl <'a> LinkedList<'a> {
    pub fn new(nodes: &'a Node) -> Self {
        Self {
            nodes,
        }
    }
}

#[derive(Debug)]
struct Node {
    val: i32,

    // Need a box for infinite size
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            next: None
        }
    }

    fn node(&mut self, val: i32) -> &mut Self {
        self.next = Some(Box::new(Self::new(val)));

        self
    }
}

fn main() -> ExitCode {
    let mut nodes = Node::new(0);
    nodes.node(1).node(2);

    let list = LinkedList::new(&nodes);

    println!("{:?}", list);

    ExitCode::SUCCESS
}
