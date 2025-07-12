use std::{
    ffi::{CStr, CString},
    os::unix::ffi::OsStrExt,
    path::Path,
};

use anyhow::anyhow;

unsafe extern "C" {
    fn gen_proof_of_burn_witness_file(
        datfile: *const i8,
        jsonfile: *const i8,
        wtnsfile: *const i8,
        errmsg: *mut i8,
    ) -> i32;
    fn gen_spend_witness_file(
        datfile: *const i8,
        jsonfile: *const i8,
        wtnsfile: *const i8,
        errmsg: *mut i8,
    ) -> i32;
}

pub fn generate_proof_of_burn_witness_file<D: AsRef<Path>, I: AsRef<Path>, W: AsRef<Path>>(
    dat_file: D,
    input_file: I,
    witness_file: W,
) -> Result<(), anyhow::Error> {
    let mut errmsg = [0i8; 512];
    let result = unsafe {
        gen_proof_of_burn_witness_file(
            CString::new(dat_file.as_ref().as_os_str().as_bytes())?.as_ptr(),
            CString::new(input_file.as_ref().as_os_str().as_bytes())?.as_ptr(),
            CString::new(witness_file.as_ref().as_os_str().as_bytes())?.as_ptr(),
            errmsg.as_mut_ptr(),
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(anyhow!(
            "Error while generating witness! {}",
            unsafe { CStr::from_ptr(errmsg.as_ptr()) }.to_str()?
        ))
    }
}

pub fn generate_spend_witness_file<D: AsRef<Path>, I: AsRef<Path>, W: AsRef<Path>>(
    dat_file: D,
    input_file: I,
    witness_file: W,
) -> Result<(), anyhow::Error> {
    let mut errmsg = [0i8; 512];
    let result = unsafe {
        gen_spend_witness_file(
            CString::new(dat_file.as_ref().as_os_str().as_bytes())?.as_ptr(),
            CString::new(input_file.as_ref().as_os_str().as_bytes())?.as_ptr(),
            CString::new(witness_file.as_ref().as_os_str().as_bytes())?.as_ptr(),
            errmsg.as_mut_ptr(),
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(anyhow!(
            "Error while generating witness! {}",
            unsafe { CStr::from_ptr(errmsg.as_ptr()) }.to_str()?
        ))
    }
}