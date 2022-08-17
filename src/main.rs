use scraper::{Html,Selector};
use webpage::{Webpage, WebpageOptions};
use notify_rust::Notification;

#[derive(Clone,Copy,Debug)]
enum SourceName {
    BiharUGMAC21
}

struct Source {
    name: SourceName,
    url: String
}

#[derive(Debug)]
struct Notice {
    pub href: String,   // can be content of onclick
    heading: String,
    pub source: SourceName
}

impl Notice {
    pub fn new(heading: String, source: SourceName) -> Notice {
        Notice {
            href: String::new(),
            heading,
            source
        }
    }
}

fn handle_source(src: Source) -> Vec<Notice> {
    /*
     * Using curl:
    let mut handle = Easy::new();
    handle.url(&src.url).unwrap();

    let mut data = Vec::new();

    {
        let mut transfer = handle.transfer();
        transfer.write_function(|bytes| {
            data.extend_from_slice(bytes);
            println!("{:?}", bytes);
            Ok(bytes.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let html = Html::parse_document(str::from_utf8(&data).unwrap());
    */

    let mut webpage_opts = WebpageOptions::default();
    webpage_opts.allow_insecure = true;
    let http = Webpage::from_url(&src.url, webpage_opts).unwrap().http;
    let html = Html::parse_document(&http.body);

    let mut notices = Vec::new();

    let selector = Selector::parse(".noticeamin li").unwrap();
    for list_item in html.select(&selector) {
        let mut heading = String::new();
        for txt in list_item.text() {
            heading.push_str(txt);
        }

        heading = String::from(heading.trim());
        let mut notice = Notice::new(heading, src.name);
        
        let anchor = list_item.first_child().unwrap();
        let av = anchor.value().as_element().unwrap();
        let onclick = av.attr("onclick");
        let href = av.attr("href");
        
        if onclick.is_some() {
            let relative_url = onclick.unwrap().split_once('\'').unwrap().1.split_once('\'').unwrap().0;
            notice.href = String::from("https://bceceboard.bihar.gov.in/");
            notice.href.push_str(relative_url);
        } else {
            notice.href = String::from(href.unwrap());
        }

        notices.push(notice);
    }

    notices
}

fn main() {
    let mut sources = Vec::new();
    sources.push(Source{
        name: SourceName::BiharUGMAC21,
        url: String::from("https://bceceboard.bihar.gov.in/UGMAC21_Aplindex.php")
    });

    let mut notices = Vec::new();
    for src in sources {
        notices.append( &mut handle_source(src) );
    }

    for notice in notices {
        println!("\n--------------------------------------");
        println!("{}", notice.heading);
        println!("Href   : {}", notice.href);
        println!("Source : {:?}", notice.source);
        println!("--------------------------------------\n");

        Notification::new()
            .summary(&notice.heading)
            .body(&notice.href)
            .icon("/usr/share/icons/hicolor/scalable/apps/ibus.svg");
            
    }
}

