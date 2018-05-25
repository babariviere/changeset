//! Library to generate a changeset.
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
//!     #[allow(missing_docs)]
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
//!     #[allow(missing_docs)]
//!     pub fn merge(&mut self, rhs: UserChangeSet) {
//!         if let Some(name) = rhs.name {
//!             self.name = Some(name);
//!         }
//!         if let Some(age) = rhs.age {
//!             self.age = Some(age);
//!         }
//!     }
//! }
//! ```
//!
//! You can also generate public struct just by adding `pub` keyword.

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
            #[allow(missing_docs)]
            pub fn new() -> $name {
                $name {
                    $(
                        $field: None,
                    )*
                }
            }

            $(
                $(#[$attrf])*
                pub fn $field(mut self, $field: $type) -> $name {
                    self.$field = Some($field);
                    self
                }
            )*

            #[allow(missing_docs)]
            pub fn merge(&mut self, rhs: $name) {
                $(
                    if let Some($field) = rhs.$field {
                        self.$field = Some($field);
                    }
                )*
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
}
