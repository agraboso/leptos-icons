#[cfg(feature = "TbBuildingLighthouse")]
use leptos::*;
#[cfg(feature = "TbBuildingLighthouse")]
///This icon requires the feature `TbBuildingLighthouse` to be enabled.
#[component]
pub fn BuildingLighthouse(
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
        "icon icon-tabler icon-tabler-building-lighthouse" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 3l2 3l2 15h-8l2 -15z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 9l8 0" />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 11l2 -2l-2 -2"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M21 11l-2 -2l2 -2" /> < title
        > { title } < / title > < / svg >
    }
}
