#[macro_export]
macro_rules! tree {
    ($value:expr) => {
        DynTree::new($value)
    };
    ($value:expr, $($child:expr),*) => {
        {
            let mut n = DynTree::new($value);
            let mut root = n.root_mut();
            $(root.push_child_tree($child);)*
            n
        }
    };
    ($value:expr ; $($iter:expr),*) => {
        {
            let mut n = DynTree::new($value);
            let mut root = n.root_mut();
            $(for child in $iter {
                root.push_child_tree(child);
            })*
            n
        }
    };
    () => {};
}
