#[cfg(feature = "FaBrandsTencentWeibo")]
use leptos::*;
#[cfg(feature = "FaBrandsTencentWeibo")]
///This icon requires the feature `FaBrandsTencentWeibo` to be enabled.
#[component]
pub fn TencentWeibo(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M72.3 495.8c1.4 19.9-27.6 22.2-29.7 2.9C31 368.8 73.7 259.2 144 185.5c-15.6-34 9.2-77.1 50.6-77.1 30.3 0 55.1 24.6 55.1 55.1 0 44-49.5 70.8-86.9 45.1-65.7 71.3-101.4 169.8-90.5 287.2zM192 .1C66.1.1-12.3 134.3 43.7 242.4 52.4 259.8 79 246.9 70 229 23.7 136.4 91 29.8 192 29.8c75.4 0 136.9 61.4 136.9 136.9 0 90.8-86.9 153.9-167.7 133.1-19.1-4.1-25.6 24.4-6.6 29.1 110.7 23.2 204-60 204-162.3C358.6 74.7 284 .1 192 .1z"
        /> < title > { title } < / title > < / svg >
    }
}
