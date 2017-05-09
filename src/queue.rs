pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { elements: Vec::<T>::new() }
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    pub fn enqueue(&mut self, elem: T) {
        self.elements.push(elem)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            None
        } else {
            Some(self.elements.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_on_an_empty_queue_returns_none() {
        let queue = Queue::<i32>::new();
        assert!(queue.peek().is_none());
    }

    #[test]
    fn enqueue_adds_an_element_at_the_end_of_the_queue() {
        let mut queue = Queue::new();
        let first_elem = 42;
        let second_elem = 100;

        queue.enqueue(first_elem);
        queue.enqueue(second_elem);

        assert_eq!(first_elem, *queue.peek().expect("Element not found"));
    }

    #[test]
    fn dequeue_takes_the_first_element_of_the_queue() {
        let mut queue = Queue::new();
        let first_elem = 42;
        let second_elem = 100;

        queue.enqueue(first_elem);
        queue.enqueue(second_elem);

        assert_eq!(first_elem, queue.dequeue().expect("Element not found"));
        assert_eq!(second_elem, *queue.peek().expect("Element not found"));
    }

    #[test]
    fn dequeue_returns_none_if_queue_is_empty() {
        let mut queue = Queue::<i32>::new();
        assert!(queue.dequeue().is_none());
    }
}