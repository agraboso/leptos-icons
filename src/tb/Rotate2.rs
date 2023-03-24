#[cfg(feature = "TbRotate2")]
use leptos::*;
#[cfg(feature = "TbRotate2")]
///This icon requires the feature `TbRotate2` to be enabled.
#[component]
pub fn Rotate2(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-rotate-2"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 4.55a8 8 0 0 0 -6 14.9m0 -4.45v5h-5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.37 7.16l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 19.94l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.84 18.37l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.37 15.1l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.94 11l0 .01" /> < title > { title } < /
        title > < / svg >
    }
}
