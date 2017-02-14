use std::fmt;

use super::{Descriptor, Encoding, PointerEncoding, Never};

pub struct Pointer<T>(T) where T: Encoding;

impl<T> Pointer<T> where T: Encoding {
    pub fn new(target: T) -> Pointer<T> {
        Pointer(target)
    }
}

impl<T> Encoding for Pointer<T> where T: Encoding {
    type Pointer = Self;
    type Struct = Never;

    fn descriptor(&self) -> Descriptor<Self, Never> {
        Descriptor::Pointer(self)
    }

    fn eq_encoding<E: ?Sized + Encoding>(&self, other: &E) -> bool {
        if let Descriptor::Pointer(p) = other.descriptor() {
            self.0.eq_encoding(p.target())
        } else {
            false
        }
    }
}

impl<T> PointerEncoding for Pointer<T> where T: Encoding {
    type Target = T;

    fn target(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Display for Pointer<T> where T: Encoding {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "^{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use encoding::Primitive;
    use super::*;

    #[test]
    fn test_pointer_display() {
        let e = Pointer::new(Primitive::Int);
        assert_eq!(e.to_string(), "^i");
    }

    #[test]
    fn test_eq_encoding() {
        let i = Primitive::Int;

        let p = Pointer::new(i);
        assert!(p.eq_encoding(&p));
        assert!(!p.eq_encoding(&i));
    }
}
