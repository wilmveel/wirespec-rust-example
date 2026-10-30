use super::super::wirespec::*;
use super::super::endpoint::logout_user;

pub struct LogoutUserClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> logout_user::LogoutUser::Call for LogoutUserClient<'a, S, T> {
    async fn logout_user(&self) -> logout_user::Response {
        let request = logout_user::Request::new();
        let raw_request = logout_user::LogoutUser::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        logout_user::LogoutUser::from_raw_response(self.serialization, raw_response)
    }
}
