use rocket::{
    http::Status,
    request::Request,
    response::{Redirect, Responder, Response, Result},
};
use std::io::Cursor;
use std::time::SystemTime;

pub enum ResponseWrapper<R> {
    MetaInterfaceResponse(R),
    PrettyPasteContentResponse(R, SystemTime),
    RawPasteContentResponse(R, SystemTime),
    Redirect(Box<Redirect>),
    NotFound(String),
    ServerError(String),
}

impl<'r, 'o: 'r, R: Responder<'r, 'o>> ResponseWrapper<R> {
    pub fn meta_response(responder: R) -> Self {
        Self::MetaInterfaceResponse(responder)
    }

    pub fn pretty_paste_response(responder: R, modified: SystemTime) -> Self {
        Self::PrettyPasteContentResponse(responder, modified)
    }

    pub fn raw_paste_response(responder: R, modified: SystemTime) -> Self {
        Self::RawPasteContentResponse(responder, modified)
    }

    pub fn redirect(redirect: Redirect) -> Self {
        Self::Redirect(Box::new(redirect))
    }

    pub fn not_found(id: &str) -> Self {
        Self::NotFound(id.to_string())
    }

    pub fn server_error<S: Into<String>>(message: S) -> Self {
        Self::ServerError(message.into())
    }
}

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o>
    for ResponseWrapper<R>
{
    fn respond_to(self, request: &'r Request<'_>) -> Result<'o> {
        use ResponseWrapper::*;

        // Add global headers.
        let mut response = Response::build();
        response.raw_header("Server", crate::SERVER_VERSION);

        // Handle individual request types.
        match self {
            MetaInterfaceResponse(sup) => response
                .join(sup.respond_to(request)?)
                .raw_header("ETag", &*crate::BINARY_ETAG)
                .raw_header(
                    "Cache-Control",
                    "max-age=604800, stale-while-revalidate=86400",
                )
                .ok(),

            PrettyPasteContentResponse(sup, modified) => response
                .join(sup.respond_to(request)?)
                .raw_header("Last-Modified", http_strftime(modified))
                .raw_header(
                    "Cache-Control",
                    "max-age=604800, stale-while-revalidate=86400",
                )
                .ok(),

            RawPasteContentResponse(sup, modified) => response
                .join(sup.respond_to(request)?)
                .raw_header("Last-Modified", http_strftime(modified))
                .raw_header("Cache-Control", "max-age=604800, immutable")
                .ok(),

            Redirect(sup) => response.join(sup.respond_to(request)?).ok(),

            NotFound(s) => {
                let body = format!("Unable to find entity '{}'", s);
                response
                    .sized_body(body.len(), Cursor::new(body))
                    .status(Status::NotFound)
                    .ok()
            }

            ServerError(s) => {
                let body = format!("Server error: '{}'", s);
                response
                    .sized_body(body.len(), Cursor::new(body))
                    .status(Status::InternalServerError)
                    .ok()
            }
        }
    }
}

fn http_strftime<T: Into<time::OffsetDateTime>>(time: T) -> String {
    time.into()
        .format(&time::format_description::well_known::Rfc2822)
        .unwrap_or_else(|_| "datetime unknown".into())
}
