pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = 17793440657059086836;
        let result = add(a, a);
        assert_eq!(result, usize::MAX);
    }
}
