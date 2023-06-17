use serde::{de::DeserializeOwned, Serialize};

pub trait FromDb {
    type Output;
    #[allow(clippy::wrong_self_convention)]
    fn from_db(self) -> Self::Output;
}

impl<T: FromDb> FromDb for Vec<T> where T: Send + Serialize + DeserializeOwned {
    type Output = Vec<T::Output>;
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    fn from_db(self) -> Self::Output {
        self.into_iter().map(FromDb::from_db).collect()
    }
}

impl<T: FromDb> FromDb for Option<T> {
    type Output = Option<T::Output>;
    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    fn from_db(self) -> Self::Output {
        self.map(crate::FromDb::from_db)
    }
}
