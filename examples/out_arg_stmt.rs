use include_oracle_sql::*;

include_sql!("examples/out_arg_stmt.sql");

#[cfg(not(feature = "tokio"))]
fn main() -> sibyl::Result<()> {
    let dbname = std::env::var("DBNAME").expect("database name");
    let dbuser = std::env::var("DBUSER").expect("user name");
    let dbpass = std::env::var("DBPASS").expect("password");

    let oracle = sibyl::env()?;
    let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

    let mut department_id = 0u32;
    session.new_deparment("Security", "Seattle", &mut department_id)?;
    println!("department_id = {department_id}");
    session.rollback()?;

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

    let mut department_id = 0u32;
    session.new_deparment("Security", "Seattle", &mut department_id).await?;
    println!("department_id = {department_id}");
    session.rollback().await?;

    Ok(())
}