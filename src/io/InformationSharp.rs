#[cfg(feature = "IoInformationSharp")]
use leptos::*;
#[cfg(feature = "IoInformationSharp")]
///This icon requires the feature `IoInformationSharp` to be enabled.
#[component]
pub fn InformationSharp(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < polyline xmlns =
        "http://www.w3.org/2000/svg" points = "196 220 260 220 260 392" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:40px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "187" y1 = "396" x2 = "325" y2
        = "396" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:40px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,160a32,32,0,1,1,32-32A32,32,0,0,1,256,160Z" /> < title > { title } < /
        title > < / svg >
    }
}
