-- schema.sql
CREATE TABLE flashcards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    ease_factor REAL NOT NULL,
    interval INTEGER NOT NULL,
    next_review DATETIME NOT NULL
);
