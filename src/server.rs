use super::{PirQuery, PirReply};
use libc;
use std::mem;
use std::slice;

extern "C" {
    fn new_parameters(ele_num: u32, ele_size: u32, N: u32, logt: u32, d: u32) -> *mut libc::c_void;
    fn delete_parameters(params: *mut libc::c_void);

    fn new_pir_server(params: *const libc::c_void) -> *mut libc::c_void;
    fn delete_pir_server(pir_server: *mut libc::c_void);

    fn set_galois_key(
        pir_server: *mut libc::c_void,
        galois_key: *const u8,
        key_size: u32,
        client_id: u32,
    );

    fn set_database(
        pir_server: *mut libc::c_void,
        database: *const u8,
        ele_num: u32,
        ele_size: u32,
    );

    fn update_database(
        pir_server: *mut libc::c_void,
        database: *const u8,
        ele_num: u32,
        ele_size: u32,
        ele_index: u32,
    );

    fn preprocess_db(pir_server: *mut libc::c_void);

    fn generate_reply(
        pir_server: *const libc::c_void,
        params: *const libc::c_void,
        query: *const u8,
        query_size: u32,
        query_num: u32,
        reply_size: &mut u32,
        reply_num: &mut u32,
        client_id: u32,
    ) -> *mut u8;
}

pub struct PirServer {
    server: *mut libc::c_void,
    params: *mut libc::c_void,
    ele_num: u32,
    ele_size: u32,
}

impl Drop for PirServer {
    fn drop(&mut self) {
        unsafe {
            delete_pir_server(self.server);
            delete_parameters(self.params);
        }
    }
}

impl PirServer {
    pub fn new(
        ele_num: u32,
        ele_size: u32,
        poly_degree: u32,
        log_plain_mod: u32,
        d: u32,
    ) -> PirServer {

        let params: *mut libc::c_void =
            unsafe { new_parameters(ele_num, ele_size, poly_degree, log_plain_mod, d) };

        let server_ptr: *mut libc::c_void = unsafe { new_pir_server(params) };

        PirServer {
            server: server_ptr,
            params,
            ele_num,
            ele_size,
        }
    }

    pub fn setup<T>(&mut self, collection: &[T]) {
        assert_eq!(collection.len(), self.ele_num as usize);
        assert_eq!(mem::size_of::<T>(), self.ele_size as usize);

        unsafe {
            set_database(
                self.server,
                collection.as_ptr() as *const u8,
                self.ele_num,
                self.ele_size,
            );

            preprocess_db(self.server);
        }
    }

    pub fn setup_bytes(&mut self, collection: &[u8], element_size: usize) {
        assert_eq!(collection.len() / element_size, self.ele_num as usize);
        assert_eq!(element_size, self.ele_size as usize);

        unsafe {
            set_database(
                self.server,
                collection.as_ptr() as *const u8,
                self.ele_num,
                self.ele_size,
            );

            preprocess_db(self.server);
        }
    }

    pub fn update<T>(&mut self, collection: &[T], index: usize) {
        assert_eq!(collection.len(), self.ele_num as usize);
        assert_eq!(mem::size_of::<T>(), self.ele_size as usize);
        assert!(index < collection.len());

        unsafe {
            update_database(
                self.server,
                collection.as_ptr() as *const u8,
                self.ele_num,
                self.ele_size,
                index as u32,
            );

            preprocess_db(self.server);
        }
    }

    pub fn update_bytes(&mut self, collection: &[u8], element_size: usize, index: usize) {
        assert_eq!(collection.len() / element_size, self.ele_num as usize);
        assert_eq!(element_size, self.ele_size as usize);
        assert!(index < collection.len());

        unsafe {
            update_database(
                self.server,
                collection.as_ptr() as *const u8,
                self.ele_num,
                self.ele_size,
                index as u32,
            );

            preprocess_db(self.server);
        }
    }

    pub fn set_galois_key(&mut self, key: &[u8], client_id: u32) {
        unsafe {
            set_galois_key(self.server, key.as_ptr(), key.len() as u32, client_id);
        }
    }

    #[inline]
    pub fn gen_reply(&self, query: &PirQuery, client_id: u32) -> PirReply {
        let mut reply_size: u32 = 0;
        let mut reply_num: u32 = 0;

        let reply: Vec<u8> = unsafe {
            let ptr = generate_reply(
                self.server,
                self.params,
                query.query.as_ptr(),
                query.query.len() as u32,
                query.num,
                &mut reply_size,
                &mut reply_num,
                client_id,
            );

            let ans = slice::from_raw_parts_mut(ptr as *mut u8, reply_size as usize).to_vec();
            libc::free(ptr as *mut libc::c_void);
            ans
        };

        PirReply {
            reply,
            num: reply_num,
        }
    }
}
