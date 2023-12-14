-- Your SQL goes here

ALTER TABLE organizations ADD COLUMN types
SMALLINT NOT NULL DEFAULT 1;
ALTER TABLE places ADD COLUMN types
SMALLINT NOT NULL DEFAULT 1;

ALTER TABLE organizations_places DROP COLUMN district_id;
ALTER TABLE organizations_places DROP COLUMN lat;
ALTER TABLE organizations_places DROP COLUMN lon;
ALTER TABLE organizations_places ADD COLUMN address2
VARCHAR(300) NOT NULL DEFAULT '';
ALTER TABLE organizations_places ALTER COLUMN city_id
SET NOT NULL;

ALTER TABLE services DROP COLUMN city_id;
