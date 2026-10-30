use super::super::wirespec::*;
use super::super::endpoint::delete_user;

pub struct DeleteUserClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> delete_user::DeleteUser::Call for DeleteUserClient<'a, S, T> {
    async fn delete_user(&self, username: String) -> delete_user::Response {
        let request = delete_user::Request::new(username);
        let raw_request = delete_user::DeleteUser::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        delete_user::DeleteUser::from_raw_response(self.serialization, raw_response)
    }
}
