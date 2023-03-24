#[cfg(feature = "FiFilm")]
use leptos::*;
#[cfg(feature = "FiFilm")]
///This icon requires the feature `FiFilm` to be enabled.
#[component]
pub fn Film(
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
        < rect xmlns = "http://www.w3.org/2000/svg" x = "2" y = "2" width = "20" height =
        "20" rx = "2.18" ry = "2.18" />< line xmlns = "http://www.w3.org/2000/svg" x1 =
        "7" y1 = "2" x2 = "7" y2 = "22" />< line xmlns = "http://www.w3.org/2000/svg" x1
        = "17" y1 = "2" x2 = "17" y2 = "22" />< line xmlns = "http://www.w3.org/2000/svg"
        x1 = "2" y1 = "12" x2 = "22" y2 = "12" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "2" y1 = "7" x2 = "7" y2 = "7" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "2" y1 = "17" x2 = "7" y2 = "17" />< line xmlns
        = "http://www.w3.org/2000/svg" x1 = "17" y1 = "17" x2 = "22" y2 = "17" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "17" y1 = "7" x2 = "22" y2 = "7" /> <
        title > { title } < / title > < / svg >
    }
}
