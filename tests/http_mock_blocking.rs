#[cfg(feature = "blocking")]
use httpmock::prelude::*;

#[cfg(feature = "blocking")]
use md5::{Digest, Md5};

#[cfg(feature = "blocking")]
fn expected_signature(path: &str, api_key: &str, timestamp_millis: i64) -> String {
    // Must match the library logic exactly (note the literal "\r\n" sequences).
    let signature = format!("{}\\r\\n{}\\r\\n{}", path, api_key, timestamp_millis);

    let mut hasher = Md5::new();
    hasher.update(signature.as_bytes());
    hasher.finalize().iter().map(|x| format!("{:02x}", x)).collect()
}

#[cfg(feature = "blocking")]
#[test]
fn async_get_settings_uses_mock_server() {
    use foxess::{Fox, FoxSettings};

    const API_KEY: &str = "TEST_API_KEY";
    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000; // fixed timestamp for deterministic tests
    const PATH: &str = "/op/v0/device/setting/get";

    fn fixed_now() -> i64 { TS }


    let server = MockServer::start();
    let sig = expected_signature(PATH, API_KEY, TS);

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
            .header("token", API_KEY)
            .header("timestamp", &TS.to_string())
            .header("signature", &sig)
            .header("lang", "en")
            .header("content-type", "application/json")
            .json_body_includes(&format!(r#"{{"sn":"{}"}}"#, SN))
            .json_body_includes(r#"{"key":"MaxSetChargeCurrent"}"#);

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "success",
                "result": { "value": "12.34" }
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_settings(vec![FoxSettings::MaxSetChargeCurrent]).unwrap();

    assert_eq!(res.get_f64(FoxSettings::MaxSetChargeCurrent).unwrap(), Some(12.34));
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_get_realtime_parses_scientific_notation() {
    use foxess::{Fox, FoxVariables};

    const API_KEY: &str = "TEST_API_KEY";
    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v1/device/real/query";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();
    let sig = expected_signature(PATH, API_KEY, TS);

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
            .header("token", API_KEY)
            .header("timestamp", &TS.to_string())
            .header("signature", &sig)
            .header("lang", "en")
            .header("content-type", "application/json")
            .json_body_includes(&format!(r#"{{"sns":["{}"]}}"#, SN));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "success",
                "result": [{
                    "datas": [
                        { "variable": "SoC", "value": "9.90E1" },
                        { "variable": "pvPower", "value": 123.0 }
                    ]
                }]
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_device_real_time_data(vec![FoxVariables::SoC, FoxVariables::PvPower]).unwrap();

    assert_eq!(res.get_u8_percent(FoxVariables::SoC), Some(99));
    assert_eq!(res.get(FoxVariables::PvPower), Some(123.0));
}

#[cfg(feature = "blocking")]
#[test]
fn async_get_history_transforms_first_datapoint_only() {
    use chrono::{TimeZone, Utc};
    use foxess::{Fox, FoxVariables};

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path("/op/v0/device/history/query")
            .header_exists("token")
            .header_exists("timestamp")
            .header_exists("signature");

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "success",
                "result": [{
                    "datas": [{
                        "variable": "pvPower",
                        "data": [
                            { "time": "2025-12-03 00:08:51 CET+0100", "value": 42.0 },
                            { "time": "2025-12-03 00:09:51 CET+0100", "value": 43.0 }
                        ]
                    }]
                }]
            }"#);
    });

    let fox = Fox::new_with_base_url("TEST_API_KEY", "TEST_SN", 5, &server.base_url()).unwrap();
    let start = Utc.timestamp_millis_opt(0).unwrap();
    let end = Utc.timestamp_millis_opt(1).unwrap();

    let res = fox.get_device_history_data(start, end, vec![FoxVariables::PvPower]).unwrap();

    let series = res.get(FoxVariables::PvPower).unwrap();
    assert_eq!(series.len(), 2);
    assert_eq!(series[0].data, 42.0);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_errno_nonzero_maps_to_error() {
    use foxess::{Fox, FoxSettings};

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST).path("/op/v0/device/setting/get");

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 401,
                "msg": "bad token",
                "result": { "value": "0" }
            }"#);
    });

    let fox = Fox::new_with_base_url("TEST_API_KEY", "TEST_SN", 5, &server.base_url()).unwrap();
    let err = fox.get_settings(vec![FoxSettings::MaxSetChargeCurrent])
        .err()
        .expect("get_settings should fail");

    // Keep the assertion broad (string format may change)
    let msg = format!("{err}");
    assert!(msg.contains("errno"));
    assert!(msg.contains("bad token"));
}

#[cfg(feature = "blocking")]
#[test]
fn async_http_status_error_maps_to_error() {
    use foxess::{Fox, FoxSettings};

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST).path("/op/v0/device/setting/get");
        then.status(500).body("oops");
    });

    let fox = Fox::new_with_base_url("TEST_API_KEY", "TEST_SN", 5, &server.base_url()).unwrap();
    let err = fox.get_settings(vec![FoxSettings::MaxSetChargeCurrent])
        .err()
        .expect("get_settings should fail");

    let msg = format!("{err}");
    assert!(msg.contains("500"));
}