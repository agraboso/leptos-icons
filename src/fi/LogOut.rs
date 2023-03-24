#[cfg(feature = "FiLogOut")]
use leptos::*;
#[cfg(feature = "FiLogOut")]
///This icon requires the feature `FiLogOut` to be enabled.
#[component]
pub fn LogOut(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap =
        "round" stroke - linejoin = "round" width = { size.clone() } height = { size } >
        < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "16 17 21 12 16 7" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "21" y1 = "12" x2 = "9" y2 = "12" /> < title >
        { title } < / title > < / svg >
    }
}
