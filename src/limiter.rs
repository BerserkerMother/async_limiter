use core::time;
use std::{
    future::Future,
    sync::{
        Arc,
        atomic::{self, AtomicUsize},
    },
    thread,
    time::Duration,
};

/// Rate limiter based on Token Bucket algorithm.
///
/// It allows for number of charges in specified interval. It can be used to limit API calls
/// or anywhere with restriction over number of request per interval.
#[derive(Clone)]
pub struct RateLimiter {
    charges: Arc<AtomicUsize>,
    tx: std::sync::mpsc::Sender<std::task::Waker>,
}

impl RateLimiter {
    /// new rate limiter from charges and duration to renew them
    ///
    /// ```
    /// // equals to 1000 requests every 60 seconds.
    /// let limiter = RateLimiter::new(1000, time::Duration::from_secs(60));
    pub fn new(total_charges: usize, charge_every: Duration) -> RateLimiter {
        let charges = Arc::new(AtomicUsize::new(total_charges));

        let (tx, rx) = std::sync::mpsc::channel::<std::task::Waker>();

        let inner = charges.clone();
        thread::spawn(move || {
            'outer: loop {
                thread::sleep(charge_every);
                inner.store(total_charges, atomic::Ordering::Relaxed);
                for _ in 0..total_charges {
                    match rx.try_recv() {
                        Ok(waker) => waker.wake(),
                        // self tx and all its clones must be dropped for this to happen!
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => break 'outer,
                        Err(_) => (),
                    }
                }
            }
        });

        RateLimiter { charges, tx }
    }

    /// new a limiter with number of requests per minute.
    pub fn with_rpm(rpm: usize) -> RateLimiter {
        RateLimiter::new(rpm, time::Duration::from_secs(60))
    }

    /// new a limiter with number of requests per second.
    pub fn with_rps(rps: usize) -> RateLimiter {
        RateLimiter::new(rps, time::Duration::from_secs(1))
    }

    /// return a future to be awaited.
    ///
    /// It will block until there is a charge to consume.
    pub fn wait(&self) -> Waiter {
        Waiter::new(self.charges.clone(), self.tx.clone())
    }
}

/// Waiter to be polled.
///
/// If there is a charge avaialble, it will complete, else it wakes up when charges renew.
pub struct Waiter {
    charges: Arc<AtomicUsize>,
    tx: std::sync::mpsc::Sender<std::task::Waker>,
}

impl Waiter {
    fn new(charges: Arc<AtomicUsize>, tx: std::sync::mpsc::Sender<std::task::Waker>) -> Waiter {
        Waiter { charges, tx }
    }
}

impl Future for Waiter {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.charges.load(atomic::Ordering::Relaxed) > 0 {
            self.charges.fetch_sub(1, atomic::Ordering::Relaxed);
            std::task::Poll::Ready(())
        } else {
            let waker = cx.waker().clone();
            self.tx.send(waker).unwrap();
            std::task::Poll::Pending
        }
    }
}
