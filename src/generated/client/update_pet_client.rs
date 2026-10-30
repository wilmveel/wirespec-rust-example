use super::super::wirespec::*;
use super::super::endpoint::update_pet;
use super::super::model::pet::Pet;

pub struct UpdatePetClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> update_pet::UpdatePet::Call for UpdatePetClient<'a, S, T> {
    async fn update_pet(&self, body: Pet) -> update_pet::Response {
        let request = update_pet::Request::new(body);
        let raw_request = update_pet::UpdatePet::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        update_pet::UpdatePet::from_raw_response(self.serialization, raw_response)
    }
}
