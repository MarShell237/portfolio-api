use actix_web::{
    HttpRequest, HttpResponse, Responder,
    http::{StatusCode, header::ContentType},
};
use serde::{Serialize, Serializer};

use crate::helpers::pagination_meta::PaginationMeta;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(serialize_with = "serialize_status_code")]
    pub status: StatusCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationMeta>,
}

fn serialize_status_code<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status.as_u16())
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            success: true,
            status: StatusCode::OK,
            message: message.into(),
            data,
            pagination: None,
        }
    }

    pub fn ok_with_pagination(
        message: impl Into<String>,
        data: T,
        pagination: PaginationMeta,
    ) -> Self {
        Self {
            success: true,
            status: StatusCode::OK,
            message: message.into(),
            data: Some(data),
            pagination: Some(pagination),
        }
    }

    // pub fn created(message: impl Into<String>, data: Option<T>) -> Self {
    //     Self {
    //         success: true,
    //         status: StatusCode::CREATED,
    //         message: message.into(),
    //         data,
    //         pagination: None,
    //     }
    // }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            success: false,
            status: StatusCode::NOT_FOUND,
            message: message.into(),
            data: None,
            pagination: None,
        }
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
            data: None,
            pagination: None,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            success: false,
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
            data: None,
            pagination: None,
        }
    }

    // pub fn success(status: StatusCode, message: impl Into<String>, data: Option<T>) -> Self {
    //     Self {
    //         success: true,
    //         message: message.into(),
    //         data,
    //         status,
    //     }
    // }

    // pub fn error(status: StatusCode, message: impl Into<String>, data: Option<T>) -> Self {
    //     Self {
    //         success: false,
    //         message: message.into(),
    //         data,
    //         status,
    //     }
    // }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.status)
            .content_type(ContentType::json())
            .json(self)
    }
}
