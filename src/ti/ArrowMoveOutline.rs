#[cfg(feature = "TiArrowMoveOutline")]
use leptos::*;
#[cfg(feature = "TiArrowMoveOutline")]
///This icon requires the feature `TiArrowMoveOutline` to be enabled.
#[component]
pub fn ArrowMoveOutline(
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
        "M22.828 10.586l-9.414-9.414c-.391-.391-.902-.586-1.414-.586s-1.023.195-1.414.586l-9.414 9.414c-.781.779-.781 2.047 0 2.828l9.414 9.414c.391.391.902.586 1.414.586s1.023-.195 1.414-.586l9.414-9.414c.781-.781.781-2.049 0-2.828zm-5.828 5.414c-.256 0-.512-.098-.707-.293-.391-.391-.391-1.023 0-1.414l1.293-1.293h-4.586v4.586l1.293-1.293c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-3.707 3.707-3.707-3.707c-.391-.391-.391-1.023 0-1.414.195-.195.451-.293.707-.293s.512.098.707.293l1.293 1.293v-4.586h-4.586l1.293 1.293c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-3.707-3.707 3.707-3.707c.195-.195.451-.293.707-.293s.512.098.707.293c.391.391.391 1.023 0 1.414l-1.293 1.293h4.586v-4.586l-1.293 1.293c-.195.195-.451.293-.707.293s-.512-.098-.707-.293c-.391-.391-.391-1.023 0-1.414l3.707-3.707 3.707 3.707c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293s-.512-.098-.707-.293l-1.293-1.293v4.586h4.586l-1.293-1.293c-.391-.391-.391-1.023 0-1.414.195-.195.451-.293.707-.293s.512.098.707.293l3.707 3.707-3.707 3.707c-.195.195-.451.293-.707.293zm-1.732-2c-.175.301-.268.643-.268 1-.357 0-.699.093-1 .268v-1.268h1.268zm-6.536 0h1.268v1.268c-.301-.175-.643-.268-1-.268 0-.357-.093-.699-.268-1zm0-4c.175-.301.268-.643.268-1 .357 0 .699-.093 1-.268v1.268h-1.268zm6.536 0h-1.268v-1.268c.301.175.643.268 1 .268 0 .357.093.699.268 1z"
        /> < title > { title } < / title > < / svg >
    }
}
