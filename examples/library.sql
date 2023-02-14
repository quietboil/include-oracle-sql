-- name: create_library!
--
-- Creates the demo library
--
DECLARE
  name_already_used EXCEPTION; PRAGMA EXCEPTION_INIT(name_already_used, -955);
BEGIN
  EXECUTE IMMEDIATE '
    CREATE TABLE library (
      book_author VARCHAR2(100),
      book_title  VARCHAR2(100),
      loaned_to   VARCHAR2(100),
      loaned_on   TIMESTAMP WITH LOCAL TIME ZONE
    )
  ';
EXCEPTION
  WHEN name_already_used THEN 
    NULL;
END;

-- name: init_library!
--
-- Initializes the demo library
--
DECLARE
  num_books NUMBER;
BEGIN
  SELECT Count(*) INTO num_books FROM library;
  IF num_books = 0 THEN
    INSERT INTO library (book_author, book_title) VALUES ('Jane Austen', 'Pride and Prejudice');
    INSERT INTO library (book_author, book_title) VALUES ('Charlotte Bronte', 'Jane Eyre');
    INSERT INTO library (book_author, book_title) VALUES ('Leo Tolstoy', 'War and Peace');
    INSERT INTO library (book_author, book_title) VALUES ('Gustave Flaubert', 'Madame Bovary');
    INSERT INTO library (book_author, book_title) VALUES ('George Eliot', 'Middlemarch');
    INSERT INTO library (book_author, book_title) VALUES ('John Milton', 'Paradise Lost');
    INSERT INTO library (book_author, book_title) VALUES ('Patrick O’Brian', 'Master and Commander');
    INSERT INTO library (book_author, book_title) VALUES ('Margaret Mitchell', 'Gone With the Wind');
    INSERT INTO library (book_author, book_title) VALUES ('Boris Pasternak', 'Doctor Zhivago');
    INSERT INTO library (book_author, book_title) VALUES ('J. R. R. Tolkien', 'The Lord of the Rings');
    INSERT INTO library (book_author, book_title) VALUES ('A. A Milne', 'Winnie the Pooh');
  END IF;
END;

-- name: drop_library!
--
-- Drops the demo library
--
DROP TABLE library

-- name: get_loaned_books?
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
