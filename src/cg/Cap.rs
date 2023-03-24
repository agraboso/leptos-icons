#[cfg(feature = "CgCap")]
use leptos::*;
#[cfg(feature = "CgCap")]
///This icon requires the feature `CgCap` to be enabled.
#[component]
pub fn Cap(
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
        "http://www.w3.org/2000/svg" d = "M8 18V20H16V18H8Z" fill = "currentColor" /><
        path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule =
        "evenodd" d =
        "M13.988 3.2203C13.9959 3.14796 14 3.07446 14 3C14 1.89543 13.1046 1 12 1C10.8954 1 10 1.89543 10 3C10 3.07446 10.0041 3.14796 10.012 3.2203C5.99799 4.12533 3 7.71255 3 12C3 12.3883 3.02459 12.7709 3.0723 13.1462C1.86949 13.5369 1 14.6669 1 16V20C1 21.6569 2.34315 23 4 23H20C21.6569 23 23 21.6569 23 20V16C23 14.6669 22.1305 13.5369 20.9277 13.1462C20.9754 12.7709 21 12.3883 21 12C21 7.71255 18.002 4.12533 13.988 3.2203ZM12 5C8.13401 5 5 8.13401 5 12C5 12.3402 5.02412 12.674 5.07063 13H18.9294C18.9759 12.674 19 12.3402 19 12C19 8.13401 15.866 5 12 5ZM3 16C3 15.4477 3.44772 15 4 15H20C20.5523 15 21 15.4477 21 16V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V16Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
