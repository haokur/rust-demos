use lazy_static::lazy_static;
use std::cell::{Cell, RefCell};
use std::ops::Sub;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, MutexGuard, Once, RwLock, mpsc};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};
use std::{hint, thread};
use thread_local::ThreadLocal;

/// 并发：多个队伍用一个咖啡机
/// 并行：N个队伍对应用N个咖啡机
/// 并发是轮流处理，并行是同时处理

#[test]
fn test_basic_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("i = {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // for i in 1..5 {
    //     println!("i = {}", i);
    //     thread::sleep(Duration::from_millis(1000));
    // }
    // thread::sleep(Duration::from_millis(1000));
    handle.join().unwrap();
}

#[test]
fn test_thread_move() {
    let v = vec![1, 2, 3];

    // 报错： may outlive borrowed value `v`
    // let handle = thread::spawn(|| {
    //     println!("v = {:?}", v);
    // });

    let handle = thread::spawn(move || {
        println!("v = {:?}", v);
    });
    handle.join().unwrap();

    // 报错：borrow of moved value: `v`
    // println!("v = {:?}", v);
}

#[test]
fn test_thread_finished() {
    let new_thread = thread::spawn(move || {
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1));
                println!("i am a new thread");
            }
        })
    });
    new_thread.join().unwrap();
    println!("child new thread is finished");
    thread::sleep(Duration::from_millis(10));
}

#[test]
fn test_thread_barrier() {
    // Promise.all??
    let mut handlers = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    // 等6个线程全部执行完了b.wait()后，各线程再继续执行
    for _ in 0..6 {
        let b = barrier.clone();
        handlers.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }))
    }

    for handle in handlers {
        handle.join().unwrap();
    }
}

#[test]
fn test_thread_local_var() {
    // 线程局部变量
    thread_local! {static FOO:RefCell<u32> = RefCell::new(1)}
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let t = thread::spawn(move || {
        // 每个线程访问FOO时，都以初始值开始
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        })
    });

    t.join().unwrap();
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    })
}

#[test]
fn test_thread_local_lib() {
    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];

    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }

    for handle in v {
        handle.join().unwrap();
    }

    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        println!("x :{},y:{}", x, y.get());
        x + y.get()
    });
    assert_eq!(total, 5);
}

// 用条件控制线程的挂起和执行

#[test]
fn test_thread_wait_and_run() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("started is {}", *started);
    while !*started {
        println!("waiting for thread");
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

#[test]
fn test_thread_run_once() {
    // 多线程只被调用一次初始化
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe { VAL = 1 });
    });

    let handle2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        // 这里线程后执行，但是VAL已经初始化了，这里就不会再初始化了
        INIT.call_once(|| unsafe { VAL = 2 });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{:?}", unsafe { VAL });
}

// 多发送者，单接收者
#[test]
fn thread_multiple_send_single_receive() {
    let (tx, rx) = mpsc::channel();

    // 消息多发可以不接收，但是不能接收消息的次数>消息发送的次数
    thread::spawn(move || {
        tx.send(10).unwrap();
        thread::sleep(Duration::from_millis(10));
        tx.send(20).unwrap();
    });

    println!("receive {:?}", rx.recv().unwrap()); // 10
    println!("receive {:?}", rx.recv().unwrap()); // 20

    println!("end");
}

#[test]
fn test_try_recv() {
    // 不阻塞的try——recv
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(10).unwrap();
    });

    println!("receive {:?}", rx.try_recv()); // receive Err(Empty)
    thread::sleep(Duration::from_millis(10));
    println!("receive {:?}", rx.try_recv()); // receive Ok(10)
    // 使用try_recv时，接收次数>send次数，也不会报错
    println!("receive {:?}", rx.try_recv()); // receive Err(Disconnected)
    println!("end");
}

// 测试所有权
#[test]
fn test_thread_ownership() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("hello");
        tx.send(s).unwrap();
        // println!("value is {}", s); // Value used after being moved
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/// 使用for接收send的数据
#[test]
fn test_for_receive() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/// 测试多发送者

#[test]
fn test_multiple_sender() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send(10).unwrap();
    });

    thread::spawn(move || {
        tx1.send(20).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/// 同步通道

#[test]
fn test_async_channel() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        println!("before send");
        tx.send(10).unwrap();
        println!("after send");
    });

    println!("before sleep");
    // 以下并不会阻止thread::spawn的执行
    thread::sleep(Duration::from_secs(1));
    println!("after sleep");

    println!("receive {:?}", rx.recv().unwrap());
    handle.join().unwrap();
}

#[test]
fn test_sync_channel() {
    // bound参数为可以无阻塞发送的消息数量
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("before send");
        tx.send(10).unwrap();
        println!("after send");
    });

    println!("before sleep");
    // 阻塞住子进程的send调用
    thread::sleep(Duration::from_secs(1));
    println!("after sleep");

    println!("receive {:?}", rx.recv().unwrap());
    handle.join().unwrap();
}

enum Fruit {
    Apple(u8),
    Orange(String),
}
#[test]
fn test_send_multiple_type() {
    let (tx, rx) = mpsc::channel();

    tx.send(Fruit::Apple(1)).unwrap();
    tx.send(Fruit::Orange("orange".to_string())).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => {
                println!("Apple {}", count);
            }
            Fruit::Orange(flavor) => {
                println!("Orange {}", flavor);
            }
        }
    }
}

#[test]
fn test_thread_bug() {
    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let send = send.clone();
        thread::spawn(move || {
            send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // send 没有被drop，导致循环无法结束？
    drop(send);
    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished");
}

#[test]
fn test_mutex_read() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

#[test]
fn test_mutex_always_lock() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    println!("m = {:?}", m);

    drop(num);

    // num 没有drop，继续申请下一个锁
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    println!("m = {:?}", m);

    drop(num1);

    println!("m = {:?}", m);
}

#[test]
fn test_mutex_in_multiple_threads_fail() {
    let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    println!("Result is {}", *counter.lock().unwrap());
}

#[test]
fn test_mutex_in_multiple_threads_success() {
    // 使用Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result is {}", *counter.lock().unwrap());
}

#[test]
fn test_lock_problem() {
    let data = Mutex::new(0);
    {
        let d1 = data.lock();
    }
    println!("d1 = {:?}", data);
    let d2 = data.lock();
    let d3 = data.lock();

    println!("d1 = {:?}", data);
}

lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}
#[test]
fn test_lock_problem_in_multiple_threads() {
    let mut children = vec![];

    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                if i_thread % 2 == 0 {
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();
                    println!("thread {} done", i_thread);

                    sleep(Duration::from_millis(10));

                    let guard = MUTEX2.lock().unwrap();
                } else {
                    let _guard = MUTEX2.lock().unwrap();
                    println!("thread {} not done", i_thread);

                    let _guard = MUTEX1.lock().unwrap();
                }
            }
        }))
    }

    for child in children {
        let _ = child.join();
    }
    println!("done");
}

#[test]
fn fix_lock_by_try_lock() {
    let mut children = vec![];

    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                if i_thread % 2 == 0 {
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();
                    println!("thread {} done", i_thread);

                    sleep(Duration::from_millis(10));

                    let guard = MUTEX2.try_lock();
                    println!("线程 {} 获取 MUTEX2 锁的结果: {:?}", i_thread, guard);
                } else {
                    let _guard = MUTEX2.lock().unwrap();
                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                    sleep(Duration::from_millis(10));
                    let _guard = MUTEX1.try_lock();
                    println!("线程 {} 获取 MUTEX1 锁的结果: {:?}", i_thread, _guard);
                }
            }
        }))
    }

    for child in children {
        let _ = child.join();
    }
    println!("done");
}

#[test]
fn test_read_write_lock() {
    let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();
        *w = 6;
        assert_eq!(*w, 6);

        // 在写的锁未drop时，read lock会阻塞,可以手动drop
        drop(w);
        let r1 = lock.read().unwrap();
        println!("r1 = {:?}", r1);
    }
}

// 用条件变量控制线程的同步
#[test]
fn test_order_by_condvar() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());

    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = thread::spawn(move || {
        let mut lock = cflag.lock().unwrap();
        let mut counter = 0;

        while counter < 3 {
            while !*lock {
                lock = ccond.wait(lock).unwrap();
            }
            *lock = false;
            counter += 1;
            println!("inner counter : {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter : {}", counter);
        cond.notify_one();
    }

    hdl.join().unwrap();
    println!("{:?}", flag);
}

const N_TIMES: u64 = 10000;
const N_THREADS: u64 = 10;
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

#[test]
fn test_atomic() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS as usize);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("{} times,{} threads", N_TIMES, N_THREADS);
    println!("R.Ordering.Relaxed {}", R.load(Ordering::Relaxed));

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    println!("{:?}", Instant::now().sub(s));
}

#[test]
fn test_atomic_with_multiple_threads() {
    let spinlock = Arc::new(AtomicU64::new(0));

    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move || {
        spinlock_clone.store(1, Ordering::Relaxed);
    });

    while spinlock.load(Ordering::Relaxed) != 1 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("thread panic: {:?}", panic);
    }
}

#[derive(Debug)]
struct MyBox(*mut u8);
unsafe impl Send for MyBox {}

struct MyBox2(*const u8);
unsafe impl Send for MyBox2 {}
unsafe impl Sync for MyBox2 {}

#[test]
fn test_send_sync_safe() {
    // `Rc<i32>` cannot be sent between threads safely
    // let v = Rc::new(5);
    // let t = thread::spawn(move || {
    //     println!("{}", v);
    // });

    // `*mut u8` cannot be sent between threads safely
    // let p = 5 as *mut u8;
    // let t = thread::spawn(move || {
    //     println!("{:?}", p);
    // });

    // OK
    // let p = MyBox(5 as *mut u8);
    // let t = thread::spawn(move || {
    //     println!("{:?}", p);
    // });

    // OK
    // let v = 5;
    // let t = thread::spawn(move || {
    //     println!("{:?}", &v);
    // });

    let b = &MyBox2(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });

    t.join().unwrap();
}
