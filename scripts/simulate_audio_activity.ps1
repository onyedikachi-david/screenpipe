Add-Type -AssemblyName System.Speech

$synthesizer = New-Object System.Speech.Synthesis.SpeechSynthesizer

while ($true) {
    $synthesizer.Speak("This is a test of screenpipe CLI audio capture")
    Start-Sleep -Seconds 2
}