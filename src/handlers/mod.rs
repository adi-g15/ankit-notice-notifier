mod bihar_ugmac;
mod default;
mod mcc_counselling;

use crate::{Error, Notice, SourceName};

pub use bihar_ugmac::handler as handle_bihar_ugmac22;
pub use default::handler as handle_default;
pub use mcc_counselling::handler as handle_mcc;

pub fn handle_source(src: (SourceName, &str)) -> Result<Vec<Notice>, Error> {
    match src.0 {
        SourceName::BiharUGMAC22 => handle_bihar_ugmac22(src),
        SourceName::MCC => handle_mcc(src),
        _ => handle_default(src),
    }
}
