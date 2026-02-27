//! Unit tests for async client functionality using httpmock.
//!

#[cfg(feature = "async")]
use httpmock::prelude::*;
#[cfg(feature = "async")]
use md5::{Digest, Md5};
#[cfg(feature = "async")]
use serde_json::json;

/// Generates the expected MD5 signature for a given API request.
///
/// The signature is calculated based on the request path, API key, and timestamp.
/// It mimics the signature generation logic used in the library.
///
/// # Arguments
/// * `path` - The API endpoint path.
/// * `api_key` - The user's API key.
/// * `timestamp_millis` - The request timestamp in milliseconds.
///
/// # Returns
/// * `String` - The hex-encoded MD5 signature.
#[cfg(feature = "async")]
fn expected_signature(path: &str, api_key: &str, timestamp_millis: i64) -> String {
    // Must match the library logic exactly (note the literal "\r\n" sequences).
    let signature = format!("{}\\r\\n{}\\r\\n{}", path, api_key, timestamp_millis);

    let mut hasher = Md5::new();
    hasher.update(signature.as_bytes());
    hasher.finalize().iter().map(|x| format!("{:02x}", x)).collect()
}

/// Verifies that `get_settings` correctly fetches and parses multiple settings from the API.
///
/// This test mocks a successful API response for a settings query and asserts that
/// the returned values are correctly converted to their expected types.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_settings() {
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

/// Verifies that `get_setting_typed` correctly fetches and parses a single setting into a specific type.
///
/// This test ensures that the generic `get_setting_typed` method works as expected for
/// settings that have a numeric representation in the API response.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_setting_typed() {
    use crate::Fox;
    use crate::fox_settings::MinSocOnGrid;

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000; // fixed timestamp for deterministic tests
    const PATH: &str = "/op/v0/device/setting/get";

    fn fixed_now() -> i64 { TS }


    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
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

/// Verifies that `set_setting_typed` correctly sends a request to update a setting.
///
/// This test mocks a successful setting update and ensures that the request body
/// contains the correct serial number, key, and value.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_set_setting_typed() {
    use crate::Fox;
    use crate::fox_settings::MinSocOnGrid;

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000; // fixed timestamp for deterministic tests
    const PATH: &str = "/op/v0/device/setting/set";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
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

/// Verifies that real-time variables can be parsed even when the API returns values in scientific notation.
///
/// This ensures robustness against different numeric formats that might be returned by the FoxESS API.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_variables_parses_scientific_notation() {
    use crate::{Fox, FoxVariables};

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v1/device/real/query";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
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
    let res = fox.get_variables(vec![FoxVariables::SoC, FoxVariables::PvPower]).await.unwrap();

    assert_eq!(res.get_u8_percent(FoxVariables::SoC), Some(99));
    assert_eq!(res.get(FoxVariables::PvPower), Some(123.0));
}

/// Verifies that `get_variable_typed` correctly handles scientific notation for a single variable.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_variable_typed_parses_scientific_notation() {
    use crate::Fox;
    use crate::fox_variables::SoC;

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v1/device/real/query";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
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

/// Verifies that `get_variable_typed` returns an error when the value is outside the valid range.
///
/// For example, an SoC value of 110% should trigger a validation error during parsing.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_variable_typed_outside_valid_range() {
    use crate::Fox;
    use crate::fox_variables::SoC;

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v1/device/real/query";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
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

/// Verifies that `get_variables_history` correctly retrieves and parses historical data points.
///
/// This test mocks a historical data response and checks that the timestamps and values
/// are correctly processed into the resulting map.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_variables_history() {
    use chrono::{TimeZone, Utc};
    use crate::{Fox, FoxVariables};

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v0/device/history/query";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
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

    let res = fox.get_variables_history(start, end, vec![FoxVariables::PvPower]).await.unwrap();

    let series = res.get(FoxVariables::PvPower).unwrap();
    assert_eq!(series.len(), 2);
    assert_eq!(series[0].data, 42.0);
    assert_eq!(series[0].date_time, Utc.with_ymd_and_hms(2025, 12, 2, 23, 8, 51).unwrap());
    assert_eq!(series[1].data, 43.0);
    assert_eq!(series[1].date_time, Utc.with_ymd_and_hms(2025, 12, 2, 23, 9, 51).unwrap());
}

/// Verifies that `set_battery_charging_time_schedule` correctly configures a charging schedule.
///
/// This test checks that the complex JSON structure required for setting schedules is
/// correctly generated and sent to the API.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_set_battery_charging_time_schedule() {
    use chrono::{TimeZone, Utc, Local};
    use crate::Fox;

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v0/device/battery/forceChargeTime/set";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
            .json_body(json!(
                {
                    "sn": SN,
                    "enable1": true,
                    "enable2": false,
                    "startTime1": {
                        "hour": 3,
                        "minute": 15
                    },
                    "endTime1": {
                        "hour": 5,
                        "minute": 29
                    },
                    "startTime2": {
                        "hour": 0,
                        "minute": 0
                    },
                    "endTime2": {
                        "hour": 0,
                        "minute": 0
                    }
                }));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": null
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let start = Local.with_ymd_and_hms(2026, 2, 9, 3, 15, 0).unwrap().with_timezone(&Utc);
    let end = Local.with_ymd_and_hms(2026, 2, 9, 5, 30, 0).unwrap().with_timezone(&Utc);

    let _ = fox.set_battery_charging_time_schedule(true, start, end).await.unwrap();
}

/// Verifies that setting a charging schedule with an end time before the start time results in an error.
///
/// Validation should happen locally before the request is even sent to the API.
#[cfg(feature = "async")]
#[tokio::test]
async fn async_set_battery_charging_time_schedule_end_before_start() {
    use chrono::{TimeZone, Utc, Local};
    use crate::Fox;

    const TS: i64 = 1_700_000_000_000;

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST);
        then.status(500);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let start = Local.with_ymd_and_hms(2026, 2, 9, 5, 30, 0).unwrap().with_timezone(&Utc);
    let end = Local.with_ymd_and_hms(2026, 2, 9, 3, 15, 0).unwrap().with_timezone(&Utc);

    let err = fox.set_battery_charging_time_schedule(true, start, end).await
        .err()
        .expect("set_battery_charging_time_schedule should fail");

    let msg = format!("{err}");
    assert!(msg.contains("charge schedule 1 start time is after end time"));
}

/// Verifies that available variables are correctly parsed from the API response, including enumeration values.
///
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_available_variables() {
    use crate::Fox;

    const TS: i64 = 1_700_000_000_000;

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(GET);
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": [
                    {
                        "pvPower": {
                            "unit": "kW",
                            "Grid-tied inverter": true,
                            "name": {
                                "de": "PV Leistung",
                                "pt": "Potência PV",
                                "en": "PVPower",
                                "zh_CN": "PV功率",
                                "pl": "Moc PV",
                                "fr": "PV Puissance"
                            },
                            "Energy-storage inverter": true
                        }
                    },
                    {
                        "SoC": {
                            "unit": "%",
                            "Grid-tied inverter": false,
                            "name": {
                                "de": "SoC",
                                "pt": "SoC",
                                "en": "SoC",
                                "zh_CN": "SoC",
                                "pl": "SoC",
                                "fr": "SoC"
                            },
                            "Energy-storage inverter": true
                        }
                    },
                    {
                        "runningState": {
                            "Grid-tied inverter": true,
                            "name": {
                                "de": "Running State",
                                "pt": "Running State",
                                "en": "Running State",
                                "zh_CN": "运行状态",
                                "pl": "Running State",
                                "fr": "Running State"
                            },
                            "Energy-storage inverter": true,
                            "enum": {
                                "165": "fault",
                                "166": "permanent-fault",
                                "167": "standby",
                                "168": "upgrading",
                                "169": "fct",
                                "170": "illegal",
                                "160": "self-test",
                                "161": "waiting",
                                "162": "checking",
                                "163": "on-grid",
                                "164": "off-grid"
                            }
                        }
                    }
                ]
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_available_variables().await.unwrap();

    assert_eq!(res.variables.len(), 3);
    assert!(res.variables.iter().filter_map(|v| v.enumeration.as_ref()).any(|e| e.contains_key("163")));
}

/// Verifies that scheduler time segments are correctly parsed from the API response, including enumeration values.
///
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_scheduler_time_segments() {
    use crate::Fox;
    use crate::FoxWorkModes;

    const TS: i64 = 1_700_000_000_000;

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST);
        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": {
                    "enable": 1,
                    "maxGroupCount": 24,
                    "groups": [
                        {
                            "endHour": 8,
                            "workMode": "SelfUse",
                            "startHour": 0,
                            "extraParam": {
                                "fdPwr": 100.0,
                                "minSocOnGrid": 10.0,
                                "fdSoc": 10.0,
                                "maxSoc": 100.0
                            },
                            "startMinute": 0,
                            "endMinute": 29
                        },
                        {
                            "endHour": 8,
                            "workMode": "ForceCharge",
                            "startHour": 8,
                            "extraParam": {
                                "fdPwr": 12000.0,
                                "minSocOnGrid": 10.0,
                                "fdSoc": 20.0,
                                "maxSoc": 100.0
                            },
                            "startMinute": 30,
                            "endMinute": 59
                        },
                        {
                            "endHour": 9,
                            "workMode": "SelfUse",
                            "startHour": 9,
                            "extraParam": {
                                "fdPwr": 100.0,
                                "minSocOnGrid": 10.0,
                                "fdSoc": 10.0,
                                "maxSoc": 100.0
                            },
                            "startMinute": 0,
                            "endMinute": 29
                        },
                        {
                            "endHour": 9,
                            "workMode": "ForceCharge",
                            "startHour": 9,
                            "extraParam": {
                                "fdPwr": 12000.0,
                                "minSocOnGrid": 10.0,
                                "fdSoc": 30.0,
                                "maxSoc": 100.0
                            },
                            "startMinute": 30,
                            "endMinute": 59
                        },
                        {
                            "endHour": 10,
                            "workMode": "SelfUse",
                            "startHour": 10,
                            "extraParam": {
                                "fdPwr": 0.0,
                                "minSocOnGrid": 10.0,
                                "fdSoc": 10.0,
                                "maxSoc": 100.0
                            },
                            "startMinute": 0,
                            "endMinute": 59
                        },
                        {
                            "endHour": 11,
                            "workMode": "Backup",
                            "startHour": 11,
                            "extraParam": {
                                "fdPwr": 0.0,
                                "minSocOnGrid": 30.0,
                                "fdSoc": 10.0,
                                "maxSoc": 100.0
                            },
                            "startMinute": 0,
                            "endMinute": 59
                        },
                        {
                            "endHour": 23,
                            "workMode": "SelfUse",
                            "startHour": 12,
                            "extraParam": {
                                "fdPwr": 0.0,
                                "minSocOnGrid": 10.0,
                                "fdSoc": 10.0,
                                "maxSoc": 100.0
                            },
                            "startMinute": 0,
                            "endMinute": 59
                        }
                    ],
                    "properties": {
                        "startminute": {
                            "unit": "",
                            "precision": 1.0,
                            "range": {
                                "min": 0.0,
                                "max": 59.0
                            }
                        },
                        "fdpwr": {
                            "unit": "W",
                            "precision": 1.0,
                            "range": {
                                "min": 0.0,
                                "max": 12000.0
                            }
                        },
                        "endhour": {
                            "unit": "",
                            "precision": 1.0,
                            "range": {
                                "min": 0.0,
                                "max": 23.0
                            }
                        },
                        "endminute": {
                            "unit": "",
                            "precision": 1.0,
                            "range": {
                                "min": 0.0,
                                "max": 59.0
                            }
                        },
                        "fdsoc": {
                            "unit": "%",
                            "precision": 1.0,
                            "range": {
                                "min": 10.0,
                                "max": 100.0
                            }
                        },
                        "starthour": {
                            "unit": "",
                            "precision": 1.0,
                            "range": {
                                "min": 0.0,
                                "max": 23.0
                            }
                        },
                        "workmode": {
                            "enumList": [
                                "ForceDischarge",
                                "PeakShaving",
                                "Feedin",
                                "Backup",
                                "SelfUse",
                                "ForceCharge"
                            ],
                            "unit": "",
                            "precision": 1.0
                        },
                        "minsocongrid": {
                            "unit": "%",
                            "precision": 1.0,
                            "range": {
                                "min": 10.0,
                                "max": 100.0
                            }
                        },
                        "maxsoc": {
                            "unit": "%",
                            "precision": 1.0,
                            "range": {
                                "min": 10.0,
                                "max": 100.0
                            }
                        }
                    }
                }
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_scheduler_time_segments().await.unwrap();

    assert_eq!(res.groups.len(), 7);
    assert_eq!(res.properties.work_mode.enum_list.len(), 6);
    assert_eq!(res.properties.work_mode.enum_list.contains(&FoxWorkModes::Unknown), false);
}

/// Verifies that scheduler time segments are correctly formatted to the API response, including enumeration values.
///
#[cfg(feature = "async")]
#[tokio::test]
async fn async_set_scheduler_time_segments() {
    use crate::Fox;
    use crate::FoxWorkModes;
    use crate::{ExtraParam, Group, TimeSegmentsDataRequest};

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v3/device/scheduler/enable";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
            .json_body(json!(
                {
                    "deviceSN": SN,
                    "isDefault": false,
                    "groups": [
                        {
                            "startHour": 0,
                            "startMinute": 0,
                            "endHour": 2,
                            "endMinute": 59,
                            "workMode": "ForceCharge",
                            "extraParam": {
                                "minSocOnGrid": 10.0,
                                "fdSoc": 80.0,
                                "fdPwr": 12000.0,
                                "maxSoc": 100.0
                            }
                        },
                        {
                            "startHour": 3,
                            "startMinute": 0,
                            "endHour": 7,
                            "endMinute": 59,
                            "workMode": "Backup"
                        },
                        {
                            "startHour": 8,
                            "startMinute": 0,
                            "endHour": 23,
                            "endMinute": 59,
                            "workMode": "SelfUse"
                        }
                    ]
                }));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": null
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let ts = TimeSegmentsDataRequest {
        is_default: None,
        groups: vec![
            Group {
                start_hour: 0,
                start_minute: 0,
                end_hour: 2,
                end_minute: 59,
                work_mode: FoxWorkModes::ForceCharge,
                extra_param: Some(ExtraParam {
                    fd_pwr: Some(12000.0),
                    min_soc_on_grid: Some(10.0),
                    fd_soc: Some(80.0),
                    max_soc: Some(100.0),
                    import_limit: None,
                    export_limit: None,
                    pv_limit: None,
                    reactive_power: None,
                }),
            },
            Group {
                start_hour: 3,
                start_minute: 0,
                end_hour: 7,
                end_minute: 59,
                work_mode: FoxWorkModes::Backup,
                extra_param: None,
            },
            Group {
                start_hour: 8,
                start_minute: 0,
                end_hour: 23,
                end_minute: 59,
                work_mode: FoxWorkModes::SelfUse,
                extra_param: None,
            },
        ],
    };

    let _ = fox.set_scheduler_time_segments(ts).await.unwrap();
}

/// Verifies that get main switch status is correctly parsed from the API response.
///
#[cfg(feature = "async")]
#[tokio::test]
async fn async_get_main_switch_status() {
    use crate::Fox;

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v1/device/scheduler/get/flag";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
            .json_body(json!(
                {
                    "deviceSN": SN,
                }));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": {
                    "support": true,
                    "enable": false
                    }
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();
    let res = fox.get_main_switch_status().await.unwrap();

    assert_eq!(res.support, true);
    assert_eq!(res.enable, false);
}

/// Verifies that set main switch status is correctly formatted to the API response.
///
#[cfg(feature = "async")]
#[tokio::test]
async fn async_set_main_switch_status() {
    use crate::Fox;

    const SN: &str = "TEST_SN";
    const TS: i64 = 1_700_000_000_000;
    const PATH: &str = "/op/v1/device/scheduler/set/flag";

    fn fixed_now() -> i64 { TS }

    let server = MockServer::start();

    let _m = server.mock(|when, then| {
        when.method(POST)
            .path(PATH)
            .json_body(json!(
                {
                    "deviceSN": SN,
                    "enable": 1
                }));

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{
                "errno": 0,
                "msg": "Operation successful",
                "result": null
            }"#);
    });

    let fox = Fox::new_with_base_url_and_clock("TEST_API_KEY", "TEST_SN", 5, &server.base_url(), fixed_now).unwrap();

    let _ = fox.set_main_switch_status(true).await.unwrap();
}

/// Verifies that non-zero `errno` values in the API response are correctly mapped to `FoxError`.
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

/// Verifies that HTTP status errors (e.g., 500 Internal Server Error) are correctly mapped to `FoxError`.
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