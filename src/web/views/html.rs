use maud::{html, Markup, DOCTYPE};

/// The final Markup, including `header` and `footer`.
///
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn page(title: &str, greeting_box: Markup) -> Markup {
    html! {
        // Add the header markup to the page
        (header(title))
        h1 { (title) }
        (greeting_box)
        (footer())
    }
}

/// A basic header with a dynamic `page_title`.
///
fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        link rel="stylesheet" href="https://cdn.simplecss.org/simple.min.css";
        title { (page_title) }
    }
}

/// A static footer.
fn footer() -> Markup {
    html! {
        footer {
            { "Meow" }
        }
    }
}
