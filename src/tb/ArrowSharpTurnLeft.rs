#[cfg(feature = "TbArrowSharpTurnLeft")]
use leptos::*;
#[cfg(feature = "TbArrowSharpTurnLeft")]
///This icon requires the feature `TbArrowSharpTurnLeft` to be enabled.
#[component]
pub fn ArrowSharpTurnLeft(
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
        "icon icon-tabler icon-tabler-arrow-sharp-turn-left" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 18v-11.31a.7 .7 0 0 0 -1.195 -.495l-9.805 9.805" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 16h-5v-5" /> < title > { title } < / title
        > < / svg >
    }
}
