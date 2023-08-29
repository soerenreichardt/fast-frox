use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("{}", msg)]
pub(crate) struct CompileError {
    pub(crate) msg: String,

    #[source_code]
    pub(crate) src: NamedSource,

    #[label("{}", self)]
    pub(crate) span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{}", msg)]
pub(crate) struct RuntimeError {
    pub(crate) msg: String
}

impl RuntimeError {
    pub(crate) fn new(msg: String) -> Self {
        RuntimeError { msg }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Error occured while interpreting")]
pub(crate) struct InterpreterError<I>
where
    I: Diagnostic,
{
    #[related]
    related: Vec<I>,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{}", msg)]
pub struct ArgumentError {
    pub msg: String,
}

