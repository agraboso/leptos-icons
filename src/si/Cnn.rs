#[cfg(feature = "SiCnn")]
use leptos::*;
#[cfg(feature = "SiCnn")]
///This icon requires the feature `SiCnn` to be enabled.
#[component]
pub fn Cnn(
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
        "M23.9962 15.514c0 2.0638-2.6676 3.0547-4.0789.6576-.1012-.173-2.3252-4.0032-2.3252-4.0032v3.3457c0 2.0637-2.6663 3.0546-4.0776.6575-.1025-.173-2.3253-4.0032-2.3253-4.0032v3.1547c0 1.4318-.8498 2.2073-2.1791 2.2073H5.5299a5.5299 5.5299 0 010-11.0598h1.7946v1.328H5.5299a4.2019 4.2019 0 100 8.4038h3.4494a.8973.8973 0 00.8794-.878V8.524a.2692.2692 0 01.1935-.273c.141-.0384.2897.0487.3987.2333l2.1522 3.7084c1.251 2.1573 2.0728 3.5738 2.083 3.5892.2807.4742.6986.5576.9973.4755a.7973.7973 0 00.582-.787v-6.945a.2705.2705 0 01.191-.2744c.1397-.0384.287.0487.3947.2333l1.9946 3.4366 2.242 3.8648c.2191.3717.5242.5038.7896.5038a.7691.7691 0 00.2063-.0282.7986.7986 0 00.591-.791V6.4707H24zM8.0026 13.9695V8.4857c0-2.0638 2.6675-3.0546 4.0788-.6563.1025.173 2.3253 4.002 2.3253 4.002V8.4856c0-2.0638 2.6662-3.0546 4.0775-.6563.1026.173 2.3253 4.002 2.3253 4.002V6.4705H22.14v8.9999a.2705.2705 0 01-.1935.2743c-.141.0384-.2897-.0487-.3987-.2333a1360.4277 1360.4277 0 01-2.2406-3.8622l-1.9946-3.434c-.2794-.4744-.696-.5577-.9921-.477a.7986.7986 0 00-.5833.7858v6.9464a.2718.2718 0 01-.1935.2743c-.1423.0384-.291-.0487-.3987-.2333-.0192-.032-1.069-1.8407-2.083-3.5892a6211.7971 6211.7971 0 00-2.1535-3.711c-.2794-.4755-.6973-.5575-.996-.4768a.7999.7999 0 00-.5845.7858v6.8002a.3717.3717 0 01-.3487.3474h-3.452a3.6712 3.6712 0 010-7.3424H7.322v1.328H5.5427a2.3432 2.3432 0 100 4.6864H7.636a.364.364 0 00.3666-.3705Z"
        /> < title > { title } < / title > < / svg >
    }
}
