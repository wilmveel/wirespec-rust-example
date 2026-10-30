pub mod upload_file_client;
pub mod add_pet_client;
pub mod update_pet_client;
pub mod find_pets_by_status_client;
pub mod find_pets_by_tags_client;
pub mod get_pet_by_id_client;
pub mod update_pet_with_form_client;
pub mod delete_pet_client;
pub mod get_inventory_client;
pub mod place_order_client;
pub mod get_order_by_id_client;
pub mod delete_order_client;
pub mod create_users_with_list_input_client;
pub mod get_user_by_name_client;
pub mod update_user_client;
pub mod delete_user_client;
pub mod login_user_client;
pub mod logout_user_client;
pub mod create_users_with_array_input_client;
pub mod create_user_client;
use super::wirespec::*;
use super::model::api_response::ApiResponse;
use super::model::pet::Pet;
use super::model::order::Order;
use super::model::user::User;
use super::endpoint::upload_file;
use upload_file_client::UploadFileClient;
use super::endpoint::add_pet;
use add_pet_client::AddPetClient;
use super::endpoint::update_pet;
use update_pet_client::UpdatePetClient;
use super::endpoint::find_pets_by_status;
use find_pets_by_status_client::FindPetsByStatusClient;
use super::endpoint::find_pets_by_tags;
use find_pets_by_tags_client::FindPetsByTagsClient;
use super::endpoint::get_pet_by_id;
use get_pet_by_id_client::GetPetByIdClient;
use super::endpoint::update_pet_with_form;
use update_pet_with_form_client::UpdatePetWithFormClient;
use super::endpoint::delete_pet;
use delete_pet_client::DeletePetClient;
use super::endpoint::get_inventory;
use get_inventory_client::GetInventoryClient;
use super::endpoint::place_order;
use place_order_client::PlaceOrderClient;
use super::endpoint::get_order_by_id;
use get_order_by_id_client::GetOrderByIdClient;
use super::endpoint::delete_order;
use delete_order_client::DeleteOrderClient;
use super::endpoint::create_users_with_list_input;
use create_users_with_list_input_client::CreateUsersWithListInputClient;
use super::endpoint::get_user_by_name;
use get_user_by_name_client::GetUserByNameClient;
use super::endpoint::update_user;
use update_user_client::UpdateUserClient;
use super::endpoint::delete_user;
use delete_user_client::DeleteUserClient;
use super::endpoint::login_user;
use login_user_client::LoginUserClient;
use super::endpoint::logout_user;
use logout_user_client::LogoutUserClient;
use super::endpoint::create_users_with_array_input;
use create_users_with_array_input_client::CreateUsersWithArrayInputClient;
use super::endpoint::create_user;
use create_user_client::CreateUserClient;

pub struct Client<S: Serialization, T: Transportation> {
    pub serialization: S,
    pub transportation: T,
}
impl<S: Serialization, T: Transportation> upload_file::UploadFile::Call for Client<S, T> {
    async fn upload_file(&self, pet_id: i64) -> upload_file::Response {
        UploadFileClient { serialization: &self.serialization, transportation: &self.transportation }
            .upload_file(pet_id).await
    }
}
impl<S: Serialization, T: Transportation> add_pet::AddPet::Call for Client<S, T> {
    async fn add_pet(&self, body: Pet) -> add_pet::Response {
        AddPetClient { serialization: &self.serialization, transportation: &self.transportation }
            .add_pet(body).await
    }
}
impl<S: Serialization, T: Transportation> update_pet::UpdatePet::Call for Client<S, T> {
    async fn update_pet(&self, body: Pet) -> update_pet::Response {
        UpdatePetClient { serialization: &self.serialization, transportation: &self.transportation }
            .update_pet(body).await
    }
}
impl<S: Serialization, T: Transportation> find_pets_by_status::FindPetsByStatus::Call for Client<S, T> {
    async fn find_pets_by_status(&self, status: Vec<String>) -> find_pets_by_status::Response {
        FindPetsByStatusClient { serialization: &self.serialization, transportation: &self.transportation }
            .find_pets_by_status(status).await
    }
}
impl<S: Serialization, T: Transportation> find_pets_by_tags::FindPetsByTags::Call for Client<S, T> {
    async fn find_pets_by_tags(&self, tags: Vec<String>) -> find_pets_by_tags::Response {
        FindPetsByTagsClient { serialization: &self.serialization, transportation: &self.transportation }
            .find_pets_by_tags(tags).await
    }
}
impl<S: Serialization, T: Transportation> get_pet_by_id::GetPetById::Call for Client<S, T> {
    async fn get_pet_by_id(&self, pet_id: i64) -> get_pet_by_id::Response {
        GetPetByIdClient { serialization: &self.serialization, transportation: &self.transportation }
            .get_pet_by_id(pet_id).await
    }
}
impl<S: Serialization, T: Transportation> update_pet_with_form::UpdatePetWithForm::Call for Client<S, T> {
    async fn update_pet_with_form(&self, pet_id: i64) -> update_pet_with_form::Response {
        UpdatePetWithFormClient { serialization: &self.serialization, transportation: &self.transportation }
            .update_pet_with_form(pet_id).await
    }
}
impl<S: Serialization, T: Transportation> delete_pet::DeletePet::Call for Client<S, T> {
    async fn delete_pet(&self, pet_id: i64, api_key: Option<String>) -> delete_pet::Response {
        DeletePetClient { serialization: &self.serialization, transportation: &self.transportation }
            .delete_pet(pet_id, api_key).await
    }
}
impl<S: Serialization, T: Transportation> get_inventory::GetInventory::Call for Client<S, T> {
    async fn get_inventory(&self) -> get_inventory::Response {
        GetInventoryClient { serialization: &self.serialization, transportation: &self.transportation }
            .get_inventory().await
    }
}
impl<S: Serialization, T: Transportation> place_order::PlaceOrder::Call for Client<S, T> {
    async fn place_order(&self, body: Order) -> place_order::Response {
        PlaceOrderClient { serialization: &self.serialization, transportation: &self.transportation }
            .place_order(body).await
    }
}
impl<S: Serialization, T: Transportation> get_order_by_id::GetOrderById::Call for Client<S, T> {
    async fn get_order_by_id(&self, order_id: i64) -> get_order_by_id::Response {
        GetOrderByIdClient { serialization: &self.serialization, transportation: &self.transportation }
            .get_order_by_id(order_id).await
    }
}
impl<S: Serialization, T: Transportation> delete_order::DeleteOrder::Call for Client<S, T> {
    async fn delete_order(&self, order_id: i64) -> delete_order::Response {
        DeleteOrderClient { serialization: &self.serialization, transportation: &self.transportation }
            .delete_order(order_id).await
    }
}
impl<S: Serialization, T: Transportation> create_users_with_list_input::CreateUsersWithListInput::Call for Client<S, T> {
    async fn create_users_with_list_input(&self, body: Vec<User>) -> create_users_with_list_input::Response {
        CreateUsersWithListInputClient { serialization: &self.serialization, transportation: &self.transportation }
            .create_users_with_list_input(body).await
    }
}
impl<S: Serialization, T: Transportation> get_user_by_name::GetUserByName::Call for Client<S, T> {
    async fn get_user_by_name(&self, username: String) -> get_user_by_name::Response {
        GetUserByNameClient { serialization: &self.serialization, transportation: &self.transportation }
            .get_user_by_name(username).await
    }
}
impl<S: Serialization, T: Transportation> update_user::UpdateUser::Call for Client<S, T> {
    async fn update_user(&self, username: String, body: User) -> update_user::Response {
        UpdateUserClient { serialization: &self.serialization, transportation: &self.transportation }
            .update_user(username, body).await
    }
}
impl<S: Serialization, T: Transportation> delete_user::DeleteUser::Call for Client<S, T> {
    async fn delete_user(&self, username: String) -> delete_user::Response {
        DeleteUserClient { serialization: &self.serialization, transportation: &self.transportation }
            .delete_user(username).await
    }
}
impl<S: Serialization, T: Transportation> login_user::LoginUser::Call for Client<S, T> {
    async fn login_user(&self, username: String, password: String) -> login_user::Response {
        LoginUserClient { serialization: &self.serialization, transportation: &self.transportation }
            .login_user(username, password).await
    }
}
impl<S: Serialization, T: Transportation> logout_user::LogoutUser::Call for Client<S, T> {
    async fn logout_user(&self) -> logout_user::Response {
        LogoutUserClient { serialization: &self.serialization, transportation: &self.transportation }
            .logout_user().await
    }
}
impl<S: Serialization, T: Transportation> create_users_with_array_input::CreateUsersWithArrayInput::Call for Client<S, T> {
    async fn create_users_with_array_input(&self, body: Vec<User>) -> create_users_with_array_input::Response {
        CreateUsersWithArrayInputClient { serialization: &self.serialization, transportation: &self.transportation }
            .create_users_with_array_input(body).await
    }
}
impl<S: Serialization, T: Transportation> create_user::CreateUser::Call for Client<S, T> {
    async fn create_user(&self, body: User) -> create_user::Response {
        CreateUserClient { serialization: &self.serialization, transportation: &self.transportation }
            .create_user(body).await
    }
}
