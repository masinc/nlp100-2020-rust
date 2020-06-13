Write-Output $PSVersionTable
Write-Output ""
Write-Output ([String]::new('-', 80))
$failCount = 0

Get-ChildItem src `
| Where-Object { Test-Path -PathType Container $_ } `
| Foreach-Object {
    $name = $_.Name
    try {
        $res = (cargo run --bin ${name})
        if (!$?) {
            throw "${name}: Failed cargo run"
        }
        $ans_fn = "answers/${name}"
        if (!(Test-Path -PathType Leaf $ans_fn)) {
            throw "${name}: Not found ${ans_fn}"
        }
        $ans = (Get-Content ${ans_fn})

        $diff = (Compare-Object -CaseSensitive $res $ans)
        try
        {
            # Diff result, If "InputObject" or "SideIndicator" propery found
            $diff.InputObject > $null
            $diff
            $failCount++
            Write-Output "${name}: NG"
        } catch {
            Write-Output "${name}: OK"
        }
    } catch {
        $failCount++
        Write-Error $_
        Write-Output "${name}: NG"
    }
}

Write-Output ""
Write-Output ([String]::new('-', 80))
Write-Output "Failed: $failCount"
exit $failCount
