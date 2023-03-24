#[cfg(feature = "TbHomeShare")]
use leptos::*;
#[cfg(feature = "TbHomeShare")]
///This icon requires the feature `TbHomeShare` to be enabled.
#[component]
pub fn HomeShare(
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
        "icon icon-tabler icon-tabler-home-share" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 21v-6a2 2 0 0 1 2 -2h2c.247 0 .484 .045 .702 .127" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 12h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M16 22l5 -5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 21.5v-4.5h-4.5" /> < title > { title } < /
        title > < / svg >
    }
}
