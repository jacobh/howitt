#[derive(Debug)]
pub struct LoginVerificationFailed;

impl warp::reject::Reject for LoginVerificationFailed {}
