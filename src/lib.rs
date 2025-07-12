#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct RapidsnarkProof {
    pub proof: Vec<u8>,
    pub public: Vec<u8>,
}

pub fn rapidsnark(zkey: &[u8], witness: &[u8]) -> Result<RapidsnarkProof, anyhow::Error> {
    unsafe {
        let handle = std::ptr::null_mut();
        let mut errmsg = vec![0i8; 1024];
        if groth16_prover_create(
            handle,
            zkey.as_ptr() as *const c_void,
            zkey.len() as u64,
            errmsg.as_mut_ptr(),
            1024,
        ) == PROVER_OK as i32
        {
            let mut proof_buffer = vec![0u8; 8192];
            let mut proof_buffer_size = 0u64;
            let mut public_buffer = vec![0u8; 8192];
            let mut public_buffer_size = 0u64;
            groth16_prover_prove(
                *handle,
                witness.as_ptr() as *const c_void,
                witness.len() as u64,
                proof_buffer.as_mut_ptr() as *mut i8,
                (&mut proof_buffer_size) as *mut u64,
                public_buffer.as_mut_ptr() as *mut i8,
                (&mut public_buffer_size) as *mut u64,
                errmsg.as_mut_ptr(),
                errmsg.len() as u64,
            );
            groth16_prover_destroy(*handle);
            Ok(RapidsnarkProof {
                proof: proof_buffer[..proof_buffer_size as usize].to_vec(),
                public: proof_buffer[..public_buffer_size as usize].to_vec(),
            })
        } else {
            let err_str = CStr::from_ptr(errmsg.as_ptr()).to_str()?.to_string();
            Err(anyhow!(err_str))
        }
    }
    //groth16_prover_create(prover_object, zkey_buffer, zkey_size, error_msg, error_msg_maxsize)
}

use std::{
    ffi::{CStr, CString},
    os::{raw::c_void, unix::ffi::OsStrExt},
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
