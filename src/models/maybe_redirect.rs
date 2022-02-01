use rocket::{response::Redirect, Responder};
use rocket_dyn_templates::Template;

#[derive(Responder)]
pub enum MaybeRedirect {
    Redirect(Redirect),
    Template(Template),
}

impl From<Redirect> for MaybeRedirect {
    fn from(other: Redirect) -> Self {
        Self::Redirect(other)
    }
}

impl From<Template> for MaybeRedirect {
    fn from(other: Template) -> Self {
        Self::Template(other)
    }
}
