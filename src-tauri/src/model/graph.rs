//! Предоставляет сущность графа

use super::error::{ErrorModel, Result};

/// # Структура графа
/// Все верщины должны иметь значения в диапазоне [0, n),
/// где n - кол-во вершин
#[derive(Debug)]
pub struct Graph {
    inner: Vec<Vec<usize>>,
}

impl Graph {
    /// Функция для создания графа
    /// * параметр size задает количество вершин в графе
    pub fn new(size: usize) -> Self {
        Graph {
            inner: vec![vec![]; size],
        }
    }

    pub fn add_edge(&mut self, v: usize, u: usize) -> Result<()> {
        if v >= self.inner.len() || u >= self.inner.len() {
            return Err(ErrorModel::NotValidNumVertex);
        }
        self.inner[v].push(u);

        Ok(())
    }

    pub fn get_ref_inner(&self) -> &Vec<Vec<usize>> {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
