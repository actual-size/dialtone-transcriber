use std::f32::consts::PI;

fn main() {
    // let omega_zero: f32 =  0.949_546_379_547_515; // radians per sample @ 8kHz, freq = 1209 Hz
    let omega_zero: f32 =  1336.0; // radians per sample @ 8kHz, freq = 1209 Hz
    let n = 80;
    let threshold = 1_000_000.0;

    let mut reader = hound::WavReader::open("testsamples/dtmf_112163_112196_11#9632_##9696.wav").unwrap();
    let sample_rate = reader.spec().sample_rate;
    let mut sample_iter = reader.samples::<i16>();
    let mut q0;
    let mut q1;
    let mut q2;
    
    'outer: loop {
        let k = 0.5 + ((n as f32) * omega_zero) / (sample_rate as f32);
        let w = (2.0 * PI / (n as f32))*k;
        let cosine = w.cos();
        // let sine = w.sin();
        let coeff = 2.0 * cosine;
        q1 = 0.0;
        q2 = 0.0;
        for _ in 0..n {
            match sample_iter.next() {
                None => break 'outer,
                Some(i) => {
                    q0 = coeff * q1 - q2 + i.unwrap() as f32;
                    q2 = q1;
                    q1 = q0;
                }
            }
            
        }
        let magnitude_squared = q1*q1 + q2*q2 - q1 * q2 * coeff;
        if magnitude_squared > threshold {
            println!("{} exists {}", omega_zero, magnitude_squared);
        }
    }
    // let test = sample_iter.next().unwrap() as f64;
    //                     .fold(0.0, |sqr_sum, s| {
    //     let sample = s.unwrap() as f64;
    //     sqr_sum + sample * sample
    // })
    // println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
}


/*

k = (0.5 + (N * target_freq) / sample_rate) as i32
w = (2*π/N)*k
cosine = cos w
sine = sin w
coeff = 2 * cosine

*/