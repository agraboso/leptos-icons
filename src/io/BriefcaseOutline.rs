#[cfg(feature = "IoBriefcaseOutline")]
use leptos::*;
#[cfg(feature = "IoBriefcaseOutline")]
///This icon requires the feature `IoBriefcaseOutline` to be enabled.
#[component]
pub fn BriefcaseOutline(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "32" y = "128" width = "448" height = "320" rx =
        "48" ry = "48" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M144,128V96a32,32,0,0,1,32-32H336a32,32,0,0,1,32,32v32" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "480" y1 = "240" x2 = "32" y2
        = "240" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,240v24a8,8,0,0,1-8,8H200a8,8,0,0,1-8-8V240" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
