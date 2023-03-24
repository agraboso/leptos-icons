#[cfg(feature = "SiClaris")]
use leptos::*;
#[cfg(feature = "SiClaris")]
///This icon requires the feature `SiClaris` to be enabled.
#[component]
pub fn Claris(
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
        "M11.56 0a3.34 3.34 0 00-.57.043L22.947 12 10.99 23.957c.132.022.307.043.57.043 6.626 0 12-5.375 12-12s-5.374-12-12-12zm-1.535 2.414C4.738 2.414.44 6.713.44 12s4.3 9.588 9.586 9.588c.264 0 .44-.023.57-.045L1.054 12l9.543-9.543a3.337 3.337 0 00-.57-.043zm.746 2.457c-.263 0-.438.021-.57.043L17.287 12l-7.086 7.086c.132.022.307.045.57.045 3.927 0 7.13-3.204 7.13-7.131s-3.203-7.129-7.13-7.129zm-.416 2.434A4.701 4.701 0 005.66 12a4.701 4.701 0 004.695 4.695c.264 0 .44-.023.57-.045L6.274 12l4.653-4.65a3.296 3.296 0 00-.57-.045Z"
        /> < title > { title } < / title > < / svg >
    }
}
