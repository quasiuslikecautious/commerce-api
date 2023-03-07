use http::Request;
use tower_http::request_id;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct RequestId {

}

impl request_id::MakeRequestId for RequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<request_id::RequestId> {
        Some(request_id::RequestId::new(Uuid::new_v4().to_string().parse().unwrap()))
    }
}
