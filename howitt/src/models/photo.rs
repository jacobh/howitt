use serde::{Deserialize, Serialize};

use super::external_ref::ExternalRef;

crate::model_id!(PhotoId, "PHOTO");

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Photo<ID> {
    pub model_id: ID,
    pub id: PhotoId,
    pub external_ref: Option<ExternalRef>,
    pub url: url::Url,
    pub caption: Option<String>,
}
