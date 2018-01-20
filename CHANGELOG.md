# CHANGELOG

## Emoji Cheatsheet
- :pencil2: doc updates
- :bug: when fixing a bug
- :rocket: when making general improvements
- :white_check_mark: when adding tests
- :arrow_up: when upgrading dependencies
- :tada: when adding new features

## Version History

## 0.5.0

- :rocket: Add support for `multipoint`, `polygon`, `multilinestrint`, `multipolygon` via XML download shim

## 0.4.0

- :tada: Service Vector Tiles directly from data for web interface
- :pencil2: Doc new tile endpoint and admin interface

## v0.3.1

- :pencil2: Huge doc update on endpoints & setting up
- :rocket: Update a ton of install & Docker instructions

## v0.3.0 

- :tada: Add bounds API

## v0.2.0

- :tada: Authentication on all endpoints!

## v0.1.3

- :rocket: Add CLI Options

## v0.1.2

- :bug: Track package.json & Cargo.toml

## v0.1.1

- :tada: `409 CONFLICT` errors are now thrown when a changeset is closed that the caller tries to access/append to

## v0.1.0

- :rocket: The first general release
  - Basic OSMXML support for points & sketchy line/simply poly suport
  - Full delta/geojson support
  - opening/committing/finalizing deltas
  - Pretty thorough suite of JS tests for JS integration
  - Basic Rust tests for node/way/rel objects

