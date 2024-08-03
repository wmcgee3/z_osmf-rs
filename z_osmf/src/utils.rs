use std::num::NonZeroU32;
use std::str::FromStr;

use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RecordRange {
    StartCount(u32, NonZeroU32),
    StartEnd(Option<u32>, u32),
}

impl From<RecordRange> for HeaderValue {
    fn from(value: RecordRange) -> Self {
        match value {
            RecordRange::StartCount(start, count) => format!("{},{}", start, count),
            RecordRange::StartEnd(Some(start), end) => format!("{}-{}", start, end),
            RecordRange::StartEnd(None, end) => format!("-{}", end),
        }
        .try_into()
        .unwrap()
    }
}

impl FromStr for RecordRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some(s) = s.strip_prefix('-') {
            return Ok(RecordRange::StartEnd(None, s.parse()?));
        }

        if let Some((start, end)) = s.split_once('-') {
            return Ok(RecordRange::StartEnd(Some(start.parse()?), end.parse()?));
        }

        if let Some((start, count)) = s.split_once(',') {
            return Ok(RecordRange::StartCount(start.parse()?, count.parse()?));
        }

        Err(Error::RecordRange(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_range_into_header_value() {
        let header_value: HeaderValue = RecordRange::StartEnd(Some(0), 249).into();
        assert_eq!(header_value, HeaderValue::from_static("0-249"));

        let header_value: HeaderValue = RecordRange::StartEnd(None, 249).into();
        assert_eq!(header_value, HeaderValue::from_static("-249"));

        let header_value: HeaderValue =
            RecordRange::StartCount(0, NonZeroU32::new(1).unwrap()).into();
        assert_eq!(header_value, HeaderValue::from_static("0,1"));
    }

    #[test]
    fn test_record_range_from_str() {
        let record_range = RecordRange::from_str("0-249").unwrap();
        assert_eq!(record_range, RecordRange::StartEnd(Some(0), 249));

        let record_range = RecordRange::from_str("-249").unwrap();
        assert_eq!(record_range, RecordRange::StartEnd(None, 249));

        let record_range = RecordRange::from_str("0,1").unwrap();
        assert_eq!(
            record_range,
            RecordRange::StartCount(0, NonZeroU32::new(1).unwrap())
        );

        assert!(RecordRange::from_str("-NONSENSE").is_err());

        assert!(RecordRange::from_str("NON-249").is_err());

        assert!(RecordRange::from_str("0-SENSE").is_err());

        assert!(RecordRange::from_str("NON,1").is_err());

        assert!(RecordRange::from_str("0,SENSE").is_err());

        assert!(RecordRange::from_str("NONSENSE").is_err());
    }
}
