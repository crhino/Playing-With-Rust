/*  Chris Piraino
 *
 *
 *  Implementation of a Graph and
 *  associated algorithms and data
 *  structures.
 *
 */
extern crate collections;

use std::vec;
use std::result::Result;
use collections::hashmap::HashMap;
use std::hash;

trait Graph<T> {
    fn new(vert: uint) -> ~Self;
    fn adjacent(&self, x: &T, y: &T) -> Result<bool, ~str>;
    fn neighbors<'a>(&'a self, x: &T) -> Result<~[&'a T], ~str>;
    fn add(&mut self, x: T, y: T, val: int) -> Result<int, ~str>;
    fn delete(&mut self, x: &T, y: &T) -> Result<int, ~str>;
    fn get_edge_value(&self, x: &T, y: &T) -> Result<int, ~str>;
    fn set_edge_value(&mut self, x: &T, y: &T, val: int) -> Result<int, ~str>;
}

trait Matrix {
    fn zero(row: uint, col: uint) -> ~Self;
    fn height(&self) -> uint; 
    fn width(&self) -> uint;
    fn at(&self, row: uint, col: uint) -> Result<int, ~str>;
    fn set(&mut self, row: uint, col: uint, val: int) -> Result<int, ~str>;
}

struct VectorMatrix {
    elements: ~[int], // Elements are stored in row-major order.
    height: uint,
    width: uint
}

impl Matrix for VectorMatrix {
    fn zero(row: uint, col: uint) -> ~VectorMatrix {
        let elem = vec::from_fn(row*col, |_: uint| -> int { 0 });
        ~VectorMatrix { elements: elem, height: row, width: col }
    }
    fn at(&self, row: uint, col: uint) -> Result<int, ~str> {
        if row >= self.height || col >= self.width {
            Err(format!("({}, {}) is out of bounds, height of matrix is {} and width of matrix is {}.",
                    row, col, self.height, self.width))
        } else {
            Ok(self.elements[row*self.width + col])
        }
    }
    fn set(& mut self, row: uint, col: uint, val: int) -> Result<int, ~str> {
        if row >= self.height || col >= self.width {
            Err(format!("({}, {}) is out of bounds, height of matrix is {} and width of matrix is {}.",
                    row, col, self.height, self.width))
        } else {
            unsafe {
                self.elements.unsafe_set(row*self.width+col, val);
            }
            Ok(val)
        }
    }
    fn height(&self) -> uint {self.height}
    fn width(&self) -> uint {self.width}
}

struct AdjacencyMatrixGraph<T> {
    matrix: ~VectorMatrix,
    vertices: HashMap<T, uint>,
    size: uint
}

/*
impl<T: hash::Hash + Eq + std::fmt::Show> AdjacencyMatrixGraph<T> {
    fn vertex_to_uint(&self, x: &T) -> Result<uint, ~str> {
        let opt_u = self.vertices.find(x);
        if opt_u.is_none() {    
            Err(format!("Vertex ({}) was not found in the graph", x))
        } else {
            Ok(*(opt_u.unwrap()))
        }
    }
}
*/

impl<T: hash::Hash + Eq + std::fmt::Show> Graph<T> for AdjacencyMatrixGraph<T> {
    fn new(vert: uint) -> ~AdjacencyMatrixGraph<T> {
        let m: ~VectorMatrix = Matrix::zero(vert, vert);
        let v = HashMap::<T, uint>::with_capacity(vert);
        ~AdjacencyMatrixGraph {matrix: m, vertices: v, size: vert}
    }
    fn adjacent(&self, x: &T, y: &T) -> Result<bool, ~str> {
        let opt_u = self.vertices.find(x);
        let opt_w = self.vertices.find(y);
        match (opt_u, opt_w) {
            (None, _) => return Err(format!("Vertex x: ({}) was not found in the graph", x)),
            (_, None) => return Err(format!("Vertex y: ({}) was not found in the graph", y)),
            (_, _) => { }
        }
        let res = self.matrix.at(*(opt_u.unwrap()), *(opt_w.unwrap()));
        match res {
            Ok(n) => if n == 0 { Ok(false) } else { Ok(true) },
            Err(err) => Err(err)
        }
    }
    fn neighbors<'a>(&'a self, x: &T) -> Result<~[&'a T], ~str> {
        let opt_u = self.vertices.find(x);
        if opt_u.is_none() {
            return Err(format!("Vertex x: ({}) was not found in the graph", x));
        }
        let u = *(opt_u.unwrap());
        let row = self.matrix.elements.slice(u*self.matrix.width, 
                                             (u+1)*self.matrix.width);
        // Builds vector of uints, which represent vertices that are
        // adjacent to vertex x.
        let build = |push: |v: uint|| {
            for r in row.iter().enumerate() {
                let (i, val) = r;
                if *val != 0 {
                    push(i)
                }
            }
        };
        let verts_uint = vec::build(None, build);
        // Filter_map on vertices' Entries to build a vector ~[&'a T].
        // If the value exists in the verts_uint vector, that key is a
        // neighbor and thus is added to the ~[&'a T].
        Ok(self.vertices.iter().filter_map(|(k, v)| {
            for i in verts_uint.iter() {
                if i == v {
                    return Some(k);
                }
            }
            None
        }).to_owned_vec())
    }
    fn add(&mut self, x: T, y: T, val: int) -> Result<int, ~str> {
        let len = self.vertices.len(); 
        let x_val = *self.vertices.find_or_insert(x, len);
        let len = self.vertices.len(); 
        let y_val = *self.vertices.find_or_insert(y, len);
        self.matrix.set(x_val, y_val, val)
    }
    fn delete(&mut self, x: &T, y: &T) -> Result<int, ~str> {
        let opt_u = self.vertices.find(x);
        let opt_w = self.vertices.find(y);
        match (opt_u, opt_w) {
            (None, _) => return Err(format!("Vertex x: ({}) was not found in the graph", x)),
            (_, None) => return Err(format!("Vertex y: ({}) was not found in the graph", y)),
            (_, _) => { }
        }
        self.matrix.set(*(opt_u.unwrap()), *(opt_w.unwrap()),0)
    }
    fn get_edge_value(&self, x: &T, y: &T) -> Result<int, ~str> {
        let opt_u = self.vertices.find(x);
        let opt_w = self.vertices.find(y);
        match (opt_u, opt_w) {
            (None, _) => return Err(format!("Vertex x: ({}) was not found in the graph", x)),
            (_, None) => return Err(format!("Vertex y: ({}) was not found in the graph", y)),
            (_, _) => { }
        }
        self.matrix.at(*(opt_u.unwrap()), *(opt_w.unwrap()))
    }
    fn set_edge_value(&mut self, x: &T, y: &T, val: int) -> Result<int, ~str> {
        // TODO: If x, y are not adjacent, return Err().
        let opt_u = self.vertices.find(x);
        let opt_w = self.vertices.find(y);
        match (opt_u, opt_w) {
            (None, _) => return Err(format!("Vertex x: ({}) was not found in the graph", x)),
            (_, None) => return Err(format!("Vertex y: ({}) was not found in the graph", y)),
            (_, _) => { }
        }
        self.matrix.set(*(opt_u.unwrap()), *(opt_w.unwrap()), val)
    }
}

fn main() {
    let mut matrix: ~VectorMatrix = Matrix::zero(5, 5);
    matrix.set(1, 1, 5);
    matrix.set(2,3,-10);
    println!("{:?}", matrix)
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
    let mut graph: ~AdjacencyMatrixGraph<int> = Graph::new(3);
    let mut res = graph.add(0,1,1);
    assert!(res.is_ok());
    res = graph.add(0,2,1);
    assert!(res.is_ok());
    let neighbors = graph.neighbors(&0);
    assert!(neighbors.is_ok());
    let neigh = neighbors.unwrap();
    println!("{}", neigh);
    assert_eq!(neigh.len(), 2);
    assert_eq!(*neigh[0], 1);
    assert_eq!(*neigh[1], 2);
}

#[test]
fn test_graph_adjacent() {
    let mut graph: ~AdjacencyMatrixGraph<int> = Graph::new(3);
    assert!(graph.adjacent(&1, &2).is_err());
    let mut res = graph.add(0,1,1);
    assert!(res.is_ok());
    res = graph.add(0,2,1);
    assert!(res.is_ok());
    assert!(graph.adjacent(&0,&1).unwrap());
    assert!(graph.adjacent(&0,&2).unwrap());
}
