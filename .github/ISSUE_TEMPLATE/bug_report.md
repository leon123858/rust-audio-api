---
name: 🐛 Bug Report
about: Create a bug report to help us improve rust-audio-api
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps or a minimal code snippet to reproduce the behavior:
1. Create '...' node
2. Connect '...' to '...'
3. Run '...' and observe the error

**Expected behavior**
A clear and concise description of what you expected to happen. (e.g., "The audio should play seamlessly, but I experienced a glitch/audio dropout or unexpectedly high latency.")

**Environment**
Since audio processing is highly dependent on the underlying system and hardware, please provide the following information:
- **OS:** [e.g., Windows 11, macOS 14, Ubuntu 22.04]
- **Audio Backend:** [e.g., WASAPI, CoreAudio, ALSA, JACK] 
- **Rust Version (`rustc --version`):** [e.g., 1.75.0]
- **Audio Settings:** [e.g., Sample Rate: 44100Hz, Buffer Size: 256]
- **Audio Hardware (Optional):** [e.g., Specific audio interface model being used]

**Additional context**
Add any other context about the problem here. (e.g., Does this issue only occur with a specific sequence of `AudioNode` connections? How does the current behavior differ from the standard Web Audio API specifications?)