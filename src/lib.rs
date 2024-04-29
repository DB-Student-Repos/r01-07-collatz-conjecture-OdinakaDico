pub fn collatz(n: u64) -> Option<u64> {
pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    

    let mut steps = 0;
    while n != 1 {
        if n % 2 == 0 {
@@ -11,7 +11,10 @@ pub fn collatz(n: u64) -> Option<u64> {
            n = 3 * n + 1;
        }
        steps += 1;

        if steps == u64::MAX {
            return None;
        }
    }

    Some(steps)
}
