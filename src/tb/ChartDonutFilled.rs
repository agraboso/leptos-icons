#[cfg(feature = "TbChartDonutFilled")]
use leptos::*;
#[cfg(feature = "TbChartDonutFilled")]
///This icon requires the feature `TbChartDonutFilled` to be enabled.
#[component]
pub fn ChartDonutFilled(
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
        "icon icon-tabler icon-tabler-chart-donut-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.883 2.207a1.9 1.9 0 0 1 2.087 1.522l.025 .167l.005 .104v4a1 1 0 0 1 -.641 .933l-.107 .035a3.1 3.1 0 1 0 3.73 3.953l.05 -.173a1 1 0 0 1 .855 -.742l.113 -.006h3.8a2 2 0 0 1 2 2a1 1 0 0 1 -.026 .226a10 10 0 1 1 -12.27 -11.933l.27 -.067l.11 -.02z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.775 2.526a.996 .996 0 0 1 .22 -.026l.122 .007l.112 .02l.103 .03a10 10 0 0 1 6.003 5.817l.108 .294a1 1 0 0 1 -.824 1.325l-.119 .007h-4.5a1 1 0 0 1 -.76 -.35a8 8 0 0 0 -.89 -.89a1 1 0 0 1 -.342 -.636l-.008 -.124v-4.495l.006 -.118c.005 -.042 .012 -.08 .02 -.116l.03 -.103a.998 .998 0 0 1 .168 -.299l.071 -.08c.03 -.028 .058 -.052 .087 -.075l.09 -.063l.088 -.05l.103 -.043l.112 -.032z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
