#[macro_export]
macro_rules! p {
    ($e:expr) => { println!(concat!(stringify!($e), ": {:?}"), {&$e}); };
}

#[macro_export]
macro_rules! pp {
    ($i:expr, $e:expr) => { println!(concat!(stringify!($i), ": {:?}"), {&$e}); };
}

#[macro_export]
macro_rules! tk { ($kind:pat) => { Tree::Leaf(Annot{value: $kind, ..}) } }

