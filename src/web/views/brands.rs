//!
//! Representations for brand models.
//!
use maud::{html, Markup};

use crate::web::{models::brands::IndexBrand, views::html::page};

/// Returns the HTML used for `/brands`.
///
pub(crate) fn index_html(brands: &[IndexBrand]) -> Markup {
    page(
        "Brands",
        html! {
            div {
                table {
                    tr {
                        th { "ID" }
                        th { "Name" }
                    }
                    @for brand in brands {
                        tr { td {  (brand.id().id) } td {  (brand.name()) } }
                    }
                }
            }
        },
    )
}
