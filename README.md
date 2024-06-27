Prototype: SwiftUI inspired 2D vector graphics API backed by GPU rendering

```
fn body(state: state::State<ViewTreeState>) -> VStack {
    let ViewTreeState { scale, .. } = state.value();

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
}
```
