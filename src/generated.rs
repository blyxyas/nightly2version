pub(crate) fn correlations_generated_rs(minor: u16, patch: u16) -> u128 {
match minor {
11 => {1404324205},
12 => {1412872439},
1 => {1435253745},
10 => {1467911358},
11 => {1471541812},
12 => {1475173179},
12 if patch == 1 => {1477001625},
13 => {1478808214},
14 => {1482433817},
15 => {1486061282},
15 if patch == 1 => {1486671910},
16 => {1489684542},
17 => {1493316190},
18 => {1496937783},
19 => {1500564126},
2 => {1438966327},
20 => {1504198621},
21 => {1507819205},
22 => {1511380094},
22 if patch == 1 => {1511401652},
23 => {1515080803},
24 => {1518718666},
24 if patch == 1 => {1519947517},
25 => {1522305786},
26 => {1525970602},
26 if patch == 1 => {1527609068},
26 if patch == 2 => {1528216213},
27 => {1529611688},
27 if patch == 1 => {1531232150},
27 if patch == 2 => {1532101166},
28 => {1533222728},
29 => {1536853732},
29 if patch == 1 => {1537894264},
29 if patch == 2 => {1539359694},
3 => {1442511414},
30 => {1540482286},
30 if patch == 1 => {1541698381},
31 => {1544113017},
31 if patch == 1 => {1545326056},
32 => {1547751362},
33 => {1551377538},
34 => {1555002328},
34 if patch == 1 => {1556209306},
34 if patch == 2 => {1557839781},
35 => {1558640997},
36 => {1562238698},
37 => {1565875971},
38 => {1569504205},
39 => {1573132268},
4 => {1446141743},
40 => {1576767764},
41 => {1580400735},
41 if patch == 1 => {1582812747},
42 => {1584021924},
43 => {1587650851},
43 if patch == 1 => {1588861775},
44 => {1591289990},
44 if patch == 1 => {1592485824},
45 => {1594904894},
45 if patch == 1 => {1596130229},
45 if patch == 2 => {1596464307},
46 => {1598541310},
47 => {1602160872},
48 => {1605795177},
49 => {1609424231},
5 => {1449760900},
50 => {1613052628},
51 => {1616679019},
52 => {1620313522},
52 if patch == 1 => {1620655243},
53 => {1623939692},
54 => {1627565195},
55 => {1631195195},
56 => {1634825356},
56 if patch == 1 => {1635764606},
57 => {1638453846},
58 => {1642090431},
58 if patch == 1 => {1642711960},
59 => {1645718740},
6 => {1453412913},
60 => {1649336651},
61 => {1652967346},
62 => {1656603397},
62 if patch == 1 => {1658237720},
63 => {1660229201},
64 => {1663853457},
65 => {1667484517},
66 => {1671120663},
66 if patch == 1 => {1673394800},
67 => {1674746027},
67 if patch == 1 => {1675956747},
68 => {1678373120},
68 if patch == 1 => {1679581283},
68 if patch == 2 => {1680008162},
69 => {1682001579},
7 => {1457031860},
70 => {1685645566},
71 => {1689257048},
71 if patch == 1 => {1691086303},
72 => {1692884807},
72 if patch == 1 => {1695132379},
73 => {1696522231},
74 => {1700142571},
74 if patch == 1 => {1701959035},
75 => {1703780510},
76 => {1707401220},
77 => {1711025824},
77 if patch == 1 => {1711628578},
77 if patch == 2 => {1712698776},
78 => {1714653548},
79 => {1718287478},
8 => {1460653474},
80 => {1721908957},
9 => {1464280168},
_ => panic!("Version {}.{}not found", minor, patch)
} }

pub(crate) fn correlations_commits(minor: u16, patch: u16) -> &'static str {
match minor {

11 => {"e1247cb1d0d681be034adb4b558b5a0c0d5720f9"},
12 => {"f0c419429ef30723ceaf6b42f9b5a2aeb5d2e2d1"},
1 => {"bc3c16f09287e5545c1d3f76b7abd54f2eca868b"},
10 => {"d5678055c2bca22696e870f9103dc10aa6c35f41"},
11 => {"f63a9c0cde3e69f0e283dd7dcd710a79a6869dee"},
12 => {"961a3c3b8d480a8b7b2df0669f8bfdfc1d1ce153"},
12 if patch == 1 => {"122a68b56819eed612d828156eded0eea4bb39d9"},
13 => {"bc7bdd4d6e9fb1ed15acd4117da3469fa18b45c8"},
14 => {"453f3aefdd8e9b077779940210a41ebea3f0d458"},
15 => {"bc5a29d5c89eab3dfa8dc6d9d64ef8e9d808dfa9"},
15 if patch == 1 => {"8a753bfbd32b006531dabe1a6102b2c8dac6fd91"},
16 => {"a9198e3dd7958a1449d8857c35a9e95100af815d"},
17 => {"58cde5c143c011bc939bd0060ad3e2c4ffcc2d24"},
18 => {"2d7dc18b235d643a4f67df13ed592f4faebbddcc"},
19 => {"3289ae1639b0bb368ebdd62657b18b4815cce874"},
2 => {"f557861f822c34f07270347b94b5280de20a597e"},
20 => {"7a854f4bc6ca225cb3475926cfc538a786e7e911"},
21 => {"f0031285a211d33b3517acca8c0dd7f8e7bc2919"},
22 => {"7147211a533c9e49dc2e6fe2c19ce2a2734b0a85"},
22 if patch == 1 => {"0d586d6615330af074c7f93b1458c9b1316d3b22"},
23 => {"6a3b6b8839092d9b9e6c2ca86b5430fcdfd871ab"},
24 => {"0e31da3b89faecbb3cc11b3d5485fea6813b4d74"},
24 if patch == 1 => {"e1782455ebfae398636c72627c2bf4cd4bbce4f1"},
25 => {"63276bd63d3bd8722867c6c4f21333db8a4b8775"},
26 => {"d7801a1f8b6f7a77952b55ba7e002a455ac4f35e"},
26 if patch == 1 => {"46ac761f9d9925b20409262485d908b37c112f36"},
26 if patch == 2 => {"b7a14a21dba30e2e9df1123222c8ca1e27153578"},
27 => {"d6023f415aa3fc509fc34100ee04fbcf67f757e1"},
27 if patch == 1 => {"99bfc70998720659b34c40807bf53001ded52d95"},
27 if patch == 2 => {"dac474b10ffa573685716899661dbd07d3d20d98"},
28 => {"0bbeff8f750dd68e687403e1eb663373f8ac3154"},
29 => {"de9114b0633ed9f3160f9c2a98e239383eb105f1"},
29 if patch == 1 => {"ca96ed349acfaf2a756ccc3d0edfc2e457eb4376"},
29 if patch == 2 => {"4da54a1fa57e588832f230a15b9d5db01a53b996"},
3 => {"1896219fb7e33b0d317ec0be4a2e21d86af2ba8e"},
30 => {"a06e4e4e9f995fd0bff09c735bf932aaea634adb"},
30 if patch == 1 => {"9df3c8af03e79d31289f43fc8d2dda2e544af348"},
31 => {"3b4b9a4247febf0e71f3480fd34775850b77fea4"},
31 if patch == 1 => {"4bc6ce83704da6fd5d56856c54393ba0df03ad95"},
32 => {"3854d05abcd904e18311ca793f60d0b71e800a44"},
33 => {"fc3b53ff103062247686e14f83f44021d5558640"},
34 => {"e1a98a0cb13cef05a3d478f75911ae3d8035b278"},
34 if patch == 1 => {"433755e0043306b621f092dad44d2aed8cd28285"},
34 if patch == 2 => {"a6a44c5d923aadedf4e3507cc001bdfdb2abf544"},
35 => {"ac2452c1b746a4bc7ffd68120dce9564d93deb35"},
36 => {"dd20f18bef1af465a09b08d92d289ab880011eb0"},
37 => {"d0be6f624ccbeb1a9fb2796c0a6d99f8bdc9c781"},
38 => {"cabda533c937a763582a2c505b38bdecb08f80a5"},
39 => {"d650e13c2975bde610b17d52db7710f38906f05c"},
4 => {"e10441ae5e655113cd024da8760b3d417ff0e4f7"},
40 => {"d4627cdbd2a26c536dcf015c024e24b798471018"},
41 => {"a2bd8df40d3a882e40b6e03b142517172a517024"},
41 if patch == 1 => {"e8b466f79c5a5cf5870b284ff433707ec7a36af3"},
42 => {"874f7cbe352033cac5a8bc889847da2fe1d13e9f"},
43 => {"2df93b48cdfa2a6e1de9edc263f879874bdf47e9"},
43 if patch == 1 => {"844c53d6d3fd180615cbddd9d882689d0ee6cde2"},
44 => {"9dcb33aa56d9783788a1321c6068e2808e1b8199"},
44 if patch == 1 => {"b16f6d814a575dec93eefc506c54487a9c6eeb8f"},
45 => {"b13c29dde6865973be1146727cb41b382e1fc905"},
45 if patch == 1 => {"e147b1a0aefe9dcb1669a6e3e19d2ebfa5820455"},
45 if patch == 2 => {"fa5abfa682ef609bbce4ab16adb1f94d3869e54e"},
46 => {"0fbbbf27e72e06134ab2317de07f69635613e06d"},
47 => {"af10768031c76eb3181543bd1cfef8ba8be7e443"},
48 => {"82457e09d726f92f384e375c5a7086e49939a3e5"},
49 => {"a31c977c800ff16bff77a8466983fde7469abb8f"},
5 => {"d8027afc009e55fb07776831094a13b1b64b6305"},
50 => {"11199001337213fbfe03cf82e8987846f1d70eaf"},
51 => {"2949b7456a958923d34eec389490adc70efd85a3"},
52 => {"af65d486380d32ae42853e8dc3db0147869ba522"},
52 if patch == 1 => {"390d3e847cb6031fb6d5063ed032ea406cde625e"},
53 => {"6c84a92fdd46ff5a8dcf903622caf360f4425d93"},
54 => {"0170fff6e0b50bcab2f8bf3321e6b5652226f20c"},
55 => {"06aec595a7ef346bd884d12b4853e9a432733e42"},
56 => {"bd180709d8dbe163333521b01320d97e4da3d182"},
56 if patch == 1 => {"7eb244d7dd25d6137469e8abc71b2fe5b0ccf527"},
57 => {"20e5e087c3de0fb9c211aa80c0d505a6b2c099be"},
58 => {"61ea9a44c133f403599e76a336331fcb675399cd"},
58 if patch == 1 => {"8832d21e92d9a502c4ae591b9d529095dcf46c05"},
59 => {"4c9bd1611af515ec325e4322ebb22f598b8569e7"},
6 => {"4fe6048e8dd100ec1c61a020d01f75046b42e818"},
60 => {"6d94daf692c1347229357379e8b5d820247b9e65"},
61 => {"a52dc32db4ecf9b0765467a1403ce59289fe2c3a"},
62 => {"cb333ec50ce4cc10c0a4b9bd86f86d9e09cbb1d3"},
62 if patch == 1 => {"110f538e89b02301f5a9195c4b7b219cbff214b8"},
63 => {"56438456e0322f813d21feb3af593d2351ec7c2e"},
64 => {"0ccf0f10802b45300c88ce37ea53edab23510a1c"},
65 => {"4abddbedbc8b5de785e7b8d31aaa31dde6bd430f"},
66 => {"f8ae1179a6deab019f4fc7aa79efbb7f15096a4f"},
66 if patch == 1 => {"c1512ff41a9e5c5fdc8c3b01efc376b1003ad404"},
67 => {"ffb22f48ba6fd4f2cf5b921c08b8e618aded1465"},
67 if patch == 1 => {"21a2bdd0c3252e9174cd0f659c52031bdcc11988"},
68 => {"473c2ac376d281c1e57254f93bfcd6d686b00f16"},
68 if patch == 1 => {"0204fd462a5fb7d6e4d84bc5b8f3072dd4029d43"},
68 if patch == 2 => {"b87584a0146f6fe23d68e4473f4ce20904c6ba27"},
69 => {"f0da3c89a5521a4d8c8e2fb89f863fd774ebb4d8"},
7 => {"2d2a9311cbe05eb47960489259e1e9408062c863"},
70 => {"0fe1acccb984f2008de6dc9dda167f388afdff70"},
71 => {"b793e03a8e63a096b14cda522a550a0efb6e287d"},
71 if patch == 1 => {"01839690de549c01de4ea4c8598892d110fbcbd8"},
72 => {"c3813f7d10107fe96c41a48201c1ed980d9c5343"},
72 if patch == 1 => {"6d4fbaaf678ec5718798476da04102e51796b45a"},
73 => {"6d43ffa8822bb652f716e757ca87eb7b0165cd04"},
74 => {"b092d43294e5543ab22d49b9bfbfe60f0a3bd826"},
74 if patch == 1 => {"7b77c6e026e6b0d7f3ea21dcaccd0a2c3d249aca"},
75 => {"2010168acb2062cf7705ccbf82e7fa84628a38f8"},
76 => {"1fc800020eb2f5ed3a66fcc04bab123a0ebf4295"},
77 => {"328759fef321406d7db674313585ca48452f311a"},
77 if patch == 1 => {"42b49de7b7c3bfd4fc990691342af520ce142013"},
77 if patch == 2 => {"8b02fd2ae7f1b730f5df5210fc93a9bbb803a1af"},
78 => {"631f1ea3617dcd61dbc5c1aa59e71e54fd47445c"},
79 => {"a6af86c649426f1d6a0ec4b188f1f5ff097c5bb0"},
8 => {"6bef00072dbaa86da9dc73b09f926cf67c696b39"},
80 => {"4f9c8cbf2386b5e35c5ba754b705c383c5f4b4cc"},
9 => {"6f9526945c2b832c0eb2187964ecd68f6ab0f600"},
_ => panic!("Version {}.{}not found", minor, patch)
} }
