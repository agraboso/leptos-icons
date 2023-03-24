#[cfg(feature = "HiLgSolidAdjustmentsVertical")]
use leptos::*;
#[cfg(feature = "HiLgSolidAdjustmentsVertical")]
///This icon requires the feature `HiLgSolidAdjustmentsVertical` to be enabled.
#[component]
pub fn AdjustmentsVertical(
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
        "M6 12C5.58579 12 5.25 11.6642 5.25 11.25L5.25002 3.75C5.25002 3.33578 5.5858 3 6.00002 3C6.41423 3 6.75002 3.33579 6.75002 3.75L6.75 11.25C6.75 11.6642 6.41421 12 6 12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 12C17.5858 12 17.25 11.6642 17.25 11.25L17.25 3.75C17.25 3.33578 17.5858 3 18 3C18.4142 3 18.75 3.33579 18.75 3.75L18.75 11.25C18.75 11.6642 18.4142 12 18 12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.75001 20.25L6.75 18.75C6.75 18.3358 6.41421 18 5.99999 18C5.58578 18 5.25 18.3358 5.25 18.75L5.25001 20.25C5.25002 20.6642 5.58581 21 6.00002 21C6.41424 21 6.75002 20.6642 6.75001 20.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.75 18.75L18.75 20.25C18.75 20.6642 18.4142 21 18 21C17.5858 21 17.25 20.6642 17.25 20.25L17.25 18.75C17.25 18.3358 17.5858 18 18 18C18.4142 18 18.75 18.3358 18.75 18.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.75 5.24999L12.75 3.74999C12.75 3.33578 12.4142 3 12 3C11.5858 3 11.25 3.33579 11.25 3.75001L11.25 5.25001C11.25 5.66422 11.5858 6 12 6C12.4142 6 12.75 5.66421 12.75 5.24999Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 21C11.5858 21 11.25 20.6642 11.25 20.25V12.75C11.25 12.3358 11.5858 12 12 12C12.4142 12 12.75 12.3358 12.75 12.75V20.25C12.75 20.6642 12.4142 21 12 21Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 15C3.75 16.2426 4.75736 17.25 6 17.25C7.24264 17.25 8.25 16.2426 8.25 15C8.25 13.7574 7.24264 12.75 6 12.75C4.75736 12.75 3.75 13.7574 3.75 15Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 11.25C10.7574 11.25 9.75 10.2426 9.75 9C9.75 7.75736 10.7574 6.75 12 6.75C13.2426 6.75 14.25 7.75736 14.25 9C14.25 10.2426 13.2426 11.25 12 11.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.75 15C15.75 16.2426 16.7574 17.25 18 17.25C19.2426 17.25 20.25 16.2426 20.25 15C20.25 13.7574 19.2426 12.75 18 12.75C16.7574 12.75 15.75 13.7574 15.75 15Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
