// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::borrow;
use std::cmp::Ordering;
use std::fmt;

use serde::{de, ser};

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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl From<&Key> for Key {
    fn from(key: &Key) -> Self {
        key.clone()
    }
}

impl From<String> for Key {
    fn from(key: String) -> Self {
        Self::new(key)
    }
}

impl From<&str> for Key {
    fn from(key: &str) -> Self {
        Self::new(key)
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ser::Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let key = String::deserialize(deserializer)?;
        Ok(Self::new(key))
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

    /// Return a random (uuid) generated key,
    /// like Key::random() == Key::new("/f98719ea086343f7b71f32ea9d9d521d").
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use plum_ipfs_datastore::Key;
    ///  let mut keys = HashSet::with_capacity(1000);
    ///  for _ in 0..1000 {
    ///     keys.insert(Key::random());
    /// }
    /// assert_eq!(keys.len(), 1000);
    /// ```
    pub fn random() -> Self {
        let uuid = uuid::Uuid::new_v4();
        Self::new(uuid.to_string().replace("-", ""))
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

    /// Return the string slice of this key's content.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
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
        if list.is_empty() || list.len() == 1 {
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
    /// let child = Key::new("/").child("Child");
    /// assert_eq!(child, Key::new("/Child"));
    /// let child = Key::new("/Comedy/MontyPython").child("Actor:JohnCleese");
    /// assert_eq!(child, Key::new("/Comedy/MontyPython/Actor:JohnCleese"));
    /// ```
    pub fn child<K: Into<Key>>(&self, child: K) -> Self {
        let child = child.into();
        if self.0 == SLASH {
            Self::new(child)
        } else if child.as_str() == SLASH {
            self.clone()
        } else {
            unsafe { Key::new_unchecked(format!("{}{}", self.0, child)) }
        }
    }

    /// Return whether this key is a prefix of `descendant`.
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// assert!(Key::new("/Comedy").is_ancestor_of("/Comedy/MontyPython"));
    /// assert!(!Key::new("/A").is_ancestor_of("/AB"));
    /// ```
    pub fn is_ancestor_of<K: Into<Key>>(&self, descendant: K) -> bool {
        let descendant = descendant.into();
        if descendant.as_str().len() <= self.as_str().len() {
            return false;
        }
        if self.as_str() == SLASH {
            return true;
        }

        // start with 'ancestor.as_str() + /'
        descendant.as_str().starts_with(self.as_str())
            && descendant.as_bytes()[self.as_str().len()] == b'/'
    }

    /// Return whether this key contains another as a prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// assert!(Key::new("/Comedy/MontyPython").is_descendant_of("/Comedy"));
    /// assert!(!Key::new("/AB").is_descendant_of("/A"));
    /// ```
    pub fn is_descendant_of<K: Into<Key>>(&self, ancestor: K) -> bool {
        ancestor.into().is_ancestor_of(self)
    }

    /// Return whether this key has only one namespace.
    ///
    /// # Example
    ///
    /// ```
    /// use plum_ipfs_datastore::Key;
    /// assert!(Key::new("/").is_top_level());
    /// assert!(Key::new("/Comedy").is_top_level());
    /// assert!(!Key::new("/Comedy/MontyPython").is_top_level());
    /// ```
    pub fn is_top_level(&self) -> bool {
        self.list().is_empty() || self.list().len() == 1
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

#[cfg(test)]
mod tests {
    use super::Key;

    #[test]
    fn test_ancestry() {
        let k1 = Key::new("/A/B/C");
        let k2 = Key::new("/A/B/C/D");
        let k3 = Key::new("/AB");
        let k4 = Key::new("/A");

        assert_eq!(k1.to_string(), "/A/B/C");
        assert_eq!(k2.to_string(), "/A/B/C/D");

        assert!(k1.is_ancestor_of(&k2));
        assert!(k2.is_descendant_of(&k1));
        assert!(k4.is_ancestor_of(&k1));
        assert!(k4.is_ancestor_of(&k2));

        assert!(!k4.is_descendant_of(&k2));
        assert!(!k4.is_descendant_of(&k1));
        assert!(!k3.is_descendant_of(&k4));
        assert!(!k4.is_ancestor_of(&k3));

        assert!(k2.is_descendant_of(&k4));
        assert!(k1.is_descendant_of(&k4));

        assert!(!k2.is_ancestor_of(&k4));
        assert!(!k1.is_ancestor_of(&k4));
        assert!(!k2.is_ancestor_of(&k2));
        assert!(!k1.is_ancestor_of(&k1));

        assert_eq!(k1.child("D"), k2);
        assert_eq!(k1, k2.parent());
        assert_eq!(k1.path(), k2.parent().path());
    }

    #[test]
    fn test_less() {
        fn assert_less<A: Into<Key>, B: Into<Key>>(a: A, b: B) {
            let a = a.into();
            let b = b.into();
            assert!(a < b);
            assert!(!(b < a));
        }

        assert_less("/a/b/c", "/a/b/c/d");
        assert_less("/a/b", "/a/b/c/d");
        assert_less("/a", "/a/b/c/d");
        assert_less("/a/a/c", "/a/b/c");
        assert_less("/a/a/d", "/a/b/c");
        assert_less("/a/b/c/d/e/f/g/h", "/b");
        assert_less("/", "/a");
    }

    #[test]
    fn test_json() {
        struct Case {
            key: Key,
            data: Vec<u8>,
        }

        let cases = vec![
            Case {
                key: Key::new("/a/b/c"),
                data: b"\"/a/b/c\"".to_vec(),
            },
            Case {
                key: Key::new("/shouldescapekey\"/with/quote"),
                data: b"\"/shouldescapekey\\\"/with/quote\"".to_vec(),
            },
            Case {
                key: Key::new(""),
                data: b"\"/\"".to_vec(),
            },
        ];

        for case in cases {
            let ser = serde_json::to_vec(&case.key).unwrap();
            assert_eq!(ser, case.data);
            let key = serde_json::from_slice::<Key>(&ser).unwrap();
            assert_eq!(key, case.key);
        }
    }
}
