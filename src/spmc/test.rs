use super::channel;
use tokio::task;

#[tokio::test]
async fn single_threaded() {
    let (mut sender, mut receiver) = channel();
    sender.send(5i32).unwrap();
    assert_eq!(receiver.recv().await.unwrap(), 5);
    assert_eq!(receiver.try_recv().unwrap(), None);
    sender.send(7).unwrap();
    sender.send(8).unwrap();
    assert_eq!(receiver.recv().await.unwrap(), 8);
    assert_eq!(receiver.try_recv().unwrap(), None);

    let mut receiver2 = receiver.clone();

    sender.send(3).unwrap();
    assert_eq!(receiver2.recv().await.unwrap(), 3);

    sender.send(9).unwrap();
    sender.send(10).unwrap();
    assert_eq!(receiver2.recv().await.unwrap(), 10);
    assert_eq!(receiver.try_recv().unwrap(), None);

    sender.send(11).unwrap();
    assert_eq!(receiver.recv().await.unwrap(), 11);
    sender.send(12).unwrap();
    assert_eq!(receiver2.recv().await.unwrap(), 12);
    assert_eq!(receiver2.try_recv().unwrap(), None);
}

#[tokio::test]
async fn multi_threaded() {
    let (mut sender, mut receiver) = channel::<u32>();

    let receiver1_handle = task::spawn({
        let mut receiver = receiver.clone();
        async move {
            let mut received = false;
            while let Ok(message) = receiver.recv().await {
                assert!(message < 200);
                received = true;
            }
            received
        }
    });

    let receiver2_handle = task::spawn({
        async move {
            let mut received = false;
            while let Ok(message) = receiver.recv().await {
                assert!(message < 200);
                received = true;
            }
            received
        }
    });

    let sender_handle = task::spawn(async move {
        for i in 0 .. 200 {
            sender.send(i).unwrap();
            task::yield_now().await;
        }
    });

    let (received1, received2, ..) =
        tokio::join!(receiver1_handle, receiver2_handle, sender_handle);

    assert!(received1.unwrap() || received2.unwrap());
}
