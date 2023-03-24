#[cfg(feature = "SiR3")]
use leptos::*;
#[cfg(feature = "SiR3")]
///This icon requires the feature `SiR3` to be enabled.
#[component]
pub fn R3(
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
        "M24 13.457c0 .841-.688 1.53-1.53 1.53-.842 0-1.53-.689-1.53-1.53a1.53 1.53 0 1 1 3.06 0zM6.147 2.853c-1.123 0-2.172.3-3.08.828v-.828H0v12.134h3.067V9a3.077 3.077 0 0 1 3.08-3.08l1.029-.007 2.105-3.06H6.147zm8.746 6.08 2.078-3.02v-3.06h-6.615l-2.104 3.06h4.99l-2.565 3.735 1.53 2.653a3.098 3.098 0 0 1 4.65 2.686c0 1.717-1.39 3.1-3.1 3.1-1.71 0-3.1-1.39-3.1-3.1h-3.06c0 3.4 2.76 6.16 6.154 6.16 3.4 0 6.16-2.76 6.16-6.16a6.162 6.162 0 0 0-5.018-6.054z"
        /> < title > { title } < / title > < / svg >
    }
}
