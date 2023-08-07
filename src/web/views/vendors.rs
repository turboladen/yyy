//!
//! Representations for vendor models.
//!
use maud::{html, Markup};

use crate::web::{models::vendors::Index, views::html::page};

/// Returns the HTML used for `/vendors`.
///
pub(crate) fn index_html(vendors: &[Index]) -> Markup {
    page(
        "Vendors",
        html! {
            div {
                table {
                    tr {
                        th { "ID" }
                        th { "Name" }
                        th { "Home Page" }
                    }
                    @for vendor in vendors {
                        tr {
                            td { (vendor.id().id) }
                            td { (vendor.name()) }
                            td {
                                a target={ "_blank" } href=(vendor.home_page()) { (vendor.home_page()) };
                            }
                        }
                    }
                }
            }
        },
    )
}
