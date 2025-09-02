use base64::{Engine, engine::general_purpose};
use regex::Regex;
use reqwest::{Error as ReqwestError, header};
use serde_json::Error as SerdeError;

use std::error::Error;
use std::fmt::Display;
use std::os::windows::process::CommandExt;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct LeagueClientConfig {
    pub port: String,
    pub auth_token: String,
    pub auth_token_encoded: String,
}

#[derive(Debug)]
pub struct LeagueClientError {
    pub error: Box<dyn Error>,
    pub response_text: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PortAndToken {
    pub port: String,
    pub auth_token: String,
}

impl LeagueClientConfig {
    pub fn new() -> Self {
        Self::default()
    }

    // Gives the full local LCU url from a given path
    pub fn build_url(&self, path: &str) -> String {
        let path = path.trim_start_matches("/");
        format!("https://127.0.0.1:{}/{}", self.port, path)
    }

    // Builds the autorization headers required to connect to the LCU
    pub fn build_client_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                format!("Basic {}", self.auth_token_encoded).as_str(),
            )
            .unwrap(),
        );
        headers
    }
}

impl Default for LeagueClientConfig {
    fn default() -> Self {
        let mut config = LeagueClientConfig {
            port: String::new(),
            auth_token: String::new(),
            auth_token_encoded: String::new(),
        };

        if let Ok(v) = get_process_port_token() {
            config.port = v.port;
            config.auth_token = v.auth_token;
            config.auth_token_encoded = general_purpose::STANDARD
                .encode(format!("riot:{}", config.auth_token));
        }

        config
    }
}

impl From<ReqwestError> for LeagueClientError {
    fn from(err: ReqwestError) -> Self {
        LeagueClientError {
            error: Box::new(err),
            response_text: None,
        }
    }
}

impl From<SerdeError> for LeagueClientError {
    fn from(err: SerdeError) -> Self {
        LeagueClientError {
            error: Box::new(err),
            response_text: None,
        }
    }
}

impl Display for LeagueClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref text) = self.response_text {
            write!(f, "{}: {}", self.error, text)
        } else {
            write!(f, "{}", self.error)
        }
    }
}

impl Error for LeagueClientError {}

// Blocking function, will block the current thread until the commands are done being executed
pub fn get_process_port_token() -> Result<PortAndToken, Box<dyn Error>> {
    let re_port = Regex::new(r"--app-port=([0-9]+)")?;
    let re_auth_token = Regex::new(r"--remoting-auth-token=([\w-]+)")?;

    // Try WMIC first (Windows 10)
    let wmic_cmd = Command::new("wmic")
        .args([
            "PROCESS",
            "WHERE",
            "name='LeagueClientUx.exe'",
            "GET",
            "commandline",
        ])
        .creation_flags(0x08000000)
        .output();

    let output_string: String;
    let cmd_output_str: &str = match wmic_cmd {
        Ok(ref out) if out.status.success() => {
            output_string = String::from_utf8_lossy(&out.stdout).to_string();
            &output_string
        }
        _ => {
            // When wmic fails, try PowerShell WMI (Windows 11)
            let wmi_cmd = r#"Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'LeagueClientUx.exe' } | Select-Object -ExpandProperty CommandLine"#;
            let wmi_out = Command::new("powershell")
                .args(["-Command", wmi_cmd])
                .creation_flags(0x08000000)
                .output()?;
            output_string =
                String::from_utf8_lossy(&wmi_out.stdout).to_string();
            &output_string
        }
    };

    let port = re_port
        .captures(cmd_output_str)
        .and_then(|v| v.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or("Port not found")?;

    let auth_token = re_auth_token
        .captures(cmd_output_str)
        .and_then(|v| v.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or("Auth token not found")?;

    Ok(PortAndToken { port, auth_token })
}
