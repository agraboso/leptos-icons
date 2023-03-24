#[cfg(feature = "SiOpenbugbounty")]
use leptos::*;
#[cfg(feature = "SiOpenbugbounty")]
///This icon requires the feature `SiOpenbugbounty` to be enabled.
#[component]
pub fn Openbugbounty(
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
        "M8.092 2.443a1.388 1.388 0 0 0-1.428 1.611c.42 2.567 2.11 4.115 3.58 4.998a14.12 14.12 0 0 0 .4 2.926H6.52a1.388 1.388 0 0 0 0 2.777h5.155c.39.767.85 1.475 1.37 2.108-1.816 1.36-3.516 3.734-4.34 4.983a1.388 1.388 0 1 0 2.316 1.531c1.376-2.08 3.15-4.046 4.09-4.604a8.208 8.208 0 0 0 3.757 1.416V6.492h-7.484c-.867-.588-1.753-1.506-1.979-2.886a1.388 1.388 0 0 0-1.313-1.163zM18.859 0c-2.971 0-5.501 1.967-6.577 4.765h6.577Z"
        /> < title > { title } < / title > < / svg >
    }
}
