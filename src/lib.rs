pub mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();   // I think this will work
    outermost::middle_secret_function();    // this will too (wrong)
    // middle_secret_function can still be used by outermost, but we are
    // outside of outermost and cannot use it.
    outermost::inside::inner_function();    // this will not
    outermost::inside::secret_function();   // nor this
}
