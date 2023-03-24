#[cfg(feature = "SiTunein")]
use leptos::*;
#[cfg(feature = "SiTunein")]
///This icon requires the feature `SiTunein` to be enabled.
#[component]
pub fn Tunein(
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
        "M7.66 11.398v.742c0 .105-.11.105-.11.105h-.847s-.11 0-.11.11v4.03c0 .11-.105.11-.105.11h-.855c-.106 0-.106-.11-.106-.11v-4.03s0-.11-.109-.11h-.844c-.105 0-.105-.105-.105-.105v-.742c0-.106.105-.106.105-.106H7.66v.106m15.458-7.52H12.301c-.68 0-.836.16-.836.816v2.414c0 .493 0 .493-.492.493H.813C.137 7.6 0 7.737 0 8.425v5.41c0 1.754 0 3.508.023 5.266 0 .922.102 1.02 1.04 1.02H9.89c.664 0 1.32.01 1.984-.01.48-.006.669-.202.669-.682v-2.56c0-.468 0-.468.469-.468h10.195c.633 0 .793-.152.793-.78V4.736c0-.7-.164-.86-.883-.86zm-11.64 14.625c0 .5-.013.5-.525.5-3.148 0-6.293 0-9.445.008-.32 0-.43-.078-.43-.418.016-3.16.008-6.324 0-9.48-.008-.34.086-.446.442-.446 3.187.012 6.363.008 9.55.008.117 0 .23.015.4.023 0 .18 0 .32.01.442-.003 3.113-.003 6.242-.003 9.363zm7.69-5.844c0 .102-.104.102-.104.102h-2.57c-.106 0-.106-.102-.106-.102v-.72c0-.1.105-.1.105-.1h.617s.102 0 .102-.102V8.659s0-.101-.102-.101h-.515c-.102 0-.102-.102-.102-.102v-.82c0-.106.102-.106.102-.106h2.367c.102 0 .102.106.102.106v.715c0 .105-.102.105-.102.105h-.516s-.101 0-.101.102v3.074s0 .105.1.105h.618c.106 0 .106.102.106.102z"
        /> < title > { title } < / title > < / svg >
    }
}
