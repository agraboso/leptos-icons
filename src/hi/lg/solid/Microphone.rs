#[cfg(feature = "HiLgSolidMicrophone")]
use leptos::*;
#[cfg(feature = "HiLgSolidMicrophone")]
///This icon requires the feature `HiLgSolidMicrophone` to be enabled.
#[component]
pub fn Microphone(
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
        "M8.25 4.5C8.25 2.42893 9.92893 0.75 12 0.75C14.0711 0.75 15.75 2.42893 15.75 4.5V12.75C15.75 14.8211 14.0711 16.5 12 16.5C9.92893 16.5 8.25 14.8211 8.25 12.75V4.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 10.5C6.41421 10.5 6.75 10.8358 6.75 11.25V12.75C6.75 15.6495 9.1005 18 12 18C14.8995 18 17.25 15.6495 17.25 12.75V11.25C17.25 10.8358 17.5858 10.5 18 10.5C18.4142 10.5 18.75 10.8358 18.75 11.25V12.75C18.75 16.2244 16.125 19.0857 12.75 19.4588V21.75H15.75C16.1642 21.75 16.5 22.0858 16.5 22.5C16.5 22.9142 16.1642 23.25 15.75 23.25H8.25C7.83579 23.25 7.5 22.9142 7.5 22.5C7.5 22.0858 7.83579 21.75 8.25 21.75H11.25V19.4588C7.87504 19.0857 5.25 16.2244 5.25 12.75V11.25C5.25 10.8358 5.58579 10.5 6 10.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
