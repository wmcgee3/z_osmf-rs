use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MigratedRecall {
    Error,
    NoWait,
    Wait,
}

impl From<MigratedRecall> for HeaderValue {
    fn from(val: MigratedRecall) -> HeaderValue {
        match val {
            MigratedRecall::Error => "error",
            MigratedRecall::NoWait => "nowait",
            MigratedRecall::Wait => "wait",
        }
        .try_into()
        .unwrap()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ObtainEnq {
    Exclusive,
    SharedReadWrite,
}

impl From<ObtainEnq> for HeaderValue {
    fn from(val: ObtainEnq) -> HeaderValue {
        match val {
            ObtainEnq::Exclusive => "EXCLU",
            ObtainEnq::SharedReadWrite => "SHRW",
        }
        .try_into()
        .unwrap()
    }
}
