use actix_passport::prelude::*;
use async_trait::async_trait;
use std::collections::HashMap;


use entity::{prelude::User, users};
use sea_orm::{
    DatabaseConnection, Set,
    ColumnTrait, EntityTrait, 
    QueryFilter, ActiveModelTrait
};

#[derive(Clone)]
pub struct SeaOrmUserStore {
    pub db: DatabaseConnection,
}

impl SeaOrmUserStore {
    pub fn new(db: DatabaseConnection) -> Self {
        SeaOrmUserStore { db }
    }
}

#[async_trait]
impl UserStore for SeaOrmUserStore {
    async fn find_by_id(&self, id: &str) -> AuthResult<Option<AuthUser>> {
        let id = id.parse::<i32>().unwrap();

        if let Ok(Some(user)) = User::find_by_id(id).one(&self.db).await {
            return Ok(Some(
                    AuthUser {
                        id: user.id.to_string(),
                        username: Some(user.name),
                        display_name: None,
                        email: Some(user.email),
                        avatar_url: Some(user.avatar),
                        created_at: user.created_at.into(),
                        last_login: None,
                        metadata: HashMap::new(),
                    }
            ))
        }

        Ok(None)
    }
     
    async fn find_by_email(&self, email: &str) -> AuthResult<Option<AuthUser>> {
        if let Ok(Some(user)) = User::find()
        .filter(users::Column::Email.eq(email))
        .one(&self.db)
        .await {
            return Ok(Some(
                    AuthUser {
                        id: user.id.to_string(),
                        username: Some(user.name),
                        display_name: None,
                        email: Some(user.email),
                        avatar_url: Some(user.avatar),
                        created_at: user.created_at.into(),
                        last_login: None,
                        metadata: HashMap::new(),
                    }
                    ))
        }

        Ok(None)
    }
     
    async fn find_by_username(&self, username: &str) -> AuthResult<Option<AuthUser>> {
        if let Ok(Some(user)) = User::find()
        .filter(users::Column::Name.eq(username))
        .one(&self.db)
        .await {
            return Ok(Some(
                    AuthUser {
                        id: user.id.to_string(),
                        username: Some(user.name),
                        display_name: None,
                        email: Some(user.email),
                        avatar_url: Some(user.avatar),
                        created_at: user.created_at.into(),
                        last_login: None,
                        metadata: HashMap::new(),
                    }
                    ))
        }

        Ok(None)
    }
     
    async fn create_user(&self, user: AuthUser) -> AuthResult<AuthUser> {
        let user_model = users::ActiveModel {
                        name: Set(user.username.unwrap()),
                        email: Set(user.email.unwrap()),
                        avatar: Set(user.avatar_url.unwrap()),
                        ..Default::default()
                    };

        let user = user_model.insert(&self.db).await.unwrap();

        Ok(AuthUser {
            id: user.id.to_string(),
            username: Some(user.name),
            display_name: None,
            email: Some(user.email),
            avatar_url: Some(user.avatar),
            created_at: user.created_at.into(),
            last_login: None,
            metadata: HashMap::new(),
        })
    }
     
    async fn update_user(&self, user: AuthUser) -> AuthResult<AuthUser> {
        let id = user.id.parse::<i32>().unwrap();

        let user_model = User::find_by_id(id).one(&self.db).await.unwrap();

        let mut user_model: users::ActiveModel = user_model.unwrap().into();
        user_model.name = Set(user.username.unwrap());
        user_model.avatar = Set(user.avatar_url.unwrap());

        let user = user_model.update(&self.db).await.unwrap();

        Ok(
            AuthUser {
                id: user.id.to_string(),
                username: Some(user.name),
                display_name: None,
                email: Some(user.email),
                avatar_url: Some(user.avatar),
                created_at: user.created_at.into(),
                last_login: None,
                metadata: HashMap::new(),
        })
    }
     
    async fn delete_user(&self, _id: &str) -> AuthResult<()> {
        Ok(())
    }
}