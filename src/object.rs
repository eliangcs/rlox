use std::slice;
use std::str;

#[derive(Copy, Clone)]
pub struct StringObj {
    pub ptr: *const u8,
    pub len: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Obj {
    StringObj(StringObj),
}

impl StringObj {
    #[inline]
    pub unsafe fn as_slice(&self) -> &[u8] {
        slice::from_raw_parts(self.ptr, self.len)
    }

    #[inline]
    pub unsafe fn as_str(&self) -> &str {
        let slice = self.as_slice();
        str::from_utf8(slice).unwrap()
    }
}

impl PartialEq for StringObj {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe { self.len == rhs.len && self.as_slice() == rhs.as_slice() }
    }
}
