#[cfg(feature = "RiMapFillSuitcase3")]
use leptos::*;
#[cfg(feature = "RiMapFillSuitcase3")]
///This icon requires the feature `RiMapFillSuitcase3` to be enabled.
#[component]
pub fn Suitcase3(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0H24V24H0z" />< path d =
        "M15 1c.552 0 1 .448 1 1v5h1V6h2v1h1c.552 0 1 .448 1 1v12c0 .552-.448 1-1 1h-1v1h-2v-1H7v1H5v-1H4c-.552 0-1-.448-1-1V8c0-.552.448-1 1-1h1V6h2v1h1V2c0-.552.448-1 1-1h6zm-6 9H7v8h2v-8zm4 0h-2v8h2v-8zm4 0h-2v8h2v-8zm-3-7h-4v4h4V3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
