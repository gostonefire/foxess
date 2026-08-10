# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [2.0.0]

Reworked error handling so that failures reported by FoxESS Cloud carry structured
data instead of a pre-formatted string.

### Changed
* **Breaking:** `FoxError::FoxCloud` is now a struct variant carrying the error
  number and message from FoxESS Cloud, rather than a single formatted `String`:

  ```rust
  FoxError::FoxCloud { errno: u32, msg: String }
  ```

  Previously this variant was used for two unrelated failures: an application-level
  rejection from FoxESS Cloud (`errno != 0` under an HTTP 200), and a non-success
  HTTP status. Only the former is reported as `FoxCloud` now.

* **Breaking:** `FoxError` is marked `#[non_exhaustive]`. Any `match` over it must
  include a wildcard arm. This allows future variants to be added without another
  major release.

* A non-2xx response that carries a FoxESS Cloud error payload is now reported as
  `FoxError::FoxCloud` with the payload's `errno` and `msg`, instead of discarding
  the body and reporting the bare status.

### Added
* `FoxError::HttpStatus { status: u16, body: String }` for transport-level failures,
  where the response carried no parseable FoxESS Cloud error payload. The body is
  truncated to 512 characters so an error page cannot flood a log line.
* `FoxError::errno() -> Option<u32>` to read the FoxESS Cloud error number without
  matching on the variant.
* `FoxError::http_status() -> Option<u16>` to read the HTTP status of a
  transport-level failure.
* `FoxError::is_transient() -> bool`, true for HTTP 429, HTTP 5xx, and connection
  or timeout failures. Application-level and local parsing errors are never
  transient, since retrying an identical request cannot change their outcome.

### Fixed
* A response omitting the `msg` field is no longer surfaced as a JSON parse error,
  which previously masked the underlying `errno`.

### Migration

Matching on the variant:

```rust
// Before
match err {
    FoxError::FoxCloud(msg) => eprintln!("{msg}"),
    _ => {}
}

// After
match err {
    FoxError::FoxCloud { errno, msg } => eprintln!("errno {errno}: {msg}"),
    FoxError::HttpStatus { status, .. } => eprintln!("http {status}"),
    _ => {}
}
```

Reacting to a specific error code no longer requires inspecting the message:

```rust
if err.errno() == Some(40256) {
    // refresh credentials and retry
}
```

Code that only formats the error with `{}` or `{:?}` needs no change, though the
`FoxCloud` display text changes from `FoxCloud: errno: N, msg: M` to
`FoxCloud: errno=N, msg=M`, and non-2xx statuses now format as
`HttpStatus: status=N, body=...`.

Note that FoxESS Cloud `errno` values are a different namespace from the inverter
fault codes returned by `get_error_code_information`; the two are not
interchangeable.

## [1.1.0]

* Adds support for retrieving error code information from FoxESS Cloud.

## [1.0.0]

* **Breaking:** changes the call signature of `set_scheduler_time_segments` to take
  a borrowed `TimeSegmentsDataRequest`.

## Earlier releases

This changelog starts at 1.0.0. For releases before that, see the
[commit history] and the [release tags].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[commit history]: https://github.com/gostonefire/foxess/commits/main
[release tags]: https://github.com/gostonefire/foxess/tags
[2.0.0]: https://github.com/gostonefire/foxess/releases/tag/v2.0.0
[1.1.0]: https://github.com/gostonefire/foxess/releases/tag/v1.1.0
[1.0.0]: https://github.com/gostonefire/foxess/releases/tag/v1.0.0
