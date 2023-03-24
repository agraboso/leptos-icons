#[cfg(feature = "TbDog")]
use leptos::*;
#[cfg(feature = "TbDog")]
///This icon requires the feature `TbDog` to be enabled.
#[component]
pub fn Dog(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-dog" width
        = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 5h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M19 12c-.667 5.333 -2.333 8 -5 8h-4c-2.667 0 -4.333 -2.667 -5 -8" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M11 16c0 .667 .333 1 1 1s1 -.333 1 -1h-2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 18v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 11v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 11v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5 4l6 .97l-6.238 6.688a1.021 1.021 0 0 1 -1.41 .111a.953 .953 0 0 1 -.327 -.954l1.975 -6.815z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 4l-6 .97l6.238 6.688c.358 .408 .989 .458 1.41 .111a.953 .953 0 0 0 .327 -.954l-1.975 -6.815z"
        /> < title > { title } < / title > < / svg >
    }
}
