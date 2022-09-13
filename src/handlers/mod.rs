mod bihar_ugmac;
mod default;

use crate::{Error, Notice, SourceName};

pub use bihar_ugmac::handler as handle_bihar_ugmac21;
pub use default::handler as handle_default;

pub fn handle_source(src: (SourceName, &str)) -> Result<Vec<Notice>, Error> {
    match src.0 {
        SourceName::BiharUGMAC21 => handle_bihar_ugmac21(src),
        _ => handle_default(src),
    }
}
