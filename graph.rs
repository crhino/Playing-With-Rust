/*  Chris Piraino
 *
 *
 *  Implementation of a Graph and
 *  associated algorithms and data
 *  structures.
 *
 */
use std::vec::Vec;
use std::result::Result;

pub trait Graph {
    fn new(vertices: uint) -> ~Self;
    fn adjacent(&self, x: uint, y: uint) -> Result<bool, ~str>;
    fn neighbors(&self, x: uint) -> Vec<uint>;
    fn add(&mut self, x: uint, y: uint, val: int) -> Result<int, ~str>;
    fn delete(&mut self, x: uint, y: uint) -> Result<int, ~str>;
    fn get_edge_value(&self, x: uint, y: uint) -> Result<int, ~str>;
    fn set_edge_value(&mut self, x: uint, y: uint, val: int) -> Result<int, ~str>;
    fn depth_first_search(&self, closure: |graph: &Self, v: uint|, start: uint);
}

pub trait Matrix {
    fn zero(row: uint, col: uint) -> ~Self;
    fn height(&self) -> uint; 
    fn width(&self) -> uint;
    fn at(&self, row: uint, col: uint) -> Result<int, ~str>;
    fn set(&mut self, row: uint, col: uint, val: int) -> Result<int, ~str>;
}

pub struct VectorMatrix {
    elements: Vec<int>, // Elements are stored in row-major order.
    height: uint,
    width: uint
}

impl Matrix for VectorMatrix {
    fn zero(row: uint, col: uint) -> ~VectorMatrix {
        let elem = Vec::from_fn(row*col, |_: uint| -> int { 0 });
        ~VectorMatrix { elements: elem, height: row, width: col }
    }
    fn at(&self, row: uint, col: uint) -> Result<int, ~str> {
        if row >= self.height || col >= self.width {
            Err(format!("({}, {}) is out of bounds, height of matrix is {} and width of matrix is {}.",
                    row, col, self.height, self.width))
        } else {
            Ok(*self.elements.get(row*self.width + col))
        }
    }
    fn set(& mut self, row: uint, col: uint, val: int) -> Result<int, ~str> {
        if row >= self.height || col >= self.width {
            Err(format!("({}, {}) is out of bounds, height of matrix is {} and width of matrix is {}.",
                    row, col, self.height, self.width))
        } else {
            *self.elements.get_mut(row*self.width+col) = val;
            Ok(val)
        }
    }
    fn height(&self) -> uint {self.height}
    fn width(&self) -> uint {self.width}
}

impl Graph for VectorMatrix {
    fn new(vertices: uint) -> ~VectorMatrix {
        Matrix::zero(vertices, vertices)
    }
    fn adjacent(&self, x: uint, y: uint) -> Result<bool, ~str> {
        let res = self.at(x, y);
        match res {
            Ok(n) => if n == 0 { Ok(false) } else { Ok(true) },
            Err(err) => Err(err)
        }
    }
    fn neighbors(&self, x: uint) -> Vec<uint> {
        let row = self.elements.slice(x*self.width, x*self.width+self.width);
        let mut adj = Vec::new();
        for r in row.iter().enumerate() {
            let (i, val) = r;
            if *val != 0 {
                adj.push(i)
            }
        }
        return adj
    }
    fn add(&mut self, x: uint, y: uint, val: int) -> Result<int, ~str> {
        self.set(x, y, val)
    }
    fn delete(&mut self, x: uint, y: uint) -> Result<int, ~str> {
        self.set(x, y, 0)
    }
    fn get_edge_value(&self, x: uint, y: uint) -> Result<int, ~str> {
        self.at(x, y)
    }
    fn set_edge_value(&mut self, x: uint, y: uint, val: int) -> Result<int, ~str> {
        // TODO: If x, y are not adjacent, return Err().
        self.set(x, y, val)
    }
    // Calls the closure on the vertices in DFS order, passing in the graph as well.
    fn depth_first_search(&self, closure: |graph: &VectorMatrix, v: uint|, start: uint) {
        let mut visited = Vec::from_fn(self.width(), |_| 0);
        let mut stack = Vec::new();
        stack.push(start);
        // Continue looping until all vertices are visited.
        while stack.len() != 0 {
            let current = stack.pop().unwrap();
            if *visited.get(current) == 1 {
                continue;
            }
            closure(self, current);
            *visited.get_mut(current) = 1;
            for x in self.neighbors(current).iter() {
                stack.push(*x);
            }
        }
    }
}

#[test]
fn test_matrix_impl() {
    let mut matrix: ~VectorMatrix = Matrix::zero(2,2);
    let mut res = matrix.set(1,1,4);
    assert!(res.is_ok())
    res = matrix.at(1,1);
    assert!(res.is_ok());
    let n = res.ok().unwrap();
    assert_eq!(4, n);
}

#[test]
fn test_matrix_set_bounds() {
    let mut matrix: ~VectorMatrix = Matrix::zero(2,2);
    assert!(matrix.set(2,2,5).is_err());
}

#[test]
fn test_graph_neighbors() {
    let mut graph: ~VectorMatrix = Graph::new(3);
    assert!(!graph.adjacent(1, 2).ok().unwrap());
    let mut res = graph.add(0,1,1);
    assert!(res.is_ok());
    res = graph.add(0,2,1);
    assert!(res.is_ok());
    let neighbors = graph.neighbors(0);
    assert_eq!(neighbors.len(), 2);
    assert_eq!(*neighbors.get(0), 1);
    assert_eq!(*neighbors.get(1), 2);
}

#[test]
fn test_graph_adjacent() {
    let mut graph: ~VectorMatrix = Graph::new(3);
    assert!(!graph.adjacent(1, 2).ok().unwrap());
    let mut res = graph.add(0,1,1);
    assert!(res.is_ok());
    res = graph.add(0,2,1);
    assert!(res.is_ok());
    assert!(graph.adjacent(0,1).ok().unwrap());
    assert!(graph.adjacent(0,2).ok().unwrap());
}
