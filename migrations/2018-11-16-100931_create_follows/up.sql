CREATE TABLE follows (
  id INTEGER NOT NULL PRIMARY KEY,
  follower INTEGER NOT NULL,
  followed INTEGER NOT NULL,
  FOREIGN KEY(follower) REFERENCES users(id),
  FOREIGN KEY(followed) REFERENCES users(id)
);

CREATE INDEX idx_follows_follower ON follows(follower);
CREATE INDEX idx_follows_followed ON follows(followed);
