#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ContainerType { Standard, Refrigerated }

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Container {
    pub type_: ContainerType,
    pub dest_port: usize,
}

impl ContainerType {
    pub fn get_all() -> [ContainerType; 2] {
        [ContainerType::Refrigerated, ContainerType::Standard]
    }
}

impl TryFrom<char> for ContainerType {
    type Error = String;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            'S' => Ok(ContainerType::Standard),
            'R' => Ok(ContainerType::Refrigerated),
            _ => Err(String::from("Not a valid container")),
        }
    }
}

impl Into<char> for ContainerType {
    fn into(self) -> char {
        match self {
            ContainerType::Standard => 'S',
            ContainerType::Refrigerated => 'R',
        }
    }
}