CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS hstore;

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id          BIGSERIAL,
    username    TEXT,
    password    TEXT,
    email       TEXT,
    meta        JSONB
);

DROP TABLE IF EXISTS geo;
CREATE TABLE geo (
    id          BIGSERIAL,
    version     BIGINT,
    geom        GEOMETRY(GEOMETRY, 4326),
    props       JSONB,
    deltas      BIGINT[]
);

DROP TABLE IF EXISTS deltas;
CREATE TABLE deltas (
    id          BIGSERIAL,
    created     TIMESTAMP,
    features    JSONB,
    affected    BIGINT[],
    props       JSONB,
    uid         BIGINT
);

-- delete_geo( id, version )
CREATE OR REPLACE FUNCTION delete_geo(BIGINT, BIGINT)
    RETURNS boolean AS $$
    BEGIN
        DELETE FROM geo
            WHERE
                id = $1
                AND version + 1 = $2;

        IF NOT FOUND THEN
            RAISE EXCEPTION 'DELETE: ID or VERSION Mismatch';
        END IF;

        RETURN true;
    END;
    $$ LANGUAGE plpgsql;

-- modify_geo( geom_str, props_str, delta, id, version)
CREATE OR REPLACE FUNCTION modify_geo(TEXT, TEXT, BIGINT, BIGINT, BIGINT)
    RETURNS boolean AS $$
    BEGIN
        UPDATE geo
            SET
                version = $5,
                geom = ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                props = $2::TEXT::JSON,
                deltas = array_append(deltas, $3::BIGINT)
            WHERE
                id = $4
                AND version + 1 = $5;

        IF NOT FOUND THEN
            RAISE EXCEPTION 'MODIFY: ID or VERSION Mismatch';
        END IF;

        RETURN true;
    END;
    $$ LANGUAGE plpgsql;
