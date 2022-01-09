use super::container::ContainerType;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum CellType { Normal, Energy, Unavailable }

#[derive(Copy, Clone)]
pub struct Cell {
    pub type_: CellType
}

impl CellType {
    pub fn get_compatible_containers(self) -> Vec<ContainerType> {
        match self {
            CellType::Normal => vec![ContainerType::Standard],
            CellType::Energy => vec![ContainerType::Standard, ContainerType::Refrigerated],
            CellType::Unavailable => vec![],
        }
    }
}

impl TryFrom<char> for CellType {
    type Error = String;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            'N' => Ok(CellType::Normal),
            'E' => Ok(CellType::Energy),
            'X' => Ok(CellType::Unavailable),
            _ => Err(String::from("Not a valid cell")),
        }
    }
}

impl Into<char> for CellType {
    fn into(self) -> char {
        match self {
            CellType::Normal => 'N',
            CellType::Energy => 'E',
            CellType::Unavailable => 'X',
        }
    }
}