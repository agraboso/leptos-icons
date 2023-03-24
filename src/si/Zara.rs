#[cfg(feature = "SiZara")]
use leptos::*;
#[cfg(feature = "SiZara")]
///This icon requires the feature `SiZara` to be enabled.
#[component]
pub fn Zara(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M8.562 7l.002.006 2.794 7.621v-7.23h-1.15v-.07h3.96c1.903 0 3.231.976 3.231 2.375 0 1.02-.91 1.868-2.263 2.109l-.249.031.25.026c.821.094 1.473.346 1.935.75l.003.003L19.141 7h.07l.002.006 3.556 9.698H24v.07h-3.918v-.07h1.154l-1.17-3.189h-2.373v.002l.013.037c.094.281.142.576.139.873v1.196c0 .615.271 1.238.79 1.238.304 0 .547-.107.837-.372l.041.038c-.314.332-.695.473-1.266.473-.43 0-.8-.104-1.096-.308l-.056-.04c-.39-.296-.644-.778-.753-1.435l-.018-.106-.018-.16-.002-.028-.654 1.78h.928v.07h-1.942v-.07h.938l.718-1.954v-.005a6.35 6.35 0 01-.013-.346v-.854c0-1.049-.78-1.65-2.14-1.65h-1.337v4.81h1.158v.07H9.433v-.07h1.154l-1.17-3.189H6.172l-1.158 3.154.048-.008c1.521-.262 2.22-1.423 2.23-2.645h.07v2.758H0l5.465-9.377H3.268c-1.822 0-2.646 1.407-2.659 2.81H.54v-2.88h6.634l-.04.07-5.425 9.307h2.854c.071 0 .141-.003.212-.009l.072-.006.09-.01L8.491 7h.07zm9.883 2.095l-1.313 3.576.007.007.067.066c.193.197.347.43.452.684l.007.017h2.375l-1.595-4.35zm-10.648 0l-1.599 4.35h3.194l-1.595-4.35zm6.026-1.698h-1.02v4.427h1.336c1.353 0 1.767-.493 1.767-2.107 0-1.517-.72-2.32-2.083-2.32z"
        /> < title > { title } < / title > < / svg >
    }
}
