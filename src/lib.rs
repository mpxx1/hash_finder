use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{stdout, BufWriter, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

trait CheckHash {
    fn check(&self, n: &usize) -> bool;
}

impl CheckHash for String {
    // checks if hash has enough zeros in the end
    fn check(&self, n: &usize) -> bool {
        self.ends_with(&"0".repeat(*n))
    }
}

// calculating hash function
fn find_hash(index: &u64) -> String {
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, index.to_string().as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn calculate(zeros: &usize, dst_count: &usize, out_file: Option<&str>) {
    // The program runs most efficiently when we create a slice and divide
    // it into as many parts as there are threads available on the machine
    let chunk_size = u64::MAX / num_cpus::get() as u64;
    let cur_count: AtomicUsize = AtomicUsize::new(0);

    // writer can be changed to file for tests
    let writer: Arc<Mutex<dyn Write + Send>> = match out_file {
        Some(path) => Arc::new(Mutex::new(BufWriter::new(
            File::create(path).expect(format!("Could not create file {}", path).as_str()),
        ))),
        None => Arc::new(Mutex::new(stdout())),
    };

    (1u64..u64::MAX)
        .step_by(chunk_size as usize)
        .par_bridge() // here we create as many threads as machine have
        .for_each(|index| {
            // last time index + chunk_size could overflow u64 in dev mode
            let chunk = index..if let Some(x) = index.checked_add(chunk_size + 1) {
                x
            } else {
                u64::MAX
            };

            // iterating over chunk in single thread
            for c in chunk {
                // if we got at least as we need hashes we stop calculating more
                // each thread stops it work by itself
                if cur_count.load(Ordering::Acquire) >= *dst_count {
                    return;
                }

                let hash = find_hash(&c);
                // if calculated hash is fine
                if hash.check(zeros) {
                    // if we steel need to print this hash
                    if cur_count.fetch_add(1, Ordering::AcqRel) <= *dst_count {
                        // we lock stdout and print found hash (as soon as possible)
                        writer
                            .lock()
                            .unwrap()
                            .write_fmt(format_args!("{c}, \"{hash}\"\n"))
                            .unwrap()
                    }
                }
            }
        });
}
