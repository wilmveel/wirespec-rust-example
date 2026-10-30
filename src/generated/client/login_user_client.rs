use super::super::wirespec::*;
use super::super::endpoint::login_user;

pub struct LoginUserClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> login_user::LoginUser::Call for LoginUserClient<'a, S, T> {
    async fn login_user(&self, username: String, password: String) -> login_user::Response {
        let request = login_user::Request::new(username, password);
        let raw_request = login_user::LoginUser::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        login_user::LoginUser::from_raw_response(self.serialization, raw_response)
    }
}
