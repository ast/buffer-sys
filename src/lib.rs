extern "C" {
    pub fn doublemunlock(
        addr: *const ::std::os::raw::c_void,
        size: usize,
    ) -> ::std::os::raw::c_int;

    pub fn doublemap(size: usize) -> *mut ::std::os::raw::c_void;

    pub fn pagesize() -> ::std::os::raw::c_int;
}

#[cfg(test)]
mod tests {
    use crate::{doublemap, doublemunlock, pagesize};
    use std::slice;
    use std::mem;

    #[test]
    fn page_size() {
        let psize = unsafe { pagesize() as usize} ;
        assert_eq!(psize, 4096)
    }

    #[test]
    fn allocate() {
        let size = 4096;
        let ptr = unsafe {doublemap(size) };
        assert_eq!(ptr.is_null(), false);
        let ret = unsafe { doublemunlock(ptr, size) };
        assert_eq!(ret, 0);
    }

    #[test]
    fn slice() {
        let size = 4096;
        let ptr = unsafe { doublemap(size) as *mut std::os::raw::c_int };
        assert_eq!(ptr.is_null(), false);

        let len = size / mem::size_of::<i32>();
        // len is the number of _elements_
        let slice = unsafe { slice::from_raw_parts_mut(ptr, 2*len) };

        let i = 3;
        slice[i] = 12000;
        assert_eq!(slice[i], slice[i+len]);

        let ret = unsafe {
            doublemunlock(ptr as *const std::os::raw::c_void, size)
        };
        assert_eq!(ret, 0);
    }
}
