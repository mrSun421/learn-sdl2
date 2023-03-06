use sdl2::pixels::Color;

pub const SCREEN_WIDTH: usize = 120;
pub const SCREEN_HEIGHT: usize = 90;

pub struct RenderState {
    screen: [Color; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            screen: [Color::RGB(0, 0, 0); SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn get_display(&self) -> &[Color] {
        &self.screen
    }

    pub fn draw_to_center(&mut self, color: Color) {
        self.draw_color_to_coordinate(CartesianCoordinates { x: 0, y: 0 }, color);
    }

    pub fn draw_color_to_coordinate(&mut self, coordinate: CartesianCoordinates, color: Color) {
        let (screen_x, screen_y) = coordinate.convert_to_screen_coordinates();
        let idx = screen_y * SCREEN_WIDTH + screen_x;
        self.screen[idx] = color;
    }
}

pub struct CartesianCoordinates {
    x: usize,
    y: usize,
}

impl CartesianCoordinates {
    fn convert_to_screen_coordinates(&self) -> (usize, usize) {
        let x_screen_coordinate = SCREEN_WIDTH / 2 + self.x;
        let y_screen_coordinate = SCREEN_HEIGHT / 2 - self.y;
        return (x_screen_coordinate, y_screen_coordinate);
    }
}
