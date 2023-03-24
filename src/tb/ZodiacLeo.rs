#[cfg(feature = "TbZodiacLeo")]
use leptos::*;
#[cfg(feature = "TbZodiacLeo")]
///This icon requires the feature `TbZodiacLeo` to be enabled.
#[component]
pub fn ZodiacLeo(
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
        "icon icon-tabler icon-tabler-zodiac-leo" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 17a4 4 0 1 0 8 0" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 16m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 7m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M7 7c0 3 2 5 2 9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 7c0 4 -2 6 -2 10" /> < title > { title } <
        / title > < / svg >
    }
}
