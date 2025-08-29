macro_rules! impl_sql_string_traits {
  ($enum_type:ty) => {
    impl FromSql for $enum_type {
      fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
          .as_str()?
          .parse()
          .map_err(|_| FromSqlError::InvalidType)
      }
    }

    impl ToSql for $enum_type {
      fn to_sql(&self) -> SqliteResult<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_string()))
      }
    }
  };
}

pub(crate) use impl_sql_string_traits;
