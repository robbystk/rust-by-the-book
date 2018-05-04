mod network {
    fn connect() {

    }

    mod client {
        fn connect() {
        }
    }
}

// Now called network::connect() and network::client::connect()

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
