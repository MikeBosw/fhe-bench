use std::time::{Duration, Instant};
use tfhe::prelude::*;
use tfhe::{ConfigBuilder, FheUint, FheUint32, FheUint32Id, generate_keys, set_server_key};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();
    let (client_key, server_keys) = generate_keys(config);
    set_server_key(server_keys);
    let (clear_a, clear_b) = (13u32, 5u32);
    let encrypted_a = FheUint32::try_encrypt(clear_a, &client_key)?;
    let encrypted_b = FheUint32::try_encrypt(clear_b, &client_key)?;
    let (encrypted_res, fhe_op_time) = benchmark_single_fhe_op(encrypted_a, encrypted_b);
    let clear_res: u64 = encrypted_res.decrypt(&client_key);
    let (normal_val, normal_nanos_per_op) = benchmark_normal_op();
    let fhe_nanos_per_op = fhe_op_time.as_nanos() as f64;
    // print the non-FHE value to stdout in case not doing so would make it easier to optimize away
    // the computation
    println!("A random non-FHE value: {}", normal_val);
    println!("FHE value: {}", clear_res,);
    println!(
        "Diff between normal op and FHE op: {}x",
        (fhe_nanos_per_op / normal_nanos_per_op) as u64
    );

    Ok(())
}

/** Returns the product of two randomly generated numbers, and the nanos for said computation. */
fn benchmark_normal_op() -> (u64, f64) {
    let mut total_time: Duration = Duration::from_secs(0);
    let mut normal_val = 0;
    const N: i32 = 1;
    // the higher the N, the worse it looks for FHE, cuz these inner operations get optimized.
    // the advantage tails off at around N=10,000,000 on my MacBook Pro M3.
    for _ in 0..N {
        let new_start_ts = Instant::now();
        let rand_a = rand::random::<u8>() as u64;
        let rand_b = rand::random::<u8>() as u64;
        normal_val = rand_a * rand_b;
        let normal_time: Duration = Instant::now() - new_start_ts;
        total_time += normal_time;
    }
    let average_time = total_time.as_nanos() as f64 / N as f64;
    (normal_val, average_time)
}

fn benchmark_single_fhe_op(
    encrypted_a: FheUint<FheUint32Id>,
    encrypted_b: FheUint<FheUint32Id>,
) -> (FheUint<FheUint32Id>, Duration) {
    // We can't get an average over a bunch of FHE ops because it would take too long, but
    // to improve accuracy, we first warm up the FHE init by doing a simple operation.

    // Clear equivalent computations: 13 * 5 = 65
    let mut encrypted_res = &encrypted_a * &encrypted_b;

    // Now start the timer and do a second operation whose performance we will measure.
    let start_ts = Instant::now();

    // Clear equivalent computations: 65 * 5 = 325
    encrypted_res = &encrypted_res * &encrypted_b;
    (encrypted_res, Instant::now() - start_ts)
}
