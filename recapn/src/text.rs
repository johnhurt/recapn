//! UTF-8 encoded data with a null terminator

use core::slice;
use core::str::Utf8Error;

use crate::list::ElementSize;
use crate::ty;
use crate::{internal::Sealed, Family};

pub type ByteCount = crate::alloc::NonZeroU29;

pub mod ptr {
    use super::ByteCount;

    use core::marker::PhantomData;
    use core::ptr::NonNull;

    #[derive(Clone, Copy)]
    pub struct Reader<'a> {
        a: PhantomData<&'a [u8]>,
        data: NonNull<u8>,
        len: ByteCount,
    }

    impl Reader<'_> {
        #[inline]
        pub const fn empty() -> Self {
            const EMPTY: &[u8] = &[0];
            Self {
                a: PhantomData,
                data: unsafe { NonNull::new_unchecked(EMPTY.as_ptr().cast_mut()) },
                len: ByteCount::MIN,
            }
        }

        #[inline]
        pub const unsafe fn new_unchecked(data: NonNull<u8>, len: ByteCount) -> Self {
            Self { a: PhantomData, data, len }
        }

        #[inline]
        pub const fn data(&self) -> NonNull<u8> {
            self.data
        }
    
        /// The length of the text blob (including the null terminator)
        #[inline]
        pub const fn len(&self) -> ByteCount {
            self.len
        }
    }

    pub struct Builder<'a> {
        a: PhantomData<&'a mut [u8]>,
        data: NonNull<u8>,
        len: ByteCount,
    }

    impl Builder<'_> {
        #[inline]
        pub unsafe fn new_unchecked(data: NonNull<u8>, len: ByteCount) -> Self {
            Self { a: PhantomData, data, len }
        }

        #[inline]
        pub const fn data(&self) -> NonNull<u8> {
            self.data
        }
    
        /// The length of the text blob (including the null terminator)
        #[inline]
        pub const fn len(&self) -> ByteCount {
            self.len
        }
    }
}

#[derive(Clone, Copy)]
pub struct Text<T = Family>(T);

pub type Reader<'a> = Text<ptr::Reader<'a>>;
pub type Builder<'a> = Text<ptr::Builder<'a>>;

impl Sealed for Text {}
impl ty::Value for Text {
    type Default = Reader<'static>;
}
impl ty::ListValue for Text {
    const ELEMENT_SIZE: ElementSize = ElementSize::Pointer;
}

impl<'a> Reader<'a> {
    #[inline]
    pub const fn empty() -> Self {
        Self(ptr::Reader::empty())
    }

    /// The length of the text (including the null terminator)
    #[inline]
    pub const fn len(&self) -> u32 {
        self.0.len().get()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 1
    }

    /// Returns the bytes of the text field without the null terminator
    #[inline]
    pub const fn as_bytes(&self) -> &'a [u8] {
        let (_, remainder) = self.as_bytes_with_nul().split_last().unwrap();
        remainder
    }

    /// Returns the bytes of the text field with the null terminator
    #[inline]
    pub const fn as_bytes_with_nul(&self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.0.data().as_ptr().cast_const(), self.len() as usize) }
    }

    #[inline]
    pub const fn as_str(&self) -> Result<&'a str, Utf8Error> {
        core::str::from_utf8(self.as_bytes())
    }

    #[inline]
    pub const unsafe fn as_str_unchecked(&self) -> &'a str {
        core::str::from_utf8_unchecked(self.as_bytes())
    }
}

impl<'a> From<ptr::Reader<'a>> for Reader<'a> {
    fn from(repr: ptr::Reader<'a>) -> Self {
        Self(repr)
    }
}

impl<'a> From<Reader<'a>> for ptr::Reader<'a> {
    fn from(value: Reader<'a>) -> Self {
        value.0
    }
}

impl<'a> AsRef<ptr::Reader<'a>> for Reader<'a> {
    #[inline]
    fn as_ref(&self) -> &ptr::Reader<'a> {
        &self.0
    }
}

impl PartialEq<Reader<'_>> for Reader<'_> {
    fn eq(&self, other: &Reader<'_>) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<str> for Reader<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Reader<'_>> for str {
    fn eq(&self, other: &Reader<'_>) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<'a> Builder<'a> {
    #[inline]
    pub fn as_reader<'b>(&'b self) -> Reader<'b> {
        Text(unsafe { ptr::Reader::new_unchecked(self.0.data(), self.0.len()) })
    }

    /// The length of the text (including the null terminator)
    #[inline]
    pub const fn len(&self) -> u32 {
        self.0.len().get()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 1
    }

    /// Returns the bytes of the text field without the null terminator
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        let (_, remainder) = self.as_bytes_with_nul().split_last().unwrap();
        remainder
    }

    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.0.data().as_ptr(), (self.len() - 1) as usize) }
    }

    /// Returns the bytes of the text field with the null terminator
    #[inline]
    pub const fn as_bytes_with_nul(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.0.data().as_ptr().cast_const(), self.len() as usize) }
    }

    #[inline]
    pub const fn as_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(self.as_bytes())
    }

    #[inline]
    pub const unsafe fn as_str_unchecked(&self) -> &str {
        core::str::from_utf8_unchecked(self.as_bytes())
    }

    #[inline]
    pub fn as_str_mut(&mut self) -> Result<&mut str, Utf8Error> {
        core::str::from_utf8_mut(self.as_bytes_mut())
    }

    #[inline]
    pub unsafe fn as_str_unchecked_mut(&mut self) -> &mut str {
        core::str::from_utf8_unchecked_mut(self.as_bytes_mut())
    }
}

impl<'a> From<ptr::Builder<'a>> for Builder<'a> {
    fn from(repr: ptr::Builder<'a>) -> Self {
        Self(repr)
    }
}

impl<'a> From<Builder<'a>> for ptr::Builder<'a> {
    fn from(value: Builder<'a>) -> Self {
        value.0
    }
}

impl<'a> AsRef<ptr::Builder<'a>> for Builder<'a> {
    #[inline]
    fn as_ref(&self) -> &ptr::Builder<'a> {
        &self.0
    }
}

impl PartialEq<Builder<'_>> for Builder<'_> {
    fn eq(&self, other: &Builder<'_>) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<str> for Builder<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Builder<'_>> for str {
    fn eq(&self, other: &Builder<'_>) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}