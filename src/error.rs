use std::fmt::Display;

pub enum FilelistError {
  IoError(std::io::Error),
  FilelistDirectoryNotFound(std::path::PathBuf),
  Custom(&'static str),
}

impl Display for FilelistError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IoError(e) => write!(f, "IoError({e}"),
      Self::Custom(m) => write!(f, "{m}"),
      Self::FilelistDirectoryNotFound(dir) => write!(f, "Directory Not Found: {}", dir.display()),
    }
  }
}

impl From<std::io::Error> for FilelistError {
  fn from(value: std::io::Error) -> Self {
    Self::IoError(value)
  }
}

impl From<&'static str> for FilelistError {
  fn from(value: &'static str) -> Self {
    Self::Custom(value)
  }
}

pub fn handle<T, E>(errors: &mut Vec<FilelistError>, res: Result<T, E>) -> Option<T>
where
  E: Into<FilelistError>,
{
  match res {
    Ok(v) => Some(v),
    Err(e) => {
      errors.push(e.into());
      None
    }
  }
}
