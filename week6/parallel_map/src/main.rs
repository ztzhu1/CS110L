use crossbeam_channel::{self, unbounded};
use std::{thread, time};

fn parallel_map<T, U, F>(input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    unsafe {
        output_vec.set_len(input_vec.len());
    }
    // TODO: implement parallel map!
    let (worker_send, worker_recv) = unbounded();
    let (main_send, main_recv) = unbounded();
    for (i, val) in input_vec.into_iter().enumerate() {
        main_send.send((i, val)).expect("fail to send!");
    }
    drop(main_send); // main thread doesn't send anymore.

    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let main_recv_clone = main_recv.clone();
        let worker_send_clone = worker_send.clone();
        threads.push(thread::spawn(move || {
            while let Ok((idx, val)) = main_recv_clone.recv() {
                worker_send_clone.send((idx, f(val))).unwrap();
            }
        }));
    }
    drop(worker_send); // workers don't send anymore.

    while let Ok((idx, val)) = worker_recv.recv() {
        *output_vec.get_mut(idx).unwrap() = val;
    }
    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
