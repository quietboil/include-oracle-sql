use include_oracle_sql::*;

include_sql!("tests/prepare_query.sql");

#[cfg(not(feature = "tokio"))]
#[test]
fn prepare_query() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

    let stmt = session.prepare_median_salary_query()?;
    let rows = stmt.query("Europe")?;
    let mut row_num = 0;
    while let Some(row) = rows.next()? {
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
    }
    assert_eq!(stmt.row_count()?, 2);

    let stmt = session.prepare_get_number_of_departments_query()?;
    let mut count : usize = 0;
    if let Some(row) = stmt.query_single(())? {
        count = row.get(0)?;
    }
    assert_eq!(count, 27);

    Ok(())
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn prepare_query() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass).await?;

    let stmt = session.prepare_median_salary_query().await?;
    let rows = stmt.query("Europe").await?;
    let mut row_num = 0;
    while let Some(row) = rows.next().await? {
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
    }
    assert_eq!(stmt.row_count()?, 2);

    let stmt = session.prepare_get_number_of_departments_query().await?;
    let mut count : usize = 0;
    if let Some(row) = stmt.query_single(()).await? {
        count = row.get(0)?;
    }
    assert_eq!(count, 27);

    Ok(())
}