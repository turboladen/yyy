//!
//! This module handles getting/putting `vendors` from/to the database.
//!
use std::{collections::HashMap, path::Path};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::Db,
    sql::{Datetime, Thing, Value},
    Surreal,
};
use tracing::info;

use crate::{cli::import::Import, database::CreateTable};

use super::Measurement;

/// This struct is solely for implementing the `CreateTable` trait.
///
pub(crate) struct Creator;

impl CreateTable for Creator {
    const QUERY: &'static str = r#"
    DEFINE TABLE capacitors SCHEMAFULL;

    DEFINE FIELD description ON TABLE capacitors TYPE string
        ASSERT $value != NONE;

    DEFINE FIELD manufacturer_id ON TABLE capacitors TYPE record
        ASSERT $value != NONE;

    DEFINE FIELD manufacturer_part_number ON TABLE capacitors TYPE string;

    DEFINE FIELD category ON TABLE capacitors TYPE record
        ASSERT $value != NONE;

    ----------------
    -- capacitance
    ----------------
    DEFINE FIELD capacitance ON TABLE capacitors TYPE object
        ASSERT $value != NONE;

    DEFINE FIELD capacitance.value ON TABLE capacitors TYPE float
        ASSERT $value != NONE;

    DEFINE FIELD capacitance.unit ON TABLE capacitors TYPE string
        ASSERT $value != NONE;

    ----------------
    -- voltage_rating
    ----------------
    DEFINE FIELD voltage_rating ON TABLE capacitors TYPE object
        ASSERT $value != NONE;

    DEFINE FIELD voltage_rating.value ON TABLE capacitors TYPE integer
        ASSERT $value != NONE;

    DEFINE FIELD voltage_rating.unit ON TABLE capacitors TYPE string
        ASSERT $value != NONE;

    DEFINE FIELD termination_style ON TABLE capacitors TYPE record
        ASSERT $value != NONE;

    ----------------
    -- tolerance
    ----------------
    DEFINE FIELD tolerance ON TABLE capacitors TYPE object;

    DEFINE FIELD tolerance.positive ON TABLE capacitors TYPE integer
        ASSERT $value != NONE;

    DEFINE FIELD tolerance.negative ON TABLE capacitors TYPE integer
        ASSERT $value != NONE;

    DEFINE FIELD created_at ON TABLE capacitors TYPE datetime
        VALUE $value OR time::now();
    "#;
}

/// Data for `/capacitors`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct Index {
    id: Thing,
    description: String,
    manufacturer_id: u16,
    manufacturer_part_number: String,
    category_id: u8,
    capacitance: Measurement<f32>,
    voltage_rating: Measurement<u16>,
    termination_style_id: u8,
    tolerance: Option<Tolerance>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Tolerance {
    positive: u8,
    negative: u8,
}

impl Index {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }
}

/// Data for `/capacitors/:id`.
///
#[derive(Debug, Deserialize)]
pub(crate) struct Show {
    id: Thing,
    description: String,
    manufacturer_id: u16,
    manufacturer_part_number: String,
    category_id: u8,
    capacitance: Measurement<f32>,
    voltage_rating: Measurement<u16>,
    termination_style_id: u8,
    tolerance: Option<Tolerance>,
    created_at: Datetime,
}

impl Show {
    pub(crate) const fn id(&self) -> &Thing {
        &self.id
    }

    pub(crate) fn created_at(&self) -> &Datetime {
        &self.created_at
    }

    pub(crate) fn description(&self) -> &str {
        self.description.as_ref()
    }
}

/// Data for reading vendors from the seed file and writing to the database. The seed file
/// only contains entries with names, hence the single attribute here.
///
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Seed {
    id: u16,
    description: String,
    manufacturer_id: u16,
    manufacturer_part_number: String,
    category_id: u8,
    capacitance: Measurement<f32>,
    voltage_rating: Measurement<u16>,
    termination_style_id: u8,
    tolerance: Option<Tolerance>,
}

#[async_trait]
impl Import for Seed {
    type InsertedType = Index;

    async fn import(file: &Path, db: &Surreal<Db>) -> anyhow::Result<()> {
        let seed_capacitors = Self::load_yaml(file).await?;

        info!("Creating capacitors...");

        let capacitor: Show = db.create("capacitors").content(&seed_capacitors[0]).await?;
        dbg!(capacitor);
        // let response = db
        //     .query("INSERT INTO capacitors $data")
        //     .bind(("data", seed_capacitors))
        //     .await?;

        // let mut response = response.check()?;
        // let capacitors: Vec<Show> = response.take(0)?;

        // for capacitor in capacitors {
        //     println!(
        //         "Inserted capacitor: [{} - {}] {}",
        //         capacitor.created_at(),
        //         capacitor.id().id,
        //         capacitor.description(),
        //     );
        // }

        Ok(())
    }
}
