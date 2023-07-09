use redis;
use redis::{Commands, Connection, ConnectionLike, RedisError};
use std::path::{Path, PathBuf};
use std::time::Duration;
use uuid::Uuid;

fn make_datasets_set_key(user_name: &String) -> String {
    return format!("users:{}:datasets", user_name);
}

fn make_dataset_hashset_key(user_name: &String, dataset_name: &String) -> String {
    return format!("users:{}:datasets:{}", user_name, dataset_name);
}

#[derive(Clone)]
pub struct RedisManager {
    redis_client: redis::Client,
}

impl RedisManager {
    pub(crate) fn new(host: String, port: u32) -> Result<RedisManager, RedisError> {
        let conn_string = format!("redis://{}/", host);
        //BUG? This should take a String
        let client = redis::Client::open(conn_string)?;

        let mut conn = client
            .get_connection_with_timeout(Duration::from_millis(500))
            .expect("Could not connect to Redis");
        if !conn.check_connection() {
            panic!("Could not connect to Redis")
        }

        return Ok(RedisManager {
            redis_client: client,
        });
    }
}

//TODO: Hardocoded timeouts
impl RedisManager {
    fn get_connection(&self) -> Result<Connection, RedisError> {
        let conn = self
            .redis_client
            .get_connection_with_timeout(Duration::from_millis(500))?;

        conn.set_read_timeout(Option::from(Duration::from_millis(500)))
            .expect("Failed to set read timeout");
        conn.set_write_timeout(Option::from(Duration::from_millis(500)))
            .expect("Failed to set write timeout");

        return Ok(conn);
    }

    pub(crate) fn new_dataset_for_user(
        &self,
        user_name: &String,
        dataset_uuid: &Uuid,
        dataset_path: &Path,
    ) -> Result<(), RedisError> {
        let mut conn = self.get_connection()?;

        //TODO: How to clean this up without panicking?
        let set_key = make_datasets_set_key(&user_name.clone());
        let uuid_string = dataset_uuid.to_string();
        conn.sadd::<String, String, ()>(set_key, uuid_string)?;

        conn.hset::<String, &str, &str, ()>(
            make_dataset_hashset_key(&user_name, &dataset_uuid.to_string()),
            "status",
            "active",
        )?;

        //TODO: Hot to make this cleaner without panicking?
        conn.hset::<String, &str, String, ()>(
            make_dataset_hashset_key(&user_name, &dataset_uuid.to_string()),
            "path",
            dataset_path.to_string_lossy().to_string(),
        )?;

        return Ok(());
    }

    //NOTE: It is the caller's responsibility to authenticate the user
    pub(crate) fn get_dataset_path(
        &self,
        user_name: &String,
        dataset_uuid: &Uuid,
    ) -> Option<Result<PathBuf, RedisError>> {
        match self.dataset_exists(user_name, dataset_uuid) {
            Ok(val) => match val {
                true => (),
                false => {
                    return None;
                }
            },
            Err(e) => {
                return Some(Err(e));
            }
        };

        let mut conn = match self.get_connection() {
            Ok(val) => val,
            Err(e) => {
                return Some(Err(e));
            }
        };

        let path: String = match conn.hget(
            make_dataset_hashset_key(user_name, &dataset_uuid.to_string()),
            "path",
        ) {
            Ok(val) => val,
            Err(e) => {
                return Some(Err(e));
            }
        };

        return Some(Ok(std::path::PathBuf::from(&path)));
    }

    fn dataset_exists(&self, user_name: &String, dataset_uuid: &Uuid) -> Result<bool, RedisError> {
        let mut conn = self.get_connection()?;

        return conn.sismember(make_datasets_set_key(user_name), &dataset_uuid.to_string());
    }

    //TODO: Need a hash set?
    pub(crate) fn list_datasets(&self, user_name: &String) -> Result<Vec<Uuid>, RedisError> {
        let mut conn = self.get_connection()?;

        return match conn.sscan(make_datasets_set_key(&user_name)) {
            Ok(val) => Ok(val
                .filter_map(|x: String| Uuid::parse_str(&x).ok())
                .collect()),
            Err(e) => return Err(e),
        };
    }

    pub(crate) fn set_dataset_size(
        &self,
        user_name: &String,
        dataset_id: Uuid,
        size: u64,
    ) -> Result<(), RedisError> {
        let mut conn = self.get_connection()?;

        conn.hset(
            make_dataset_hashset_key(user_name, &dataset_id.to_string()),
            "size",
            size,
        )?;

        return Ok(());
    }

    pub(crate) fn get_dataset_size(
        &self,
        user_name: &String,
        dataset_id: Uuid,
    ) -> Result<u64, RedisError> {
        return Ok(self.get_connection()?.hget(
            make_dataset_hashset_key(user_name, &dataset_id.to_string()),
            "size",
        )?);
    }
}
