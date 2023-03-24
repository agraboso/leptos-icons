#[cfg(feature = "SiZapier")]
use leptos::*;
#[cfg(feature = "SiZapier")]
///This icon requires the feature `SiZapier` to be enabled.
#[component]
pub fn Zapier(
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
        "M15 12.004c0 .893-.165 1.746-.461 2.535-.787.297-1.643.461-2.535.461h-.009c-.893 0-1.745-.165-2.534-.461C9.164 13.75 9 12.896 9 12.004v-.009c0-.893.164-1.745.461-2.534C10.25 9.164 11.103 9 11.995 9h.009c.893 0 1.748.164 2.535.462.297.788.461 1.641.461 2.535v.007zM23.835 10H16.83l4.948-4.952c-.39-.548-.82-1.06-1.295-1.533-.473-.474-.985-.907-1.53-1.296l-4.954 4.949V.165C13.35.061 12.686 0 12.004 0h-.01c-.68 0-1.346.061-1.995.165V7.17l-4.95-4.949c-.549.386-1.06.821-1.534 1.294-.474.474-.908.987-1.296 1.533L7.168 10H.165S0 11.316 0 11.995v.009c0 .68.061 1.348.165 1.995H7.17l-4.949 4.952c.777 1.096 1.733 2.051 2.827 2.83L10 16.831v7.004c.648.105 1.313.165 1.991.165h.017c.679 0 1.344-.06 1.991-.165v-7.004l4.952 4.95c.548-.375 1.06-.812 1.529-1.29h.005c.473-.465.906-.976 1.296-1.531l-4.95-4.949h7.004c.105-.645.165-1.304.165-1.98V12c0-.678-.06-1.343-.165-1.99"
        /> < title > { title } < / title > < / svg >
    }
}
