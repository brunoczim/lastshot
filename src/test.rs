use crate::channel;
use tokio::task;

#[tokio::test]
async fn single_threaded() {
    let (sender, receiver) = channel();
    sender.send(5i32).unwrap();
    assert_eq!(receiver.recv().await.unwrap(), 5);
    assert_eq!(receiver.try_recv().unwrap(), None);
    sender.send(7).unwrap();
    sender.send(8).unwrap();
    assert_eq!(receiver.recv().await.unwrap(), 8);
    assert_eq!(receiver.try_recv().unwrap(), None);

    let sender2 = sender.clone();
    let receiver2 = receiver.clone();

    sender2.send(3).unwrap();
    assert_eq!(receiver2.recv().await.unwrap(), 3);

    sender.send(9).unwrap();
    sender2.send(10).unwrap();
    assert_eq!(receiver.recv().await.unwrap(), 10);
    assert_eq!(receiver2.try_recv().unwrap(), None);

    sender2.send(11).unwrap();
    sender.send(12).unwrap();
    assert_eq!(receiver2.recv().await.unwrap(), 12);
    assert_eq!(receiver.try_recv().unwrap(), None);
}

#[tokio::test]
async fn multi_threaded() {
    let (sender, receiver) = channel::<u32>();

    let receiver1_handle = task::spawn({
        let receiver = receiver.clone();
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
        let receiver = receiver.clone();
        async move {
            let mut received = false;
            while let Ok(message) = receiver.recv().await {
                assert!(message < 200);
                received = true;
            }
            received
        }
    });

    let sender1_handle = task::spawn({
        let sender = sender.clone();
        async move {
            for i in 0 .. 100 {
                sender.send(i).unwrap();
                task::yield_now().await;
            }
        }
    });

    let sender2_handle = task::spawn({
        async move {
            for i in 100 .. 200 {
                sender.send(i).unwrap();
                task::yield_now().await;
            }
        }
    });

    let (received1, received2, ..) = tokio::join!(
        receiver1_handle,
        receiver2_handle,
        sender1_handle,
        sender2_handle
    );

    assert!(received1.unwrap() || received2.unwrap());
}
