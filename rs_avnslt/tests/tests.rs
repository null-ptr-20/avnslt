#[cfg(test)]
pub mod test {

    use avnslt::scan_input;

    #[test]
    fn test_scan() {
        let mut string = String::from("okokok");
        let output = String::from("okokok\n");
        assert_eq!(output, scan_input(&mut string));
    }
}
