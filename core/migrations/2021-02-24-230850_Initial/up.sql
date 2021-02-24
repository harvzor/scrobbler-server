CREATE TABLE trackables (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  colour VARCHAR NOT NULL,
  deleted BOOLEAN NOT NULL DEFAULT 'f'
)

CREATE TABLE scrobbles (
  id SERIAL PRIMARY KEY,
  trackable_id INT NOT NULL,
  timestamp timestamp NOT NULL,
  CONSTRAINT fk_trackables
    FOREIGN KEY(trackable_id) 
        REFERENCES trackables(id)
)
