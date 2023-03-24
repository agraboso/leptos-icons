#[cfg(feature = "TbLadderOff")]
use leptos::*;
#[cfg(feature = "TbLadderOff")]
///This icon requires the feature `TbLadderOff` to be enabled.
#[component]
pub fn LadderOff(
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
        "icon icon-tabler icon-tabler-ladder-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 3v1m0 4v13" />< path xmlns = "http://www.w3.org/2000/svg" d = "M16 3v9m0 4v5"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 14h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 10h2m4 0h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 6h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 18h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
