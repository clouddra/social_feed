CREATE TABLE likes (
  id INTEGER NOT NULL PRIMARY KEY,
  actor INTEGER NOT NULL,
  object VARCHAR NOT NULL,
  target INTEGER NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(target) REFERENCES users(id),
  FOREIGN KEY(actor) REFERENCES users(id),
  UNIQUE (actor, object)
);

CREATE INDEX idx_likes_object ON likes(object);
CREATE INDEX idx_likes_target ON likes(target);
CREATE INDEX idx_likes_actor ON likes(actor);
CREATE INDEX idx_likes_created_at ON likes(actor);
