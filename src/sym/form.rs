#![doc = include_str!("form.md")]

#[doc = include_str!("form.md")]
#[derive(Debug, Clone)]
pub enum Form {
    /// A combination of nomials.
    Polynomial {},
}

impl std::fmt::Display for Form {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for Form {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Form::Polynomial {}, Form::Polynomial {}) => todo!(),
            _ => false,
        }
    }
}
