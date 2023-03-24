#[cfg(feature = "SiVala")]
use leptos::*;
#[cfg(feature = "SiVala")]
///This icon requires the feature `SiVala` to be enabled.
#[component]
pub fn Vala(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "m9.3836 23.9993-.5152-21.859q-2.2504.8435-3.5153 2.64-1.2509 1.7971-1.2509 4.4689 0 .6092.0629 1.0002.0776.3748.156.6092.0783.2188.1411.3438.0777.1249.0777.2188-.828 0-1.4682-.156-.641-.1723-1.0786-.5316-.4222-.3585-.6565-.9529-.2188-.593-.2188-1.4682 0-1.0624.4524-2.0937.4687-1.0306 1.2657-1.9532.8125-.9211 1.891-1.7025 1.0942-.7815 2.328-1.344Q8.3044.6409 9.6484.3289 11.0079 0 12.3519 0q.3593 0 .6565.0155.312.0156.624.0466l.2816 19.687L20.6481.1554h2.2341L13.9924 24H9.3829Z"
        /> < title > { title } < / title > < / svg >
    }
}
