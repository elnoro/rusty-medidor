use std::env;

use self::metrics::{*};
use self::mailer::{*};
use self::report::{*};

pub mod mailer;
pub mod metrics;
pub mod report;

pub fn send_report(mut mailer: Mailer, metric_values: &[MetricValue]) {
    let to = get_env("REPORTS_TO", "");
    let from = get_env("REPORTS_FROM", "");

    let report = generate_report_from(metric_values)
        .expect("Failed to generate report");

    mailer.send_email(&from, &to, &report.title, &report.content);
    ()
}


pub fn get_env(env_name: &str, default: &str) -> String {
    String::from(env::var(env_name).unwrap_or(default.to_string()))
}