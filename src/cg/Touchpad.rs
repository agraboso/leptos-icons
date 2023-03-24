#[cfg(feature = "CgTouchpad")]
use leptos::*;
#[cfg(feature = "CgTouchpad")]
///This icon requires the feature `CgTouchpad` to be enabled.
#[component]
pub fn Touchpad(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M20 21C21.6569 21 23 19.6569 23 18V6C23 4.34315 21.6569 3 20 3H4C2.34315 3 1 4.34315 1 6V18C1 19.6569 2.34315 21 4 21H20ZM4 5H20C20.5523 5 21 5.44772 21 6V14H3V6C3 5.44772 3.44772 5 4 5ZM3 16V18C3 18.5523 3.44772 19 4 19H11V16H3ZM13 19V16H21V18C21 18.5523 20.5523 19 20 19H13Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
