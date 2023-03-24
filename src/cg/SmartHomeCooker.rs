#[cfg(feature = "CgSmartHomeCooker")]
use leptos::*;
#[cfg(feature = "CgSmartHomeCooker")]
///This icon requires the feature `CgSmartHomeCooker` to be enabled.
#[component]
pub fn SmartHomeCooker(
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
        "M15 16C15 17.6569 13.6569 19 12 19C10.3431 19 9 17.6569 9 16C9 14.3431 10.3431 13 12 13C13.6569 13 15 14.3431 15 16ZM13 16C13 16.5523 12.5523 17 12 17C11.4477 17 11 16.5523 11 16C11 15.4477 11.4477 15 12 15C12.5523 15 13 15.4477 13 16Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M15 1H9V3H11V5H7C4.79086 5 3 6.79086 3 9V19C3 21.2091 4.79086 23 7 23H17C19.2091 23 21 21.2091 21 19V9C21 6.79086 19.2091 5 17 5H13V3H15V1ZM17 7H7C5.89543 7 5 7.89543 5 9H19C19 7.89543 18.1046 7 17 7ZM19 11H5V19C5 20.1046 5.89543 21 7 21H17C18.1046 21 19 20.1046 19 19V11Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
