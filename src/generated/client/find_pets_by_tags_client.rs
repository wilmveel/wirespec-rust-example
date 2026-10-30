use super::super::wirespec::*;
use super::super::endpoint::find_pets_by_tags;
use super::super::model::pet::Pet;

pub struct FindPetsByTagsClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> find_pets_by_tags::FindPetsByTags::Call for FindPetsByTagsClient<'a, S, T> {
    async fn find_pets_by_tags(&self, tags: Vec<String>) -> find_pets_by_tags::Response {
        let request = find_pets_by_tags::Request::new(tags);
        let raw_request = find_pets_by_tags::FindPetsByTags::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        find_pets_by_tags::FindPetsByTags::from_raw_response(self.serialization, raw_response)
    }
}
