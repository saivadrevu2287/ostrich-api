use std::{error::Error, fmt};

#[derive(Debug, Copy, Clone)]
pub enum OstrichErrorType {
    EmailError,
    ListingResultError,
    PropertyResultError,
    ApiError,
}

#[derive(Debug)]
pub struct OstrichError {
    pub details: String,
    pub etype: OstrichErrorType,
}

impl OstrichError {
    pub fn new(msg: String, etype: OstrichErrorType) -> OstrichError {
        OstrichError {
            details: msg,
            etype,
        }
    }
}

impl fmt::Display for OstrichError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}{}", self.etype, self.details)
    }
}

impl Error for OstrichError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn map_ostrich_error(e: OstrichError) -> () {
    log::error!("{:?}", e);
}
