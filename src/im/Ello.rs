#[cfg(feature = "ImEllo")]
use leptos::*;
#[cfg(feature = "ImEllo")]
///This icon requires the feature `ImEllo` to be enabled.
#[component]
pub fn Ello(
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
        "M8 0c-4.418 0-8 3.582-8 8s3.582 8 8 8 8-3.582 8-8-3.582-8-8-8zM12.885 9.212c-0.575 2.23-2.584 3.788-4.885 3.788s-4.31-1.558-4.885-3.788c-0.097-0.377 0.131-0.764 0.508-0.861 0.058-0.015 0.118-0.023 0.177-0.023 0.322 0 0.604 0.218 0.684 0.531 0.414 1.605 1.86 2.727 3.516 2.727s3.102-1.121 3.516-2.727c0.081-0.313 0.362-0.531 0.684-0.531 0.060 0 0.12 0.008 0.178 0.023 0.183 0.047 0.336 0.163 0.432 0.326s0.123 0.353 0.075 0.536z"
        /> < title > { title } < / title > < / svg >
    }
}
