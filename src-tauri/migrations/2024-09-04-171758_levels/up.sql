CREATE TABLE levels (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    current_level INTEGER NOT NULL,
    level_date DATETIME NOT NULL
);

INSERT INTO levels (current_level, level_date) VALUES (1, datetime('now'));