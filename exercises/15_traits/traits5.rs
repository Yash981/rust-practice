// In Rust, traits are similar to interfaces in TypeScript
// TypeScript equivalent:
// interface SomeTrait {
//   someFunction(): boolean;
// }

trait SomeTrait {
    // This is a default implementation, similar to default methods in TypeScript interfaces
    // TypeScript equivalent:
    // interface SomeTrait {
    //   someFunction(): boolean;
    //   // Default implementation in TypeScript would be:
    //   // someFunction(): boolean { return true; }
    // }
    fn some_function(&self) -> bool {
        true
    }
}

// TypeScript equivalent:
// interface OtherTrait {
//   otherFunction(): boolean;
//   // Default implementation:
//   // otherFunction(): boolean { return true; }
// }
trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

// TypeScript equivalent:
// class SomeStruct implements SomeTrait, OtherTrait {}
struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

// TypeScript equivalent:
// class OtherStruct implements SomeTrait, OtherTrait {}
struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// This function uses multiple trait bounds (SomeTrait + OtherTrait)
// TypeScript equivalent:
// function someFunc<T extends SomeTrait & OtherTrait>(item: T): boolean {
//   return item.someFunction() && item.otherFunction();
// }
// TODO: Fix the compiler error by only changing the signature of this function.
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TypeScript equivalent:
    // describe('Trait Tests', () => {
    //   it('should test someFunc with different structs', () => {
    //     expect(someFunc(new SomeStruct())).toBe(true);
    //     expect(someFunc(new OtherStruct())).toBe(true);
    //   });
    // });
    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
