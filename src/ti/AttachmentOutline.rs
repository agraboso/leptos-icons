#[cfg(feature = "TiAttachmentOutline")]
use leptos::*;
#[cfg(feature = "TiAttachmentOutline")]
///This icon requires the feature `TiAttachmentOutline` to be enabled.
#[component]
pub fn AttachmentOutline(
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
        "M15.534 4.466c1.024 0 2.05.39 2.829 1.169 1.561 1.561 1.561 4.098 0 5.656l-7.071 7.072c-.778.779-1.804 1.17-2.828 1.17s-2.049-.391-2.828-1.17c-1.56-1.559-1.56-4.098 0-5.656l.807-.807c-.004.805.25 1.524.701 2.125l-.094.096c-.78.779-.78 2.049 0 2.828.39.39.901.584 1.414.584s1.024-.195 1.414-.584l2.535-2.535 4.537-4.537c.778-.779.778-2.049 0-2.828-.392-.39-.904-.584-1.417-.584-.512 0-1.023.195-1.413.584l-4.535 4.537c-.128.127-.146.275-.146.354 0 .076.019.226.146.353.099.099.228.147.356.147.127 0 .254-.049.352-.146l2.122-2.121 1.414-1.414c.392.392.586.902.586 1.414 0 .511-.194 1.021-.584 1.41l-2.124 2.125c-.486.487-1.127.729-1.768.729s-1.28-.244-1.769-.729c-.472-.474-.731-1.101-.731-1.769 0-.67.261-1.297.732-1.77l4.534-4.535c.779-.779 1.805-1.168 2.829-1.168m0-2c-1.604 0-3.11.623-4.242 1.755l-7.069 7.073c-1.133 1.131-1.757 2.638-1.757 4.242s.624 3.11 1.757 4.243c1.131 1.132 2.639 1.755 4.241 1.755s3.11-.624 4.242-1.757l7.071-7.071c1.133-1.131 1.757-2.638 1.757-4.242 0-1.603-.623-3.11-1.755-4.241-1.133-1.134-2.64-1.757-4.245-1.757z"
        /> < title > { title } < / title > < / svg >
    }
}
