use std::borrow::Cow;

use rocket::request::FromParam;

pub struct PasteIdSyntax<'a> {
    syn_id: Cow<'a, str>,
}

fn valid_syn(syn: &str) -> bool {
    let mut flag = false;
    let split: Vec<&str> = syn.split('.').collect();
    if split.len() >= 2 {
        for s in split {
            if s.chars().all(char::is_alphanumeric) {
                flag = true;
            }
        }
    }

    flag
}

impl<'a> PasteIdSyntax<'a> {
    pub fn get_fname(&self) -> &str {
        self.syn_id.split('.').collect::<Vec<&str>>()[0]
    }
    pub fn get_ext(&self) -> &str {
        self.syn_id.split_once('.').unwrap().1
    }
}

impl<'a> FromParam<'a> for PasteIdSyntax<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match valid_syn(param) {
            true => Ok(PasteIdSyntax {
                syn_id: Cow::Borrowed(param),
            }),
            false => Err(param),
        }
    }
}
