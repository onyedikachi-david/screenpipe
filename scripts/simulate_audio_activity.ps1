# scripts/simulate_audio_activity.ps1
Add-Type -AssemblyName System.Speech

try {
    $synthesizer = New-Object System.Speech.Synthesis.SpeechSynthesizer

    Write-Output "Available voices:"
    [System.Speech.Synthesis.SpeechSynthesizer]::InstalledVoices | ForEach-Object {
        Write-Output "  $($_.VoiceInfo.Name)"
    }

    # Try to set the audio output to the VB-CABLE device
    try {
        $devices = [System.Speech.Synthesis.SpeechSynthesizer]::InstalledVoices
        $vbCableDevice = $devices | Where-Object { $_.VoiceInfo.Name -like "*CABLE Output*" }
        
        if ($vbCableDevice) {
            $synthesizer.SelectVoice($vbCableDevice.VoiceInfo.Name)
            Write-Output "Selected VB-CABLE Output device: $($vbCableDevice.VoiceInfo.Name)"
        } else {
            Write-Output "VB-CABLE Output device not found. Using default device."
            $synthesizer.SetOutputToDefaultAudioDevice()
        }
    } catch {
        Write-Output "Failed to set audio output: $_"
        Write-Output "Using default audio device."
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