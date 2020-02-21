use ggez::graphics::{DrawMode, FillOptions, MeshBuilder, Rect, Color, WHITE};

type FPoint = (f32, f32);

fn floor(point: FPoint) -> FPoint {
    (point.0.floor(), point.1.floor())
}

#[derive(Debug)]
pub struct TextMeshBuilder<'a> {
    pub color: Color,
    point: FPoint,
    mesh_builder: &'a mut MeshBuilder,
}

impl<'a> TextMeshBuilder<'a> {
    pub fn new(mesh_builder: &'a mut MeshBuilder) -> Self {
        Self {
            color: WHITE,
            point: (0.0, 0.0),
            mesh_builder,
        }
    }

    pub fn with_color(color: Color, mesh_builder: &'a mut MeshBuilder) -> Self {
        Self {
            color,
            point: (0.0, 0.0),
            mesh_builder,
        }
    }

    pub fn write(&mut self, text: &str, point: FPoint) -> &mut Self {
        self.point = floor(point);

        for c in text.to_ascii_uppercase().chars() {
            match c {
                'A' => self.a(),
                'B' => self.b(),
                'C' => self.c(),
                'D' => self.d(),
                'E' => self.e(),
                'F' => self.f(),
                'G' => self.g(),
                'H' => self.h(),
                'I' => self.i(),
                'J' => self.j(),
                'K' => self.k(),
                'L' => self.l(),
                'M' => self.m(),
                'N' => self.n(),
                'O' => self.o(),
                'P' => self.p(),
                'Q' => self.q(),
                'R' => self.r(),
                'S' => self.s(),
                'T' => self.t(),
                'U' => self.u(),
                'V' => self.v(),
                'W' => self.w(),
                'X' => self.x(),
                'Y' => self.y(),
                'Z' => self.z(),
                '1' => self.one(),
                '2' => self.two(),
                '3' => self.three(),
                '4' => self.four(),
                '5' => self.five(),
                '6' => self.six(),
                '7' => self.seven(),
                '8' => self.eight(),
                '9' => self.nine(),
                '0' => self.zero(),
                '-' => self.hyphen(),
                '_' => self.underscore(),
                '=' => self.equals(),
                '+' => self.plus(),
                '[' => self.l_sqr_bracket(),
                '{' => self.l_crl_bracket(),
                ']' => self.r_sqr_bracket(),
                '}' => self.r_crl_bracket(),
                '\\' => self.back_slash(),
                '|' => self.vertical_line(),
                ';' => self.semicolon(),
                ':' => self.colon(),
                '\'' => self.apostrophe(),
                '\"' => self.quotation(),
                ' ' => self.shrink(2),
                ',' => self.comma(),
                '<' => self.less_than(),
                '.' => self.period(),
                '>' => self.greater_than(),
                '/' => self.fwd_slash(),
                '?' => self.question(),
                '!' => self.exclamation(),
                '@' => self.at(),
                '#' => self.octothorpe(),
                '$' => self.dollar(),
                '%' => self.percent(),
                '^' => self.caret(),
                '&' => self.ampersand(),
                '*' => self.star(),
                '(' => self.l_round_bracket(),
                ')' => self.r_round_bracket(),
                '`' => self.accent(),
                '~' => self.tilde(),
                '\n' => {
                    self.point.0 = point.0.floor() - 8.0;
                    self.point.1 += 9.0;
                }
                _ => self.invalid(),
            }

            self.point.0 += 8.0;
        }

        self
    }

    fn a(&mut self) {
        self.v_line(0, 1, 4);
        self.v_line(4, 1, 4);
        self.h_line(0, 1, 3);
        self.h_line(2, 1, 3);
    }

    fn b(&mut self) {
        self.v_line(0, 0, 4);
        self.pixel(4, 1);
        self.pixel(4, 3);
        self.h_line(0, 0, 3);
        self.h_line(2, 0, 3);
        self.h_line(4, 0, 3);
    }

    fn c(&mut self) {
        self.v_line(0, 1, 3);
        self.h_line(0, 1, 4);
        self.h_line(4, 1, 4);
    }

    fn d(&mut self) {
        self.v_line(0, 0, 4);
        self.v_line(4, 1, 3);
        self.h_line(0, 1, 3);
        self.h_line(4, 1, 3);
    }

    fn e(&mut self) {
        self.v_line(0, 0, 4);
        self.h_line(0, 1, 4);
        self.h_line(2, 1, 2);
        self.h_line(4, 1, 4);
    }

    fn f(&mut self) {
        self.v_line(0, 0, 4);
        self.h_line(0, 1, 4);
        self.h_line(2, 1, 2);
    }

    fn g(&mut self) {
        self.v_line(0, 1, 3);
        self.h_line(0, 1, 4);
        self.h_line(4, 1, 4);
        self.h_line(2, 3, 4);
        self.pixel(4, 3);
    }

    fn h(&mut self) {
        self.v_line(0, 0, 4);
        self.v_line(4, 0, 4);
        self.h_line(2, 1, 3);
    }

    fn i(&mut self) {
        self.v_line(0, 0, 4);
        self.shrink(4);
    }

    fn j(&mut self) {
        self.pixel(0, 3);
        self.v_line(4, 0, 3);
        self.h_line(4, 1, 3);
    }

    fn k(&mut self) {
        self.v_line(0, 0, 4);
        self.h_line(2, 1, 2);
        self.pixel(3, 1);
        self.pixel(4, 0);
        self.pixel(3, 3);
        self.pixel(4, 4);
    }

    fn l(&mut self) {
        self.v_line(0, 0, 4);
        self.h_line(4, 1, 4);
    }

    fn m(&mut self) {
        self.v_line(0, 0, 4);
        self.v_line(4, 0, 4);
        self.pixel(1, 1);
        self.pixel(2, 2);
        self.pixel(3, 1);
    }

    fn n(&mut self) {
        self.v_line(0, 0, 4);
        self.v_line(4, 0, 4);
        self.pixel(1, 1);
        self.pixel(2, 2);
        self.pixel(3, 3);
    }

    fn o(&mut self) {
        self.v_line(0, 1, 3);
        self.v_line(4, 1, 3);
        self.h_line(0, 1, 3);
        self.h_line(4, 1, 3);
    }

    fn p(&mut self) {
        self.v_line(0, 0, 4);
        self.h_line(0, 1, 3);
        self.h_line(2, 1, 3);
        self.pixel(4, 1);
    }

    fn q(&mut self) {
        self.o();
        self.pixel(4, 5);
    }

    fn r(&mut self) {
        self.v_line(0, 0, 4);
        self.pixel(4, 1);
        self.v_line(4, 3, 4);
        self.h_line(0, 1, 3);
        self.h_line(2, 1, 3);
    }

    fn s(&mut self) {
        self.h_line(0, 1, 4);
        self.pixel(0, 1);
        self.h_line(2, 1, 3);
        self.pixel(4, 3);
        self.h_line(4, 0, 3);
    }

    fn t(&mut self) {
        self.v_line(2, 1, 4);
        self.h_line(0, 0, 4);
    }

    fn u(&mut self) {
        self.v_line(0, 0, 3);
        self.v_line(4, 0, 3);
        self.h_line(4, 1, 3);
    }

    fn v(&mut self) {
        self.v_line(0, 0, 2);
        self.v_line(4, 0, 2);
        self.pixel(1, 3);
        self.pixel(2, 4);
        self.pixel(3, 3);
    }

    fn w(&mut self) {
        self.v_line(0, 0, 4);
        self.v_line(4, 0, 4);
        self.pixel(1, 3);
        self.pixel(3, 3);
        self.pixel(2, 2);
    }

    fn x(&mut self) {
        self.pixel(0, 0);
        self.pixel(1, 1);
        self.pixel(2, 2);
        self.pixel(3, 3);
        self.pixel(4, 4);
        self.pixel(0, 4);
        self.pixel(1, 3);
        self.pixel(3, 1);
        self.pixel(4, 0);
    }

    fn y(&mut self) {
        self.pixel(0, 0);
        self.pixel(1, 1);
        self.v_line(2, 2, 4);
        self.pixel(3, 1);
        self.pixel(4, 0);
    }

    fn z(&mut self) {
        self.h_line(0, 0, 4);
        self.h_line(4, 0, 4);
        self.pixel(3, 1);
        self.pixel(2, 2);
        self.pixel(1, 3);
    }

    fn one(&mut self) {
        self.pixel(1, 0);
        self.v_line(2, 0, 3);
        self.h_line(4, 1, 3);
        self.shrink(1);
    }

    fn two(&mut self) {
        self.h_line(0, 0, 2);
        self.pixel(3, 1);
        self.h_line(2, 1, 2);
        self.pixel(0, 3);
        self.h_line(4, 0, 3);
        self.shrink(1);
    }

    fn three(&mut self) {
        self.h_line(0, 0, 2);
        self.h_line(2, 1, 2);
        self.h_line(4, 0, 2);
        self.pixel(3, 1);
        self.pixel(3, 3);
        self.shrink(1);
    }

    fn four(&mut self) {
        self.v_line(2, 0, 4);
        self.h_line(3, 0, 3);
        self.pixel(0, 2);
        self.pixel(1, 1);
        self.shrink(1);
    }

    fn five(&mut self) {
        self.h_line(0, 0, 3);
        self.pixel(0, 1);
        self.h_line(2, 0, 2);
        self.pixel(3, 3);
        self.h_line(4, 0, 2);
        self.shrink(1);
    }

    fn six(&mut self) {
        self.h_line(0, 1, 2);
        self.h_line(2, 1, 2);
        self.h_line(4, 1, 2);
        self.v_line(0, 1, 3);
        self.pixel(3, 3);
        self.shrink(1);
    }

    fn seven(&mut self) {
        self.h_line(0, 0, 3);
        self.pixel(3, 1);
        self.pixel(2, 2);
        self.pixel(1, 3);
        self.pixel(0, 4);
        self.shrink(1);
    }

    fn eight(&mut self) {
        self.h_line(0, 1, 2);
        self.h_line(2, 1, 2);
        self.h_line(4, 1, 2);
        self.pixel(0, 1);
        self.pixel(3, 1);
        self.pixel(0, 3);
        self.pixel(3, 3);
        self.shrink(1);
    }

    fn nine(&mut self) {
        self.pixel(0, 1);
        self.v_line(3, 1, 3);
        self.h_line(0, 1, 2);
        self.h_line(2, 1, 2);
        self.h_line(4, 1, 2);
        self.shrink(1);
    }

    fn zero(&mut self) {
        self.h_line(0, 1, 2);
        self.h_line(4, 1, 2);
        self.v_line(0, 1, 3);
        self.v_line(3, 1, 3);
        self.shrink(1);
    }

    fn hyphen(&mut self) {
        self.h_line(2, 0, 3);
        self.shrink(1);
    }

    fn underscore(&mut self) {
        self.h_line(4, 0, 4);
    }

    fn equals(&mut self) {
        self.h_line(1, 0, 4);
        self.h_line(3, 0, 4);
    }

    fn plus(&mut self) {
        self.v_line(2, 0, 4);
        self.h_line(2, 0, 4);
    }

    fn l_sqr_bracket(&mut self) {
        self.v_line(0, 0, 4);
        self.h_line(0, 1, 2);
        self.h_line(4, 1, 2);
        self.shrink(1);
    }

    fn l_crl_bracket(&mut self) {
        self.pixel(2, 0);
        self.pixel(2, 4);
        self.v_line(1, 1, 3);
        self.pixel(0, 2);
        self.shrink(1);
    }

    fn r_sqr_bracket(&mut self) {
        self.v_line(2, 0, 4);
        self.h_line(0, 0, 1);
        self.h_line(4, 0, 1);
        self.shrink(1);
    }

    fn r_crl_bracket(&mut self) {
        self.pixel(0, 0);
        self.pixel(0, 4);
        self.v_line(1, 1, 3);
        self.pixel(2, 2);
        self.shrink(1);
    }

    fn back_slash(&mut self) {
        for i in 0..5 {
            self.pixel(i, i);
        }
    }

    fn vertical_line(&mut self) {
        self.v_line(0, 0, 4);
        self.shrink(4);
    }

    fn semicolon(&mut self) {
        self.v_line(0, 1, 1);
        self.v_line(0, 3, 4);
        self.shrink(4);
    }

    fn colon(&mut self) {
        self.v_line(0, 1, 1);
        self.v_line(0, 3, 3);
        self.shrink(4);
    }

    fn apostrophe(&mut self) {
        self.v_line(0, 0, 1);
        self.shrink(4);
    }

    fn quotation(&mut self) {
        self.v_line(0, 0, 1);
        self.v_line(2, 0, 1);
        self.shrink(2);
    }

    fn comma(&mut self) {
        self.v_line(0, 3, 4);
        self.shrink(4);
    }

    fn less_than(&mut self) {
        self.pixel(0, 2);
        self.pixel(1, 1);
        self.pixel(2, 0);
        self.pixel(1, 3);
        self.pixel(2, 4);
        self.shrink(2);
    }

    fn period(&mut self) {
        self.pixel(0, 4);
        self.shrink(4);
    }

    fn greater_than(&mut self) {
        self.pixel(0, 0);
        self.pixel(1, 1);
        self.pixel(2, 2);
        self.pixel(1, 3);
        self.pixel(0, 4);
        self.shrink(2);
    }

    fn fwd_slash(&mut self) {
        self.pixel(0, 4);
        self.pixel(1, 3);
        self.pixel(2, 2);
        self.pixel(3, 1);
        self.pixel(4, 0);
    }

    fn question(&mut self) {
        self.pixel(0, 1);
        self.h_line(0, 1, 3);
        self.pixel(4, 1);
        self.h_line(2, 2, 3);
        self.pixel(2, 4);
    }

    fn accent(&mut self) {
        self.pixel(0, 0);
        self.pixel(1, 1);
        self.shrink(3);
    }

    fn tilde(&mut self) {
        self.pixel(0, 1);
        self.pixel(2, 1);
        self.pixel(1, 0);
        self.pixel(3, 0);
        self.shrink(1);
    }

    fn exclamation(&mut self) {
        self.v_line(0, 0, 2);
        self.pixel(0, 4);
        self.shrink(4);
    }

    fn at(&mut self) {
        self.h_line(0, 1, 3);
        self.h_line(4, 1, 3);
        self.v_line(0, 1, 3);
        self.v_line(4, 1, 2);
        self.v_line(2, 1, 2);
        self.pixel(3, 2);
    }

    fn octothorpe(&mut self) {
        self.v_line(1, 0, 4);
        self.v_line(3, 0, 4);
        self.h_line(1, 0, 4);
        self.h_line(3, 0, 4);
    }

    fn dollar(&mut self) {
        self.s();
        self.v_line(2, -1, 5);
    }

    fn percent(&mut self) {
        self.fwd_slash();
        self.pixel(0, 0);
        self.pixel(4, 4);
    }

    fn caret(&mut self) {
        self.pixel(0, 2);
        self.pixel(1, 1);
        self.pixel(2, 0);
        self.pixel(3, 1);
        self.pixel(4, 2);
    }

    fn ampersand(&mut self) {
        self.h_line(0, 1, 3);
        self.h_line(2, 1, 4);
        self.h_line(4, 1, 3);
        self.pixel(0, 1);
        self.pixel(0, 3);
        self.pixel(3, 3);
    }

    fn star(&mut self) {
        self.v_line(2, 0, 4);
        self.h_line(2, 1, 3);
        self.pixel(0, 1);
        self.pixel(4, 1);
        self.pixel(0, 3);
        self.pixel(4, 3);
    }

    fn l_round_bracket(&mut self) {
        self.v_line(0, 1, 3);
        self.pixel(1, 0);
        self.pixel(1, 4);
        self.shrink(3);
    }

    fn r_round_bracket(&mut self) {
        self.v_line(1, 1, 3);
        self.pixel(0, 0);
        self.pixel(0, 4);
        self.shrink(3);
    }

    fn invalid(&mut self) {
        self.h_line(0, 0, 3);
        self.h_line(4, 0, 3);
        self.v_line(0, 1, 3);
        self.v_line(3, 1, 3);
        self.shrink(1);
    }

    fn shrink(&mut self, by: u8) {
        self.point.0 -= f32::from(by);
    }

    fn pixel(&mut self, col: i8, row: i8) {
        let rect = Rect {
            x: self.point.0 + f32::from(col) + Self::MARGIN,
            y: self.point.1 + f32::from(row) + Self::MARGIN,
            w: Self::WIDTH,
            h: Self::WIDTH,
        };
        self.draw_rect(rect);
    }

    fn v_line(&mut self, col: i8, from: i8, to: i8) {
        let rect = Rect {
            x: self.point.0 + f32::from(col) + Self::MARGIN,
            y: self.point.1 + f32::from(from) + Self::MARGIN,
            w: Self::WIDTH,
            h: f32::from(to - from) + Self::WIDTH,
        };
        self.draw_rect(rect);
    }

    fn h_line(&mut self, row: i8, from: i8, to: i8) {
        let rect = Rect {
            x: self.point.0 + f32::from(from) + Self::MARGIN,
            y: self.point.1 + f32::from(row) + Self::MARGIN,
            w: f32::from(to - from) + Self::WIDTH,
            h: Self::WIDTH,
        };
        self.draw_rect(rect);
    }

    fn draw_rect(&mut self, rect: Rect) {
        self.mesh_builder.rectangle(Self::FILL, rect, self.color);
    }

    const FILL: DrawMode = DrawMode::Fill(FillOptions::DEFAULT);
    const MARGIN: f32 = 0.0;
    const WIDTH: f32 = 1.0;
}
