# yew-plotly
[![GitHub open issues](https://img.shields.io/github/issues/butzist/yew-plotly.svg)](https://github.com/butzist/yew-plotly/issues)

## Example

```rust
use yew::prelude::*;
use yew_plotly::plotly::common::Mode;
use yew_plotly::plotly::{Plot, Scatter};
use yew_plotly::Plotly;

#[function_component]
fn App() -> Html {
    let mut plot = Plot::new();
    let x_values = vec![1, 2, 3];
    let y_values = vec![1, 3, 2];

    let trace = Scatter::new(x_values, y_values)
        .mode(Mode::LinesMarkersText)
        .name("Scatter");
        
    plot.add_trace(trace);

    html! { <Plotly plot={plot}/> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## Setup

#### Load plotly.js in `index.html`

```html
<!DOCTYPE html>
<html>

<head>
    <meta charset="utf-8" />
    <title>Yew App</title>
    <script src="https://cdn.plot.ly/plotly-2.16.1.min.js"></script>
</head>

</html>
```

#### Install `yew-plotly`

```
cargo add yew-plotly
```

## Links

* [Yew](https://yew.rs)
* [Plot.ly](https://plot.ly/javascript/)
* [plotly-rs](https://github.com/igiagkiozis/plotly)