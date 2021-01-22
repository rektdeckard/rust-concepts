pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn new(element: T) -> Self {
        Self {
            head: Some(Box::new(Node {
                element,
                next: None,
            })),
        }
    }

    pub fn push(&mut self, element: T) {
        // Equivalent to std::mem::replace(&mut self.head, None);
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<T> {
        // Map can be used as a shorthand for matching and returning Some/None
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(n) => Some(&n.element),
            None => None,
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = LinkedList {
            head: Some(Box::new(Node {
                element: 1023,
                next: None,
            })),
        };
        assert_eq!(list.head.unwrap().element, 1023);
    }

    #[test]
    fn constructor_works() {
        let list = LinkedList::new(42);
        assert_eq!(list.head.unwrap().element, 42);
    }

    #[test]
    fn can_push() {
        let mut list = LinkedList::empty();
        list.push(11);
        assert_eq!(list.head.unwrap().element, 11);
    }

    #[test]
    fn can_pop() {
        let mut list = LinkedList {
            head: Some(Box::new(Node {
                element: "one",
                next: Some(Box::new(Node {
                    element: "two",
                    next: None,
                })),
            })),
        };
        assert_eq!(list.pop(), Some("one"));
        assert_eq!(list.pop(), Some("two"));
    }

    #[test]
    fn can_peek() {
        let list = LinkedList::new(13.33);
        assert_eq!(list.peek(), Some(&13.33));
    }

    #[test]
    fn push_and_pop() {
        let mut list = LinkedList::empty();
        list.push(1);
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(5);
        list.push(8);

        let _ = list.pop();
        let five = list.pop();

        list.push(42);
        let _ = list.pop();
        let three = list.pop();

        assert_eq!(five, Some(5));
        assert_eq!(three, Some(3));
    }
}
