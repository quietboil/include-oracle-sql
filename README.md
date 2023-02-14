[![crates.io](https://img.shields.io/crates/v/include-oracle-sql)](https://crates.io/crates/include-oracle-sql)
[![Documentation](https://docs.rs/include-oracle-sql/badge.svg)](https://docs.rs/include-oracle-sql)
![MIT](https://img.shields.io/crates/l/include-oracle-sql.svg)

**include-oracle-sql** is an extension of [include-sql][1] for using Oracle SQL in Rust. It completes include-sql by providing `impl_sql` macro to generate database access methods from the included SQL. include-oracle-sql uses [Sibyl][2] for database access.

# Example

Write your SQL and save it in a file. For example, let's say the following is saved as `library.sql` in the project's `sql` folder:

```sql
-- name: get_loaned_books ?
--
-- Returns the list of books loaned to a patron
--
-- # Parameters
--
-- param: user_id: &str - user ID
--
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1

-- name: loan_books!
--
-- Updates the book records to reflect loan to a patron
--
-- # Parameters
--
-- param: book_titles: &str - book titles
-- param: user_id: &str - user ID
--
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_title IN (:book_titles)
```

And then use it in Rust as:

```rust , ignore
use include_oracle_sql::{include_sql, impl_sql};
use sibyl as oracle;

include_sql!("sql/library.sql");

fn main() -> oracle::Result<()> {
    let db_name = std::env::var("DBNAME").expect("database name");
    let db_user = std::env::var("DBUSER").expect("user name");
    let db_pass = std::env::var("DBPASS").expect("password");

    let oracle = oracle::env()?;
    let session = oracle.connect(&db_name, &db_user, &db_pass)?;

    db.loan_books(&["War and Peace", "Gone With the Wind"], "Sheldon Cooper")?;

    db.get_loaned_books("Sheldon Cooper", |row| {
        let book_title : &str = row.get("BOOK_TITLE")?;
        println!("{}", book_title);
        Ok(())
    })?;

    Ok(())
}
```

# Documentation

The included [documentation][3] describes the supported SQL file format and provides additional details on the generated code.

# 💥 Breaking Changes in 0.2

* [include-sql][1] changed optional statement terminator from `;` to `/`. SQL files that used `;` terminator would need to change it to `/` or remove it completely.

[1]: https://crates.io/crates/include-sql
[2]: https://crates.io/crates/sibyl
[3]: https://quietboil.github.io/include-oracle-sql
