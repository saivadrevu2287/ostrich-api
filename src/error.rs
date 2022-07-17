use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OstrichErrorType {
    EmailError,
    ListingError,
}

#[derive(Debug)]
pub struct OstrichError {
    details: String,
    etype: OstrichErrorType,
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
    log::error!("{:?}", e)
}
