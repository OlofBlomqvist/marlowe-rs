wasm-pack build --target nodejs --all-features --out-dir pkg_njs

$file = ".\pkg_njs\marlowe_lang.js"
$offendingline = "new URL\('marlowe_lang_bg.wasm', import.meta.url\)"
$fixedline = "'marlowe_lang_bg.wasm'"

$newContent = (Get-Content $file) -replace $offendingline , $fixedline
Set-Content -Value $newContent -Path $file

copy-item pkg_njs/marlowe_lang* ./examples/wasm_nodejs/


