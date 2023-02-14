use include_oracle_sql::*;

include_sql!("examples/prepare_query.sql");

#[cfg(not(feature = "tokio"))]
fn main() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

    let stmt = session.prepare_median_salary_query()?;
    let rows = stmt.query("Europe")?;
    while let Some(row) = rows.next()? {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;

        println!("{country_name:25}: {median_salary:>5}");
    }

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

    let stmt = session.prepare_median_salary_query().await?;
    let rows = stmt.query("Europe").await?;
    while let Some(row) = rows.next().await? {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;

        println!("{country_name:25}: {median_salary:>5}");
    }

    Ok(())
}