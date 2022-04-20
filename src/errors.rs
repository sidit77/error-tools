use core::fmt::{Display, Formatter, Debug};
use std::error::Error;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct OptionIsNoneError;

impl Display for OptionIsNoneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "an Option<T> was `None`")
    }
}

impl Error for OptionIsNoneError { }

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EitherError<L, R> {
    Left(L),
    Right(R)
}

impl<L: Display, R: Display> Display for EitherError<L, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EitherError::Left(err) => write!(f, "{}", err),
            EitherError::Right(err) => write!(f, "{}", err)
        }
    }
}

impl<L: 'static + Error, R: 'static + Error> Error for EitherError<L, R> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            EitherError::Left(err) => err,
            EitherError::Right(err) => err
        })
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ErrorWrapper<T>(pub T);

impl<T> Deref for ErrorWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ErrorWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> Debug for ErrorWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Display> Display for ErrorWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Display + Debug> Error for ErrorWrapper<T> {

}