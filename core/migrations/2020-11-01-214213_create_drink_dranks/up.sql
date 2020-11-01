CREATE TABLE drink_dranks (
  id SERIAL PRIMARY KEY,
  drink_id INT NOT NULL,
  drank_timestamp timestamp NOT NULL,
  CONSTRAINT fk_drink
    FOREIGN KEY(drink_id) 
        REFERENCES drinks(id)
)
