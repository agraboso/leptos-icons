#[cfg(feature = "TiArrowMinimiseOutline")]
use leptos::*;
#[cfg(feature = "TiArrowMinimiseOutline")]
///This icon requires the feature `TiArrowMinimiseOutline` to be enabled.
#[component]
pub fn ArrowMinimiseOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M22 6c0-.801-.312-1.555-.879-2.121-.566-.567-1.32-.879-2.121-.879s-1.555.312-2.121.879l-.883.883c-.531-.474-1.23-.762-1.996-.762-.919 0-1.732.424-2.283 1.077-.212-.047-.431-.077-.656-.077h-6.061v6.06c0 .255.042.499.102.736-.598.549-.98 1.33-.98 2.204 0 .735.266 1.409.705 1.931l-.947.948c-.568.566-.88 1.32-.88 2.121s.312 1.555.879 2.121c.566.567 1.32.879 2.121.879.539 0 1.334-.152 2.061-.879l.903-.919c.535.495 1.251.798 2.036.798.934 0 1.758-.437 2.309-1.107.241.063.49.107.752.107h5.939v-6.061c0-.226-.029-.444-.077-.656.653-.55 1.077-1.364 1.077-2.283 0-.766-.288-1.465-.762-1.996l.883-.883c.567-.566.879-1.32.879-2.121zm-15 1h4c.553 0 1 .448 1 1s-.447 1-1 1h-2v2c0 .552-.448 1-1 1-.553 0-1-.448-1-1v-4zm12.707-.293l-3.293 3.293h1.586c.553 0 1 .448 1 1s-.448 1-1 1h-5v-5c0-.552.447-1 1-1s1 .448 1 1v1.586l3.293-3.293c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414zm-7.707 11.293c0 .552-.447 1-1 1s-1-.448-1-1v-1.707l-3.354 3.414c-.195.195-.39.293-.646.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l3.293-3.293h-1.465c-.553 0-1-.448-1-1s.447-1 1-1h4.879v5zm0-6h-2.278c.173-.295.278-.634.278-1v-1h1.061c.342 0 .658-.094.939-.245v2.245zm1 1h2.713c-.433.094-.713.33-.713.939v1.061h-.939c-.391 0-.752.117-1.061.311v-2.311zm.061 4c0-.552.447-1 1-1h1.939v-2c0-.552.447-1 1-1s1 .448 1 1v4h-3.939c-.553 0-1-.448-1-1z"
        /> < title > { title } < / title > < / svg >
    }
}
