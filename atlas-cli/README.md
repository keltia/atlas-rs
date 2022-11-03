<!-- omit in TOC -->
# Atlas-cli

[![CircleCI](https://circleci.com/gh/keltia/atlas-rs/tree/main.svg?style=shield)](https://circleci.com/gh/keltia/atlas-rs/atlas-cli/tree/main)
[![dependency status](https://deps.rs/repo/github/keltia/atlas-rs/status.svg)](https://deps.rs/repo/github/keltia/atlas-rs)
[![](https://img.shields.io/crates/v/atlas-cli.svg)](https://crates.io/crates/atlas-cli)
[![Docs](https://docs.rs/atlas-cli/badge.svg)](https://docs.rs/atlas-cli)

[![SemVer](http://img.shields.io/SemVer/2.0.0.png)](https://semver.org/spec/v2.0.0.html)
[![License](https://img.shields.io/badge/license-MIT-red.svg?style=flat)](https://raw.githubusercontent.com/keltia/atlas-rs/atlas-cli/main/LICENSE)

This crate features a simple CLI-based tool called `atlas-cli` which serve both as a collection of use-cases for the
library and an easy way to use it.

The `atlas-api` crate is the library part to access the RIPE Atlas [REST API]. It is a rewrite in Rust of my Go library
called [ripe-atlas](https://github.com/keltia/ripe-atlas).

**Work in progress, still incomplete**

- [Features](#features)
- [Installation](#installation)
- [Documentation](#documentation)
- [CLI Utility](#cli-utility)
  - [Configuration](#configuration)
- [TODO](#todo)
- [Official documentation](#official-documentation)
- [Contributing](#contributing)

## Features

I am trying to implement the full REST API in Rust. The API itself is not particularly complex but the settings and
parameters are. The CLI part tries to implement as any API calls as possible in an easy-to-use utility.

See the API [README.md](https://github.com/keltia/atlas-rs/atlas-api/README.md) for the details on what is currently
available in the API and usable in `atlas-cli`.

The following topic are available:

- anchors

  Anchors are special probes

- anchors_measurements

  Manage measurements tied to anchors

- credits

  Find how many credits you have and what to do with them

- keys

  API Key management

- probes

  you can query one probe or ask for a list of probes with a few criterias

- measurements

  you can create and list measurements.

- results

  every measurement has a URI in the result json that points to the actual results. This fetch and display them.

In addition to these major commands, there are a few shortcut commands (see below):

- dns
- http
- keys
- ntp
- ping
- sslcert/tls
- traceroute

And finally two for convenience:

- ip
- version
 
## Installation

This will be available as a crate on [crates.io](https://crates.io/atlas-cli) when it can be released, there are still
many incomplete parts.

## Documentation

All the documentation on the API itself is available through Rust builtin doc system and will be visible at
[atlas-api page on docs.rs](https://docs.rs/atlas-api).

## CLI utility

The `atlas-cli` command is a command-line client for the Rust API.

It can be used to exercise various aspects of the API (I use it to test my implementation).

```text
Rust CLI for RIPE Atlas.

Usage: atlas-cli [OPTIONS] <COMMAND>

Commands:
  credits      Dislays informations about credits [aliases: c]
  key          Key management [aliases: keys, k]
  measurement  Create, starts, displays measurements [aliases: m]
  probe        Get informations about probes [aliases: probes, p]
  dns          DNS-related measurements
  http         HTTP-related measurements
  ntp          NTP-related measurements
  ping         ICMP-related measurements
  tls-cert     Certificate info management [aliases: cert]
  traceroute   Traceroute from probes [aliases: tracert]
  version      Display the full version stuff
  ip           Displays the default probe IPs
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  configuration file
  -D, --debug            debug mode
  -v, --verbose          Verbose mode
  -h, --help             Print help information
  -V, --version          Print version information
```

### Configuration

The `atlas-cli` utility uses a configuration file in the [TOML] file format.

On Unix, it is located in `$HOME/.config/ripe-atlas/config.toml` and in `%LOCALAPPDATA%\RIPE-ATLAS` on Windows.

There are only a few parameters for now, the most important one being your API Key for authenticate against the RIPE API
endpoint. You can now specify the default probe set (and override it from the CLI):

```toml
    # Default configuration file
    
    API_key = "<INSERT-API-KEY>"
    default_probe = <PROBE-ID>
    
    [probe_set]
    
    pool_size = 10
    type = "area"
    value = "WW"
```

Everything is a string except for `pool_size` and `default_probe` which are integers.

Be aware that if you ask for an IPv6 object (like a domain or machine name), the API will refuse your request if the
IPv6 version of that object does not exist.

Most of the API calls require use of an API key and in some cases, not using one will mask a few fields in results.

**NOTE**: If you don't have a configuration file, some defaults will be picked but as above, many calls **require** an
API key.

### Important note

Not all parameters specified for the different commands are implemented, as you can see in the [REST API Reference],
there are *a lot* of different parameters like all the `id__{gt,gte,lt,lte,in}` stuff. See TODO :)

## Proxy features

**NOTE**: System proxies are enabled by default.

System proxies look in environment variables to set HTTP or HTTPS proxies.

`HTTP_PROXY` or `http_proxy` provide http proxies for http connections while
`HTTPS_PROXY` or `https_proxy` provide HTTPS proxies for HTTPS connections.
`ALL_PROXY` or `all_proxy`is a "catch-all" setting for all protocols.
`NO_PROXY` or `no_proxy` can prevent using any of the proxies.

## TODO

It is not currently completely usable, only a few parts have been implemented (notable part of the `Probes` API) to
validate our design (see [APIDESIGN.md](../atlas-api/APIDESIGN.md) for my musings about issues).

- Implement a good way to pass arguments to various calls besides the `opts` HashMap.
- ~~Implement generic pagination~~
- Complete the various implementations for the "core" features like `Measurements` and `Probes`.
- Add many many more tests
- Refactor to get as much idiomatic Rust as possible
- Cleanup the information displayed by `atlas`, right now a mix of json & deserialized stuff.

## Official Documentation

Metadata API (probes, keys, credits) and Measurement Results API.

- [Main RIPE Atlas site]
- [REST API Documentation]
- [REST API Reference]

## Contributing

I use the [Git Flow] system to manage development & release branches, hotfix, etc.  Most of the development is done on the `develop` branch, merged in `main` for each release. At the moment, there are no feature branches, all dev is on `develop`, merged from time to time with a tag on `main`.

If you want to contribute, please fork the project, fetch/sync the `develop` branch and submit pull requests based on
it. Or open a ticket with a patch from the `develop` branch.

Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for some simple rules.

[Main RIPE Atlas site]: https://atlas.ripe.net/
[REST API Documentation]: https://atlas.ripe.net/docs/api/v2/manual/
[REST API Reference]: https://atlas.ripe.net/docs/api/v2/reference/
[Git Flow]: https://jeffkreeftmeijer.com/git-flow/
[Rust]: https://rust-lang.org/
[TOML]: https://github.com/naoina/toml
[REST API]: https://en.wikipedia.org/wiki/REST_API
