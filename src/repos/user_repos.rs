use entity::user_account as UserAccount;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn get_user(db: &DatabaseConnection, username: &str) -> Option<UserAccount::Model> {
    let user = UserAccount::Entity::find()
        .filter(UserAccount::Column::Username.contains(username))
        .all(db)
        .await;
    match user {
        Ok(u) => u.into_iter().next(),
        Err(e) => panic!("Error in getting all users,{:?}", e),
    }
}
