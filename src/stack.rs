pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { elements: Vec::<T>::new() }
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn push(&mut self, elem: T) {
        self.elements.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_on_an_empty_stack_returns_none() {
      let stack = Stack::<i32>::new();
      assert!(stack.peek().is_none());
    }

    #[test]
    fn push_adds_an_element_at_the_end_of_the_stack() {
      let mut stack = Stack::new();
      let elem = 42;
      stack.push(elem);
      assert_eq!(elem, *stack.peek().expect("Element not found"));
    }

    #[test]
    fn pop_takes_an_element_from_the_end_of_the_list() {
      let mut stack = Stack::new();
      let first_elem = 42;
      let second_elem = 100;

      stack.push(first_elem);
      stack.push(second_elem);

      assert_eq!(second_elem, stack.pop().expect("Element not found"));
      assert_eq!(first_elem, *stack.peek().expect("Element not found"));
    }
}