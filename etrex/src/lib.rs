use std::fmt;

pub struct EtrexFile {
    data: Vec<u8>
}
impl EtrexFile {
    pub fn new(data: impl Into<Vec<u8>>) -> EtrexFile {
        EtrexFile { data: data.into() }
    }
}

impl fmt::Debug for EtrexFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EtrexFile: {{ data: {} }}", self.data.len())
    }
}