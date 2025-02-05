pub fn run() {
    println!("default status {:?}", Status::default());
}

#[derive(Debug)]
enum Status {
    Open,
    Closed,
}

impl Status {
    fn default() -> Self {
        Status::Open
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Closed
    }
}
