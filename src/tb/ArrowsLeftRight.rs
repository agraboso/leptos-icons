#[cfg(feature = "TbArrowsLeftRight")]
use leptos::*;
#[cfg(feature = "TbArrowsLeftRight")]
///This icon requires the feature `TbArrowsLeftRight` to be enabled.
#[component]
pub fn ArrowsLeftRight(
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
        "icon icon-tabler icon-tabler-arrows-left-right" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 17l-18 0" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 10l-3 -3l3 -3" />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 7l18 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M18 20l3 -3l-3 -3" /> < title
        > { title } < / title > < / svg >
    }
}
