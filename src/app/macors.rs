#[macro_export]
macro_rules! internal_page {
    ($title:expr, $body:tt $(, $extra:tt)?) => {
        ::maud::html! {
            (::maud::DOCTYPE)
            html lang = "en" {
                head {
                    meta charset="utf8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta name="description" content="TTRPG website for all nerds.";

                    meta property="og:type"
                        content="website";

                    meta property="og:title"
                        content=($title);

                    meta property="og:description"
                        content="TTRPG website for all nerds.";

                    link rel="stylesheet" href="/css/main.css";

                    $($extra)?;

                    title { $title };
                }

                body {
                    $body;

                    script src="/js/global.js" {};
                }
            }
        }
    };
}

#[macro_export]
macro_rules! page {
    ($title:expr, $body:tt $(, $extra:tt)?) => {
        $crate::internal_page!(
            $title,
            {
                ($crate::web_pages::components::header());

                main {
                    $body;
                };

                ($crate::web_pages::components::footer());
            }
            $(, $extra)?
        )
    };
}

#[macro_export]
macro_rules! baker {
    ($km:expr, $key:expr, $val:expr) => {
        $crate::http::cookie::bake_cookie($km, $key, $val, true)
    };
    ($km:expr, $key:expr, $val:expr, $http:expr) => {
        $crate::http::cookie::bake_cookie($km, $key, $val, $http)
    };
}
