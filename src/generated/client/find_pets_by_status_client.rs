use super::super::wirespec::*;
use super::super::endpoint::find_pets_by_status;
use super::super::model::pet::Pet;

pub struct FindPetsByStatusClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> find_pets_by_status::FindPetsByStatus::Call for FindPetsByStatusClient<'a, S, T> {
    async fn find_pets_by_status(&self, status: Vec<String>) -> find_pets_by_status::Response {
        let request = find_pets_by_status::Request::new(status);
        let raw_request = find_pets_by_status::FindPetsByStatus::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        find_pets_by_status::FindPetsByStatus::from_raw_response(self.serialization, raw_response)
    }
}
