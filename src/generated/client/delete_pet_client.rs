use super::super::wirespec::*;
use super::super::endpoint::delete_pet;

pub struct DeletePetClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> delete_pet::DeletePet::Call for DeletePetClient<'a, S, T> {
    async fn delete_pet(&self, pet_id: i64, api_key: Option<String>) -> delete_pet::Response {
        let request = delete_pet::Request::new(pet_id, api_key);
        let raw_request = delete_pet::DeletePet::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        delete_pet::DeletePet::from_raw_response(self.serialization, raw_response)
    }
}
