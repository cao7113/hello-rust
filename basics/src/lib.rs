#[cfg(test)]
mod tests {
    #[test]
    fn try_deref() {
        let x = 5;
        let y = &x;
        assert_eq!(x, 5);
        assert_eq!(*y, 5);
        // assert_eq!(y, 5);
    }
}
