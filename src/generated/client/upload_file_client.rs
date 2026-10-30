use super::super::wirespec::*;
use super::super::endpoint::upload_file;
use super::super::model::api_response::ApiResponse;

pub struct UploadFileClient<'a, S: Serialization, T: Transportation> {
    pub serialization: &'a S,
    pub transportation: &'a T,
}
impl<'a, S: Serialization, T: Transportation> upload_file::UploadFile::Call for UploadFileClient<'a, S, T> {
    async fn upload_file(&self, pet_id: i64) -> upload_file::Response {
        let request = upload_file::Request::new(pet_id);
        let raw_request = upload_file::UploadFile::to_raw_request(self.serialization, request);
        let raw_response = self.transportation.transport(&raw_request).await;
        upload_file::UploadFile::from_raw_response(self.serialization, raw_response)
    }
}
