#[cfg(feature = "TbTractor")]
use leptos::*;
#[cfg(feature = "TbTractor")]
///This icon requires the feature `TbTractor` to be enabled.
#[component]
pub fn Tractor(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-tractor"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 15m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 15l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 17m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M10.5 17l6.5 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 15.2v-4.2a1 1 0 0 0 -1 -1h-6l-2 -5h-6v6.5"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M18 5h-1a1 1 0 0 0 -1 1v4" />
        < title > { title } < / title > < / svg >
    }
}
