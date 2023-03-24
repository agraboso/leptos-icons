#[cfg(feature = "SiRadiopublic")]
use leptos::*;
#[cfg(feature = "SiRadiopublic")]
///This icon requires the feature `SiRadiopublic` to be enabled.
#[component]
pub fn Radiopublic(
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
        "M13.793 4.0692c3.0122-.2832 6.0245-1.9773 9.037-.5353v19.892c-4.1555-1.9892-8.3106 1.989-12.4656 0l-.0002-3.1517v-.3473c-3.0646.271-6.1294 1.988-9.1941.5392V.5742c4.2077 1.989 8.4155-1.989 12.6229 0zm2.2949 10.7988c.3625-.0852.725-.1737 1.0875-.2584 1.6054-.3793 2.2448-1.403 2.244-3.015V8.9974c.0004-1.5849-.4136-2.591-2.3648-2.1348-1.0875.2563-2.1747.5422-3.2626.6441v12.9706c.7652-.0717 1.5303-.2347 2.2959-.414zm0-2.1095V9.0973c.1435-.0335.2875-.068.431-.1023.5373-.1282.5864.1568.5876.6274-.0004.7207-.0004 1.441 0 2.1618-.0004.401-.0507.7437-.5876.872a82.0824 82.0824 0 0 1-.431.1023zm-9.148 4.3506v-5.5437c.1609-.0373.3223-.0754.4833-.1134.4667-.1102.6033.0151.6041.6253v3.7832c.0009.5754.0872.8338.2072.9464.7656-.1732 1.5304-.3227 2.2955-.3772v-.0872c-.0855-.1507-.1902-.3854-.1893-.9435v-4.4106c.0008-.9632-.622-1.3306-.9842-1.401.4303-.2724 1.019-.8019 1.0182-1.8793V5.6334c0-1.3907-.7072-2.1254-2.1057-1.8048-1.208.2724-2.4167.6057-3.625.7183v12.9705c.7655-.0712 1.5307-.2313 2.2958-.4083zm0-7.688V6.1437a73.198 73.198 0 0 0 .5177-.1217c.466-.1101.5693.041.5697.5988v1.761c0 .6101-.1715.8239-.6386.934-.1496.0353-.2993.071-.4489.1053z"
        /> < title > { title } < / title > < / svg >
    }
}
