#[cfg(feature = "CgSmileUpside")]
use leptos::*;
#[cfg(feature = "CgSmileUpside")]
///This icon requires the feature `CgSmileUpside` to be enabled.
#[component]
pub fn SmileUpside(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16 11H14C14 9.89543 13.1046 9 12 9C10.8954 9 10 9.89543 10 11H8C8 8.79086 9.79086 7 12 7C14.2091 7 16 8.79086 16 11Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 14C10 13.4477 9.55228 13 9 13C8.44772 13 8 13.4477 8 14C8 14.5523 8.44772 15 9 15C9.55228 15 10 14.5523 10 14Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 13C15.5523 13 16 13.4477 16 14C16 14.5523 15.5523 15 15 15C14.4477 15 14 14.5523 14 14C14 13.4477 14.4477 13 15 13Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C17.5228 22 22 17.5228 22 12ZM20 12C20 7.58172 16.4183 4 12 4C7.58172 4 4 7.58172 4 12C4 16.4183 7.58172 20 12 20C16.4183 20 20 16.4183 20 12Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
