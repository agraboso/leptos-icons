#[cfg(feature = "TiCloudStorageOutline")]
use leptos::*;
#[cfg(feature = "TiCloudStorageOutline")]
///This icon requires the feature `TiCloudStorageOutline` to be enabled.
#[component]
pub fn CloudStorageOutline(
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
        "M16.5 8.999l-.352.015c-.824-2.375-3.312-4.015-5.898-4.015-3.309 0-6.25 2.69-6.25 6v.126c-1 .445-2.75 2.014-2.75 3.875 0 2.206 2.044 4 4.25 4h11c2.757 0 5-2.244 5-5s-2.243-5.001-5-5.001zm0 8.001h-4.5v-3.794l2.146 2.146c.098.099.226.146.354.146s.256-.049.354-.146c.195-.194.195-.512 0-.707l-2.998-3-.164-.107c-.123-.051-.26-.051-.383 0l-.162.107-3 3c-.194.195-.194.513 0 .707.099.099.227.146.354.146s.256-.049.354-.146l2.145-2.146v3.794h-5.5c-1.104 0-2-.896-2-2s.896-2 1.908-2.005l1.422.015-.248-1.201c-.055-.264-.082-.536-.082-.809 0-2.206 1.794-4 4-4 1.951 0 3.604 1.402 3.93 3.334l.187 1.102 1.073-.306c.312-.089.569-.13.812-.13 1.653 0 3 1.346 3 3s-1.348 3-3.002 3z"
        /> < title > { title } < / title > < / svg >
    }
}
