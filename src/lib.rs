//! [Yew](https://yew.rs) wrapper for [plotly.js](https://plot.ly/javascript/)
//!
//! Provides a yew component wrapper around the plot.ly charting library.

use plotly::Plot;
use yew::prelude::*;

/// Re-exported plotly crate
pub use plotly;

/// Properties for the [Plotly] component
#[derive(Properties, Clone, PartialEq)]
pub struct PlotlyProps {
    /// Plot data and layout
    pub plot: Plot,
}

/// Plotly component wrapper
///
/// # Examples
///
/// ```
/// # use yew::prelude::*;
/// # use yew_plotly::plotly::{Plot, Scatter};
/// # use yew_plotly::Plotly;
/// # use yew_plotly::plotly::common::Mode;
/// #
/// #[function_component(MyPlot)]
/// fn my_plot() -> Html {
///     let mut plot = Plot::new();
///     let x_values = vec![1, 2, 3];
///     let y_values = vec![1, 3, 2];
///
///     let trace = Scatter::new(x_values, y_values)
///         .mode(Mode::LinesMarkersText)
///         .name("Scatter");
///
///     plot.add_trace(trace);
///
///     html! { <Plotly plot={plot}/> }
/// }
/// ```
pub struct Plotly {
    /// Reference to the parent DOM node
    node_ref: NodeRef,
}

impl Component for Plotly {
    type Message = ();
    type Properties = PlotlyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Plotly {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={ self.node_ref.clone() } />
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        if let Some(node) = self.node_ref.get() {
            PlotlyJS::purge(node.into());
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if let Some(node) = self.node_ref.get() {
            let props_obj = ctx.props().plot.to_js_object().into();
            if first_render {
                PlotlyJS::newPlot(node.into(), props_obj);
            } else {
                PlotlyJS::react(node.into(), props_obj.into());
            }
        }
    }
}

#[allow(non_snake_case)]
mod PlotlyJS {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(js_namespace = Plotly)]
    extern "C" {
        pub(crate) fn newPlot(node: JsValue, obj: JsValue);
        pub(crate) fn react(node: JsValue, obj: JsValue);
        pub(crate) fn purge(node: JsValue);
    }
}
