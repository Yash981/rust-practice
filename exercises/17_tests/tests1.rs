// Tests are important to ensure that your code does what you think it should
// do.

// TypeScript equivalent:
// function isEven(n: number): boolean {
//   return n % 2 === 0;
// }
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;

    // TypeScript equivalent:
    // describe('isEven', () => {
    //   it('should correctly identify even numbers', () => {
    //     expect(isEven(2)).toBe(true);
    //     expect(isEven(3)).toBe(false);
    //   });
    // });
    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(2));
        assert!(!is_even(3));
    }
}
