#[cfg(feature = "VsTerminalDebian")]
use leptos::*;
#[cfg(feature = "VsTerminalDebian")]
///This icon requires the feature `VsTerminalDebian` to be enabled.
#[component]
pub fn TerminalDebian(
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
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M7.084.029a1.276 1.276 0 0 0-.355.05L6.622.065a9.46 9.46 0 0 1 .514-.048c.075-.005.15-.01.224-.017a1.67 1.67 0 0 1-.276.029zm4.127 7.646c.094-.238.172-.436.16-.762l-.133.282c.135-.41.123-.847.112-1.262-.005-.187-.01-.37-.002-.543l-.054-.015c-.048-1.411-1.268-2.911-2.354-3.419-.936-.432-2.376-.506-3.042-.18a.657.657 0 0 1 .212-.085c.107-.031.197-.058.135-.093-.6.06-.778.171-.973.294a1.92 1.92 0 0 1-.635.273c-.11.106.051.063.181.029.129-.035.226-.06-.004.076a1.7 1.7 0 0 1-.303.05c-.26.025-.492.048-.96.532.026.041.11-.009.168-.044.072-.043.106-.063-.054.137C3.07 2.871 1.78 4.31 1.507 4.787l.143.025c-.1.25-.213.461-.313.649-.136.254-.249.464-.273.667a16.97 16.97 0 0 1-.062.635C.907 7.619.79 8.679 1.12 9.06l-.04.406.052.11c.036.079.071.157.12.23l-.093.008c.22.692.338.704.473.717.137.013.291.028.585.757-.084-.028-.17-.06-.293-.226-.015.127.18.508.41.806l-.097.112a.89.89 0 0 0 .27.311c.023.019.045.036.066.055-.372-.203.1.428.371.79.078.104.14.186.159.218l.073-.132c-.01.19.136.433.41.772l.229-.009c.094.186.438.522.647.538l-.139.181c.254.08.321.135.397.195.08.064.17.136.502.253l-.13-.23c.108.095.192.186.273.272.162.176.31.335.62.481.352.123.536.152.74.184.168.026.35.055.649.14a33.82 33.82 0 0 0-.217-.005c-.506-.012-1.056-.025-1.443-.163-3.016-.817-5.776-4.356-5.574-8-.02-.311-.01-.655 0-.961.012-.422.022-.776-.049-.882l.032-.105c.166-.54.365-1.191.742-1.957L.861 3.92v-.002.001c.012.012.106.107.275-.18.04-.09.079-.182.117-.276.08-.19.16-.383.264-.56l.08-.02c.054-.315.533-.744.93-1.1.19-.171.362-.326.46-.443l.02.138C3.541.977 4.414.611 5.074.334c.152-.063.291-.122.414-.176-.107.118.067.082.311.032.15-.03.325-.067.478-.076-.04.023-.082.044-.122.065-.085.045-.17.088-.25.145.26-.062.373-.044.499-.024.109.018.227.036.456.006-.174.025-.384.094-.35.12.245.029.398-.002.537-.03.174-.034.327-.065.61.03L7.625.275c.235.085.409.137.564.183.313.094.55.165 1.067.439a.58.58 0 0 0 .23-.037c.112-.035.218-.069.477.037.014.025.022.046.03.066.03.08.054.143.456.383.056-.022-.097-.162-.22-.274l-.003-.004c1.01.54 2.108 1.692 2.443 2.924-.188-.347-.162-.171-.134.015.018.124.037.253-.006.235.14.377.255.766.325 1.168l-.023-.085c-.102-.368-.3-1.081-.626-1.555-.012.137-.092.122-.165.108-.105-.019-.196-.036-.058.393.081.119.096.074.109.034.015-.047.027-.086.147.164.002.133.034.266.07.414.022.094.046.195.065.306-.034-.006-.07-.07-.106-.13-.045-.076-.087-.147-.117-.101.076.358.201.545.25.572-.009.02-.021.02-.034.021-.027.002-.056.003-.059.167.022.428.102.39.166.361.02-.009.037-.017.051-.01a1.724 1.724 0 0 1-.083.245c-.086.221-.188.48-.106.816a2.356 2.356 0 0 0-.106-.295 5.896 5.896 0 0 1-.046-.117c-.018.151-.01.256-.003.355.013.166.023.312-.094.62.135-.442.12-.841-.007-.649.03.343-.12.642-.254.908-.111.222-.211.42-.184.602l-.161-.222c-.238.344-.22.417-.202.489.015.06.03.12-.105.339.051-.09.041-.112.031-.133-.01-.024-.021-.046.053-.158-.05.003-.17.12-.316.265-.123.121-.265.261-.402.368-1.172.94-2.571 1.062-3.926.556.006-.031-.006-.066-.097-.128-1.148-.88-1.827-1.628-1.591-3.36.068-.051.117-.193.175-.362.09-.263.203-.59.448-.745.245-.541.979-1.04 1.764-1.052.8-.044 1.476.427 1.816.872-.618-.576-1.63-.751-2.493-.324-.882.396-1.405 1.368-1.329 2.336.01-.016.021-.023.03-.03.02-.015.037-.027.048-.108-.027 1.88 2.026 3.258 3.504 2.563l.018.039c.397-.109.497-.205.633-.335.07-.067.148-.142.28-.233a.441.441 0 0 1-.075.085c-.068.067-.143.14-.05.142.166-.043.634-.465.947-.746l.133-.119c.062-.134.051-.177.04-.221-.012-.052-.025-.104.076-.3l.229-.114c.03-.088.062-.168.092-.243zM6.612 10.06a.018.018 0 0 0-.005.016.114.114 0 0 0 .005-.016zm-.005.016c.008.069.269.268.465.369.516.19 1.1.198 1.559.181-.993.415-2.889-.422-3.509-1.532.057.012.168.14.303.297.204.234.462.532.678.605-.213-.17-.377-.387-.53-.61.288.33.637.6 1.019.779a.102.102 0 0 1 .01-.077l.005-.012zM6.752.219a6.612 6.612 0 0 1-.075-.013c.472.014.437.045.283.08.018-.029-.09-.047-.208-.067zM9.63 6.732c.032-.477-.094-.326-.136-.144.019.01.036.059.052.107.028.08.054.158.084.037zm-.211.664a1.68 1.68 0 0 1-.314.703c.006-.061-.038-.074-.083-.086-.092-.026-.183-.052.176-.504a1.113 1.113 0 0 1-.126.242c-.112.184-.21.344.126.133l.033-.06a1.43 1.43 0 0 0 .188-.428zm-1.34 1.247c-.347-.053-.662-.186-.397-.19.221.02.44.02.656-.033a3.544 3.544 0 0 1-.26.223zM6.958.285l-.1.02.094-.008.006-.012zM4.79 8.818l-.038.186c.047.064.092.13.136.195.12.175.237.348.4.483a4.73 4.73 0 0 0-.214-.368c-.08-.13-.169-.272-.285-.496zm.226-.319c.052.108.104.213.185.302l.082.24-.038-.063c-.1-.166-.2-.333-.252-.524l.023.045zm7.474-1.282l-.039.098a4.717 4.717 0 0 1-.462 1.474c.261-.49.43-1.028.501-1.572zM.436 3.426zm.002.022c.008.037.043.028.075.02.06-.015.114-.03-.004.236-.074.052-.119.087-.144.106l-.027.02a.05.05 0 0 1 .008-.017.597.597 0 0 0 .092-.365zM.118 4.76a2.92 2.92 0 0 1-.106.436.588.588 0 0 0-.005-.154c-.013-.105-.025-.197.135-.402a4.009 4.009 0 0 0-.023.12z"
        /> < title > { title } < / title > < / svg >
    }
}
