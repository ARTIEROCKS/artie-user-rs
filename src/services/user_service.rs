use futures::TryStreamExt;
use tonic::{Request, Response, Status};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use crate::{config::pb::UserList, models::user_model::UserModel};
use crate::config::pb::user_service_server::UserService;
use crate::config::pb::{User, UserId, UserLogin};
use crate::services::security_service::validate_password;
use crate::config::error::ArtieError;

use super::security_service::generate_password_hash;

pub struct ArtieUserService {
    pub db: Database,
}

#[tonic::async_trait]
impl UserService for ArtieUserService {
    /**
     * Add a new user
     */
    async fn add_user(&self, request: Request<User>) -> Result<Response<UserId>, Status> {
        let user = request.into_inner();
        let collection = self.db.collection::<UserModel>("User");

        let (salt, password) = generate_password_hash(user.password.as_str());

        let mut new_user = UserModel {
            id: ObjectId::new(),
            login: user.login,
            password: "".to_string(),
            first_name: Some(user.first_name),
            last_name:  Some(user.last_name),
            email: user.email,
            institution_id:  Some(user.institution_id),
            active: user.active,
            role: user.role,
        };

        new_user.set_password(&password, &salt);

        collection.insert_one(&new_user).await.map_err(|err| ArtieError::MongoDBError(err.into()))?;
        Ok(Response::new(UserId{id: new_user.id.to_string()}))
    }

    /**
     * Update a user
     */
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

    /**
     * Get user by id
     */
    async fn get_all_users(&self, _: Request<()>) -> Result<Response<UserList>, Status> {
        let collection = self.db.collection("User");
        let mut cursor = collection.find(doc! {}).await.unwrap();

        let mut users = vec![];
        while let Some(user_doc) = cursor.try_next().await.unwrap() {
            let user: UserModel = mongodb::bson::from_document(user_doc).unwrap();
            users.push(User {
                id: user.id.to_string(),
                login: user.login,
                password: user.password,
                first_name: user.first_name.unwrap_or_default(),
                last_name: user.last_name.unwrap_or_default(),
                email: user.email,
                institution_id: user.institution_id.unwrap_or_default(),
                active: user.active,
                role: user.role,
            });
        }

        Ok(Response::new(UserList { users }))
    }

    /**
     * Get all users by institution id
     */
    async fn get_users_by_institution_id(&self, request: Request<UserId>) -> Result<Response<UserList>, Status> {
        let institution_id = request.into_inner().id;
        let collection = self.db.collection("User");

        let filter = doc! { "institutionId": institution_id };
        let mut cursor = collection.find(filter).await.unwrap();

        let mut users = vec![];
        while let Some(user_doc) = cursor.try_next().await.unwrap() {
            let user: UserModel = mongodb::bson::from_document(user_doc).unwrap();
            users.push(User {
                id: user.id.to_string(),
                login: user.login,
                password: user.password,
                first_name: user.first_name.unwrap_or_default(),
                last_name: user.last_name.unwrap_or_default(),
                email: user.email,
                institution_id: user.institution_id.unwrap_or_default(),
                active: user.active,
                role: user.role,
            });
        }

        Ok(Response::new(UserList { users }))
    }

    /**
     * Get user by id
     */
    async fn get_user_by_id(&self, request: Request<UserId>) -> Result<Response<User>, Status> {
        let id = ObjectId::parse_str(request.into_inner().id).unwrap();
        let collection = self.db.collection("User");

        let filter = doc! { "_id": id };
        if let Some(user_doc) = collection.find_one(filter).await.unwrap() {
            let user: UserModel = mongodb::bson::from_document(user_doc).unwrap();
            Ok(Response::new(User {
                id: user.id.to_string(),
                login: user.login,
                password: user.password,
                first_name: user.first_name.unwrap_or_default(),
                last_name: user.last_name.unwrap_or_default(),
                email: user.email,
                institution_id: user.institution_id.unwrap_or_default(),
                active: user.active,
                role: user.role,
            }))
        } else {
            Err(Status::not_found("User not found"))
        }
    }

    /**
     * Delete a user
     */
    async fn delete_user(&self, request: Request<UserId>) -> Result<Response<()>, Status> {
        let id = ObjectId::parse_str(request.into_inner().id).unwrap();
        let collection: mongodb::Collection<bson::Document>  = self.db.collection("User");
        
        let filter = doc! { "_id": id };
        collection.delete_one(filter).await.unwrap();

        Ok(Response::new(()))
    }

    /**
     * Update a user
     */
    async fn update_user(&self, request: Request<User>) -> Result<Response<()>, Status> {
        let user = request.into_inner();
        let collection: mongodb::Collection<bson::Document> = self.db.collection("User");

        let filter = doc! { "_id": ObjectId::parse_str(user.id.clone()).unwrap() };
        let update = doc! {
            "$set": {
                "login": user.login,
                "password": user.password,
                "first_name": user.first_name,
                "last_name": user.last_name,
                "email": user.email,
                "institution_id": user.institution_id,
                "active": user.active,
                "role": user.role,
            }
        };

        collection.update_one(filter, update).await.unwrap();

        Ok(Response::new(()))
    }
}

impl ArtieUserService {
    pub fn new(db: Database) -> Self {
        ArtieUserService { db }
    }
}
