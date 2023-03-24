#[cfg(feature = "SiApachekylin")]
use leptos::*;
#[cfg(feature = "SiApachekylin")]
///This icon requires the feature `SiApachekylin` to be enabled.
#[component]
pub fn Apachekylin(
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
        "M8.033 16.459l-.59 1.29h.142l.155-.344h.72l.154.345h.144l-.59-1.291h-.135zm-.243.835l.31-.694.31.694h-.62zm1.154-.515v1.328h.131v-.54a.424.424 0 0 0 .369.192c.277 0 .483-.2.483-.494a.47.47 0 0 0-.483-.493.42.42 0 0 0-.374.2v-.192h-.126zm.49.107c.205 0 .362.153.362.378 0 .227-.157.379-.361.379-.207 0-.362-.152-.362-.379 0-.224.155-.378.362-.378zm1.487.864v-.602c0-.25-.142-.376-.392-.376a.616.616 0 0 0-.395.13l.059.099a.494.494 0 0 1 .323-.115c.18 0 .274.09.274.257v.059h-.308c-.278 0-.374.125-.374.275 0 .167.135.282.354.282.16 0 .275-.061.334-.16v.15h.125zm-.13-.292c-.052.128-.161.198-.309.198-.154 0-.245-.07-.245-.183 0-.1.06-.173.25-.173h.303v.158zm.525-.193c0 .228.155.378.367.378a.328.328 0 0 0 .282-.142l.098.066c-.077.125-.216.192-.38.192-.29 0-.5-.205-.5-.494 0-.29.21-.493.5-.493.164 0 .303.065.38.192l-.098.066a.33.33 0 0 0-.282-.144c-.212 0-.367.152-.367.379zm1.12-.884v.568c.072-.112.199-.177.363-.177.236 0 .398.137.398.415v.563h-.13v-.55c0-.205-.108-.31-.292-.31-.208 0-.34.13-.34.35v.51h-.13v-1.37h.13zm1.968.884c0-.292-.198-.493-.47-.493-.274 0-.475.205-.475.493 0 .287.207.494.51.494a.477.477 0 0 0 .374-.159l-.074-.085a.382.382 0 0 1-.297.128c-.212 0-.367-.135-.382-.338h.812l.002-.04zm-.814-.058a.337.337 0 0 1 .343-.322c.19 0 .329.132.345.322h-.688zm1.728-.748v.78l.758-.78h.157l-.559.584.598.707h-.162l-.528-.607-.264.267v.34h-.136v-1.291h.136zm1.292 1.383l.042-.094-.433-.969h.136l.365.825.365-.825h.13l-.486 1.087c-.08.19-.184.25-.322.25a.332.332 0 0 1-.236-.088l.06-.098a.243.243 0 0 0 .178.074c.086 0 .145-.04.2-.162zm.92-1.461v1.369h-.131v-1.37h.13zm.365.398h.13v.97h-.13v-.97zm.066-.212a.095.095 0 0 1-.095-.094c0-.05.042-.092.095-.092a.093.093 0 1 1 0 .186zm.556.212v.18c.07-.119.2-.187.369-.187.236 0 .398.137.398.415v.562h-.131v-.55c0-.204-.107-.309-.291-.309-.209 0-.34.13-.34.35v.51h-.13v-.97h.125zM2.086 15.41C.264 15.4.124 15.395.063 15.356c-.084-.055-.084-.137-.002-.302.223-.448.78-.76 1.676-.943.575-.116.917-.252 1.23-.487.374-.281.418-.538.12-.69a2.52 2.52 0 0 1-.868-.786c-.294-.447-.492-1.162-.492-1.778 0-.321.168-.938.36-1.323.282-.565.68-.963 1.236-1.235a3.043 3.043 0 0 1 1.25-.298c.085 0 .142-.014.178-.044.05-.04.052-.051.026-.13-.04-.119-.206-.266-.373-.328-.156-.058-.332-.085-.807-.122-.18-.013-.38-.04-.444-.06a.788.788 0 0 1-.411-.28c-.057-.085-.067-.126-.067-.259 0-.14.011-.179.098-.338.092-.168.13-.267.2-.522.023-.086.037-.102.09-.102.033 0 .1.031.148.07.292.23 1.183.53 1.571.53.246 0 .361-.035.453-.14.064-.073.071-.095.058-.181a1.036 1.036 0 0 0-.268-.502c-.14-.14-.426-.285-.694-.355a1.921 1.921 0 0 0-.504-.054c-.345-.004-.373.004-.563.163-.158.132-.21.12-.386-.089-.441-.522-.646-1.065-.518-1.37.041-.1.095-.121.467-.184.163-.027.498-.091.744-.143.965-.202 1.498-.268 2.164-.267.56.001.585-.006.736-.231.199-.297.271-.585.272-1.088 0-.406.018-.455.162-.455.113 0 .276.08.357.176.16.189.221.439.249 1.012.017.351.035.509.08.686.094.364.146.445.543.837.493.488.64.722.731 1.16.073.352.032.768-.107 1.085-.171.393-.579.845-1.033 1.147-.351.234-.757.427-2.302 1.097-.297.128-.61.27-.696.312-.559.282-1.082.816-1.283 1.31-.058.14-.07.21-.08.443-.016.43.076.72.34 1.07.181.242.53.508.791.606.323.121.776.127 1.154.016.37-.108.797-.361.747-.442-.026-.042-.08-.031-.245.048-.627.299-1.316.344-1.71.112a2.043 2.043 0 0 1-.55-.551c-.228-.386-.3-.932-.173-1.303a1.14 1.14 0 0 1 .313-.468c.24-.233.993-.618 2.46-1.258.868-.38 1.04-.464 1.342-.666a3.719 3.719 0 0 0 1.67-3.135c0-.148.01-.27.021-.27.028 0 .19.216.313.416.627 1.026.843 2.398.54 3.443-.284.978-.948 1.741-1.77 2.034a2.307 2.307 0 0 1-.88.14 2.57 2.57 0 0 1-.852-.13c-.467-.135-.814-.148-1.19-.044-.346.096-.52.235-.588.47a.682.682 0 0 0 .08.518c.086.16.165.215.323.228.234.019.547-.145.547-.287 0-.031-.07-.178-.156-.326-.086-.148-.156-.295-.156-.328 0-.089.084-.133.251-.133.22 0 .36.076.528.284.158.196.301.3.47.34.206.05.235.158.13.476-.175.522-.62 1.133-1.429 1.96a8.642 8.642 0 0 0-.596.66c-.224.319-.44.787-.565 1.224-.087.306-.164.384-.41.412-.039.005-.952.003-2.028-.004zM9.741 2.668a10.68 10.68 0 0 1 5.137-.638c2.634.336 4.891 1.67 6.295 3.72.499.728.699 1.143.7 1.451 0 .346-.242.533-.618.477a.98.98 0 0 1-.508-.271 24.729 24.729 0 0 1-.672-.781c-1.308-1.565-1.733-2.01-2.404-2.516a8.626 8.626 0 0 0-1.856-1.044c-1.82-.713-3.944-.805-6.106-.266-.494.123-.48.067.032-.132zM23.807 12.89c-.359.932-1.224 1.631-2.401 1.942-.939.247-1.7.244-2.46-.01a4.319 4.319 0 0 1-1.42-.84c-.22-.205-.294-.23-.294-.097 0 .067.034.124.18.306.231.285.336.488.316.613-.02.123-.104.19-.261.21-.185.022-2.544.004-2.593-.02-.104-.051.012-.31.203-.45.143-.104.275-.158.492-.199.184-.035.247-.075.247-.157 0-.02-.054-.146-.12-.278a2.606 2.606 0 0 1-.23-.723c-.024-.13-.177-.073-.319.117-.247.331-.419.856-.53 1.623-.056.377-.085.46-.18.502-.048.022-.56.028-2.072.026a104.858 104.858 0 0 1-2.07-.014c-.213-.04-.178-.254.08-.48.21-.184.703-.417 1.303-.616.406-.135.454-.173.452-.36-.002-.29-.235-.573-.873-1.06-.671-.511-.864-1.516-.43-2.235.261-.431.73-.782 1.2-.897.803-.196 1.43.033 1.722.628.11.225.182.477.16.563-.022.089-.093.071-.272-.066-.258-.198-.434-.27-.682-.28-.255-.01-.416.036-.552.16-.158.141-.198.244-.199.508 0 .208.007.242.077.392.377.796 1.599.95 2.453.308.278-.21.623-.788.724-1.216.043-.181.05-.267.04-.528-.009-.267-.02-.342-.082-.519-.155-.445-.343-.685-.672-.855-.503-.26-1.133-.33-1.989-.22-.728.092-1.493.059-1.95-.086-.2-.063-.27-.114-.27-.194 0-.067.023-.074.3-.094.428-.031.896-.147 1.651-.408.248-.086.556-.183.684-.215.783-.198 1.609-.252 2.218-.146.26.045.572.2.811.402.695.588 1.368 1.78 1.913 3.39.168.497.323.839.506 1.117.41.627.958 1.01 1.65 1.155.08.017.28.03.444.029a2.19 2.19 0 0 0 1.002-.235c.608-.289.977-.74 1.163-1.423.068-.25.075-.85.013-1.086-.144-.54-.492-.896-1.038-1.059-.239-.071-.773-.091-.942-.035a.877.877 0 0 0-.537.514c-.044.11-.054.186-.055.4-.002.237.005.278.062.4.174.371.505.547 1.031.548.26 0 .3.02.274.14-.03.136-.178.225-.437.265a1.667 1.667 0 0 1-.986-.13c-.308-.143-.595-.477-.72-.835a1.452 1.452 0 0 1-.021-.76 2.4 2.4 0 0 1 .368-.732c.399-.492 1.06-.746 1.775-.68 1.002.092 1.779.639 2.121 1.494.278.693.288 1.796.022 2.486zm-12.888.57c-.265.18-1.02.268-1.913.22-.547-.028-.6-.019-.639.113-.041.145.034.318.234.535.27.295.291.447.077.562-.098.052-.107.052-1.539.058-1.2.004-1.453 0-1.517-.029-.139-.06-.129-.264.021-.434.133-.15.178-.163.652-.178.387-.012.431-.017.462-.06a.189.189 0 0 0 .033-.095c0-.081-.102-.304-.212-.466-.175-.258-.205-.345-.206-.608-.001-.257.024-.317.175-.42.253-.172.937-.1 1.626.173.67.265 1.328.371 2.309.371.509 0 .585.005.597.038.019.047-.055.15-.16.22zm10.194 5.002c.047.04.038.178-.02.29-.123.242-.518.705-1.008 1.183-.842.821-1.599 1.354-2.673 1.884-1.593.786-3.18 1.16-4.87 1.148a19.02 19.02 0 0 1-.675-.012 10.35 10.35 0 0 1-2.395-.479c-2.186-.718-4.043-2.16-5.4-4.19-.375-.563-.517-.895-.538-1.256-.015-.254.011-.37.12-.531a.539.539 0 0 1 .242-.2c.156-.08.178-.084.396-.083.503.004.899.247 1.353.83.086.11.326.449.535.752.43.628.756 1.068 1.046 1.416.255.306.72.77.976.975.419.334 1.405.863 2.117 1.134 2.32.883 4.7.8 7.069-.247 1.22-.54 2.408-1.364 3.27-2.27.295-.31.4-.39.455-.344z"
        /> < title > { title } < / title > < / svg >
    }
}
