use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("{}", msg)]
pub(crate) struct CompileError<'a> {
    pub(crate) msg: &'a str,

    #[source_code]
    pub(crate) src: NamedSource,

    #[label("{}", self)]
    pub(crate) span: SourceSpan
}