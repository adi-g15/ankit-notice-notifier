use crate::{get_html, Error, Notice, SourceName};
use scraper::{Html, Selector};

pub fn handler(src: (SourceName, &str)) -> Result<Vec<Notice>, Error> {
    let body = get_html(src.1)?;
    let html = Html::parse_document(&body);

    let mut notices = Vec::new();

    let selector = Selector::parse(".noticeamin li").unwrap();
    for list_item in html.select(&selector) {
        let mut heading = String::new();
        for txt in list_item.text() {
            heading.push_str(txt);
        }

        heading = String::from(heading.trim());
        let mut notice = Notice::new(heading, src.0);

        let anchor = list_item.first_child().unwrap();
        let av = anchor.value().as_element().unwrap();
        let onclick = av.attr("onclick");
        let link = av.attr("href");

        if onclick.is_some() {
            let relative_url = onclick
                .unwrap()
                .split_once('\'')
                .unwrap()
                .1
                .split_once('\'')
                .unwrap()
                .0;
            notice.link = String::from("https://bceceboard.bihar.gov.in/");
            notice.link.push_str(relative_url);
        } else {
            notice.link = String::from(link.unwrap());
        }

        notices.push(notice);
    }

    Ok(notices)
}
