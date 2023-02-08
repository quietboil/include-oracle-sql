[![crates.io](https://img.shields.io/crates/v/include-oracle-sql)](https://crates.io/crates/include-oracle-sql)
[![Documentation](https://docs.rs/include-oracle-sql/badge.svg)](https://docs.rs/include-oracle-sql)
![MIT](https://img.shields.io/crates/l/include-oracle-sql.svg)

**include-oracle-sql** is an extension of [include-sql][1] for using Oracle SQL in Rust. It completes include-sql by providing `impl_sql` macro to generate database access methods from the included SQL. include-oracle-sql uses [Sibyl][2] for database access.

# Example

Write your SQL and save it in a file. For example, let's say the following is saved as `library.sql` in the project's `src` folder:

```sql
-- name: get_loaned_books?
-- Returns the list of books loaned to a patron
-- # Parameters
-- param: user_id: &str - user ID
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1;

-- name: loan_books!
-- Updates book records to reflect loan to a patron
-- # Parameters
-- param: user_id: &str - user ID
-- param: book_ids: usize - book IDs
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_id IN (:book_ids);
```

And then use it in Rust as:

```rust , ignore
use include_oracle_sql::{include_sql, impl_sql};
use sibyl as oracle;

include_sql!("src/library.sql");

fn main() -> oracle::Result<()> {
    let db_name = std::env::var("DBNAME").expect("database name");
    let db_user = std::env::var("DBUSER").expect("user name");
    let db_pass = std::env::var("DBPASS").expect("password");
    let user_id = std::env::var("USERID").expect("library user ID");

    let oracle = oracle::env()?;
    let session = oracle.connect(&db_name, &db_user, &db_pass)?;

    session.get_loaned_books(user_id, |row| {
        let book_title : &str = row.get("BOOK_TITLE")?;
        println!("{}", book_title);
        Ok(())
    })?;

    Ok(())
}
```

# Documentation

The included [documentation][3] describes the supported SQL file format and provides additional details on the generated code.

[1]: https://crates.io/crates/include-sql
[2]: https://crates.io/crates/sibyl
[3]: https://quietboil.github.io/include-oracle-sql
