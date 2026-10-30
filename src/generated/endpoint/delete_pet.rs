use super::super::wirespec::*;
use regex;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Path {
    pub pet_id: i64,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Queries;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RequestHeaders {
    pub api_key: Option<String>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Request {
    pub path: Path,
    pub method: Method,
    pub queries: Queries,
    pub headers: RequestHeaders,
    pub body: (),
}
impl Request {
    pub fn new(pet_id: i64, api_key: Option<String>) -> Self {
        Request {
            path: Path { pet_id: pet_id },
            method: Method::DELETE,
            queries: Queries {},
            headers: RequestHeaders { api_key: api_key },
            body: ()
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    Response400(Response400),
    Response404(Response404),
}
impl From<Response400> for Response { fn from(value: Response400) -> Self { Response::Response400(value) } }
impl From<Response404> for Response { fn from(value: Response404) -> Self { Response::Response404(value) } }
#[derive(Debug, Clone, PartialEq)]
pub enum Response4XX {
    Response400(Response400),
    Response404(Response404),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseUnit {
    Response400(Response400),
    Response404(Response404),
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Response400Headers;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Response400 {
    pub status: i32,
    pub headers: Response400Headers,
    pub body: (),
}
impl Response400 {
    pub fn new() -> Self {
        Response400 {
            status: 400_i32,
            headers: Response400Headers {},
            body: ()
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Response404Headers;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Response404 {
    pub status: i32,
    pub headers: Response404Headers,
    pub body: (),
}
impl Response404 {
    pub fn new() -> Self {
        Response404 {
            status: 404_i32,
            headers: Response404Headers {},
            body: ()
        }
    }
}
pub mod DeletePet {
    use super::*;
    pub trait Handler {
        async fn delete_pet(&self, request: Request) -> Response;
    }
    pub trait Call {
        async fn delete_pet(&self, pet_id: i64, api_key: Option<String>) -> Response;
    }
    pub fn to_raw_request(serialization: &impl Serializer, request: Request) -> RawRequest {
        return RawRequest { method: format!("{:?}", request.method), path: vec![String::from("pet"), serialization.serialize_path(&request.path.pet_id, std::any::TypeId::of::<i64>())], queries: std::collections::HashMap::new(), headers: std::collections::HashMap::from([(String::from("api_key"), request.headers.api_key.as_ref().map(|it| serialization.serialize_param(it, std::any::TypeId::of::<String>())).unwrap_or(Vec::<String>::new()))]), body: None };
    }
    pub fn from_raw_request(serialization: &impl Deserializer, request: RawRequest) -> Request {
        return Request::new(serialization.deserialize_path(&request.path[1], std::any::TypeId::of::<i64>()), request.headers.iter().find(|(k, _)| k.eq_ignore_ascii_case("api_key")).map(|(_, v)| v.clone()).as_ref().map(|it| serialization.deserialize_param(it, std::any::TypeId::of::<String>())));
    }
    pub fn to_raw_response(serialization: &impl Serializer, response: Response) -> RawResponse {
        match response {
            Response::Response400(r) => {
                return RawResponse { status_code: r.status, headers: std::collections::HashMap::new(), body: None };
            }
            Response::Response404(r) => {
                return RawResponse { status_code: r.status, headers: std::collections::HashMap::new(), body: None };
            }
        }
    }
    pub fn from_raw_response(serialization: &impl Deserializer, response: RawResponse) -> Response {
        match response.status_code {
            400_i32 => {
                return Response::Response400(Response400::new());
            }
            404_i32 => {
                return Response::Response404(Response404::new());
            }
            _ => {
                panic!("Cannot match response with status: {}", response.status_code);
            }
        }
    }
    impl<C: Client> Handler for C {
        async fn delete_pet(&self, request: Request) -> Response {
            let raw = to_raw_request(self.serialization(), request);
            let resp = self.transport().transport(&raw).await;
            from_raw_response(self.serialization(), resp)
        }
    }
    pub struct Api;
    impl Server for Api {
        type Req = Request;
        type Res = Response;
        fn path_template(&self) -> &'static str { "/pet/{petId}" }
        fn method(&self) -> Method { Method::DELETE }
    }
}
