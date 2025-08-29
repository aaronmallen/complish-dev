#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvResult<T, E> {
  Invalid(E),
  NotSet,
  Ok(T),
}
