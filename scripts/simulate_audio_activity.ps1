Add-Type -AssemblyName System.Speech

$synthesizer = New-Object System.Speech.Synthesis.SpeechSynthesizer

# Try to set the audio output to the default device
try {
    $synthesizer.SetOutputToDefaultAudioDevice()
} catch {
    Write-Output "Failed to set audio output to default device: $_"
    # If setting to default device fails, try to use the first available device
    $audioDevices = [System.Speech.Synthesis.SpeechSynthesizer]::InstalledVoices
    if ($audioDevices.Count -gt 0) {
        $synthesizer.SelectVoice($audioDevices[0].VoiceInfo.Name)
    } else {
        Write-Output "No audio devices available. Audio simulation will be skipped."
        exit
    }
}

while ($true) {
    try {
        $synthesizer.SpeakAsync("This is a test of screenpipe CLI audio capture")
        Start-Sleep -Seconds 2
    } catch {
        Write-Output "Error during speech synthesis: $_"
        Start-Sleep -Seconds 5
    }
}