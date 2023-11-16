use anyhow::Result;
use rusqlite::{params, Connection};

struct Bookmark {
    name: String,
    url: String,
}

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open("bookmarks.db")?;
}
