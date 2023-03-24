#[cfg(feature = "IoTimeOutline")]
use leptos::*;
#[cfg(feature = "IoTimeOutline")]
///This icon requires the feature `IoTimeOutline` to be enabled.
#[component]
pub fn TimeOutline(
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
        "http://www.w3.org/2000/svg" d =
        "M256,64C150,64,64,150,64,256s86,192,192,192,192-86,192-192S362,64,256,64Z" style
        = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< polyline
        xmlns = "http://www.w3.org/2000/svg" points = "256 128 256 272 352 272" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
