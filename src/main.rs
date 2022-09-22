mod handlers;
use handlers::*;

pub use std::io::Error;
use std::{thread::sleep, time::Duration};

use notify_rust::Notification;
use webpage::{Webpage, WebpageOptions};

#[derive(Clone, Copy, Debug)]
pub enum SourceName {
    BiharUGMAC22,
    MCC,
}

#[derive(Debug)]
pub struct Notice {
    pub link: String, // can be content of onclick
    heading: String,
    pub source: SourceName,
}

impl Notice {
    pub fn new(heading: String, source: SourceName) -> Notice {
        Notice {
            link: String::new(),
            heading,
            source,
        }
    }
}

pub fn get_html(url: &str) -> Result<String, Error> {
    let mut options = WebpageOptions::default();
    options.allow_insecure = true;
    let webpage = Webpage::from_url(url, options)?;

    Ok(webpage.http.body)
}

fn main() -> ! {
    loop {
        let sources = [
            (
                SourceName::BiharUGMAC22,
                "https://bceceboard.bihar.gov.in/UGMACIndex.php",
                //"https://bceceboard.bihar.gov.in/UGMAC22_Aplindex.php",
            ),
            (
                SourceName::MCC,
                "https://mcc.nic.in/WebinfoUG/Page/Page?PageId=1&LangId=P",
            ),
        ];

        let mut all_notices = Vec::new();
        for src in sources {
            match handle_source(src) {
                Ok(mut notices) => all_notices.append(&mut notices),
                Err(e) => {
                    println!("Failed to fetch {:?}. Error: {:?}", src, e);
                }
            }
        }

        if all_notices.is_empty() {
            println!("No new notices");
        }

        for notice in all_notices {
            println!("\n--------------------------------------");
            println!("{}", notice.heading);
            println!("Link   : {}", notice.link);
            println!("Source : {:?}", notice.source);
            println!("--------------------------------------\n");

            Notification::new()
                .summary(&notice.heading)
                .body(&notice.link)
                .icon("/usr/share/icons/hicolor/scalable/apps/ibus.svg");
        }

        // Sleep for 5 minutes
        sleep(Duration::from_secs(5 * 60));
    }
}
