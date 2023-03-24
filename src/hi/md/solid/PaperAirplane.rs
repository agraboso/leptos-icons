#[cfg(feature = "HiMdSolidPaperAirplane")]
use leptos::*;
#[cfg(feature = "HiMdSolidPaperAirplane")]
///This icon requires the feature `HiMdSolidPaperAirplane` to be enabled.
#[component]
pub fn PaperAirplane(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.10526 2.28868C2.85362 2.25302 2.60104 2.34722 2.43421 2.53895C2.26738 2.73068 2.209 2.99386 2.27911 3.23815L3.69276 8.16402C3.87733 8.80713 4.4655 9.25024 5.13456 9.25024H11.25C11.6642 9.25024 12 9.58603 12 10.0002C12 10.4145 11.6642 10.7502 11.25 10.7502H5.13457C4.4655 10.7502 3.87733 11.1934 3.69277 11.8365L2.27911 16.7624C2.209 17.0067 2.26738 17.2698 2.43421 17.4616C2.60104 17.6533 2.85362 17.7475 3.10526 17.7118C8.94303 16.8844 14.221 14.319 18.3983 10.5576C18.5563 10.4154 18.6465 10.2128 18.6465 10.0003C18.6465 9.7877 18.5563 9.58513 18.3983 9.4429C14.221 5.68154 8.94303 3.1161 3.10526 2.28868Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
