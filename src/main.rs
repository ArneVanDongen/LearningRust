#[derive(Debug)]
struct Position {
    x: i8,
    y: i8,
}

impl Position {
    fn update_x(&mut self) {
        self.x += 1;
        self.update_y();
    }
    fn update_y(&mut self) {
        self.y -= 10;
    }
}

fn main() {
    let mut pos = Position { x: 42, y: 13 };
    println!("Created position: {:?}", pos);
    pos.update_x();
    println!("updated position: {:?}", pos);
    Position::update_y(&mut pos);
    println!("updated position: {:?}", pos);

}
