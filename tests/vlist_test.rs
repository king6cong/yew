#[cfg(feature = "wasm_test")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::virtual_dom::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

#[cfg(feature = "wasm_test")]
wasm_bindgen_test_configure!(run_in_browser);

struct Comp;

impl Component for Comp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Comp
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }
}

impl Renderable<Comp> for Comp {
    fn view(&self) -> Html<Self> {
        unimplemented!();
    }
}

#[test]
fn check_fragments() {
    let fragment: VNode<Comp> = html! {
        <>
        </>
    };
    let _: VNode<Comp> = html! {
        <div>
            { fragment }
        </div>
    };
}
