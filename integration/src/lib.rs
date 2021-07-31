pub mod pythonCaller;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn calling() {
        super::pythonCaller::test();
    }
}