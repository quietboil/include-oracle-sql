use include_oracle_sql::*;

include_sql!("examples/library.sql");

#[cfg(not(feature = "tokio"))]
fn main() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
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

    session.drop_library()?;

    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass).await?;

    session.create_library().await?;
    session.init_library().await?;

    session.loan_books(&["War and Peace", "Gone With the Wind"], "Sheldon Cooper").await?;
    session.loan_books(&["The Lord of the Rings", "Master and Commander"], "Leonard Hofstadter").await?;

    session.get_loaned_books("Sheldon Cooper", |row| {
        let book_title : &str = row.get(0)?;
        println!("{book_title}");
        Ok(())
    }).await?;

    println!("---");

    session.get_loaned_books("Leonard Hofstadter", |row| {
        let book_title : &str = row.get(0)?;
        println!("{book_title}");
        Ok(())
    }).await?;

    session.drop_library().await?;

    Ok(())
}