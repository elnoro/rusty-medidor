extern crate mysql;
extern crate lettre;
extern crate lettre_email;
extern crate chrono;
extern crate handlebars;
extern crate native_tls;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate env_logger;
#[macro_use]
extern crate log;

mod app;

use app::{*};

fn main() {
    env_logger::init();
    let mailer = mailer::create_from_env("MAIL_HOST", "MAIL_USERNAME", "MAIL_PASSWORD", "MAIL_ENV");

    let metrics_collector = metrics::MetricsCollector::connect(&get_env("DB_CONNECTION_STRING", ""));
    let collected = metrics_collector.collect_all();
    
    send_report(mailer, &collected);
}
