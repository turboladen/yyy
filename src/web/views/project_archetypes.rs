//!
//! Representations for project models.
//!
use maud::{html, Markup};

use crate::web::{models::project_archetypes::Index, views::html::page};

/// Returns the HTML used for `/project_archetypes`.
///
pub(crate) fn index_html(project_archetypes: &[Index]) -> Markup {
    page(
        "Project Archetypes",
        html! {
            div {
                table {
                    tr {
                        th { "ID" }
                        th { "Name" }
                        th { "Related Links" }
                    }
                    @for project_archetype in project_archetypes {
                        tr {
                            td { (project_archetype.id().id) }
                            td { (project_archetype.name()) }
                            td {
                                ul {
                                    @for related_link in project_archetype.related_links() {
                                        li {
                                a target={ "_blank" } href=(related_link) { (related_link) };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}
