// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow;
use std::cmp::Ordering;
use std::fmt;

use serde::{Deserialize, Serialize};

/// A Key represents the unique identifier of an object.
/// Our Key scheme is inspired by file systems and Google App Engine key model.
///
/// Keys are meant to be unique across a system. Keys are hierarchical,
/// incorporating more and more specific namespaces.
/// Thus keys can be deemed 'children' or 'ancestors' of other keys::
///
/// Key("/Comedy")
/// Key("/Comedy/MontyPython")
///
/// Also, every namespace can be parametrized to embed relevant object information.
/// For example, the Key `name` (most specific namespace) could include the object type::
///
/// Key("/Comedy/MontyPython/Actor:JohnCleese")
/// Key("/Comedy/MontyPython/Sketch:CheeseShop")
/// Key("/Comedy/MontyPython/Sketch:CheeseShop/Character:Mousebender")
///
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Key(String);

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        let list1 = self.list();
        let list2 = other.list();
        list1.cmp(&list2)
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const SLASH: &str = "/";
const COLON: &str = ":";

// Ensure the `s` is start with "/" and apply the rule of `path_clean`.
fn clean<S: AsRef<str>>(s: S) -> String {
    let path = s.as_ref();
    if path.is_empty() {
        SLASH.to_owned()
    } else if path.starts_with(SLASH) {
        path_clean::clean(path)
    } else {
        let mut path_buf = String::with_capacity(SLASH.len() + path.len());
        path_buf.push_str(SLASH);
        path_buf.push_str(path);
        path_clean::clean(&path_buf)
    }
}

impl Key {
    /// Create a new key from a string.
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let key = clean(s);
        Self(key)
    }

    /// Create a new key without safety checking the input.
    ///
    /// # Safety
    ///
    /// The key should start with "/" and shouldn't end with "/".
    /// Specially, "/" is valid key.
    ///
    pub unsafe fn new_unchecked<S: AsRef<str>>(s: S) -> Self {
        let input = s.as_ref();

        // accept an empty string and fix it to avoid special cases elsewhere
        if input.is_empty() {
            return Self(SLASH.to_owned());
        }

        // perform a quick sanity check that the key is in the correct format,
        // if it is not then it is a programmer error and it is okay to panic
        if !input.starts_with(SLASH) || (input.len() > 1 && input.ends_with(SLASH)) {
            panic!("invalid datastore key: {}", input);
        }

        Self(input.to_owned())
    }

    /// Create a key out of a namespace slice.
    pub fn with_namespaces<I, T>(namespaces: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: borrow::Borrow<str>,
    {
        let key = namespaces
            .into_iter()
            .map(|namespace| namespace)
            .collect::<Vec<_>>()
            .join(SLASH);
        Self::new(key)
    }

    /// Clean up a key, ensure the key is start with "/" and apply the rule of `path_clean`.
    pub fn clean(&mut self) {
        self.0 = clean(&self.0);
    }

    /// Return the byte slice of this key's content.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Return the reverse of this key.
    pub fn reverse(&self) -> Self {
        let mut namespaces = self.list();
        namespaces.reverse();
        Self::with_namespaces(namespaces)
    }

    /// Return the `list` representation of this key.
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let key = Key::new("/");
    /// assert_eq!(key.list(), Vec::<&str>::new());
    /// let key = Key::new("/Comedy/MontyPython/Actor:JohnCleese");
    /// assert_eq!(key.list(), vec!["Comedy", "MontyPython", "Actor:JohnCleese"]);
    /// ```
    pub fn list(&self) -> Vec<&str> {
        if self.0 == SLASH {
            vec![]
        } else {
            self.0.split(SLASH).skip(1).collect::<Vec<_>>()
        }
    }

    /// Return the `namespaces` making up this key.
    /// It's the same as `list` method.
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let key = Key::new("/");
    /// assert_eq!(key.namespaces(), Vec::<&str>::new());
    /// let key = Key::new("/Comedy/MontyPython/Actor:JohnCleese");
    /// assert_eq!(key.namespaces(), vec!["Comedy", "MontyPython", "Actor:JohnCleese"]);
    /// ```
    pub fn namespaces(&self) -> Vec<&str> {
        self.list()
    }

    /// Return the "base" namespace of this key
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let key = Key::new("/");
    /// assert_eq!(key.base_namespace(), "");
    /// let key = Key::new("/Comedy/MontyPython/Actor:JohnCleese");
    /// assert_eq!(key.base_namespace(), "Actor:JohnCleese");
    /// ```
    pub fn base_namespace(&self) -> &str {
        self.namespaces().last().unwrap_or(&"")
    }

    /// Return the "type" of this key (value of last namespace).
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let key = Key::new("/");
    /// assert_eq!(key.r#type(), "");
    /// let key = Key::new("/Comedy/MontyPython/Actor:JohnCleese");
    /// assert_eq!(key.r#type(), "Actor");
    /// ```
    pub fn r#type(&self) -> &str {
        namespace_type(self.base_namespace())
    }

    /// Return the "name" of this key (field of last namespace).
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let key = Key::new("/");
    /// assert_eq!(key.name(), "");
    /// let key = Key::new("/Comedy/MontyPython/Actor:JohnCleese");
    /// assert_eq!(key.name(), "JohnCleese");
    /// ```
    pub fn name(&self) -> &str {
        namespace_value(self.base_namespace())
    }

    /// Return an "instance" of this type key (appends value to namespace).
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let instance = Key::new("/Comedy/MontyPython/Actor").instance("JohnCleese");
    /// assert_eq!(instance, Key::new("/Comedy/MontyPython/Actor:JohnCleese"));
    /// ```
    pub fn instance<S: AsRef<str>>(&self, s: S) -> Self {
        let key = format!("{}{}{}", self.0, COLON, s.as_ref());
        Self::new(key)
    }

    /// Return the "path" of this key (parent + type).
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let path = Key::new("/Comedy/MontyPython/Actor:JohnCleese").path();
    /// assert_eq!(path, Key::new("/Comedy/MontyPython/Actor"));
    /// ```
    pub fn path(&self) -> Self {
        let key = format!("{}{}{}", self.parent(), SLASH, self.r#type());
        Self::new(key)
    }

    /// Return the "parent" of this key.
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let parent = Key::new("/").parent();
    /// assert_eq!(parent, Key::new("/"));
    /// let parent = Key::new("/Comedy/MontyPython/Actor:JohnCleese").parent();
    /// assert_eq!(parent, Key::new("/Comedy/MontyPython"));
    /// ```
    pub fn parent(&self) -> Self {
        let mut list = self.list();
        if list.len() == 0 || list.len() == 1 {
            Self::new(SLASH)
        } else {
            list.pop(); // pop the last namespace
            let key = list.join(SLASH);
            Self::new(key)
        }
    }

    /// Return the `child` key of this key.
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// let child = Key::new("/").child(Key::new("Child"));
    /// assert_eq!(child, Key::new("/Child"));
    /// let child = Key::new("/Comedy/MontyPython").child(Key::new("Actor:JohnCleese"));
    /// assert_eq!(child, Key::new("/Comedy/MontyPython/Actor:JohnCleese"));
    /// ```
    pub fn child<K: AsRef<str>>(&self, key: K) -> Self {
        if self.0 == SLASH {
            Self::new(key)
        } else if key.as_ref() == SLASH {
            self.clone()
        } else {
            unsafe { Key::new_unchecked(format!("{}{}", self.0, key.as_ref())) }
        }
    }
}

/// Return the first component of a namespace, like `foo` in `foo:bar`.
///
/// # Example
///
/// ```
/// use plum_ipfs_datastore::namespace_type;
/// assert_eq!(namespace_type("foo:bar"), "foo");
/// ```
pub fn namespace_type(namespace: &str) -> &str {
    namespace
        .rfind(COLON)
        .map(|i| namespace.split_at(i).0)
        .unwrap_or(&"")
}

/// Return the last component of a namespace, like `baz` in `f:b:baz`.
///
/// # Example
///
/// ```
/// use plum_ipfs_datastore::namespace_value;
/// assert_eq!(namespace_value("f:b:baz"), "baz");
/// ```
pub fn namespace_value(namespace: &str) -> &str {
    namespace
        .rfind(COLON)
        .map(|i| namespace.split_at(i + 1).1)
        .unwrap_or(namespace)
}
