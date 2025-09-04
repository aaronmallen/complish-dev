use eyre::Result;

use super::constructor::*;
use crate::{Tag, Task, TaskRelationship, TaskWorkLog};

impl<'a> Repo<'a> {
  pub(crate) fn hydrate(&self, mut task: Task) -> Result<Task> {
    task.relationships = self.relationships(task.id())?;
    task.tags = self.tags(task.id())?;
    task.work_logs = self.work_logs(task.id())?;

    Ok(task)
  }

  fn relationships(&self, task_id: impl Into<String>) -> Result<Vec<TaskRelationship>> {
    let mut statement = self.connection.prepare(RELATIONSHIPS_BY_TASK_ID_SQL)?;
    statement
      .query_map([task_id.into()], |row| TaskRelationship::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }

  fn tags(&self, task_id: impl Into<String>) -> Result<Vec<Tag>> {
    let mut statement = self.connection.prepare(TAG_BY_TASK_ID_SQL)?;
    statement
      .query_map([task_id.into()], |row| Tag::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }

  fn work_logs(&self, task_id: impl Into<String>) -> Result<Vec<TaskWorkLog>> {
    let mut statement = self.connection.prepare(WORK_LOGS_BY_TASK_ID_SQL)?;
    statement
      .query_map([task_id.into()], |row| TaskWorkLog::try_from(row))?
      .map(|r| r.map_err(Into::into))
      .collect()
  }
}

const RELATIONSHIPS_BY_TASK_ID_SQL: &str = "SELECT * FROM task_relationships WHERE source_id = ?1";

const TAG_BY_TASK_ID_SQL: &str = r"
  SELECT
    tags.*
  FROM tags
  JOIN task_tags
    ON tags.id = task_tags.tag_id
  WHERE task_tags.task_id = ?1
";

const WORK_LOGS_BY_TASK_ID_SQL: &str = "SELECT * FROM task_work_logs WHERE task_id = ?1";
