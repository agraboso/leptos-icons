#[cfg(feature = "HiLgSolidPresentationChartLine")]
use leptos::*;
#[cfg(feature = "HiLgSolidPresentationChartLine")]
///This icon requires the feature `HiLgSolidPresentationChartLine` to be enabled.
#[component]
pub fn PresentationChartLine(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M2.25 2.25C1.83579 2.25 1.5 2.58579 1.5 3C1.5 3.41421 1.83579 3.75 2.25 3.75H3V14.25C3 15.9069 4.34315 17.25 6 17.25H7.20943L6.03849 20.7628C5.9075 21.1558 6.11987 21.5805 6.51283 21.7115C6.90579 21.8425 7.33053 21.6301 7.46151 21.2372L7.79057 20.25H16.2094L16.5385 21.2372C16.6695 21.6301 17.0942 21.8425 17.4872 21.7115C17.8801 21.5805 18.0925 21.1558 17.9615 20.7628L16.7906 17.25H18C19.6569 17.25 21 15.9069 21 14.25V3.75H21.75C22.1642 3.75 22.5 3.41421 22.5 3C22.5 2.58579 22.1642 2.25 21.75 2.25H2.25ZM8.79057 17.25H15.2094L15.7094 18.75H8.29057L8.79057 17.25ZM16.8755 8.25467C17.2341 8.04727 17.3566 7.58847 17.1492 7.22992C16.9418 6.87138 16.483 6.74886 16.1245 6.95626C14.7577 7.74688 13.5517 8.78371 12.5667 10.0061L11.0303 8.46975C10.7374 8.17686 10.2626 8.17686 9.96967 8.46975L6.96967 11.4698C6.67678 11.7626 6.67678 12.2375 6.96967 12.5304C7.26256 12.8233 7.73744 12.8233 8.03033 12.5304L10.5 10.0607L12.1173 11.678C12.2742 11.835 12.4927 11.9143 12.7138 11.8947C12.9349 11.8751 13.136 11.7586 13.2629 11.5765C14.207 10.2217 15.4414 9.08428 16.8755 8.25467Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
