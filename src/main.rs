use std::{collections::HashSet, sync::mpsc::channel, time::Instant};

use rand::Rng;
use threadpool::ThreadPool;

fn generate_random_hash() -> u16 {
    rand::thread_rng().gen()
}

const WORKERS: usize = 4;
const JOBS: usize = 1_000_000;

fn main() {
    let start = Instant::now();

    let mut channels = Vec::new();

    for n in (2..=16).map(|t| 2usize.pow(t)) {
        let pool = ThreadPool::new(WORKERS);
        let (tx, rx) = channel();

        channels.push((rx, n));

        for _ in 0..JOBS {
            let tx = tx.clone();
            pool.execute(move || {
                let mut hashes = HashSet::new();

                for _ in 0..n {
                    let hash = generate_random_hash();

                    if hashes.contains(&hash) {
                        tx.send(true).expect("Failed to send");
                        return;
                    }

                    hashes.insert(hash);
                }

                tx.send(false).expect("Failed to send");
            });
        }
        // for _ in 0..n {
        //     let hash = generate_random_hash();
        //     println!("{:?}", hash);
        // }
    }

    for (rx, n) in channels {
        let collision_count = rx.iter().take(JOBS).filter(|b| *b).count();

        println!(
            "{}: {}/{} = {:.5}%",
            n,
            collision_count,
            JOBS,
            collision_count as f64 / JOBS as f64 * 100.0
        );
    }

    println!("Finished in {:.2} seconds", start.elapsed().as_secs_f64());
}
