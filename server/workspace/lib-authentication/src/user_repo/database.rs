// use lib_database::Connection;

// pub struct Repo {
//     connection: Connection,
// }
//
// impl Repo {
//     pub fn new(connection: Connection) -> Self {
//         Self { connection }
//     }
// }
//
// #[async_trait]
// impl super::Interface for Repo {
//     async fn check_password(
//         &self,
//         username: &str,
//         password: &str,
//     ) -> crate::user_repo::Result<Option<User>> {
//         todo!()
//     }
//
//     async fn update_password(&self, id: UserId, password: &str) -> crate::user_repo::Result<()> {
//         todo!()
//     }
//
//     async fn create(&self, user: &CreateUser) -> crate::user_repo::Result<UserId> {
//         todo!()
//     }
//
//     async fn get(&self, id: UserId) -> crate::user_repo::Result<Option<User>> {
//         todo!()
//     }
//
//     async fn get_by_username(&self, username: &str) -> crate::user_repo::Result<Option<User>> {
//         todo!()
//     }
//
//     async fn delete(&self, id: UserId) -> crate::user_repo::Result<()> {
//         todo!()
//     }
//
//     async fn update(&self, id: UserId, user: &UpdateUser) -> crate::user_repo::Result<()> {
//         todo!()
//     }
// }
