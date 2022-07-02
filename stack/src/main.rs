struct Stack {
    items: [u8; 32],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: [0; 32],
        }
    }

    fn push(&mut self, num: u8) -> &Self {
        self.items.rotate_right(1);
        self.items[0] = num;

        self
    }

    fn pop(&mut self) -> &Self {
        self.items[0] = 0;
        self.items.rotate_left(1);

        self
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("{:?}", stack.items);

    stack.pop();
    stack.pop();
    stack.pop();

    println!("{:?}", stack.items);
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn push_test() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.items[0], 3);
    }
}
