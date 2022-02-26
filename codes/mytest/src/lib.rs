#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        //panic!("Make this test fail!");
    }

    #[test]
    fn test_tup() {
        let tup = (500, 6.4, 1);
        let (_x, y, _) = tup;

        assert_eq!(6.4, y);
        assert_eq!(500, _x);

        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;

        assert_eq!(6.4, six_point_four);
        assert_eq!(500, five_hundred);
        assert_eq!(1, one);

        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let first = a[0];
        assert_eq!(1, first);

        
    }
}
