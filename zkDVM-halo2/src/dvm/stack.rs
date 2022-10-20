use std::ops;

pub struct Stack {
    stack: Vec<u32>,
    depth: usize,
}

impl Stack {

    pub const INACCESSIBLE_ELEMENTS: usize = 4;

    pub fn new() -> Self {
        // initial stack must have 2 dummy elements at indices 0 and 1 for ease of handling later in constructing ZKP
        // depth must be at least 2
        Self {
            stack: vec![0; Self::INACCESSIBLE_ELEMENTS],
            depth: Self::INACCESSIBLE_ELEMENTS,
        }
    }

    // return current depth of the stack
    pub fn get_depth(&self) -> usize {
        self.depth
    }

    // if depth == stack.len(), possibly need to expand space to push new elements later. Otherwise, just increase self.depth
    fn is_depth_reaching_limit(&self) -> bool {
        if self.depth == self.stack.len() {
            return true;
        }
        false
    }

    // push element to stack
    pub fn push(&mut self, element: u32) {
        if self.is_depth_reaching_limit() {
            self.stack.push(element.clone());
        } else {
            self.stack[self.depth] = element.clone();
        }
        self.depth += 1;
    }

    // get value of last element
    pub fn back(&self) -> u32 {
        self.stack[self.depth - 1]
    }

    // pop element from stack
    pub fn pop(&mut self) -> u32 {
        let last_element = self.back();
        self.depth -= 1;
        last_element
    }

    pub fn is_depth_violating(&self) -> bool {
        self.depth < Self::INACCESSIBLE_ELEMENTS
    }
}

impl ops::Index<usize> for Stack {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stack[index]
    }
}