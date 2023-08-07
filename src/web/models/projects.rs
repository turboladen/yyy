//!
//! This module handles getting/putting `projects` from/to the database.
//!
use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::Db,
    sql::{Datetime, Thing},
    Surreal,
};
use tracing::info;

use crate::{cli::import::Import, database::CreateTable};

/// This struct is solely for implementing the `CreateTable` trait.
///
pub(crate) struct Creator;

impl CreateTable for Creator {
    const QUERY: &'static str = r#"
    DEFINE TABLE projects SCHEMAFULL;

    DEFINE FIELD name ON TABLE projects TYPE string
        ASSERT $value != NONE;

    DEFINE FIELD related_links ON TABLE projects TYPE array;

    -- Assert that all elements of the `related_links` array are URLs.
    DEFINE FIELD related_links.* ON TABLE projects TYPE string
        ASSERT is::url($value);

    DEFINE FIELD created_at ON TABLE projects TYPE datetime
        VALUE $value OR time::now();
    "#;
}

/// Data for `/brands`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct Index {
    id: Thing,
    name: String,
    related_links: Vec<String>,
}

impl Index {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn related_links(&self) -> &[String] {
        self.related_links.as_ref()
    }
}

/// Data for `/projects/:id`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct Show {
    id: Thing,
    name: String,
    related_links: Vec<String>,
    created_at: Datetime,
}

impl Show {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) const fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at.0
    }

    pub(crate) fn related_links(&self) -> &[String] {
        self.related_links.as_ref()
    }
}
/// Data for reading projects from the seed file and writing to the database. The seed file
/// only contains entries with names, hence the single attribute here.
///
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Seed {
    name: String,
    related_links: Vec<String>,
}

#[async_trait]
impl Import for Seed {
    type InsertedType = Index;

    async fn import(file: &Path, db: &Surreal<Db>) -> anyhow::Result<()> {
        let seed_projects = Self::load_yaml(file).await?;

        for seed_project in seed_projects {
            info!("Creating project: {:?}", &seed_project);

            let project: Show = db.create("projects").content(seed_project).await?;
            println!(
                "Inserted project: [{} - {}] {}: {:?}",
                project.created_at(),
                project.id().id,
                project.name(),
                project.related_links()
            );
        }

        Ok(())
    }
}
