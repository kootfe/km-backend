use maud::Markup;

use crate::page;

pub fn index_page(user_count: usize) -> Markup {
    page!("Tost", {
        p { (user_count) " Users! Registered" }
    })
}
