macro_rules! impl_string_sql_traits {
  ($type:ty) => {
    impl diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for $type {
      fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>,
      ) -> diesel::deserialize::Result<Self> {
        let s = <String as diesel::deserialize::FromSql<
          diesel::sql_types::Text,
          diesel::sqlite::Sqlite,
        >>::from_sql(bytes)?;
        Self::from_str(&s).map_err(Into::into)
      }
    }

    impl diesel::serialize::ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite> for $type {
      fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
      ) -> diesel::serialize::Result {
        out.set_value(self.to_string());
        Ok(diesel::serialize::IsNull::No)
      }
    }
  };
}

pub(crate) use impl_string_sql_traits;
