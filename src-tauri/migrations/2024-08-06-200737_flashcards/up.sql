CREATE TABLE flashcards (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    content_front TEXT NOT NULL,
    content_back TEXT NOT NULL,
    interval INTEGER NOT NULL,
    next_review DATETIME NOT NULL
);