/*
 * Christopher Piraino
 *
 * An implementation of the Fibonacci Heap
 * for use in network algorithms.
 */
extern crate collections = "collections#0.11-pre";
use collections::dlist::DList;
use collections::deque::Deque;
use std::option::Option;
use std::cell::RefCell;

#[deriving(Clone)]
struct FibNode<T> {
    parent: Option<~RefCell<FibNode<T>>>,
    children: DList<~RefCell<FibNode<T>>>,
    rank: uint,
    marked: bool,
    value: T
}

pub struct FHeap<T> {
    // Minimum node is the first node in the first tree.
    trees: DList<~RefCell<FibNode<T>>>
}

impl<T: Ord + Clone> FHeap<T> {
    pub fn new() -> FHeap<T> {
        FHeap { 
            trees: DList::new()
        }
    }
    pub fn insert(&mut self, val: T) {
        let node =
            FibNode {
                parent: None,
                children: DList::new(),
                rank: 0,
                marked: false,
                value: val
            };
        let rc_node = ~RefCell::new(node);
        let mut tree = DList::new();
        tree.push_front(rc_node);
        let singleton = 
            FHeap { 
                trees: tree
            };
        self.meld(singleton);
    }
    // Returns a copy of the minimum element.
    pub fn find_min(& self) -> T {
        match self.trees.front() {
            Some(n) => {
                let borrow = n.borrow();
                borrow.deref().value.clone()
            },
            None => fail!("Fibonacci heap is empty")
        }
    }
    //pub fn delete_min(&self) -> T;
    pub fn meld(&mut self, other: FHeap<T>) {
        if self.find_min() <= other.find_min() {
            self.trees.append(other.trees);
        } else {
            self.trees.prepend(other.trees);
        }
    }
    //pub fn decrease_key(&self, val: T, delta: uint) -> bool;
    //pub fn delete(&self, val: T) -> bool;
}


/* 
 *
 * Test Functions
 *
 */

#[test]
fn test_fheap_meld() {
    let node =
        FibNode {
            parent: None,
            children: DList::new(),
            rank: 0,
            marked: false,
            value: 0 
        };
    let mut new_node = node.clone();
    let mut rc_node = ~RefCell::new(node);
    let mut tree = DList::new();
    tree.push_front(rc_node);
    let singleton = 
        FHeap { 
            trees: tree
        };
    tree = DList::new();
    new_node.value = 1;
    rc_node = ~RefCell::new(new_node);
    let child1 = 
        ~RefCell::new(FibNode {
            parent: Some(rc_node.clone()),
            children: DList::new(),
            rank: 0,
            marked: false,
            value: 3
        });
     let child2 = 
        ~RefCell::new(FibNode {
            parent: Some(rc_node.clone()),
            children: DList::new(),
            rank: 0,
            marked: false,
            value: 9
        });
    {
        let mut node_mut = rc_node.borrow_mut();
        node_mut.deref_mut().children.push_front(child1);
        node_mut.deref_mut().children.push_front(child2);
        node_mut.deref_mut().rank = 2;
    }
    tree.push_front(rc_node);
    let mut fheap = FHeap { trees: tree };
    fheap.meld(singleton);
    assert_eq!(fheap.find_min(), 0);
}
