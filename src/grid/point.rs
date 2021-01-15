//! Main implementation of Point functionality
//! Current focus on reusability with Grid Style problems

#[derive(Debug, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Eq for Point {}

impl Point {
    pub fn new(x: i32, y:i32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn update(&mut self, x: i32, y:i32) {
        self.x = x;
        self.y = y;
    }

    fn in_bounds(&self, xmax: i32, ymax: i32) -> bool {
        self.x >= 0 && self.x < xmax && self.y >= 0 && self.y < ymax
    }

    fn get_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn get_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn get_up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn get_down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn get_neighbours(&self) -> [Point; 4] {
        [self.get_right(), self.get_left(), self.get_up(), self.get_down()]
    }

    // supports only 4 main directions
    fn get_direction(&self, other: &Point) -> &str {
        let dir = *other - *self;
        if dir.x < 0 {
            return "LEFT";
        }
        else if dir.x > 0 {
            return "RIGHT";
        }
        else if dir.y < 0 {
            return "UP";
        }
        else {
            return "DOWN";
        }
    }
}