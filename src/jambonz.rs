use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case", tag = "verb")]
pub(crate) enum Verb {
    Pause(Pause),
    Say(Say),
}

/// See https://www.jambonz.org/docs/webhooks/pause/
#[derive(Debug, Clone, Serialize)]
pub(crate) struct Pause {
    pub length: u64,
}

impl Pause {
    pub(crate) fn new(length: u64) -> Self {
        Self { length }
    }
}

/// See https://www.jambonz.org/docs/webhooks/say/
#[derive(Debug, Clone, Serialize)]
pub(crate) struct Say {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesizer: Option<SaySynthesizer>,
}

/// See https://www.jambonz.org/docs/webhooks/say/
#[derive(Debug, Clone, Serialize)]
pub(crate) struct SaySynthesizer {
    pub vendor: String,

    pub language: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    pub voice: String,
}

/// See https://www.jambonz.org/docs/webhooks/overview/
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CallStatusDetails {
    #[allow(unused)]
    call_id: String,

    #[allow(unused)]
    call_sid: String,

    pub call_status: CallStatus,

    #[allow(unused)]
    call_termination_by: Option<String>,

    #[allow(unused)]
    duration: Option<i64>,

    pub from: String,
}

/// See https://www.jambonz.org/docs/webhooks/overview/
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum CallStatus {
    Trying,
    Ringing,
    EarlyMedia,
    InProgress,
    Completed,
    Failed,
    Busy,
    NoAnswer,
}
