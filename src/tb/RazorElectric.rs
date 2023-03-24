#[cfg(feature = "TbRazorElectric")]
use leptos::*;
#[cfg(feature = "TbRazorElectric")]
///This icon requires the feature `TbRazorElectric` to be enabled.
#[component]
pub fn RazorElectric(
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
        "icon icon-tabler icon-tabler-razor-electric" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 3v2" />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 3v2" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M16 3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 12v6a3 3 0 0 0 6 0v-6h-6z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 5h8l-1 4h-6z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 17v1" /> < title > { title } < / title > <
        / svg >
    }
}
