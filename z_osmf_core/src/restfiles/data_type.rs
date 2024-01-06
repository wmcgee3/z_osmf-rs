pub struct Binary;
pub struct Record;
pub struct Text;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DataType {
    Binary,
    Record,
    #[default]
    Text,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataType::Binary => "binary",
                DataType::Record => "record",
                DataType::Text => "text",
            }
        )
    }
}
