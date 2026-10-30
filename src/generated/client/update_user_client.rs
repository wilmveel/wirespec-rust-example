use super::super::wirespec::*;
use super::super::endpoint::update_user;
use super::super::model::user::User;

pub struct UpdateUserClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> update_user::UpdateUser::Call for UpdateUserClient<'a, S, T> {
    async fn update_user(&self, username: String, body: User) -> update_user::Response {
        let request = update_user::Request::new(username, body);
        let raw_request = update_user::UpdateUser::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        update_user::UpdateUser::from_raw_response(self.serialization, raw_response)
    }
}
