![](assets/frame_brush.png)
# framebrush
`framebrush` is a simple crate that can draw simple shapes on a frame buffer provided by the user.
 

## Why use framebrush?

### Ease of use
Scaling, drawing lines/shapes or even indexing into the frame buffer can be a bit tedious while using a simple frame buffer. (like [softbuffer](https://github.com/rust-windowing/softbuffer), which is the crate used across the [examples](https://github.com/serd223/framebrush/tree/master/examples)!)

`framebrush` can handle scaling, line drawing and rectangle drawing for you. And because `framebrush` doesn't have any platform specific code, (it only writes to the buffer you provided) you can use it in practically any context!

### Simplicity
`framebrush` is a fundamentally simple crate so it has a pretty simple API. All you need to do is create a `Canvas` and use its methods to draw on the buffer.


## Pixel Formats
The `RGBu32::Rgb` uses the same format that [softbuffer](https://github.com/rust-windowing/softbuffer) uses, which is;

```
00000000RRRRRRRRGGGGGGGGBBBBBBBB (u32)
R: Red channel
G: Green channel
B: Blue channel
```

If you want to create your own color abstraction, implement the `Color` trait on your own color type.
