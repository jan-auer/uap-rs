use std::fs::File;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use serde_derive::Deserialize;
use uaparser::{Parser, UserAgentParser};

fn bench_os(c: &mut Criterion) {
    #[derive(Deserialize, Debug)]
    struct OSTestCases {
        test_cases: Vec<OSTestCase>,
    }

    #[derive(Deserialize, Debug)]
    struct OSTestCase {
        user_agent_string: String,
        family: String,
        major: Option<String>,
        minor: Option<String>,
        patch: Option<String>,
        patch_minor: Option<String>,
    }

    let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    let file = File::open("./src/core/tests/test_os.yaml")
        .expect("test_device.yaml failed to load");

    let test_cases: OSTestCases =
        serde_yaml::from_reader(file).expect("Failed to deserialize device test cases");

    for case in &test_cases.test_cases {
        parser.parse_os(&case.user_agent_string);
        parser.parse_os_set(&case.user_agent_string);
    }

    let mut group = c.benchmark_group("UserAgentParser::parse_os");
    for case in test_cases.test_cases {
        let ua = case.user_agent_string.as_str();
        group.bench_with_input(BenchmarkId::new("iter", ua), ua, |b, ua| {
            b.iter(|| parser.parse_os(ua))
        });
        group.bench_with_input(BenchmarkId::new("set", ua), ua, |b, ua| {
            b.iter(|| parser.parse_os_set(ua))
        });
    }
}

fn bench_device(c: &mut Criterion) {
    #[derive(Deserialize, Debug)]
    struct DeviceTestCases {
        test_cases: Vec<DeviceTestCase>,
    }

    #[derive(Deserialize, Debug)]
    struct DeviceTestCase {
        user_agent_string: String,
        family: String,
        brand: Option<String>,
        model: Option<String>,
    }

    let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    let file = std::fs::File::open("./src/core/tests/test_device.yaml")
        .expect("test_device.yaml failed to load");

    let test_cases: DeviceTestCases =
        serde_yaml::from_reader(file).expect("Failed to deserialize device test cases");

    for case in &test_cases.test_cases {
        parser.parse_device(&case.user_agent_string);
        parser.parse_device_set(&case.user_agent_string);
    }

    let mut group = c.benchmark_group("UserAgentParser::parse_device");
    for case in test_cases.test_cases {
        let ua = case.user_agent_string.as_str();
        group.bench_with_input(BenchmarkId::new("iter", ua), ua, |b, ua| {
            b.iter(|| parser.parse_device(ua))
        });
        group.bench_with_input(BenchmarkId::new("set", ua), ua, |b, ua| {
            b.iter(|| parser.parse_device_set(ua))
        });
    }
}

fn bench_ua(c: &mut Criterion) {
    #[derive(Deserialize, Debug)]
    struct UserAgentTestCases {
        test_cases: Vec<UserAgentTestCase>,
    }

    #[derive(Deserialize, Debug)]
    struct UserAgentTestCase {
        user_agent_string: String,
        family: String,
        major: Option<String>,
        minor: Option<String>,
        patch: Option<String>,
    }

    let parser = UserAgentParser::from_yaml("./src/core/regexes.yaml")
        .expect("Parser creation failed");

    let file = std::fs::File::open("./src/core/tests/test_ua.yaml")
        .expect("test_device.yaml failed to load");

    let test_cases: UserAgentTestCases =
        serde_yaml::from_reader(file).expect("Failed to deserialize device test cases");

    for case in &test_cases.test_cases {
        parser.parse_user_agent(&case.user_agent_string);
        parser.parse_user_agent_set(&case.user_agent_string);
    }

    let mut group = c.benchmark_group("UserAgentParser::parse_user_agent");
    for case in test_cases.test_cases {
        let ua = case.user_agent_string.as_str();
        group.bench_with_input(BenchmarkId::new("iter", ua), ua, |b, ua| {
            b.iter(|| parser.parse_user_agent(ua))
        });
        group.bench_with_input(BenchmarkId::new("set", ua), ua, |b, ua| {
            b.iter(|| parser.parse_user_agent_set(ua))
        });
    }
}

criterion_group!(benches, bench_device, bench_os, bench_ua);
criterion_main!(benches);
