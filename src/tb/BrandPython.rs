#[cfg(feature = "TbBrandPython")]
use leptos::*;
#[cfg(feature = "TbBrandPython")]
///This icon requires the feature `TbBrandPython` to be enabled.
#[component]
pub fn BrandPython(
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
        "icon icon-tabler icon-tabler-brand-python" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9h-7a2 2 0 0 0 -2 2v4a2 2 0 0 0 2 2h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 15h7a2 2 0 0 0 2 -2v-4a2 2 0 0 0 -2 -2h-3"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 9v-4a2 2 0 0 1 2 -2h4a2 2 0 0 1 2 2v5a2 2 0 0 1 -2 2h-4a2 2 0 0 0 -2 2v5a2 2 0 0 0 2 2h4a2 2 0 0 0 2 -2v-4"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 6l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 18l0 .01" /> < title > { title } < / title
        > < / svg >
    }
}
