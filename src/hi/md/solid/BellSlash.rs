#[cfg(feature = "HiMdSolidBellSlash")]
use leptos::*;
#[cfg(feature = "HiMdSolidBellSlash")]
///This icon requires the feature `HiMdSolidBellSlash` to be enabled.
#[component]
pub fn BellSlash(
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
        "M3.99997 8C3.99997 7.73938 4.01658 7.48265 4.04881 7.23079L11.7713 14.9533C11.1847 14.9843 10.5942 15 9.99997 15C9.34517 15 8.69477 14.9809 8.0493 14.9433C8.25097 15.8345 9.04777 16.5 10 16.5C10.8982 16.5 11.6581 15.9079 11.9107 15.0927L13.045 16.227C12.4432 17.2858 11.305 18 10 18C8.17421 18 6.67494 16.602 6.51426 14.818C5.41181 14.7023 4.32537 14.5322 3.25769 14.3105C3.02851 14.2629 2.83469 14.1111 2.73365 13.9C2.6326 13.6888 2.63591 13.4426 2.7426 13.2343C3.54624 11.6651 3.99997 9.88663 3.99997 8Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.2663 13.9C17.2467 13.9408 17.2237 13.9795 17.1976 14.0156L6.38945 3.20747C7.39404 2.44946 8.64452 2 9.99997 2C13.3137 2 16 4.68629 16 8C16 9.88663 16.4537 11.6651 17.2573 13.2343C17.364 13.4426 17.3673 13.6888 17.2663 13.9Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.28033 2.21967C2.98744 1.92678 2.51256 1.92678 2.21967 2.21967C1.92678 2.51256 1.92678 2.98744 2.21967 3.28033L16.7197 17.7803C17.0126 18.0732 17.4874 18.0732 17.7803 17.7803C18.0732 17.4874 18.0732 17.0126 17.7803 16.7197L3.28033 2.21967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
