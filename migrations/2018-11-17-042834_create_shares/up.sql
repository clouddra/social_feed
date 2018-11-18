CREATE TABLE shares (
  id INTEGER NOT NULL PRIMARY KEY,
  actor INTEGER NOT NULL,
  object VARCHAR NOT NULL,
  target INTEGER NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(target) REFERENCES users(id),
  FOREIGN KEY(actor) REFERENCES users(id),
  UNIQUE (actor, object)
);

CREATE INDEX idx_shares_object ON shares(object);
CREATE INDEX idx_shares_target ON shares(target);
CREATE INDEX idx_shares_actor ON shares(actor);
CREATE INDEX idx_shares_created_at ON shares(created_at);
