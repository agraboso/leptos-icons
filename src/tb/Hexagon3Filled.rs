#[cfg(feature = "TbHexagon3Filled")]
use leptos::*;
#[cfg(feature = "TbHexagon3Filled")]
///This icon requires the feature `TbHexagon3Filled` to be enabled.
#[component]
pub fn Hexagon3Filled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-hexagon-3-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.543 2.426c.862 -.48 1.9 -.512 2.785 -.096l.187 .096l6.026 3.588l.095 .063l.084 .07l.113 .083a3 3 0 0 1 1.143 1.992l.019 .198l.005 .2v6.536c0 1.022 -.52 1.968 -1.326 2.492l-.165 .099l-6.053 3.864c-.845 .47 -1.86 .501 -2.772 .069l-.193 -.1l-5.947 -3.802a3 3 0 0 1 -1.537 -2.418l-.007 -.203v-6.537c0 -1.022 .52 -1.968 1.348 -2.505l6.195 -3.689zm2.457 4.574h-2l-.15 .005a2 2 0 0 0 -1.85 1.995a1 1 0 0 0 1.974 .23l.02 -.113l.006 -.117h2v2h-2l-.133 .007c-1.111 .12 -1.154 1.73 -.128 1.965l.128 .021l.133 .007h2v2h-2l-.007 -.117a1 1 0 0 0 -1.993 .117a2 2 0 0 0 1.85 1.995l.15 .005h2l.15 -.005a2 2 0 0 0 1.844 -1.838l.006 -.157v-2l-.005 -.15a1.988 1.988 0 0 0 -.17 -.667l-.075 -.152l-.019 -.032l.02 -.03a2.01 2.01 0 0 0 .242 -.795l.007 -.174v-2l-.005 -.15a2 2 0 0 0 -1.838 -1.844l-.157 -.006z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
