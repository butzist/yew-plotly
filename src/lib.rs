use plotly::Plot;
use yew::prelude::*;

// re-export plotly
pub use plotly;

#[derive(Properties, Clone, PartialEq)]
pub struct PlotlyProps {
    pub plot: Plot,
}

pub struct Plotly {
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

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Plotly)]
        pub(crate) fn newPlot(node: JsValue, obj: JsValue);

        #[wasm_bindgen(js_namespace = Plotly)]
        pub(crate) fn react(node: JsValue, obj: JsValue);

        #[wasm_bindgen(js_namespace = Plotly)]
        pub(crate) fn purge(node: JsValue);
    }
}
