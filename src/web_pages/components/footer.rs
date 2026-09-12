use maud::{html, Markup};
pub fn footer() -> Markup {
    html! {
        footer {
            div .footer {
                p { "KM" }
            }
        }
    }
}

