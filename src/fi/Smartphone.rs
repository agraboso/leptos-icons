#[cfg(feature = "FiSmartphone")]
use leptos::*;
#[cfg(feature = "FiSmartphone")]
///This icon requires the feature `FiSmartphone` to be enabled.
#[component]
pub fn Smartphone(
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
        < rect xmlns = "http://www.w3.org/2000/svg" x = "5" y = "2" width = "14" height =
        "20" rx = "2" ry = "2" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1
        = "18" x2 = "12.01" y2 = "18" /> < title > { title } < / title > < / svg >
    }
}
