# yew-plotly
[![GitHub open issues](https://img.shields.io/github/issues/butzist/yew-plotly.svg)](https://github.com/butzist/yew-plotly/issues)

## Example

```rust
use yew::prelude::*;
use yew_plotly::plotly::common::Mode;
use yew_plotly::plotly::{Plot, Scatter};
use yew_plotly::Plotly;

#[function_component(App)]
fn app() -> Html {
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
    yew::start_app::<App>();
}
```

## Installation

```
cargo add yew-plotly
```

## Links

* [Yew](https://yew.rs)
* [Plot.ly](https://plot.ly/javascript/)