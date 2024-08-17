use std::ffi::{c_char, c_int, c_void, CStr, CString};

use crate::error::RelayError;



extern "C" {
    fn nac_init(
        certificate_bytes: *const c_void, 
        certificate_len: usize, 
        out_ctx: *mut u64, 
        out_session_request: *mut *mut c_void, 
        session_requestCnt: *mut usize
    ) -> c_int;

    fn nac_key_establishment(
        val_ctx: u64, 
        session_response: *const c_void, 
        session_response_len: usize
    ) -> c_int;

    fn nac_sign(
        val_ctx: u64, 
        data: *const c_void, 
        data_len: usize, 
        out_signature: *mut *mut c_void, 
        out_sig_len: *mut usize
    ) -> c_int;

    fn mig_deallocate(
        data: *mut c_void,
        data_len: usize,
    );
    
    fn mg_copy_answer(property: *const c_char) -> *mut c_char;
}

pub fn nac_init_rs(cert: &[u8], output: &mut Vec<u8>) -> Result<u64, RelayError> {
    unsafe {
        let mut out_req: *mut c_void = std::ptr::null_mut();
        let mut out_req_cnt: usize = 0;
        let mut ctx_out: u64 = 0;
        let resp = nac_init(cert.as_ptr() as *const c_void, cert.len(), &mut ctx_out, &mut out_req, &mut out_req_cnt);
        if resp == 0 {
            output.extend_from_slice(std::slice::from_raw_parts(out_req as *mut u8, out_req_cnt));
            mig_deallocate(out_req, out_req_cnt);
            Ok(ctx_out)
        } else {
            Err(RelayError::NacError(resp as u64))
        }
    }
}

pub fn nac_key_establishment_rs(ctx: u64, response: &[u8]) -> Result<(), RelayError> {
    unsafe {
        let resp = nac_key_establishment(ctx, response.as_ptr() as *const c_void, response.len());
        if resp == 0 {
            Ok(())
        } else {
            Err(RelayError::NacError(resp as u64))
        }
    }
}

pub fn nac_sign_rs(ctx: u64, data: &[u8]) -> Result<Vec<u8>, RelayError> {
    unsafe {
        let mut out_sig: *mut c_void = std::ptr::null_mut();
        let mut out_sig_cnt: usize = 0;
        let resp = nac_sign(ctx, data.as_ptr() as *const c_void, data.len(), &mut out_sig, &mut out_sig_cnt);
        if resp == 0 {
            let vec = std::slice::from_raw_parts(out_sig as *mut u8, out_sig_cnt).to_vec();
            mig_deallocate(out_sig, out_sig_cnt);
            Ok(vec)
        } else {
            Err(RelayError::NacError(resp as u64))
        }
    }
}

pub fn mg_copy_answer_rs(item: &str) -> String {
    unsafe {
        let c_str = CString::new(item).unwrap();
        let answer = mg_copy_answer(c_str.as_ptr());
        let c_str = CStr::from_ptr(answer).to_str().unwrap().to_string();
        libc::free(answer as *mut c_void);
        c_str
    }
}
