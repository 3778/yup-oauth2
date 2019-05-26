#![allow(dead_code)]

//! Helper functions allowing you to avoid writing boilerplate code for common operations, such as
//! parsing JSON or reading files.

// Copyright (c) 2016 Google Inc (lewinb@google.com).
//
// Refer to the project root for licensing information.

use serde_json;

use std::fs;
use std::io::{self, Read};
use std::path::Path;

use crate::service_account::ServiceAccountKey;
use crate::types::{ApplicationSecret, ConsoleApplicationSecret};

/// Read an application secret from a file.
pub fn read_application_secret(path: &Path) -> io::Result<ApplicationSecret> {
    let mut secret = String::new();
    let mut file = fs::OpenOptions::new().read(true).open(path)?;
    file.read_to_string(&mut secret)?;

    parse_application_secret(&secret)
}

/// Read an application secret from a JSON string.
pub fn parse_application_secret<S: AsRef<str>>(secret: S) -> io::Result<ApplicationSecret> {
    let result: serde_json::Result<ConsoleApplicationSecret> =
        serde_json::from_str(secret.as_ref());
    match result {
        Err(e) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Bad application secret: {}", e),
        )),
        Ok(decoded) => {
            if decoded.web.is_some() {
                Ok(decoded.web.unwrap())
            } else if decoded.installed.is_some() {
                Ok(decoded.installed.unwrap())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown application secret format",
                ))
            }
        }
    }
}

/// Read a service account key from a JSON file. You can download the JSON keys from the Google
/// Cloud Console or the respective console of your service provider.
pub fn service_account_key_from_file<S: AsRef<Path>>(path: S) -> io::Result<ServiceAccountKey> {
    let mut key = String::new();
    let mut file = fs::OpenOptions::new().read(true).open(path)?;
    file.read_to_string(&mut key)?;

    match serde_json::from_str(&key) {
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))),
        Ok(decoded) => Ok(decoded),
    }
}
