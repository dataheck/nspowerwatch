-- Your SQL goes here

CREATE TABLE outages
(
  id                 bigserial NOT NULL,
  datetime           TIMESTAMP NOT NULL,
  area_name          TEXT      NOT NULL,
  customers_affected BIGINT    NOT NULL,
  PRIMARY KEY (id)
);
