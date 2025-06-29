#![allow(unreachable_patterns)]
#[inline]
pub(crate) fn correlations_dates(minor: u16, patch: u16) -> Result<i64, &'static str> {
    match minor {
        9 => Ok(1464280168),
        81 => Ok(1725552388),
        80 if patch == 1 => Ok(1723126065),
        80 => Ok(1721908957),
        8 => Ok(1460653474),
        79 => Ok(1718287478),
        78 => Ok(1714653548),
        77 if patch == 2 => Ok(1712698776),
        77 if patch == 1 => Ok(1711628578),
        77 => Ok(1711025824),
        76 => Ok(1707401220),
        75 => Ok(1703780510),
        74 if patch == 1 => Ok(1701959035),
        74 => Ok(1700142571),
        73 => Ok(1696522231),
        72 if patch == 1 => Ok(1695132379),
        72 => Ok(1692884807),
        71 if patch == 1 => Ok(1691086303),
        71 => Ok(1689257048),
        70 => Ok(1685645566),
        7 => Ok(1457031860),
        69 => Ok(1682001579),
        68 if patch == 2 => Ok(1680008162),
        68 if patch == 1 => Ok(1679581283),
        68 => Ok(1678373120),
        67 if patch == 1 => Ok(1675956747),
        67 => Ok(1674746027),
        66 if patch == 1 => Ok(1673394800),
        66 => Ok(1671120663),
        65 => Ok(1667484517),
        64 => Ok(1663853457),
        63 => Ok(1660229201),
        62 if patch == 1 => Ok(1658237720),
        62 => Ok(1656603397),
        61 => Ok(1652967346),
        60 => Ok(1649336651),
        6 => Ok(1453412913),
        59 => Ok(1645718740),
        58 if patch == 1 => Ok(1642711960),
        58 => Ok(1642090431),
        57 => Ok(1638453846),
        56 if patch == 1 => Ok(1635764606),
        56 => Ok(1634825356),
        55 => Ok(1631195195),
        54 => Ok(1627565195),
        53 => Ok(1623939692),
        52 if patch == 1 => Ok(1620655243),
        52 => Ok(1620313522),
        51 => Ok(1616679019),
        50 => Ok(1613052628),
        5 => Ok(1449760900),
        49 => Ok(1609424231),
        48 => Ok(1605795177),
        47 => Ok(1602160872),
        46 => Ok(1598541310),
        45 if patch == 2 => Ok(1596464307),
        45 if patch == 1 => Ok(1596130229),
        45 => Ok(1594904894),
        44 if patch == 1 => Ok(1592485824),
        44 => Ok(1591289990),
        43 if patch == 1 => Ok(1588861775),
        43 => Ok(1587650851),
        42 => Ok(1584021924),
        41 if patch == 1 => Ok(1582812747),
        41 => Ok(1580400735),
        40 => Ok(1576767764),
        4 => Ok(1446141743),
        39 => Ok(1573132268),
        38 => Ok(1569504205),
        37 => Ok(1565875971),
        36 => Ok(1562238698),
        35 => Ok(1558640997),
        34 if patch == 2 => Ok(1557839781),
        34 if patch == 1 => Ok(1556209306),
        34 => Ok(1555002328),
        33 => Ok(1551377538),
        32 => Ok(1547751362),
        31 if patch == 1 => Ok(1545326056),
        31 => Ok(1544113017),
        30 if patch == 1 => Ok(1541698381),
        30 => Ok(1540482286),
        3 => Ok(1442511414),
        29 if patch == 2 => Ok(1539359694),
        29 if patch == 1 => Ok(1537894264),
        29 => Ok(1536853732),
        28 => Ok(1533222728),
        27 if patch == 2 => Ok(1532101166),
        27 if patch == 1 => Ok(1531232150),
        27 => Ok(1529611688),
        26 if patch == 2 => Ok(1528216213),
        26 if patch == 1 => Ok(1527609068),
        26 => Ok(1525970602),
        25 => Ok(1522305786),
        24 if patch == 1 => Ok(1519947517),
        24 => Ok(1518718666),
        23 => Ok(1515080803),
        22 if patch == 1 => Ok(1511401652),
        22 => Ok(1511380094),
        21 => Ok(1507819205),
        20 => Ok(1504198621),
        2 => Ok(1438966327),
        19 => Ok(1500564126),
        18 => Ok(1496937783),
        17 => Ok(1493316190),
        16 => Ok(1489684542),
        15 if patch == 1 => Ok(1486671910),
        15 => Ok(1486061282),
        14 => Ok(1482433817),
        13 => Ok(1478808214),
        12 if patch == 1 => Ok(1477001625),
        12 => Ok(1475173179),
        11 => Ok(1471541812),
        10 => Ok(1467911358),
        1 => Ok(1435253745),
        12 => Ok(1412872439),
        11 => Ok(1404324205),
        88 => Ok(1750961413),
        87 => Ok(1747330093),
        86 => Ok(1743673352),
        85 if patch == 1 => Ok(1742319689),
        85 => Ok(1740071365),
        84 if patch == 1 => Ok(1738262759),
        84 => Ok(1736437309),
        83 => Ok(1732802513),
        82 => Ok(1729183468),
        _ => Err("Version not found"),
    }
}

#[inline]
pub(crate) fn correlations_commits(
    major: u16,
    minor: u16,
    patch: u16,
) -> Result<&'static str, &'static str> {
    match minor {
        11 if major == 0 => Ok("e1247cb1d0d681be034adb4b558b5a0c0d5720f9"),
        12 if major == 0 => Ok("f0c419429ef30723ceaf6b42f9b5a2aeb5d2e2d1"),
        9 => Ok("6f9526945c2b832c0eb2187964ecd68f6ab0f600"),
        81 => Ok("0071a0c49a7e6958ea6c493098d2c6a6040447b3"),
        80 if patch == 1 => Ok("747731434e39818e8c959b14a3e5df99ac1ff52e"),
        80 => Ok("4f9c8cbf2386b5e35c5ba754b705c383c5f4b4cc"),
        8 => Ok("6bef00072dbaa86da9dc73b09f926cf67c696b39"),
        79 => Ok("a6af86c649426f1d6a0ec4b188f1f5ff097c5bb0"),
        78 => Ok("631f1ea3617dcd61dbc5c1aa59e71e54fd47445c"),
        77 if patch == 2 => Ok("8b02fd2ae7f1b730f5df5210fc93a9bbb803a1af"),
        77 if patch == 1 => Ok("42b49de7b7c3bfd4fc990691342af520ce142013"),
        77 => Ok("328759fef321406d7db674313585ca48452f311a"),
        76 => Ok("1fc800020eb2f5ed3a66fcc04bab123a0ebf4295"),
        75 => Ok("2010168acb2062cf7705ccbf82e7fa84628a38f8"),
        74 if patch == 1 => Ok("7b77c6e026e6b0d7f3ea21dcaccd0a2c3d249aca"),
        74 => Ok("b092d43294e5543ab22d49b9bfbfe60f0a3bd826"),
        73 => Ok("6d43ffa8822bb652f716e757ca87eb7b0165cd04"),
        72 if patch == 1 => Ok("6d4fbaaf678ec5718798476da04102e51796b45a"),
        72 => Ok("c3813f7d10107fe96c41a48201c1ed980d9c5343"),
        71 if patch == 1 => Ok("01839690de549c01de4ea4c8598892d110fbcbd8"),
        71 => Ok("b793e03a8e63a096b14cda522a550a0efb6e287d"),
        70 => Ok("0fe1acccb984f2008de6dc9dda167f388afdff70"),
        7 => Ok("2d2a9311cbe05eb47960489259e1e9408062c863"),
        69 => Ok("f0da3c89a5521a4d8c8e2fb89f863fd774ebb4d8"),
        68 if patch == 2 => Ok("b87584a0146f6fe23d68e4473f4ce20904c6ba27"),
        68 if patch == 1 => Ok("0204fd462a5fb7d6e4d84bc5b8f3072dd4029d43"),
        68 => Ok("473c2ac376d281c1e57254f93bfcd6d686b00f16"),
        67 if patch == 1 => Ok("21a2bdd0c3252e9174cd0f659c52031bdcc11988"),
        67 => Ok("ffb22f48ba6fd4f2cf5b921c08b8e618aded1465"),
        66 if patch == 1 => Ok("c1512ff41a9e5c5fdc8c3b01efc376b1003ad404"),
        66 => Ok("f8ae1179a6deab019f4fc7aa79efbb7f15096a4f"),
        65 => Ok("4abddbedbc8b5de785e7b8d31aaa31dde6bd430f"),
        64 => Ok("0ccf0f10802b45300c88ce37ea53edab23510a1c"),
        63 => Ok("56438456e0322f813d21feb3af593d2351ec7c2e"),
        62 if patch == 1 => Ok("110f538e89b02301f5a9195c4b7b219cbff214b8"),
        62 => Ok("cb333ec50ce4cc10c0a4b9bd86f86d9e09cbb1d3"),
        61 => Ok("a52dc32db4ecf9b0765467a1403ce59289fe2c3a"),
        60 => Ok("6d94daf692c1347229357379e8b5d820247b9e65"),
        6 => Ok("4fe6048e8dd100ec1c61a020d01f75046b42e818"),
        59 => Ok("4c9bd1611af515ec325e4322ebb22f598b8569e7"),
        58 if patch == 1 => Ok("8832d21e92d9a502c4ae591b9d529095dcf46c05"),
        58 => Ok("61ea9a44c133f403599e76a336331fcb675399cd"),
        57 => Ok("20e5e087c3de0fb9c211aa80c0d505a6b2c099be"),
        56 if patch == 1 => Ok("7eb244d7dd25d6137469e8abc71b2fe5b0ccf527"),
        56 => Ok("bd180709d8dbe163333521b01320d97e4da3d182"),
        55 => Ok("06aec595a7ef346bd884d12b4853e9a432733e42"),
        54 => Ok("0170fff6e0b50bcab2f8bf3321e6b5652226f20c"),
        53 => Ok("6c84a92fdd46ff5a8dcf903622caf360f4425d93"),
        52 if patch == 1 => Ok("390d3e847cb6031fb6d5063ed032ea406cde625e"),
        52 => Ok("af65d486380d32ae42853e8dc3db0147869ba522"),
        51 => Ok("2949b7456a958923d34eec389490adc70efd85a3"),
        50 => Ok("11199001337213fbfe03cf82e8987846f1d70eaf"),
        5 => Ok("d8027afc009e55fb07776831094a13b1b64b6305"),
        49 => Ok("a31c977c800ff16bff77a8466983fde7469abb8f"),
        48 => Ok("82457e09d726f92f384e375c5a7086e49939a3e5"),
        47 => Ok("af10768031c76eb3181543bd1cfef8ba8be7e443"),
        46 => Ok("0fbbbf27e72e06134ab2317de07f69635613e06d"),
        45 if patch == 2 => Ok("fa5abfa682ef609bbce4ab16adb1f94d3869e54e"),
        45 if patch == 1 => Ok("e147b1a0aefe9dcb1669a6e3e19d2ebfa5820455"),
        45 => Ok("b13c29dde6865973be1146727cb41b382e1fc905"),
        44 if patch == 1 => Ok("b16f6d814a575dec93eefc506c54487a9c6eeb8f"),
        44 => Ok("9dcb33aa56d9783788a1321c6068e2808e1b8199"),
        43 if patch == 1 => Ok("844c53d6d3fd180615cbddd9d882689d0ee6cde2"),
        43 => Ok("2df93b48cdfa2a6e1de9edc263f879874bdf47e9"),
        42 => Ok("874f7cbe352033cac5a8bc889847da2fe1d13e9f"),
        41 if patch == 1 => Ok("e8b466f79c5a5cf5870b284ff433707ec7a36af3"),
        41 => Ok("a2bd8df40d3a882e40b6e03b142517172a517024"),
        40 => Ok("d4627cdbd2a26c536dcf015c024e24b798471018"),
        4 => Ok("e10441ae5e655113cd024da8760b3d417ff0e4f7"),
        39 => Ok("d650e13c2975bde610b17d52db7710f38906f05c"),
        38 => Ok("cabda533c937a763582a2c505b38bdecb08f80a5"),
        37 => Ok("d0be6f624ccbeb1a9fb2796c0a6d99f8bdc9c781"),
        36 => Ok("dd20f18bef1af465a09b08d92d289ab880011eb0"),
        35 => Ok("ac2452c1b746a4bc7ffd68120dce9564d93deb35"),
        34 if patch == 2 => Ok("a6a44c5d923aadedf4e3507cc001bdfdb2abf544"),
        34 if patch == 1 => Ok("433755e0043306b621f092dad44d2aed8cd28285"),
        34 => Ok("e1a98a0cb13cef05a3d478f75911ae3d8035b278"),
        33 => Ok("fc3b53ff103062247686e14f83f44021d5558640"),
        32 => Ok("3854d05abcd904e18311ca793f60d0b71e800a44"),
        31 if patch == 1 => Ok("4bc6ce83704da6fd5d56856c54393ba0df03ad95"),
        31 => Ok("3b4b9a4247febf0e71f3480fd34775850b77fea4"),
        30 if patch == 1 => Ok("9df3c8af03e79d31289f43fc8d2dda2e544af348"),
        30 => Ok("a06e4e4e9f995fd0bff09c735bf932aaea634adb"),
        3 => Ok("1896219fb7e33b0d317ec0be4a2e21d86af2ba8e"),
        29 if patch == 2 => Ok("4da54a1fa57e588832f230a15b9d5db01a53b996"),
        29 if patch == 1 => Ok("ca96ed349acfaf2a756ccc3d0edfc2e457eb4376"),
        29 => Ok("de9114b0633ed9f3160f9c2a98e239383eb105f1"),
        28 => Ok("0bbeff8f750dd68e687403e1eb663373f8ac3154"),
        27 if patch == 2 => Ok("dac474b10ffa573685716899661dbd07d3d20d98"),
        27 if patch == 1 => Ok("99bfc70998720659b34c40807bf53001ded52d95"),
        27 => Ok("d6023f415aa3fc509fc34100ee04fbcf67f757e1"),
        26 if patch == 2 => Ok("b7a14a21dba30e2e9df1123222c8ca1e27153578"),
        26 if patch == 1 => Ok("46ac761f9d9925b20409262485d908b37c112f36"),
        26 => Ok("d7801a1f8b6f7a77952b55ba7e002a455ac4f35e"),
        25 => Ok("63276bd63d3bd8722867c6c4f21333db8a4b8775"),
        24 if patch == 1 => Ok("e1782455ebfae398636c72627c2bf4cd4bbce4f1"),
        24 => Ok("0e31da3b89faecbb3cc11b3d5485fea6813b4d74"),
        23 => Ok("6a3b6b8839092d9b9e6c2ca86b5430fcdfd871ab"),
        22 if patch == 1 => Ok("0d586d6615330af074c7f93b1458c9b1316d3b22"),
        22 => Ok("7147211a533c9e49dc2e6fe2c19ce2a2734b0a85"),
        21 => Ok("f0031285a211d33b3517acca8c0dd7f8e7bc2919"),
        20 => Ok("7a854f4bc6ca225cb3475926cfc538a786e7e911"),
        2 => Ok("f557861f822c34f07270347b94b5280de20a597e"),
        19 => Ok("3289ae1639b0bb368ebdd62657b18b4815cce874"),
        18 => Ok("2d7dc18b235d643a4f67df13ed592f4faebbddcc"),
        17 => Ok("58cde5c143c011bc939bd0060ad3e2c4ffcc2d24"),
        16 => Ok("a9198e3dd7958a1449d8857c35a9e95100af815d"),
        15 if patch == 1 => Ok("8a753bfbd32b006531dabe1a6102b2c8dac6fd91"),
        15 => Ok("bc5a29d5c89eab3dfa8dc6d9d64ef8e9d808dfa9"),
        14 => Ok("453f3aefdd8e9b077779940210a41ebea3f0d458"),
        13 => Ok("bc7bdd4d6e9fb1ed15acd4117da3469fa18b45c8"),
        12 if patch == 1 => Ok("122a68b56819eed612d828156eded0eea4bb39d9"),
        12 => Ok("961a3c3b8d480a8b7b2df0669f8bfdfc1d1ce153"),
        11 => Ok("f63a9c0cde3e69f0e283dd7dcd710a79a6869dee"),
        10 => Ok("d5678055c2bca22696e870f9103dc10aa6c35f41"),
        1 => Ok("bc3c16f09287e5545c1d3f76b7abd54f2eca868b"),
        88 => Ok("28f5c58846af69ce1a13e529312404f11fcdea49"),
        87 => Ok("a9edcdbe65eaccae242ec93e5831da9b826756b1"),
        86 => Ok("c8496d7e31c236266bbf00b10d40234abbed55eb"),
        85 if patch == 1 => Ok("0035dbbca59afecf0b4e53d96d61e00680dca9de"),
        85 => Ok("ebfd33271586f0dd1bc6fe1016a0c38aae31f88b"),
        84 if patch == 1 => Ok("39e90365d9562dabea5c415e23838581e5d88ab1"),
        84 => Ok("a7abb900ebbe7f481d2a8826fbd0718b79ca58f5"),
        83 => Ok("5056432f7fbd9d936f350c0d0f990ca5df1d5e20"),
        82 => Ok("6a9c384f2e78b84f43686755309c235908bbd784"),
        _ => Err("Version not found"),
    }
}
#[inline]
pub(crate) fn version_exists(minor: u16, patch: u16) -> bool {
    match minor {
        9 if patch == 0 => true,
        81 if patch == 0 => true,
        80 if patch == 1 => true,
        80 if patch == 0 => true,
        8 if patch == 0 => true,
        79 if patch == 0 => true,
        78 if patch == 0 => true,
        77 if patch == 2 => true,
        77 if patch == 1 => true,
        77 if patch == 0 => true,
        76 if patch == 0 => true,
        75 if patch == 0 => true,
        74 if patch == 1 => true,
        74 if patch == 0 => true,
        73 if patch == 0 => true,
        72 if patch == 1 => true,
        72 if patch == 0 => true,
        71 if patch == 1 => true,
        71 if patch == 0 => true,
        70 if patch == 0 => true,
        7 if patch == 0 => true,
        69 if patch == 0 => true,
        68 if patch == 2 => true,
        68 if patch == 1 => true,
        68 if patch == 0 => true,
        67 if patch == 1 => true,
        67 if patch == 0 => true,
        66 if patch == 1 => true,
        66 if patch == 0 => true,
        65 if patch == 0 => true,
        64 if patch == 0 => true,
        63 if patch == 0 => true,
        62 if patch == 1 => true,
        62 if patch == 0 => true,
        61 if patch == 0 => true,
        60 if patch == 0 => true,
        6 if patch == 0 => true,
        59 if patch == 0 => true,
        58 if patch == 1 => true,
        58 if patch == 0 => true,
        57 if patch == 0 => true,
        56 if patch == 1 => true,
        56 if patch == 0 => true,
        55 if patch == 0 => true,
        54 if patch == 0 => true,
        53 if patch == 0 => true,
        52 if patch == 1 => true,
        52 if patch == 0 => true,
        51 if patch == 0 => true,
        50 if patch == 0 => true,
        5 if patch == 0 => true,
        49 if patch == 0 => true,
        48 if patch == 0 => true,
        47 if patch == 0 => true,
        46 if patch == 0 => true,
        45 if patch == 2 => true,
        45 if patch == 1 => true,
        45 if patch == 0 => true,
        44 if patch == 1 => true,
        44 if patch == 0 => true,
        43 if patch == 1 => true,
        43 if patch == 0 => true,
        42 if patch == 0 => true,
        41 if patch == 1 => true,
        41 if patch == 0 => true,
        40 if patch == 0 => true,
        4 if patch == 0 => true,
        39 if patch == 0 => true,
        38 if patch == 0 => true,
        37 if patch == 0 => true,
        36 if patch == 0 => true,
        35 if patch == 0 => true,
        34 if patch == 2 => true,
        34 if patch == 1 => true,
        34 if patch == 0 => true,
        33 if patch == 0 => true,
        32 if patch == 0 => true,
        31 if patch == 1 => true,
        31 if patch == 0 => true,
        30 if patch == 1 => true,
        30 if patch == 0 => true,
        3 if patch == 0 => true,
        29 if patch == 2 => true,
        29 if patch == 1 => true,
        29 if patch == 0 => true,
        28 if patch == 0 => true,
        27 if patch == 2 => true,
        27 if patch == 1 => true,
        27 if patch == 0 => true,
        26 if patch == 2 => true,
        26 if patch == 1 => true,
        26 if patch == 0 => true,
        25 if patch == 0 => true,
        24 if patch == 1 => true,
        24 if patch == 0 => true,
        23 if patch == 0 => true,
        22 if patch == 1 => true,
        22 if patch == 0 => true,
        21 if patch == 0 => true,
        20 if patch == 0 => true,
        2 if patch == 0 => true,
        19 if patch == 0 => true,
        18 if patch == 0 => true,
        17 if patch == 0 => true,
        16 if patch == 0 => true,
        15 if patch == 1 => true,
        15 if patch == 0 => true,
        14 if patch == 0 => true,
        13 if patch == 0 => true,
        12 if patch == 1 => true,
        12 if patch == 0 => true,
        11 if patch == 0 => true,
        10 if patch == 0 => true,
        1 if patch == 0 => true,
        12 if patch == 0 => true,
        11 if patch == 0 => true,
        88 if patch == 0 => true,
        87 if patch == 0 => true,
        86 if patch == 0 => true,
        85 if patch == 1 => true,
        85 if patch == 0 => true,
        84 if patch == 1 => true,
        84 if patch == 0 => true,
        83 if patch == 0 => true,
        82 if patch == 0 => true,
        _ => false,
    }
}
#[inline]
pub(crate) fn all_versions() -> [((u16, u16, u16), i64); 125] {
    [
        ((1, 88, 0), 1750961413),
        ((1, 87, 0), 1747330093),
        ((1, 86, 0), 1743673352),
        ((1, 85, 1), 1742319689),
        ((1, 85, 0), 1740071365),
        ((1, 84, 1), 1738262759),
        ((1, 84, 0), 1736437309),
        ((1, 83, 0), 1732802513),
        ((1, 82, 0), 1729183468),
        ((1, 81, 0), 1725552388),
        ((1, 80, 1), 1723126065),
        ((1, 80, 0), 1721908957),
        ((1, 79, 0), 1718287478),
        ((1, 78, 0), 1714653548),
        ((1, 77, 2), 1712698776),
        ((1, 77, 1), 1711628578),
        ((1, 77, 0), 1711025824),
        ((1, 76, 0), 1707401220),
        ((1, 75, 0), 1703780510),
        ((1, 74, 1), 1701959035),
        ((1, 74, 0), 1700142571),
        ((1, 73, 0), 1696522231),
        ((1, 72, 1), 1695132379),
        ((1, 72, 0), 1692884807),
        ((1, 71, 1), 1691086303),
        ((1, 71, 0), 1689257048),
        ((1, 70, 0), 1685645566),
        ((1, 69, 0), 1682001579),
        ((1, 68, 2), 1680008162),
        ((1, 68, 1), 1679581283),
        ((1, 68, 0), 1678373120),
        ((1, 67, 1), 1675956747),
        ((1, 67, 0), 1674746027),
        ((1, 66, 1), 1673394800),
        ((1, 66, 0), 1671120663),
        ((1, 65, 0), 1667484517),
        ((1, 64, 0), 1663853457),
        ((1, 63, 0), 1660229201),
        ((1, 62, 1), 1658237720),
        ((1, 62, 0), 1656603397),
        ((1, 61, 0), 1652967346),
        ((1, 60, 0), 1649336651),
        ((1, 59, 0), 1645718740),
        ((1, 58, 1), 1642711960),
        ((1, 58, 0), 1642090431),
        ((1, 57, 0), 1638453846),
        ((1, 56, 1), 1635764606),
        ((1, 56, 0), 1634825356),
        ((1, 55, 0), 1631195195),
        ((1, 54, 0), 1627565195),
        ((1, 53, 0), 1623939692),
        ((1, 52, 1), 1620655243),
        ((1, 52, 0), 1620313522),
        ((1, 51, 0), 1616679019),
        ((1, 50, 0), 1613052628),
        ((1, 49, 0), 1609424231),
        ((1, 48, 0), 1605795177),
        ((1, 47, 0), 1602160872),
        ((1, 46, 0), 1598541310),
        ((1, 45, 2), 1596464307),
        ((1, 45, 1), 1596130229),
        ((1, 45, 0), 1594904894),
        ((1, 44, 1), 1592485824),
        ((1, 44, 0), 1591289990),
        ((1, 43, 1), 1588861775),
        ((1, 43, 0), 1587650851),
        ((1, 42, 0), 1584021924),
        ((1, 41, 1), 1582812747),
        ((1, 41, 0), 1580400735),
        ((1, 40, 0), 1576767764),
        ((1, 39, 0), 1573132268),
        ((1, 38, 0), 1569504205),
        ((1, 37, 0), 1565875971),
        ((1, 36, 0), 1562238698),
        ((1, 35, 0), 1558640997),
        ((1, 34, 2), 1557839781),
        ((1, 34, 1), 1556209306),
        ((1, 34, 0), 1555002328),
        ((1, 33, 0), 1551377538),
        ((1, 32, 0), 1547751362),
        ((1, 31, 1), 1545326056),
        ((1, 31, 0), 1544113017),
        ((1, 30, 1), 1541698381),
        ((1, 30, 0), 1540482286),
        ((1, 29, 2), 1539359694),
        ((1, 29, 1), 1537894264),
        ((1, 29, 0), 1536853732),
        ((1, 28, 0), 1533222728),
        ((1, 27, 2), 1532101166),
        ((1, 27, 1), 1531232150),
        ((1, 27, 0), 1529611688),
        ((1, 26, 2), 1528216213),
        ((1, 26, 1), 1527609068),
        ((1, 26, 0), 1525970602),
        ((1, 25, 0), 1522305786),
        ((1, 24, 1), 1519947517),
        ((1, 24, 0), 1518718666),
        ((1, 23, 0), 1515080803),
        ((1, 22, 1), 1511401652),
        ((1, 22, 0), 1511380094),
        ((1, 21, 0), 1507819205),
        ((1, 20, 0), 1504198621),
        ((1, 19, 0), 1500564126),
        ((1, 18, 0), 1496937783),
        ((1, 17, 0), 1493316190),
        ((1, 16, 0), 1489684542),
        ((1, 15, 1), 1486671910),
        ((1, 15, 0), 1486061282),
        ((1, 14, 0), 1482433817),
        ((1, 13, 0), 1478808214),
        ((1, 12, 1), 1477001625),
        ((1, 12, 0), 1475173179),
        ((1, 11, 0), 1471541812),
        ((1, 10, 0), 1467911358),
        ((1, 9, 0), 1464280168),
        ((1, 8, 0), 1460653474),
        ((1, 7, 0), 1457031860),
        ((1, 6, 0), 1453412913),
        ((1, 5, 0), 1449760900),
        ((1, 4, 0), 1446141743),
        ((1, 3, 0), 1442511414),
        ((1, 2, 0), 1438966327),
        ((1, 1, 0), 1435253745),
        ((0, 12, 0), 1412872439),
        ((0, 11, 0), 1404324205),
    ]
}
#[inline]
pub(crate) fn timestamp_ranges(timestamp: i64) -> Result<(u16, u16, u16), &'static str> {
    match timestamp - 1 {
        1747330093..1750961413 => Ok((1, 88, 0)),
        1743673352..1747330093 => Ok((1, 87, 0)),
        1742319689..1743673352 => Ok((1, 86, 0)),
        1740071365..1742319689 => Ok((1, 85, 1)),
        1738262759..1740071365 => Ok((1, 85, 0)),
        1736437309..1738262759 => Ok((1, 84, 1)),
        1732802513..1736437309 => Ok((1, 84, 0)),
        1729183468..1732802513 => Ok((1, 83, 0)),
        1725552388..1729183468 => Ok((1, 82, 0)),
        1723126065..1725552388 => Ok((1, 81, 0)),
        1721908957..1723126065 => Ok((1, 80, 1)),
        1718287478..1721908957 => Ok((1, 80, 0)),
        1714653548..1718287478 => Ok((1, 79, 0)),
        1712698776..1714653548 => Ok((1, 78, 0)),
        1711628578..1712698776 => Ok((1, 77, 2)),
        1711025824..1711628578 => Ok((1, 77, 1)),
        1707401220..1711025824 => Ok((1, 77, 0)),
        1703780510..1707401220 => Ok((1, 76, 0)),
        1701959035..1703780510 => Ok((1, 75, 0)),
        1700142571..1701959035 => Ok((1, 74, 1)),
        1696522231..1700142571 => Ok((1, 74, 0)),
        1695132379..1696522231 => Ok((1, 73, 0)),
        1692884807..1695132379 => Ok((1, 72, 1)),
        1691086303..1692884807 => Ok((1, 72, 0)),
        1689257048..1691086303 => Ok((1, 71, 1)),
        1685645566..1689257048 => Ok((1, 71, 0)),
        1682001579..1685645566 => Ok((1, 70, 0)),
        1680008162..1682001579 => Ok((1, 69, 0)),
        1679581283..1680008162 => Ok((1, 68, 2)),
        1678373120..1679581283 => Ok((1, 68, 1)),
        1675956747..1678373120 => Ok((1, 68, 0)),
        1674746027..1675956747 => Ok((1, 67, 1)),
        1673394800..1674746027 => Ok((1, 67, 0)),
        1671120663..1673394800 => Ok((1, 66, 1)),
        1667484517..1671120663 => Ok((1, 66, 0)),
        1663853457..1667484517 => Ok((1, 65, 0)),
        1660229201..1663853457 => Ok((1, 64, 0)),
        1658237720..1660229201 => Ok((1, 63, 0)),
        1656603397..1658237720 => Ok((1, 62, 1)),
        1652967346..1656603397 => Ok((1, 62, 0)),
        1649336651..1652967346 => Ok((1, 61, 0)),
        1645718740..1649336651 => Ok((1, 60, 0)),
        1642711960..1645718740 => Ok((1, 59, 0)),
        1642090431..1642711960 => Ok((1, 58, 1)),
        1638453846..1642090431 => Ok((1, 58, 0)),
        1635764606..1638453846 => Ok((1, 57, 0)),
        1634825356..1635764606 => Ok((1, 56, 1)),
        1631195195..1634825356 => Ok((1, 56, 0)),
        1627565195..1631195195 => Ok((1, 55, 0)),
        1623939692..1627565195 => Ok((1, 54, 0)),
        1620655243..1623939692 => Ok((1, 53, 0)),
        1620313522..1620655243 => Ok((1, 52, 1)),
        1616679019..1620313522 => Ok((1, 52, 0)),
        1613052628..1616679019 => Ok((1, 51, 0)),
        1609424231..1613052628 => Ok((1, 50, 0)),
        1605795177..1609424231 => Ok((1, 49, 0)),
        1602160872..1605795177 => Ok((1, 48, 0)),
        1598541310..1602160872 => Ok((1, 47, 0)),
        1596464307..1598541310 => Ok((1, 46, 0)),
        1596130229..1596464307 => Ok((1, 45, 2)),
        1594904894..1596130229 => Ok((1, 45, 1)),
        1592485824..1594904894 => Ok((1, 45, 0)),
        1591289990..1592485824 => Ok((1, 44, 1)),
        1588861775..1591289990 => Ok((1, 44, 0)),
        1587650851..1588861775 => Ok((1, 43, 1)),
        1584021924..1587650851 => Ok((1, 43, 0)),
        1582812747..1584021924 => Ok((1, 42, 0)),
        1580400735..1582812747 => Ok((1, 41, 1)),
        1576767764..1580400735 => Ok((1, 41, 0)),
        1573132268..1576767764 => Ok((1, 40, 0)),
        1569504205..1573132268 => Ok((1, 39, 0)),
        1565875971..1569504205 => Ok((1, 38, 0)),
        1562238698..1565875971 => Ok((1, 37, 0)),
        1558640997..1562238698 => Ok((1, 36, 0)),
        1557839781..1558640997 => Ok((1, 35, 0)),
        1556209306..1557839781 => Ok((1, 34, 2)),
        1555002328..1556209306 => Ok((1, 34, 1)),
        1551377538..1555002328 => Ok((1, 34, 0)),
        1547751362..1551377538 => Ok((1, 33, 0)),
        1545326056..1547751362 => Ok((1, 32, 0)),
        1544113017..1545326056 => Ok((1, 31, 1)),
        1541698381..1544113017 => Ok((1, 31, 0)),
        1540482286..1541698381 => Ok((1, 30, 1)),
        1539359694..1540482286 => Ok((1, 30, 0)),
        1537894264..1539359694 => Ok((1, 29, 2)),
        1536853732..1537894264 => Ok((1, 29, 1)),
        1533222728..1536853732 => Ok((1, 29, 0)),
        1532101166..1533222728 => Ok((1, 28, 0)),
        1531232150..1532101166 => Ok((1, 27, 2)),
        1529611688..1531232150 => Ok((1, 27, 1)),
        1528216213..1529611688 => Ok((1, 27, 0)),
        1527609068..1528216213 => Ok((1, 26, 2)),
        1525970602..1527609068 => Ok((1, 26, 1)),
        1522305786..1525970602 => Ok((1, 26, 0)),
        1519947517..1522305786 => Ok((1, 25, 0)),
        1518718666..1519947517 => Ok((1, 24, 1)),
        1515080803..1518718666 => Ok((1, 24, 0)),
        1511401652..1515080803 => Ok((1, 23, 0)),
        1511380094..1511401652 => Ok((1, 22, 1)),
        1507819205..1511380094 => Ok((1, 22, 0)),
        1504198621..1507819205 => Ok((1, 21, 0)),
        1500564126..1504198621 => Ok((1, 20, 0)),
        1496937783..1500564126 => Ok((1, 19, 0)),
        1493316190..1496937783 => Ok((1, 18, 0)),
        1489684542..1493316190 => Ok((1, 17, 0)),
        1486671910..1489684542 => Ok((1, 16, 0)),
        1486061282..1486671910 => Ok((1, 15, 1)),
        1482433817..1486061282 => Ok((1, 15, 0)),
        1478808214..1482433817 => Ok((1, 14, 0)),
        1477001625..1478808214 => Ok((1, 13, 0)),
        1475173179..1477001625 => Ok((1, 12, 1)),
        1471541812..1475173179 => Ok((1, 12, 0)),
        1467911358..1471541812 => Ok((1, 11, 0)),
        1464280168..1467911358 => Ok((1, 10, 0)),
        1460653474..1464280168 => Ok((1, 9, 0)),
        1457031860..1460653474 => Ok((1, 8, 0)),
        1453412913..1457031860 => Ok((1, 7, 0)),
        1449760900..1453412913 => Ok((1, 6, 0)),
        1446141743..1449760900 => Ok((1, 5, 0)),
        1442511414..1446141743 => Ok((1, 4, 0)),
        1438966327..1442511414 => Ok((1, 3, 0)),
        1435253745..1438966327 => Ok((1, 2, 0)),
        1412872439..1435253745 => Ok((1, 1, 0)),
        1404324205..1412872439 => Ok((0, 12, 0)),
        ..1404324205 => Ok((0, 11, 0)),
        _ => Err("Timestamp is not a version's release date, maybe it is the current version?"),
    }
}
