pub mod network;

pub mod client;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect()   // Will not work
    }
}
