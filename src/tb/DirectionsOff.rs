#[cfg(feature = "TbDirectionsOff")]
use leptos::*;
#[cfg(feature = "TbDirectionsOff")]
///This icon requires the feature `TbDirectionsOff` to be enabled.
#[component]
pub fn DirectionsOff(
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
        "icon icon-tabler icon-tabler-directions-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 21v-4" />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 13v-1" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 5v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 21h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 8v1h1m4 0h6l2 -2l-2 -2h-10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 14v3h-8l-2 -2l2 -2h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
