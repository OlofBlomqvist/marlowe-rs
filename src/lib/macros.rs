
#[macro_export]
#[doc(hidden)]
macro_rules! Impl_From_For {

    (@$x:ident,$y:ident) => {
        impl From<AstNode> for Result<$x,String> {
            fn from(x: AstNode) -> Self { 
                let expected = stringify!($x); 
                match x {
                    AstNode::$y(xx) => Ok(xx),
                    hmm => Err(format!("Expected {} (result), received: {:?}....",expected,hmm).to_string())
                }
            }
        }
        impl From<$x> for AstNode {
            fn from(x: $x) -> Self {
                AstNode::$y(x)
            }
        }
        impl From<&$x> for AstNode {
            fn from(x: &$x) -> Self {
                AstNode::$y(x.clone())
            }
        }
        impl From<&Box<$x>> for AstNode {
            fn from(x: &Box<$x>) -> Self {
                x.to_owned().into()
            }
        }
        impl From<Box<$x>> for AstNode {
            fn from(x: Box<$x>) -> Self {
                let actual : $x = *x;
                AstNode::$y(actual)
            }
        }
        impl TryFrom<AstNode> for $x {
            type Error = String;
            fn try_from(x: AstNode) -> std::result::Result<Self,Self::Error> {
                let expected = stringify!($x);
                match x {
                    AstNode::$y(xx) => Ok(xx),
                    hmm => Err(format!("Expected {}, received: {:?}...",expected,hmm).to_string())
                }
            }
        }
        impl TryFrom<AstNode> for Option<$x> {
            type Error = String;
            fn try_from(a: AstNode) -> std::result::Result<Self,Self::Error> {
                let expected = stringify!($x);
                match a {
                    AstNode::$y(xx) => Ok(Some(xx)),
                    AstNode::Null => Ok(None),
                    hmm => Err(format!("Expected {} (option), received: {:?}..",expected,hmm).to_string())
                }
            }
        }
        impl TryFrom<AstNode> for Option<Box<$x>> {
            type Error = String;
            fn try_from(a: AstNode) -> std::result::Result<Self,Self::Error> {
                let expected = stringify!($x);
                match a {
                    AstNode::$y(b) => Ok(Some(Box::new(b))),
                    AstNode::Null => Ok(None),
                    hmm => Err(format!("Expected {} (option,boxed), received: {:?}.",expected,hmm).to_string())
                }
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! Impl_From_For_Vec {
    (@$vectype:ident,@$innerwrappertype:ident,@$innertype:ident) => {
        impl TryFrom<AstNode> for Vec<Option<$innertype>> {
            type Error = String;
            fn try_from(x: AstNode) -> Result<Vec<Option<$innertype>>,Self::Error> { match x {
                AstNode::$vectype(items) => {
                    let mut result = vec![];
                    for item in items {
                        match item {
                            AstNode::$innerwrappertype(x) => result.push(Some(x)),
                            AstNode::Null => result.push(None),
                            xxx => {
                                let expected = stringify!($innerwrappertype);
                                return Err(format!("Expected AstNode::{expected}, found: {xxx:?}"))
                            }
                        }
                    };
                    Ok(result)
                },
                xx => Err(format!("Expected array, found: {xx:?}"))
            }}
        }
    }
}

