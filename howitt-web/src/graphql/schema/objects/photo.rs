use async_graphql::Object;
use howitt::models::photo::PhotoId;

use crate::graphql::schema::ModelId;

pub struct Photo<ID>(pub howitt::models::photo::Photo<ID>);

#[Object]
impl<ID: howitt::models::ModelId> Photo<ID> {
    async fn id(&self) -> ModelId<PhotoId> {
        ModelId(self.0.id)
    }
    async fn url(&self) -> &url::Url {
        &self.0.url
    }
    async fn caption(&self) -> Option<&str> {
        self.0.caption.as_deref()
    }
}
