**include-oracle-sql** is an extension of [include-sql][1] for using Oracle SQL in Rust. It completes include-sql by providing `impl_sql` macro to generate database access methods from the included SQL. include-oracle-sql uses [Sibyl][2] for database access.

# Usage

Add `include-oracle-sql` as a dependency:

```toml
[dependencies]
include-oracle-sql = "0.1"
```

Write your SQL and save it in a file. For example, let's say the following is the content of the `library.sql` file that is saved in the project's `sql` folder:

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

```rust
use include_oracle_sql::{include_sql, impl_sql};
use sibyl as oracle;

include_sql!("sql/library.sql");

fn main() -> oracle::Result<()> {
    let db_name = std::env::var("DBNAME").expect("database name");
    let db_user = std::env::var("DBUSER").expect("user name");
    let db_pass = std::env::var("DBPASS").expect("password");
    let user_id = std::env::var("USERID").expect("library user ID");

    let oracle = oracle::env()?;
    let session = oracle.connect(&db_name, &db_user, &db_pass)?;

    session.get_loaned_books(user_id, |row| {
        let book_title : &str = row.get("BOOK_TITLE")?;
        println!("{book_title}");
        Ok(())
    })?;

    Ok(())
}
```

> **Note** that the path to the SQL file must be specified relative to the project root, i.e. relative to `CARGO_MANIFEST_DIR`. Because include-sql targets stable Rust this requirement will persist until [SourceFile][3] stabilizes.

# Anatomy of the Included SQL File

Please see the **Anatomy of the Included SQL File** in [include-sql][4] documentation for the description of the format that include-sql can parse.

# Generated Methods

**include-oracle-sql** generates 3 variants of database access methods using the following selectors:
* `?` - methods that process rows retrieved by `SELECT`,
* `!` - methods that execute all other non-`SELECT` methods.
* `.` - methods that only prepare and return prepared statements.

> **Note** that `.` methods are nothing more than helpers that wrap `sibyl::Statement::prepare()`. While they do very little, they allow one to handle scenarios which might be difficult to process otherwise and still keep the SQL code in a separate file.

## Process Selected Rows

For the `SELECT` statement like:

```sql
-- name: get_loaned_books?
-- param: user_id: &str
SELECT book_title FROM library WHERE loaned_to = :user_id;
```

The method with the following signature is generated:

```rust
fn get_loaned_books<F>(&self, user_id: &str, row_callback: F) -> sibyl::Result<()>
where F: FnMut(sibyl::Row<'_>) -> sibyl::Result<()>;
```

Where:
- `user_id` is a parameter that has the same name as the SQL parameter with the declared (in the SQL) type as `&str`.
- `F` is a type of a callback (closure) that the method implementation will call to process each row.

## Execute Non-Select Statements

For non-select statements - INSERT, UPDATE, DELETE, etc. - like the following:

```sql
-- name: loan_books!
-- param: user_id: &str
-- param: book_ids: usize
UPDATE library
   SET loaned_to = :user_id
     , loaned_on = current_timestamp
 WHERE book_id IN (:book_ids);
```

The method with the following signature is generated:

```rust
fn loan_books(&self, user_id: &str, book_ids: &[usize]) -> sibyl::Result<usize>;
```

Where:
- `user_id` is a parameter that has the same name as the SQL parameter with the declared (in the SQL) type as `&str`,
- `book_ids` is a parameter for the matching IN-list parameter where each item in a collection has type `usize`.

## RETURNING, OUT, INOUT Statements

For DELETE, INSERT, and UPDATE statements that return data via `RETURNING` clause like:

```sql
-- name: add_new_book!
-- param: isbn: &str
-- param: book_title: &str
-- param: book_id: &mut usize
INSERT INTO library ( isbn, book_title )
VALUES ( :isbn, :book_title )
RETURN book_id INTO :book_id;
```

The method with the following signature is generated:

```rust
fn add_new_book(&self, isbn: &str, book_title: &str, book_id: &mut usize) -> sibyl::Result<usize>;
```

## Prepared Statements

When a statement name in the SQL file is tagged with `.` **include-oracle-sql** will generate a method that only prepares a statement and returns it:

```sql
-- name: prepare_loaned_books_query.
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1;
```

The generated method will have the following signature:

```rust
fn prepare_loaned_books_query(&self) -> sibyl::Result<sibyl::Statement>;
```

> **Note** that in this case the SQL parameters are ignored and it becomes a user's responsibility to pass them properly to the `sibyl::Statement::execute()` or `sibyl::Statement::query()` calls. [prepare_query.rs][5] provides a simple example.

# Inferred Parameter Types

If a statement parameter type is not explicitly specified via `param:`, include-oracle-sql will use `impl sibyl::ToSql` for the corresponding method parameters. For example, if the SQL from the example above has not provided its parameter type:

```sql
-- name: get_loaned_books?
-- Returns the list of books loaned to a patron
SELECT book_title
  FROM library
 WHERE loaned_to = :user_id
 ORDER BY 1;
```

Then the signature of the generated method would be:

```rust
/// Returns the list of books loaned to a patron
fn get_loaned_books<F>(&self, user_id: impl sibyl::ToSql, row_callback: F) -> sibyl::Result<()>
where F: Fn(sibyl::Row<'_>) -> sibyl::Result<()>;
```

> **Note** that include-sql is not able to infer whether a parameter needs to to be `mut` without a `param:` type annotation. Therefore an output parameter used as a RETURNING, OUT, or INOUT parameter must be annotated via `param:`

[1]: https://crates.io/crates/include-sql
[2]: https://crates.io/crates/sibyl
[3]: https://doc.rust-lang.org/proc_macro/struct.SourceFile.html
[4]: https://quietboil.github.io/include-sql
[5]: /examples/prepare_query.rs