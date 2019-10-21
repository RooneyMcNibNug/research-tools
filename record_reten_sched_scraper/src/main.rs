extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() -> Result<()> {
    let res = reqwest::get("https://sos.oregon.gov/archives/Pages/records_retention_schedule.aspx")?;

    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
