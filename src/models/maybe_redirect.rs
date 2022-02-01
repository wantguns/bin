use rocket::{
    request::Request,
    response::{Redirect, Responder, Result},
};
use rocket_dyn_templates::Template;

pub enum MaybeRedirect {
    Redirect(Box<Redirect>),
    Template(Box<Template>),
}

impl From<Redirect> for MaybeRedirect {
    fn from(other: Redirect) -> Self {
        Self::Redirect(Box::new(other))
    }
}

impl From<Template> for MaybeRedirect {
    fn from(other: Template) -> Self {
        Self::Template(Box::new(other))
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for MaybeRedirect {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'o> {
        match self {
            Self::Template(t) => t.respond_to(req),
            Self::Redirect(r) => r.respond_to(req),
        }
    }
}
