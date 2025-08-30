use crate::constants::*;

const fn rook_relevancy_masks() -> [u64; 64] {
    let mut m = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        m[sq] = clear_rim(sq, get_rays(sq));
        sq += 1;
    }
    m
}

const fn bishop_relevancy_masks() -> [u64; 64] {
    let mut m = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        m[sq] = clear_rim(sq, get_diagonals(sq));
        sq += 1;
    }
    m
}

pub fn compute_bishop_lookup() -> Vec<Vec<u64>> {
    let mut result: Vec<Vec<u64>> = Vec::new();
    for i in 0..64 {
        let rays = BISHOP_RELEVANCY_MASKS[i];

        let shift = BISHOP_SHIFTS[i];
        let size = 1usize << (64 - shift);
        let mut arr = vec![0u64; size];

        for j in 0..size {
            let occ = get_occ_from_index(rays, j);
            let bishop_attacks = get_bishop_attacks_from_occ(occ, i);

            let idx = index_magic(occ, BISHOP_MAGICS[i], BISHOP_SHIFTS[i]);
            arr[idx] = bishop_attacks;
        }
        result.push(arr);
    }
    result
}

pub fn compute_rook_lookup() -> Vec<Vec<u64>> {
    let mut result: Vec<Vec<u64>> = Vec::new();
    for i in 0..64 {
        let rays = ROOK_RELEVANCY_MASKS[i];

        let shift = ROOK_SHIFTS[i];
        let size = 1usize << (64 - shift);
        let mut arr = vec![0u64; size];

        for j in 0..size {
            let occ = get_occ_from_index(rays, j);
            let rook_attacks = get_rook_attacks_from_occ(occ, i);

            let idx = index_magic(occ, ROOK_MAGICS[i], ROOK_SHIFTS[i]);
            arr[idx] = rook_attacks;
        }
        result.push(arr);
    }
    result
}

pub fn index_magic(occ: u64, magic: u64, shift: usize) -> usize {
    (occ.wrapping_mul(magic)) as usize >> shift
}

fn get_bishop_attacks_from_occ(occ: u64, sq: usize) -> u64 {
    let mut result = 0u64;

    // NW
    let mut t_sq = sq;
    let mut ptr = 1u64 << sq;
    while t_sq % 8 != 0 && t_sq / 8 != 7 && ptr != 0 {
        t_sq += 7;
        ptr <<= 7;
        result |= ptr;
        ptr &= !occ;
    }

    // NE
    t_sq = sq;
    ptr = 1u64 << sq;
    while t_sq % 8 != 7  && t_sq / 8 != 7 && ptr != 0 {
        t_sq += 9;
        ptr <<= 9;
        result |= ptr;
        ptr &= !occ;
    }

    //SW 
    t_sq = sq;
    ptr = 1u64 << sq;
    while t_sq % 8 != 0 && t_sq / 8 != 0 && ptr != 0 {
        t_sq -= 9;
        ptr >>= 9;
        result |= ptr;
        ptr &= !occ;
    }

    // SE
    t_sq = sq;
    ptr = 1u64 << sq;
    while t_sq % 8 != 7 && t_sq / 8 != 0 && ptr != 0 {
        t_sq -= 7;
        ptr >>= 7;
        result |= ptr;
        ptr &= !occ;
    }
    result
}

fn get_rook_attacks_from_occ(occ: u64, sq: usize) -> u64 {
    let mut result = 0u64;

    let mut t_sq = sq;
    let mut ptr = 1u64 << sq;
    // left
    while t_sq % 8 != 0 && ptr != 0 {
        t_sq -= 1;
        ptr >>= 1;
        result |= ptr;
        ptr &= !occ;
    }

    // right
    t_sq = sq;
    ptr = 1u64 << sq;
    while t_sq % 8 != 7 && ptr != 0 {
        t_sq += 1;
        ptr <<= 1;
        result |= ptr;
        ptr &= !occ;
    }

    // top
    t_sq = sq;
    ptr = 1u64 << sq;
    while t_sq / 8 != 7 && ptr != 0 {
        t_sq += 8;
        ptr <<= 8;
        result |= ptr;
        ptr &= !occ;
    }

    // bottom
    t_sq = sq;
    ptr = 1u64 << sq;
    while t_sq / 8 != 0 && ptr != 0 {
        t_sq -= 8;
        ptr >>= 8;
        result |= ptr;
        ptr &= !occ;
    }
    result
}

fn get_occ_from_index(mut attack_map: u64, mut idx: usize) -> u64 {
    let mut idx_pointer = 1usize;
    let mut result = 0u64;
    if attack_map == 0 {
        return 0;
    }

    while idx != 0 && attack_map != 0 {
        assert!(attack_map.leading_zeros() < 63, "{}", attack_map.leading_zeros());
        let attack_pointer = 1u64 << (63 - attack_map.leading_zeros());
        attack_map &= !attack_pointer;
        if idx_pointer & idx != 0 {
            result |= attack_pointer;
            idx &= !idx_pointer;
        }
        idx_pointer <<= 1;
    }
    result
}

pub const fn clear_rim(sq: usize, mut bb: u64) -> u64 {
    let file = sq % 8;
    let rank = sq / 8;

    if file != 0 {
        bb &= !FILEA;
    }
    if file != 7 {
        bb &= !FILEH;
    }
    if rank != 0 {
        bb &= !RANK1;
    }
    if rank != 7 {
        bb &= !RANK8;
    }
    bb
}

const fn get_rays(sq: usize) -> u64 {
    let files: [u64; 8] = [FILEA, FILEB, FILEC, FILED, FILEE, FILEF, FILEG, FILEH];
    let ranks: [u64; 8]  = [RANK1, RANK2, RANK3, RANK4, RANK5, RANK6, RANK7, RANK8];

    let file = sq % 8;
    let rank = sq / 8;
    (files[file] | ranks[rank]) & !(1 << sq)
}

const fn get_diagonals(sq: usize) -> u64 {
    let mut result = 0u64;
    // NW
    let mut tsq = sq;
    while tsq % 8 != 0 && tsq / 8 != 7 {
        tsq += 7;
        result |= 1 << tsq;
    }
    // NE
    tsq = sq;
    while tsq % 8 != 7 && tsq / 8 != 7 {
        tsq += 9;
        result |= 1 << tsq;
    }
    // SW
    tsq = sq;
    while tsq % 8 != 0 && tsq / 8 != 0 {
        tsq -= 9;
        result |= 1 << tsq;
    }
    tsq = sq;
    while tsq % 8 != 7 && tsq / 8 != 0 {
        tsq -= 7;
        result |= 1 << tsq;
    }
    result
}

pub static BISHOP_RELEVANCY_MASKS: [u64; 64] = bishop_relevancy_masks();
pub static ROOK_RELEVANCY_MASKS: [u64; 64] = rook_relevancy_masks();

pub static ROOK_MAGICS: [u64; 64] = [
    7035315465041784025,
    10681852079867199381,
    9144433878675229191,
    12715829227078368949,
    3677275950918892973,
    15119714564125737037,
    15655010974651088058,
    1037017061092786140,
    13510659775649576503,
    2748294137189399693,
    13573532300355281005,
    16532230426036866460,
    6410588624937985655,
    9697685240690418141,
    6600569263284224506,
    9296630260047278016,
    15572520509954733135,
    724097128831143368,
    12737410542226846240,
    9166292649406850671,
    17827875984121574649,
    8162133750098703880,
    1634689354836784728,
    6967333781113829897,
    80833984154021187,
    15400503912424068873,
    7739093144475243635,
    6766277894501671208,
    1890285862163650299,
    4443664888360653912,
    9299460700912463670,
    9299460700912463670,
    9144433878675229191,
    3927757354765725995,
    11034288451168190801,
    17783837470889278794,
    13650880483342724769,
    2128812098201290397,
    16425513883796234081,
    15611391974590906893,
    6081541118440770148,
    6332531675034898414,
    17395623880672698369,
    8462271628134320029,
    8998873414143194392,
    12557269419879538198,
    12481718542061564246,
    2484178166730559452,
    349587381804722688,
    349587381804722688,
    6607528582023352998,
    6970392990774609584,
    16177742589595067844,
    13855553921007937727,
    17705390255989242880,
    15923201104149236264,
    2170322281395210590,
    2170322281395210590,
    7131866450072760830,
    3669241129923181398,
    8221344403219350278,
    1052304407931279594,
    2857467099844446724,
    6834427993197314114,
];

pub static ROOK_SHIFTS: [usize; 64] = [50, 51, 51, 51, 51, 51, 51, 50, 52, 53, 52, 52, 52, 52, 53, 52, 51, 53, 52, 52, 52, 52, 53, 52, 52, 53, 52, 52, 52, 52, 53, 52, 51, 53, 52, 52, 52, 52, 53, 52, 52, 53, 53, 52, 52, 53, 53, 52, 52, 53, 52, 53, 52, 52, 53, 52, 51, 52, 52, 52, 52, 51, 53, 52,];

pub static BISHOP_MAGICS: [u64; 64] = [
    8162133750098703880,
    12558445981157360152,
    11092405518535592955,
    7996177986613054998,
    9629270019050479025,
    4318398077988087100,
    15608920066098276120,
    13236432969697076320,
    14165862587537564038,
    13646377118369440120,
    10368500425185342560,
    16693920367555100621,
    260544694916547982,
    9032151582126397287,
    4893573604293218970,
    2613134948894779399,
    10673799171557826370,
    3774191654591401933,
    14107352781960469148,
    17488042259621663381,
    1437679679268416452,
    7219246880997377384,
    17078636710450110509,
    9512169642983173695,
    15390971263527704189,
    16018122290936415930,
    17516895317138358258,
    7321543603588731428,
    652878855908929711,
    16324949580526897656,
    6503826804753880497,
    3549688646903826432,
    13002054870707001367,
    17405823239083995257,
    1895248083825584055,
    12237671718365729700,
    10825231920229254154,
    7341914777554225862,
    13130820909658724870,
    16419037104801481731,
    6935842597145092270,
    5009889477388252044,
    16869427809440716315,
    5770505103128441973,
    2128812098201290397,
    17383146621527863132,
    14906927328604065958,
    10441942203642848614,
    15608920066098276120,
    11629984745249602192,
    7612535728549924286,
    15126064557536578518,
    5419444230551896938,
    14333639986832016068,
    3381803201400218016,
    12558445981157360152,
    13236432969697076320,
    2613134948894779399,
    16287714993460250666,
    13842742111574825472,
    10703648593032315909,
    15043715739471848206,
    14165862587537564038,
    8162133750098703880,
];

pub static BISHOP_SHIFTS: [usize; 64] = [57, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 56, 56, 56, 56, 59, 59, 59, 59, 56, 53, 53, 56, 59, 59, 59, 59, 56, 53, 53, 56, 59, 59, 59, 59, 56, 56, 56, 56, 59, 58, 59, 59, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 57,];
