#[cfg(feature = "FiSkipBack")]
use leptos::*;
#[cfg(feature = "FiSkipBack")]
///This icon requires the feature `FiSkipBack` to be enabled.
#[component]
pub fn SkipBack(
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
        < polygon xmlns = "http://www.w3.org/2000/svg" points = "19 20 9 12 19 4 19 20"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "5" y1 = "19" x2 = "5" y2 =
        "5" /> < title > { title } < / title > < / svg >
    }
}
