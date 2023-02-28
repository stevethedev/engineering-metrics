use sea_orm::prelude::*;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::entities::prelude::UserCredentials;
use crate::entities::user_credentials::{ActiveModel, Column, Model};
use crate::Result;

pub struct Controller {
    connection: crate::Connection,
}

#[derive(Default)]
pub struct Filter {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub is_enabled: Option<bool>,
}

impl Filter {
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = Some(is_enabled);
        self
    }
}

#[derive(Default)]
pub struct Write {
    pub username: Option<String>,
    pub password_hash: Option<String>,
    pub is_enabled: Option<bool>,
}

impl Write {
    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn password_hash(mut self, password_hash: String) -> Self {
        self.password_hash = Some(password_hash);
        self
    }

    pub fn enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = Some(is_enabled);
        self
    }
}

impl Controller {
    pub fn new(connection: crate::Connection) -> Self {
        Self { connection }
    }

    pub async fn create(&self, write: Write) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let user_credentials = {
            let mut tmp = get_write(write);
            tmp.id = Set(id);
            tmp
        };

        user_credentials.insert(self.connection.as_ref()).await?;

        Ok(id)
    }

    pub async fn read(&self, filter: Filter) -> Result<Option<Model>> {
        let model = get_filter(UserCredentials::find(), filter)
            .one(self.connection.as_ref())
            .await?;

        Ok(model)
    }

    pub async fn read_many(&self, filter: Filter) -> Result<Vec<Model>> {
        let models = get_filter(UserCredentials::find(), filter)
            .all(self.connection.as_ref())
            .await?;

        Ok(models)
    }

    pub async fn update(&self, id: Uuid, write: Write) -> Result<Model> {
        let write = {
            let mut tmp = get_write(write);
            tmp.id = Set(id);
            tmp
        };
        let model = UserCredentials::update(write)
            .filter(Column::Id.eq(id))
            .exec(self.connection.as_ref())
            .await?;

        Ok(model)
    }

    pub async fn update_many(&self, filter: Filter, write: Write) -> Result<u64> {
        let write = get_write(write);

        let result = get_filter(UserCredentials::update_many(), filter)
            .set(write)
            .exec(self.connection.as_ref())
            .await?;

        Ok(result.rows_affected)
    }

    pub async fn delete(&self, id: Uuid) -> Result<u64> {
        let result = UserCredentials::delete_by_id(id)
            .exec(self.connection.as_ref())
            .await?;

        Ok(result.rows_affected)
    }

    pub async fn delete_many(&self, filter: Filter) -> Result<u64> {
        let result = get_filter(UserCredentials::delete_many(), filter)
            .exec(self.connection.as_ref())
            .await?;

        Ok(result.rows_affected)
    }
}

fn get_filter<T: QueryFilter>(mut query: T, filter: Filter) -> T {
    let Filter {
        id,
        username,
        is_enabled,
    } = filter;

    if let Some(id) = id {
        query = query.filter(Column::Id.eq(id));
    }

    if let Some(username) = username {
        query = query.filter(Column::Username.eq(username));
    }

    if let Some(is_enabled) = is_enabled {
        query = query.filter(Column::IsEnabled.eq(is_enabled));
    }

    query
}

fn get_write(write: Write) -> ActiveModel {
    ActiveModel {
        username: write.username.map(Set).unwrap_or(NotSet),
        password_hash: write.password_hash.map(Set).unwrap_or(NotSet),
        is_enabled: write.is_enabled.map(Set).unwrap_or(NotSet),
        ..Default::default()
    }
}
