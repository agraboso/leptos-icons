#[cfg(feature = "TbBarbell")]
use leptos::*;
#[cfg(feature = "TbBarbell")]
///This icon requires the feature `TbBarbell` to be enabled.
#[component]
pub fn Barbell(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-barbell"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M2 12h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 8h-2a1 1 0 0 0 -1 1v6a1 1 0 0 0 1 1h2" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 7v10a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-10a1 1 0 0 0 -1 -1h-1a1 1 0 0 0 -1 1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 12h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15 7v10a1 1 0 0 0 1 1h1a1 1 0 0 0 1 -1v-10a1 1 0 0 0 -1 -1h-1a1 1 0 0 0 -1 1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 8h2a1 1 0 0 1 1 1v6a1 1 0 0 1 -1 1h-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M22 12h-1" /> < title > { title } < / title > <
        / svg >
    }
}
