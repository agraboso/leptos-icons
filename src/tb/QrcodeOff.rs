#[cfg(feature = "TbQrcodeOff")]
use leptos::*;
#[cfg(feature = "TbQrcodeOff")]
///This icon requires the feature `TbQrcodeOff` to be enabled.
#[component]
pub fn QrcodeOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-qrcode-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4h1a1 1 0 0 1 1 1v1m-.297 3.711a1 1 0 0 1 -.703 .289h-4a1 1 0 0 1 -1 -1v-4c0 -.275 .11 -.524 .29 -.705"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 17v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 4m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 7v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 14m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M17 7v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 14v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 14v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 20h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
