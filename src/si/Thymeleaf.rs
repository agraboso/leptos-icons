#[cfg(feature = "SiThymeleaf")]
use leptos::*;
#[cfg(feature = "SiThymeleaf")]
///This icon requires the feature `SiThymeleaf` to be enabled.
#[component]
pub fn Thymeleaf(
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
        "M1.727 0C.782 0 .02.761.02 1.707v20.586C.02 23.24.782 24 1.727 24h20.546c.945 0 1.707-.761 1.707-1.707V1.707C23.98.76 23.218 0 22.273 0H1.727zm18.714 3.273c-1.861 3.694-3.3 7.627-5.674 11.046-1.064 1.574-2.329 3.163-4.16 3.86-1.31.552-2.936.337-3.98-.647-.628-.523-.54-1.43-.173-2.075.96-1.224 2.34-2.02 3.59-2.915 3.842-2.625 7.446-5.654 10.397-9.27zm-1.693 1.25c-2.503 2.751-5.381 5.16-8.452 7.269l-.003.002-.003.003c-1.327.979-2.835 1.824-3.993 3.114-.349.333-.583 1.042-.537 1.481-.622-1.043-.8-2.614-.257-3.74.526-1.19 1.742-1.807 2.876-2.292 3.757-1.353 6.695-2.926 10.369-5.836z"
        /> < title > { title } < / title > < / svg >
    }
}
