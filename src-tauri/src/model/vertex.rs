//! Представляет собой вспомогательную обёртку для взаимодействия с пользователем при обходе графа.
//! Хранит текущую вершину и соседей

use std::mem::swap;

use super::error::{ErrorModel, Result};
use super::graph::Graph;

#[derive(Debug)]
struct Vertex<'a> {
    // Номер текущей вершины
    current: usize,
    // Ссылка на граф
    graph: &'a Graph,
    // Посещенные вершины
    visits: Vec<bool>,
}

impl<'a> Vertex<'a> {
    /// Проверка, что номер вершины валиден
    fn is_valid_num_vertex_graph(num_vertex: usize, graph: &Graph) -> Result<()> {
        if num_vertex >= graph.get_ref_inner().len() {
            return Err(ErrorModel::NotValidNumVertex);
        }
        Ok(())
    }
    fn is_valid_num_vertex(&self, num_vertex: usize) -> Result<()> {
        Self::is_valid_num_vertex_graph(num_vertex, self.graph)
    }

    fn try_new(num_vertex: usize, graph: &'a Graph) -> Result<Self> {
        // Великий прикол 0_o
        // Интересно насколько он нужен...
        Self::is_valid_num_vertex_graph(num_vertex, graph)?;

        let mut visits = vec![false; graph.get_ref_inner().len()];
        visits[num_vertex] = true;

        Ok(Vertex {
            current: num_vertex,
            graph,
            visits,
        })
    }
}

/// Для взаимодействия с пользователем при обходе DFS
#[derive(Debug)]
pub struct VertexDFS<'a> {
    vertex: Vertex<'a>,
    stack: Vec<usize>,
}

impl<'a> VertexDFS<'a> {
    pub fn try_new(num_vertex: usize, graph: &'a Graph) -> Result<Self> {
        let vertex = Vertex::try_new(num_vertex, graph)?;
        let mut stack = Vec::with_capacity(graph.get_ref_inner().len());
        stack.push(num_vertex);
        Ok(VertexDFS { vertex, stack })
    }

    /// Когда на ветке нет соседей, нужно вернуться по стеку
    fn update_stack(&mut self) {
        loop {
            if self.count_next() == 0 {
                match self.stack.pop() {
                    Some(val) => self.vertex.current = val,
                    None => break,
                }
            } else {
                break;
            }
        }
    }
    fn count_next(&self) -> usize {
        self.vertex.graph.get_ref_inner()[self.vertex.current]
            .iter()
            .filter(|&&el| !self.vertex.visits[el])
            .count()
    }

    /// Проверка что вершина является смежной для текущей
    fn is_next(&self, num_vertex: usize) -> bool {
        self.vertex.graph.get_ref_inner()[self.vertex.current].contains(&num_vertex)
    }
    /// Совершить перемещение в вершину
    pub fn step_to(&mut self, num_vertex: usize) -> Result<()> {
        self.vertex.is_valid_num_vertex(num_vertex)?;
        if !self.is_next(num_vertex) {
            return Err(ErrorModel::CantMoveToVertex);
        }
        if self.vertex.visits[num_vertex] {
            return Err(ErrorModel::AlreadyVisited);
        }
        self.vertex.current = num_vertex;
        self.vertex.visits[num_vertex] = true;
        self.stack.push(num_vertex);

        self.update_stack();

        Ok(())
    }

    pub fn get_current(&self) -> usize {
        self.vertex.current
    }
}

/// Для взаимодействия с пользователем при обходе BFS
#[derive(Debug)]
pub struct VertexBFS<'a> {
    vertex: Vertex<'a>,
    level: Vec<usize>,
    queue: Vec<usize>,
}

impl<'a> VertexBFS<'a> {
    pub fn try_new(num_vertex: usize, graph: &'a Graph) -> Result<Self> {
        let vertex = Vertex::try_new(num_vertex, graph)?;
        let mut level = vec![];
        for &i in &graph.get_ref_inner()[num_vertex] {
            level.push(i);
        }
        Ok(VertexBFS {
            vertex,
            level,
            queue: Vec::new(),
        })
    }

    /// Проверка что вершина является смежной для текущей
    fn is_next(&self, num_vertex: usize) -> bool {
        self.level.contains(&num_vertex)
    }
    fn update_level(&mut self) {
        if self.level.is_empty() {
            swap(&mut self.level, &mut self.queue);
        }
    }
    /// Заполнить очередь соседями вершины
    fn fill_queue(&mut self, vertex: usize) {
        for &i in &self.vertex.graph.get_ref_inner()[vertex] {
            if !self.vertex.visits[i] {
                self.queue.push(i);
            }
        }
    }
    /// Совершить перемещение в вершину
    pub fn step_to(&mut self, num_vertex: usize) -> Result<()> {
        self.vertex.is_valid_num_vertex(num_vertex)?;
        if !self.is_next(num_vertex) {
            return Err(ErrorModel::CantMoveToVertex);
        }
        if self.vertex.visits[num_vertex] {
            return Err(ErrorModel::AlreadyVisited);
        }

        self.vertex.current = num_vertex;
        self.vertex.visits[num_vertex] = true;
        self.level.retain(|&x| x != num_vertex);

        self.fill_queue(num_vertex);

        // обновить level
        self.update_level();
        Ok(())
    }

    pub fn get_current(&self) -> usize {
        self.vertex.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_msg() {
        let mut graph = Graph::new(4);
        let vertex = Vertex::try_new(7, &mut graph);
        assert_eq!(true, vertex.is_err());
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new(7);
        let vc = [[1, 2], [2, 3], [2, 4], [4, 5], [4, 6], [6, 7]];

        for [u, v] in vc {
            graph.add_edge(v - 1, u - 1).unwrap();
            graph.add_edge(u - 1, v - 1).unwrap();
        }

        let mut dfs = VertexDFS::try_new(0, &graph).unwrap();
        dfs.step_to(1).unwrap();
        dfs.step_to(2).unwrap();
        dfs.step_to(3).unwrap();
        dfs.step_to(4).unwrap();
        dfs.step_to(5).unwrap();
        dfs.step_to(6).unwrap();

        assert!(
            !dfs.vertex.visits.contains(&false),
            "Должен быть полный проход"
        );
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new(14);
        let vc = [
            [1, 0],
            [0, 2],
            [0, 3],
            [1, 11],
            [1, 10],
            [1, 9],
            [2, 8],
            [2, 7],
            [3, 6],
            [3, 5],
            [3, 4],
            [10, 12],
            [10, 13],
        ];

        for [u, v] in vc {
            graph.add_edge(v, u).unwrap();
            graph.add_edge(u, v).unwrap();
        }

        let mut bfs = VertexBFS::try_new(0, &graph).unwrap();
        println!("Before level 1: {:?}\n{:?}", &bfs.queue, &bfs.level);
        // 1 level
        bfs.step_to(1).unwrap();
        bfs.step_to(2).unwrap();
        bfs.step_to(3).unwrap();
        println!("After level 1: {:?}\n{:?}", &bfs.queue, &bfs.level);
        // 2 level
        bfs.step_to(4).unwrap();
        bfs.step_to(8).unwrap();
        bfs.step_to(7).unwrap();
        bfs.step_to(11).unwrap();
        bfs.step_to(5).unwrap();
        bfs.step_to(9).unwrap();
        bfs.step_to(6).unwrap();
        bfs.step_to(10).unwrap();
        println!("After level 2: {:?}\n{:?}", &bfs.queue, &bfs.level);
        // 3 level
        bfs.step_to(12).unwrap();
        bfs.step_to(13).unwrap();
        println!("After level 3: {:?}\n{:?}", &bfs.queue, &bfs.level);

        assert!(
            !bfs.vertex.visits.contains(&false),
            "Должен быть полный проход"
        );
    }
}
