CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  actor INTEGER NOT NULL,
  object VARCHAR NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(actor) REFERENCES users(id),
  UNIQUE (actor, object)
);

CREATE INDEX idx_posts_object ON posts(object);
CREATE INDEX idx_posts_actor ON posts(actor);
CREATE INDEX idx_posts_created_at ON posts(created_at);
