use super::super::wirespec::*;
use regex;
use super::super::model::pet::Pet;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Path;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Queries;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RequestHeaders;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Request {
    pub path: Path,
    pub method: Method,
    pub queries: Queries,
    pub headers: RequestHeaders,
    pub body: Pet,
}
impl Request {
    pub fn new(body: Pet) -> Self {
        Request {
            path: Path {},
            method: Method::POST,
            queries: Queries {},
            headers: RequestHeaders {},
            body: body
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    Response405(Response405),
}
impl From<Response405> for Response { fn from(value: Response405) -> Self { Response::Response405(value) } }
#[derive(Debug, Clone, PartialEq)]
pub enum Response4XX {
    Response405(Response405),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseUnit {
    Response405(Response405),
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Response405Headers;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Response405 {
    pub status: i32,
    pub headers: Response405Headers,
    pub body: (),
}
impl Response405 {
    pub fn new() -> Self {
        Response405 {
            status: 405_i32,
            headers: Response405Headers {},
            body: ()
        }
    }
}
pub mod AddPet {
    use super::*;
    pub trait Handler {
        async fn add_pet(&self, request: Request) -> Response;
    }
    pub trait Call {
        async fn add_pet(&self, body: Pet) -> Response;
    }
    pub fn to_raw_request(serialization: &impl Serializer, request: Request) -> RawRequest {
        return RawRequest { method: format!("{:?}", request.method), path: vec![String::from("pet")], queries: std::collections::HashMap::new(), headers: std::collections::HashMap::new(), body: Some(serialization.serialize_body(&request.body, std::any::TypeId::of::<Pet>())) };
    }
    pub fn from_raw_request(serialization: &impl Deserializer, request: RawRequest) -> Request {
        return Request::new(request.body.as_ref().map(|it| serialization.deserialize_body(it, std::any::TypeId::of::<Pet>())).expect("body is null"));
    }
    pub fn to_raw_response(serialization: &impl Serializer, response: Response) -> RawResponse {
        match response {
            Response::Response405(r) => {
                return RawResponse { status_code: r.status, headers: std::collections::HashMap::new(), body: None };
            }
        }
    }
    pub fn from_raw_response(serialization: &impl Deserializer, response: RawResponse) -> Response {
        match response.status_code {
            405_i32 => {
                return Response::Response405(Response405::new());
            }
            _ => {
                panic!("Cannot match response with status: {}", response.status_code);
            }
        }
    }
    impl<C: Client> Handler for C {
        async fn add_pet(&self, request: Request) -> Response {
            let raw = to_raw_request(self.serialization(), request);
            let resp = self.transport().transport(&raw).await;
            from_raw_response(self.serialization(), resp)
        }
    }
    pub struct Api;
    impl Server for Api {
        type Req = Request;
        type Res = Response;
        fn path_template(&self) -> &'static str { "/pet" }
        fn method(&self) -> Method { Method::POST }
    }
}
