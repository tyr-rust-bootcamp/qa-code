use anyhow::Result;
use arc_swap::ArcSwapOption;
use std::thread;
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

struct Oneshot<T> {
    data: ArcSwapOption<T>,
    is_filled: AtomicBool,
}

struct Sender<T>(Arc<Oneshot<T>>);

struct Receiver<T>(Arc<Oneshot<T>>);

impl<T> Oneshot<T> {
    fn channel() -> (Sender<T>, Receiver<T>) {
        let oneshot = Arc::new(Oneshot {
            data: ArcSwapOption::from(None),
            is_filled: AtomicBool::new(false),
        });
        (Sender(oneshot.clone()), Receiver(oneshot))
    }
}

impl<T> Sender<T> {
    fn send(&self, value: T) -> Result<(), T> {
        if self
            .is_filled
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.data.store(Some(Arc::new(value)));
            Ok(())
        } else {
            Err(value)
        }
    }
}

impl<T> Receiver<T> {
    fn recv(&self) -> Result<T> {
        if self
            .is_filled
            .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            let v = self.data.swap(None);

            v.ok_or_else(|| anyhow::anyhow!("No value"))
                .and_then(|v| Arc::try_unwrap(v).map_err(|_| anyhow::anyhow!("Multiple receivers")))
        } else {
            Err(anyhow::anyhow!("Sender has not sent a value yet"))
        }
    }
}

impl<T> Deref for Receiver<T> {
    type Target = Arc<Oneshot<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for Sender<T> {
    type Target = Arc<Oneshot<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() -> Result<()> {
    let (tx, rx) = Oneshot::channel();

    let sender_thread = thread::spawn(move || {
        tx.send(42).unwrap();
    });

    let receiver_thread = thread::spawn(move || {
        let Ok(v) = rx.recv() else {
            eprintln!("Failed to receive value");
            return;
        };

        println!("Received: {}", v);
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();

    Ok(())
}
