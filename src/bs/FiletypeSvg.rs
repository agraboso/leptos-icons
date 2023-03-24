#[cfg(feature = "BsFiletypeSvg")]
use leptos::*;
#[cfg(feature = "BsFiletypeSvg")]
///This icon requires the feature `BsFiletypeSvg` to be enabled.
#[component]
pub fn FiletypeSvg(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-filetype-svg" viewBox = "0 0 16 16" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" d =
        "M14 4.5V14a2 2 0 0 1-2 2v-1a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM0 14.841a1.13 1.13 0 0 0 .401.823c.13.108.288.192.478.252.19.061.411.091.665.091.338 0 .624-.053.858-.158.237-.105.416-.252.54-.44a1.17 1.17 0 0 0 .187-.656c0-.224-.045-.41-.135-.56a1 1 0 0 0-.375-.357 2.027 2.027 0 0 0-.565-.21l-.621-.144a.97.97 0 0 1-.405-.176.37.37 0 0 1-.143-.299c0-.156.061-.284.184-.384.125-.101.296-.152.513-.152.143 0 .266.023.37.068a.625.625 0 0 1 .245.181.56.56 0 0 1 .12.258h.75a1.092 1.092 0 0 0-.199-.566 1.21 1.21 0 0 0-.5-.41 1.813 1.813 0 0 0-.78-.152c-.293 0-.552.05-.776.15-.225.099-.4.24-.528.421-.127.182-.19.395-.19.639 0 .201.04.376.123.524.082.149.199.27.351.367.153.095.332.167.54.213l.618.144c.207.049.36.113.462.193a.387.387 0 0 1 .153.326.512.512 0 0 1-.085.29.559.559 0 0 1-.256.193c-.111.047-.249.07-.413.07-.117 0-.224-.013-.32-.04a.837.837 0 0 1-.248-.115.578.578 0 0 1-.255-.384H0Zm4.575 1.09h.952l1.327-3.999h-.879l-.887 3.138H5.05l-.897-3.138h-.917l1.339 4Zm5.483-3.293c.076.152.123.316.14.492h-.776a.797.797 0 0 0-.096-.249.689.689 0 0 0-.17-.19.707.707 0 0 0-.237-.126.963.963 0 0 0-.3-.044c-.284 0-.506.1-.664.302-.157.2-.235.484-.235.85v.497c0 .235.033.44.097.616a.881.881 0 0 0 .305.413.87.87 0 0 0 .518.146.965.965 0 0 0 .457-.097.67.67 0 0 0 .273-.263c.06-.11.09-.23.09-.364v-.254h-.823v-.59h1.576v.798c0 .193-.032.377-.096.55a1.29 1.29 0 0 1-.293.457 1.37 1.37 0 0 1-.495.314c-.198.074-.43.111-.698.111a1.98 1.98 0 0 1-.752-.132 1.447 1.447 0 0 1-.534-.377 1.58 1.58 0 0 1-.319-.58 2.482 2.482 0 0 1-.105-.745v-.507c0-.36.066-.677.199-.949.134-.271.329-.482.583-.633.256-.152.564-.228.926-.228.238 0 .45.033.635.1.188.066.348.158.48.275.134.117.238.253.314.407Z"
        /> < title > { title } < / title > < / svg >
    }
}
