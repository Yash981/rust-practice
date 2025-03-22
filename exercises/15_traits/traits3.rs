// In Rust, traits are similar to interfaces in TypeScript
// TypeScript equivalent:
// interface Licensed {
//   licensingInfo(): string;
// }

trait Licensed {
    // TODO: Add a default implementation for `licensing_info` so that
    // implementors like the two structs below can share that default behavior
    // without repeating the function.
    // The default license information should be the string "Default license".

    
    // This is a default implementation, similar to default methods in TypeScript interfaces
    // TypeScript equivalent:
    // interface Licensed {
    //   licensingInfo(): string;
    //   // Default implementation in TypeScript would be:
    //   // licensingInfo(): string { return "Default license"; }
    // }
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

// Implementing a trait for String is like extending an interface in TypeScript
// TypeScript equivalent:
// class String implements Licensed {
//   licensingInfo(): string { return "Default license"; }
// }
impl Licensed for String {
    fn licensing_info(&self) -> String {
        String::from("Default license")
    } 
}

// These structs are like classes in TypeScript
// TypeScript equivalent:
// class SomeSoftware implements Licensed {
//   versionNumber: number;
//   constructor(versionNumber: number) {
//     this.versionNumber = versionNumber;
//   }
// }
struct SomeSoftware {
    version_number: i32,
}

// TypeScript equivalent:
// class OtherSoftware implements Licensed {
//   versionNumber: string;
//   constructor(versionNumber: string) {
//     this.versionNumber = versionNumber;
//   }
// }
struct OtherSoftware {
    version_number: String,
}

// Empty implementations use the default trait method
// In TypeScript, this would be automatic when implementing an interface with default methods
impl Licensed for SomeSoftware {} // Don't edit this line.
impl Licensed for OtherSoftware {} // Don't edit this line.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
