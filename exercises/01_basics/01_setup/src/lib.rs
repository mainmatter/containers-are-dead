#[link(wasm_import_module = "host")]
unsafe extern "C" {
    fn print(offset: u32, len: u32);
}

// This function is exported from the WebAssembly module!
#[unsafe(no_mangle)]
extern "C" fn it_works() -> bool {
    let str = "foo".to_string();
    unsafe {
        print(str.as_ptr() as usize as u32, str.len() as u32);
    }
    false

    // make this compile and then continue with the book to inspect the disassembly of this
}
