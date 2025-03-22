// In Rust, traits are similar to interfaces in TypeScript
// TypeScript equivalent:
// interface Licensed {
//   licensingInfo(): string;
// }

trait Licensed {
    // This is a default implementation, similar to default methods in TypeScript interfaces
    // TypeScript equivalent:
    // interface Licensed {
    //   licensingInfo(): string;
    //   // Default implementation in TypeScript would be:
    //   // licensingInfo(): string { return "Default license"; }
    // }
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

// These are like classes in TypeScript
// TypeScript equivalent:
// class SomeSoftware implements Licensed {}
struct SomeSoftware;
// TypeScript equivalent:
// class OtherSoftware implements Licensed {}
struct OtherSoftware;

// Empty implementations use the default trait method
// In TypeScript, this would be automatic when implementing an interface with default methods
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}


// This function uses trait bounds (impl Licensed)
// TypeScript equivalent:
// function compareLicenseTypes(software1: Licensed, software2: Licensed): boolean {
//   return software1.licensingInfo() === software2.licensingInfo();
// }
// TODO: Fix the compiler error by only changing the signature of this function.
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TypeScript equivalent:
    // describe('License Information', () => {
    //   it('should compare license information', () => {
    //     expect(compareLicenseTypes(new SomeSoftware(), new OtherSoftware())).toBe(true);
    //   });
    // });
    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    // TypeScript equivalent:
    // it('should compare license information backwards', () => {
    //   expect(compareLicenseTypes(new OtherSoftware(), new SomeSoftware())).toBe(true);
    // });
    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
