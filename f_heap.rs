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
use std::cast;

pub type FibEntry<K,V> = *mut FibNode<K,V>;
trait HeapEntry<K, V> {
    fn key(&self) -> K;
    fn value(&self) -> V;
    fn mark(&self) -> bool;
    fn parent(&self) -> Option<Self>;
    fn rank(&self) -> uint;
    fn link(&self, child: Self);
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
impl<K: std::fmt::Show+Eq+Clone, V: Clone> HeapEntry<K,V> for FibEntry<K,V> {
    fn key(&self) -> K {
        unsafe { (**self).key.clone() }
    }
    fn value(&self) -> V {
        unsafe { (**self).value.clone() }
    }
    fn mark(&self) -> bool {
        unsafe { (**self).marked }
    }
    fn parent(&self) -> Option<FibEntry<K,V>> {
        unsafe { (**self).parent.clone() }
    }
    fn rank(&self) -> uint {
        unsafe { (**self).rank() }
    }
    fn link(&self, child: FibEntry<K,V>) {
        unsafe { 
            (*child).marked = false;
            (*child).parent = Some(*self);
            (**self).children.push_back(child);
        }
    }
}

// Private Methods on Node values.
impl<K: std::fmt::Show+Eq+Clone ,V: Clone> FibNode<K,V> {
    fn remove_child(&mut self, key: K) {
        for _ in range(0, self.children.len()) {
            print!("Child key: {}\n", self.children.front().unwrap().key());
            if self.children.front().unwrap().key().eq(&key) {
                self.children.pop_front();
                return;
            }
            self.children.rotate_backward();
        }
        fail!("Failed to remove child with key: {}\n", key);
    }
    fn rank(&self) -> uint {
        self.children.len()
    }
}

// Private methods on FHeap.
impl<K: std::fmt::Show+Eq+Clone,V: Clone> FHeap<K,V> {
    fn remove_child(&mut self, node: FibEntry<K,V>, key: K) {
        print!("FHeap.remove_child({}, {})\n", node, key);
        unsafe { (*node).remove_child(key); }
        if node.parent().is_some() {
            if node.mark() {
                print!("start cascading cut\n");
                self.cascading_cut(node);
            } else {
                print!("first removed node, mark node\n");
                unsafe { (*node).marked = true; }
            }
        }
    }
    fn cascading_cut(&mut self, node: FibEntry<K,V>) {
        self.remove_child(node.parent().unwrap(), node.key());
        self.trees.push_back(node)
    }
    fn same_rank(&mut self) -> Option<(FibEntry<K,V>, FibEntry<K,V>)> {
        // Only a single tree, no linking step.
        if self.trees.len() == 1 {
            return None;
        }
        print!("Looking for roots of same rank\n");
        let mut same = false;
        for i in range(0, self.trees.len()) {
            print!("On {}th iteration of self.trees loop\n", i);
            {
                let front = self.trees.front().unwrap();
                let back = self.trees.back().unwrap();
                if front.rank() == back.rank() {
                    same = true;   
                }
            }
            if same {
                return Some((self.trees.pop_front().unwrap(), 
                             self.trees.pop_back().unwrap()));
            }
            self.trees.rotate_backward();
            print!("End of {}th iteration\n", i);
        }
        return None;
    }

}

impl<K: Clone + Sub<K,K> + Ord + std::fmt::Show, V: Clone> FHeap<K, V> {
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
        let ptr: *mut FibNode<K,V> = unsafe { cast::transmute(node) };
        let ret = ptr.clone();
        let mut tree = DList::new();
        tree.push_front(ptr);
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
                (n.key(), n.value())
            },
            None => fail!("Fibonacci heap is empty")
        }
    }
    pub fn delete_min(&mut self) -> (K, V) {
        let min_tree = self.trees.pop_front().unwrap();
        let value = min_tree.value();
        let key = min_tree.key();
        print!("Setting parent of children to None\n");
        unsafe {
            for n in (*min_tree).children.mut_iter() {
                (**n).parent = None;
            }
            print!("Appending children to root list\n");
            self.trees.append((*min_tree).children.clone());
        }
        //let mut dlist = DList::new();
        // Explicit closure scope.
        {
            // Closure to find to trees with the same rank.
            let mut link = self.same_rank();
            while link.is_some() {
                print!("merging roots of same rank\n");
                let (a, b) = link.unwrap();
                if a.key().lt(&b.key()) {
                    print!("less: {} more: {}\n", a.key(), b.key());
                    a.link(b);
                    self.trees.push_front(a);
                } else {
                    print!("less: {} more: {}\n", b.key(), a.key());
                    b.link(a);
                    self.trees.push_front(b);
                }
                link = self.same_rank();
            }
        }
        // Append all newly formed roots to list of roots.
        //self.trees.append(dlist);
        // Find the minimum node and put the tree first.
        print!("Finding minimum node and putting at front\n");
        let mut min_node = self.trees.pop_front().unwrap();
        print!("min node: {}, self.trees.len(): {}\n", min_node.key(), self.trees.len());
        for _ in range(0, self.trees.len()) {
            print!("min_node: {} tree.front: {}\n", min_node.key(), self.trees.front().unwrap().key());
            if self.trees.front().unwrap().key().lt(&min_node.key()) {
                self.trees.push_back(min_node);
                min_node = self.trees.pop_front().unwrap();
            } else {
                self.trees.rotate_backward();
            }
        }
        self.trees.push_front(min_node);
        // Drop ptr and return the minimum value.
        unsafe { drop(cast::transmute::<_, ~FibNode<K,V>>(min_tree)); }
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
        unsafe { (*node).key = (*node).key - delta; }
        if node.parent().is_none() {
            print!("node: {} has no parent\n", node.key());
            return
        }
        print!("parent is Some\n");
        print!("parent: {}\n", node.parent());
        let parent = node.parent().unwrap();
        self.remove_child(parent, node.key());
        if self.find_min().val0().lt(&node.key()) {
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
            for _ in range(0, self.trees.len()) {
                if self.trees.front().unwrap().key().eq(&key) {
                    self.trees.pop_front();
                    break;
                }
                self.trees.rotate_backward();
            }
            let value = node.value();
            unsafe {
                self.trees.append((*node).children.clone());
                drop(cast::transmute::<_, ~FibNode<K,V>>(node));
            }
            (key, value)
        } else {
            let key = node.key();
            let value = node.value();
            self.remove_child(node.parent().unwrap(), node.key());
            unsafe {
                self.trees.append((*node).children.clone());
                drop(cast::transmute::<_, ~FibNode<K,V>>(node));
            }
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
    let new_node = ~FibNode {
            parent: None,
            children: DList::new(),
            marked: false,
            key: 1,
            value: 1 
        };
    unsafe {
    let mut ptr = cast::transmute(node);
    let mut tree = DList::new();
    tree.push_front(ptr);
    let singleton = 
        FHeap { 
            trees: tree
        };
    tree = DList::new();
    ptr = cast::transmute(new_node);
    let child1 = 
        cast::transmute(~FibNode {
            parent: Some(ptr.clone()),
            children: DList::new(),
            marked: false,
            key: 3,
            value: 3
        });
     let child2 = 
        cast::transmute(~FibNode {
            parent: Some(ptr.clone()),
            children: DList::new(),
            marked: false,
            key: 9,
            value: 9
        });
    (*ptr).children.push_front(child1);
    (*ptr).children.push_front(child2);
    tree.push_front(ptr);
    let mut fheap = FHeap { trees: tree };
    fheap.meld(singleton);
    assert_eq!(fheap.find_min(), (0,0));
    }
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
    assert_eq!(fheap.trees.len(), 4);
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
    fheap.decrease_key(four, 2);
    assert_eq!(four.key(), 2);
    assert!(four.parent().is_none());
    assert_eq!(fheap.trees.len(), 2);
    fheap.decrease_key(five, 5);
    assert_eq!(fheap.trees.len(), 3);
    assert_eq!(fheap.find_min(), (0, ~"5"))
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
    assert_eq!(fheap.find_min(), (5, ~"5"))
}

#[test]
fn test_fheap_cascading_cut() {
    let mut fheap = FHeap::new();
    fheap.insert(1, "1");
    fheap.insert(4, "4");
    fheap.insert(0, "0");
    fheap.insert(5, "5");
    fheap.insert(2, "2");
    let h3 = fheap.insert(3, "3");
    let h6 = fheap.insert(6, "6");
    fheap.insert(7, "7");
    fheap.insert(18, "18");
    fheap.insert(9, "9");
    fheap.insert(11, "11");
    fheap.insert(15, "15");
    fheap.delete_min();
    assert_eq!(fheap.find_min(), (1, "1"));
    assert_eq!(fheap.trees.len(), 3);
    fheap.decrease_key(h6, 2);
    fheap.decrease_key(h3, 3);
    assert_eq!(fheap.trees.len(), 6);
}
