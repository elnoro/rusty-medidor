use lettre::{SmtpClient, SmtpTransport, Transport, ClientSecurity};
use lettre::smtp::ConnectionReuseParameters;
use lettre_email::EmailBuilder;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::client::net::ClientTlsParameters;
use native_tls::{Protocol, TlsConnector};
use crate::app::get_env;

pub fn create_from_env(host_env: &str, username_env: &str, password_env: &str, encryption_env: &str) -> Mailer {
    let config = EmailServerConfig::from_env(host_env, username_env, password_env, encryption_env);

    Mailer::new(config)
}

#[derive(Debug)]
struct EmailServerConfig {
    host: String,
    username: String,
    password: String,
    encryption_enabled: bool,
}

impl EmailServerConfig {
    pub fn from_env(host_env: &str, username_env: &str, password_env: &str, encryption_env: &str) -> EmailServerConfig {
        let host = get_env(host_env, "localhost");
        let username = get_env(username_env, "");
        let password = get_env(password_env, "");
        let encryption = get_env(encryption_env, "");
        let encryption_enabled = if "tls" == encryption { true } else { false };
        EmailServerConfig { host, username, password, encryption_enabled }
    }
}

pub struct Mailer {
    smtp: SmtpTransport,
}

impl Mailer {
    fn new(config: EmailServerConfig) -> Mailer {
        let port = if config.encryption_enabled { 587 } else { 1025 };
        let smtp = SmtpClient::new((config.host.as_str(), port), Mailer::get_security(&config))
            .expect(&format!("Cannot connect to SMTP server at {}, check settings", &config.host))
            .credentials(Credentials::new(config.username, config.password))
            .authentication_mechanism(Mechanism::Login)
            .smtp_utf8(true)
            .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
            .transport();

        Mailer { smtp }
    }

    fn get_security(config: &EmailServerConfig) -> ClientSecurity {
        if config.encryption_enabled {
            let mut tls_builder = TlsConnector::builder();
            tls_builder.min_protocol_version(Some(Protocol::Tlsv10));
            let tls_parameters = ClientTlsParameters::new(
                config.host.as_str().to_string(),
                tls_builder.build().unwrap()
            );

            ClientSecurity::Required(tls_parameters)
        } else {
            ClientSecurity::None
        }
    }

    pub fn send_email(&mut self, from: &str, to: &str, subject: &str, body: &str) {
        let email = EmailBuilder::new()
            .to((to, ""))
            .from(from)
            .subject(subject)
            .html(body)
            .build()
            .unwrap();
        
        self.send(email.into());
    }

    fn send(&mut self, email: lettre::SendableEmail) {
        let result = self.smtp.send(email);   
        if result.is_err() {
            error!("Failed to send an email: {:?}", result);
        }
    }
}