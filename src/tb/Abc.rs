#[cfg(feature = "TbAbc")]
use leptos::*;
#[cfg(feature = "TbAbc")]
///This icon requires the feature `TbAbc` to be enabled.
#[component]
pub fn Abc(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-abc" width
        = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 16v-6a2 2 0 1 1 4 0v6" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 13h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 8v6a2 2 0 1 0 4 0v-1a2 2 0 1 0 -4 0v1" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.732 12a2 2 0 0 0 -3.732 1v1a2 2 0 0 0 3.726 1.01" /> < title > { title } < /
        title > < / svg >
    }
}
