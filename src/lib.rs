//! Library to generate a changeset.
//!
//! # Usage
//!
//! Add dependency to Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! changeset = "0.1"
//! ```
//!
//! And in your main.rs or lib.rs:
//!
//! ```ignore
//! #[macro_use]
//! extern crate changeset;
//! ```
//!
//! # Exemple
//!
//! ```ignore
//! changeset!(UserChangeSet {
//!     /// User's name
//!     name: String,
//!     age: usize
//! });
//! ```
//!
//! This will generate:
//!
//! ```
//! struct UserChangeSet {
//!     /// User's name
//!     pub name: Option<String>,
//!     pub age: Option<usize>,
//! }
//!
//! impl UserChangeSet {
//!     /// Some doc here
//!     pub fn new() -> UserChangeSet {
//!         UserChangeSet {
//!             name: None,
//!             age: None,
//!         }
//!     }
//!
//!     /// User's name
//!     pub fn name(mut self, name: String) -> UserChangeSet {
//!         self.name = Some(name);
//!         self
//!     }
//!
//!     pub fn age(mut self, age: usize) -> UserChangeSet {
//!         self.age = Some(age);
//!         self
//!     }
//!
//!     /// Some doc here
//!     pub fn merge(&mut self, rhs: UserChangeSet) {
//!         if let Some(name) = rhs.name {
//!             self.name = Some(name);
//!         }
//!         if let Some(age) = rhs.age {
//!             self.age = Some(age);
//!         }
//!     }
//!
//!     // I may add some new functions later
//! }
//! ```
//!
//! You can also generate public struct just by adding `pub` keyword.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_export]
macro_rules! changeset {
    (
        $( #[$attr:meta] )*
        pub $name:ident {
            $(
                $( #[$attrf:meta] )*
                $field:ident : $type:ty
            ),*
        }
    ) => {
        __changeset!(
            $(#[$attr])*
            (pub) $name {
                $(
                    $(#[$attrf])*
                    $field: $type
                ),*
            }
        );
    };

    (
        $( #[$attr:meta] )*
        $name:ident {
            $(
                $( #[$attrf:meta] )*
                $field:ident : $type:ty
            ),*
        }
    ) => {
        __changeset!(
            $(#[$attr])*
            () $name {
                $(
                    $(#[$attrf])*
                    $field: $type
                ),*
            }
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __changeset {
    (
        $( #[$attr:meta] )*
        ($($vis:tt)*) $name:ident {
            $(
                $( #[$attrf:meta] )*
                $field:ident : $type:ty
            ),*
        }
    ) => {
        $(#[$attr])*
        $($vis)* struct $name {
            $(
                $(#[$attrf])*
                pub $field : Option<$type>,
            )*
        }

        impl $name {
            /// Create a new changeset.
            pub fn new() -> $name {
                $name {
                    $(
                        $field: None,
                    )*
                }
            }

            $(
                $(#[$attrf])*
                pub fn $field<T: Into<$type>>(mut self, $field: T) -> $name {
                    self.$field = Some($field.into());
                    self
                }
            )*

            /// Merge with another changeset.
            pub fn merge(&mut self, rhs: $name) {
                $(
                    if let Some($field) = rhs.$field {
                        self.$field = Some($field);
                    }
                )*
            }

            /// Check if there is a value that has changed.
            pub fn has_changed(&self) -> bool {
                $(
                    if let Some(_) = self.$field {
                        return true;
                    }
                )*
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(dead_code)]
    mod in_mod {
        changeset!(pub PubStruct {
            name: String
        });
    }

    changeset!(PrivStruct {
        name: String
    });

    #[test]
    fn access_pub() {
        let s = in_mod::PubStruct::new();
        assert_eq!(s.name, None);
    }

    #[test]
    fn merge() {
        let mut a = PrivStruct::new().name("test".to_owned());
        let b = PrivStruct::new().name("success".to_owned());
        assert_eq!(a.name, Some("test".to_owned()));
        a.merge(b);
        assert_eq!(a.name, Some("success".to_owned()));
    }

    #[test]
    fn has_changed() {
        let mut a = PrivStruct::new();
        assert_eq!(a.has_changed(), false);
        a = a.name("success".to_owned());
        assert_eq!(a.has_changed(), true);
    }
}
