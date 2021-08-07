pub mod pythonCaller;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn calling0() {
        super::pythonCaller::test();
    }

    #[test]
    fn calling1() {
        let mut emgData: Vec<u64> = Vec::new();
        emgData.push(1);
        emgData.push(2);
        super::pythonCaller::passArray(&emgData);
    }
}