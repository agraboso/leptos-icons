#[cfg(feature = "ImWondering2")]
use leptos::*;
#[cfg(feature = "ImWondering2")]
///This icon requires the feature `ImWondering2` to be enabled.
#[component]
pub fn Wondering2(
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
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM11 4c0.552 0 1 0.448 1 1s-0.448 1-1 1-1-0.448-1-1 0.448-1 1-1zM4 5c0-0.552 0.448-1 1-1s1 0.448 1 1-0.448 1-1 1-1-0.448-1-1zM5.176 12.6l-0.351-1.2 6.828-2 0.351 1.2-6.828 2z"
        /> < title > { title } < / title > < / svg >
    }
}
