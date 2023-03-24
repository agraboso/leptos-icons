#[cfg(feature = "IoLogoAppleAr")]
use leptos::*;
#[cfg(feature = "IoLogoAppleAr")]
///This icon requires the feature `IoLogoAppleAr` to be enabled.
#[component]
pub fn LogoAppleAr(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width = {
        size.clone() } height = { size } > < polyline xmlns =
        "http://www.w3.org/2000/svg" points = "201.14 64 256 32 310.86 64" fill = "none"
        stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round" stroke -
        width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "32"
        x2 = "256" y2 = "112" fill = "none" stroke = "#000" stroke - linecap = "round"
        stroke - miterlimit = "10" stroke - width = "32" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "310.86 448 256 480 201.14 448" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1
        = "480" x2 = "256" y2 = "400" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - miterlimit = "10" stroke - width = "32" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "64 207.51 64 144 117.15 112.49" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "64" y1
        = "144" x2 = "131.29" y2 = "184" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - miterlimit = "10" stroke - width = "32" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "448 304.49 448 368 394.85 399.51" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "448" y1
        = "368" x2 = "380.71" y2 = "328" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - miterlimit = "10" stroke - width = "32" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "117.15 400 64 368 64 304.49" fill = "none"
        stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round" stroke -
        width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "64" y1 = "368"
        x2 = "130.64" y2 = "328" fill = "none" stroke = "#000" stroke - linecap = "round"
        stroke - miterlimit = "10" stroke - width = "32" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "394.85 112.49 448 144 448 207.51" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "448" y1
        = "144" x2 = "380.71" y2 = "184" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - miterlimit = "10" stroke - width = "32" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "256 320 256 256 310.86 224" fill = "none"
        stroke = "#000" stroke - linecap = "round" stroke - miterlimit = "10" stroke -
        width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "256" y1 = "256"
        x2 = "201.14" y2 = "224" fill = "none" stroke = "#000" stroke - linecap = "round"
        stroke - miterlimit = "10" stroke - width = "32" /> < title > { title } < / title
        > < / svg >
    }
}
