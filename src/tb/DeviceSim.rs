#[cfg(feature = "TbDeviceSim")]
use leptos::*;
#[cfg(feature = "TbDeviceSim")]
///This icon requires the feature `TbDeviceSim` to be enabled.
#[component]
pub fn DeviceSim(
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
        "icon icon-tabler icon-tabler-device-sim" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 3h8.5l4.5 4.5v12.5a1 1 0 0 1 -1 1h-12a1 1 0 0 1 -1 -1v-16a1 1 0 0 1 1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 11h3v6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 17v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 14v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 11v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 14v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 17v.01" /> < title > { title } < / title > <
        / svg >
    }
}
