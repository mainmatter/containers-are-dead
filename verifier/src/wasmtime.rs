use std::path::Path;
use std::slice;

use wasmtime::component::{Component, Linker};
use wasmtime::{Caller, Engine, Instance, Linker as ModuleLinker, Module, Store};
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};

pub fn run_module(path: &Path) -> wasmtime::Result<()> {
    let engine = Engine::default();

    let module = Module::from_file(&engine, path)?;
    let mut store = Store::new(&engine, ());

    let mut linker = ModuleLinker::new(&engine);
    linker.func_wrap(
        "host",
        "print",
        |mut caller: Caller<'_, ()>, offset: u32, len: u32| {
            let mem = caller.get_export("memory").unwrap();
            let mem = mem.into_memory().unwrap();

            let mut buf = vec![0u8; len as usize];
            mem.read(caller, offset as usize, &mut buf).unwrap();
            let str = str::from_utf8(&buf).expect("input is invalid utf8");
            println!("{str}");
        },
    )?;

    let instance = linker.instantiate(&mut store, &module)?;

    let main_fn = instance
        .get_typed_func::<(i32, i32), u32>(&mut store, "main")
        .unwrap();

    let ret_code = main_fn.call(&mut store, (0, 0)).unwrap();
    assert_eq!(ret_code, 0);

    Ok(())
}

struct ComponentRunStates {
    // These two are required basically as a standard way to enable the impl of IoView and
    // WasiView.
    // impl of WasiView is required by [`wasmtime_wasi::p2::add_to_linker_sync`]
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
    // You can add other custom host states if needed
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

pub fn run_component(path: &Path, args: &[String]) -> wasmtime::Result<()> {
    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtx` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtx::builder().inherit_stdio().args(args).build();
    let state = ComponentRunStates {
        wasi_ctx: wasi,
        resource_table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, state);

    // Instantiate our component with the imports we've created, and run it.
    let component = Component::from_file(&engine, path)?;
    let command = Command::instantiate(&mut store, &component, &linker)?;
    let _ = command.wasi_cli_run().call_run(&mut store)?;

    Ok(())
}
