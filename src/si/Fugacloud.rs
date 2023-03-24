#[cfg(feature = "SiFugacloud")]
use leptos::*;
#[cfg(feature = "SiFugacloud")]
///This icon requires the feature `SiFugacloud` to be enabled.
#[component]
pub fn Fugacloud(
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
        "m19.0876 13.9765-1.1014-.6361v-2.5345l1.1013.6389 3.1738-1.8305v2.5275zm-14.1752.01h-.0067l-3.167-1.8341V9.625l3.1702 1.8284 1.0982-.6371v2.5345zm.0592-3.5472L1.7387 8.5686V5.9464l.971-.5568L11.9941 0l1.9539 1.1371 5.9567 3.4708.2013.1303 1.0303.5922.1657.095.9593.5686v2.5597l-3.1738 1.8349-1.1013-.6353V8.4341l-4.95-2.8903-1.0421-.6277-.971.5567-5.033 2.9495v1.4254l-1.0185.5883m0 4.4197 1.0184-.5805v1.2082l6.0633 3.5418.225-.1421.0473-.024 5.6725-3.3168-.0119-.4264v-.8918l1.1014.6313 3.1737-1.833v5.0193l-.9593.5568-1.196.6988-.2013.1185-5.9567 3.4826L11.994 24l-1.9302-1.1371-5.9685-3.4708-.2013-.1303-1.1842-.687-.9711-.5698v-4.6197l.0118-.3971 3.2211 1.8673"
        /> < title > { title } < / title > < / svg >
    }
}
