CREATE TABLE IF NOT EXISTS Account (
    username TEXT PRIMARY KEY,
    name     TEXT NOT NULL,
    image    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Request (
    id          SERIAL PRIMARY KEY,
    title       TEXT NOT NULL,
    category    TEXT NOT NULL,
    upvotes     INTEGER DEFAULT 0,
    upvoted     BOOLEAN DEFAULT FALSE,
    status      TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Comment (
    id         SERIAL PRIMARY KEY,
    owner      TEXT NOT NULL REFERENCES Account(username),
    content    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS RequestComment (
    id         SERIAL PRIMARY KEY,
    id_request INTEGER NOT NULL REFERENCES Request(id),
    id_comment INTEGER NOT NULL REFERENCES Comment(id)
);

CREATE TABLE IF NOT EXISTS CommentReply (
    id        SERIAL PRIMARY KEY,
    id_parent INTEGER NOT NULL REFERENCES Comment(id),
    id_reply  INTEGER NOT NULL REFERENCES Comment(id)
);