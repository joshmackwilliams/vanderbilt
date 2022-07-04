use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, num::NonZeroU8};

/// Represents a typed Id, used to index [`TiVec`]
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Id<T> {
    value: NonZeroU8,
    _phantom: PhantomData<fn(T) -> ()>,
}

// Have to add these manually instead of deriving for some weird, generic-
// related reason
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

// We use [`NonZeroU8`] to enable optimizations on `Option<Id<T>>`
impl<T> From<Id<T>> for usize {
    fn from(id: Id<T>) -> Self {
        (id.value.get() - 1) as usize
    }
}

// I know that [`From`] is supposed to be infallible, but in this case we need
// to do this in order to use `Id` to index [`TiVec`]
impl<T> From<usize> for Id<T> {
    fn from(value: usize) -> Self {
        Self {
            value: NonZeroU8::new(
                (value + 1)
                    .try_into()
                    .expect("Can't create id with value greater than 254"),
            )
            .unwrap(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    // Test the size of the Id when wrapped in an Option
    #[test]
    fn test_optional_id_size() {
        assert_eq!(1, size_of::<Option<Id<()>>>());
    }

    // Test edge cases
    #[test]
    fn test_create_zero() {
        let _ = Id::<()>::from(0);
    }

    #[test]
    fn test_create_254() {
        let _ = Id::<()>::from(254);
    }

    #[test]
    #[should_panic]
    fn test_create_255() {
        let _ = Id::<()>::from(255);
    }
}
