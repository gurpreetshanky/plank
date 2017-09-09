use std::ops;
pub use errors::position::{Position, Span};


#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone)]
pub struct Spanned<T> {
    value: T,
    span: Span,
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Spanned<T> {
        Spanned {
            value,
            span,
        }
    }

    pub fn value(this: &Self) -> &T {
        &this.value
    }

    pub fn value_mut(this: &mut Self) -> &mut T {
        &mut this.value
    }

    pub fn into_value(this: Self) -> T {
        this.value
    }

    pub fn span(this: &Self) -> Span {
        this.span
    }

    pub fn map<U, F: FnOnce(T) -> U>(this: Self, f: F) -> Spanned<U> {
        Spanned {
            value: f(this.value),
            span: this.span,
        }
    }
}

impl<T> ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> ops::DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
