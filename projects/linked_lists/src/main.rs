#![allow(unused)]

use std::rc::Rc;
use utils::index::basic::{Node as BasicNode};
use utils::index::with_rc::{Node as RcNode, LinkedList as RcLinkedList};
use utils::index::with_refcell::{Node as RfcNode, LinkedList as RfcLinkedList};

fn main() {
    let basic_linked_list = BasicNode { value: 1, next: Some(Box::new(BasicNode {value: 2, next: Some(Box::new(BasicNode {value: 3, next: Some(Box::new(BasicNode {value: 4, next: None}))}))}))};
    basic_linked_list.print();

    let rc_linked_list = RcLinkedList {head: None}.prepend(2).prepend(3).prepend(4);
    rc_linked_list.print();

    let rfc_linked_list = RfcLinkedList {head: None}.prepend(2).prepend(3).prepend(4);
    rfc_linked_list.print();

    println!("how many strong counts for the rc linked list ?:  {:?}", (Rc::strong_count(&rc_linked_list.head.unwrap())));
    // println!("how many strong counts for the rc linked list ?:  {:?}", (Rc::strong_count(&rc_linked_list.head.unwrap().next.unwrap().next.unwrap())));

    println!("Hello, world!");
}
