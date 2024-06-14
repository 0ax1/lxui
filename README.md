SwiftUI inspired 2D vector graphics API backed by GPU rendering

```
fn body(state: state::State<ViewTreeState>) -> VStack {
    let ViewTreeState { scale, .. } = state.value();

    VStack::new((
        HStack::new((
            Rectangle::default()
                .size(100.0, 100.0)
                .stroke(Color::rgb8(122, 122, 122), 2.0 * scale)
                .on_click(state::callback(&state, {
                    |state| {
                        state.scale += 1.0;
                        println!("clicked {}", state.scale);
                    }
                })),

            Circle::default()
                .stroke(Color::rgb8(255, 255, 255), 4.0)
                .diameter(100.0)
                .on_click(state::callback(&state, {
                    |state| {
                        state.scale += 1.0;
                        println!("clicked {}", state.scale);
                    }
                })),
        ))
        .spacing(40.0),

        HStack::new((
            Loop::new(0..18, |idx| {
                VStack::new((
                    Loop::new(0..10, |idx2| {
                        Circle::default()
                            .stroke(Color::rgba8(122, 122, 255, 50), 2.0)
                            .fill(Color::rgb8(
                                25 * idx2 as u8,
                                25 * idx2 as u8,
                                25 * idx2 as u8,
                            ))
                            .diameter(5.0 * (idx + 1) as f64 / 2.0)
                    })),
                )
                .visible(idx % 2 == 0)
                .spacing(20.0)
            })),
        )
        .spacing(20.0),
    ))
    .spacing(100.0)
    .padding_top(40.0)
    .padding_left(40.0)
}
```
