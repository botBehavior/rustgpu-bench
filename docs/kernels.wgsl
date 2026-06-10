struct type_4 {
    member: array<u32>,
}

struct type_8 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_9 {
    member: type_8,
}

struct type_13 {
    member: array<f32>,
}

struct type_17 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_18 {
    member: f32,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
    member_4: u32,
    member_5: f32,
}

struct type_19 {
    member: vec3<f32>,
    member_1: f32,
    member_2: vec3<f32>,
    member_3: u32,
    member_4: f32,
}

struct type_21 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
    member_4: f32,
    member_5: f32,
    member_6: f32,
    member_7: f32,
    member_8: f32,
    member_9: f32,
}

struct type_22 {
    member: type_21,
}

struct type_24 {
    member: f32,
    member_1: f32,
    member_2: f32,
    member_3: f32,
}

struct type_26 {
    member: array<type_24>,
}

struct type_29 {
    member: u32,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<storage, read_write> global_1: type_4;
@group(0) @binding(0) 
var<storage> global_2: type_9;
@group(0) @binding(1) 
var<storage, read_write> global_3: type_13;
@group(0) @binding(0) 
var<storage> global_4: type_22;
@group(0) @binding(1) 
var<storage, read_write> global_5: type_26;
@group(0) @binding(2) 
var<storage, read_write> global_6: type_13;
@group(0) @binding(0) 
var<storage> global_7: type_29;
@group(0) @binding(1) 
var<storage> global_8: type_13;
@group(0) @binding(2) 
var<storage> global_9: type_13;
@group(0) @binding(3) 
var<storage, read_write> global_10: type_13;
@group(0) @binding(1) 
var<storage> global_11: type_13;
@group(0) @binding(2) 
var<storage, read_write> global_12: type_13;

fn function_() {
    var phi_1804_: u32;
    var phi_1805_: u32;
    var phi_1820_: u32;
    var phi_1825_: u32;
    var phi_1826_: bool;
    var phi_1831_: u32;
    var phi_1835_: u32;
    var phi_1836_: u32;
    var phi_1837_: bool;
    var phi_1838_: bool;
    var phi_1841_: u32;
    var phi_1842_: u32;
    var phi_1843_: bool;
    var phi_1844_: bool;
    var phi_1845_: bool;
    var local: bool;
    var local_1: bool;
    var local_2: bool;
    var local_3: bool;
    var local_4: u32;
    var phi_1867_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e91 = (_e87.x < arrayLength((&global_1.member)));
            if _e91 {
                if _e91 {
                } else {
                    break;
                }
                let _e94 = global_1.member[_e87.x];
                if (_e94 == 0u) {
                    phi_1867_ = 4294967295u;
                } else {
                    phi_1804_ = 0u;
                    phi_1805_ = _e94;
                    loop {
                        let _e97 = phi_1804_;
                        let _e99 = phi_1805_;
                        local_4 = _e97;
                        let _e100 = (_e99 == 1u);
                        local = _e100;
                        local_2 = _e100;
                        if _e100 {
                            phi_1841_ = u32();
                            phi_1842_ = u32();
                            phi_1843_ = false;
                            phi_1844_ = false;
                            phi_1845_ = false;
                        } else {
                            let _e101 = (_e97 >= 1000u);
                            if _e101 {
                                phi_1835_ = u32();
                                phi_1836_ = u32();
                                phi_1837_ = false;
                                phi_1838_ = false;
                            } else {
                                if ((_e99 % 2u) == 0u) {
                                    phi_1825_ = (_e99 / 2u);
                                    phi_1826_ = true;
                                } else {
                                    let _e104 = (_e99 > 1431655764u);
                                    if _e104 {
                                        phi_1820_ = u32();
                                    } else {
                                        phi_1820_ = ((3u * _e99) + 1u);
                                    }
                                    let _e108 = phi_1820_;
                                    phi_1825_ = _e108;
                                    phi_1826_ = select(true, false, _e104);
                                }
                                let _e112 = phi_1825_;
                                let _e114 = phi_1826_;
                                if _e114 {
                                    phi_1831_ = (_e97 + 1u);
                                } else {
                                    phi_1831_ = u32();
                                }
                                let _e117 = phi_1831_;
                                phi_1835_ = _e117;
                                phi_1836_ = _e112;
                                phi_1837_ = _e114;
                                phi_1838_ = select(true, false, _e114);
                            }
                            let _e120 = phi_1835_;
                            let _e122 = phi_1836_;
                            let _e124 = phi_1837_;
                            let _e126 = phi_1838_;
                            phi_1841_ = _e120;
                            phi_1842_ = _e122;
                            phi_1843_ = _e124;
                            phi_1844_ = _e101;
                            phi_1845_ = _e126;
                        }
                        let _e128 = phi_1841_;
                        let _e130 = phi_1842_;
                        let _e132 = phi_1843_;
                        let _e134 = phi_1844_;
                        let _e136 = phi_1845_;
                        local_1 = _e134;
                        local_3 = _e136;
                        continue;
                        continuing {
                            phi_1804_ = _e128;
                            phi_1805_ = _e130;
                            break if !(_e132);
                        }
                    }
                    let _e139 = local;
                    let _e141 = local_1;
                    let _e142 = select(_e141, false, _e139);
                    let _e144 = local_2;
                    let _e146 = local_3;
                    let _e151 = local_4;
                    phi_1867_ = select(_e151, 4294967295u, select(_e142, true, select(select(_e146, false, _e144), false, _e142)));
                }
                let _e154 = phi_1867_;
                if _e91 {
                } else {
                    break;
                }
                global_1.member[_e87.x] = _e154;
            }
            break;
        }
    }
    return;
}

fn function_1() {
    var phi_290_: vec3<f32>;
    var phi_293_: u32;
    var phi_1963_: vec3<f32>;
    var phi_1964_: u32;
    var phi_1965_: u32;
    var phi_1966_: vec3<f32>;
    var phi_1967_: vec3<f32>;
    var phi_2147_: u32;
    var phi_2148_: type_18;
    var phi_2279_: type_19;
    var phi_2209_: f32;
    var phi_2248_: type_18;
    var phi_2250_: type_18;
    var phi_2252_: type_18;
    var phi_2255_: u32;
    var phi_2256_: type_18;
    var local_5: type_18;
    var local_6: type_18;
    var phi_1995_: vec3<f32>;
    var local_7: type_18;
    var local_8: type_18;
    var local_9: type_18;
    var phi_2379_: bool;
    var local_10: type_18;
    var phi_2460_: bool;
    var phi_2084_: vec3<f32>;
    var phi_2085_: u32;
    var phi_2086_: bool;
    var phi_2091_: u32;
    var phi_2095_: vec3<f32>;
    var phi_2096_: u32;
    var phi_2097_: u32;
    var phi_2098_: vec3<f32>;
    var phi_2099_: vec3<f32>;
    var phi_2100_: u32;
    var phi_2101_: bool;
    var phi_2102_: bool;
    var phi_2104_: vec3<f32>;
    var phi_2105_: u32;
    var phi_2106_: u32;
    var phi_2107_: vec3<f32>;
    var phi_2108_: vec3<f32>;
    var phi_2109_: u32;
    var phi_2110_: bool;
    var phi_2111_: bool;
    var phi_2113_: bool;
    var local_11: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var local_15: vec3<f32>;
    var local_16: u32;
    var phi_2122_: type_17;
    var local_17: bool;
    var local_18: bool;
    var local_19: bool;
    var local_20: bool;
    var local_21: u32;
    var phi_2129_: type_17;
    var local_22: bool;
    var local_23: u32;
    var phi_2136_: type_17;
    var phi_2141_: type_17;
    var phi_291_: vec3<f32>;
    var phi_294_: u32;
    var local_24: u32;
    var local_25: vec3<f32>;
    var local_26: vec3<f32>;
    var local_27: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e89 = arrayLength((&global_3.member));
            let _e94 = global_2.member.member;
            if (_e87.x >= _e94) {
            } else {
                let _e98 = global_2.member.member_1;
                if (_e87.y >= _e98) {
                } else {
                    let _e100 = f32(_e94);
                    let _e101 = f32(_e98);
                    let _e105 = ((_e87.y * _e94) + _e87.x);
                    phi_290_ = vec3<f32>(0f, 0f, 0f);
                    phi_293_ = 0u;
                    loop {
                        let _e107 = phi_290_;
                        let _e109 = phi_293_;
                        local_25 = _e107;
                        local_26 = _e107;
                        local_27 = _e107;
                        let _e112 = global_2.member.member_2;
                        let _e113 = (_e109 < _e112);
                        local_24 = _e112;
                        if _e113 {
                            let _e116 = global_2.member.member_3;
                            let _e121 = (((_e116 ^ 61u) ^ (_e116 >> bitcast<u32>(16i))) * 9u);
                            let _e125 = ((_e121 ^ (_e121 >> bitcast<u32>(4i))) * 668265261u);
                            let _e129 = (_e109 ^ (_e125 ^ (_e125 >> bitcast<u32>(15i))));
                            let _e134 = (((_e129 ^ 61u) ^ (_e129 >> bitcast<u32>(16i))) * 9u);
                            let _e138 = ((_e134 ^ (_e134 >> bitcast<u32>(4i))) * 668265261u);
                            let _e142 = (_e105 ^ (_e138 ^ (_e138 >> bitcast<u32>(15i))));
                            let _e147 = (((_e142 ^ 61u) ^ (_e142 >> bitcast<u32>(16i))) * 9u);
                            let _e151 = ((_e147 ^ (_e147 >> bitcast<u32>(4i))) * 668265261u);
                            let _e154 = (_e151 ^ (_e151 >> bitcast<u32>(15i)));
                            let _e156 = select(_e154, 2654435769u, (_e154 == 0u));
                            let _e159 = (_e156 ^ (_e156 << bitcast<u32>(13i)));
                            let _e162 = (_e159 ^ (_e159 >> bitcast<u32>(17i)));
                            let _e165 = (_e162 ^ (_e162 << bitcast<u32>(5i)));
                            let _e172 = (_e165 ^ (_e165 << bitcast<u32>(13i)));
                            let _e175 = (_e172 ^ (_e172 >> bitcast<u32>(17i)));
                            let _e178 = (_e175 ^ (_e175 << bitcast<u32>(5i)));
                            phi_1963_ = vec3<f32>(1f, 1f, 1f);
                            phi_1964_ = 0u;
                            phi_1965_ = _e178;
                            phi_1966_ = vec3<f32>(((((f32(_e87.x) + (f32((_e165 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e100) - 0.5f) * (2f * (_e100 / _e101))), ((0.5f - ((f32(_e87.y) + (f32((_e178 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e101)) * 2f), -1f);
                            phi_1967_ = vec3<f32>(0f, 0f, 0f);
                            loop {
                                let _e195 = phi_1963_;
                                let _e197 = phi_1964_;
                                let _e199 = phi_1965_;
                                let _e201 = phi_1966_;
                                let _e203 = phi_1967_;
                                local_12 = _e201;
                                local_13 = _e201;
                                local_14 = _e201;
                                local_15 = _e195;
                                local_16 = _e199;
                                local_21 = _e199;
                                let _e204 = (_e197 < 8u);
                                if _e204 {
                                    phi_2147_ = 0u;
                                    phi_2148_ = type_18(1000000000000000000000000000000f, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0u, 0f);
                                    loop {
                                        let _e206 = phi_2147_;
                                        let _e208 = phi_2148_;
                                        local_5 = _e208;
                                        local_6 = _e208;
                                        local_7 = _e208;
                                        local_8 = _e208;
                                        local_9 = _e208;
                                        local_10 = _e208;
                                        let _e209 = (_e206 < 4u);
                                        if _e209 {
                                            switch bitcast<i32>(_e206) {
                                                case 0: {
                                                    phi_2279_ = type_19(vec3<f32>(0f, -100.5f, -1f), 100f, vec3<f32>(0.8f, 0.8f, 0f), 0u, 0f);
                                                    break;
                                                }
                                                case 1: {
                                                    phi_2279_ = type_19(vec3<f32>(0f, 0f, -1.2f), 0.5f, vec3<f32>(0.1f, 0.2f, 0.5f), 0u, 0f);
                                                    break;
                                                }
                                                case 2: {
                                                    phi_2279_ = type_19(vec3<f32>(-1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.8f, 0.8f), 1u, 0.05f);
                                                    break;
                                                }
                                                default: {
                                                    phi_2279_ = type_19(vec3<f32>(1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.6f, 0.2f), 1u, 0.4f);
                                                    break;
                                                }
                                            }
                                            let _e212 = phi_2279_;
                                            let _e216 = (_e203.x - _e212.member.x);
                                            let _e219 = (_e203.y - _e212.member.y);
                                            let _e222 = (_e203.z - _e212.member.z);
                                            let _e230 = (((_e201.x * _e201.x) + (_e201.y * _e201.y)) + (_e201.z * _e201.z));
                                            let _e235 = (((_e216 * _e201.x) + (_e219 * _e201.y)) + (_e222 * _e201.z));
                                            let _e246 = ((_e235 * _e235) - (_e230 * ((((_e216 * _e216) + (_e219 * _e219)) + (_e222 * _e222)) - (_e212.member_1 * _e212.member_1))));
                                            if (_e246 > 0f) {
                                                let _e248 = sqrt(_e246);
                                                let _e249 = -(_e235);
                                                let _e251 = ((_e249 - _e248) / _e230);
                                                if (_e251 < 0.001f) {
                                                    phi_2209_ = ((_e249 + _e248) / _e230);
                                                } else {
                                                    phi_2209_ = _e251;
                                                }
                                                let _e256 = phi_2209_;
                                                if (_e256 >= 0.001f) {
                                                    if (_e256 < _e208.member) {
                                                        let _e260 = (_e256 * _e201.x);
                                                        let _e261 = (_e256 * _e201.y);
                                                        let _e262 = (_e256 * _e201.z);
                                                        phi_2248_ = type_18(_e256, (_e203 + vec3<f32>(_e260, _e261, _e262)), vec3<f32>((((_e203.x + _e260) - _e212.member.x) / _e212.member_1), (((_e203.y + _e261) - _e212.member.y) / _e212.member_1), (((_e203.z + _e262) - _e212.member.z) / _e212.member_1)), _e212.member_2, _e212.member_3, _e212.member_4);
                                                    } else {
                                                        phi_2248_ = _e208;
                                                    }
                                                    let _e280 = phi_2248_;
                                                    phi_2250_ = _e280;
                                                } else {
                                                    phi_2250_ = _e208;
                                                }
                                                let _e282 = phi_2250_;
                                                phi_2252_ = _e282;
                                            } else {
                                                phi_2252_ = _e208;
                                            }
                                            let _e284 = phi_2252_;
                                            phi_2255_ = (_e206 + 1u);
                                            phi_2256_ = _e284;
                                        } else {
                                            phi_2255_ = u32();
                                            phi_2256_ = type_18();
                                        }
                                        let _e287 = phi_2255_;
                                        let _e289 = phi_2256_;
                                        continue;
                                        continuing {
                                            phi_2147_ = _e287;
                                            phi_2148_ = _e289;
                                            break if !(_e209);
                                        }
                                    }
                                    let _e292 = local_5;
                                    let _e294 = (_e292.member >= 1000000000000000000000000000000f);
                                    if _e294 {
                                        phi_2095_ = vec3<f32>();
                                        phi_2096_ = u32();
                                        phi_2097_ = u32();
                                        phi_2098_ = vec3<f32>();
                                        phi_2099_ = vec3<f32>();
                                        phi_2100_ = u32();
                                        phi_2101_ = false;
                                        phi_2102_ = false;
                                    } else {
                                        let _e296 = local_6;
                                        if ((((_e201.x * _e296.member_2.x) + (_e201.y * _e296.member_2.y)) + (_e201.z * _e296.member_2.z)) < 0f) {
                                            phi_1995_ = _e296.member_2;
                                        } else {
                                            phi_1995_ = -(_e296.member_2);
                                        }
                                        let _e312 = phi_1995_;
                                        let _e314 = local_7;
                                        let _e318 = local_8;
                                        let _e321 = local_9;
                                        let _e326 = (_e199 ^ (_e199 << bitcast<u32>(13i)));
                                        let _e329 = (_e326 ^ (_e326 >> bitcast<u32>(17i)));
                                        let _e332 = (_e329 ^ (_e329 << bitcast<u32>(5i)));
                                        let _e335 = (_e332 ^ (_e332 << bitcast<u32>(13i)));
                                        let _e338 = (_e335 ^ (_e335 >> bitcast<u32>(17i)));
                                        let _e341 = (_e338 ^ (_e338 << bitcast<u32>(5i)));
                                        if (_e321.member_4 == 0u) {
                                            let _e416 = (1f - (f32((_e332 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e417 = (f32((_e341 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e419 = (1f - (_e416 * _e416));
                                            if (_e419 != _e419) {
                                                phi_2460_ = true;
                                            } else {
                                                phi_2460_ = (0f >= _e419);
                                            }
                                            let _e423 = phi_2460_;
                                            let _e425 = sqrt(select(_e419, 0f, _e423));
                                            let _e427 = (_e425 * cos(_e417));
                                            let _e429 = (_e425 * sin(_e417));
                                            let _e432 = (_e312.x + _e427);
                                            let _e434 = (_e312.y + _e429);
                                            let _e436 = (_e312.z + _e416);
                                            phi_2084_ = select((_e312 + vec3<f32>(_e427, _e429, _e416)), _e312, vec3(((((_e432 * _e432) + (_e434 * _e434)) + (_e436 * _e436)) < 0.00000001f)));
                                            phi_2085_ = u32();
                                            phi_2086_ = true;
                                        } else {
                                            let _e349 = (_e201 * (1f / sqrt((((_e201.x * _e201.x) + (_e201.y * _e201.y)) + (_e201.z * _e201.z)))));
                                            let _e361 = (2f * (((_e349.x * _e312.x) + (_e349.y * _e312.y)) + (_e349.z * _e312.z)));
                                            let _e366 = (_e349 - vec3<f32>((_e361 * _e312.x), (_e361 * _e312.y), (_e361 * _e312.z)));
                                            let _e374 = (1f - (f32((_e332 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e375 = (f32((_e341 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e377 = (1f - (_e374 * _e374));
                                            if (_e377 != _e377) {
                                                phi_2379_ = true;
                                            } else {
                                                phi_2379_ = (0f >= _e377);
                                            }
                                            let _e381 = phi_2379_;
                                            let _e383 = sqrt(select(_e377, 0f, _e381));
                                            let _e389 = local_10;
                                            let _e391 = (_e389.member_5 * (_e383 * cos(_e375)));
                                            let _e392 = (_e389.member_5 * (_e383 * sin(_e375)));
                                            let _e393 = (_e389.member_5 * _e374);
                                            phi_2084_ = (_e366 + vec3<f32>(_e391, _e392, _e393));
                                            phi_2085_ = _e341;
                                            phi_2086_ = select(true, false, (((((_e366.x + _e391) * _e312.x) + ((_e366.y + _e392) * _e312.y)) + ((_e366.z + _e393) * _e312.z)) <= 0f));
                                        }
                                        let _e447 = phi_2084_;
                                        let _e449 = phi_2085_;
                                        let _e451 = phi_2086_;
                                        if _e451 {
                                            phi_2091_ = (_e197 + 1u);
                                        } else {
                                            phi_2091_ = u32();
                                        }
                                        let _e454 = phi_2091_;
                                        phi_2095_ = (_e195 * _e314.member_3);
                                        phi_2096_ = _e454;
                                        phi_2097_ = _e341;
                                        phi_2098_ = _e447;
                                        phi_2099_ = _e318.member_1;
                                        phi_2100_ = _e449;
                                        phi_2101_ = _e451;
                                        phi_2102_ = select(true, false, _e451);
                                    }
                                    let _e457 = phi_2095_;
                                    let _e459 = phi_2096_;
                                    let _e461 = phi_2097_;
                                    let _e463 = phi_2098_;
                                    let _e465 = phi_2099_;
                                    let _e467 = phi_2100_;
                                    let _e469 = phi_2101_;
                                    let _e471 = phi_2102_;
                                    phi_2104_ = _e457;
                                    phi_2105_ = _e459;
                                    phi_2106_ = _e461;
                                    phi_2107_ = _e463;
                                    phi_2108_ = _e465;
                                    phi_2109_ = _e467;
                                    phi_2110_ = _e469;
                                    phi_2111_ = _e294;
                                    phi_2113_ = _e471;
                                } else {
                                    phi_2104_ = vec3<f32>();
                                    phi_2105_ = u32();
                                    phi_2106_ = u32();
                                    phi_2107_ = vec3<f32>();
                                    phi_2108_ = vec3<f32>();
                                    phi_2109_ = u32();
                                    phi_2110_ = false;
                                    phi_2111_ = false;
                                    phi_2113_ = false;
                                }
                                let _e473 = phi_2104_;
                                let _e475 = phi_2105_;
                                let _e477 = phi_2106_;
                                let _e479 = phi_2107_;
                                let _e481 = phi_2108_;
                                let _e483 = phi_2109_;
                                let _e485 = phi_2110_;
                                let _e487 = phi_2111_;
                                let _e489 = phi_2113_;
                                local_11 = _e487;
                                local_17 = _e487;
                                local_18 = select(true, false, _e204);
                                local_19 = _e487;
                                local_20 = _e489;
                                local_22 = _e487;
                                local_23 = _e483;
                                continue;
                                continuing {
                                    phi_1963_ = _e473;
                                    phi_1964_ = _e475;
                                    phi_1965_ = _e477;
                                    phi_1966_ = _e479;
                                    phi_1967_ = _e481;
                                    break if !(_e485);
                                }
                            }
                            let _e493 = local_11;
                            if _e493 {
                                let _e495 = local_12;
                                let _e499 = local_13;
                                let _e504 = local_14;
                                let _e511 = ((_e499.y * (1f / sqrt((((_e495.x * _e495.x) + (_e499.y * _e499.y)) + (_e504.z * _e504.z))))) + 1f);
                                let _e513 = (1f - (0.5f * _e511));
                                let _e520 = local_15;
                                let _e523 = local_16;
                                phi_2122_ = type_17(_e523, (_e520 * vec3<f32>((_e513 + (_e511 * 0.25f)), (_e513 + (_e511 * 0.35f)), 1f)));
                            } else {
                                phi_2122_ = type_17();
                            }
                            let _e526 = phi_2122_;
                            let _e528 = local_17;
                            let _e530 = local_18;
                            let _e531 = select(_e530, false, _e528);
                            let _e533 = local_19;
                            let _e535 = local_20;
                            if _e531 {
                                let _e538 = local_21;
                                phi_2129_ = type_17(_e538, vec3<f32>(0f, 0f, 0f));
                            } else {
                                phi_2129_ = type_17();
                            }
                            let _e541 = phi_2129_;
                            let _e542 = select(select(_e535, false, _e533), false, _e531);
                            let _e544 = local_22;
                            if _e542 {
                                let _e547 = local_23;
                                phi_2136_ = type_17(_e547, vec3<f32>(0f, 0f, 0f));
                            } else {
                                phi_2136_ = _e526;
                            }
                            let _e550 = phi_2136_;
                            if select(select(_e544, false, _e531), true, _e542) {
                                phi_2141_ = _e550;
                            } else {
                                phi_2141_ = _e541;
                            }
                            let _e553 = phi_2141_;
                            phi_291_ = (_e107 + _e553.member_1);
                            phi_294_ = (_e109 + 1u);
                        } else {
                            phi_291_ = vec3<f32>();
                            phi_294_ = u32();
                        }
                        let _e558 = phi_291_;
                        let _e560 = phi_294_;
                        continue;
                        continuing {
                            phi_290_ = _e558;
                            phi_293_ = _e560;
                            break if !(_e113);
                        }
                    }
                    let _e563 = local_24;
                    let _e564 = f32(_e563);
                    let _e566 = local_25;
                    let _e570 = local_26;
                    let _e574 = local_27;
                    let _e577 = (_e105 * 3u);
                    if (_e577 < _e89) {
                    } else {
                        break;
                    }
                    global_3.member[_e577] = (_e566.x / _e564);
                    let _e581 = (_e577 + 1u);
                    if (_e581 < _e89) {
                    } else {
                        break;
                    }
                    global_3.member[_e581] = (_e570.y / _e564);
                    let _e585 = (_e577 + 2u);
                    if (_e585 < _e89) {
                    } else {
                        break;
                    }
                    global_3.member[_e585] = (_e574.z / _e564);
                }
            }
            break;
        }
    }
    return;
}

fn function_2() {
    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e93 = global_4.member.member_2;
            if (_e87.x < _e93) {
                let _e97 = global_4.member.member_3;
                let _e102 = (((_e97 ^ 61u) ^ (_e97 >> bitcast<u32>(16i))) * 9u);
                let _e106 = ((_e102 ^ (_e102 >> bitcast<u32>(4i))) * 668265261u);
                let _e110 = (12648430u ^ (_e106 ^ (_e106 >> bitcast<u32>(15i))));
                let _e115 = (((_e110 ^ 61u) ^ (_e110 >> bitcast<u32>(16i))) * 9u);
                let _e119 = ((_e115 ^ (_e115 >> bitcast<u32>(4i))) * 668265261u);
                let _e123 = (_e87.x ^ (_e119 ^ (_e119 >> bitcast<u32>(15i))));
                let _e128 = (((_e123 ^ 61u) ^ (_e123 >> bitcast<u32>(16i))) * 9u);
                let _e132 = ((_e128 ^ (_e128 >> bitcast<u32>(4i))) * 668265261u);
                let _e135 = (_e132 ^ (_e132 >> bitcast<u32>(15i)));
                let _e137 = select(_e135, 2654435769u, (_e135 == 0u));
                let _e140 = (_e137 ^ (_e137 << bitcast<u32>(13i)));
                let _e143 = (_e140 ^ (_e140 >> bitcast<u32>(17i)));
                let _e146 = (_e143 ^ (_e143 << bitcast<u32>(5i)));
                let _e150 = (f32((_e146 >> bitcast<u32>(8i))) * 0.00000037450704f);
                let _e153 = global_4.member.member_1;
                let _e154 = f32(_e153);
                let _e158 = (_e146 ^ (_e146 << bitcast<u32>(13i)));
                let _e161 = (_e158 ^ (_e158 >> bitcast<u32>(17i)));
                let _e170 = ((0.35f * _e154) * (0.6f + (f32(((_e161 ^ (_e161 << bitcast<u32>(5i))) >> bitcast<u32>(8i))) * 0.000000023841858f)));
                let _e173 = global_4.member.member;
                if (_e87.x < arrayLength((&global_5.member))) {
                } else {
                    break;
                }
                global_5.member[_e87.x] = type_24(((f32(_e173) * 0.5f) + (cos(_e150) * _e170)), ((_e154 * 0.5f) + (sin(_e150) * _e170)), (_e150 + 3.1415927f), 0f);
            }
            break;
        }
    }
    return;
}

fn function_3() {
    var phi_2581_: f32;
    var phi_2585_: f32;
    var phi_2595_: f32;
    var phi_2599_: f32;
    var phi_2609_: f32;
    var phi_2613_: f32;
    var phi_2623_: f32;
    var phi_2627_: f32;
    var phi_2637_: f32;
    var phi_2641_: f32;
    var phi_2651_: f32;
    var phi_2655_: f32;
    var phi_1114_: bool;
    var phi_1147_: f32;
    var phi_1148_: f32;
    var phi_1149_: f32;
    var phi_1150_: f32;
    var phi_2719_: f32;
    var phi_2723_: f32;
    var phi_2733_: f32;
    var phi_2737_: f32;
    var phi_2747_: f32;
    var phi_2751_: f32;
    var phi_2761_: f32;
    var phi_2765_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e91 = arrayLength((&global_6.member));
            let _e95 = global_4.member.member_2;
            if (_e87.x >= _e95) {
            } else {
                let _e97 = (_e87.x < arrayLength((&global_5.member)));
                if _e97 {
                } else {
                    break;
                }
                let _e102 = global_4.member.member_6;
                let _e106 = global_5.member[_e87.x].member_2;
                let _e107 = (_e106 + _e102);
                let _e111 = global_5.member[_e87.x].member;
                let _e115 = global_4.member.member_7;
                let _e117 = (_e111 + (cos(_e107) * _e115));
                let _e121 = global_5.member[_e87.x].member_1;
                let _e124 = (_e121 + (sin(_e107) * _e115));
                let _e127 = global_4.member.member;
                let _e128 = f32(_e127);
                if (_e117 < 0f) {
                    phi_2585_ = (_e117 + _e128);
                } else {
                    if (_e117 >= _e128) {
                        phi_2581_ = (_e117 - _e128);
                    } else {
                        phi_2581_ = _e117;
                    }
                    let _e133 = phi_2581_;
                    phi_2585_ = _e133;
                }
                let _e136 = phi_2585_;
                let _e142 = (_e127 == 0u);
                if _e142 {
                    break;
                }
                let _e146 = global_4.member.member_1;
                let _e147 = f32(_e146);
                if (_e124 < 0f) {
                    phi_2599_ = (_e124 + _e147);
                } else {
                    if (_e124 >= _e147) {
                        phi_2595_ = (_e124 - _e147);
                    } else {
                        phi_2595_ = _e124;
                    }
                    let _e152 = phi_2595_;
                    phi_2599_ = _e152;
                }
                let _e155 = phi_2599_;
                let _e161 = (_e146 == 0u);
                if _e161 {
                    break;
                }
                let _e164 = (((select(select(u32(_e155), 0u, (_e155 < 0f)), 4294967295u, (_e155 > 4294967000f)) % _e146) * _e127) + (select(select(u32(_e136), 0u, (_e136 < 0f)), 4294967295u, (_e136 > 4294967000f)) % _e127));
                if (_e164 < _e91) {
                } else {
                    break;
                }
                let _e168 = global_6.member[_e164];
                let _e169 = global_5.member[_e87.x].member_2;
                let _e170 = global_5.member[_e87.x].member;
                let _e173 = (_e170 + (cos(_e169) * _e115));
                let _e174 = global_5.member[_e87.x].member_1;
                let _e177 = (_e174 + (sin(_e169) * _e115));
                if (_e173 < 0f) {
                    phi_2613_ = (_e173 + _e128);
                } else {
                    if (_e173 >= _e128) {
                        phi_2609_ = (_e173 - _e128);
                    } else {
                        phi_2609_ = _e173;
                    }
                    let _e182 = phi_2609_;
                    phi_2613_ = _e182;
                }
                let _e185 = phi_2613_;
                if _e142 {
                    break;
                }
                if (_e177 < 0f) {
                    phi_2627_ = (_e177 + _e147);
                } else {
                    if (_e177 >= _e147) {
                        phi_2623_ = (_e177 - _e147);
                    } else {
                        phi_2623_ = _e177;
                    }
                    let _e196 = phi_2623_;
                    phi_2627_ = _e196;
                }
                let _e199 = phi_2627_;
                if _e161 {
                    break;
                }
                let _e207 = (((select(select(u32(_e199), 0u, (_e199 < 0f)), 4294967295u, (_e199 > 4294967000f)) % _e146) * _e127) + (select(select(u32(_e185), 0u, (_e185 < 0f)), 4294967295u, (_e185 > 4294967000f)) % _e127));
                if (_e207 < _e91) {
                } else {
                    break;
                }
                let _e211 = global_6.member[_e207];
                let _e213 = global_5.member[_e87.x].member_2;
                let _e214 = (_e213 + -(_e102));
                let _e215 = global_5.member[_e87.x].member;
                let _e218 = (_e215 + (cos(_e214) * _e115));
                let _e219 = global_5.member[_e87.x].member_1;
                let _e222 = (_e219 + (sin(_e214) * _e115));
                if (_e218 < 0f) {
                    phi_2641_ = (_e218 + _e128);
                } else {
                    if (_e218 >= _e128) {
                        phi_2637_ = (_e218 - _e128);
                    } else {
                        phi_2637_ = _e218;
                    }
                    let _e227 = phi_2637_;
                    phi_2641_ = _e227;
                }
                let _e230 = phi_2641_;
                if _e142 {
                    break;
                }
                if (_e222 < 0f) {
                    phi_2655_ = (_e222 + _e147);
                } else {
                    if (_e222 >= _e147) {
                        phi_2651_ = (_e222 - _e147);
                    } else {
                        phi_2651_ = _e222;
                    }
                    let _e241 = phi_2651_;
                    phi_2655_ = _e241;
                }
                let _e244 = phi_2655_;
                if _e161 {
                    break;
                }
                let _e252 = (((select(select(u32(_e244), 0u, (_e244 < 0f)), 4294967295u, (_e244 > 4294967000f)) % _e146) * _e127) + (select(select(u32(_e230), 0u, (_e230 < 0f)), 4294967295u, (_e230 > 4294967000f)) % _e127));
                if (_e252 < _e91) {
                } else {
                    break;
                }
                let _e256 = global_6.member[_e252];
                let _e257 = global_5.member[_e87.x].member_2;
                if (_e211 >= _e168) {
                    phi_1114_ = select(true, false, (_e211 >= _e256));
                } else {
                    phi_1114_ = true;
                }
                let _e262 = phi_1114_;
                if _e262 {
                    if (_e168 > _e256) {
                        let _e325 = global_4.member.member_5;
                        phi_1149_ = (_e257 + _e325);
                    } else {
                        if (_e256 > _e168) {
                            let _e319 = global_4.member.member_5;
                            phi_1148_ = (_e257 - _e319);
                        } else {
                            let _e267 = global_4.member.member_3;
                            let _e268 = (_e267 ^ 935473873u);
                            let _e273 = (((_e268 ^ 61u) ^ (_e268 >> bitcast<u32>(16i))) * 9u);
                            let _e277 = ((_e273 ^ (_e273 >> bitcast<u32>(4i))) * 668265261u);
                            let _e281 = (_e87.x ^ (_e277 ^ (_e277 >> bitcast<u32>(15i))));
                            let _e286 = (((_e281 ^ 61u) ^ (_e281 >> bitcast<u32>(16i))) * 9u);
                            let _e290 = ((_e286 ^ (_e286 >> bitcast<u32>(4i))) * 668265261u);
                            let _e293 = (_e290 ^ (_e290 >> bitcast<u32>(15i)));
                            let _e295 = select(_e293, 2654435769u, (_e293 == 0u));
                            let _e298 = (_e295 ^ (_e295 << bitcast<u32>(13i)));
                            let _e301 = (_e298 ^ (_e298 >> bitcast<u32>(17i)));
                            if (((_e301 ^ (_e301 << bitcast<u32>(5i))) & 1u) == 0u) {
                                let _e313 = global_4.member.member_5;
                                phi_1147_ = (_e257 + _e313);
                            } else {
                                let _e309 = global_4.member.member_5;
                                phi_1147_ = (_e257 - _e309);
                            }
                            let _e316 = phi_1147_;
                            phi_1148_ = _e316;
                        }
                        let _e322 = phi_1148_;
                        phi_1149_ = _e322;
                    }
                    let _e328 = phi_1149_;
                    phi_1150_ = _e328;
                } else {
                    phi_1150_ = _e257;
                }
                let _e330 = phi_1150_;
                let _e331 = global_5.member[_e87.x].member;
                let _e335 = global_4.member.member_4;
                let _e337 = (_e331 + (cos(_e330) * _e335));
                if (_e337 < 0f) {
                    phi_2723_ = (_e337 + _e128);
                } else {
                    if (_e337 >= _e128) {
                        phi_2719_ = (_e337 - _e128);
                    } else {
                        phi_2719_ = _e337;
                    }
                    let _e342 = phi_2719_;
                    phi_2723_ = _e342;
                }
                let _e345 = phi_2723_;
                let _e346 = global_5.member[_e87.x].member_1;
                let _e349 = (_e346 + (sin(_e330) * _e335));
                if (_e349 < 0f) {
                    phi_2737_ = (_e349 + _e147);
                } else {
                    if (_e349 >= _e147) {
                        phi_2733_ = (_e349 - _e147);
                    } else {
                        phi_2733_ = _e349;
                    }
                    let _e354 = phi_2733_;
                    phi_2737_ = _e354;
                }
                let _e357 = phi_2737_;
                if (_e345 < 0f) {
                    phi_2751_ = (_e345 + _e128);
                } else {
                    if (_e345 >= _e128) {
                        phi_2747_ = (_e345 - _e128);
                    } else {
                        phi_2747_ = _e345;
                    }
                    let _e363 = phi_2747_;
                    phi_2751_ = _e363;
                }
                let _e366 = phi_2751_;
                if _e142 {
                    break;
                }
                if (_e357 < 0f) {
                    phi_2765_ = (_e357 + _e147);
                } else {
                    if (_e357 >= _e147) {
                        phi_2761_ = (_e357 - _e147);
                    } else {
                        phi_2761_ = _e357;
                    }
                    let _e377 = phi_2761_;
                    phi_2765_ = _e377;
                }
                let _e380 = phi_2765_;
                if _e161 {
                    break;
                }
                let _e388 = (((select(select(u32(_e380), 0u, (_e380 < 0f)), 4294967295u, (_e380 > 4294967000f)) % _e146) * _e127) + (select(select(u32(_e366), 0u, (_e366 < 0f)), 4294967295u, (_e366 > 4294967000f)) % _e127));
                if _e97 {
                } else {
                    break;
                }
                global_5.member[_e87.x] = type_24(_e345, _e357, _e330, 0f);
                let _e391 = global_4.member.member_8;
                if (_e388 < _e91) {
                } else {
                    break;
                }
                let _e395 = global_6.member[_e388];
                global_6.member[_e388] = (_e395 + _e391);
            }
            break;
        }
    }
    return;
}

fn function_4() {
    var phi_1268_: u32;
    var phi_1271_: f32;
    var phi_1269_: u32;
    var phi_1272_: f32;
    var local_28: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e91 = global_7.member;
            if (_e87.y >= _e91) {
            } else {
                if (_e87.x >= _e91) {
                } else {
                    phi_1268_ = 0u;
                    phi_1271_ = 0f;
                    loop {
                        let _e97 = phi_1268_;
                        let _e99 = phi_1271_;
                        local_28 = _e99;
                        let _e100 = (_e97 < _e91);
                        if _e100 {
                            let _e109 = global_8.member[((_e87.y * _e91) + _e97)];
                            let _e110 = global_9.member[((_e97 * _e91) + _e87.x)];
                            phi_1269_ = (_e97 + 1u);
                            phi_1272_ = (_e99 + (_e109 * _e110));
                        } else {
                            phi_1269_ = u32();
                            phi_1272_ = f32();
                        }
                        let _e115 = phi_1269_;
                        let _e117 = phi_1272_;
                        continue;
                        continuing {
                            phi_1268_ = _e115;
                            phi_1271_ = _e117;
                            break if !(_e100);
                        }
                    }
                    let _e120 = ((_e87.y * _e91) + _e87.x);
                    if (_e120 < arrayLength((&global_10.member))) {
                    } else {
                        break;
                    }
                    let _e125 = local_28;
                    global_10.member[_e120] = _e125;
                }
            }
            break;
        }
    }
    return;
}

fn function_5() {
    var phi_3329_: bool;
    var phi_1340_: u32;
    var phi_1343_: f32;
    var phi_1352_: u32;
    var phi_1355_: f32;
    var phi_1353_: u32;
    var phi_1356_: f32;
    var phi_3326_: bool;
    var phi_3331_: bool;
    var phi_1341_: u32;
    var phi_1344_: f32;
    var phi_3330_: bool;
    var local_29: f32;
    var local_30: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e96 = global_4.member.member;
            if (_e87.x < _e96) {
                let _e100 = global_4.member.member_1;
                if (_e87.y < _e100) {
                    phi_3329_ = false;
                    phi_1340_ = 0u;
                    phi_1343_ = 0f;
                    loop {
                        let _e103 = phi_3329_;
                        let _e105 = phi_1340_;
                        let _e107 = phi_1343_;
                        local_29 = _e107;
                        let _e108 = (_e105 < 3u);
                        if _e108 {
                            phi_1352_ = 0u;
                            phi_1355_ = _e107;
                            loop {
                                let _e110 = phi_1352_;
                                let _e112 = phi_1355_;
                                local_30 = _e112;
                                let _e113 = (_e110 < 3u);
                                if _e113 {
                                    if (_e96 == 0u) {
                                        phi_3326_ = true;
                                        break;
                                    }
                                    if (_e100 == 0u) {
                                        phi_3326_ = true;
                                        break;
                                    }
                                    let _e125 = ((((((_e87.y + _e100) + _e105) - 1u) % _e100) * _e96) + ((((_e87.x + _e96) + _e110) - 1u) % _e96));
                                    if (_e125 < arrayLength((&global_11.member))) {
                                    } else {
                                        phi_3326_ = true;
                                        break;
                                    }
                                    let _e129 = global_11.member[_e125];
                                    phi_1353_ = (_e110 + 1u);
                                    phi_1356_ = (_e112 + _e129);
                                } else {
                                    phi_1353_ = u32();
                                    phi_1356_ = f32();
                                }
                                let _e133 = phi_1353_;
                                let _e135 = phi_1356_;
                                continue;
                                continuing {
                                    phi_1352_ = _e133;
                                    phi_1355_ = _e135;
                                    phi_3326_ = _e103;
                                    break if !(_e113);
                                }
                            }
                            let _e138 = phi_3326_;
                            phi_3330_ = _e138;
                            if _e138 {
                                break;
                            }
                            phi_3331_ = _e138;
                            phi_1341_ = (_e105 + 1u);
                            let _e173 = local_30;
                            phi_1344_ = _e173;
                        } else {
                            phi_3331_ = _e103;
                            phi_1341_ = u32();
                            phi_1344_ = f32();
                        }
                        let _e141 = phi_3331_;
                        let _e143 = phi_1341_;
                        let _e145 = phi_1344_;
                        continue;
                        continuing {
                            phi_3329_ = _e141;
                            phi_1340_ = _e143;
                            phi_1343_ = _e145;
                            phi_3330_ = _e141;
                            break if !(_e108);
                        }
                    }
                    let _e148 = phi_3330_;
                    if _e148 {
                        break;
                    }
                    let _e150 = local_29;
                    let _e154 = global_4.member.member_9;
                    let _e157 = ((_e87.y * _e96) + _e87.x);
                    if (_e157 < arrayLength((&global_12.member))) {
                    } else {
                        break;
                    }
                    global_12.member[_e157] = ((_e150 * 0.11111111f) * _e154);
                }
            }
            break;
        }
    }
    return;
}

fn function_6() {
    var phi_1426_: u32;
    var phi_1429_: f32;
    var phi_1427_: u32;
    var phi_1430_: f32;
    var phi_3332_: bool;
    var local_31: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e95 = global_7.member;
            if (_e87.y >= _e95) {
            } else {
                if (_e87.x >= _e95) {
                } else {
                    phi_1426_ = 0u;
                    phi_1429_ = 0f;
                    loop {
                        let _e101 = phi_1426_;
                        let _e103 = phi_1429_;
                        local_31 = _e103;
                        let _e104 = (_e101 < _e95);
                        if _e104 {
                            let _e106 = ((_e87.y * _e95) + _e101);
                            if (_e106 < arrayLength((&global_8.member))) {
                            } else {
                                phi_3332_ = true;
                                break;
                            }
                            let _e110 = global_8.member[_e106];
                            let _e112 = ((_e101 * _e95) + _e87.x);
                            if (_e112 < arrayLength((&global_9.member))) {
                            } else {
                                phi_3332_ = true;
                                break;
                            }
                            let _e116 = global_9.member[_e112];
                            phi_1427_ = (_e101 + 1u);
                            phi_1430_ = (_e103 + (_e110 * _e116));
                        } else {
                            phi_1427_ = u32();
                            phi_1430_ = f32();
                        }
                        let _e121 = phi_1427_;
                        let _e123 = phi_1430_;
                        continue;
                        continuing {
                            phi_1426_ = _e121;
                            phi_1429_ = _e123;
                            phi_3332_ = false;
                            break if !(_e104);
                        }
                    }
                    let _e126 = phi_3332_;
                    if _e126 {
                        break;
                    }
                    let _e128 = ((_e87.y * _e95) + _e87.x);
                    if (_e128 < arrayLength((&global_10.member))) {
                    } else {
                        break;
                    }
                    let _e133 = local_31;
                    global_10.member[_e128] = _e133;
                }
            }
            break;
        }
    }
    return;
}

fn function_7() {
    var phi_1499_: vec3<f32>;
    var phi_1502_: u32;
    var phi_3338_: u32;
    var phi_2846_: vec3<f32>;
    var phi_2847_: u32;
    var phi_2848_: vec3<f32>;
    var phi_2849_: vec3<f32>;
    var phi_3014_: u32;
    var phi_3015_: type_18;
    var phi_3145_: type_19;
    var phi_3076_: f32;
    var phi_3115_: type_18;
    var phi_3117_: type_18;
    var phi_3119_: type_18;
    var phi_3122_: u32;
    var phi_3123_: type_18;
    var local_32: type_18;
    var local_33: type_18;
    var phi_2877_: vec3<f32>;
    var local_34: type_18;
    var local_35: type_18;
    var local_36: type_18;
    var local_37: type_18;
    var phi_3227_: bool;
    var phi_3291_: bool;
    var phi_2961_: vec3<f32>;
    var phi_2962_: bool;
    var phi_2967_: u32;
    var phi_3345_: u32;
    var phi_2971_: vec3<f32>;
    var phi_2972_: u32;
    var phi_2973_: vec3<f32>;
    var phi_2974_: vec3<f32>;
    var phi_2975_: bool;
    var phi_2976_: bool;
    var phi_3344_: u32;
    var phi_2978_: vec3<f32>;
    var phi_2979_: u32;
    var phi_2980_: vec3<f32>;
    var phi_2981_: vec3<f32>;
    var phi_2982_: bool;
    var phi_2983_: bool;
    var phi_2985_: bool;
    var local_38: bool;
    var local_39: vec3<f32>;
    var local_40: vec3<f32>;
    var local_41: vec3<f32>;
    var local_42: vec3<f32>;
    var phi_2993_: vec3<f32>;
    var local_43: bool;
    var local_44: bool;
    var local_45: bool;
    var local_46: bool;
    var local_47: bool;
    var phi_1500_: vec3<f32>;
    var phi_1503_: u32;
    var local_48: u32;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e87 = global;
            let _e89 = arrayLength((&global_3.member));
            let _e94 = global_2.member.member;
            if (_e87.x >= _e94) {
            } else {
                let _e98 = global_2.member.member_1;
                if (_e87.y >= _e98) {
                } else {
                    let _e100 = f32(_e94);
                    let _e101 = f32(_e98);
                    let _e105 = ((_e87.y * _e94) + _e87.x);
                    phi_1499_ = vec3<f32>(0f, 0f, 0f);
                    phi_1502_ = 0u;
                    loop {
                        let _e107 = phi_1499_;
                        let _e109 = phi_1502_;
                        local_49 = _e107;
                        local_50 = _e107;
                        local_51 = _e107;
                        let _e112 = global_2.member.member_2;
                        let _e113 = (_e109 < _e112);
                        local_48 = _e112;
                        if _e113 {
                            let _e116 = global_2.member.member_3;
                            let _e121 = (((_e116 ^ 61u) ^ (_e116 >> bitcast<u32>(16i))) * 9u);
                            let _e125 = ((_e121 ^ (_e121 >> bitcast<u32>(4i))) * 668265261u);
                            let _e129 = (_e109 ^ (_e125 ^ (_e125 >> bitcast<u32>(15i))));
                            let _e134 = (((_e129 ^ 61u) ^ (_e129 >> bitcast<u32>(16i))) * 9u);
                            let _e138 = ((_e134 ^ (_e134 >> bitcast<u32>(4i))) * 668265261u);
                            let _e142 = (_e105 ^ (_e138 ^ (_e138 >> bitcast<u32>(15i))));
                            let _e147 = (((_e142 ^ 61u) ^ (_e142 >> bitcast<u32>(16i))) * 9u);
                            let _e151 = ((_e147 ^ (_e147 >> bitcast<u32>(4i))) * 668265261u);
                            let _e154 = (_e151 ^ (_e151 >> bitcast<u32>(15i)));
                            let _e156 = select(_e154, 2654435769u, (_e154 == 0u));
                            let _e159 = (_e156 ^ (_e156 << bitcast<u32>(13i)));
                            let _e162 = (_e159 ^ (_e159 >> bitcast<u32>(17i)));
                            let _e165 = (_e162 ^ (_e162 << bitcast<u32>(5i)));
                            let _e172 = (_e165 ^ (_e165 << bitcast<u32>(13i)));
                            let _e175 = (_e172 ^ (_e172 >> bitcast<u32>(17i)));
                            let _e178 = (_e175 ^ (_e175 << bitcast<u32>(5i)));
                            phi_3338_ = _e178;
                            phi_2846_ = vec3<f32>(1f, 1f, 1f);
                            phi_2847_ = 0u;
                            phi_2848_ = vec3<f32>(((((f32(_e87.x) + (f32((_e165 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e100) - 0.5f) * (2f * (_e100 / _e101))), ((0.5f - ((f32(_e87.y) + (f32((_e178 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e101)) * 2f), -1f);
                            phi_2849_ = vec3<f32>(0f, 0f, 0f);
                            loop {
                                let _e195 = phi_3338_;
                                let _e197 = phi_2846_;
                                let _e199 = phi_2847_;
                                let _e201 = phi_2848_;
                                let _e203 = phi_2849_;
                                local_39 = _e201;
                                local_40 = _e201;
                                local_41 = _e201;
                                local_42 = _e197;
                                let _e204 = (_e199 < 8u);
                                if _e204 {
                                    phi_3014_ = 0u;
                                    phi_3015_ = type_18(1000000000000000000000000000000f, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0u, 0f);
                                    loop {
                                        let _e206 = phi_3014_;
                                        let _e208 = phi_3015_;
                                        local_32 = _e208;
                                        local_33 = _e208;
                                        local_34 = _e208;
                                        local_35 = _e208;
                                        local_36 = _e208;
                                        local_37 = _e208;
                                        let _e209 = (_e206 < 4u);
                                        if _e209 {
                                            switch bitcast<i32>(_e206) {
                                                case 0: {
                                                    phi_3145_ = type_19(vec3<f32>(0f, -100.5f, -1f), 100f, vec3<f32>(0.8f, 0.8f, 0f), 0u, 0f);
                                                    break;
                                                }
                                                case 1: {
                                                    phi_3145_ = type_19(vec3<f32>(0f, 0f, -1.2f), 0.5f, vec3<f32>(0.1f, 0.2f, 0.5f), 0u, 0f);
                                                    break;
                                                }
                                                case 2: {
                                                    phi_3145_ = type_19(vec3<f32>(-1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.8f, 0.8f), 1u, 0.05f);
                                                    break;
                                                }
                                                default: {
                                                    phi_3145_ = type_19(vec3<f32>(1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.6f, 0.2f), 1u, 0.4f);
                                                    break;
                                                }
                                            }
                                            let _e212 = phi_3145_;
                                            let _e216 = (_e203.x - _e212.member.x);
                                            let _e219 = (_e203.y - _e212.member.y);
                                            let _e222 = (_e203.z - _e212.member.z);
                                            let _e230 = (((_e201.x * _e201.x) + (_e201.y * _e201.y)) + (_e201.z * _e201.z));
                                            let _e235 = (((_e216 * _e201.x) + (_e219 * _e201.y)) + (_e222 * _e201.z));
                                            let _e246 = ((_e235 * _e235) - (_e230 * ((((_e216 * _e216) + (_e219 * _e219)) + (_e222 * _e222)) - (_e212.member_1 * _e212.member_1))));
                                            if (_e246 > 0f) {
                                                let _e248 = sqrt(_e246);
                                                let _e249 = -(_e235);
                                                let _e251 = ((_e249 - _e248) / _e230);
                                                if (_e251 < 0.001f) {
                                                    phi_3076_ = ((_e249 + _e248) / _e230);
                                                } else {
                                                    phi_3076_ = _e251;
                                                }
                                                let _e256 = phi_3076_;
                                                if (_e256 >= 0.001f) {
                                                    if (_e256 < _e208.member) {
                                                        let _e260 = (_e256 * _e201.x);
                                                        let _e261 = (_e256 * _e201.y);
                                                        let _e262 = (_e256 * _e201.z);
                                                        phi_3115_ = type_18(_e256, (_e203 + vec3<f32>(_e260, _e261, _e262)), vec3<f32>((((_e203.x + _e260) - _e212.member.x) / _e212.member_1), (((_e203.y + _e261) - _e212.member.y) / _e212.member_1), (((_e203.z + _e262) - _e212.member.z) / _e212.member_1)), _e212.member_2, _e212.member_3, _e212.member_4);
                                                    } else {
                                                        phi_3115_ = _e208;
                                                    }
                                                    let _e280 = phi_3115_;
                                                    phi_3117_ = _e280;
                                                } else {
                                                    phi_3117_ = _e208;
                                                }
                                                let _e282 = phi_3117_;
                                                phi_3119_ = _e282;
                                            } else {
                                                phi_3119_ = _e208;
                                            }
                                            let _e284 = phi_3119_;
                                            phi_3122_ = (_e206 + 1u);
                                            phi_3123_ = _e284;
                                        } else {
                                            phi_3122_ = u32();
                                            phi_3123_ = type_18();
                                        }
                                        let _e287 = phi_3122_;
                                        let _e289 = phi_3123_;
                                        continue;
                                        continuing {
                                            phi_3014_ = _e287;
                                            phi_3015_ = _e289;
                                            break if !(_e209);
                                        }
                                    }
                                    let _e292 = local_32;
                                    let _e294 = (_e292.member >= 1000000000000000000000000000000f);
                                    if _e294 {
                                        phi_3345_ = _e195;
                                        phi_2971_ = vec3<f32>();
                                        phi_2972_ = u32();
                                        phi_2973_ = vec3<f32>();
                                        phi_2974_ = vec3<f32>();
                                        phi_2975_ = false;
                                        phi_2976_ = false;
                                    } else {
                                        let _e296 = local_33;
                                        if ((((_e201.x * _e296.member_2.x) + (_e201.y * _e296.member_2.y)) + (_e201.z * _e296.member_2.z)) < 0f) {
                                            phi_2877_ = _e296.member_2;
                                        } else {
                                            phi_2877_ = -(_e296.member_2);
                                        }
                                        let _e312 = phi_2877_;
                                        let _e314 = local_34;
                                        let _e318 = local_35;
                                        let _e321 = local_36;
                                        let _e326 = (_e195 ^ (_e195 << bitcast<u32>(13i)));
                                        let _e329 = (_e326 ^ (_e326 >> bitcast<u32>(17i)));
                                        let _e332 = (_e329 ^ (_e329 << bitcast<u32>(5i)));
                                        let _e335 = (_e332 ^ (_e332 << bitcast<u32>(13i)));
                                        let _e338 = (_e335 ^ (_e335 >> bitcast<u32>(17i)));
                                        let _e341 = (_e338 ^ (_e338 << bitcast<u32>(5i)));
                                        if (_e321.member_4 == 0u) {
                                            let _e413 = (1f - (f32((_e332 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e417 = (f32((_e341 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e419 = (1f - (_e413 * _e413));
                                            if (_e419 != _e419) {
                                                phi_3291_ = true;
                                            } else {
                                                phi_3291_ = (0f >= _e419);
                                            }
                                            let _e423 = phi_3291_;
                                            let _e425 = sqrt(select(_e419, 0f, _e423));
                                            let _e427 = (_e425 * cos(_e417));
                                            let _e429 = (_e425 * sin(_e417));
                                            let _e432 = (_e312.x + _e427);
                                            let _e434 = (_e312.y + _e429);
                                            let _e436 = (_e312.z + _e413);
                                            phi_2961_ = select((_e312 + vec3<f32>(_e427, _e429, _e413)), _e312, vec3(((((_e432 * _e432) + (_e434 * _e434)) + (_e436 * _e436)) < 0.00000001f)));
                                            phi_2962_ = true;
                                        } else {
                                            let _e349 = (_e201 * (1f / sqrt((((_e201.x * _e201.x) + (_e201.y * _e201.y)) + (_e201.z * _e201.z)))));
                                            let _e361 = (2f * (((_e349.x * _e312.x) + (_e349.y * _e312.y)) + (_e349.z * _e312.z)));
                                            let _e366 = (_e349 - vec3<f32>((_e361 * _e312.x), (_e361 * _e312.y), (_e361 * _e312.z)));
                                            let _e368 = local_37;
                                            let _e374 = (1f - (f32((_e332 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e378 = (f32((_e341 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e380 = (1f - (_e374 * _e374));
                                            if (_e380 != _e380) {
                                                phi_3227_ = true;
                                            } else {
                                                phi_3227_ = (0f >= _e380);
                                            }
                                            let _e384 = phi_3227_;
                                            let _e386 = sqrt(select(_e380, 0f, _e384));
                                            let _e391 = (_e368.member_5 * (_e386 * cos(_e378)));
                                            let _e392 = (_e368.member_5 * (_e386 * sin(_e378)));
                                            let _e393 = (_e368.member_5 * _e374);
                                            phi_2961_ = (_e366 + vec3<f32>(_e391, _e392, _e393));
                                            phi_2962_ = select(true, false, (((((_e366.x + _e391) * _e312.x) + ((_e366.y + _e392) * _e312.y)) + ((_e366.z + _e393) * _e312.z)) <= 0f));
                                        }
                                        let _e447 = phi_2961_;
                                        let _e449 = phi_2962_;
                                        if _e449 {
                                            phi_2967_ = (_e199 + 1u);
                                        } else {
                                            phi_2967_ = u32();
                                        }
                                        let _e452 = phi_2967_;
                                        phi_3345_ = _e341;
                                        phi_2971_ = (_e197 * _e314.member_3);
                                        phi_2972_ = _e452;
                                        phi_2973_ = _e447;
                                        phi_2974_ = _e318.member_1;
                                        phi_2975_ = _e449;
                                        phi_2976_ = select(true, false, _e449);
                                    }
                                    let _e455 = phi_3345_;
                                    let _e457 = phi_2971_;
                                    let _e459 = phi_2972_;
                                    let _e461 = phi_2973_;
                                    let _e463 = phi_2974_;
                                    let _e465 = phi_2975_;
                                    let _e467 = phi_2976_;
                                    phi_3344_ = _e455;
                                    phi_2978_ = _e457;
                                    phi_2979_ = _e459;
                                    phi_2980_ = _e461;
                                    phi_2981_ = _e463;
                                    phi_2982_ = _e465;
                                    phi_2983_ = _e294;
                                    phi_2985_ = _e467;
                                } else {
                                    phi_3344_ = _e195;
                                    phi_2978_ = vec3<f32>();
                                    phi_2979_ = u32();
                                    phi_2980_ = vec3<f32>();
                                    phi_2981_ = vec3<f32>();
                                    phi_2982_ = false;
                                    phi_2983_ = false;
                                    phi_2985_ = false;
                                }
                                let _e469 = phi_3344_;
                                let _e471 = phi_2978_;
                                let _e473 = phi_2979_;
                                let _e475 = phi_2980_;
                                let _e477 = phi_2981_;
                                let _e479 = phi_2982_;
                                let _e481 = phi_2983_;
                                let _e483 = phi_2985_;
                                local_38 = _e481;
                                local_43 = _e481;
                                local_44 = select(true, false, _e204);
                                local_45 = _e481;
                                local_46 = _e483;
                                local_47 = _e481;
                                continue;
                                continuing {
                                    phi_3338_ = _e469;
                                    phi_2846_ = _e471;
                                    phi_2847_ = _e473;
                                    phi_2848_ = _e475;
                                    phi_2849_ = _e477;
                                    break if !(_e479);
                                }
                            }
                            let _e487 = local_38;
                            if _e487 {
                                let _e489 = local_39;
                                let _e493 = local_40;
                                let _e498 = local_41;
                                let _e505 = ((_e493.y * (1f / sqrt((((_e489.x * _e489.x) + (_e493.y * _e493.y)) + (_e498.z * _e498.z))))) + 1f);
                                let _e507 = (1f - (0.5f * _e505));
                                let _e514 = local_42;
                                phi_2993_ = (_e514 * vec3<f32>((_e507 + (_e505 * 0.25f)), (_e507 + (_e505 * 0.35f)), 1f));
                            } else {
                                phi_2993_ = vec3<f32>();
                            }
                            let _e517 = phi_2993_;
                            let _e519 = local_43;
                            let _e521 = local_44;
                            let _e522 = select(_e521, false, _e519);
                            let _e524 = local_45;
                            let _e526 = local_46;
                            let _e528 = select(select(_e526, false, _e524), false, _e522);
                            let _e530 = local_47;
                            phi_1500_ = (_e107 + select(vec3<f32>(0f, 0f, 0f), select(_e517, vec3<f32>(0f, 0f, 0f), vec3(_e528)), vec3(select(select(_e530, false, _e522), true, _e528))));
                            phi_1503_ = (_e109 + 1u);
                        } else {
                            phi_1500_ = vec3<f32>();
                            phi_1503_ = u32();
                        }
                        let _e540 = phi_1500_;
                        let _e542 = phi_1503_;
                        continue;
                        continuing {
                            phi_1499_ = _e540;
                            phi_1502_ = _e542;
                            break if !(_e113);
                        }
                    }
                    let _e545 = local_48;
                    let _e546 = f32(_e545);
                    let _e548 = local_49;
                    let _e552 = local_50;
                    let _e556 = local_51;
                    let _e559 = (_e105 * 3u);
                    if (_e559 < _e89) {
                    } else {
                        break;
                    }
                    global_3.member[_e559] = (_e548.x / _e546);
                    let _e563 = (_e559 + 1u);
                    if (_e563 < _e89) {
                    } else {
                        break;
                    }
                    global_3.member[_e563] = (_e552.y / _e546);
                    let _e567 = (_e559 + 2u);
                    if (_e567 < _e89) {
                    } else {
                        break;
                    }
                    global_3.member[_e567] = (_e556.z / _e546);
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(64, 1, 1) 
fn collatz_cs(@builtin(global_invocation_id) param: vec3<u32>) {
    global = param;
    function_();
}

@compute @workgroup_size(8, 8, 1) 
fn render_v2_cs(@builtin(global_invocation_id) param_1: vec3<u32>) {
    global = param_1;
    function_1();
}

@compute @workgroup_size(64, 1, 1) 
fn physarum_spawn_cs(@builtin(global_invocation_id) param_2: vec3<u32>) {
    global = param_2;
    function_2();
}

@compute @workgroup_size(64, 1, 1) 
fn physarum_update_cs(@builtin(global_invocation_id) param_3: vec3<u32>) {
    global = param_3;
    function_3();
}

@compute @workgroup_size(16, 16, 1) 
fn matmul_unchecked_cs(@builtin(global_invocation_id) param_4: vec3<u32>) {
    global = param_4;
    function_4();
}

@compute @workgroup_size(8, 8, 1) 
fn physarum_diffuse_cs(@builtin(global_invocation_id) param_5: vec3<u32>) {
    global = param_5;
    function_5();
}

@compute @workgroup_size(16, 16, 1) 
fn matmul_cs(@builtin(global_invocation_id) param_6: vec3<u32>) {
    global = param_6;
    function_6();
}

@compute @workgroup_size(8, 8, 1) 
fn render_cs(@builtin(global_invocation_id) param_7: vec3<u32>) {
    global = param_7;
    function_7();
}
