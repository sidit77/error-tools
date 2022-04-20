pub mod errors;

use std::fmt::{Debug, Display};
use errors::{EitherError, ErrorWrapper, OptionIsNoneError};

pub trait IgnoreResult {
    fn ignore(self) -> ();
}

impl<T, E> IgnoreResult for Result<T, E>  {
    fn ignore(self) -> () {
        ()
    }
}

#[cfg(log)]
pub trait LogResult {
    fn log(self, level: log::Level) -> Self;
}

#[cfg(log)]
impl<T, E: core::fmt::Display> LogResult for Result<T, E> {
    fn log(self, level: log::Level) -> Self {
        if let Err(err) = &self {
            log::log!(level, "{}", err);
        }
        self
    }
}

pub trait OptionToError<T> {
    fn err(self) -> Result<T, OptionIsNoneError>;
}

impl<T> OptionToError<T> for Option<T> {
    fn err(self) -> Result<T, OptionIsNoneError> {
        self.ok_or(OptionIsNoneError::default())
    }
}

pub trait TransposeError<T, E1, E2> {
    fn transpose_err(self) -> Result<T, EitherError<E1, E2>>;
}

impl<T, E1, E2> TransposeError<T, E1, E2> for Result<Result<T, E2>, E1> {
    fn transpose_err(self) -> Result<T, EitherError<E1, E2>> {
        match self {
            Ok(res) => match res {
                Ok(ok) => Ok(ok),
                Err(err) => Err(EitherError::Right(err))
            }
            Err(err) => Err(EitherError::Left(err))
        }
    }
}

//impl<T, E> TransposeError<T, OptionIsNoneError, E> for Option<Result<T, E>>{
//    fn transpose_err(self) -> Result<T, EitherError<OptionIsNoneError, E>> {
//        self.err().transpose_err()
//    }
//}

impl<T, E> TransposeError<T, E, OptionIsNoneError> for Result<Option<T>, E>{
    fn transpose_err(self) -> Result<T, EitherError<E, OptionIsNoneError>> {
        self.map(OptionToError::err).transpose_err()
    }
}


pub trait WrapError<T, E: Display + Debug> {
    fn wrapped(self) -> Result<T, ErrorWrapper<E>>;
}

impl<T, E: Display + Debug> WrapError<T, E> for Result<T, E> {
    fn wrapped(self) -> Result<T, ErrorWrapper<E>> {
        self.map_err(ErrorWrapper)
    }
}


#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::{IgnoreResult, OptionIsNoneError, OptionToError, TransposeError, WrapError};

    #[test]
    fn test_ignore() {
        assert_eq!(Err::<u32, u32>(23).ignore(), ());
    }

    #[test]
    fn test_option_error() {
        let mut value: Option<i32> = Some(12);
        assert_eq!(value.err(), Ok(12));
        value = None;
        assert_eq!(value.err(), Err(OptionIsNoneError));
        assert_eq!(value.err().ok(), None);
    }

    #[test]
    fn test_transpose_error() {
        let value: Result<Result<i32, i32>, i32> = Ok(Ok(12));
        assert_eq!(value.transpose_err(), Ok(12));

        let value: Result<Option<i32>, i32> = Ok(Some(12));
        assert_eq!(value.transpose_err(), Ok(12));

        //let value: Option<Result<i32, i32>> = Some(Ok(12));
        //assert_eq!(value.transpose_err(), Ok(12));
    }

    #[test]
    fn test_wrap_error() {

        fn string_err() -> Result<(), i32> {
            Err(123)
        }

        fn generic_err() -> Result<(), Box<dyn Error>> {
            string_err().wrapped()?;
            Ok(())
        }

        assert_eq!(format!("{:?}", generic_err()), "Err(123)")
    }

}
