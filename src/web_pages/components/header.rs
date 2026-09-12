use maud::{Markup, html};
pub fn header() -> Markup {
    html! {
        header {
            div .site_header {
                h1 .site_title {
                    "Kittens' Mayhem"
                }

                nav .navbar {
                    a .nav_link .active href="/"  { "Home" }
                    a .nav_link href="#"  { "Rules" }
                    a .nav_link href="#"  { "Sheets" }
                    a .nav_link href="#"  { "Chars" }
                }
            }
        }
    }
}
