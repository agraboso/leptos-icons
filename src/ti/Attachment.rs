#[cfg(feature = "TiAttachment")]
use leptos::*;
#[cfg(feature = "TiAttachment")]
///This icon requires the feature `TiAttachment` to be enabled.
#[component]
pub fn Attachment(
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
        "M18.364 6.635c-1.561-1.559-4.1-1.559-5.658 0l-4.534 4.535c-.473.473-.733 1.1-.733 1.77 0 .668.261 1.295.732 1.768.487.486 1.128.73 1.769.73.64 0 1.279-.242 1.767-.73l2.122-2.121c.391-.395.586-.904.586-1.414 0-.512-.195-1.023-.586-1.414l-3.536 3.535c-.193.195-.511.195-.708-.002-.127-.127-.146-.275-.146-.352 0-.078.019-.227.146-.354l4.535-4.537c.778-.779 2.048-.779 2.83 0 .779.779.779 2.049 0 2.828l-4.537 4.537-2.535 2.535c-.779.779-2.049.779-2.828 0-.78-.779-.78-2.049 0-2.828l.095-.096c-.451-.6-.702-1.359-.702-2.125l-.807.807c-1.56 1.559-1.56 4.098 0 5.656.779.779 1.804 1.17 2.828 1.17s2.049-.391 2.828-1.17l7.072-7.072c1.56-1.559 1.56-4.096 0-5.656z"
        /> < title > { title } < / title > < / svg >
    }
}
