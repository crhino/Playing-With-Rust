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

pub type FibEntry<K,V> = RefCell<~FibNode<K,V>>;
trait HeapEntry<K, V> {
    fn key(&self) -> K;
    fn value(&self) -> V;
    fn mark(&self) -> bool;
    fn parent(&self) -> Option<Self>;
}

#[deriving(Clone)]
struct FibNode<K, V> {
    parent: Option<FibEntry<K,V>>,
    children: DList<FibEntry<K,V>>,
    marked: bool,
    key: K,
    value: V 
}

pub struct FHeap<K, V> {
    // Minimum node is the first node in the first tree.
    trees: DList<FibEntry<K, V>>
}

// Private Methods on FibEntry.
impl<K: Clone, V: Clone> HeapEntry<K,V> for FibEntry<K,V> {
    fn key(&self) -> K {
        self.borrow().deref().key.clone()
    }
    fn value(&self) -> V {
        self.borrow().deref().value.clone()
    }
    fn mark(&self) -> bool {
        self.borrow().deref().marked
    }
    fn parent(&self) -> Option<FibEntry<K,V>> {
        self.borrow().deref().parent.clone()
    }
}

// Private Methods on Node values.
impl<K,V> FibNode<K,V> {
    // Hack that relies on the fact that the child has
    // already been mutably borrowed, and thus try_borrow()
    // returns None.
    fn remove_child_none_hack(&mut self) {
        for _ in range(0, self.children.len()) {
            if self.children.front().unwrap().try_borrow().is_none() {
                self.children.pop_front();
                break;
            }
            self.children.rotate_backward();
        }
    }
    fn rank(&self) -> uint {
        self.children.len()
    }
}

// Private methods on FHeap.
impl<K: Clone,V: Clone> FHeap<K,V> {
    fn remove_child_none_hack(&mut self, node: FibEntry<K,V>) {
        let mut borrow_mut = node.borrow_mut();
        let deref_mut = borrow_mut.deref_mut();
        deref_mut.remove_child_none_hack();
        if deref_mut.parent.is_some() {
            if deref_mut.marked {
                self.cascading_cut(deref_mut);
            } else {
                deref_mut.marked = true;
            }
        }
    }
    fn cascading_cut(&mut self, node: &mut ~FibNode<K,V>) {
        self.remove_child_none_hack(node.parent.clone().unwrap())
    }
}

impl<K: Clone + Sub<K,K> + Ord, V: Clone> FHeap<K, V> {
    pub fn new() -> FHeap<K, V> {
        FHeap { 
            trees: DList::new() 
        }
    }
    /* In order to ensure O(1) time for delete(node) and decrease_key(node, delta),
     * we need O(1) access to the element in the heap. Thus, insert returns a pointer
     * to the Entry.
     */
    pub fn insert(&mut self, k: K, val: V) -> FibEntry<K,V> {
        let node =
            ~FibNode {
                parent: None,
                children: DList::new(),
                marked: false,
                key: k,
                value: val
            };
        let rc_node = RefCell::new(node);
        let ret = rc_node.clone();
        let mut tree = DList::new();
        tree.push_front(rc_node);
        let singleton = 
            FHeap { 
                trees: tree
            };
        self.meld(singleton);
        ret
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
            let same_rank = || -> Option<(FibEntry<K,V>, FibEntry<K,V>)> {
                // Only a single tree, no linking step.
                if self.trees.len() == 1 {
                    return None;
                }
                for _ in range(0, self.trees.len()) {
                    let front = self.trees.front().unwrap().borrow();
                    let back = self.trees.back().unwrap().borrow();
                    if front.rank() == back.rank() {
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
        if self.trees.is_empty() {
            self.trees.append(other.trees);
        } else if self.find_min().val0() <= other.find_min().val0() {
            self.trees.append(other.trees);
        } else {
            self.trees.prepend(other.trees);
        }
    }
    pub fn decrease_key(&mut self, node: FibEntry<K,V>, delta: K) {
        let mut borrow_mut = node.borrow_mut();
        let deref = borrow_mut.deref_mut();
        let key = deref.key.clone();
        deref.key = key - delta;
        if deref.parent.is_none() {
            return
        }
        let parent = deref.parent.clone().unwrap();
        self.remove_child_none_hack(parent);
        if self.find_min().val0() > deref.key {
            self.trees.push_back(node.clone());
        } else {
            self.trees.push_front(node.clone());
        }
    }
    pub fn delete(&mut self, node: FibEntry<K,V>) -> (K, V) {
        if self.find_min().val0() == node.key() {
            return self.delete_min()
        } else if node.parent().is_none() {
            let key = node.key();
            let value = node.value();
            let borrow = node.borrow();
            let deref = borrow.deref();
            self.trees.append(deref.children.clone());
            (key, value)
        } else {
            let key = node.key();
            let value = node.value();
            let mut borrow_mut = node.borrow_mut();
            let deref_mut = borrow_mut.deref_mut();
            self.remove_child_none_hack(deref_mut.parent.clone().unwrap());
            self.trees.append(deref_mut.children.clone());
            (key, value)
        }
    }
}


/* 
 *
 * Test Functions
 *
 */

#[test]
fn test_fheap_meld() {
    let node =
        ~FibNode {
            parent: None,
            children: DList::new(),
            marked: false,
            key: 0,
            value: 0 
        };
    let mut new_node = node.clone();
    let mut rc_node = RefCell::new(node);
    let mut tree = DList::new();
    tree.push_front(rc_node);
    let singleton = 
        FHeap { 
            trees: tree
        };
    tree = DList::new();
    new_node.value = 1;
    rc_node = RefCell::new(new_node);
    let child1 = 
        RefCell::new(~FibNode {
            parent: Some(rc_node.clone()),
            children: DList::new(),
            marked: false,
            key: 3,
            value: 3
        });
     let child2 = 
        RefCell::new(~FibNode {
            parent: Some(rc_node.clone()),
            children: DList::new(),
            marked: false,
            key: 9,
            value: 9
        });
    {
        let mut node_mut = rc_node.borrow_mut();
        node_mut.deref_mut().children.push_front(child1);
        node_mut.deref_mut().children.push_front(child2);
    }
    tree.push_front(rc_node);
    let mut fheap = FHeap { trees: tree };
    fheap.meld(singleton);
    assert_eq!(fheap.find_min(), (0,0));
}

#[test]
fn test_fheap_insert() {
    let mut fheap = FHeap::new();
    fheap.insert(1, ~"one");
    fheap.insert(4, ~"four");
    assert_eq!(fheap.find_min(), (1, ~"one"));
    fheap.insert(0, ~"zero");
    assert_eq!(fheap.find_min(), (0, ~"zero"));
}

#[test]
fn test_fheap_delete_min() {
    let mut fheap = FHeap::new();
    fheap.insert(1, ~"1");
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

#[test]
fn test_fheap_decrease_key() {
    let mut fheap = FHeap::new();
    fheap.insert(1, ~"1");
    let four = fheap.insert(4, ~"4");
    fheap.insert(0, ~"0");
    let five = fheap.insert(5, ~"5");
    fheap.delete_min();
    fheap.decrease_key(four.clone(), 2);
    let borrow = four.borrow();
    let deref = borrow.deref();
    assert_eq!(deref.key, 2);
    assert!(deref.parent.is_none());
    assert_eq!(fheap.trees.len(), 3);
    fheap.decrease_key(five, 5);
    assert_eq!(fheap.find_min(), (0, ~"five"))
}

#[test]
fn test_fheap_delete() {
    let mut fheap = FHeap::new();
    let one = fheap.insert(1, ~"1");
    let four = fheap.insert(4, ~"4");
    fheap.insert(0, ~"0");
    fheap.insert(5, ~"5");
    fheap.delete_min();
    let (kfour, _) = fheap.delete(four.clone());
    assert_eq!(kfour, 4);
    assert_eq!(fheap.trees.len(), 1);
    fheap.delete(one);
    assert_eq!(fheap.trees.len(), 1);
    assert_eq!(fheap.find_min(), (5, ~"five"))
}

#[test]
fn test_fheap_cascading_cut() {
    let mut fheap = FHeap::new();
    fheap.insert(1, "1");
    fheap.insert(4, "4");
    fheap.insert(0, "0");
    fheap.insert(5, "5");
    fheap.insert(2, "2");
    fheap.insert(3, "3");
    let h6 = fheap.insert(6, "6");
    let h7 = fheap.insert(7, "7");
    fheap.insert(18, "18");
    fheap.insert(9, "9");
    fheap.insert(11, "11");
    fheap.insert(15, "15");
    fheap.delete_min();
    assert_eq!(fheap.find_min(), (1, "1"));
    assert_eq!(fheap.trees.len(), 3);
    fheap.decrease_key(h6.clone(), 2);
    fheap.decrease_key(h7.clone(), 1);
    assert_eq!(fheap.trees.len(), 6);
}
