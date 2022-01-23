#[cfg(test)]
mod tests {
    use rust_library_project_template::hello_world;

    #[test]
    fn test_hello_world() {
        assert_eq!(hello_world(), "Hello, world!".to_string());
    }
}
