use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn connect() -> MysqlPool {
    init(dotenv!("DB_URL")).expect("Error")
}