mod network {
    fn connect() {

    }
}

mod client {
    fn connect() {
    }
}

// Two completely separate functions, called network::connect() and
// client::connect()

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
