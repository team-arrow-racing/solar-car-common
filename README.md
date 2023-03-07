# Solar Car

This is a common library shared between device projects to facilitate shared
communication and deduplication of utility code.

Whilst this is a cargo library, it is not intended to by published to the crate
registry.

## How to use

Since this project is not published to crates.io, please include it in your
project by referencing the Git repository directly.

```toml
[dependencies.solar-car]
version = "0.1.1"
git = "https://gitlab.com/team-arrow-racing/arrow-3/solar-car"
```

or via the CLI

```shell
cargo add solar-car --git "https://gitlab.com/team-arrow-racing/arrow-3/solar-car"
```

## What this should contain

- Utility functionality useful to more than one application.
- Drivers used by more than one device.
- Common communication code.
- Example code for design patterns we want to re-use.

## What shouldn't this contain

- Drivers that are better split out into their own crate. (see `wurth-calypso`).

## Other useful tools

- [Online DBC File Editor](https://www.csselectronics.com/pages/dbc-editor-can-bus-database) (can be downloaded also).
