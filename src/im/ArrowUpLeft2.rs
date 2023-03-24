#[cfg(feature = "ImArrowUpLeft2")]
use leptos::*;
#[cfg(feature = "ImArrowUpLeft2")]
///This icon requires the feature `ImArrowUpLeft2` to be enabled.
#[component]
pub fn ArrowUpLeft2(
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
        "M13.707 12.293l-8.293-8.293h3.586c0.552 0 1-0.448 1-1s-0.448-1-1-1h-6c-0.404 0-0.769 0.244-0.924 0.617-0.051 0.124-0.076 0.254-0.076 0.383h-0.001v6c0 0.552 0.448 1 1 1s1-0.448 1-1v-3.586l8.293 8.293c0.195 0.195 0.451 0.293 0.707 0.293s0.512-0.098 0.707-0.293c0.391-0.39 0.391-1.024 0-1.414z"
        /> < title > { title } < / title > < / svg >
    }
}
