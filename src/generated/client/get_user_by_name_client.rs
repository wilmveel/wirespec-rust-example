use super::super::wirespec::*;
use super::super::endpoint::get_user_by_name;
use super::super::model::user::User;

pub struct GetUserByNameClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> get_user_by_name::GetUserByName::Call for GetUserByNameClient<'a, S, T> {
    async fn get_user_by_name(&self, username: String) -> get_user_by_name::Response {
        let request = get_user_by_name::Request::new(username);
        let raw_request = get_user_by_name::GetUserByName::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        get_user_by_name::GetUserByName::from_raw_response(self.serialization, raw_response)
    }
}
