
use std::error::Error;
use std::io::{self, Error as IoError, ErrorKind};
use std::str::FromStr;

pub type Result<T, E = Box<dyn Error + Send + Sync>> = std::result::Result<T, E>;

/// Read one CSV line from stdin, interpret all fields as `T`.
///
/// Example:
///
/// ```no_run
///     let items = read_csv_lineL::<i64>()?;
/// ```
///
pub fn read_csv_line<T>() -> io::Result<Vec<T>>
where
    T: FromStr,
    T::Err: Into<Box<dyn Error + Send + Sync>>,
{
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    line
        .split(',')
        .map(|v| v.trim().parse::<T>().map_err(|e| IoError::new(ErrorKind::Other, e)))
        .collect()
}
