/*  Chris Piraino
 *
 *
 *  Implementation of a Graph and
 *  associated algorithms and data
 *  structures.
 *
 */
use std::vec;
use std::result::Result;

trait Graph {
    fn new(vertices: uint) -> ~Self;
    fn adjacent(&self, x: uint, y: uint) -> Result<bool, ~str>;
    fn neighbors(&self, x: uint) -> ~[uint];
    fn add(&mut self, x: uint, y: uint, val: int) -> Result<int, ~str>;
    fn delete(&mut self, x: uint, y: uint) -> Result<int, ~str>;
    fn get_edge_value(&self, x: uint, y: uint) -> Result<int, ~str>;
    fn set_edge_value(&mut self, x: uint, y: uint, val: int) -> Result<int, ~str>;
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
    fn neighbors(&self, x: uint) -> ~[uint] {
        let row = self.elements.slice(x*self.width, x*self.width+self.width);
        let build = |push: |v: uint|| {
            for r in row.iter().enumerate() {
                let (i, val) = r;
                if *val != 0 {
                    push(i)
                }
            }
        };
        vec::build(None, build)
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
        self.set(x, y, val)
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
