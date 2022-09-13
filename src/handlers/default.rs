use crate::{get_html, Error, Notice, SourceName};
use xdg::BaseDirectories;

pub fn handler(src: (SourceName, &str)) -> Result<Vec<Notice>, Error> {
    // Since, SourceName derives Debug, so format("{:?}",enum) can be used to get the enum name as
    // String
    let source_name = format!("{:?}", src.0);
    let new_filepath = format!("{}.html", source_name);

    let xdg_dir =
        BaseDirectories::with_prefix(format!("ankit-neet-notify/{}", source_name))?;

    let existing_filepath = xdg_dir.find_data_file(&new_filepath);

    // If already exists, move it into an 'archive' folder with the count added at end
    let updated_filepath = match existing_filepath {
        Some(filepath) => {
            let mut archive_dir = xdg_dir.get_data_home();
            archive_dir.push("archive");

            std::fs::create_dir_all(&archive_dir)?;

            // Get count of files inside archive_dir
            let count = archive_dir.read_dir()?.filter(|f| f.is_ok()).count();

            let mut archive_filepath = archive_dir.clone();
            archive_filepath.push(format!("{}_{}.html", source_name, count));

            std::fs::rename(filepath, &archive_filepath)?;

            Some(archive_filepath)
        }
        None => None,
    };

    // Now fetch the latest HTML
    let body = get_html(src.1)?;

    // Write body into file
    let filepath = xdg_dir.place_data_file(&new_filepath)?;
    std::fs::write(&filepath, body)?;

    if let Some(archived_filepath) = updated_filepath {
        // check if archived_file and filepath are same
        // If yes, then no new notices were added
        // So, return empty vector
        // Else, return notices
        let new_content = std::fs::read_to_string(&filepath)?;
        let old_content = std::fs::read_to_string(&archived_filepath)?;

        if new_content == old_content {
            return Ok(vec![]);
        }
    }

    Ok(vec![Notice::new(
        format!("Updated: {}", source_name),
        src.0,
    )])
}
