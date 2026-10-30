use super::super::wirespec::*;
use super::super::endpoint::create_user;
use super::super::model::user::User;

pub struct CreateUserClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> create_user::CreateUser::Call for CreateUserClient<'a, S, T> {
    async fn create_user(&self, body: User) -> create_user::Response {
        let request = create_user::Request::new(body);
        let raw_request = create_user::CreateUser::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        create_user::CreateUser::from_raw_response(self.serialization, raw_response)
    }
}
