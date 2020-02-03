use std::error::Error;
use serde_json::value::Map;
use handlebars::{to_json, Handlebars};
use chrono::Utc;

use crate::app::metrics::MetricValue;

pub struct Report {
    pub title: String,
    pub content: String,
}

pub fn generate_report_from(metric_values: &[MetricValue]) -> Result<Report, Box<dyn Error>> {
    let mut render_values = Map::new();
    let now = get_now();
    let title = format!("Business metrics report, {}", now);

    render_values.insert("now".to_string(), to_json(&now));
    render_values.insert("metric_values".to_string(), to_json(&metric_values));

    let mut handlebars = Handlebars::new();
    // TODO should be loaded once, no need to load it every time
    handlebars.register_template_file("report", "./src/templates/report.hbs")?;

    let content = handlebars.render("report", &render_values)?;

    Ok(Report{title, content})
}

fn get_now() -> String {
    let now = Utc::now();

    now.format("%Y-%m-%d %H:%M").to_string()
}