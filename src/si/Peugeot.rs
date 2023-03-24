#[cfg(feature = "SiPeugeot")]
use leptos::*;
#[cfg(feature = "SiPeugeot")]
///This icon requires the feature `SiPeugeot` to be enabled.
#[component]
pub fn Peugeot(
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
        "M12.0001 0c3.499 0 7.1308.2987 10.8171.9349.055 1.4778.1175 3.762.0126 5.7004-.2349 4.3217-1.1861 7.676-2.9943 10.5564-1.8026 2.872-4.5938 5.3416-7.8354 6.8083-3.2416-1.4667-6.0331-3.9362-7.8356-6.8083-1.808-2.8804-2.7594-6.2348-2.9941-10.5564-.1053-1.9383-.0427-4.2226.0124-5.7004C4.8691.2987 8.5011 0 12.0001 0zm0 .4163c-3.4494 0-6.9514.2933-10.4139.8722-.076 2.1923-.076 3.9367-.0005 5.3243.2305 4.248 1.1619 7.5394 2.9309 10.3575 1.7688 2.8179 4.421 5.1457 7.4835 6.5718 3.0622-1.4261 5.7147-3.7539 7.4835-6.5718 1.769-2.8182 2.7001-6.1095 2.9309-10.3575.0755-1.3876.0755-3.132-.0005-5.3243C18.9515.7096 15.4493.4163 12.0001.4163zM11.97 13.0361s.0681.4045.0888.5218c.0116.0665.0141.0843-.0027.1495-.0477.1695-.3633 1.59-.5385 2.6258-.0822.4767-.1533.9498-.1947 1.3546-.0234.23-.0158.2851.0449.4892.172.5767.8082 2.0834.9304 2.3581a.3288.3288 0 0 1 .0257.0951l.0459.3553c-.2038-.3081-1.3115-2.3458-1.7465-3.4742-.0513-.1329-.0612-.1984-.0044-.4428.3025-1.2962 1.1211-3.4992 1.3511-4.0324zm-1.6153-3.1317c.0312.0913.2909.9805.3107 1.1919.016.1174.0133.149-.0284.2614-.2276.606-.9159 2.1129-1.1814 2.6161-.059.1107-.1145.1828-.2256.2958-.2535.2582-.7386.7462-1.039 1.0192-.1091.0993-.1385.1841-.1607.3126-.0358.2063-.0713.5584-.0812.7402-.0086.1599.0173.2347.1064.3931.5294.9451 2.2966 3.1194 2.8001 3.576.0365.0331.0632.0553.1402.1065.074.0494.3154.2061.3154.2061a.085.085 0 0 1 .0239.0257l.0874.1539c-.2256-.1123-.5874-.2928-.7742-.3929a.7527.7527 0 0 1-.1249-.0845c-.5896-.4902-2.3963-2.6064-2.9277-3.49-.1325-.2206-.1461-.292-.1283-.5008.0333-.3887.0849-.8774.1301-1.0916.0274-.1292.0558-.2051.1547-.3192.7533-.8648 2.2059-2.7743 2.6293-3.5908.0471-.0924.0511-.1245.0521-.2117.0027-.253-.078-.7457-.0992-.8702a.2385.2385 0 0 1-.0025-.0591l.0228-.2877zm4.931-.4748c.0812.0377.2137.0801.2732.1502.9366.9784 1.6856 1.9929 1.8356 2.2603.0289.0514.0486.1028.0558.1616.1683 1.4269.0237 2.9385-.3201 4.2535-.0311.1186-.0659.1794-.1799.2651-.7607.5713-2.5689 2.0716-3.176 2.6297a.996.996 0 0 0-.1718.2088c-.153.2511-.3419.6012-.462.825-.1234-.2696-.3924-.9273-.4847-1.1931-.0301-.0867-.0355-.1322-.0037-.2483.1505-.5512.862-2.1538 1.0005-2.4436-.0018.0372.0063.1347-.0079.1693-.0624.1895-.445 1.4526-.5227 2.0003-.0173.1218-.0158.1547 0 .2743.0341.2557.1868.7247.1868.7247.0174-.0414.1887-.3836.309-.4774.7089-.6984 2.1971-1.9725 3.0499-2.6638.0958-.0776.1315-.1295.1528-.2767.1056-.7254.1362-2.0275.0726-2.9751a1.012 1.012 0 0 0-.1708-.5003c-.231-.3417-.5072-.6651-.926-1.0805a.4152.4152 0 0 1-.1157-.2187c-.1222-.6524-.3949-1.8453-.3949-1.8453zm-7.9201 1.5091a.3306.3306 0 0 1 .1794-.0067c.267.0642.8322.2029 1.2594.3123.06.0153.1347.0714.1666.1245l.093.1549a.1822.1822 0 0 1 .0059.1774 21.7947 21.7947 0 0 1-.3055.5651l-.0886.1583c-.1631.2896-.3254.5691-.4176.7161-.0607.0971-.0945.1401-.2026.1784-.4203.148-.9423.3249-1.2525.4255-.0634.0205-.0849.0591-.0726.1295.0222.1282.0938.5636.1212.6906.0101.0469.0476.0843.1172.0709.1834-.0348.6715-.1544.6715-.1544s-.1439.1332-.23.2117a.1456.1456 0 0 1-.0545.0324c-.133.0445-.4578.1356-.559.1579-.079.017-.1293-.0141-.1496-.1035 0 0-.1385-.59-.1885-.7907a1.552 1.552 0 0 0-.0254-.0872c-.0375-.1161-.1276-.3444-.1784-.464a.108.108 0 0 1-.0087-.044c.0015-.1366.0267-.4415.0267-.4415s.2989.2804.4435.3817c.0326.023.0521.0277.0921.0175.1987-.0511.6555-.2024.9062-.2962a.3475.3475 0 0 0 .1463-.1048c.2594-.3135.5846-.8013.7725-1.1692a.2194.2194 0 0 0 .0146-.1631l-.0405-.1334c-.0118-.039-.0531-.083-.0911-.0976a34.0249 34.0249 0 0 0-1.1999-.4329l.0489-.0155zm8.7416-3.7975a.407.407 0 0 1 .2994.0069c.7409.3113 1.9867 1.0437 2.6792 1.5596.0669.0499.0935.0843.1111.1648.1626.7402.2243 1.8845.1219 2.8478-.0074.0692-.0212.1107-.0573.1846-.4388.8981-.9326 1.7434-1.4524 2.3954.0078-.0807.0446-.4617.0471-.6834a.0765.0765 0 0 1 .0304-.0605 9.0576 9.0576 0 0 0 .304-.2468c.0365-.0316.0568-.0553.0758-.086.2117-.3425.5903-1.0595.7428-1.4909a.787.787 0 0 0 .0429-.3281c-.0489-.6011-.1431-1.434-.2836-1.9776a.3337.3337 0 0 0-.1703-.2147c-.1212-.0623-.3588-.1707-.9-.3879a.7631.7631 0 0 1-.2068-.1238c-.3549-.2985-.9097-.7459-1.2549-1.0034a.4301.4301 0 0 0-.3527-.0749 21.4993 21.4993 0 0 0-.7517.1856c-.0612.0166-.0854.018-.1486.0106-.1009-.0117-.3007-.0275-.382-.0336.5871-.2761 1.152-.5142 1.5057-.6437zM8.274 12.0001c-.1836.3024-.403.6236-.5689.8243a.1988.1988 0 0 1-.0834.0588 14.1489 14.1489 0 0 1-.7063.2382l1.3586-1.1213zm-2.4267-.397l-.1083.7484-.3685-.4329c.0671-.1455.2633-.2829.4768-.3155zm-.6866-1.6986l-.133.0838c-.0333.0212-.0424.0304-.0563.0595a2.6968 2.6968 0 0 1-.0856.1616c-.0091.0158-.0336.0403-.0466.0504a16.6112 16.6112 0 0 1-.4773.3533c-.0089.0062-.0202.0035-.0279-.004-.0165-.0166-.0859-.0954-.0982-.1139a.0823.0823 0 0 1-.0151-.0489 2.0329 2.0329 0 0 1 .0062-.1337c.0039-.0509.0185-.0751.0805-.1394.0693-.0719.1449-.1475.2243-.2251.1947-.1641.6863-.5633 1.5642-1.1904a.1927.1927 0 0 0 .0437-.0423c.0921-.1244.354-.4644.4186-.5452a.1928.1928 0 0 1 .0308-.0311 2.4849 2.4849 0 0 1 .2595-.1833c.2445-.1542.7181-.4354.9834-.5898l-.1007.124c-.0056.0055-.0111.0106-.0167.0163a42.2875 42.2875 0 0 0-.5625.4314c-.0286.0227-.04.0425-.0466.0783-.0227.1266-.0605.367-.0716.5001-.0032.039-.02.0583-.0597.0744-.173.0704-.3781.1445-.5439.211-.0356.0141-.0454.0195-.0664.0356-.0166.0128-.2369.2098-.2369.2098s.479-.1181.6547-.1579c.042-.0094.06-.0069.0987.0114.0363.0168.0967.0408.1286.0524.0489.018.0748.0172.1288.0054.1979-.0442.5308-.1233.708-.1695.0921-.024.1424-.0489.2204-.1035.0849-.0591.2999-.2205.2999-.2205s-.004.0285-.0069.0443a.0591.0591 0 0 1-.0161.0318 3.986 3.986 0 0 1-.1581.1777c-.0871.0902-.1392.1223-.2485.168-.5035.21-1.3638.5362-2.0091.7598-.0423.0148-.0606.0269-.0919.059-.0341.0346-.0795.085-.0795.085s.5262.0383.6868.0568c.0521.0059.0893.023.1291.0573.132.1137.5691.5641.6999.7205 0 0-1.2426.2068-1.8184.3499-.0711.0175-.1066.0628-.1227.1198-.0476.1675-.1599.7062-.1599.7062s-.04-.0128-.0958-.045c-.0383-.022-.0577-.0403-.0997-.0847-.1362-.144-.3601-.4198-.4677-.5796-.0316-.0472-.0449-.1072-.0054-.1554.1167-.1438.3512-.4062.4805-.5411.0138-.0146.0313-.04.0499-.0766.0498-.1032.1038-.3175.1282-.4139zm8.0217-2.8166a.1805.1805 0 0 1 .1083.0148l.2564.1228a.0935.0935 0 0 1 .0503.0628c.0348.1468.0775.3412.1061.5065a.2993.2993 0 0 1-.036.1994c-.1061.1846-.324.5166-.446.695-.0271.04-.0304.0885-.022.1361.0553.3059.2858 1.374.2858 1.374l-.1451-.1322a.2102.2102 0 0 1-.0602-.0917c-.1032-.3022-.2962-.9335-.385-1.2406a.1588.1588 0 0 1 .0299-.145c.1513-.1838.3717-.4778.5005-.6641.036-.0524.0479-.1102.0267-.1534a4.6142 4.6142 0 0 0-.1273-.2369.1058.1058 0 0 0-.0597-.0489l-.174-.0558a.1452.1452 0 0 0-.093.0017c-.3791.1376-.8889.336-1.2009.4655-.0457.019-.0644.0205-.1197.0121-.0903-.0133-.3534-.0605-.4825-.0847a.0344.0344 0 0 1-.023-.0511l.153-.2654a.1474.1474 0 0 1 .0955-.0704c.4922-.1123 1.143-.2413 1.7619-.3505zM7.7369 8.3065a.0544.0544 0 0 1 .0548.0326c.0309.072.0225.1523-.0074.2209-.0116.028-.0474.0551-.0767.0616l-.2456.0546a.0227.0227 0 0 1-.0274-.0237l.023-.2886a.04.04 0 0 1 .0365-.0366l.2428-.0208zm2.7941-1.8543a.0366.0366 0 0 1 .0321.0549l-.5987 1.032a.2643.2643 0 0 1-.1552.126l-1.1227.3516a1.6503 1.6503 0 0 1-.3761.0712c-.249.0188-.9886.042-.9886.042l.0162-.0351a.0741.0741 0 0 1 .038-.0368l.35-.1487a.169.169 0 0 0 .0666-.0502l.5405-.6661c.0289-.0356.0666-.0697.1069-.0909.357-.1881.992-.4705 1.4368-.6192a.5085.5085 0 0 1 .1426-.0235l.5116-.0072zm-.6002-1.1984c1.2389-.109 2.9153-.1386 4.5168-.0173a.837.837 0 0 1 .2638.064c.5247.2209 1.4556.7042 2.4906 1.2969a.7764.7764 0 0 1 .1281.0917c.2742.2419.9689.9535.9689.9535l-.2167-.043a.2282.2282 0 0 1-.0844-.0351c-.4383-.2948-1.0782-.6785-1.6145-.949a.5915.5915 0 0 0-.2034-.0598 14.6895 14.6895 0 0 0-.8443-.0618c-.1138-.0047-.1913-.0237-.3026-.0803-.2127-.108-.728-.342-.9847-.44-.1098-.042-.1658-.0519-.2734-.0502-.441.0064-1.4457.0385-1.9588.0704a.2445.2445 0 0 1-.0521-.0025l-.2445-.0383c.6935-.1196 2.4077-.2725 3.1172-.3098l-.2887-.0966a.596.596 0 0 0-.1728-.0299c-.9698-.0218-2.3036-.0178-3.4321.067a.6098.6098 0 0 0-.2021.0504c-.3228.1419-1.11.5692-1.8994 1.0081-.2546.1416-.529.2952-.7731.4326 0 0 .0985-.1268.1175-.1515.0168-.0215.0219-.0255.0447-.0403.1018-.0684.4109-.2678.5343-.3585.2377-.1794.6128-.4956.8364-.6916a.6677.6677 0 0 0 .1288-.1529c.0723-.1171.1513-.2441.1999-.3145.0414-.0611.1123-.1038.2006-.1117zM4.655 2.404c.306-.0316.477.0875.477.3975v.1715c0 .3172-.171.4455-.477.4774a71.2552 71.2552 0 0 0-1.269.144v.4687c-.0886.0109-.1774.0218-.266.0331V2.5814a70.6625 70.6625 0 0 1 1.535-.1774zm14.2032-.0491a71.245 71.245 0 0 1 2.022.2263v.2444a68.7666 68.7666 0 0 0-.8741-.105v1.2707a94.0383 94.0383 0 0 0-.2663-.0299V2.6906a68.3322 68.3322 0 0 0-.8815-.0914v-.2443zM7.6494 2.1558v.2441a71.8527 71.8527 0 0 0-1.6641.1223v.3731a68.5725 68.5725 0 0 1 1.48-.1107v.2444a70.5409 70.5409 0 0 0-1.48.1104v.4094a69.7002 69.7002 0 0 1 1.6641-.1223v.2441a70.9377 70.9377 0 0 0-1.9245.145v-1.515a70.5346 70.5346 0 0 1 1.9245-.1448zm8.6326.4149c0-.3397.1683-.4097.5476-.384.3332.023.6658.0479.9988.0756.3295.0272.5469.1198.5469.4665v.6765c0 .3402-.1878.4015-.5469.3719a72.5525 72.5525 0 0 0-.9988-.0754c-.3628-.0245-.5476-.0946-.5476-.4546v-.6765zm-2.5812-.5284c.6426.0153 1.285.0395 1.9274.0726v.2441a70.7366 70.7366 0 0 0-1.6666-.0657v.3728c.4943.0139.9884.0327 1.4823.0567v.2444a70.4407 70.4407 0 0 0-1.4822-.0566v.4092a69.1564 69.1564 0 0 1 1.6666.066v.2441a71.29 71.29 0 0 0-1.9274-.0726v-1.515zm-3.344-.0013v1.0958c0 .3467-.1819.4228-.5484.4341a68.0289 68.0289 0 0 0-.9573.0361c-.3401.0153-.5479-.0494-.5479-.3926V2.1186l.2606-.0133v1.142c0 .0694.0595.126.1385.1223a70.1195 70.1195 0 0 1 1.2547-.0475c.0795-.0022.1323-.063.1323-.1324v-1.142c.089-.0025.1781-.0047.2675-.0067zm6.3245.3802c-.0792-.0054-.1387.0502-.1387.1295v.7491c0 .0694.0595.1423.1387.1478.4321.0287.8638.0613 1.2954.0978.0691.0059.1382-.0576.1382-.1268v-.7493c0-.0791-.0691-.1445-.1382-.1505a70.1864 70.1864 0 0 0-1.2954-.0976zm-5.1721-.3976c.479-.0035.958-.0017 1.4368.0047v.2441a72.1473 72.1473 0 0 0-1.536-.004c-.0693.0007-.1387.0608-.1387.1302v.759c0 .0694.0693.1381.1387.1374.4593-.004.9183-.0032 1.3776.002v-.4094a66.592 66.592 0 0 0-.7994-.0042v-.2443c.3534 0 .7068.0027 1.0602.0079v.8976a71.1228 71.1228 0 0 0-1.4896-.0064c-.3635.0025-.5484-.0603-.5484-.4136v-.6602c0-.3299.1651-.4383.4988-.4408zm-6.7546.6162l-.0206.0004a70.2289 70.2289 0 0 0-1.348.1522V3.35a71.0344 71.0344 0 0 1 1.348-.152c.0689-.0074.138-.0704.138-.1364v-.3068c0-.0694-.0691-.1218-.138-.1146l.0206-.0004z"
        /> < title > { title } < / title > < / svg >
    }
}
