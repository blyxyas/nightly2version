#[inline]
pub(crate) fn correlations_dates(minor: u16, patch: u16) -> anyhow::Result<i64> {
    match minor {
        9 => Ok(1464280168),
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
        _ => anyhow::bail!("Version {}.{}not found", minor, patch),
    }
}

#[inline]
pub(crate) fn correlations_commits(minor: u16, patch: u16) -> anyhow::Result<&'static str> {
    match minor {
        9 => Ok("6f9526945c2b832c0eb2187964ecd68f6ab0f600"),
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
        12 => Ok("f0c419429ef30723ceaf6b42f9b5a2aeb5d2e2d1"),
        11 => Ok("e1247cb1d0d681be034adb4b558b5a0c0d5720f9"),
        _ => anyhow::bail!("Version {}.{}not found", minor, patch),
    }
}
#[inline]
pub(crate) fn version_exists(minor: u16) -> bool {
    match minor {
        9 | 80 | 8 | 79 | 78 | 77 | 76 | 75 | 74 | 73 | 72 | 71 | 70 | 7 | 69 | 68 | 67 | 66
        | 65 | 64 | 63 | 62 | 61 | 60 | 6 | 59 | 58 | 57 | 56 | 55 | 54 | 53 | 52 | 51 | 50 | 5
        | 49 | 48 | 47 | 46 | 45 | 44 | 43 | 42 | 41 | 40 | 4 | 39 | 38 | 37 | 36 | 35 | 34
        | 33 | 32 | 31 | 30 | 3 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 2 | 19 | 18
        | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 1 | 12 | 11 => true,
        _ => false,
    }
}
