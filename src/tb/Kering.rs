#[cfg(feature = "TbKering")]
use leptos::*;
#[cfg(feature = "TbKering")]
///This icon requires the feature `TbKering` to be enabled.
#[component]
pub fn Kering(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-kering"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 15v-3.5a2.5 2.5 0 1 1 5 0v3.5m0 -2h-5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 9l3 6l3 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 20l6 -16" /> < title > { title } < / title >
        < / svg >
    }
}
