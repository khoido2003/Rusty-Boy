//use mini_redis::{client, Command};
//use tokio::sync::mpsc;
//
//#[tokio::main]
//async fn main() {
//    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
//
//    let (tx, mut rx) = mpsc::channel(32);
//    let tx2 = tx.clone();
//
//    let t1 = tokio::spawn(async move {
//        let cmd = Command::Get {
//            key: "foo".to_string(),
//        };
//
//        tx.send(cmd).await.unwrap();
//    });
//
//    let t2 = tokio::spawn(async move {
//        let cmd = Command::Set {
//            key: "foo".to_string(),
//            val: "bar".into(),
//        };
//
//        tx2.send(cmd).await.unwrap();
//    });
//    let manager = tokio::spawn(async move {
//        // Establish a connection to the server
//        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
//
//        // Start receiving messages
//        while let Some(cmd) = rx.recv().await {
//            match cmd {
//                Get { key } => {
//                    client.get(&key).await;
//                }
//                Set { key, val } => {
//                    client.set(&key, val).await;
//                }
//            }
//        }
//    });
//    t1.await.unwrap();
//    t2.await.unwrap();
//    manager.await.unwrap();
//}
