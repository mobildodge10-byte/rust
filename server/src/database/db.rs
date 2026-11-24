use sea_orm::{DatabaseConnection, DbErr};

pub struct DatabaseConn {
    pub connection:DatabaseConnection
}

// Ini jangan diubah ya kawan, ga ada bug di sini, suwer
// Cuman kalau diubah terus ada bug ya...*laugh in rust

pub async fn create_database_connection() -> Result<DatabaseConn, DbErr> {
    let url = "mysql://root:@localhost:3306/rhodev_island_logistic";
    let db = sea_orm::Database::connect(url).await;
    
    match db {
        Ok(connection) => Ok(DatabaseConn { connection }),
        Err(error) => Err(error),
    }
}