#[cfg(feature = "OcSmUnverified")]
use leptos::*;
#[cfg(feature = "OcSmUnverified")]
///This icon requires the feature `OcSmUnverified` to be enabled.
#[component]
pub fn Unverified(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.415.52a2.677 2.677 0 0 1 3.17 0l.928.68c.153.113.33.186.518.215l1.138.175a2.678 2.678 0 0 1 2.241 2.24l.175 1.138c.029.187.102.365.215.518l.68.928a2.677 2.677 0 0 1 0 3.17l-.68.928a1.186 1.186 0 0 0-.215.518l-.175 1.138a2.678 2.678 0 0 1-2.241 2.241l-1.138.175a1.186 1.186 0 0 0-.518.215l-.928.68a2.677 2.677 0 0 1-3.17 0l-.928-.68a1.186 1.186 0 0 0-.518-.215L3.83 14.41a2.678 2.678 0 0 1-2.24-2.24l-.175-1.138a1.186 1.186 0 0 0-.215-.518l-.68-.928a2.677 2.677 0 0 1 0-3.17l.68-.928a1.17 1.17 0 0 0 .215-.518l.175-1.14a2.678 2.678 0 0 1 2.24-2.24l1.138-.175c.187-.029.365-.102.518-.215l.928-.68Zm2.282 1.209a1.18 1.18 0 0 0-1.394 0l-.928.68a2.67 2.67 0 0 1-1.18.489l-1.136.174a1.18 1.18 0 0 0-.987.987l-.174 1.137a2.67 2.67 0 0 1-.489 1.18l-.68.927c-.305.415-.305.98 0 1.394l.68.928c.256.348.423.752.489 1.18l.174 1.136c.078.51.478.909.987.987l1.137.174c.427.066.831.233 1.18.489l.927.68c.415.305.98.305 1.394 0l.928-.68a2.67 2.67 0 0 1 1.18-.489l1.136-.174c.51-.078.909-.478.987-.987l.174-1.137c.066-.427.233-.831.489-1.18l.68-.927c.305-.415.305-.98 0-1.394l-.68-.928a2.67 2.67 0 0 1-.489-1.18l-.174-1.136a1.18 1.18 0 0 0-.987-.987l-1.137-.174a2.67 2.67 0 0 1-1.18-.489ZM6.92 6.085h.001a.75.75 0 0 1-1.342-.67c.169-.339.436-.701.849-.977C6.846 4.16 7.369 4 8 4a2.76 2.76 0 0 1 1.638.525c.502.377.862.965.862 1.725 0 .448-.115.83-.329 1.15-.205.307-.47.513-.692.662-.109.072-.22.138-.313.195l-.006.004a6.24 6.24 0 0 0-.26.16.952.952 0 0 0-.276.245.75.75 0 0 1-1.248-.832c.184-.264.42-.489.692-.661.109-.073.22-.139.313-.195l.007-.004c.1-.061.182-.11.258-.161a.969.969 0 0 0 .277-.245C8.96 6.514 9 6.427 9 6.25a.612.612 0 0 0-.262-.525A1.27 1.27 0 0 0 8 5.5c-.369 0-.595.09-.74.187a1.01 1.01 0 0 0-.34.398ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
