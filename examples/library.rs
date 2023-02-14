use sibyl::*;
use include_oracle_sql::*;

include_sql!("examples/library.sql");

fn main() -> Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = Environment::new()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

    session.create_library()?;
    session.init_library()?;

    session.loan_books(&["War and Peace", "Gone With the Wind"], "Sheldon Cooper")?;
    session.loan_books(&["The Lord of the Rings", "Master and Commander"], "Leonard Hofstadter")?;

    session.get_loaned_books("Sheldon Cooper", |row| {
        let book_title : &str = row.get(0)?;
        println!("{book_title}");
        Ok(())
    })?;

    println!("---");

    session.get_loaned_books("Leonard Hofstadter", |row| {
        let book_title : &str = row.get(0)?;
        println!("{book_title}");
        Ok(())
    })?;

    // session.drop_library()?;

    Ok(())
}
