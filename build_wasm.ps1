wasm-pack build --target web --all-features

$file = ".\pkg\marlowe_lang.js"
$offendingline = "new URL\('marlowe_lang_bg.wasm', import.meta.url\)"
$fixedline = "'marlowe_lang_bg.wasm'"

$newContent = (Get-Content $file) -replace $offendingline , $fixedline
Set-Content -Value $newContent -Path $file

copy-item pkg/marlowe_lang* ./examples/wasm/


