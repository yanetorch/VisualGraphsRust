//! Алгоритмы кодирования и декодирования кода Прюфера по графу

use crate::model::graph::Graph;

#[derive(Debug)]
pub struct PruferCode {
    code: Vec<usize>,
}

impl PruferCode {
    pub fn new(graph: Graph) -> Self {
        PruferCode {
            code: incode_prufer(graph.get_ref_inner()),
        }
    }
}

fn incode_prufer(v: &Vec<Vec<usize>>) -> Vec<usize> {
    todo!()
}

fn decode_prufer(code: &Vec<usize>) -> Vec<(usize, usize)> {
    todo!()
}
