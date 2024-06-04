use std::{error::Error, fmt::Display};

pub type Result<T> = core::result::Result<T, ErrorModel>;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorModel {
    NotValidNumVertex,
    CantMoveToVertex,
    AlreadyVisited,
}

impl ErrorModel {
    pub fn message(&self) -> String {
        match self {
            Self::NotValidNumVertex => {
                format!("Номер вершины должен быть в диапазоне [0, n), где n - номер вершины.")
            }
            Self::CantMoveToVertex => {
                format!("Нельзя перейти к вершине, которая не является смежной.")
            }
            Self::AlreadyVisited => {
                format!("Нельзя перейти в уже посещенную вершину.")
            }
        }
    }
}

impl Display for ErrorModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ErrorModel {}
