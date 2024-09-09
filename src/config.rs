pub(crate) const WIN_NT: [&str; 8] = ["5.1", "5.2", "6.0", "6.1", "6.2", "6.3", "6.4", "10.0"];
pub(crate) const WIN_BIT: [&str; 3] = ["Win64; x64", "WOW64", ""];
pub(crate) const MACINTOSH_VER: [&str; 19] = [
    "13_3_1", "12_6_5", "11_7_6", "10_15_7", "10_14_6", "10_13_6", "10_12_6", "10_11_6", "10_10_5",
    "10_9_5", "10_8_5", "10_7_5", "10_6_8", "10_5_8", "10_4_11", "10_3_9", "10_2_8", "10_1_5",
    "10_0_4",
];
pub(crate) const LINUX_VER: [&str; 13] = [
    "Ubuntu; Linux x86-64",
    "Ubuntu; Linux i686",
    "Ubuntu 24.04 LTS; Linux x86_64",
    "Ubuntu; Linux",
    "Debian; Linux",
    "Debian; Linux i686",
    "U; Linux armv5tejl",
    "Linux i686; nb-NO; rv:1.8.1.3",
    "FreeBSD amd64; rv:102.0",
    "FreeBSD amd64; rv:124.0",
    "U; FreeBSD amd64 3.99; rv:224.6",
    "FreeBSD amd64",
    "U; Linux i686",
];
pub(crate) const ANDROID_VER: [&str; 4] = ["9", "10", "11", "12"];

pub(crate) const HUAWEI: [&str; 35] = [
    // "HUAWEI U8825-1 Build/HuaweiU8825-1",
    // "HUAWEI G510-0200 Build/HuaweiG510-0200",
    // "HUAWEI G525-U00 Build/HuaweiG525-U00",
    // "HUAWEI G525-U00 Build/HuaweiG525-U00",
    // "HUAWEI U8950D Build/HuaweiU8950D",
    // "G7-L01 Build/HuaweiG7-L01",
    // "HUAWEI G700-U10 Build/HuaweiG700-U10",
    // "HUAWEI MT1-U06 Build/HuaweiMT1-U06",
    // "HUAWEI MT2-L01 Build/HuaweiMT2-L01; wv",
    // "U9200 Build/HuaweiU9200",
    // "HUAWEI P6-U06 Build/HuaweiP6-U06",
    "U8860 Build/HuaweiU8860",
    "HUAWEI U9508 Build/HuaweiU9508",
    "H30-U10 Build/HuaweiH30-U10",
    "HUAWEI G750-U10 Build/HuaweiG750-U10",
    "NEM-L51 Build/HONORNEM-L51",
    "KIW-L21 Build/HONORKIW-L21",
    "HUAWEI BLN-L24 Build/HUAWEIBLN-L24",
    "BLN-L22 Build/HONORBLN-L22",
    "PLK-AL10 Build/HONORPLK-AL10",
    "NEM-L21 Build/HONORNEM-L21",
    "LND-L29 Build/HONORLND-L29",
    "ATH-AL00 Build/HONORATH-AL00",
    "BND-L21 Build/HONORBND-L21",
    "FRD-L19 Build/HUAWEIFRD-L19",
    "PRA-AL00X Build/HONORPRA-AL00X",
    "PRA-AL00X Build/HONORPRA-AL00X",
    "VEN-L22 Build/HONORVEN-L22",
    "LLD-AL10 Build/HONORLLD-AL10",
    "LLD-AL20 Build/HONORLLD-AL20",
    "AUM-AL20 Build/HONORAUM-AL20",
    "EDI-AL10 Build/HUAWEIEDISON-AL10",
    "HUAWEI U8950-1 Build/HuaweiU8950-1",
    "RNE-L21 Build/HUAWEIRNE-L21",
    "EVR-AL00 Build/HUAWEIEVR-AL00",
    "HUAWEI NXT-AL10 Build/HUAWEINXT-AL10",
    "Nexus 6P",
    "LDN-LX2 Build/HUAWEILDN-LX2",
    "BAC-L21 Build/HUAWEIBAC-L21",
    "INE-LX2 Build/HUAWEIINE-LX2",
    "VTR-L29 Build/HUAWEIVTR-L29",
    "ELE-TL00 Build/HUAWEIELE-TL00",
    "ELE-L29 Build/HUAWEIELE-L29",
    "VOG-AL10 Build/HUAWEIVOG-AL10",
    "VOG-L29 Build/HUAWEIVOG-L29",
    "EVA-AL10 Build/HUAWEIEVA-AL10",
];
pub(crate) const HARMONYOS: [&str; 9] = [
    "HarmonyOS; DBR-W19; HMSCore 6.14.0.302",
    "HarmonyOS; MLD-AL00; HMSCore 6.13.0.352",
    "HarmonyOS; STG-AL00; HMSCore 6.14.0.302",
    "HarmonyOS; CTR-AL20; HMSCore 6.13.0.339",
    "HarmonyOS; FGD-AL00; HMSCore 6.13.0.342",
    "HarmonyOS; CTR-AL00; HMSCore 6.13.0.352",
    "HarmonyOS; ADA-AL00U; HMSCore 6.13.0.309",
    "HarmonyOS; CTR-AL00; HMSCore 6.14.0.302",
    "HarmonyOS; BRA-AL00; HMSCore 6.13.0.339",
];
pub(crate) const GALAXY: [&str; 24] = [
    "SAMSUNG SM-A105F Build/PPR1.180610.011",
    "SAMSUNG SM-A205F Build/PPR1.180610.011",
    "SAMSUNG SM-A305F Build/PPR1.180610.011",
    "SAMSUNG SM-A505F Build/PPR1.180610.011",
    "SAMSUNG SM-A536W Build/TP1A.220624.014",
    "SAMSUNG SM-A536B Build/TP1A.220624.014",
    "SAMSUNG SM-A536E Build/TP1A.220624.014",
    "SAMSUNG SM-A546E Build/TP1A.220624.014",
    "SAMSUNG SM-A546W Build/TP1A.220624.014",
    "SAMSUNG SM-A546B Build/TP1A.220624.014",
    "SAMSUNG SM-A556E Build/UP1A.231005.007",
    "SAMSUNG SM-A556W Build/UP1A.231005.007",
    "SAMSUNG SM-A556B Build/UP1A.231005.007",
    "SAMSUNG SM-W2021",
    "SAMSUNG SM-E346B",
    "SAMSUNG SM-E146B",
    "SAMSUNG SM-G925F",
    "SAMSUNG SM-M346B2",
    "SAMSUNG SM-M346B1",
    "SM-M536S Build/SP1A.210812.016",
    "SM-M536S Build/SP1A.210812.016",
    "SM-A826S Build/RP1A.200720.012",
    "SM-M536S Build/SP1A.210812.016",
    "SAMSUNG SM-E045F",
];
// iPhone; CPU iPhone OS 15_3 like Mac OS X
pub(crate) const IPHONE: [&str; 12] = [
    "15_1_1", "15_2", "15_3", "16_0", "14_2", "13_2_3", "13_4", "14_0", "14_4", "15_4_1", "15_3",
    "15_1",
];

pub(crate) const APPLE_WEB_KIT: [&str; 2] = [
    "AppleWebKit/537.36 (KHTML, like Gecko)",
    "AppleWebKit/605.1.15 (KHTML, like Gecko)",
];
pub(crate) const EDGE: [&str; 10] = [
    "Chrome/70.0.3538.102 Safari/537.36 Edge/18.19582",
    "Chrome/70.0.3538.102 Safari/537.36 Edge/18.19577",
    "Chrome/64.0.3282.140 Safari/537.36 Edge/18.17720",
    "Chrome/86.0.8810.3391 Safari/537.36 Edge/18.14383",
    "Edge/17.10859 Safari/452.6",
    "Chrome/51.0.2704.79 Safari/537.36 Edge/14.14931",
    "Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393",
    "Chrome/46.0.2486.0 Safari/537.36 Edge/13.9200",
    "Chrome/46.0.2486.0 Safari/537.36 Edge/13.10586",
    "Chrome/42.0.2311.135 Safari/537.36 Edge/12.246",
];
pub(crate) const OPERA: [&str; 9] = [
    "Presto/2.12.388 Version/12.16.2",
    "Presto/2.12.388 Version/12.16",
    "Presto/2.12.388 Version/12.14",
    "Gecko/20100101 Firefox/4.0 Opera 12.14",
    "Opera 12.14",
    "Presto/2.10.289 Version/12.02",
    "Presto/2.9.181 Version/12.00",
    "Presto/22.9.168 Version/12.00",
    "Gecko/20100101 Firefox/14.0 Opera/12.0",
];
pub(crate) const CHROME: [&str; 18] = [
    "104.0.5112.79",
    "104.0.0.0",
    "103.0.5060.53",
    "99.0.4844.84",
    "70.0.3538.77",
    "55.0.2919.83",
    "54.0.2866.71",
    "53.0.2820.59",
    "52.0.2762.73",
    "49.0.2656.18",
    "44.0.2403.155",
    "41.0.2228.0",
    "41.0.2227.1",
    "41.0.2227.0",
    "41.0.2226.0",
    "41.0.2225.0",
    "41.0.2224.3",
    "40.0.2214.93",
];
pub(crate) const FIREFOX: [&str; 12] = [
    "Gecko/20100101 Firefox/101.0",
    "Gecko/20100101 Firefox/99.0",
    "Gecko/20190101 Firefox/77.0",
    "Gecko/20100101 Firefox/77.0",
    "Gecko/20100101 Firefox/75.0",
    "Gecko/20100101 Firefox/74.0",
    "Gecko/20100101 Firefox/73.0",
    "Gecko/20100101 Firefox/72.0",
    "Gecko/20100101 Firefox/71.0",
    "Gecko/20191022 Firefox/70.0",
    "Gecko/20190101 Firefox/70.0",
    "Gecko/20100821 Firefox/70",
];
pub(crate) const SAFARI: [&str; 12] = [
    "Version/7.0.3 Safari/7046A194A",
    "Version/6.0 Mobile/10A5355d Safari/8536.25",
    "Version/5.1.7 Safari/534.57.2",
    "Version/5.1.3 Safari/534.53.10",
    "Version/5.1 Mobile/9B176 Safari/7534.48.3",
    "Version/5.0.5 Safari/533.21.1",
    "Version/5.0.4 Safari/533.20.27",
    "Version/5.0.3 Safari/533.19.4",
    "Version/5.0.2 Safari/533.18.5",
    "Version/5.0.1 Safari/533.17.8",
    "Version/5.0 Safari/531.2+",
    "Version/5.0 Safari/533.16",
];
