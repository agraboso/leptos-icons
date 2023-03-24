#[cfg(feature = "TbChartInfographic")]
use leptos::*;
#[cfg(feature = "TbChartInfographic")]
///This icon requires the feature `TbChartInfographic` to be enabled.
#[component]
pub fn ChartInfographic(
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
        "icon icon-tabler icon-tabler-chart-infographic" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 7m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 3v4h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 17l0 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 14l0 7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 13l0 8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 12l0 9" /> < title > { title } < / title >
        < / svg >
    }
}
