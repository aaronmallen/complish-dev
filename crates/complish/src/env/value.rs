#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value<T, E> {
  Invalid(E),
  NotSet,
  Ok(T),
}
