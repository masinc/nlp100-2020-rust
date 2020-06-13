Write-Output $PSVersionTable
Write-Output ""
Write-Output ([String]::new('-', 80))
$failCount = 0

Get-ChildItem src `
| Where-Object { Test-Path -PathType Container $_ } `
| Foreach-Object {
    $name = $_.Name
    try {
        $r = (cargo run --bin ${name})
        if (!$?) {
            throw "${name}: Failed cargo run"
        }
        $ans_fn = "answers/${name}"
        if (!(Test-Path -PathType Leaf $ans_fn)) {
            throw "${name}: Not found ${ans_fn}"
        }
        $ans = (Get-Content ${ans_fn})

        if ( $r -eq $ans) {
            Write-Output "${name}: OK" `
        } else {
            $failCount++
            Write-Output "${name}: NG"
        }
    } catch {
        $failCount++
        Write-Error $Error[0]
        Write-Output "${name}: NG"
    }
}

Write-Output ""
Write-Output ([String]::new('-', 80))
Write-Output "Failed: $failCount"
exit $failCount
