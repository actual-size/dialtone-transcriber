use std::f32::consts::PI;

fn main() {
    let target_freq: f32 =  1336.0; // in Hz
    let n = 80;
    let threshold = 1_000_000.0;

    let mut reader = hound::WavReader::open("testsamples/dtmf_112163_112196_11#9632_##9696.wav").unwrap();
    let sample_rate = reader.spec().sample_rate;
    let sample_iter = reader.samples::<i16>();
    let mut q0;
    let mut q1;
    let mut q2;
    for (number_of_iterations, sample) in sample_iter.enumerate() {
        // set up
        let k = 0.5 + ((n as f32) * target_freq) / (sample_rate as f32);
        let w = (2.0 * PI / (n as f32))*k;
        let cosine = w.cos();
        let coeff = 2.0 * cosine;
        q1 = 0.0;
        q2 = 0.0;

        // per sample calculation
        q0 = coeff * q1 - q2 + (sample.unwrap() as f32);
        q2 = q1;
        q1 = q0;

        // end of block
        if number_of_iterations % n == 0 {
            let magnitude_squared = q1*q1 + q2*q2 - q1 * q2 * coeff;
            println!("{}Hz {} exist", target_freq, if magnitude_squared > threshold { "doesn't" } else { "does" });
    
        }
    }
}