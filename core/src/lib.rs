use log::{info, trace, warn, error};

pub fn core()
{
    info!("Hello core!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
