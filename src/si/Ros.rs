#[cfg(feature = "SiRos")]
use leptos::*;
#[cfg(feature = "SiRos")]
///This icon requires the feature `SiRos` to be enabled.
#[component]
pub fn Ros(
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
        "M2.807 0C1.353 0 .173 1.22.173 2.722c0 1.504 1.18 2.723 2.634 2.723 1.455 0 2.635-1.22 2.635-2.723S4.262 0 2.807 0zM12 0c-1.455 0-2.634 1.22-2.634 2.722 0 1.504 1.18 2.723 2.634 2.723 1.455 0 2.634-1.22 2.634-2.723S13.454 0 12 0zm9.193 0c-1.455 0-2.635 1.22-2.635 2.722 0 1.504 1.18 2.723 2.635 2.723 1.455 0 2.634-1.22 2.634-2.723S22.647 0 21.193 0zM2.807 9.277C1.353 9.277.173 10.497.173 12s1.18 2.722 2.634 2.722c1.455 0 2.635-1.219 2.635-2.722 0-1.504-1.18-2.723-2.635-2.723zm9.193 0c-1.455 0-2.634 1.22-2.634 2.723s1.18 2.722 2.634 2.722c1.455 0 2.634-1.219 2.634-2.722 0-1.504-1.18-2.723-2.634-2.723zm9.193 0c-1.455 0-2.635 1.22-2.635 2.723s1.18 2.722 2.635 2.722c1.455 0 2.634-1.219 2.634-2.722 0-1.504-1.18-2.723-2.634-2.723zM2.807 18.555c-1.454 0-2.634 1.22-2.634 2.722C.173 22.781 1.353 24 2.807 24c1.455 0 2.635-1.22 2.635-2.723s-1.18-2.722-2.635-2.722zm9.193 0c-1.455 0-2.634 1.22-2.634 2.722C9.366 22.781 10.546 24 12 24c1.455 0 2.634-1.22 2.634-2.723s-1.18-2.722-2.634-2.722zm9.193 0c-1.455 0-2.635 1.22-2.635 2.722 0 1.504 1.18 2.723 2.635 2.723 1.455 0 2.634-1.22 2.634-2.723s-1.18-2.722-2.634-2.722z"
        /> < title > { title } < / title > < / svg >
    }
}
