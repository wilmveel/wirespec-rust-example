use super::super::wirespec::*;
use super::super::endpoint::get_order_by_id;
use super::super::model::order::Order;

pub struct GetOrderByIdClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> get_order_by_id::GetOrderById::Call for GetOrderByIdClient<'a, S, T> {
    async fn get_order_by_id(&self, order_id: i64) -> get_order_by_id::Response {
        let request = get_order_by_id::Request::new(order_id);
        let raw_request = get_order_by_id::GetOrderById::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        get_order_by_id::GetOrderById::from_raw_response(self.serialization, raw_response)
    }
}
