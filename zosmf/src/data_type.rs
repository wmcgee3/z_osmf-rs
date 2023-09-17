pub struct Binary;
pub struct Record;
pub struct Text;

pub trait BytesDataType {}
impl BytesDataType for Binary {}
impl BytesDataType for Record {}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DataType {
    Binary,
    Record,
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
