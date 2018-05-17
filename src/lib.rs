mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    // let's make this public so it inner_function can be called
    pub mod inside {
        pub fn inner_function() {
            // I think this will work because we are in a child of outermost and
            // middle_secret_function can be accessed from outermost.
            // But there might be shenanigans because we went to the root (::)
            ::outermost::middle_secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();   // I think this will work
    outermost::middle_secret_function();    // this will not
    // middle_secret_function can be used by outermost, but we are
    // outside of outermost and cannot use it.
    outermost::inside::inner_function();    // this will now work
    outermost::inside::secret_function();   // this will not
}
