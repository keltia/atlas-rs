# Atlas-rs

[![CircleCI](https://circleci.com/gh/keltia/atlas-rs/tree/main.svg?style=shield)](https://circleci.com/gh/keltia/atlas-rs/tree/main)
[![dependency status](https://deps.rs/repo/github/keltia/atlas-rs/status.svg)](https://deps.rs/repo/github/keltia/atlas-rs)
[![](https://img.shields.io/crates/v/atlas-rs.svg)](https://crates.io/crates/atlas-rs)
[![Docs](https://docs.rs/atlas-rs/badge.svg)](https://docs.rs/atlas-rs)

[![SemVer](http://img.shields.io/SemVer/2.0.0.png)](https://semver.org/spec/v2.0.0.html)
[![License](https://img.shields.io/badge/license-MIT-red.svg?style=flat)](https://raw.githubusercontent.com/keltia/ripe-atlas/master/LICENSE)

`ripe-atlas` is a [Rust](https://rust-lang.org/) library to access the RIPE Atlas [REST API](https://atlas.ripe.net/docs/api/v2/manual/).  It is a rewrite in Rust of my Go library called [ripe-atlas](https://github.com/keltia/ripe-atlas).

It features a simple CLI-based tool called `atlas` which serve both as a collection of use-cases for the library and an easy way to use it.

**Work in progress, still incomplete**

- [Features](#features)
- [Installation](#installation)
- [API usage](#api-usage)
    - [Basics](#basics)
- [CLI Utility](#cli-utility)
    - [Configuration](#configuration)
    - [Proxy Authentication](#proxy-authentication)
    - [Usage](#usage)
- [TODO](#todo)
- [External Documentation](#external-documentation)
- [Contributing](#contributing)

## Features

I am trying to implement the full REST API in Rust.  The API itself is not particularly complex but the settings and parameters are.

The following topic are available:

- probes

  you can query one probe or ask for a list of probes with a few criterias

- measurements

  you can create and list measurements.

- results

  every measurement has a URI in the result json that points to the actual results. This fetch and display them.

In addition to these major commands, there are a few shortcut commands (see below):

- dns
- http
- ip
- keys
- ntp
- ping
- sslcert/tls
- traceroute

## Installation

This will be available as a crate on [crate.io](https://crate.io/atlas-rs) when it is in a release-able shape.

### Basics

- Authentication
- Probes
- Measurements
- Applications

## CLI utility

The `atlas` command is a command-line client for the Rust API.

### Configuration

The `atlas` utility uses a configuration file in the [TOML](https://github.com/naoina/toml) file format.

On UNIX, it is located in `$HOME/.config/ripe-atlas/config.toml` and in `%LOCALAPPDATA%\RIPE-ATLAS` on Windows.

There are only a few parameters for now, the most important one being your API Key for authenticate against the RIPE API endpoint.  You can now specify the default probe set (and override it from the CLI):

    # Default configuration file
    
    API_key = "<INSERT-API-KEY>"
    default_probe = <PROBE-ID>
    
    [probe_set]
    
    pool_size = <POOL-SIZE>
    type = "area"
    value = "WW"

Everything is a string except for `pool_size` and `default_probe` which are integers.

Be aware that if you ask for an IPv6 object (like a domain or machine name), the API will refuse your request if the IPv6 version of that object does not exist.

### Important note

Not all parameters specified for the different commands are implemented, as you can see in the [API Reference](https://atlas.ripe.net/docs/api/v2/reference/), there are *a lot* of different parameters like all the `id__{gt,gte,lt,lte,in}` stuff.

## TODO

It is not currently completely usable, only a few parts have been implemented (notable part of the `Probes` API) to validate our design (see [./APIDESIGN.md] for my musings about issues).

- Complete the various implementations for the "core" features like `Measurements` and `Probes`.
- add many many more tests
- refactor to get as much idiomatic Rust as possible

## Contributing

Please see CONTRIBUTING.md for some simple rules.


