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
struct FibNode<K, V> {
    parent: Option<~RefCell<FibNode<K, V>>>,
    children: DList<~RefCell<FibNode<K, V>>>,
    rank: uint,
    marked: bool,
    key: K,
    value: V 
}

pub struct FHeap<K, V> {
    // Minimum node is the first node in the first tree.
    trees: DList<~RefCell<FibNode<K, V>>>
}

impl<K: Clone + Ord, V: Clone> FHeap<K, V> {
    pub fn new(k: K, val: V) -> FHeap<K, V> {
        let node = FibNode { 
            parent: None,
            children: DList::new(),
            rank: 0,
            marked: false,
            key: k, 
            value: val
        };
        let rc_node = ~RefCell::new(node);
        let mut tree = DList::new();
        tree.push_front(rc_node);
        FHeap { 
            trees: tree 
        }
    }
    pub fn insert(&mut self, k: K, val: V) {
        let node =
            FibNode {
                parent: None,
                children: DList::new(),
                rank: 0,
                marked: false,
                key: k,
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
    // Returns a copy of the minimum key and value.
    pub fn find_min(& self) -> (K, V) {
        match self.trees.front() {
            Some(n) => {
                let borrow = n.borrow();
                let deref = borrow.deref();
                (deref.key.clone(), deref.value.clone())
            },
            None => fail!("Fibonacci heap is empty")
        }
    }
    pub fn delete_min(&mut self) -> (K, V) {
        let mut min_tree = self.trees.pop_front().unwrap().unwrap();
        let value = min_tree.value.clone();
        let key = min_tree.key.clone();
        for n in min_tree.children.mut_iter() {
            let mut mut_borrow = n.borrow_mut();
            mut_borrow.deref_mut().parent = None;
        }
        self.trees.append(min_tree.children);

        // Explicit closure scope.
        {
            // Closure to find to trees with the same rank.
            let same_rank = || -> Option<(~RefCell<FibNode<K,V>>, ~RefCell<FibNode<K,V>>)> {
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
            if self.trees.front().unwrap().borrow().key.lt(&min_node.borrow().key) {
                self.trees.push_back(min_node);
                min_node = self.trees.pop_front().unwrap();
            } else {
                self.trees.rotate_backward();
            }
        }
        self.trees.push_front(min_node);
        // Return the minimum value.
        (key, value)
    }
    pub fn meld(&mut self, other: FHeap<K, V>) {
        if self.find_min().val0() <= other.find_min().val0() {
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
            key: 0,
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
            key: 3,
            value: 3
        });
     let child2 = 
        ~RefCell::new(FibNode {
            parent: Some(rc_node.clone()),
            children: DList::new(),
            rank: 0,
            marked: false,
            key: 9,
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
    assert_eq!(fheap.find_min(), (0,0));
}

#[test]
fn test_fheap_insert() {
    let mut fheap = FHeap::new(1, ~"one");
    fheap.insert(4, ~"four");
    assert_eq!(fheap.find_min(), (1, ~"one"));
    fheap.insert(0, ~"zero");
    assert_eq!(fheap.find_min(), (0, ~"zero"));
}

#[test]
fn test_fheap_delete_min() {
    let mut fheap = FHeap::new(1, ~"1");
    fheap.insert(4, ~"4");
    fheap.insert(0, ~"0");
    fheap.insert(5, ~"5");
    fheap.delete_min();
    assert_eq!(fheap.find_min(), (1, ~"1"));
    assert_eq!(fheap.trees.len(), 2);
    assert_eq!(fheap.delete_min(), (1, ~"1"));
    assert_eq!(fheap.find_min(), (4,~"4"));
    assert_eq!(fheap.trees.len(), 1);
}
