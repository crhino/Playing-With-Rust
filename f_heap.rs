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
    pub fn new(val: T) -> FHeap<T> {
        let node = FibNode { 
            parent: None,
            children: DList::new(),
            rank: 0,
            marked: false,
            value: val
        };
        let rc_node = ~RefCell::new(node);
        let mut tree = DList::new();
        tree.push_front(rc_node);
        FHeap { 
            trees: tree 
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
    pub fn delete_min(&mut self) -> T {
        let mut min_tree = self.trees.pop_front().unwrap().unwrap();
        let value = min_tree.value.clone();
        for n in min_tree.children.mut_iter() {
            let mut mut_borrow = n.borrow_mut();
            mut_borrow.deref_mut().parent = None;
        }
        self.trees.append(min_tree.children);

        // Explicit closure scope.
        {
            // Closure to find to trees with the same rank.
            let same_rank = || -> Option<(~RefCell<FibNode<T>>, ~RefCell<FibNode<T>>)> {
                // Only a single tree, no linking step.
                if self.trees.len() == 1 {
                    return None;
                }
                for _ in range(0, self.trees.len()) {
                    let front = self.trees.front().unwrap().borrow();
                    let back = self.trees.back().unwrap().borrow();
                    if front.rank == back.rank {
                        return Some((self.trees.pop_front().unwrap(), 
                                     self.trees.pop_back().unwrap()));
                    }
                    self.trees.rotate_backward();
                }
                return None;
            };
            let mut link = same_rank();
            while link.is_some() {
                let (a, b) = link.unwrap();
                let mut tree = a.borrow_mut();
                {
                    let mut b_mut = b.borrow_mut();
                    b_mut.marked = false;
                }
                tree.deref_mut().children.push_front(b);
                tree.deref_mut().rank+=1;
                link = same_rank();
            }
        }
        // Find the minimum node and put the tree first.
        let mut min_node = self.trees.pop_front().unwrap();
        for _ in range(0, self.trees.len()) {
            if self.trees.front().unwrap().borrow().value.lt(&min_node.borrow().value) {
                self.trees.push_back(min_node);
                min_node = self.trees.pop_front().unwrap();
            } else {
                self.trees.rotate_backward();
            }
        }
        self.trees.push_front(min_node);
        // Return the minimum value.
        value
    }
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

#[test]
fn test_fheap_insert() {
    let mut fheap = FHeap::new(1);
    fheap.insert(4);
    assert_eq!(fheap.find_min(), 1);
    fheap.insert(0);
    assert_eq!(fheap.find_min(), 0);
}

#[test]
fn test_fheap_delete_min() {
    let mut fheap = FHeap::new(1);
    fheap.insert(4);
    fheap.insert(0);
    fheap.insert(5);
    fheap.delete_min();
    assert_eq!(fheap.find_min(), 1);
    assert_eq!(fheap.trees.len(), 2);
    assert_eq!(fheap.delete_min(), 1);
    assert_eq!(fheap.find_min(), 4);
    assert_eq!(fheap.trees.len(), 1);
}
