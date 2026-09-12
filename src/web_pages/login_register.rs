use maud::Markup;

use crate::page;

pub fn login_page() -> Markup {
    page!("Login", {
        form method="POST" action="/login" {
            input type="text" name="username";
            input type="password" name="password";
            button type="submit" { "Login" };
        }
    })
}

pub fn register_page() -> Markup {
    page!("Register", {
        form method="POST" action="/register" {
            input type="text" name="username";
            input type="password" name="password";
            input type="password" name="password2";
            button type="submit" { "Register" };
        }
    })
}
