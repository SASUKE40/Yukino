use crate::mapping::error::CompileError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct DefinitionError(String);

impl DefinitionError {
    #[allow(dead_code)]
    pub fn new<D: Display + ?Sized>(message: &D) -> Self {
        DefinitionError(format!(
            "Definition Error: Error('{}') occurred while resolving Definition.",
            message
        ))
    }
}

impl Display for DefinitionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_message())
    }
}

impl Error for DefinitionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl CompileError for DefinitionError {
    fn get_message(&self) -> String {
        self.0.clone()
    }
}
