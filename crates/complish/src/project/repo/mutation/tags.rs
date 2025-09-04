use eyre::Result;

use super::constructor::*;
use crate::{Project, Tag, tag::Repo as TagRepo};

impl<'a> Repo<'a> {
  pub fn add_tag(&self, project: Project, label: impl Into<String>) -> Result<Project> {
    let label = label.into();
    let tag_repo = TagRepo::new(self.connection);

    let tag = if let Ok(tag) = tag_repo.by_label(&label) {
      tag
    } else {
      let new_tag = Tag::new(label);
      tag_repo.create(new_tag)?
    };

    let mut statement = self.connection.prepare(UPSERT_TAG_SQL)?;
    statement.execute([project.id(), tag.id()])?;

    self.by_pk(project.id())
  }

  pub fn remove_tag(&self, project: Project, tag_id: impl Into<String>) -> Result<Project> {
    let mut statement = self.connection.prepare(DELETE_TAG_SQL)?;
    statement.execute([project.id(), &tag_id.into()])?;

    self.by_pk(project.id())
  }
}

const DELETE_TAG_SQL: &str = r"
  DELETE FROM project_tags
  WHERE project_id = ?1 AND tag_id = ?2
";

const UPSERT_TAG_SQL: &str = r"
  INSERT INTO project_tags (project_id, tag_id)
  VALUES (?1, ?2)
  ON CONFLICT (project_id, tag_id) DO NOTHING
";
