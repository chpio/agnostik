pub use agnostik::prelude::*;
pub use tokio_crate as tokio;

#[cfg(feature = "runtime_tokio")]
mod tokio_tests {
    use super::*;
    #[test]
    fn test_tokio() {
        agnostik::block_on(async {
            agnostik::spawn(async {
                let mut i = 0;
                while i < 5 {
                    println!("Counting from Tokio: {}", i);
                    i += 1;
                }
            })
            .await
        });
    }

    #[test]
    fn test_basic_scheduler() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let rt = std::sync::Arc::new(rt);

        for _ in 0..2 {
            let rt = rt.clone();
            std::thread::spawn(move || {
                rt.block_on(
                    async move { tokio::time::sleep(std::time::Duration::from_secs(1)).await },
                );
            })
            .join()
            .unwrap();
        }
    }

    #[test]
    fn test_tokio_implicit() {
        let res = agnostik::block_on(async {
            agnostik::spawn(async {
                println!("hello world");
                1
            })
            .await
        });
        assert_eq!(res, 1);
    }

    // #[test]
    // fn test_tokio_spawn_local() {
    //     let runtime = tokio::runtime::Runtime::new().expect("Failed to create a runtime");
    //     let runtime = Agnostik::tokio_with_runtime(runtime);
    //     let runtime = std::sync::Arc::new(runtime);
    //     let runtime_clone = runtime.clone();

    //     let result =
    //         runtime.block_on(async move { runtime_clone.spawn_local(async { 1337 }).await });

    //     assert_eq!(result, 1337);
    // }
}
