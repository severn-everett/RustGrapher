use std::convert::TryFrom;
use std::error::Error;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::lib::common::{IllegalArgumentError, InputError};

pub struct Graph {
    vertices: u32,
    edges: u32,
    adj: Box<[Vec<u32>]>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphConfig {
    vertex_amt: u32,
    edges: Vec<[u32; 2]>,
}

impl Graph {
    pub fn new(v: &u32) -> Result<Graph, Box<dyn Error>> {
        let mut adj: Vec<Vec<u32>> = Vec::with_capacity(usize::try_from(*v)?);
        for _ in 0..*v {
            adj.push(Vec::new())
        }
        Ok(Graph {
            vertices: *v,
            edges: 0,
            adj: adj.into_boxed_slice(),
        })
    }

    pub fn load(input_file: &str) -> Result<Graph, Box<dyn Error>> {
        let raw_input = fs::read_to_string(input_file)?;
        let graph_config: GraphConfig = serde_json::from_str(&raw_input)?;
        let mut graph = Graph::new(&graph_config.vertex_amt)?;
        for (i, edge) in graph_config.edges.iter().enumerate() {
            let v = match edge.get(0) {
                Some(v) => v,
                None => {
                    let error = Box::new(InputError::new(
                        format!("Edge point 1 missing for row {}", i)
                    ));
                    return Err(error);
                }
            };
            let w = match edge.get(1) {
                Some(w) => w,
                None => {
                    let error = Box::new(InputError::new(
                        format!("Edge point 2 missing for row {}", i)
                    ));
                    return Err(error);
                }
            };
            graph.add_edge(v, w)?;
        }
        Ok(graph)
    }

    pub fn add_edge(&mut self, v: &u32, w: &u32) -> Result<(), Box<dyn Error>> {
        let v_usize = usize::try_from(*v)?;
        let w_usize = usize::try_from(*w)?;
        if self.adj.len() > v_usize && self.adj.len() > w_usize {
            self.adj.get_mut(v_usize).unwrap().push(*w);
            self.adj.get_mut(w_usize).unwrap().push(*v);
            Ok(())
        } else {
            let error = Box::new(IllegalArgumentError::new(format!("Both v and w must be less than {}", self.adj.len())));
            Err(error)
        }
    }
}