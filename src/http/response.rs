use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub msg: &'static str,
}

#[allow(dead_code)]
pub enum ApiErrorE {
    WrongLogin,
    Generic,
    Internal,
    DuplicateUsername,
    Forbiden,
    InvalidToken,
    Unathorized,
    CityProjectDuplicate,
    BrandDuplicate,
    ModelDuplicate,
    ItemDuplicate,
}

impl From<ApiErrorE> for ApiError {
    fn from(value: ApiErrorE) -> Self {
        match value {
            ApiErrorE::WrongLogin => Self {
                code: "WRONG_LOGIN",
                msg: "Invalid username or password.",
            },
            ApiErrorE::Generic => Self {
                code: "GENERIC",
                msg: "An unexpected error occurred.",
            },
            ApiErrorE::Internal => Self {
                code: "INTERNAL",
                msg: "Internal server error occurred.",
            },
            ApiErrorE::DuplicateUsername => Self {
                code: "DUPLICATE_USERNAME",
                msg: "This username is allready in use.",
            },
            ApiErrorE::Forbiden => Self {
                code: "FORBIDEN",
                msg: "Unathorized but fancier",
            },
            ApiErrorE::InvalidToken => Self {
                code: "INVALID_TOKEN",
                msg: "Token is invalid or expired.",
            },
            ApiErrorE::CityProjectDuplicate => Self {
                code: "CITY_NAME_DUPLICATE",
                msg: "This allready has project named this.",
            },
            ApiErrorE::Unathorized => Self {
                code: "UNATHORIZED",
                msg: "Not enought permisions.",
            },
            ApiErrorE::BrandDuplicate => Self {
                code: "BRAND_DUPLICATE",
                msg: "This brand allready exists.",
            },
            ApiErrorE::ModelDuplicate => Self {
                code: "MODEL_DUPLICATE",
                msg: "This model allready exists.",
            },
            ApiErrorE::ItemDuplicate => Self {
                code: "ITEM_DUPLICATE",
                msg: "This item allready exists.",
            },
        }
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

#[allow(dead_code)]
impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
}

#[allow(dead_code)]
impl ApiResponse<()> {
    pub fn err() -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiErrorE::Generic.into()),
        }
    }

    pub fn error(code: ApiErrorE) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(code.into()),
        }
    }

    pub fn suc() -> Self {
        Self {
            success: true,
            error: None,
            data: None,
        }
    }
}
