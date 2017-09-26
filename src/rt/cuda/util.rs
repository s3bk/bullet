pub struct ZeroString<'a> {
    string: &'a mut String
}
impl<'a> ZeroString<'a> {
    pub fn new(s: &'a mut String) -> ZeroString<'a> {
        if s.capacity() <= s.len() + 1 {
            s.reserve(1);
        }
        s.push('\0');
        ZeroString { string: s }
    }
    pub fn ptr(&self) -> *const i8 {
        self.string.as_ptr() as *const i8
    }
}
impl<'a> Drop for ZeroString<'a> {
    fn drop(&mut self) {
        self.string.pop();
    }
}
