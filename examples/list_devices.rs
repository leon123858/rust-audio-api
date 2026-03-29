use rust_audio_api::AudioContext;

fn main() {
    println!("=== Available Output Devices ===");
    println!();

    let devices = AudioContext::available_output_devices().unwrap();

    if devices.is_empty() {
        println!("  (no output devices found)");
        return;
    }

    for (i, (name, _)) in devices.iter().enumerate() {
        // Hint: on Windows, Bluetooth devices show up as two endpoints:
        //   - "Speakers (BT Device)"  → A2DP music mode (high quality, ~200ms latency)
        //   - "Headset (BT Device)"   → HFP communication mode (low latency, ~40ms)
        println!("  [{}] {}", i, name);
    }

    println!();
    println!("Tip: Use AudioContext::new_with_device_name(\"Headset\") to select");
    println!("     the Bluetooth HFP endpoint for low-latency real-time audio.");
}
