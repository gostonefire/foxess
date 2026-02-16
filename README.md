# Foxess API client library
The foxess crate implements a subset of available [FoxESS Cloud APIs].

Its purpose is mainly focused on APIs that help in executing automatic scheduling of battery charging and battery discharging (self-use) given external data such as tariffs from e.g.:
* [Nordpool] in the Nordic European region or some other supplier of daily tariffs.
* Weather temperature forecast data to estimate household power consumption
* Weather cloud forecast data and sun incidence calculations to estimate PV power production
* Etc. depending on level of ambition/precision in estimates

The APIs are tested for a Fox H3 model SK-HWR-12, and although the FoxESS Cloud APIs are general,
settings and variables are not guaranteed to be fully supported by all inverters.

## License
This library comes with a standard [MIT license]

## Features
* `async` (default) - Enables async requests using [reqwest](https://crates.io/crates/reqwest)
* `blocking` - Enables blocking requests using [reqwest](https://crates.io/crates/reqwest)

`async` and `blocking` features are mutually exclusive, and since `async` is default, one must declare
default-features = false when enabling `blocking`

## Blocking mode (alternative)
This crate defaults to the `async` feature, and the documentation on docs.rs is generated for the async API.

If you prefer a blocking API, disable default features and enable `blocking` instead:
```toml
[dependencies]
foxess = { version = "0.x.y", default-features = false, features = ["blocking"] }
```

The blocking API uses the same `Fox` type name, but methods are synchronous (no `.await`).

## Usage Overview
Note down your inverter serial number, can be found from within the FoxCloud2.0 app or the [FoxESS Cloud V2 site] web site.

Get an API key, it can be retrieved from the [FoxESS Cloud V1 site] under User Profile/API Management.

Decide whether to use the blocking or non-blocking feature in cargo.toml dependencies
```
[dependencies]
# Non-blocking (async)
foxess = "0.x.y"

# Non-blocking (async) if you want clarity
foxess = { version = "0.x.y", features = ["async"] }

# Blocking
foxess = { version = "0.x.y", default-features = false, features = ["blocking"] }
```

[MIT license]: https://github.com/gostonefire/foxess/blob/main/LICENSE
[FoxESS Cloud APIs]: https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html
[Nordpool]: https://data.nordpoolgroup.com/auction/day-ahead/prices
[FoxESS Cloud V2 site]: https://www.foxesscloud.com/v2
[FoxESS Cloud V1 site]: https://www.foxesscloud.com/user/center