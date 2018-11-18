CREATE TABLE follows (
  follower INTEGER NOT NULL,
  followed INTEGER NOT NULL,
  FOREIGN KEY(follower) REFERENCES users(id),
  FOREIGN KEY(followed) REFERENCES users(id),
  PRIMARY KEY (follower, followed)
);

CREATE INDEX idx_follows_follower ON follows(follower);
CREATE INDEX idx_follows_followed ON follows(followed);
