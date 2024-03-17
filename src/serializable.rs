// use std::fmt;

pub trait Serializable {
    fn serialized_object_name(&self) -> String;
}

// impl<T: fmt::Display> fmt::Display for Vec<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[")?;
//         for (i, item) in self.iter().enumerate() {
//             if i > 0 {
//                 write!(f, " ")?;
//             }
//             write!(f, "{}", item)?;
//         }
//         write!(f, " ]")
//     }
// }
