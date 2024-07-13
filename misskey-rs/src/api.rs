use super::autogen::apiClientJSDoc;

use super::api_types::Endpoints;
pub use super::api_types::SwitchCaseResponseType;
use super::autogen::endpoint::endpointReqTypes;

use serde_json;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

pub static MK_API_ERROR: &'static str = "MK_API_ERROR";

#[derive(Debug, Clone)]
pub struct APIError {
    pub id: String,
    pub code: String,
    pub message: String,
    pub kind: APIErrorKind,
    pub info: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum APIErrorKind {
    Client,
    Server,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "APIError {{ id: {}, code: {}, message: {}, kind: {:?}, info: {:?} }}",
            self.id, self.code, self.message, self.kind, self.info
        )
    }
}

pub fn isApiError(reason: &HashMap<String, serde_json::Value>) -> bool {
    reason.get(MK_API_ERROR).map_or(false, |v| v == &true)
}

type FetchLike =
    Box<dyn Fn(&str, Option<FetchOptions>) -> Pin<Box<dyn Future<Output = FetchResponse>>>>;

#[derive(Default)]
struct FetchOptions {
    method: Option<String>,
    body: Option<FetchBody>,
    credentials: Option<RequestCredentials>,
    cache: Option<RequestCache>,
    headers: HashMap<String, String>,
}

#[derive(Clone)]
enum FetchBody {
    Blob(Vec<u8>),
    FormData(Vec<(String, Vec<u8>)>),
    String(String),
}

//#[derive(Debug)]
struct FetchResponse {
    status: u16,
    json: Box<dyn Fn() -> Pin<Box<dyn Future<Output = serde_json::Value>>>>,
}
