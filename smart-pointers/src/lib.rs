pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::{Rc, Weak};

    #[test]
    fn box_dereferencing() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }

    #[test]
    fn recursive_type_with_box() {
        use List::{Cons, Nil};
        // A simple cons list allowing recursive types
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        let first = match list {
            Cons(v, _) => v,
            Nil => panic!("Shouldn't be here!"),
        };
        assert_eq!(first, 1);
    }

    #[test]
    fn custom_box_with_deref_coercion() {
        // A custom Box type implementing Deref
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            pub fn new(x: T) -> Self {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(x, 5);
        assert_eq!(*y, 5);

        let m = MyBox::new(String::from("Rust"));
        // We can do this because m is coerced via call to deref into a type we can use
        hello(&m);
    }

    #[test]
    #[allow(unused_variables)]
    fn implementing_drop() {
        // Implementing the Drop trait to run custom behavior on drop
        struct CustomSmartPointer {
            data: String,
        }

        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }

        let c = CustomSmartPointer {
            data: String::from("Something"),
        };
        let d = CustomSmartPointer {
            data: String::from("Another thing"),
        };
        println!("CustomSmartPointers created.");

        // drop() is called on c and d in reverse-allocation order when they go out of scope.
        // We can force it to happen early with std::mem::drop!
        drop(c);
    }

    #[test]
    #[allow(unused_variables)]
    fn ref_counts() {
        enum RcList {
            Cons(i32, Rc<RcList>),
            Nil,
        }

        let a = Rc::new(RcList::Cons(2, Rc::new(RcList::Nil)));
        // Reference count is now 1
        assert_eq!(Rc::strong_count(&a), 1);

        let b = RcList::Cons(3, Rc::clone(&a));
        // Reference count is now 2
        assert_eq!(Rc::strong_count(&a), 2);

        {
            let c = RcList::Cons(4, Rc::clone(&a));
            // Reference count is now 3, but only for as long as this scope!
            assert_eq!(Rc::strong_count(&a), 3);
        }

        // Reference count is back down to 2
        assert_eq!(Rc::strong_count(&a), 2);
    }

    #[test]
    fn interior_mutability() {
        // A mock implementor of Messenger that uses a RefCell for interior mutability
        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> Self {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));

                // This would complie, but panic at runtime, since we aren't allowed
                // multiple mutable borrows of a RefCell at one time!
                // let mut one_borrow = self.sent_messages.borrow_mut();
                // let mut two_borrow = self.sent_messages.borrow_mut();
                // one_borrow.push(String::from(message));
                // two_borrow.push(String::from(message));
            }
        }

        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn multiple_ownership_with_refcell_and_rc() {
        use List::{Cons, Nil};
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    #[test]
    fn reference_cycle_memory_leak() {
        use List::{Cons, Nil};
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }

        // These two lists are manipulated to point at each other, and thus
        // are never dropped as their reference counts never reach zero!
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            // Replace tail of a with b
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    fn tree_with_weak_refs() {
        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
