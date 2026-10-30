use super::super::wirespec::*;
use super::super::endpoint::add_pet;
use super::super::model::pet::Pet;

pub struct AddPetClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> add_pet::AddPet::Call for AddPetClient<'a, S, T> {
    async fn add_pet(&self, body: Pet) -> add_pet::Response {
        let request = add_pet::Request::new(body);
        let raw_request = add_pet::AddPet::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        add_pet::AddPet::from_raw_response(self.serialization, raw_response)
    }
}
