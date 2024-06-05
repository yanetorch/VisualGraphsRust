//! Предоставляет сущность графа

use std::str::FromStr;

use super::error::{ModelError, Result};

/// # Структура графа
/// Все верщины должны иметь значения в диапазоне [0, n),
/// где n - кол-во вершин
#[derive(Debug)]
pub struct Graph {
    inner: Vec<Vec<usize>>,
}

impl FromStr for Graph {
    type Err = ModelError;
    /// # Создает неориентированный граф
    /// Для создания ориентированного графа использовать функцию add_edge
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut it = s.chars().peekable();
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut get_size = false;
        let mut lst: Option<usize> = None;
        let mut size = 0;

        while let Some(&ch) = it.peek() {
            if ch.is_numeric() {
                let mut token = String::new();
                while let Some(&ch) = it.peek() {
                    if !ch.is_numeric() {
                        break;
                    }
                    token.push(ch);
                    it.next();
                }
                let num = match token.parse::<usize>() {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(ModelError::FailParse(err.to_string()));
                    }
                };

                if !get_size {
                    edges = Vec::with_capacity(num);
                    size = num;
                    get_size ^= true;
                } else if lst.is_some() {
                    let lst = lst.take().unwrap();
                    edges.push((lst, num));
                } else {
                    lst = Some(num);
                }
            } else {
                it.next();
            }
        }
        if lst.is_some() {
            return Err(ModelError::FailParse(
                "Не найдена вторая вершина для ребра".to_string(),
            ));
        }

        Ok(Self::from_edges(edges, size)?)
    }
}

impl Graph {
    /// Функция для создания графа
    /// * параметр size задает количество вершин в графе
    pub fn new(size: usize) -> Self {
        Graph {
            inner: vec![vec![]; size],
        }
    }
    /// Создать Граф из вектора рёбер
    pub fn from_edges(edges: Vec<(usize, usize)>, size: usize) -> Result<Self> {
        let mut vector = vec![vec![]; size];
        for (x, y) in edges {
            if x >= size || y >= size {
                return Err(ModelError::NotValidNumVertex);
            }
            vector[x].push(y);
            vector[y].push(x);
        }
        Ok(Graph { inner: vector })
    }
    /// Добавить вершину
    pub fn add_edge(&mut self, v: usize, u: usize) -> Result<()> {
        if v >= self.inner.len() || u >= self.inner.len() {
            return Err(ModelError::NotValidNumVertex);
        }
        self.inner[v].push(u);

        Ok(())
    }
    /// Получить ссылку на веткор векторов
    pub fn get_ref_inner(&self) -> &Vec<Vec<usize>> {
        &self.inner
    }
    /// Получить степень каждой вершины
    pub fn get_vertex_degree(&self) -> Vec<usize> {
        self.inner.iter().map(|v| v.len()).collect()
    }

    pub fn is_euler(&self) -> bool {
        self.get_vertex_degree()
            .into_iter()
            .filter(|el| el % 2 != 0)
            .count()
            == 0
    }

    pub fn is_halfeuler(&self) -> bool {
        self.get_vertex_degree()
            .into_iter()
            .filter(|el| el % 2 != 0)
            .count()
            == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_parse_graph() {
        let str = "5 0 1 2 3 0 4 2 1";

        let graph = Graph::from_str(str).unwrap();

        assert_eq!(graph.inner.len(), 5);
    }
}
