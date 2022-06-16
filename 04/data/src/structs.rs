#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}
