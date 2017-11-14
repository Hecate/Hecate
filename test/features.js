const test = require('tape');
const request = require('request');
const exec = require('child_process').exec;
const Pool = require('pg-pool');

const pool = new Pool({
    database: 'hecate',
    user: 'postgres',
    port: 5432
});

test('Reset Database', (t) => {
    exec(`
        echo "
            SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE
                    pg_stat_activity.datname = 'hecate'
                    AND pid <> pg_backend_pid();
        " | psql -U postgres -q >/dev/null

        echo "
            DROP DATABASE hecate;
            CREATE DATABASE hecate;
        " | psql -U postgres -q

        psql -q -U postgres -f src/schema.sql hecate
    `, (err, stdout, stderr) => {
        t.error(err, 'no errors');
        t.end();
    });

});

if (!process.env.DEBUG) {
    test('Compile & Run', (t) => {
        exec(`
            pkill hecate || true

            cargo build
        `, (err, stdout, stderr) => {
            t.error(err, 'no errors');
            t.end();
        });
    });

    test('Start Server', (t) => {
        exec('cargo run');
        exec('sleep 2', (err, stdout, stderr) => {
            t.error(err, 'no errors');
            t.end();
        });
    });
}

test('features', (t) => {
    t.test('features - basic', (q) => {
        q.test('features - basic - endpoint', (r) => {
            request.post({
                headers: { 'content-type' : 'application/json' },
                url: 'http://localhost:3000/api/data/features',
                body: JSON.stringify({
                    type: 'FeatureCollection',
                    features: [{
                        type: 'Feature',
                        action: 'create',
                        properties: {
                            shop: true,
                        },
                        geometry: {
                            type: 'Point',
                            coordinates: [ 1,1 ]
                        }
                    }]
                })
            }, (err, res) => {
                r.error(err, 'no errors');
                r.equals(res.statusCode, 200);
                r.equals(res.body, 'true');
                r.end();
            });
        });

        q.test('features - basic - database', (r) => {
            pool.query('SELECT id, version, ST_AsGeoJSON(geom) AS geom, props, deltas FROM geo WHERE id = 1;', (err, res) => {
                r.error(err, 'no errors');
                r.deepEquals(res.rows[0], {
                    id: '1',
                    version: '1',
                    geom: '{"type":"Point","coordinates":[1,1]}',
                    props: { shop: true },
                    deltas: [ '1' ]
                });
                r.end();
            });
        });
        q.end();
    });
    t.end();
});

if (!process.env.DEBUG) {
    test('Stop Server', (t) => {
        exec(`
            pkill hecate || true
        `, (err, stdout, stderr) => {
            t.error(err, 'no errors');
            t.end();
        });
    });
}

/**
echo -e "\n# XML Map"
    echo $(curl -s -X GET 'localhost:3000/api/0.6/map?bbox=-1,-1,1,1')
    echo ""

echo -e "\n# XML Changeset Create"
    DATA='<osm><changeset><tag k="created_by" v="JOSM 1.61"/><tag k="comment" v="Just adding some streetnames"/></changeset></osm>'

    curl -s -X PUT --data "$DATA" 'localhost:3000/api/0.6/changeset/create'
    echo ""
    echo "SELECT id, props FROM deltas" | psql -U postgres hecate

echo -e "\n# Features Post"
    DATA='
        {
            "type": "FeatureCollection",
            "features": [
                { "action": "create", "type": "Feature", "properties": { "addr:number": "543" }, "geometry": {"type": "Point", "coordinates": [2.1, 2.1] } },
                { "id": 1, "action": "modify", "version": 2, "type": "Feature", "properties": { "addr:number": "543" }, "geometry": {"type": "Point", "coordinates": [2.2, 1.1] } }
            ]
        }
    '

    curl -i -s -X POST --data "$DATA" -H 'Content-Type: application/json' 'localhost:3000/api/data/features'
*/
