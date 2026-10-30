use super::super::wirespec::*;
use super::super::endpoint::delete_order;

pub struct DeleteOrderClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> delete_order::DeleteOrder::Call for DeleteOrderClient<'a, S, T> {
    async fn delete_order(&self, order_id: i64) -> delete_order::Response {
        let request = delete_order::Request::new(order_id);
        let raw_request = delete_order::DeleteOrder::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        delete_order::DeleteOrder::from_raw_response(self.serialization, raw_response)
    }
}
