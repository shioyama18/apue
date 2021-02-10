use num::Num;
use std::io::{Error, Result};

pub trait NumericResult
where
    Self: Sized + Copy + PartialOrd + Num,
{
    fn positive(&self) -> Result<Self> {
        if *self < Self::zero() {
            Err(Error::last_os_error())
        } else if *self == Self::zero() {
            Err(Error::from_raw_os_error(0))
        } else {
            Ok(*self)
        }
    }

    fn non_negative(&self) -> Result<Self> {
        if *self < Self::zero() {
            Err(Error::last_os_error())
        } else {
            Ok(*self)
        }
    }
}

impl NumericResult for i32 {}
impl NumericResult for isize {}

pub trait PointerResult<T> {
    fn not_null(&self) -> Result<T>;
}

impl<T> PointerResult<*mut T> for *mut T {
    fn not_null(&self) -> Result<*mut T> {
        if self.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(*self)
        }
    }
}
