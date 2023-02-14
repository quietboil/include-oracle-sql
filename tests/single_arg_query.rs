use include_oracle_sql::*;

include_sql!("tests/single_arg_query.sql");

#[cfg(not(feature = "tokio"))]
#[test]
fn single_arg_query() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

    let mut row_num = 0;
    session.median_salary("Europe", |row| {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;

        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(country_name, "Germany");
                assert_eq!(median_salary, 10000);
            },
            2 => {
                assert_eq!(country_name, "United Kingdom");
                assert_eq!(median_salary, 8800);
            },
            _ => panic!("only 2 rows were expected")
        }
        Ok(())
    })?;

    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn single_arg_query() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass).await?;

    let mut row_num = 0;
    session.median_salary("Europe", |row| {
        let country_name: &str = row.get(0)?;
        let median_salary: u16 = row.get(1)?;

        row_num += 1;
        match row_num {
            1 => {
                assert_eq!(country_name, "Germany");
                assert_eq!(median_salary, 10000);
            },
            2 => {
                assert_eq!(country_name, "United Kingdom");
                assert_eq!(median_salary, 8800);
            },
            _ => panic!("only 2 rows were expected")
        }
        Ok(())
    }).await?;

    Ok(())
}
