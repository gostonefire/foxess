#[cfg(feature = "async")]
use httpmock::prelude::*;

#[cfg(feature = "async")]
use md5::{Digest, Md5};

#[cfg(feature = "async")]
fn expected_signature(path: &str, api_key: &str, timestamp_millis: i64) -> String {
    // Must match the library logic exactly (note the literal "\r\n" sequences).
    let signature = format!("{}\\r\\n{}\\r\\n{}", path, api_key, timestamp_millis);

    let mut hasher = Md5::new();
    hasher.update(signature.as_bytes());
    hasher.finalize().iter().map(|x| format!("{:02x}", x)).collect()
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_settings_uses_mock_server() {
    use crate::{Fox, FoxSettings};

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
                "msg": "Operation successful",
                "result": { "value": "12.34" }
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_settings(vec![FoxSettings::MaxSetChargeCurrent]).await.unwrap();

    assert_eq!(res.get_f64(FoxSettings::MaxSetChargeCurrent).unwrap(), Some(12.34));
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_setting_typed_uses_mock_server() {
    use crate::Fox;
    use crate::fox_settings::MinSocOnGrid;

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
            .json_body_includes(r#"{"key":"MinSocOnGrid"}"#);

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": { "value": "55" }
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_setting_typed::<MinSocOnGrid>().await.unwrap();

    assert_eq!(res, 55);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_set_setting_typed_uses_mock_server() {
    use crate::Fox;
    use crate::fox_settings::MinSocOnGrid;

    const API_KEY: &str = "TEST_API_KEY";
    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000; // fixed timestamp for deterministic tests
    const PATH: &str = "/op/v0/device/setting/set";

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
            .json_body_includes(&format!(r#"{{"sn":"{}","key":"MinSocOnGrid","value":"55"}}"#, SN));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": null
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let _ = fox.set_setting_typed::<MinSocOnGrid>(55).await.unwrap();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_realtime_parses_scientific_notation() {
    use crate::{Fox, FoxVariables};

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
                "msg": "Operation successful",
                "result": [{
                    "datas": [
                        { "variable": "SoC", "value": "9.90E1" },
                        { "variable": "pvPower", "value": 123.0 }
                    ]
                }]
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_device_real_time_data(vec![FoxVariables::SoC, FoxVariables::PvPower]).await.unwrap();

    assert_eq!(res.get_u8_percent(FoxVariables::SoC), Some(99));
    assert_eq!(res.get(FoxVariables::PvPower), Some(123.0));
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_variable_typed_parses_scientific_notation() {
    use crate::Fox;
    use crate::fox_variables::SoC;

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
                "msg": "Operation successful",
                "result": [{
                    "datas": [
                        { "variable": "SoC", "value": "9.90E1" }
                    ]
                }]
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_variable_typed::<SoC>().await.unwrap();

    assert_eq!(res, 99);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_variable_typed_outside_valid_range() {
    use crate::Fox;
    use crate::fox_variables::SoC;

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
                "msg": "Operation successful",
                "result": [{
                    "datas": [
                        { "variable": "SoC", "value": "110" }
                    ]
                }]
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let err = fox.get_variable_typed::<SoC>().await
        .err()
        .expect("get_variable_typed should fail");

    let msg = format!("{err}");
    assert!(msg.contains("value out of range for u8 percentage after rounding"));
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_history_transforms_first_datapoint_only() {
    use chrono::{TimeZone, Utc};
    use crate::{Fox, FoxVariables};

    const API_KEY: &str = "TEST_API_KEY";
    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v0/device/history/query";

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
            .json_body_includes(&format!(r#"{{"sn": "{}","variables": ["pvPower"],"begin": 0,"end": 1}}"#, SN));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
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

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let start = Utc.timestamp_millis_opt(0).unwrap();
    let end = Utc.timestamp_millis_opt(1).unwrap();

    let res = fox.get_device_history_data(start, end, vec![FoxVariables::PvPower]).await.unwrap();

    let series = res.get(FoxVariables::PvPower).unwrap();
    assert_eq!(series.len(), 2);
    assert_eq!(series[0].data, 42.0);
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_errno_nonzero_maps_to_error() {
    use crate::{Fox, FoxSettings};

    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v0/device/setting/get";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST).path(PATH);

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 40256,
                "msg": "The request header parameters are missing",
                "result": null
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let err = fox.get_settings(vec![FoxSettings::MaxSetChargeCurrent]).await
        .err()
        .expect("get_settings should fail");

    let msg = format!("{err}");
    assert!(msg.contains("40256"));
    assert!(msg.contains("The request header parameters are missing"));
}

#[cfg(feature = "async")]
#[tokio::test]
async fn async_http_status_error_maps_to_error() {
    use crate::{Fox, FoxSettings};

    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v0/device/setting/get";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST).path(PATH);
        then.status(500).body("oops");
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let err = fox.get_settings(vec![FoxSettings::MaxSetChargeCurrent]).await
        .err()
        .expect("get_settings should fail");

    let msg = format!("{err}");
    assert!(msg.contains("500"));
}