#[cfg(test)]
#[macro_export]
macro_rules! with_test_connection {
    ($($body:tt)*) => {{
        let temp_dir = tempfile::TempDir::new().unwrap();
        let data_dir = temp_dir.path().join("data/complish");

        temp_env::with_var("COMPLISH_DATA_DIR", Some(data_dir), || {
            $crate::store::connect().unwrap();

            $($body)*
        })
    }};
}

#[cfg(test)]
pub use with_test_connection;
