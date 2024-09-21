extern "C" {
    fn MySleep(seconds: f64);
}

fn main() {
    let random_sleep = rand::thread_rng().gen_range(1..10);
    
    let begin = SystemTime::now();

    unsafe {
        MySleep(random_sleep as f64);
    }

    let end = SystemTime::now();

    let delta = end.duration_since(begin).unwrap();

    assert!(delta.as_secs_f32() >= random_sleep - 0.1, "The OS didn't sleep for 5s");
    assert!(delta.as_secs_f32() <= random_sleep + 1.0, "Slept too long");
}
