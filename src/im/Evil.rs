#[cfg(feature = "ImEvil")]
use leptos::*;
#[cfg(feature = "ImEvil")]
///This icon requires the feature `ImEvil` to be enabled.
#[component]
pub fn Evil(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M10 7c-0.552 0-1-0.448-1-1 0-0.018 0.001-0.036 0.002-0.054 0.032-0.741 0.706-1.234 1.275-1.518 0.543-0.271 1.080-0.407 1.102-0.413 0.268-0.067 0.539 0.096 0.606 0.364s-0.096 0.539-0.364 0.606c-0.275 0.070-0.602 0.189-0.89 0.334 0.166 0.179 0.268 0.418 0.268 0.681 0 0.552-0.448 1-1 1zM4.379 4.985c-0.268-0.067-0.431-0.338-0.364-0.606s0.338-0.431 0.606-0.364c0.023 0.006 0.559 0.141 1.102 0.413 0.568 0.284 1.243 0.776 1.275 1.518 0.001 0.018 0.002 0.036 0.002 0.054 0 0.552-0.448 1-1 1s-1-0.448-1-1c0-0.263 0.102-0.503 0.268-0.681-0.288-0.144-0.614-0.264-0.89-0.334zM8 11.5c1.274 0 2.389-0.681 3.002-1.699l1.286 0.772c-0.874 1.454-2.467 2.427-4.288 2.427s-3.413-0.973-4.288-2.427l1.286-0.772c0.612 1.018 1.727 1.699 3.002 1.699zM16 1c0-0.711-0.149-1.387-0.416-2-0.525 1.201-1.507 2.155-2.726 2.643-1.347-1.031-3.030-1.643-4.857-1.643s-3.51 0.613-4.857 1.643c-1.22-0.488-2.202-1.443-2.726-2.643-0.268 0.613-0.416 1.289-0.416 2 0 1.15 0.388 2.208 1.040 3.053-0.662 1.165-1.040 2.512-1.040 3.947 0 4.418 3.582 8 8 8s8-3.582 8-8c0-1.436-0.378-2.783-1.040-3.947 0.652-0.845 1.040-1.903 1.040-3.053zM8 14.5c-3.59 0-6.5-2.91-6.5-6.5s2.91-6.5 6.5-6.5 6.5 2.91 6.5 6.5-2.91 6.5-6.5 6.5z"
        /> < title > { title } < / title > < / svg >
    }
}
