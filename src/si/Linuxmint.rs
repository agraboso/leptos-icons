#[cfg(feature = "SiLinuxmint")]
use leptos::*;
#[cfg(feature = "SiLinuxmint")]
///This icon requires the feature `SiLinuxmint` to be enabled.
#[component]
pub fn Linuxmint(
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
        "M0 1.693v4.193h1.828c1.276 0 1.502.865 1.502 2.058l.01 7.412c0 3.84 3.44 6.951 7.68 6.951h10.464c1.342 0 2.516-.83 2.516-2.108V8.706c0-3.84-3.44-6.95-7.683-6.95h-4.405v-.013L0 1.693zm5.723 2.566h2.102V14.82c0 1.413.984 2.51 2.139 2.51l7.17.03c1.496 0 2.661-1.01 2.661-2.206l-.012-5.607a1.2 1.2 0 0 0-.386-.91 1.224 1.224 0 0 0-.917-.384c-.374 0-.65.12-.918.384a1.2 1.2 0 0 0-.386.91v4.798h-2.223V9.548c0-.364-.124-.648-.389-.91a1.208 1.208 0 0 0-.917-.384c-.366 0-.647.12-.914.384-.265.262-.39.546-.39.91v4.798H10.12V9.548c0-.95.36-1.792 1.042-2.466a3.445 3.445 0 0 1 2.485-1.022c.937 0 1.752.345 2.413.97a3.448 3.448 0 0 1 2.42-.97c.954 0 1.803.348 2.485 1.022a3.385 3.385 0 0 1 1.041 2.466l.009 5.991c-.105 1.004-.539 1.894-1.28 2.637h-.002a4.367 4.367 0 0 1-3.174 1.314H9.574v-.038c-.976-.103-1.846-.519-2.57-1.217-.845-.825-1.281-1.846-1.281-3.01V4.26z"
        /> < title > { title } < / title > < / svg >
    }
}
