#[cfg(feature = "TbBrandPhp")]
use leptos::*;
#[cfg(feature = "TbBrandPhp")]
///This icon requires the feature `TbBrandPhp` to be enabled.
#[component]
pub fn BrandPhp(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-brand-php"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-10 0a10 9 0 1 0 20 0a10 9 0 1 0 -20 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.5 15l.395 -1.974l.605 -3.026h1.32a1 1 0 0 1 .986 1.164l-.167 1a1 1 0 0 1 -.986 .836h-1.653"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5 15l.395 -1.974l.605 -3.026h1.32a1 1 0 0 1 .986 1.164l-.167 1a1 1 0 0 1 -.986 .836h-1.653"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 7.5l-1 5.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M11.6 10h2.4l-.5 3" /> < title > { title } <
        / title > < / svg >
    }
}
