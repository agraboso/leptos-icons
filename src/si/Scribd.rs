#[cfg(feature = "SiScribd")]
use leptos::*;
#[cfg(feature = "SiScribd")]
///This icon requires the feature `SiScribd` to be enabled.
#[component]
pub fn Scribd(
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
        "M14.839 21.059c0-2.123 1.572-3.939 3.543-4.307-.094-1.973-.924-3.328-2.219-4.343-1.305-1.016-3.121-1.785-5.088-2.557-2.13-.832-3.721-1.569-3.721-3.172 0-1.352 1.17-2.369 3.137-2.369 1.201 0 1.974.185 2.371.399 0 .093-.029.216-.09.309-.061.123-.09.276-.09.493 0 1.354.983 2.555 2.738 2.555 1.602 0 2.555-1.201 2.555-2.957 0-1.477-.832-2.77-2.188-3.663C14.409.555 12.487 0 10.312 0 8.06.155 6.123.985 4.77 2.217 3.415 3.447 2.615 5.111 2.615 7.08c0 2.187.77 3.603 2.031 4.683 1.262 1.077 3.078 1.846 5.265 2.616 2.372.764 3.757 1.561 3.757 3.137 0 1.59-1.385 2.551-3.572 2.551-1.141 0-2.124-.193-2.957-.764.219-.406.219-.813.219-1.201 0-1.143-1.006-2.568-2.764-2.568-1.56 0-2.73 1.201-2.73 2.957 0 1.471.93 2.867 2.445 3.844C5.794 23.354 7.88 24 10.132 24c1.982 0 3.768-.375 5.148-1.365-.21-.406-.39-.992-.39-1.607l-.051.031zm7.299 0c0 1.572-1.275 2.773-2.777 2.773-1.5 0-2.746-1.201-2.746-2.771 0-1.5 1.23-2.732 2.73-2.732 1.502 0 2.764 1.232 2.764 2.748l.029-.018z"
        /> < title > { title } < / title > < / svg >
    }
}
