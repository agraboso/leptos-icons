#[cfg(feature = "TbMultiplier15x")]
use leptos::*;
#[cfg(feature = "TbMultiplier15x")]
///This icon requires the feature `TbMultiplier15x` to be enabled.
#[component]
pub fn Multiplier15x(
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
        "icon icon-tabler icon-tabler-multiplier-1-5x" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 16v-8l-2 2" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 16h2a2 2 0 1 0 0 -4h-2v-4h4" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M7 16v.01" />< path xmlns = "http://www.w3.org/2000/svg" d = "M17 16l4 -4" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M21 16l-4 -4" /> < title > { title
        } < / title > < / svg >
    }
}
