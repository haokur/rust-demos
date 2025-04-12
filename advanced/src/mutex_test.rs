use std::sync::{Arc, Mutex};
use std::thread;
use tokio::task;

// 以下在多线程下运行报错
// #[tokio::test]
// async fn test_mutex() {
//     let x = Mutex::new(0);
//     println!("x is {:?}", x);
//
//     let mut handlers = vec![];
//     for _ in 0..10 {
//         let handler = thread::spawn(|| async {
//             let mut num = x.lock().unwrap();
//             *num += 1;
//             println!(
//                 "handler is {:?} num is {}",
//                 std::thread::current().id(),
//                 *num
//             );
//         });
//         handlers.push(handler);
//     }
//     for handler in handlers {
//         handler.join().unwrap().await;
//     }
// }

#[tokio::test]
async fn test_tokio_mutex() {
    let x = Arc::new(Mutex::new(0));

    let mut handlers = vec![];

    for _ in 0..10 {
        let x = Arc::clone(&x);
        let handler = task::spawn(async move {
            let mut num = x.lock().unwrap();
            *num += 1;
            println!(
                "handler is {:?} num is {}",
                std::thread::current().id(),
                *num
            );
        });
        handlers.push(handler);
    }
    for handler in handlers {
        handler.await.unwrap();
    }
}
