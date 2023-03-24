#[cfg(feature = "TbListCheck")]
use leptos::*;
#[cfg(feature = "TbListCheck")]
///This icon requires the feature `TbListCheck` to be enabled.
#[component]
pub fn ListCheck(
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
        "icon icon-tabler icon-tabler-list-check" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 5.5l1.5 1.5l2.5 -2.5" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 11.5l1.5 1.5l2.5 -2.5" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 17.5l1.5 1.5l2.5 -2.5" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 6l9 0" />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 12l9 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M11 18l9 0" /> < title > { title }
        < / title > < / svg >
    }
}
