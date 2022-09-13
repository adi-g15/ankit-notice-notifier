use crate::{Error, Notice, SourceName};

pub fn handler(src: (SourceName, &str)) -> Result<Vec<Notice>, Error> {
    Ok(vec![])
}
