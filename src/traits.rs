use leptos::{Errors, view, View};
use leptos::error::Error;
use crate::error_template::ErrorTemplate;

pub trait MakeErrorViewTrait {
    fn make_view<E: Into<Error>>(error: E) -> View;
}

impl MakeErrorViewTrait for Errors {
    fn make_view<E: Into<Error>>(error: E) -> View {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(error);
        view! {
            <ErrorTemplate outside_errors/>
        }
    }
}
