#[cfg(feature = "SiPresto")]
use leptos::*;
#[cfg(feature = "SiPresto")]
///This icon requires the feature `SiPresto` to be enabled.
#[component]
pub fn Presto(
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
        "M1.3716 0C.6184 0 .0009.618.0009 1.3724v21.256C.0009 23.3825.6184 24 1.3716 24h21.256c.7545 0 1.3715-.6175 1.3715-1.3716V1.3724c0-.754-.617-1.3724-1.3715-1.3724zm11.9596 4.4849a.854.854 0 0 1 .032 0 .854.854 0 0 1 .854.854.854.854 0 0 1-.854.854.854.854 0 0 1-.854-.854.854.854 0 0 1 .822-.854zm-4.4458.094a.7595.7595 0 0 1 .007 0 .7595.7595 0 0 1 .76.76.7595.7595 0 0 1-.76.759.7595.7595 0 0 1-.759-.759.7595.7595 0 0 1 .752-.76zm-4.4636.095a.6645.6645 0 0 1 .664.665.6645.6645 0 0 1-.664.664.6645.6645 0 0 1-.6649-.664.6645.6645 0 0 1 .665-.665zm11.1766 1.9513a.9486.9486 0 0 1 .949.949.9486.9486 0 0 1-.949.948.9486.9486 0 0 1-.949-.948.9486.9486 0 0 1 .949-.949zm-4.5026.094a.8545.8545 0 0 1 .032 0 .8545.8545 0 0 1 .8549.854.8545.8545 0 0 1-.8549.855.8545.8545 0 0 1-.8549-.855.8545.8545 0 0 1 .823-.854zm-4.4458.095a.7595.7595 0 0 1 .0071 0 .7595.7595 0 0 1 .76.759.7595.7595 0 0 1-.76.76.7595.7595 0 0 1-.759-.76.7595.7595 0 0 1 .752-.759zm11.167 1.9513a1.0436 1.0436 0 0 1 .0177 0 1.0436 1.0436 0 0 1 1.043 1.0431 1.0436 1.0436 0 0 1-1.043 1.044 1.0436 1.0436 0 0 1-1.044-1.044 1.0436 1.0436 0 0 1 1.0262-1.043zm-4.4539.095a.9486.9486 0 0 1 .949.949.9486.9486 0 0 1-.949.9481.9486.9486 0 0 1-.949-.948.9486.9486 0 0 1 .949-.949zm-4.4706.0941a.854.854 0 0 1 .8549.854.854.854 0 0 1-.855.854.854.854 0 0 1-.854-.854.854.854 0 0 1 .854-.854zM20.069 10.906a1.1386 1.1386 0 0 1 1.139 1.139 1.1386 1.1386 0 0 1-1.139 1.138 1.1386 1.1386 0 0 1-1.139-1.138 1.1386 1.1386 0 0 1 1.139-1.139zm-4.4698.095a1.0436 1.0436 0 0 1 1.0431 1.044 1.0436 1.0436 0 0 1-1.043 1.043 1.0436 1.0436 0 0 1-1.044-1.043 1.0436 1.0436 0 0 1 1.044-1.044zm-4.4715.094a.949.949 0 0 1 .9499.949.949.949 0 0 1-.9499.949.949.949 0 0 1-.949-.949.949.949 0 0 1 .949-.949zm6.706 2.0463a1.1386 1.1386 0 0 1 1.139 1.139 1.1386 1.1386 0 0 1-1.139 1.138 1.1386 1.1386 0 0 1-1.139-1.138 1.1386 1.1386 0 0 1 1.139-1.139zm-4.4884.095a1.0436 1.0436 0 0 1 .0177 0 1.0436 1.0436 0 0 1 1.044 1.043 1.0436 1.0436 0 0 1-1.044 1.044 1.0436 1.0436 0 0 1-1.044-1.044 1.0436 1.0436 0 0 1 1.0263-1.043zm-4.46.284a.7595.7595 0 0 1 .007 0 .7595.7595 0 0 1 .76.759.7595.7595 0 0 1-.76.76.7595.7595 0 0 1-.759-.76.7595.7595 0 0 1 .752-.759zm6.6793 1.8563a1.1386 1.1386 0 0 1 .0337 0 1.1386 1.1386 0 0 1 1.139 1.138 1.1386 1.1386 0 0 1-1.139 1.139 1.1386 1.1386 0 0 1-1.139-1.139 1.1386 1.1386 0 0 1 1.1053-1.138zm-8.9146.379a.7595.7595 0 0 1 .007 0 .7595.7595 0 0 1 .76.759.7595.7595 0 0 1-.76.76.7595.7595 0 0 1-.759-.76.7595.7595 0 0 1 .752-.759zm4.4706 0a.7595.7595 0 0 1 .0071 0 .7595.7595 0 0 1 .76.759.7595.7595 0 0 1-.76.76.7595.7595 0 0 1-.759-.76.7595.7595 0 0 1 .7519-.759zM4.4157 17.99a.7595.7595 0 0 1 .007 0 .7595.7595 0 0 1 .76.76.7595.7595 0 0 1-.76.759.7595.7595 0 0 1-.7599-.759.7595.7595 0 0 1 .7528-.76zm4.4697.0009a.7595.7595 0 0 1 .007 0 .7595.7595 0 0 1 .76.759.7595.7595 0 0 1-.76.76.7595.7595 0 0 1-.759-.76.7595.7595 0 0 1 .752-.759zm4.4777.0009a.7591.7591 0 0 1 .538.222c.2954.2963.2954.7773 0 1.0732a.7603.7603 0 0 1-1.075 0c-.296-.2959-.296-.777 0-1.0733a.7563.7563 0 0 1 .537-.222Z"
        /> < title > { title } < / title > < / svg >
    }
}
