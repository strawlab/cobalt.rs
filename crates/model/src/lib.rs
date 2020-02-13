pub mod assets;
pub mod fs;
pub mod page;

type Status = status::Status;
type Result<T, E = Status> = std::result::Result<T, E>;
