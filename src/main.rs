use std::ffi::c_void;
use std::io::{self, Write};
use std::mem::size_of;

use windows::Win32::Foundation::{FALSE, HANDLE};
use windows::Win32::System::Threading::OpenProcess;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;

fn read_input_address() -> u64 {
    io::stdout().flush().unwrap();
    let mut address_target = String::new();
    println!("enter address:");
    io::stdin().read_line(&mut address_target).unwrap();
    let address = address_target.trim().trim_start_matches("0x");
    u64::from_str_radix(address, 16).unwrap()
}

fn read_input_write_params() -> u64{
    io::stdout().flush().unwrap();
    let mut int_to_write = String::new();
    println!("enter int to write:");
    io::stdin().read_line(&mut int_to_write).unwrap();
    int_to_write.trim().parse::<u64>().unwrap()
}


fn write_to_exe<T>(handle_process: HANDLE, address: u64, int_to_write: T) {
    unsafe{
        WriteProcessMemory(
            handle_process,
            address as *const c_void,
            &int_to_write as *const T as *const c_void,
            size_of::<T>() as usize,
            None,
        );
    }
}


fn main() {
    io::stdout().flush().unwrap();
    let mut pid_target = String::new();
    println!("enter pid:");
    io::stdin().read_line(&mut pid_target).unwrap();
    let pid = pid_target.trim().parse::<u32>().unwrap();

    let hand_process = unsafe {
        OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid)
    };

    match hand_process {
        Ok(handle) => {
            let address = read_input_address();
            let int_to_write = read_input_write_params();
            write_to_exe::<u64>(handle, address, int_to_write);
        }
        Err(_) => {
            println!("error");
        }
        
    }

}
