use eyre::{Result, eyre};

use super::constructor::*;
use crate::{Task, TaskWorkLog, TaskWorkflowStatus};

impl<'a> Repo<'a> {
  pub fn start(&self, mut task: Task) -> Result<Task> {
    match task.workflow_status() {
      TaskWorkflowStatus::Done => return Err(eyre!("Cannot start a task that is already done")),
      TaskWorkflowStatus::Blocked => return Err(eyre!("Cannot start a task that is blocked")),
      _ => {}
    }

    self.stop(task.clone())?;
    let work_log = TaskWorkLog::new(task.id());
    let mut statement = self.connection.prepare(UPSERT_SQL)?;
    statement.execute(work_log.to_sql_params())?;

    task.set_workflow_status(TaskWorkflowStatus::InProgress);
    self.update(task)
  }

  pub fn stop(&self, task: Task) -> Result<Task> {
    let mut statement = self.connection.prepare(STOP_SQL)?;
    statement.execute([task.id()])?;

    self.update(task)
  }
}

const UPSERT_SQL: &str = r"
  INSERT INTO task_work_logs (
    id,
    task_id,
    note,
    source,
    started_at,
    ended_at,
    created_at,
    updated_at
  )
  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
  ON CONFLICT (id) DO UPDATE SET
    task_id = excluded.task_id,
    note = excluded.note,
    source = excluded.source,
    started_at = excluded.started_at,
    ended_at = excluded.ended_at,
    updated_at = excluded.updated_at
";

const STOP_SQL: &str = r"
  UPDATE task_work_logs
  SET ended_at = datetime('now', 'utc')
  WHERE ended_at IS NULL
    AND task_id = ?1
";
