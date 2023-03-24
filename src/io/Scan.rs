#[cfg(feature = "IoScan")]
use leptos::*;
#[cfg(feature = "IoScan")]
///This icon requires the feature `IoScan` to be enabled.
#[component]
pub fn Scan(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M342,444h46a56,56,0,0,0,56-56V342" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M444,170V124a56,56,0,0,0-56-56H342" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M170,444H124a56,56,0,0,1-56-56V342" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M68,170V124a56,56,0,0,1,56-56h46" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:44px"
        /> < title > { title } < / title > < / svg >
    }
}
