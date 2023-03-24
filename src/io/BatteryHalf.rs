#[cfg(feature = "IoBatteryHalf")]
use leptos::*;
#[cfg(feature = "IoBatteryHalf")]
///This icon requires the feature `IoBatteryHalf` to be enabled.
#[component]
pub fn BatteryHalf(
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
        "http://www.w3.org/2000/svg" x = "32" y = "144" width = "400" height = "224" rx =
        "45.7" ry = "45.7" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "85.69" y = "198.93" width =
        "154.31" height = "114.13" rx = "4" ry = "4" style =
        "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "480" y1 = "218.67" x2 = "480" y2
        = "293.33" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
