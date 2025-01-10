use async_graphql::Object;
use howitt::services::user::auth::Login;

pub struct Viewer(pub Login);

#[Object]
impl Viewer {
    async fn id(&self) -> String {
        self.0.session.user_id.to_string()
    }
}
