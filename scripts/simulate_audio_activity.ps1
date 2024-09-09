# scripts/simulate_audio_activity.ps1
Add-Type -AssemblyName System.Speech

try {
    $synthesizer = New-Object System.Speech.Synthesis.SpeechSynthesizer

    Write-Output "Available voices:"
    [System.Speech.Synthesis.SpeechSynthesizer]::InstalledVoices | ForEach-Object {
        Write-Output "  $($_.VoiceInfo.Name)"
    }

    $audioDevice = $env:TEST_AUDIO_DEVICE
    if ($audioDevice) {
        Write-Output "Using audio device: $audioDevice"
        $synthesizer.SetOutputToAudioDevice($audioDevice)
    } else {
        Write-Output "TEST_AUDIO_DEVICE not set. Using default audio device."
        $synthesizer.SetOutputToDefaultAudioDevice()
    }

    while ($true) {
        try {
            Write-Output "Attempting to speak..."
            $synthesizer.Speak("This is a test of screenpipe CLI audio capture")
            Write-Output "Speech completed successfully"
            Start-Sleep -Seconds 2
        } catch {
            Write-Output "Error during speech synthesis: $_"
            Start-Sleep -Seconds 5
        }
    }
} catch {
    Write-Error "Error in simulate_audio_activity: $_"
    exit 1
}