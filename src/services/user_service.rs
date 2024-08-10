use tonic::{Request, Response, Status};
use mongodb::Database;
use crate::models::user_model::UserModel;
use crate::config::pb::user_service_server::UserService;
use crate::config::pb::{User, UserId};

pub struct ArtieUserService {
    pub db: Database,
}

#[tonic::async_trait]
impl UserService for ArtieUserService {
    async fn add_user(&self, request: Request<User>) -> Result<Response<UserId>, Status> {
        let user = request.into_inner();
        let collection = self.db.collection("User");

        let new_user = UserModel {
            id: None,
            login: user.login,
            password: user.password,
            first_name: Some(user.first_name),
            last_name:  Some(user.last_name),
            email: user.email,
            institution_id:  Some(user.institution_id),
            active: user.active,
            role: user.role,
        };

        let insert_result = collection.insert_one(new_user).await.unwrap();
        let id = insert_result.inserted_id.as_str().unwrap().to_string();

        Ok(Response::new(UserId { id }))
    }
}

impl ArtieUserService {
    pub fn new(db: Database) -> Self {
        ArtieUserService { db }
    }
}
