use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorProto {
    /// Debugging information. This property is internal to Google and should not be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_info: Option<String>,
    /// Specifies where the error occurred, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// A human-readable description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A short error code that summarizes the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl Display for ErrorProto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();

        if let Some(debug_info) = &self.debug_info {
            let _ = write!(buffer, "debug_info: {}", debug_info);
        }

        if let Some(location) = &self.location {
            if !buffer.is_empty() {
                buffer += ", ";
            }
            let _ = &write!(buffer, "location: {}", location);
        }

        if let Some(message) = &self.message {
            if !buffer.is_empty() {
                buffer += ", ";
            }
            let _ = write!(buffer, "message: {}", message);
        }

        if let Some(reason) = &self.reason {
            if !buffer.is_empty() {
                buffer += ", ";
            }
            let _ = write!(buffer, "reason: {}", reason);
        }

        f.write_str(&format!("ErrorProto: {{{}}}", buffer))
    }
}
