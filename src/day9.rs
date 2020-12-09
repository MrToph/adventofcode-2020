use std::collections::HashSet;

pub fn run(inputs: &[u64], preamble_length: usize) -> Result<u64, &'static str> {
    let mut tally = HashSet::<u64>::new();
    for i in 0..preamble_length {
        tally.insert(inputs[i]);
    }

    for i in preamble_length..inputs.len() {
        // println!("tally = {:?}", &tally);
        let found = tally.iter().any(|&entry| {
          &inputs[i] > &entry && tally.contains(&(inputs[i] - entry))
        });

        if !found {
            return Ok(inputs[i]);
        }
        tally.remove(&inputs[i - preamble_length]);
        tally.insert(inputs[i]);
    }
    panic!("all items could be matched")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: [u64; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    const PUZZLE_INPUT: [u64; 1000] = [
        21,
        41,
        40,
        48,
        6,
        37,
        31,
        10,
        50,
        9,
        44,
        45,
        36,
        34,
        13,
        17,
        32,
        16,
        8,
        35,
        30,
        42,
        29,
        24,
        11,
        14,
        83,
        22,
        15,
        23,
        18,
        19,
        20,
        21,
        91,
        25,
        33,
        26,
        27,
        28,
        38,
        34,
        31,
        32,
        45,
        64,
        30,
        35,
        29,
        60,
        36,
        37,
        47,
        39,
        67,
        40,
        51,
        46,
        48,
        61,
        52,
        53,
        54,
        55,
        63,
        59,
        75,
        62,
        65,
        83,
        76,
        108,
        91,
        88,
        104,
        82,
        77,
        95,
        106,
        86,
        127,
        97,
        114,
        100,
        171,
        105,
        113,
        162,
        130,
        121,
        124,
        137,
        233,
        208,
        201,
        153,
        159,
        163,
        287,
        211,
        188,
        172,
        290,
        186,
        183,
        313,
        197,
        205,
        267,
        218,
        266,
        261,
        430,
        245,
        258,
        323,
        398,
        331,
        312,
        335,
        316,
        573,
        471,
        355,
        450,
        455,
        417,
        401,
        463,
        380,
        402,
        415,
        423,
        721,
        476,
        503,
        625,
        557,
        628,
        1027,
        639,
        643,
        647,
        715,
        816,
        778,
        735,
        770,
        757,
        781,
        782,
        795,
        803,
        817,
        825,
        838,
        891,
        899,
        979,
        1131,
        1060,
        1185,
        1196,
        1267,
        1358,
        1282,
        1492,
        1362,
        2012,
        1505,
        1641,
        1516,
        1694,
        1737,
        1599,
        1612,
        1598,
        1898,
        1790,
        1663,
        2076,
        2407,
        1878,
        2866,
        2191,
        2672,
        2381,
        2629,
        3336,
        2798,
        4270,
        3743,
        2867,
        3146,
        3668,
        3262,
        4464,
        3197,
        4284,
        3261,
        5744,
        5160,
        4044,
        4999,
        3541,
        7545,
        4285,
        8026,
        4989,
        6203,
        5010,
        6842,
        5826,
        5665,
        5944,
        6013,
        7209,
        10886,
        11494,
        6459,
        6458,
        6802,
        6738,
        7305,
        10159,
        7585,
        7826,
        9485,
        8530,
        8551,
        9274,
        12471,
        9999,
        12285,
        11213,
        10675,
        18111,
        18743,
        12402,
        19611,
        12472,
        14988,
        12917,
        17311,
        13260,
        13196,
        18036,
        28017,
        14890,
        16115,
        15411,
        16356,
        19764,
        20487,
        17825,
        20674,
        28758,
        21212,
        21888,
        23077,
        25663,
        29616,
        27362,
        24874,
        25389,
        25668,
        40377,
        28671,
        50439,
        26456,
        28607,
        30301,
        36085,
        31005,
        31526,
        43493,
        59059,
        37589,
        38312,
        39037,
        42562,
        43100,
        48574,
        44965,
        73963,
        50263,
        70354,
        68367,
        57461,
        51057,
        52124,
        55063,
        55127,
        66386,
        59612,
        73401,
        82425,
        62531,
        108186,
        69115,
        75901,
        77349,
        158449,
        80874,
        81599,
        147703,
        88065,
        93539,
        100092,
        101320,
        118510,
        103181,
        131646,
        106120,
        106184,
        107187,
        122143,
        128528,
        155000,
        133013,
        135932,
        179082,
        138432,
        145016,
        182085,
        235874,
        158223,
        182919,
        162473,
        181691,
        181604,
        194859,
        199659,
        221691,
        302046,
        237766,
        209301,
        250671,
        213307,
        341835,
        268945,
        255156,
        261541,
        271445,
        324098,
        274364,
        283448,
        296655,
        339914,
        491514,
        320696,
        339827,
        344077,
        713205,
        539486,
        376463,
        394518,
        408960,
        459457,
        478246,
        422608,
        496755,
        463978,
        468463,
        516697,
        526601,
        611272,
        544989,
        545809,
        614278,
        557812,
        840832,
        734345,
        812210,
        1191451,
        660523,
        1009787,
        1017732,
        921452,
        1418207,
        770981,
        932441,
        1083131,
        882065,
        946709,
        886586,
        1563541,
        1013452,
        995064,
        1084413,
        1554776,
        1090798,
        1427874,
        1103621,
        1881650,
        1218335,
        2059167,
        1698796,
        1592964,
        1431504,
        1547109,
        1653046,
        1657567,
        1692433,
        3613943,
        1703422,
        2515917,
        1768651,
        1828774,
        2539632,
        3734252,
        2008516,
        2079477,
        2085862,
        4308283,
        3967512,
        2194419,
        2321956,
        2535125,
        2649839,
        2765444,
        3204676,
        2978613,
        3024468,
        3315760,
        5855392,
        3854513,
        3350000,
        3395855,
        3472073,
        5717811,
        3837290,
        3597425,
        3908251,
        4087993,
        4330472,
        5087400,
        4165339,
        4280281,
        4516375,
        4729544,
        4844258,
        4857081,
        5885125,
        5415283,
        5744057,
        6711615,
        6913185,
        6340228,
        6665760,
        6745855,
        8637795,
        7069498,
        12550885,
        7745541,
        7925283,
        7927897,
        7505676,
        7996244,
        11602936,
        8445620,
        9909396,
        8681714,
        9124539,
        9245919,
        10473601,
        9701339,
        10272364,
        16373517,
        17762334,
        12084285,
        18516121,
        13086083,
        14268125,
        13411615,
        18197647,
        14575174,
        16194037,
        15251217,
        18200261,
        21304275,
        15433573,
        32337508,
        16677958,
        24177521,
        17127334,
        17806253,
        17927633,
        18370458,
        18947258,
        53641783,
        28662832,
        25523581,
        25170368,
        25495900,
        27517858,
        35194079,
        26497698,
        30538949,
        34875605,
        41689937,
        29826391,
        30684790,
        31929175,
        33239826,
        37982233,
        32111531,
        35048416,
        34484211,
        34933587,
        53564537,
        35733886,
        53833200,
        37317716,
        62711937,
        51019481,
        51993598,
        52688226,
        61755566,
        56324089,
        54015556,
        82948656,
        65733206,
        60365340,
        60511181,
        61937922,
        62613965,
        64040706,
        65169001,
        86477809,
        90670459,
        118733538,
        70667473,
        69417798,
        118262011,
        87727484,
        73051602,
        88337197,
        89311314,
        118079655,
        103013079,
        104681824,
        123355699,
        118056262,
        120876521,
        124551887,
        127782966,
        135151004,
        148238665,
        129209707,
        166619746,
        153352020,
        133458504,
        154480315,
        140085271,
        142469400,
        143719075,
        228422468,
        157145282,
        177038798,
        264637158,
        162362916,
        272928782,
        193993138,
        207694903,
        247151224,
        222738086,
        238932783,
        242608149,
        245428408,
        390785384,
        360612884,
        262668211,
        419646947,
        269294978,
        493059626,
        282554671,
        273543775,
        384733701,
        351138420,
        371031936,
        300864357,
        461670869,
        319508198,
        582009863,
        356356054,
        588803176,
        401688041,
        416731224,
        430432989,
        603507278,
        481540932,
        536211986,
        488036557,
        629899829,
        531963189,
        542838753,
        545222882,
        551849649,
        556098446,
        574408132,
        658277476,
        593051973,
        949707426,
        620372555,
        657220411,
        675864252,
        1058908452,
        721196239,
        832121030,
        1081088530,
        818419265,
        847164213,
        1017752918,
        1172738582,
        1024248543,
        1570079981,
        731031916,
        1152335744,
        1871412756,
        1074801942,
        1475639676,
        1097072531,
        1126257781,
        1268916225,
        1194780687,
        1388252327,
        1213424528,
        1296236807,
        1578196129,
        1333084663,
        2064116579,
        1539615504,
        3045719657,
        1549451181,
        1665583478,
        1748784834,
        1755280459,
        2280483097,
        2482340753,
        3634196560,
        3213372669,
        1805833858,
        2646523712,
        2463054269,
        4801000116,
        2934499703,
        2601676855,
        2583033014,
        2937703508,
        3298236015,
        5761290284,
        4031791934,
        3045021641,
        4195974893,
        3089066685,
        3205198982,
        3215034659,
        4754650163,
        4086316955,
        3414368312,
        5668253251,
        3561114317,
        4268888127,
        7846021757,
        4388866872,
        4407510713,
        4452357570,
        5046087283,
        12253532470,
        5184709869,
        5517532717,
        5520736522,
        8818972537,
        9800737446,
        8841224442,
        7285041578,
        6606135958,
        6134088326,
        6503434997,
        8676398840,
        6629402971,
        6776148976,
        6975482629,
        11013646671,
        7949981189,
        7830002444,
        11183659689,
        8657754999,
        13293582012,
        8796377585,
        9453597996,
        9498444853,
        10230797152,
        18174843693,
        12126872480,
        11038269239,
        14453416186,
        12637523323,
        15942796577,
        13419129904,
        12910237302,
        12740224284,
        16506401284,
        13751631605,
        13405551947,
        14579384160,
        16626380029,
        14805485073,
        15779983633,
        22283418630,
        22193822280,
        19027174737,
        23948506541,
        18249975581,
        18952042849,
        32449197861,
        36035050235,
        21269066391,
        34894839426,
        23165141719,
        23675792562,
        25377747607,
        41157731240,
        25650461586,
        26159354188,
        32286384917,
        26145776231,
        29531615238,
        27157183552,
        27984936107,
        29384869233,
        37979217586,
        53333375774,
        76498517493,
        55035330819,
        55544223421,
        37202018430,
        64426454664,
        39519041972,
        111393356919,
        44944858953,
        94554372791,
        44434208110,
        46840934281,
        52696756957,
        51523523838,
        51796237817,
        51809815774,
        80685792405,
        52305130419,
        53302959783,
        54130712338,
        58916484471,
        55142119659,
        67364086819,
        66586887663,
        75181236016,
        76721060402,
        81636226540,
        82146877383,
        83953250082,
        84042952711,
        105926950155,
        84463900925,
        89379067063,
        96741096770,
        95957731948,
        105112775557,
        104114946193,
        201853872327,
        107447250078,
        108445079442,
        212560025635,
        105608090202,
        113047196809,
        196826317141,
        109272831997,
        114058604130,
        121729007322,
        133950974482,
        286317773252,
        177593958488,
        158357286942,
        192398329524,
        202349186972,
        167996202793,
        193494013256,
        195306017218,
        194987157265,
        185336799011,
        390293174483,
        200072678141,
        481623790470,
        209723036395,
        213055340280,
        214053169644,
        214880922199,
        218655287011,
        282054806923,
        227105800939,
        615574764952,
        223331436127,
        235787611452,
        326353489735,
        292308261424,
        335951245430,
        350755616466,
        343694085953,
        353333001804,
        404710193660,
        380323956276,
        378830812267,
        380642816229,
        385409477152,
        395059835406,
        545008776746,
        409795714536,
        422778376675,
        436386776407,
        427936262479,
        445761087950,
        433536209210,
        441986723138,
        632810423389,
        951526010382,
        925118684813,
        459119047579,
        528095872876,
        618661751159,
        817029592636,
        736165093618,
        724336902182,
        812367021477,
        733975818033,
        759154768543,
        1353054947292,
        788626526803,
        766052293381,
        1361505461220,
        804855549942,
        832574091211,
        837731977015,
        1060746685868,
        861472471689,
        875522932348,
        879297297160,
        1713745211616,
        1201141491681,
        1193094865612,
        1195284141197,
        987214920455,
        1077780798738,
        1740769768849,
        1352637569192,
        1593482076745,
        1458312720215,
        1483491670725,
        1595448289722,
        1667923823963,
        1670306068226,
        3224261439574,
        1627524765070,
        2941804390940,
        1848687392144,
        1637429641153,
        1694046562900,
        1699204448704,
        1866512217615,
        2657520988681,
        1754820229508,
        2445527640670,
        2064995719193,
        2273064939935,
        2180309786067,
        2182499061652,
        3258090584805,
        3732919543156,
        2810950289407,
        3078939960447,
        3152359283115,
        3177538233625,
        3111016435795,
        3222973054792,
        3565716666319,
        4781322504021,
        3621332447123,
        5468471278088,
        4455564001587,
        3331476204053,
        3759042282093,
        3393251011604,
        3454024678212,
        3819815948701,
        4991260075474,
        4510523359863,
        4245305505260,
        5405472116444,
        4362808847719,
        4993449351059,
        5261439022099,
        7321473649270,
        5889890249854,
        6256478194072,
        9026804563567,
        6263375718910,
        6288554669420,
        12312733724744,
        9221366453907,
        6785500882265,
        6724727215657,
        7441148395824,
        6847275689816,
        9710502872284,
        7699330183472,
        9656626730514,
        12690590258946,
        8445284753686,
        10501783699332,
        8608114352979,
        13211363899124,
        9238754856319,
        10626184566629,
        12614617465511,
        10254888373158,
        11517917216171,
        13135830359236,
        12146368443926,
        19963201969857,
        12551930388330,
        13074055551685,
        13013281885077,
        13510228097922,
        13572002905473,
        23178114954959,
        14165875611481,
        15292560443502,
        19234298919608,
        23639466451706,
        16144614937158,
        20997215142016,
        17053399106665,
        17684039610005,
        17846869209298,
        18863002726137,
        24698298832256,
        21385123300245,
        36775296810942,
        41323506061711,
        23765116471080,
        23664285660097,
        33139429652800,
        25159650329003,
        42843689939008,
        25565212273407,
        32976600053507,
        26523509982999,
        27082231003395,
        27737878516954,
        29458436054983,
        33028878337618,
        31437175380660,
        33991484146456,
        39069162910250,
        54361723353752,
        34737438716670,
        34900268315963,
        44929100212693,
        36709871935435,
        93430886264002,
        61982499319358,
        55023648328390,
        62638146832917,
        47429402131177,
        52647443276802,
        51683160312002,
        50724862602410,
        60302650990077,
        52088722256406,
        53605740986394,
        65592672893249,
        54261388499953,
        56540667058378,
        57196314571937,
        60895611435643,
        64466053718278,
        116317535495659,
        118727442218231,
        86988990572369,
        69637707032633,
        107732053121254,
        71610140251398,
        98692371254793,
        84139274066612,
        104330603588804,
        98154264733587,
        124661355361023,
        99112562443179,
        141250379072322,
        102408022914412,
    ];

    #[test]
    fn part1_works_for_sample_input() {
        assert_eq!(run(&SAMPLE_INPUT, 5).unwrap(), 127);
    }

    #[test]
    fn part1_works_for_puzzle_input() {
        assert_eq!(run(&PUZZLE_INPUT, 25).unwrap(), 731031916);
    }
}
