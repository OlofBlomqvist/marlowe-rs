throw "this script is disabled pending wasi support in cml"
Remove-Item pkg_wasi/* -ea stop
cargo build --features="wasi,utils" --target wasm32-wasi --release --out-dir pkg_wasi -Z unstable-options
remove-item pkg_wasi/*.rlib -ea stop
if($null -eq (test-path .\pkg_wasi\marlowe_lang_cli.wasm)) {
    throw "marlowe_lang_cli.wasm was not correctly built"
}
if($null -eq (test-path .\pkg_wasi\marlowe_lang.wasm)) {
    throw "marlowe_lang.wasm was not correctly built"
}
write-host -ForegroundColor Green "Successfully built crate for wasi wasm @ pkg_wasi"

# you can try calling these files using wasmtime , wasmer or any other runtime

# For installing wasmer:
# linux: curl https://get.wasmer.io -sSfL | sh
# windows: iwr https://win.wasmer.io -useb | iex

# Calling an exposed method in the library using wasmtime
# wasmtime --invoke foo \pkg_wasi\marlowe_lang_cli.wasm --wasm-features all

# Calling the cli using wasmtime
# wasmtime .\pkg_wasi\marlowe_lang_cli.wasm

# Calling the cli using wasmer
# wasmer run .\pkg_wasi\marlowe_lang_cli.wasm
# wasmer run .\pkg_wasi\marlowe_lang_cli.wasm contract from-string Close marlowe-dsl json

# Calling an exposed lib method using wasmer
# wasmer run .\pkg_wasi\marlowe_lang_cli.wasm --invoke foo 

# Listing the exported functions in a wasm file using wasmer:
# wasmer inspect .\pkg_wasi\marlowe_lang_cli.wasm