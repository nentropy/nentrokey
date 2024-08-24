pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn displays_welcome_message_correctly() {
        let project_name = "Nentrokey";
        let description = "Encryption library for Rust";
        let result = run_orioncli(project_name, description);
        assert!(result.is_ok());
    }    

    #[test]
    fn handles_empty_strings() {
        let project_name = "";
        let description = "";
        let result = run_orioncli(project_name, description);
        assert!(result.is_ok());
    }
}
