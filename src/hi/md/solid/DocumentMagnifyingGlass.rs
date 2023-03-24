#[cfg(feature = "HiMdSolidDocumentMagnifyingGlass")]
use leptos::*;
#[cfg(feature = "HiMdSolidDocumentMagnifyingGlass")]
///This icon requires the feature `HiMdSolidDocumentMagnifyingGlass` to be enabled.
#[component]
pub fn DocumentMagnifyingGlass(
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
        "M8 10C8 9.17157 8.67157 8.5 9.5 8.5C10.3284 8.5 11 9.17157 11 10C11 10.4144 10.8329 10.7884 10.5607 11.0607C10.2884 11.3329 9.91442 11.5 9.5 11.5C8.67157 11.5 8 10.8284 8 10Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M4.5 2C3.67157 2 3 2.67157 3 3.5V16.5C3 17.3284 3.67157 18 4.5 18H15.5C16.3284 18 17 17.3284 17 16.5V7.62132C17 7.2235 16.842 6.84197 16.5607 6.56066L12.4393 2.43934C12.158 2.15804 11.7765 2 11.3787 2H4.5ZM9.5 7C7.84315 7 6.5 8.34315 6.5 10C6.5 11.6569 7.84315 13 9.5 13C10.056 13 10.5773 12.8483 11.0239 12.5845L12.2197 13.7803C12.5126 14.0732 12.9874 14.0732 13.2803 13.7803C13.5732 13.4874 13.5732 13.0126 13.2803 12.7197L12.0845 11.5239C12.3483 11.0773 12.5 10.556 12.5 10C12.5 8.34315 11.1569 7 9.5 7Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
