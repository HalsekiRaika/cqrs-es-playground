CREATE TABLE journal(
  aggregate_id TEXT     NOT NULL,
  sequence     INTEGER  NOT NULL,
  registry_key TEXT     NOT NULL,
  bytes        BLOB     NOT NULL,
  created_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

  PRIMARY KEY (aggregate_id, sequence)
);
