use include_oracle_sql::*;

include_sql!("examples/dyn_query.sql");

#[cfg(not(feature = "tokio"))]
fn main() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

    let from_date = sibyl::Date::with_date(2005,  1,  1, &session);
    let thru_date = sibyl::Date::with_date(2007, 12, 31, &session);
    session.median_salary(&from_date, &thru_date, &["CA", "DE", "US", "UK"], |row| {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;
        println!("{country_name:25}: {median_salary:>5}");
        Ok(())
    })?;

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

    let from_date = sibyl::Date::with_date(2005,  1,  1, &session);
    let thru_date = sibyl::Date::with_date(2007, 12, 31, &session);
    session.median_salary(&from_date, &thru_date, &["CA", "DE", "US", "UK"], |row| {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;
        println!("{country_name:25}: {median_salary:>5}");
        Ok(())
    }).await?;

    Ok(())
}