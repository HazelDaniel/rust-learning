#![allow(unused)]
pub mod basic {
    use std::fmt::{Debug, Display};
    #[derive(Debug)]
    pub struct Node<T> {
        pub value: T,
        pub next: Option<Box<Node<T>>>
    }

    impl<T: Debug + Display> Node<T> {
        pub fn print(&self) -> () {
            loop {
                if let Some(curr_node) = &self.next {
                    print!(" -> {:?} ", curr_node.value);
                    break curr_node.print();
                } else {
                    print!("\n");
                    break ()
                }
            }
        }
    }
}

pub mod with_rc {
    use std::rc::Rc;
    use std::fmt::{Debug, Display};

    #[derive(Debug)]
    pub struct LinkedList<T> {
        pub head: Option<Rc<Node<T>>>,
    }
    #[derive(Debug)]
    pub struct Node<T> {
        pub value: T,
        pub next: Option<Rc<Node<T>>>
    }

    impl<T: Debug + Display> LinkedList<T> {
        pub fn prepend(&self, value: T) -> LinkedList<T> {
            let result_node: Rc<Node<T>> = match &self.head {
                Some(node) => Rc::new(Node { value, next: Some(Rc::clone(&node)) }),
                None =>  Rc::new(Node { value, next: None })
            };

            LinkedList { head: Some(result_node) }
        }

        pub fn print(&self) -> () {
            if let Some(node) = &self.head {
                print!("[<RC linked list>] {:?}", node.value);
                self.print_(Rc::clone(&node));
            }
            println!();
        }

        fn print_(&self, node: Rc<Node<T>>) -> () {
            loop {
                if let Some(next_node) = &node.next {
                    print!(" -> {:?}", next_node.value);
                    break self.print_(Rc::clone(&next_node));
                } else {
                    break ()
                }
            }
        }
    }
}

pub mod with_refcell {
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::fmt::{Debug, Display};

    #[derive(Debug)]
    pub struct LinkedList<T> {
        pub head: Option<Rc<RefCell<Node<T>>>>,
    }
    #[derive(Debug)]
    pub struct Node<T> {
        pub value: T,
        pub next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T: Debug + Display> LinkedList<T> {
        pub fn prepend(&self, value: T) -> LinkedList<T> {
            let new_node: Rc<RefCell<Node<T>>>;

            if let Some(node) = self.head.clone() {
                new_node = Rc::new(RefCell::new(Node {value, next: Some(Rc::clone(&node)) }));
            } else {
                new_node = Rc::new(RefCell::new(Node {value: value, next: None}));
            }

            LinkedList {
                head: Some(Rc::clone(&new_node)),
            }
        }

        pub fn print(&self) -> () {
            if let Some(node) = &self.head {
                let mut curr_node_ref = Rc::clone(&node);

                let curr_node = curr_node_ref.borrow();
                print!("{:?}", curr_node.value);

                let mut next_node_option  = curr_node.next.clone();
                loop {
                    next_node_option = match next_node_option {
                        Some(next_node) => {
                            print!("-> {:?}", next_node.borrow().value);
                            next_node.borrow().next.clone()
                        },
                        _ => None
                    };

                    match next_node_option {
                        None => break,
                        _ => ()
                    }

                }
            }
            println!();
        }

    }
}
