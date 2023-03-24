#[cfg(feature = "HiLgSolidNewspaper")]
use leptos::*;
#[cfg(feature = "HiLgSolidNewspaper")]
///This icon requires the feature `HiLgSolidNewspaper` to be enabled.
#[component]
pub fn Newspaper(
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
        "M4.125 3C3.08947 3 2.25 3.83947 2.25 4.875V18C2.25 19.6569 3.59315 21 5.25 21H20.25C18.5931 21 17.25 19.6569 17.25 18V4.875C17.25 3.83947 16.4105 3 15.375 3H4.125ZM12 9.75C11.5858 9.75 11.25 10.0858 11.25 10.5C11.25 10.9142 11.5858 11.25 12 11.25H13.5C13.9142 11.25 14.25 10.9142 14.25 10.5C14.25 10.0858 13.9142 9.75 13.5 9.75H12ZM11.25 7.5C11.25 7.08579 11.5858 6.75 12 6.75H13.5C13.9142 6.75 14.25 7.08579 14.25 7.5C14.25 7.91421 13.9142 8.25 13.5 8.25H12C11.5858 8.25 11.25 7.91421 11.25 7.5ZM6 12.75C5.58579 12.75 5.25 13.0858 5.25 13.5C5.25 13.9142 5.58579 14.25 6 14.25H13.5C13.9142 14.25 14.25 13.9142 14.25 13.5C14.25 13.0858 13.9142 12.75 13.5 12.75H6ZM5.25 16.5C5.25 16.0858 5.58579 15.75 6 15.75H13.5C13.9142 15.75 14.25 16.0858 14.25 16.5C14.25 16.9142 13.9142 17.25 13.5 17.25H6C5.58579 17.25 5.25 16.9142 5.25 16.5ZM6 6.75C5.58579 6.75 5.25 7.08579 5.25 7.5V10.5C5.25 10.9142 5.58579 11.25 6 11.25H9C9.41421 11.25 9.75 10.9142 9.75 10.5V7.5C9.75 7.08579 9.41421 6.75 9 6.75H6Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.75 6.75H20.625C21.2463 6.75 21.75 7.25368 21.75 7.875V18C21.75 18.8284 21.0784 19.5 20.25 19.5C19.4216 19.5 18.75 18.8284 18.75 18V6.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
