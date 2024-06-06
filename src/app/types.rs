
pub type Error = async_graphql::Error;

pub type Result<T> = std::result::Result<T , Error>;