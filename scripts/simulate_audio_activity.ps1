Add-Type -AssemblyName System.Speech

try {
    $synthesizer = New-Object System.Speech.Synthesis.SpeechSynthesizer

    Write-Output "Available voices:"
    [System.Speech.Synthesis.SpeechSynthesizer]::InstalledVoices | ForEach-Object {
        Write-Output "  $($_.VoiceInfo.Name)"
    }

    # Try to set the audio output to the default device
    try {
        $synthesizer.SetOutputToDefaultAudioDevice()
        Write-Output "Successfully set audio output to default device"
    } catch {
        Write-Output "Failed to set audio output to default device: $_"
        # If setting to default device fails, try to use the first available device
        $audioDevices = [System.Speech.Synthesis.SpeechSynthesizer]::InstalledVoices
        if ($audioDevices.Count -gt 0) {
            $synthesizer.SelectVoice($audioDevices[0].VoiceInfo.Name)
            Write-Output "Selected voice: $($audioDevices[0].VoiceInfo.Name)"
        } else {
            Write-Output "No audio devices available. Audio simulation will be skipped."
            exit
        }
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