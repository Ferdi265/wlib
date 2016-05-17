use std::ops;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Copy, Clone)]
pub enum Pos {
    Left,
    Center,
    Right
}

#[derive(Copy, Clone)]
pub struct Corner {
    pub x: Pos,
    pub y: Pos
}

impl Corner {
    pub const TOP_LEFT: Corner = Corner { x: Pos::Left, y: Pos::Left };
    pub const TOP: Corner = Corner { x: Pos::Center, y: Pos::Left };
    pub const TOP_RIGHT: Corner = Corner { x: Pos::Right, y: Pos::Left };
    pub const LEFT: Corner = Corner { x: Pos::Left, y: Pos::Center };
    pub const CENTER: Corner = Corner { x: Pos::Center, y: Pos::Center };
    pub const RIGHT: Corner = Corner { x: Pos::Right, y: Pos::Center };
    pub const BOTTOM_LEFT: Corner = Corner { x: Pos::Left, y: Pos::Right };
    pub const BOTOOM: Corner = Corner { x: Pos::Center, y: Pos::Right };
    pub const BOTTOM_RIGHT: Corner = Corner { x: Pos::Right, y: Pos::Right };
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub w: u32,
    pub h: u32
}

impl Rectangle {
    pub fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {
            w: w,
            h: h
        }
    }
    pub fn pos_x(&self, p: Pos) -> i32 {
        match p {
            Pos::Left => 0 as i32,
            Pos::Center => (self.w / 2) as i32,
            Pos::Right => self.w as i32
        }
    }
    pub fn pos_y(&self, p: Pos) -> i32 {
        match p {
            Pos::Left => 0 as i32,
            Pos::Center => (self.h / 2) as i32,
            Pos::Right => self.h as i32
        }
    }
    pub fn corner(&self, c: Corner) -> Point {
        Point::new(self.pos_x(c.x), self.pos_y(c.y))
    }
    pub fn point_inside(&self, p: Point) -> bool {
        self.pos_x(Pos::Left) <= p.x && p.x <= self.pos_x(Pos::Right) &&
        self.pos_y(Pos::Left) <= p.y && p.y <= self.pos_y(Pos::Right)
    }
}

#[derive(Copy, Clone)]
pub struct PositionedRectangle {
    pub p: Point,
    pub r: Rectangle
}

impl PositionedRectangle {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> PositionedRectangle {
        PositionedRectangle {
            p: Point::new(x, y),
            r: Rectangle::new(w, h)
        }
    }
    pub fn pos_x(&self, p: Pos) -> i32 {
        self.r.pos_x(p) + self.p.x
    }
    pub fn pos_y(&self, p: Pos) -> i32 {
        self.r.pos_y(p) + self.p.y
    }
    pub fn corner(&self, c: Corner) -> Point {
        self.r.corner(c) + self.p
    }
    pub fn point_inside(&self, p: Point) -> bool {
        self.r.point_inside(p - self.p)
    }
    pub fn rectangle_inside(&self, r: PositionedRectangle) -> bool {
        self.point_inside(r.corner(Corner::TOP_LEFT)) &&
        self.point_inside(r.corner(Corner::BOTTOM_RIGHT))
    }
    pub fn rectangle_overlaps(&self, r: PositionedRectangle) -> bool {
        !(
            self.pos_x(Pos::Right) <= r.pos_x(Pos::Left) || r.pos_x(Pos::Right) <= self.pos_x(Pos::Left) ||
            self.pos_y(Pos::Right) <= r.pos_y(Pos::Left) || r.pos_y(Pos::Right) <= self.pos_y(Pos::Left)
        )
    }
}
