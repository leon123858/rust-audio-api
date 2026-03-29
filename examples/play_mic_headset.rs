use rust_audio_api::AudioContext;
use rust_audio_api::nodes::{GainNode, MicrophoneNode, NodeType};

fn main() {
    // 1. List all available output devices
    println!("=== Available Output Devices ===");
    let devices = AudioContext::available_output_devices().unwrap();
    for (i, (name, _)) in devices.iter().enumerate() {
        println!("  [{}] {}", i, name);
    }
    println!();

    // 2. Try to find a "Headset" (HFP / communication mode) endpoint.
    //    If not found, fall back to the default output device.
    let mut ctx = match AudioContext::new_with_device_name("Headset") {
        Ok(ctx) => {
            println!("✓ Found Headset (HFP) endpoint — using low-latency communication mode");
            ctx
        }
        Err(_) => {
            println!("⚠ No Headset endpoint found, falling back to default output device");
            AudioContext::new().unwrap()
        }
    };

    let sample_rate = ctx.sample_rate();
    println!("  Sample rate: {} Hz", sample_rate);
    println!();

    // 3. Build audio graph: Microphone → Gain → Output
    let dest_id = ctx.build_graph(|builder| {
        println!("Initializing microphone...");
        let mic_node = MicrophoneNode::new(sample_rate).expect("Unable to initialize microphone");
        let mic = builder.add_node(NodeType::Microphone(mic_node));

        let master_gain = builder.add_node(NodeType::Gain(GainNode::new(1.0)));
        builder.connect(mic, master_gain);

        master_gain
    });

    // 4. Start audio thread
    ctx.resume(dest_id).unwrap();

    // 5. Monitor real-time performance
    let monitor = ctx.performance_monitor();

    println!("========================================");
    println!("🎤 Real-time Microphone Test");
    println!("   Speak into the mic — you should hear yourself!");
    println!("   Press Enter to exit...");
    println!("========================================");
    println!();

    // Spawn a thread to print performance stats periodically
    let monitor_clone = monitor.clone();
    let stats_thread = std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let load = monitor_clone
                .current_load_percent
                .load(std::sync::atomic::Ordering::Relaxed);
            let late = monitor_clone
                .late_callbacks
                .load(std::sync::atomic::Ordering::Relaxed);
            println!("  [perf] CPU load: {}% | late callbacks: {}", load, late);
        }
    });

    let _ = std::io::stdin().read_line(&mut String::new());

    drop(stats_thread);
    println!("Done.");
}
