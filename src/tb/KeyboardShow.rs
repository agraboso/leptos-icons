#[cfg(feature = "TbKeyboardShow")]
use leptos::*;
#[cfg(feature = "TbKeyboardShow")]
///This icon requires the feature `TbKeyboardShow` to be enabled.
#[component]
pub fn KeyboardShow(
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
        "icon icon-tabler icon-tabler-keyboard-show" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 3m0 2a2 2 0 0 1 2 -2h16a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-16a2 2 0 0 1 -2 -2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 7l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 7l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 7l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 7l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 11l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 11l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 11l4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 19l2 2l2 -2" /> < title > { title } < /
        title > < / svg >
    }
}
