CREATE TABLE reviews (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    card_id INTEGER REFERENCES flashcards(id) NOT NULL,
    review_date DATETIME NOT NULL,  -- date of review, when passed or failed
    duration_front REAL NOT NULL,   -- time looking at front of card
    duration_back REAL NOT NULL,    -- time looking at back of card
    correct BOOLEAN NOT NULL,       -- whether the answer was correct
    interval INTEGER NOT NULL       -- new interval after review
);
