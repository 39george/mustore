type ManagedRedisPool = deadpool::managed::Pool<
    deadpool_redis::Manager,
    deadpool_redis::Connection,
>;

#[derive(Clone)]
pub struct RedisPool(ManagedRedisPool);

impl RedisPool {
    pub fn new(inner: ManagedRedisPool) -> Self {
        RedisPool(inner)
    }
}

impl std::ops::Deref for RedisPool {
    type Target = ManagedRedisPool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RedisPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for RedisPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDIS POOL DEBUG NOT IMPLEMENTED]")
    }
}
