use std::io::Error;

use dotenv;
use mysql::{prelude::*, Conn, OptsBuilder};
use scraper::{Html, Selector};
use webpage::{Webpage, WebpageOptions};
use whoami;

/**
 * The NMC value collector.
 *
 * URL: https://www.nmc.org.in
 *
 * Prerequisites:
 * 1. You MUST have these environment variables set:
 *
 * DB_NAME=medicaldata
 * TABLE_NAME=nmc
 * SQL_USERNAME=your-sql-username
 * SQL_PASSWORD=your-sql-password
 *
 * 2. Create a database named as given in the `DB_NAME` (eg. medicaldata) environment variable.
 * 3. Create a table named as given in the `TABLE_NAME` (eg. nmc) environment variable,
 * with following columns:
 *
 * CREATE TABLE nmc (
 *    date DATE NOT NULL,
 *    title VARCHAR(100) NOT NULL,
 *    value INT NOT NULL,
 *    CONSTRAINT nmc_pk PRIMARY KEY (date, title)
 * );
 *
 * */

static SQL_FAILED_ERROR_ENV: &str = "Failed to get environment variable";
static SQL_FAILED_ERROR_CONN: &str = "Failed connecting to SQL";
static SQL_FAILED_ERROR_QUERY: &str = "Failed quering SQL";

fn get_html(url: &str) -> Result<String, Error> {
    let mut options = WebpageOptions::default();
    options.allow_insecure = true;
    let webpage = Webpage::from_url(url, options)?;

    Ok(webpage.http.body)
}

fn save_to_mariadb(title: &String, value: &String, date: &String) {
    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    let title = title.trim();
    let value = value
        .trim()
        .parse::<i32>()
        .expect("Failed parsing value to i32");
    let date = date.trim();

    println!("date: \"{}\"", date);
    println!("title: \"{}\"", title);
    println!("value: \"{}\"", value);

    let sql_username = match dotenv::var("SQL_USERNAME") {
        Ok(uname) => uname,
        Err(err) => {
            println!("ERROR: {}", SQL_FAILED_ERROR_ENV);
            println!("DETAILED ERROR: {:?}", err);

            // trying the user's own username
            println!(
                "WARN: Using your current username: {} as sql username",
                whoami::username()
            );
            whoami::username()
        }
    };

    // Password can be None
    let sql_password = dotenv::var("SQL_PASSWORD").ok();

    let db_name = dotenv::var("DB_NAME")
        .expect("DB_NAME environment variable is mandatory, and must contain the database name");

    let table_name = dotenv::var("TABLE_NAME")
        .expect("TABLE_NAME environment variable is mandatory, and must contain the table name");

    let opts = OptsBuilder::new()
        .user(Some(sql_username))
        .pass(sql_password)
        .db_name(Some(db_name));

    let mut conn = Conn::new(opts).expect(SQL_FAILED_ERROR_CONN);

    let query = format!(
        "INSERT INTO {} (date, title, value) VALUES ('{}', '{}', {})",
        table_name, date, title, value
    );

    // 1st type is T, which is the type of column data, it's required since .query returns Vec<T>
    // 2nd type is Q, which represents the type of `query` variable, i used simple string for `queue` variable :)
    conn.query::<String, String>(query)
        .expect(SQL_FAILED_ERROR_QUERY);
}

// https://www.nmc.org.in/
fn main() -> Result<(), Error> {
    // TODO: Change naming of body to html, and html to document
    let body = get_html("https://www.nmc.org.in/")?;
    let html = Html::parse_document(&body);

    // SAFETY: If this fails, it will fail during development too
    let selector = Selector::parse(".col-md-2.main-block1.bg3.hvr-grow.widthmain").unwrap();

    // Get current date, in "YYYY-MM-DD" format
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();

    for list_item in html.select(&selector) {
        let title_selector = Selector::parse("p.pnameclass").unwrap();
        let value_selector = Selector::parse("p.main-block-numbersize").unwrap();

        // SAFETY: Atleast one should be there on the website
        let title = list_item
            .select(&title_selector)
            .next()
            .expect("Heading p not found")
            .text()
            .collect::<Vec<_>>()
            .join("");
        let value = list_item
            .select(&value_selector)
            .next()
            .expect("Data p not found")
            .text()
            .collect::<Vec<_>>()
            .join("");

        save_to_mariadb(&title, &value, &date);
    }

    Ok(())
}
