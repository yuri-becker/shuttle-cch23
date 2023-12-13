use shuttle_persist::PersistInstance;
use sqlx::PgPool;

pub struct Infrastructure {
    pub postgres: PgPool,
    pub persist: PersistInstance,
}
