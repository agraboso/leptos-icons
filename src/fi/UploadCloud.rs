#[cfg(feature = "FiUploadCloud")]
use leptos::*;
#[cfg(feature = "FiUploadCloud")]
///This icon requires the feature `FiUploadCloud` to be enabled.
#[component]
pub fn UploadCloud(
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
        < polyline xmlns = "http://www.w3.org/2000/svg" points = "16 16 12 12 8 16" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "12" y1 = "12" x2 = "12" y2 = "21"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3" />< polyline xmlns =
        "http://www.w3.org/2000/svg" points = "16 16 12 12 8 16" /> < title > { title } <
        / title > < / svg >
    }
}
