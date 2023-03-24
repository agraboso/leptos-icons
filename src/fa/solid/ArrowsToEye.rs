#[cfg(feature = "FaSolidArrowsToEye")]
use leptos::*;
#[cfg(feature = "FaSolidArrowsToEye")]
///This icon requires the feature `FaSolidArrowsToEye` to be enabled.
#[component]
pub fn ArrowsToEye(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 15C16.4 5.7 31.6 5.7 41 15l63 63V40c0-13.3 10.7-24 24-24s24 10.7 24 24v96c0 13.3-10.7 24-24 24H32c-13.3 0-24-10.7-24-24s10.7-24 24-24H70.1L7 49C-2.3 39.6-2.3 24.4 7 15zM125.5 243.9C150.6 193.6 214.7 112 312 112s161.4 81.6 186.5 131.9c3.8 7.6 3.8 16.5 0 24.2C473.4 318.4 409.3 400 312 400s-161.4-81.6-186.5-131.9c-3.8-7.6-3.8-16.5 0-24.2zM312 320a64 64 0 1 0 0-128 64 64 0 1 0 0 128zM583 15c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-63 63H592c13.3 0 24 10.7 24 24s-10.7 24-24 24H496c-13.3 0-24-10.7-24-24V40c0-13.3 10.7-24 24-24s24 10.7 24 24V78.1l63-63zM7 497c-9.4-9.4-9.4-24.6 0-33.9l63-63H32c-13.3 0-24-10.7-24-24s10.7-24 24-24h96c13.3 0 24 10.7 24 24v96c0 13.3-10.7 24-24 24s-24-10.7-24-24V433.9L41 497c-9.4 9.4-24.6 9.4-33.9 0zm576 0l-63-63V472c0 13.3-10.7 24-24 24s-24-10.7-24-24V376c0-13.3 10.7-24 24-24h96c13.3 0 24 10.7 24 24s-10.7 24-24 24H553.9l63 63c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0z"
        /> < title > { title } < / title > < / svg >
    }
}
