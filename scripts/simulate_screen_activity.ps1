Add-Type -AssemblyName System.Windows.Forms

while ($true) {
    # Simulate mouse movement
    for ($i = 0; $i -lt 20; $i++) {
        $x = Get-Random -Minimum 0 -Maximum 1024
        $y = Get-Random -Minimum 0 -Maximum 768
        [System.Windows.Forms.Cursor]::Position = New-Object System.Drawing.Point($x, $y)
        Start-Sleep -Milliseconds 200
    }

    # Simulate typing
    [System.Windows.Forms.SendKeys]::SendWait("Screenpipe Test Content{ENTER}")
    [System.Windows.Forms.SendKeys]::SendWait("This is a test of screenpipe CLI{ENTER}")

    Start-Sleep -Seconds 1
}