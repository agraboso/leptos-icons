#[cfg(feature = "TbWashMachine")]
use leptos::*;
#[cfg(feature = "TbWashMachine")]
///This icon requires the feature `TbWashMachine` to be enabled.
#[component]
pub fn WashMachine(
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
        "icon icon-tabler icon-tabler-wash-machine" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 3m0 2a2 2 0 0 1 2 -2h10a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 14m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 6h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 6h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 6h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 14c1.333 -.667 2.667 -.667 4 0c1.333 .667 2.667 .667 4 0" /> < title > {
        title } < / title > < / svg >
    }
}
