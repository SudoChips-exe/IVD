$body = '{"url":"https://www.tiktok.com/@scout2015/video/6718335390845095173"}'
try {
    $resp = Invoke-WebRequest -Uri 'http://localhost:8080/api/download' -Method Post -Body $body -ContentType 'application/json' -UseBasicParsing
    Write-Output 'STATUS:'
    Write-Output $resp.StatusCode
    Write-Output 'CONTENT:'
    Write-Output $resp.Content
} catch {
    Write-Output 'ERROR:'
    Write-Output $_.Exception.GetType().FullName
    if ($_.Exception.Response) {
        $reader = New-Object System.IO.StreamReader($_.Exception.Response.GetResponseStream())
        Write-Output 'BODY:'
        Write-Output $reader.ReadToEnd()
    }
}
