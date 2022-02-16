use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        // Clone the handle to the hash map.
        let db = db.clone();

        println!("Accepted");

        // 当一个请求被接收，服务端会阻塞在这个 accept 这里，直到 response 被发送给 socket.
        // 需要注意 async 函数中多个 await 操作之前是顺序执行的，也就是前面执行结束后，才会轮到后者。
        // process(socket).await;
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        // 为了能够处理更多的请求，我们可以通过 spawn 为每个请求创建一个协程。
        // tokio task 是一个协程，可以通过 tokio::spawn 创建. tokio::spawn 会返回一个
        // JoinHandle，可以在 JoinHandle 后添加 await 获取返回值。
        // spawn 会吧 task 提交给 tokio 的调度器，通过调度器运行异步程序。
        // task 的生命周期必须是静态的，也就是 task 中不能存在对外部数据的引用。
        // 可以通过 move 去避免。当多个 task 中共享同一个外部变量的时候，我们可以通过 Arc 指针。
        // Send 语义：通过 tokio::spawn 创建的 task 必须要实现 Send，通过 Send 允许 tokio 可以跨线程操作 task.
        // 当出现 await 调用的时候，此时调度器会记录 await 后面的程序状态 (这里就包含 rs 未释放信息). 如果此时等待的是一个线程全局锁的话，那么其他任务调度到相同的线程会出现死锁。
        // 我们可以通过 Arc 和 Mutex 在 task 间共享数据，但是有时候也不行。
        // 问题就出现在 await 这里，此时我们需要 mutex lock，但是我们前面提到
        // 普通的 mutex 没有实现 Send，所以必须要在 await 前被释放。我们这里可以采用异步锁。
        // 异步锁: tokio::sync::Mutex，采用异步锁之后，其作用域就可以跨过 await，但是他的开始是比普通锁大的。
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
  use mini_redis::Command::{self, Get, Set};

  // Connection, provided by `mini-redis`, handles parsing frames from
  // the socket
  let mut connection = Connection::new(socket);

  // Use `read_frame` to receive a command from the connection.
  while let Some(frame) = connection.read_frame().await.unwrap() {
      let response = match Command::from_frame(frame).unwrap() {
          Set(cmd) => {
              let mut db = db.lock().unwrap();
              // The value is stored as `Vec<u8>`
              db.insert(cmd.key().to_string(), cmd.value().clone());
              Frame::Simple("OK".to_string())
          }
          Get(cmd) => {
              let db = db.lock().unwrap();
              if let Some(value) = db.get(cmd.key()) {
                  // `Frame::Bulk` expects data to be of type `Bytes`. This
                  // type will be covered later in the tutorial. For now,
                  // `&Vec<u8>` is converted to `Bytes` using `into()`.
                  Frame::Bulk(value.clone())
              } else {
                  Frame::Null
              }
          }
          cmd => panic!("unimplemented {:?}", cmd),
      };

      // Write the response to the client
      connection.write_frame(&response).await.unwrap();
  }
}