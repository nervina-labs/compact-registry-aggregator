use crate::error::Error;
use crate::smt::db::db::RocksDB;
use crate::smt::db::schema::Col;

pub struct CotaRocksDB {
    pub(crate) inner: RocksDB,
}

impl CotaRocksDB {
    pub fn default() -> Self {
        CotaRocksDB {
            inner: RocksDB::default().expect("RocksDB create error"),
        }
    }

    pub fn get(&self, col: Col, key: &[u8]) -> Option<Box<[u8]>> {
        self.inner
            .get(col, key)
            .expect("db operation should be ok")
            .map(|v| Box::<[u8]>::from(v.as_ref()))
    }

    pub fn insert_raw(&self, col: Col, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.inner.put(col, key, value)
    }

    pub fn delete(&self, col: Col, key: &[u8]) -> Result<(), Error> {
        self.inner.delete(col, key)
    }
}
