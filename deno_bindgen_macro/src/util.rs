#[derive(Debug)]
pub enum Error {
  Asyncness,
  Reciever,
  UnsupportedType,
  Generics,
  WhereClause,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Error::Asyncness => write!(f, "async functions are not supported"),
      Error::Reciever => write!(f, "methods are not supported"),
      Error::UnsupportedType => write!(f, "unsupported type"),
      Error::Generics => write!(f, "generics are not supported"),
      Error::WhereClause => write!(f, "where clauses are not supported"),
    }
  }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
