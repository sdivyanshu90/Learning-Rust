fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 8;
    'outer: for i in 0..=2 {
        'inner: for j in 0..=2 {
            elements_searched += 1;
            dbg!(s[i][j]);
            if s[i][j] == target_value {
                break 'inner;
            }
        }
    }
    dbg!(elements_searched);
}