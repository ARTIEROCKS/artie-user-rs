use tonic::{Request, Response, Status};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use crate::models::user_model::UserModel;
use crate::config::pb::user_service_server::UserService;
use crate::config::pb::{User, UserId, UserLogin};
use crate::services::security_service::validate_password;
use crate::config::error::ArtieError;

pub struct ArtieUserService {
    pub db: Database,
}

#[tonic::async_trait]
impl UserService for ArtieUserService {
    async fn add_user(&self, request: Request<User>) -> Result<Response<UserId>, Status> {
        let user = request.into_inner();
        let collection = self.db.collection::<UserModel>("User");

        let new_user = UserModel {
            id: ObjectId::new(),
            login: user.login,
            password: user.password,
            first_name: Some(user.first_name),
            last_name:  Some(user.last_name),
            email: user.email,
            institution_id:  Some(user.institution_id),
            active: user.active,
            role: user.role,
        };

        collection.insert_one(&new_user).await.map_err(|err| ArtieError::MongoDBError(err.into()))?;
        Ok(Response::new(UserId{id: new_user.id.to_string()}))
    }

    async fn login_user(&self, request: Request<UserLogin>) -> Result<Response<User>, Status> {
        let user = request.into_inner();
        let collection = self.db.collection::<UserModel>("User");
        let filter = doc! {"login": user.login};

        // Getting the user from the database
        if let Some(result) = collection.find_one(filter).await.map_err(|err| ArtieError::MongoDBError(err.into()))? {
            
            let is_valid = validate_password(&user.password, &result.get_passw_only(), &result.get_salt());
            if !is_valid {
                return Err(Status::unauthenticated("Invalid login and password"));
            }

            Ok(Response::new(User{
                id: result.id.to_string(),
                login: result.login,
                password: result.password,
                first_name: result.first_name.unwrap_or("".to_string()),
                last_name: result.last_name.unwrap_or("".to_string()),
                email: result.email,
                institution_id: result.institution_id.unwrap_or("".to_string()),
                active: result.active,
                role: result.role,
            }))

        } else {
            return Err(Status::not_found("User not found"));
        }
    }
}

impl ArtieUserService {
    pub fn new(db: Database) -> Self {
        ArtieUserService { db }
    }
}
