use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};

struct Delay{
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        println!("try poll");
        // 若生成了waker则取出进行比对
        if let Some(waker) = &self.waker {
            println!("match waker");
            let mut waker = waker.lock().unwrap();
            // 比对是否为同一个线程的waker
            if !waker.will_wake(cx.waker()) {
                println!("match waker fail");
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }

                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready(())
        } else {
            // cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when  = Instant::now() + Duration::from_millis(500);
    let future = Delay {when, waker: None};
    let _ = future.await;

    // assert_eq!(out, "done");
}