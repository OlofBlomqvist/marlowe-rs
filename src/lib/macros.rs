mod macros {
    
    #[macro_export]
    #[doc(hidden)]
    macro_rules! Impl_From_For {

        (@$x:ident,$y:ident) => {
            impl From<AstNode> for $x {
                fn from(x: AstNode) -> $x {
                    let expected = stringify!($x);
                    match x {
                        AstNode::$y(xx) => xx,
                        y => panic!("Expected {}, received: {:?}",expected,y)
                    }
                }
            }
        }
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! Impl_From_For_Vec {
        (@$vectype:ident,@$innerwrappertype:ident,@$innertype:ident) => {
            impl From<AstNode> for Vec<$innertype> {
                fn from(x: AstNode) -> Vec<$innertype> { match x {
                    AstNode::$vectype(items) => {
                        let mut result = vec![];
                        for item in items {
                            match item {
                                AstNode::$innerwrappertype(x) => result.push(x),
                                xxx => {
                                    let expected = stringify!($innertype);
                                    unreachable!("Expected {expected} , received: {:?}",xxx)
                                }
                            }
                        };
                        result
                    },
                    _ => panic!()}}
            }
        }
    }

}