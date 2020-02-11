<h1 align='center'>Hecate</h1>

<p align=center>OpenStreetMap Inspired Data Storage Backend Focused on Performance and GeoJSON Interchange</p>

<p align="center">
  <a href="https://crates.io/crates/hecate"><img src="https://img.shields.io/crates/v/hecate.svg"/></a>
</p>

<img src="https://i.imgur.com/xLjiTqL.jpg"/>

## Hecate Feature Comparison

| Feature               | Hecate             | ESRI MapServer     | OSM Backend        |
| --------------------- | ------------------ | ------------------ | ------------------ |
| Vector Tile Creation  | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| Streaming Query API   | :heavy_check_mark: | :x:                | :x:                |
| Multi User Support    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| Feature History       | :heavy_check_mark: | :x:                | :heavy_check_mark: |
| Atomic API Operations | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| GeoJSON-LD Based API  | :heavy_check_mark: | :x:                | :heavy_check_mark: |
| Mapbox GL JS Styling  | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| Integrated Data Stats | :heavy_check_mark: | :heavy_check_mark: | :x:                |

## Table Of Contents

<details>

1. [Brief](#brief)
1. [Why Use Hecate](#why-use-hecate)
2. [Table of Contents](#table-of-contents)
3. [Related Libraries](#related-libraries)
4. [Build Environment](#build-environment)
5. [Docker File](#docker-file-coverage-tests)
6. [Feature Format](#feature-format)
7. [Server](#server)
    - [Database Connection](#database)
    - [JSON Validation](#json-validation)
    - [Custom Authentication](#custom-authentication)
8. [API](#api)

</details>

## Related Libraries

<details>

- [HecateJS](https://github.com/mapbox/HecateJS) Javascript Library & CLI Tool for interacting with the Hecate API
- [Hecate-Example](https://github.com/ingalls/hecate-example) Script for importing some fake data for testing

Built something cool that uses the Hecate API? Let us know!

</details>

## Build Environment

<details>

- Start by installing Rust from [rust-lang.org](https://www.rust-lang.org/en-US/), this will install the current stable version

```bash
curl https://sh.rustup.rs -sSf | sh
```

- Hecate is designed to run on the latest stable version of Rust, but has been thoroughly tested with `1.40.0`. This will install `1.40.0`

```bash
curl https://sh.rustup.rs -sSf | sh -s --  --default-toolchain 1.40.0
```

- Source your `bashrc/bash_profile` to update your `PATH` variable

```bash
source ~/.bashrc        # Most Linux Distros, some OSX
source ~/.bash_profile  # Most OSX, some Linux Distros
```

- Download and compile the project and all of it's libraries

```bash
cargo build
```

- Ensure you have database dependencies `postgres` and `postgis` installed.

- Create the `hecate` database using the provided schema file. These instructions assume you have set up a role `postgres` with sufficient privileges.

```bash
echo "CREATE DATABASE hecate;" | psql -U postgres

psql -U postgres -f src/schema.sql hecate
```

- This step will also create a database role called `hecate` and `hecate_read`. If
the connection fails due to authentication, your pg_hba file may not be set up
to trust local connections.

Your pb_hba file location can be found using `echo "show hba_file;" | psql -U postgres`

Replace the file with the following:

```
local all postgres trust
local all all trust
host all all 127.0.0.1/32 trust
host all all ::1/128 trust
host replication postgres samenet trust
```

- [Install Node/Npm](https://nodejs.org/en/) for building the frontend code

- [Install Yarn](https://yarnpkg.com/lang/en/docs/install/) for installing
  frontend dependencies

- Install frontend dependencies

```
cd web/
yarn install
```

- Build frontend UI

```
yarn build
```

Note: if actively working on developing the UI, a live reloading server
can be started via:

```
yarn dev
```

- Start the server

```bash
cargo run
```

- Test it is working - should respond with `HTTP200`

```bash
curl localhost:8000
```

You will now have an empty database which can be populated with your own data/user accounts.

If you want to populate the database with sample data for testing, [ingalls/hecate-example](https://github.com/ingalls/hecate-example)
has a selection of scripts to populate the database with test data.

</details>

## Docker File (Coverage Tests)

<details>

The Docker file is designed to give the user a testing environment to easily run rust tests.

Install docker and then run

```
docker build .

docker run {{HASH FROM ABOVE}}
```

</details>

## Feature Format

<details>

Hecate is designed as a GeoJSON first interchange and uses [standard GeoJSON](http://geojson.org/) with a couple additions
and exceptions as outlined below.

*Supported Geometry Types*
- `Point`
- `MultiPoint`
- `LineString`
- `MultiLineString`
- `Polygon`
- `MultiPolygon`

*Unsupported Geometry Types*
- `GeometryCollection`

### Additional Members

The following table outlines top-level members used by hecate to handle feature creation/modification/deletion.

Key/Value pairs in the `.properties` of a given feature are _never_ directly used by the server and are simply
passed through to the storage backend. This prevents potential conflicts between user properties and required
server members.

| Member    | Notes |
| :-------: | ----- |
| `id`      | The unique integer `id` of a given feature. Note that all features get a unique id accross GeoJSON Geometry Type |
| `version` | The version of a given feature, starts at `1` for a newly created feature |
| `action`  | Only used for uploads, the desired action to be performed. One of `create`, `modify`, `delete`, or `restore` |
| `key`     | `Optional` A String containing a value that hecate will ensure remains unique across all features. Can be a natural id (wikidata id, PID, etc), computed property hash, geometry hash etc. The specifics are left up to the client. Should an attempt at importing a Feature with a differing `id` but identical `key` be made, the feature with will be rejected, ensuring the uniqueness of the `key` values. By default this value will be `NULL`. Duplicate `NULL` values are allowed.
| `force`   | `Optional` Boolean allowing a user to override version locking and force UPSERT a feature. Disabled by default |

### Examples

#### Downloaded Features

```JSON
{
    "id": 123,
    "key": "Q1234",
    "version": 2,
    "type": "Feature",
    "properties": {
        "shop": true,
        "name": "If Pigs Could Fly"
    },
    "geometry": {
        "type": "Point",
        "coordinates": [0,0]
    }
}
```

Downloaded Features will return the integer `id` of the feature, the current `version` and the user supplied `properties` and `geojson`.
`action` is not applicable for downloaded features, it is only used on upload.

#### Create Features

```JSON
{
    "action": "create",
    "key": "11-22-33-44-1234",
    "type": "Feature",
    "properties": {
        "shop": true,
        "name": "If Pigs Could Fly"
    },
    "geometry": {
        "type": "Point",
        "coordinates": [0,0]
    }
}
```

A features being uploaded for creation must have the `action: create` property. Since an `id` and `version` have not yet been
assigned they must be omitted. Should an `id` be included it will be ignored. Adding a `version` property will throw an error.

Optionally create actions can use the `force: true` option to perform an `UPSERT` like option. In this mode the uploader must
specify the `key` value. Hecate will then `INSERT` the feature if the `key` value is new, if the `key` is already existing, the
existing feature will be overwritten with the forced feature. Note that this mode ignores version checks and is therefore unsafe.

Force Prerequisites
- Disabled by default, must be explicitly enabled via [Custom Authentication](#custom-authentication)
- Can only be performed on a feature with `action: create`
- Must specify a valid `key`

#### Modify Features

```JSON
{
    "id": 123,
    "key": "Fn4aAsJ30",
    "version": 1,
    "action": "modify",
    "type": "Feature",
    "properties": {
        "shop": true,
        "name": "If Pigs Could Fly"
    },
    "geometry": {
        "type": "Point",
        "coordinates": [0,0]
    }
}
```

A feature being uploaded for modification must have the `action: modify` as well as the `id` and `version` property. The `id` is the integer id of the feature to modify and the `version` property is the
current version of the feature as stored by the server. If the version uploaded does not match the version that the server has stored, the modify will fail. This prevents consecutive edits from conflicting.

Note that the modify operation is _not a delta operation_ and the full feature with the complete Geometry & All Properties must be included with each modify.

Also note that since the `id` pool is shared accross geometry types, an id is allowed to change it's geometry type. eg. If `id: 1` is a `Point` and then a subsequent `action: modify` with a `Polygon` geometry is performed, `id: 1` is allowed to switch to the new `Polygon` type.

#### Delete Features

```JSON
{
    "id": 123,
    "version": 1,
    "action": "delete",
    "type": "Feature",
    "properties": null,
    "geometry": null
}
```

A feature being uploaded for deletion must have the `action: delete` as well as the `id` and `version` property. See _Modify Features_ above for an explanation of those properties.

Note the `properties` and `geometry` attributes must still be included. They can be set to `null` or be their previous value. They will be ignored.

#### Restore Features

```JSON
{
    "id": 123,
    "version": 2,
    "key": "new-optional-key",
    "action": "restore",
    "type": "Feature",
    "properties": {
        "test": true,
        "random_array": [1, 2, 3]
    },
    "geometry": {
        "type": "Point",
        "coordinates": [ 12.34, 56.78 ]
    }
}
```

A feature being uploaded for restoration must have the `action: restore` as well as the `id` and `version` properties. A `restore` action is just a `modify` on a deleted feature.

Restore places the new given geometry/properties at the id specified. It does not automatically roll back the feature to it's state before deletion, if this is desired, one
must use the Feature History API to get the state before deletion and then perform the `restore` action.

Note: Restore will throw an error if an feature still exists.

</details>

## Server

This section of the guide goes over various options for launching the server

Hecate can be launched with default options with

```
cargo run
```

### Database

<details>

#### Main Connection

By default hecate will attempt to connect to `hecate@localhost:5432/hecate` for read/write
operations and simultaneously connect to `hecate_read@localhost:5432/hecate` for
sandboxed read only operations.

Note that only postgres w/ postgis enabled is supported.

This database should be created prior to launching hecate. For instructions on setting up the database
see the [Build Environment](#build-environment) section of this doc.

A custom database name, postgres user or port can be specified using the database flag.

*Example*

```bash
cargo run -- --database "<USER>:<PASSWORD>@<HOST>/<DATABASE>"

cargo run -- --database "<USER>@<HOST>/<DATABASE>"
```

#### Sandbox Connection

A second read-only account should also be created with permissions to SELECT from the
`geo` & `deltas` table. This endpoint will only be used for the `query` endpoint, which
allows arbitrary user query execution. A sample implementation can be found in the `schema.sql` document

Note: It is up to the DB Admin to ensure the permissions are limited in scope for this user. Hecate will
expose access to this user via the query endpoint.

If multiple instances of `database_sandbox` are present, hecate will load balance accross the multiple read instances.

```bash
cargo run -- --database_sandbox "<USER>:<PASSWORD>@<HOST>/<DATABASE>"

cargo run -- --database_sandbox "<USER>@<HOST>/<DATABASE>"

cargo run -- --database_sandbox "<USER>@<HOST>/<DATABASE>" --database_sandbox "<USER>@<HOST>/<DATABASE>"
```

#### Replica Connection [optional]

Finally, optionally multiple `--database_replica` conncetions can be specified which hecate
will use to load balance read traffic accross, alleviating capacity on the master db for write operations.

```bash
cargo run -- --database_replica "<USER>:<PASSWORD>@<HOST>/<DATABASE>"

cargo run -- --database_replica "<USER>@<HOST>/<DATABASE>"

cargo run -- --database_replica"<USER>@<HOST>/<DATABASE>" --database_replica "<USER>@<HOST>/<DATABASE>"
```


</details>

### JSON Validation

<details>

By default Hecate will allow any property on a given GeoJSON feature, including nestled arrays, maps, etc.

A custom property validation file can be specified using the schema flag.

*Example*

```bash
cargo run -- --schema <PATH-TO-SCHEMA>.json
```

Note hecate currently supports the JSON Schema draft-04. Once draft-06/07 support lands in
[valico](https://github.com/rustless/valico) we can support newer versions of the spec.

</details>

### Custom Authentication

<details>

By default the Hecate API is most favourable to a crowd-sourced data server. Any users
can access the data/vector tiles, users can create & manage data, and admins
can manage user accounts.

This provides a middle ground for most users but all endpoints are entirely configurable
and can run from a fully open server to fully locked down.

If the default values aren't suitable for what you intend, passing in an authentication
configuration JSON document will override the defaults.

*Example*

```
cargo run -- --auth path/to/auth.json
```

__Contents of auth.json__
```
{
    "endpoints": {
        "server": "public",
        "schema": null,
        "mvt": {
            "get": "user",
            "regen": "admin",
            "meta": null
        },
        "users": {
            "info": "admin",
            "create": "admin",
            "create_session": null
        },

        ....

    }
}
```

It is important to note that if custom authentication is used, _every_ category must be either disabled or have
an option for every sub category within it set. One cannot conditionally override only a subset of of the default options. This is for the security of private
servers, since adding a new API endpoint is a non-breaking change, the server checks that you have specified
a policy for every endpoint or are happy with just the defaults before it will start.

IE:

The below schema is invalid. Each category (schema, user, style) etc. must be specified as disabled or
have a map containing the auth for each subkey.

```
{
    "endpoint": {
        "schema": null
    }
}
```

#### Behavior Types

| Type          | Description |
| ------------- | ----------- |
| `"public"`    | Allow any authenticated or unauthenticated user access |
| `"admin"`     | Allow only users with the `access: 'admin'` property on their user accounts access |
| `"user"`      | Allow any user access to the endpoint |
| `"self"`      | Only the specific user or an admin can edit their own metadata |
| `"disabled"`  | Disable all access to the endpoint |

#### Endpoint Lookup

| Example Endpoint                      | Config Name               | Default       | Supported Behaviors           | Notes |
| ------------------------------------- | ------------------------- | :-----------: | ----------------------------- | :---: |
| `GET /api`                            | `server`                  | `public`      | All                           |       |
| **Server Meta**                       | `meta`                    |               | `null`                        | 2     |
| `GET /api/meta/<key>`                 | `meta::get`               | `public`      | All                           |       |
| `POST /api/meta/<key>`                | `meta::set`               | `admin`       | `user`, `admin`, `disabled`   |       |
| **JSON Schema**                       | `schema`                  |               | `null`                        | 2     |
| `GET /api/schema`                     | `schema::get`             | `public`      | All                           |       |
| **Custom Auth JSON**                  | `auth`                    |               | `null`                        | 2     |
| `GET /api/auth`                       | `auth::get`               | `public`      | All                           |       |
| **Mapbox Vector Tiles**               | `mvt`                     |               | `null`                        | 2     |
| `DELETE /api/tiles`                   | `mvt::delete`             | `admin`       | All                           |       |
| `GET /api/tiles/<z>/<x>/<y>`          | `mvt::get`                | `public`      | All                           |       |
| `GET /api/tiles/<z>/<x>/<y>/regen`    | `mvt::regen`              | `user`        | All                           |       |
| `GET /api/tiles/<z>/<x>/<y>/meta`     | `mvt::meta`               | `public`      | All                           |       |
| **Users**                             | `user`                    |               | `null`                        | 2     |
| `GET /api/users`                      | `user::list`              | `user`        | All                           |       |
| `GET /api/user/info`                  | `user::info`              | `self`        | `self`, `admin`, `disabled`   |       |
| `GET /api/create`                     | `user::create`            | `public`      | All                           |       |
| `GET /api/create/session`             | `user::create_session`    | `self`        | `self`, `admin`, `disabled`   |       |
| **Mapbox GL Styles**                  | `style`                   |               | `null`                        | 2     |
| `POST /api/style`                     | `style::create`           | `self`        | `self`, `admin`, `disabled`   |       |
| `PATCH /api/style`                    | `style::patch`            | `self`        | `self`, `admin`, `disabled`   |       |
| `POST /api/style/<id>/public`         | `style::set_public`       | `self`        | All                           |       |
| `POST /api/style/<id>/private`        | `style::set_private`      | `self`        | `self`, `admin`, `disabled`   |       |
| `DELETE /api/style/<id>`              | `style::delete`           | `self`        | `self`, `admin`, `disabled`   |       |
| `GET /api/style/<id>`                 | `style::get`              | `public`      | All                           | 1     |
| `GET /api/styles`                     | `style::list`             | `public`      | All                           | 1     |
| **Deltas**                            | `delta`                   |               | `null`                        | 2     |
| `GET /api/delta/<id>`                 | `delta::get`              | `public`      | All                           |       |
| `GET /api/deltas`                     | `delta::list`             | `public`      | All                           |       |
| **Webhooks**                          | `webhooks`                |               | `null`                        | 2     |
| `GET /api/webhooks/<id>`              | `webhooks::get`           | `admin`       | All                           |       |
| `POST /api/webhooks/<id>`             | `webhooks::set`           | `admin`       | All                           |       |
| **Data Stats**                        | `stats`                   | `public`      | All                           |       |
| `GET /api/data/stats`                 | `stats::get`              | `public`      | All                           |       |
| **Features**                          | `feature`                 |               | `null`                        | 2     |
| `POST /api/data/feature(s)`           | `feature::create`         | `user`        | `user`, `admin`, `disabled`   |       |
| `GET /api/data/feature/<id>`          | `feature::get`            | `public`      | All                           |       |
| `GET /api/data/feature/<id>/history`  | `feature::history`        | `public`      | All                           |       |
| `POST /api/data/feature(s) w/ `force` | `feature::force`          | `admin`       | `user`, `admin`, `disabled`   |       |
| **Clone**                             | `clone`                   |               | `null`                        | 2     |
| `GET /api/data/clone`                 | `clone::get`              | `user`        | All                           |       |
| `GET /api/data/query`                 | `clone::query`            | `user`        | All                           |       |
| **Bounds**                            | `bounds`                  |               | `null`                        | 2     |
| `GET /api/bounds`                     | `bounds::list`            | `public`      | All                           |       |
| `GET /api/bounds/<id>`                | `bounds::get`             | `public`      | All                           |       |
| `POST /api/bounds/<id>`               | `bounds::create`          | `admin`       | All                           |       |
| `DELETE /api/bounds/<id>`             | `bounds:delete`           | `admin`       | All                           |       |
| **OpenStreetMap Shim**                | `osm`                     |               | `null`                        | 2     |
| `GET /api/0.6/map`                    | `osm::get`                | `public`      | All                           | 3     |
| `PUT /api/0.6/changeset/<id>/upload`  | `osm::create`             | `user`        | `user`, `admin`, `disabled`   | 3     |

*Notes*

1. This only affectes `public` styles. The `private` attribute on a style overrides this. A `private` style can _never_ be seen publicly regardless of this setting.
2. This is a category, the only valid option is `null` this will disable access to the endpoint entirely
3. OSM software expects the authentication on these endpoints to mirror OSM. Setting these to a non-default option is supported but will likely have unpredicable
support when using OSM software. If you are running a private server you should disable OSM support entirely.

</details>

---
<h2 align='center'>API</h2>

Our API docs have been moved to 

<h3 align='center'>OpenStreetMap API</h3>

The primary goal of the hecate project is a very fast GeoJSON based Interchange. That said, the tooling the OSM community has built around editing is unparalleled. As such,
Hecate provides a Work-In-Progress OpenStreetMap Shim to support a subset of API operations as defined by the [OSM API v0.6](httpl://wiki.openstreetmap.org/wiki/API_v0.6) document.

<details>

*Important Notes*
- All GeoJSON types can be downloaded via the API and viewed in JOSM
- MultiPoints
    - Are represented using an OSM  `Relation`
    - The type will be `multipoint`
    - The member type will be `point`
- MultiLineStrings
    - Are represented using an OSM `Relation`
    - The type will be `multilinestring`
    - The member will be `line`
- Uploading `Way` & `Relation` types are not currently supported, attempting to upload them may produce undesirable results.

The following incomplete list of endpoints are implemented with some degree of coverage with the OSM API Spec but are likely incomplete/or written with the minimum flexibility required to
support editing from JOSM. See the code for a full list.

#### `GET` `/api/capabilities`
#### `GET` `/api/0.6/capabilities`

Return a static XML document describing the capabilities of the API.

*Example*

```bash
curl -X GET 'http://localhost:8000/api/capabilities'
```

---

#### `GET` `/api/0.6/user/details` *Auth Required*

Returns a static XML document describing the number of unread messages that a user has. Every n minutes JOSM checks
this and displays in the interface if there is a new message, to cut down on errors it simply returns a 0 message response.

*Example*

```bash
curl -X GET 'http://localhost:8000/api/0.6/user/details'
```

---

#### `PUT` `/api/0.6/changeset/create` *Auth Required*

Create a new changeset and set the meta information, returning the opened id.

*Example*

```bash
curl \
    -X PUT \
    -d '<osm><changeset><tag k="comment" v="Just adding some streetnames"/></changeset></osm>' \
    'http://localhost:8000/api/0.6/changeset/create
```

---

#### `GET` `/api/0.6/changeset/<changeset_id>/upload` *Auth Required*

Upload osm xml data to a given changeset

*Example*

```bash
curl \
    -X POST \
    -d '<diffResult version="0.6">NODE/WAY/RELATIONS here</diffResult>' \
    'http://localhost:8000/api/0.6/changeset/1/upload'
```

---

#### `PUT` `/api/0.6/changeset/<changeset_id>/close` *Auth Required*

Close a given changeset, preventing further modification to it

*Example*

```bash
curl -X PUT 'http://localhost:8000/api/0.6/changeset/1/close'
```

</details>

---
