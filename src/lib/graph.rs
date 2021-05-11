use std::convert::TryFrom;
use std::error::Error;
use crate::lib::common::IllegalArgumentError;

pub struct Graph {
    vertices: usize,
    edges: usize,
    adj: Box<[Vec<u32>]>,
}

impl Graph {
    pub fn new(v: &usize) -> Graph {
        let mut adj: Vec<Vec<u32>> = Vec::with_capacity(*v);
        for _ in 0..*v {
            adj.push(Vec::new())
        }
        Graph {
            vertices: *v,
            edges: 0,
            adj: adj.into_boxed_slice(),
        }
    }

    pub fn add_edge(&mut self, v: &usize, w: &usize) -> Result<(), Box<dyn Error>> {
        let v_u32 = u32::try_from(*v)?;
        let w_u32 = u32::try_from(*w)?;
        if self.adj.len() < *v && self.adj.len() < *w {
            self.adj.get_mut(*v).unwrap().push(w_u32);
            self.adj.get_mut(*w).unwrap().push(v_u32);
            Ok(())
        } else {
            Err(Box::new(IllegalArgumentError::new(format!("Both v and w must be less than {}", self.adj.len()))))
        }
    }
}