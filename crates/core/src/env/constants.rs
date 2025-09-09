use std::{path::PathBuf, sync::LazyLock};

use super::{Var, resolvers::*};

pub static COMPLISH_DATA_DIR: LazyLock<Var<PathBuf>> =
  LazyLock::new(|| Var::new("COMPLISH_DATA_DIR", resolve_absolute_path));
