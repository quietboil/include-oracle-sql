use include_oracle_sql::{include_sql, impl_sql};

include_sql!("examples/dyn_query.sql");

fn main() -> sibyl::Result<()> {
    let oracle = sibyl::env()?;

    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;
    let from_date = sibyl::Date::with_date(2005,  1,  1, &session);
    let thru_date = sibyl::Date::with_date(2007, 12, 31, &session);
    session.median_salary(&from_date, &thru_date, &["CA", "DE", "US", "UK"], |row| {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;
        println!("{:25}: {:>5}", country_name, median_salary);
        Ok(())
    })?;

    Ok(())
}