#[cfg(feature = "HiMdSolidMagnifyingGlassMinus")]
use leptos::*;
#[cfg(feature = "HiMdSolidMagnifyingGlassMinus")]
///This icon requires the feature `HiMdSolidMagnifyingGlassMinus` to be enabled.
#[component]
pub fn MagnifyingGlassMinus(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.75 8.25C6.33579 8.25 6 8.58579 6 9C6 9.41421 6.33579 9.75 6.75 9.75L11.25 9.75C11.6642 9.75 12 9.41421 12 9C12 8.58579 11.6642 8.25 11.25 8.25L6.75 8.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M9 2C5.13401 2 2 5.13401 2 9C2 12.866 5.13401 16 9 16C10.6625 16 12.1906 15.4197 13.3911 14.4517L16.7197 17.7803C17.0126 18.0732 17.4874 18.0732 17.7803 17.7803C18.0732 17.4874 18.0732 17.0126 17.7803 16.7197L14.4517 13.3911C15.4197 12.1906 16 10.6625 16 9C16 5.13401 12.866 2 9 2ZM3.5 9C3.5 5.96243 5.96243 3.5 9 3.5C12.0376 3.5 14.5 5.96243 14.5 9C14.5 10.519 13.8852 11.893 12.8891 12.8891C11.893 13.8852 10.519 14.5 9 14.5C5.96243 14.5 3.5 12.0376 3.5 9Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
