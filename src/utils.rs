use crate::DynResult;
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use thousands::Separable;

#[derive(Deserialize)]
pub struct JwtPayload {
    pub sub: String,
    pub email: String,
}

// Base64 ( HMAC_SHA256 ( "Client Secret Key", "Username" + "Client Id" ) )
pub fn base64_hmac(key: String, message: String) -> DynResult<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes())?;
    mac.update(message.as_bytes());

    let secret_hash = base64::encode(mac.finalize().into_bytes());
    Ok(secret_hash)
}

pub fn now() -> chrono::naive::NaiveDateTime {
    Utc::now().naive_local()
}

pub fn decode_jwt(token: &str) -> DynResult<JwtPayload> {
    let body = token.split(".").collect::<Vec<&str>>();
    let jwt_string = String::from_utf8(base64::decode(body[1])?)?;
    let x: JwtPayload = serde_json::from_str(&jwt_string)?;
    Ok(x)
}

pub fn decode_bearer(header: &str) -> DynResult<JwtPayload> {
    let parts = header.split(" ").collect::<Vec<&str>>();
    decode_jwt(parts[1])
}

pub fn format_optional_float(x: Option<f64>) -> String {
    x.map_or(String::from("N/A"), |x| {
        format!("${}", x.separate_with_commas())
    })
}

pub fn format_optional_string(x: Option<String>) -> String {
    x.map_or(String::from("N/A"), |x| x)
}

#[test]
fn auth_id_in_jwt() {
    let x = decode_jwt("eyJraWQiOiIxUExQVXpIYThLNGsxaEZjTkN6cXpOWThuXC8ydjd4UVI3NUFkZ1BcL0duWEE9IiwiYWxnIjoiUlMyNTYifQ.eyJzdWIiOiI2MWI5NzZhNS04MDdhLTQyNTctYmE1ZS00Y2RlYTZjMzlkMTgiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMi5hbWF6b25hd3MuY29tXC91cy1lYXN0LTJfMG9jMzJ5N2RtIiwiY29nbml0bzp1c2VybmFtZSI6ImhnbWF4d2VsbGtpbmdAZ21haWwuY29tIiwib3JpZ2luX2p0aSI6IjMxNWUxNGMwLWM2OWItNDI2MS1hNWMxLTBlMWI3OTdhNzhhMSIsImF1ZCI6ImQ5djU4Z282OTJpY2xzNDRkMDZwb2M3N2EiLCJldmVudF9pZCI6IjhmZTRjYjFkLThiNmYtNDcxNC1hMDRhLTFhYzZkZWIxNDQ1ZiIsInRva2VuX3VzZSI6ImlkIiwiYXV0aF90aW1lIjoxNjUzMDAyNDkyLCJleHAiOjE2NTMwMDYwOTIsImlhdCI6MTY1MzAwMjQ5MiwianRpIjoiZDE3OTU1ODAtZGJjNy00MTY0LTljN2YtMjBlM2RkN2NhMmNkIiwiZW1haWwiOiJoZ21heHdlbGxraW5nQGdtYWlsLmNvbSJ9.mWRuP8BENKQhbPX2H_-L0myKtS9_yvwkRnqcHCF7Aij8D3oU6in1P2x1Yf0_wh9Tn32XMzX4baMeePpz27_9GpwpX3q2_XmGpncdt1mKpSx5SqWzo3gbmTq4LcoPoay-JBYMzzuEu6be0_rnHLt-oXir6oftLzwsv8vwz5096uTcdfDrsYCBIYiEtytR6JbVrxtT8IkZVPDGtk-SNFhKTQoNPLJGlN5nVo_n4L2NWG8fFVCDMZBsKFcmkTzDCTNCcLQzfsVfdM-mYug7gq7GQkO4v8ZJpc-jnvnaaCtpVcJnwBd4nlMIT1Dd95lCCoo3Yavf82hGHiRgDVpOxk4RmQ");
    assert_eq!(x.unwrap().sub, "61b976a5-807a-4257-ba5e-4cdea6c39d18");
}

#[test]
fn test_decode_bearer() {
    let x = decode_bearer("Bearer eyJraWQiOiIxUExQVXpIYThLNGsxaEZjTkN6cXpOWThuXC8ydjd4UVI3NUFkZ1BcL0duWEE9IiwiYWxnIjoiUlMyNTYifQ.eyJzdWIiOiI2MWI5NzZhNS04MDdhLTQyNTctYmE1ZS00Y2RlYTZjMzlkMTgiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMi5hbWF6b25hd3MuY29tXC91cy1lYXN0LTJfMG9jMzJ5N2RtIiwiY29nbml0bzp1c2VybmFtZSI6ImhnbWF4d2VsbGtpbmdAZ21haWwuY29tIiwib3JpZ2luX2p0aSI6IjMxNWUxNGMwLWM2OWItNDI2MS1hNWMxLTBlMWI3OTdhNzhhMSIsImF1ZCI6ImQ5djU4Z282OTJpY2xzNDRkMDZwb2M3N2EiLCJldmVudF9pZCI6IjhmZTRjYjFkLThiNmYtNDcxNC1hMDRhLTFhYzZkZWIxNDQ1ZiIsInRva2VuX3VzZSI6ImlkIiwiYXV0aF90aW1lIjoxNjUzMDAyNDkyLCJleHAiOjE2NTMwMDYwOTIsImlhdCI6MTY1MzAwMjQ5MiwianRpIjoiZDE3OTU1ODAtZGJjNy00MTY0LTljN2YtMjBlM2RkN2NhMmNkIiwiZW1haWwiOiJoZ21heHdlbGxraW5nQGdtYWlsLmNvbSJ9.mWRuP8BENKQhbPX2H_-L0myKtS9_yvwkRnqcHCF7Aij8D3oU6in1P2x1Yf0_wh9Tn32XMzX4baMeePpz27_9GpwpX3q2_XmGpncdt1mKpSx5SqWzo3gbmTq4LcoPoay-JBYMzzuEu6be0_rnHLt-oXir6oftLzwsv8vwz5096uTcdfDrsYCBIYiEtytR6JbVrxtT8IkZVPDGtk-SNFhKTQoNPLJGlN5nVo_n4L2NWG8fFVCDMZBsKFcmkTzDCTNCcLQzfsVfdM-mYug7gq7GQkO4v8ZJpc-jnvnaaCtpVcJnwBd4nlMIT1Dd95lCCoo3Yavf82hGHiRgDVpOxk4RmQ");
    assert_eq!(x.unwrap().sub, "61b976a5-807a-4257-ba5e-4cdea6c39d18");
}

#[test]
fn email_in_jwt() {
    let x = decode_jwt("eyJraWQiOiIxUExQVXpIYThLNGsxaEZjTkN6cXpOWThuXC8ydjd4UVI3NUFkZ1BcL0duWEE9IiwiYWxnIjoiUlMyNTYifQ.eyJzdWIiOiI2MWI5NzZhNS04MDdhLTQyNTctYmE1ZS00Y2RlYTZjMzlkMTgiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLnVzLWVhc3QtMi5hbWF6b25hd3MuY29tXC91cy1lYXN0LTJfMG9jMzJ5N2RtIiwiY29nbml0bzp1c2VybmFtZSI6ImhnbWF4d2VsbGtpbmdAZ21haWwuY29tIiwib3JpZ2luX2p0aSI6IjMxNWUxNGMwLWM2OWItNDI2MS1hNWMxLTBlMWI3OTdhNzhhMSIsImF1ZCI6ImQ5djU4Z282OTJpY2xzNDRkMDZwb2M3N2EiLCJldmVudF9pZCI6IjhmZTRjYjFkLThiNmYtNDcxNC1hMDRhLTFhYzZkZWIxNDQ1ZiIsInRva2VuX3VzZSI6ImlkIiwiYXV0aF90aW1lIjoxNjUzMDAyNDkyLCJleHAiOjE2NTMwMDYwOTIsImlhdCI6MTY1MzAwMjQ5MiwianRpIjoiZDE3OTU1ODAtZGJjNy00MTY0LTljN2YtMjBlM2RkN2NhMmNkIiwiZW1haWwiOiJoZ21heHdlbGxraW5nQGdtYWlsLmNvbSJ9.mWRuP8BENKQhbPX2H_-L0myKtS9_yvwkRnqcHCF7Aij8D3oU6in1P2x1Yf0_wh9Tn32XMzX4baMeePpz27_9GpwpX3q2_XmGpncdt1mKpSx5SqWzo3gbmTq4LcoPoay-JBYMzzuEu6be0_rnHLt-oXir6oftLzwsv8vwz5096uTcdfDrsYCBIYiEtytR6JbVrxtT8IkZVPDGtk-SNFhKTQoNPLJGlN5nVo_n4L2NWG8fFVCDMZBsKFcmkTzDCTNCcLQzfsVfdM-mYug7gq7GQkO4v8ZJpc-jnvnaaCtpVcJnwBd4nlMIT1Dd95lCCoo3Yavf82hGHiRgDVpOxk4RmQ");
    assert_eq!(x.unwrap().email, "hgmaxwellking@gmail.com");
}
