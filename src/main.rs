pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    use a::series::of::nested_modules;

    nested_modules();
}
