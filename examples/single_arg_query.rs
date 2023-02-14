use include_oracle_sql::*;

include_sql!("examples/single_arg_query.sql");

#[cfg(not(feature = "tokio"))]
fn main() -> sibyl::Result<()> {
    let oracle = sibyl::env()?;

    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;
    session.median_salary("Europe", |row| {
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
    let oracle = sibyl::env()?;

    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let session = oracle.connect(&dbname, &dbuser, &dbpass).await?;
    session.median_salary("Europe", |row| {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;
        println!("{country_name:25}: {median_salary:>5}");
        Ok(())
    }).await?;

    Ok(())
}