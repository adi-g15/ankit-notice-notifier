use super::default::handler as handle_default;
use crate::{Error, Notice, SourceName};
use std::fs;
use xdg::BaseDirectories;

pub fn handler(src: (SourceName, &str)) -> Result<Vec<Notice>, Error> {
    let mut notices = handle_default(src)?;

    // Mostly this will not happen, since there will atleast be a 1 word change while fetching from MCC
    if notices.is_empty() {
        return Ok(notices);
    }

    // Otherwise, If there is a notice, that means handle_default thinks there is a
    // update but most of the time, only a single integer change is there
    // so this function just handles that case
    let source_name = format!("{:?}", src.0);

    let xdg_dir = BaseDirectories::with_prefix(format!("ankit-neet-notify/{}", source_name))?;

    let mut archive_dir = xdg_dir.get_data_home();
    archive_dir.push("archive");

    let count = if archive_dir.exists() {
        // Get count of files inside archive_dir
        archive_dir.read_dir()?.filter(|f| f.is_ok()).count()
    } else {
        0
    };

    // If there is atleast 1 file in archives, that means we can safely access "MCC_${count-1}".html
    if count != 0 {
        // SAFETY: Since count != 0, that means atleast 1 file exists in archive, and following our naming convention this will work
        let old_filepath = xdg_dir
            .find_data_file(&format!("archive/{}_{}.html", &source_name, count - 1))
            .unwrap();

        // SAFETY: Believing that `handle_default()` created this file
        let new_filepath = xdg_dir
            .find_data_file(&format!("{}.html", &source_name))
            .unwrap();

        let old_content = fs::read_to_string(&old_filepath)?;
        let new_content = fs::read_to_string(&new_filepath)?;

        // diff `old_content` and `new_content` word by word, if it's a 1-one change, treat it as insignificant change (that is that 1 number changes everytime fetched from MCC)
        let old_content = old_content.split_whitespace();
        let new_content = new_content.split_whitespace();

        let mut diff_count = 0;
        for (old, new) in old_content.zip(new_content) {
            if old != new {
                diff_count += 1;
            }
        }

        // If diff_count is 1, that means only 1 word changed, and that is the number, so we can safely ignore this update
        if diff_count == 1 {
            notices.clear();
        }
    }

    Ok(notices)
}
