CREATE TABLE asks (
    identifier INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    ipAddress TEXT NOT NULL,
    userAgent TEXT NOT NULL,
    createdAt INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    deleteAfter INTEGER NOT NULL DEFAULT (strftime('%s', 'now', '+3 days'))
)