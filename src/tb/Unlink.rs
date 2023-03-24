#[cfg(feature = "TbUnlink")]
use leptos::*;
#[cfg(feature = "TbUnlink")]
///This icon requires the feature `TbUnlink` to be enabled.
#[component]
pub fn Unlink(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-unlink"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 14a3.5 3.5 0 0 0 5 0l4 -4a3.5 3.5 0 0 0 -5 -5l-.5 .5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 10a3.5 3.5 0 0 0 -5 0l-4 4a3.5 3.5 0 0 0 5 5l.5 -.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 21l0 -2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 16l2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 8l2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 3l0 2" /> < title > { title } < / title > <
        / svg >
    }
}
