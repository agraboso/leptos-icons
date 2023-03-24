#[cfg(feature = "FaSolidFingerprint")]
use leptos::*;
#[cfg(feature = "FaSolidFingerprint")]
///This icon requires the feature `FaSolidFingerprint` to be enabled.
#[component]
pub fn Fingerprint(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M454.4 93c7.3 8.8 6.1 21.6-2 29.7c-10.6 10.6-28.2 8.6-38-2.7C376.2 75.9 319.9 48 257 48C142.1 48 49 141.1 49 256v24.9c0 6.1-.2 12.2-.6 18.3C47.7 311.2 37.6 320 25.6 320C11.1 320 .1 307 .7 292.5c.2-3.9 .3-7.7 .3-11.6V256C1 114.6 115.6 0 257 0c79.4 0 150.4 36.2 197.4 93zm19.3 89.6c13.1-6.5 29-.2 32.4 14.1c4.5 19.1 6.9 39 6.9 59.4v24.9c0 5.4-.1 10.9-.2 16.3C512.6 310 502 320 489.2 320c-13.7 0-24.6-11.5-24.4-25.3c.1-4.6 .1-9.2 .1-13.8V256c0-15.1-1.6-29.8-4.6-43.9c-2.5-11.8 2.5-24.2 13.3-29.6zM257 80c97.2 0 176 78.8 176 176v24.9c0 27.7-1.7 55.3-5 82.7c-1.4 11.7-11.5 20.3-23.3 20.3c-14.7 0-25.9-13.2-24.2-27.8c3-24.9 4.4-50.1 4.4-75.3V256c0-70.7-57.3-128-128-128c-11.6 0-22.8 1.5-33.4 4.4c-10.6 2.9-22.3 .4-29.4-7.9c-10.4-12.1-6.9-30.9 8.3-35.9C219.6 83 238 80 257 80zM151.7 148.7c8.2 9.6 7.5 23.8 .2 34.2C137.5 203.6 129 228.8 129 256v24.9c0 28.9-3.3 57.7-9.7 85.8C116.9 377 107.6 384 97.1 384c-15.9 0-27.3-15.6-23.9-31.1c5.2-23.6 7.8-47.7 7.8-71.9V256c0-40.6 13.7-78 36.8-107.7c8.5-11 24.8-10.2 33.9 .4zM257 160c53 0 96 43 96 96v24.9c0 39.7-3.9 79.3-11.6 118.1c-2 10-10.8 17-21 17c-14.2 0-24.5-13.3-21.8-27.2c6.9-35.5 10.4-71.6 10.4-107.9V256c0-28.7-23.3-52-52-52s-52 23.3-52 52v24.9c0 40.5-5.3 80.7-15.9 119.7c-2.5 9.2-10.9 15.4-20.4 15.4c-14.8 0-25.3-14.6-21.5-29c9.1-34.6 13.8-70.2 13.8-106.1V256c0-53 43-96 96-96zm24 96v24.9c0 65.8-12.1 131-35.7 192.4l-5.9 15.3c-4.8 12.4-18.6 18.5-31 13.8s-18.5-18.6-13.8-31l5.9-15.3C222 400.2 233 340.8 233 280.9V256c0-13.3 10.7-24 24-24s24 10.7 24 24z"
        /> < title > { title } < / title > < / svg >
    }
}
