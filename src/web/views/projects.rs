//!
//! Representations for project models.
//!
use maud::{html, Markup};

use crate::web::{models::projects::Index, views::html::page};

/// Returns the HTML used for `/projects`.
///
pub(crate) fn index_html(projects: &[Index]) -> Markup {
    page(
        "Projects",
        html! {
            div {
                table {
                    tr {
                        th { "ID" }
                        th { "Name" }
                        th { "Related Links" }
                    }
                    @for project in projects {
                        tr {
                            td { (project.id().id) }
                            td { (project.name()) }
                            td {
                                ul {
                                    @for related_link in project.related_links() {
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
