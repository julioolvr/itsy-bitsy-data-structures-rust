use std::mem;

pub struct List<T> {
    elements: Vec<T>,
    length: usize,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { elements: Vec::<T>::new(), length: 0 }
    }

    pub fn get(&self, n: usize) -> &T {
        &self.elements[n]
    }

    pub fn push(&mut self, elem: T) {
        self.elements.push(elem);
        self.length += 1;
    }

    pub fn pop(&mut self) -> T {
        if self.length == 0 {
            panic!("Can't pop from an empty List")
        }

        self.length -= 1;
        self.elements.remove(self.length)
    }

    pub fn unshift(&mut self, elem: T) {
        let mut previous = elem;

        for i in 0..self.length {
            previous = mem::replace(&mut self.elements[i], previous);
        }

        self.elements.push(previous);
        self.length += 1;
    }

    pub fn shift(&mut self) -> T {
        self.length -= 1;

        // This feels like cheating but I don't think I can do it in some other way.
        // I _have_ to take ownership of the element I want to return, so I can't just
        // take a reference, and I can't take ownership without removing it from the Vec<T>
        // and I can't leave an empty space to implement the shifting to the left
        // manually. I think.
        self.elements.remove(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_get_element() {
        let mut list = List::new();
        let elem = 42;
        list.push(elem);
        assert_eq!(elem, *list.get(0));
        assert_eq!(list.length, 1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn get_panics_if_index_not_present() {
        let list: List<i32> = List::new();
        list.get(0);
    }

    #[test]
    fn push_and_pop() {
        let mut list = List::new();
        let elem = 42;
        list.push(elem);
        assert_eq!(elem, list.pop());
        assert_eq!(list.length, 0);
    }

    #[test]
    #[should_panic(expected = "Can't pop from an empty List")]
    fn pop_panics_if_list_empty() {
        let mut list: List<i32> = List::new();
        list.pop();
    }

    #[test]
    fn unshift_adds_element_at_the_beginning() {
        let mut list = List::new();
        let elem = 42;
        list.push(1);
        list.unshift(elem);
        assert_eq!(elem, *list.get(0));
        assert_eq!(list.length, 2);
    }

    #[test]
    fn shift_takes_element_from_the_first_position() {
        let mut list = List::new();
        let elem = 42;
        let remaining_elem = 100;
        list.push(elem);
        list.push(remaining_elem);
        assert_eq!(elem, list.shift());
        assert_eq!(remaining_elem, *list.get(0));
        assert_eq!(list.length, 1);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn shift_panics_if_list_is_empty() {
        let mut list: List<i32> = List::new();
        list.shift();
    }
}
