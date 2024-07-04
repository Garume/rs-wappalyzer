use tokio::time::{Duration, sleep};

#[tokio::test]
async fn my_async_test() {
    // 非同期関数のテストコードを書く
    async fn do_something() -> u32 {
        // 1秒待機
        sleep(Duration::from_secs(1)).await;
        42
    }

    // 非同期関数を呼び出して結果を検証
    let result = do_something().await;
    assert_eq!(result, 42);
}
