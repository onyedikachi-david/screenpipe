Add-Type -AssemblyName System.Windows.Forms

try {
    Write-Output "Starting screen activity simulation"
    while ($true) {
        # Simulate mouse movement
        for ($i = 0; $i -lt 20; $i++) {
            $x = Get-Random -Minimum 0 -Maximum ([System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Width)
            $y = Get-Random -Minimum 0 -Maximum ([System.Windows.Forms.Screen]::PrimaryScreen.Bounds.Height)
            [System.Windows.Forms.Cursor]::Position = New-Object System.Drawing.Point($x, $y)
            Write-Output "Moved cursor to ($x, $y)"
            Start-Sleep -Milliseconds 200
        }

        # Simulate typing
        Write-Output "Simulating typing"
        [System.Windows.Forms.SendKeys]::SendWait("Screenpipe Test Content{ENTER}")
        [System.Windows.Forms.SendKeys]::SendWait("This is a test of screenpipe CLI{ENTER}")

        Start-Sleep -Seconds 1
    }
} catch {
    Write-Error "Error in simulate_screen_activity: $_"
    exit 1
}