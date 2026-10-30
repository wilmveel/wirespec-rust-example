use super::super::wirespec::*;
use super::super::endpoint::place_order;
use super::super::model::order::Order;

pub struct PlaceOrderClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> place_order::PlaceOrder::Call for PlaceOrderClient<'a, S, T> {
    async fn place_order(&self, body: Order) -> place_order::Response {
        let request = place_order::Request::new(body);
        let raw_request = place_order::PlaceOrder::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        place_order::PlaceOrder::from_raw_response(self.serialization, raw_response)
    }
}
