#[cfg(feature = "BiSolidBolt")]
use leptos::*;
#[cfg(feature = "BiSolidBolt")]
///This icon requires the feature `BiSolidBolt` to be enabled.
#[component]
pub fn Bolt(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.168 8H13l.806-4.835A1 1 0 0 0 12.819 2H7.667a1 1 0 0 0-.986.835l-1.667 10A1 1 0 0 0 6 14h4v8l8.01-12.459A1 1 0 0 0 17.168 8z"
        /> < title > { title } < / title > < / svg >
    }
}
