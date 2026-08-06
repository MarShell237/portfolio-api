use actix_web::{
    HttpRequest, HttpResponse, Responder,
    http::{StatusCode, header::ContentType},
};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(serialize_with = "serialize_status_code")]
    pub status: StatusCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
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
            message: message.into(),
            data,
            status: StatusCode::OK,
        }
    }

    pub fn success(status: StatusCode, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data,
            status,
        }
    }

    pub fn error(status: StatusCode, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data,
            status,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.status)
            .content_type(ContentType::json())
            .json(self)
    }
}
