macro_rules! impl_repo_constructor {
  () => {
    pub struct Repo<'a> {
      pub(super) connection: &'a rusqlite::Connection,
    }

    impl<'a> Repo<'a> {
      pub fn new(connection: &'a rusqlite::Connection) -> Self {
        Self {
          connection,
        }
      }
    }
  };
}

pub(crate) use impl_repo_constructor;
