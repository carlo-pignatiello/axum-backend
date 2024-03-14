use entity::user_account as UserAccount;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

pub async fn get_user(db: &DatabaseConnection, username: &str) -> Result<UserAccount::Model, DbErr> {
    let users = UserAccount::Entity::find()
        .filter(UserAccount::Column::Username.contains(username))
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Error while fetching user".to_owned()));
    users
}

pub async fn insert_user(db: &DatabaseConnection, user: UserAccount::ActiveModel) -> Result<i32, DbErr> {
        let res = UserAccount::Entity::insert(user).exec(db).await?;
        Ok(res.last_insert_id)
}


