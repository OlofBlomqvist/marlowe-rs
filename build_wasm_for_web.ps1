wasm-pack build --target web --all-features --out-dir pkg_web

$file = ".\pkg_web\marlowe_lang.js"
$offendingline = "new URL\('marlowe_lang_bg.wasm', import.meta.url\)"
$fixedline = "'marlowe_lang_bg.wasm'"

$newContent = (Get-Content $file) -replace $offendingline , $fixedline
Set-Content -Value $newContent -Path $file

copy-item pkg_Web/marlowe_lang* ./examples/wasm_web/


