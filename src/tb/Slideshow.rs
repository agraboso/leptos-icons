#[cfg(feature = "TbSlideshow")]
use leptos::*;
#[cfg(feature = "TbSlideshow")]
///This icon requires the feature `TbSlideshow` to be enabled.
#[component]
pub fn Slideshow(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-slideshow"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 6l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 3m0 3a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v8a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 13l4 -4a3 5 0 0 1 3 0l4 4"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M13 12l2 -2a3 5 0 0 1 3 0l3 3"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 21l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 21l.01 0" /> < title > { title } < / title
        > < / svg >
    }
}
