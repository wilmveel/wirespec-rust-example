use super::super::wirespec::*;
use super::super::endpoint::update_pet_with_form;

pub struct UpdatePetWithFormClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> update_pet_with_form::UpdatePetWithForm::Call for UpdatePetWithFormClient<'a, S, T> {
    async fn update_pet_with_form(&self, pet_id: i64) -> update_pet_with_form::Response {
        let request = update_pet_with_form::Request::new(pet_id);
        let raw_request = update_pet_with_form::UpdatePetWithForm::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        update_pet_with_form::UpdatePetWithForm::from_raw_response(self.serialization, raw_response)
    }
}
