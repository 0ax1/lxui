#![allow(dead_code)]

mod view;
use view::*;

mod shapes;
use shapes::*;

#[rustfmt::skip]
fn canvas() -> impl view::View {
    VStack::new((
        HStack::new((
            Rectangle::default()
                .size(100, 100)
                .padding_left(10)
                .padding_right(10),

            Circle::default()
                .size(100, 100),

            Rectangle::default()
                .size(100, 100)
                .padding_left(10)
                .padding_right(10),

        ))
        .size(340, 100)
        .padding_bottom(10),

        HStack::new((
            Rectangle::default()
                .size(200, 200),

            Circle::default()
                .size(200, 200)
                .visible(false),

            Circle::default()
                .size(200, 200),

            Rectangle::default()
                .size(200, 200),
        ))
        .spacing(10)
        .size(620, 200),
    ))
    .size(620, 300)
}

fn main() {
    let cx = view::Context {
        origin: view::Origin { x: 0, y: 0 },
        level: 0,
    };
    canvas().draw(cx);
}
