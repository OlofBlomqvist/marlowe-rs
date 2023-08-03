[cmdletbinding()] 
param(
    [switch]$publish,
    [switch]$skipBuild,
    [switch]$publishLatestTag,
    [string]$version = $null
)



$nodejs_dir = "pkg_njs"
$bundle_dir = "pkg_bundle"
$web_dir = "pkg_web"
$latest_dir = "pkg_latest"

if($skipBuild -ne $true) {


    if(Test-Path $nodejs_dir) {
        Remove-Item $nodejs_dir/ -Recurse -Force -ErrorAction Stop
    }
    if(Test-Path $bundle_dir) {
        Remove-Item $bundle_dir/ -Recurse -Force -ErrorAction Stop
    }
    if(Test-Path $web_dir) {
        Remove-Item $web_dir/ -Recurse -Force -ErrorAction Stop
    }
    if(Test-Path $latest_dir) {
        Remove-Item $latest_dir/ -Recurse -Force -ErrorAction Stop
    }
    # Build for NodeJS
    wasm-pack build --all-features --release --target nodejs --out-dir $nodejs_dir/pkg
    $j = Get-Content $nodejs_dir/pkg/package.json |convertfrom-json
    if($null -ne $version -and $version.Length -gt 1) {
        $j.version = $version + "-nodejs"
    } else {
        $j.version = $j.version + "-nodejs"
    }    
    $j | convertto-json | out-file .\$nodejs_dir/pkg/package.json
    $njsv = $j.version
    #wasm-pack pack $nodejs_dir


    # Build for Web
    wasm-pack build --all-features --release --target web --out-dir $web_dir/pkg
    $j = Get-Content $web_dir/pkg/package.json |convertfrom-json
    if($null -ne $version -and $version.Length -gt 0) {
        $j.version = $version + "-web"
    } else {
        $j.version = $j.version + "-web"
    }   
    $j | convertto-json | out-file .\$web_dir/pkg/package.json
    $webv = $j.version
    
    $file = ".\$web_dir\pkg\marlowe_lang.js"
    $offendingline = "new URL\('marlowe_lang_bg.wasm', import.meta.url\)"
    $fixedline = "'marlowe_lang_bg.wasm'"

    $newContent = (Get-Content $file) -replace $offendingline , $fixedline
    Set-Content -Value $newContent -Path $file

    #wasm-pack pack $web_dir

    # Build bundle
    wasm-pack build --all-features --release --target bundler --out-dir $bundle_dir/pkg
    $j = Get-Content $bundle_dir/pkg/package.json |convertfrom-json
    if($null -ne $version -and $version.Length -gt 0) {
        $j.version = $version + "-bundle"
    } else {
        $j.version = $j.version + "-bundle"
    }   
    $j | convertto-json | out-file .\$bundle_dir/pkg/package.json
    
    $bundlev = $j.version
    #wasm-pack pack $bundle_dir

    # Build latest
    wasm-pack build --all-features --release --target nodejs --out-dir $latest_dir/pkg
    $j = Get-Content $latest_dir/pkg/package.json |convertfrom-json
    if($null -ne $version -and $version.Length -gt 0) {
        $j.version = $version
    } else {
        $j.version = $j.version
    }   
    $j | convertto-json | out-file .\$latest_dir/pkg/package.json
    
    $latestlev = $j.version
    #wasm-pack pack $bundle_dir

    Write-Host -ForegroundColor Green "---> $njsv"
    Write-Host -ForegroundColor Green "---> $webv"
    Write-Host -ForegroundColor Green "---> $bundlev"
    Write-Host -ForegroundColor Green "---> $latestlev"

} else {
    Write-Warning "Build skipped"
}

if($publish) {
    # write-host -ForegroundColor Green "Publishing web... $web_dir $webv"
    # wasm-pack publish -t web --tag web $web_dir
    # write-host -ForegroundColor Green "Publishing nodejs... $nodejs_dir $njsv"
    # wasm-pack publish -t nodejs --tag nodejs   $nodejs_dir
    # write-host -ForegroundColor Green "Publishing bundle... $bundle_dir $bundlev"
    # wasm-pack publish -t bundler --tag bundle $bundle_dir
    if($publishLatestTag) {
        write-host -ForegroundColor Green "Publishing latest... $latest_dir $latestlev"
        wasm-pack publish -t nodejs --tag latest $latest_dir
        
    }
}