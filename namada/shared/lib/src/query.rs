use namada::sdk::rpc::query_epoch;
use wasm_bindgen::prelude::*;
use crate::rpc_client::HttpClient;
use gloo_utils::format::JsValueSerdeExt;

#[wasm_bindgen]
/// Represents an API for querying the ledger
pub struct Query {
    client: HttpClient,
}

#[wasm_bindgen]
impl Query {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> Query {
        let client = HttpClient::new(url);
        Query { client }
    }

    /// Queries current epoch
    pub async fn query_epoch(&self) -> Result<JsValue, JsError> {
        let epoch = query_epoch(&self.client).await?;
        match JsValue::from_serde(&epoch) {
            Ok(v) => Ok(v),
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }
}