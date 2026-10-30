use super::super::wirespec::*;
use super::super::endpoint::create_users_with_array_input;
use super::super::model::user::User;

pub struct CreateUsersWithArrayInputClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> create_users_with_array_input::CreateUsersWithArrayInput::Call for CreateUsersWithArrayInputClient<'a, S, T> {
    async fn create_users_with_array_input(&self, body: Vec<User>) -> create_users_with_array_input::Response {
        let request = create_users_with_array_input::Request::new(body);
        let raw_request = create_users_with_array_input::CreateUsersWithArrayInput::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        create_users_with_array_input::CreateUsersWithArrayInput::from_raw_response(self.serialization, raw_response)
    }
}
