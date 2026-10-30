use super::super::wirespec::*;
use super::super::endpoint::get_inventory;

pub struct GetInventoryClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> get_inventory::GetInventory::Call for GetInventoryClient<'a, S, T> {
    async fn get_inventory(&self) -> get_inventory::Response {
        let request = get_inventory::Request::new();
        let raw_request = get_inventory::GetInventory::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        get_inventory::GetInventory::from_raw_response(self.serialization, raw_response)
    }
}
