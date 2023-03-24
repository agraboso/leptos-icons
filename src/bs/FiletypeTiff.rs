#[cfg(feature = "BsFiletypeTiff")]
use leptos::*;
#[cfg(feature = "BsFiletypeTiff")]
///This icon requires the feature `BsFiletypeTiff` to be enabled.
#[component]
pub fn FiletypeTiff(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-filetype-tiff" viewBox = "0 0 16 16" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" d =
        "M14 4.5V14a2 2 0 0 1-2 2h-1v-1h1a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM1.928 12.512v3.337h-.794v-3.337H0v-.662h3.064v.662H1.928Zm2.131-.662v3.999h-.79V11.85h.79Zm1.373 3.999v-1.59h1.606v-.64H5.432v-1.116H7.19v-.653H4.641v3.999h.791Zm2.868-1.59v1.59h-.791V11.85h2.548v.653H8.3v1.117h1.605v.638H8.3Z"
        /> < title > { title } < / title > < / svg >
    }
}
