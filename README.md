<h1 align='center'>Hecate</h1>

## Brief

OpenStreetMap Inspired Data Storage Backend Focused on Performance and GeoJSON Interchange

## Table Of Contents

1. [Brief](#brief)
2. [Table of Contents](#table-of-contents)
3. [Docker File](#docker-file-coverage-tests)
4. [Feature Format](#feature-format)
5. [API](#api)
    - [User Options](#user-options)
    - [Downloading via Boundaries](#downloading-via-boundaries)
    - [Downloading Individual Features](#downloading-individual-features)
    - [Downloading Multiple Features via BBOX](#downloading-multiple-features-via-bbox)
    - [Feature Creation](#feature-creation)
    - [OpenStreetMap API](#openstreetmap-api)

## Docker File (Coverage Tests)

The Docker file is designed to give the user a testing environment to get tests up and running and be able to view coverage information.

Install docker and then run

```
docker build .

docker run  --security-opt seccomp=unconfined {{HASH FROM ABOVE}}
```

The --security-opt flag is required to be able to run and view `kcov` output.

## Feature Format

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
| `action`  | Only used for uploads, the desired action to be performed. One of `create`, `modify` or `delete` |


### Examples

#### Downloaded Features

```JSON
{
    "id": 123,
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
assigned they should be omitted. Should an `id` or `version` be included the server will ignore them, assigning a new
`id` and `version` per the servers internal id provisioner.

#### Modify Features

```JSON
{
    "id": 123,
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

### Samples

## API

### Index

#### `GET` `/`

Healthcheck URL, currently returns `Hello World!`

*Example*

```bash
curl -X GET 'http://localhost:8000/
```

---

<h3 align='center'>User Options</h3>

#### `GET` `/api/user/create`

Create a new user, provied the username & password are not already taken

*Required Options*

| Option     | Notes |
| :--------: | ----- |
| `username` | `REQUIRED` Desired username, must be unique |
| `password` | `REQUIRED` Desired password |
| `email`    | `REQUIRED` Desired email, must be unique |

*Example*

```bash
curl -X GET 'http://localhost:8000/api/user/create?ingalls&password=yeaheh&email=ingalls@protonmail.com
```

---

<h3 align='center'>Downloading via Boundaries</h3>

#### `GET` `/api/data/bounds/`

Return an array of possible boundary files with which data can be extracted from the server with

*Example*

```bash
curl -X GET 'http://localhost:8000/api/data/bounds
```

---

#### `GET` `/api/data/bounds/<bounds>`

Return line delimited GeoJSON `Feature` of all the geometries within the specified boundary file.

*Required Options*

| Option     | Notes |
| :--------: | ----- |
| `<bounds>` | One of the boundary files as specified via the `/ap/data/bounds` |

*Example*

```bash
curl -X GET 'http://localhost:8000/api/data/bounds/us_dc
```

---

<h3 align='center'>Downloading Individual Features</h3>

#### `GET` `/api/data/feature/<id>`

Return a single GeoJSON `Feature` given its' ID.

*Required Options*

| Option | Notes |
| :----: | ----- |
| `<id>` | Numeric ID of a given feature to download |

*Example*

```bash
curl -X GET 'http://localhost:8000/api/data/features/1542
```

---

<h3 align='center'>Downloading Multiple Features via BBOX</h3>

#### `GET` `/api/data/features`

Return a `FeatureCollection` of all features within a given bbox

*Required Options*

| Option | Notes |
| :----: | ----- |
| `bbox` | `REQUIRED` Bounding Box in format `left,bottom,right,top` |

---

<h3 align='center'>Feature Creation</h3>

#### `POST` `/api/data/feature` *Auth Required*

Create, Modify, or Delete an individual GeoJSON `Feature`

The Feature must follow format defined in [Feature Format](#feature-format).

*Example*

```bash
curl \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"action": "create", "type":"Feature","properties":{"shop": true},"geometry":{"type":"Point","coordinates":[0,0]}}' \
    'http://username:password@localhost:8000/api/data/feature'
```

---

#### `POST` `/api/data/features` *Auth Required*

Create, Modify, and/or Delete many features via a GeoJSON `FeatureCollection`

The Feature must follow format defined in [Feature Format](#feature-format).

Note that a mix of `create`, `modify`, and `delete` operatioons are allowed
within each `FeatureCollection`

*Example*

```bash
curl \
    -X POST \
    -H "Content-Type: application/json" \
    -d '{"type":"FeatureCollection","features": [{"action": "create", "type":"Feature","properties":{"shop": true},"geometry":{"type":"Point","coordinates":[0,0]}}]}' \
    'http://username:password@localhost:8000/api/data/features'
```

---

<h3 align='center'>OpenStreetMap API</h3>

The primary goal of the hecate project is a very fast GeoJSON based Interchange. That said, the tooling the OSM community has built around editing is unparalled. As such, 
Hecate provides a Work-In-Progress OpenStreetMap Shim to support a subset of API operations as defined by the [OSM API v0.6](httpl://wiki.openstreetmap.org/wiki/API_v0.6) document.

*Current Gotchas*
- Only `Way` & `Relation` editing is not currentltly supported. (`Node` create/modify/delete should work 100%)

The following incomplete list of endpoints are implemented with some degree of coverage with the OSM API Spec but are likely incomplete/or written with the minimum flexibility required to
support editing from JOSM. See the code for a full list.

#### `GET` `/api/capabilities`
#### `GET` `/api/0.6/capabilities`

Return a static XML document describing the capabilities of the API.

*Example*

```bash
curl -X GET 'http://localhost:8000/api/capabilities
```

---

#### `GET` `/api/0.6/user/details` *Auth Required*

Returns a static XML document describing the number of unread messages that a user has. Every n minutes JOSM checks
this and displays in the interface if there is a new message, to cut down on errors it simply returns a 0 message response.

*Example*

```bash
curl -X GET 'http://localhost:8000/api/0.6/user/details
```

---

#### `PUT` `/api/0.6/changeset/create` *Auth Required*

Create a new changset and set the meta information, returning the opened id.

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

---
