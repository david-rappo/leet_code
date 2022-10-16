mod solution;

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
