use color_eyre::{Result, eyre::eyre};
use complish::TaskEstimation;

pub fn task_estimation(s: &str) -> Result<TaskEstimation> {
  s.parse::<TaskEstimation>()
    .map_err(|e| eyre!(e.to_string()))
}
