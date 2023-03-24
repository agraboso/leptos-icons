#[cfg(feature = "CgBorderBottom")]
use leptos::*;
#[cfg(feature = "CgBorderBottom")]
///This icon requires the feature `CgBorderBottom` to be enabled.
#[component]
pub fn BorderBottom(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 8H16V15H19V5H5V15H8V8Z" fill =
        "currentColor" fill - opacity = "0.3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 17H19V20H5V17Z" fill = "currentColor" /> <
        title > { title } < / title > < / svg >
    }
}
