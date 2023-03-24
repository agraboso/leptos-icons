#[cfg(feature = "TbJetpack")]
use leptos::*;
#[cfg(feature = "TbJetpack")]
///This icon requires the feature `TbJetpack` to be enabled.
#[component]
pub fn Jetpack(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-jetpack"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 6a3 3 0 1 0 -6 0v7h6v-7z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 13h6v-7a3 3 0 0 0 -6 0v7z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 16c0 2.333 .667 4 2 5c1.333 -1 2 -2.667 2 -5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15 16c0 2.333 .667 4 2 5c1.333 -1 2 -2.667 2 -5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 8h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 11h4" /> < title > { title } < / title > <
        / svg >
    }
}
