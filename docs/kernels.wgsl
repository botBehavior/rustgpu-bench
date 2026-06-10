struct type_4 {
    member: array<u32>,
}

struct type_8 {
    member: u32,
}

struct type_12 {
    member: array<f32>,
}

struct type_15 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_16 {
    member: type_15,
}

struct type_20 {
    member: f32,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
    member_4: u32,
    member_5: f32,
}

struct type_21 {
    member: vec3<f32>,
    member_1: f32,
    member_2: vec3<f32>,
    member_3: u32,
    member_4: f32,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<storage, read_write> global_1: type_4;
@group(0) @binding(0) 
var<storage> global_2: type_8;
@group(0) @binding(1) 
var<storage> global_3: type_12;
@group(0) @binding(2) 
var<storage> global_4: type_12;
@group(0) @binding(3) 
var<storage, read_write> global_5: type_12;
@group(0) @binding(0) 
var<storage> global_6: type_16;
@group(0) @binding(1) 
var<storage, read_write> global_7: type_12;

fn function_() {
    var phi_844_: u32;
    var phi_845_: u32;
    var phi_864_: u32;
    var phi_867_: u32;
    var phi_868_: bool;
    var phi_873_: u32;
    var phi_876_: u32;
    var phi_877_: u32;
    var phi_878_: bool;
    var phi_879_: bool;
    var phi_881_: u32;
    var phi_882_: u32;
    var phi_883_: bool;
    var phi_884_: bool;
    var phi_885_: bool;
    var local: bool;
    var local_1: bool;
    var local_2: bool;
    var local_3: bool;
    var local_4: u32;
    var phi_906_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e72 = global;
            let _e76 = (_e72.x < arrayLength((&global_1.member)));
            if _e76 {
                if _e76 {
                } else {
                    break;
                }
                let _e79 = global_1.member[_e72.x];
                if (_e79 == 0u) {
                    phi_906_ = 4294967295u;
                } else {
                    phi_844_ = 0u;
                    phi_845_ = _e79;
                    loop {
                        let _e82 = phi_844_;
                        let _e84 = phi_845_;
                        local_4 = _e82;
                        let _e85 = (_e84 == 1u);
                        local = _e85;
                        local_2 = _e85;
                        if _e85 {
                            phi_881_ = u32();
                            phi_882_ = u32();
                            phi_883_ = false;
                            phi_884_ = false;
                            phi_885_ = false;
                        } else {
                            let _e86 = (_e82 >= 1000u);
                            if _e86 {
                                phi_876_ = u32();
                                phi_877_ = u32();
                                phi_878_ = false;
                                phi_879_ = false;
                            } else {
                                if ((_e84 % 2u) == 0u) {
                                    phi_867_ = (_e84 / 2u);
                                    phi_868_ = true;
                                } else {
                                    let _e90 = (_e84 > 1431655764u);
                                    if _e90 {
                                        phi_864_ = u32();
                                    } else {
                                        phi_864_ = ((3u * _e84) + 1u);
                                    }
                                    let _e94 = phi_864_;
                                    phi_867_ = _e94;
                                    phi_868_ = select(true, false, _e90);
                                }
                                let _e97 = phi_867_;
                                let _e99 = phi_868_;
                                if _e99 {
                                    phi_873_ = (_e82 + 1u);
                                } else {
                                    phi_873_ = u32();
                                }
                                let _e102 = phi_873_;
                                phi_876_ = _e102;
                                phi_877_ = _e97;
                                phi_878_ = _e99;
                                phi_879_ = select(true, false, _e99);
                            }
                            let _e105 = phi_876_;
                            let _e107 = phi_877_;
                            let _e109 = phi_878_;
                            let _e111 = phi_879_;
                            phi_881_ = _e105;
                            phi_882_ = _e107;
                            phi_883_ = _e109;
                            phi_884_ = _e86;
                            phi_885_ = _e111;
                        }
                        let _e113 = phi_881_;
                        let _e115 = phi_882_;
                        let _e117 = phi_883_;
                        let _e119 = phi_884_;
                        let _e121 = phi_885_;
                        local_1 = _e119;
                        local_3 = _e121;
                        continue;
                        continuing {
                            phi_844_ = _e113;
                            phi_845_ = _e115;
                            break if !(_e117);
                        }
                    }
                    let _e124 = local;
                    let _e126 = local_1;
                    let _e127 = select(_e126, false, _e124);
                    let _e129 = local_2;
                    let _e131 = local_3;
                    let _e136 = local_4;
                    phi_906_ = select(_e136, 4294967295u, select(_e127, true, select(select(_e131, false, _e129), false, _e127)));
                }
                let _e139 = phi_906_;
                if _e76 {
                } else {
                    break;
                }
                global_1.member[_e72.x] = _e139;
            }
            break;
        }
    }
    return;
}

fn function_1() {
    var phi_237_: u32;
    var phi_240_: f32;
    var phi_238_: u32;
    var phi_241_: f32;
    var phi_1471_: bool;
    var local_5: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e72 = global;
            let _e80 = global_2.member;
            if (_e72.y >= _e80) {
            } else {
                if (_e72.x >= _e80) {
                } else {
                    phi_237_ = 0u;
                    phi_240_ = 0f;
                    loop {
                        let _e86 = phi_237_;
                        let _e88 = phi_240_;
                        local_5 = _e88;
                        let _e89 = (_e86 < _e80);
                        if _e89 {
                            let _e91 = ((_e72.y * _e80) + _e86);
                            if (_e91 < arrayLength((&global_3.member))) {
                            } else {
                                phi_1471_ = true;
                                break;
                            }
                            let _e95 = global_3.member[_e91];
                            let _e97 = ((_e86 * _e80) + _e72.x);
                            if (_e97 < arrayLength((&global_4.member))) {
                            } else {
                                phi_1471_ = true;
                                break;
                            }
                            let _e101 = global_4.member[_e97];
                            phi_238_ = (_e86 + 1u);
                            phi_241_ = (_e88 + (_e95 * _e101));
                        } else {
                            phi_238_ = u32();
                            phi_241_ = f32();
                        }
                        let _e106 = phi_238_;
                        let _e108 = phi_241_;
                        continue;
                        continuing {
                            phi_237_ = _e106;
                            phi_240_ = _e108;
                            phi_1471_ = false;
                            break if !(_e89);
                        }
                    }
                    let _e111 = phi_1471_;
                    if _e111 {
                        break;
                    }
                    let _e113 = ((_e72.y * _e80) + _e72.x);
                    if (_e113 < arrayLength((&global_5.member))) {
                    } else {
                        break;
                    }
                    let _e118 = local_5;
                    global_5.member[_e113] = _e118;
                }
            }
            break;
        }
    }
    return;
}

fn function_2() {
    var phi_310_: vec3<f32>;
    var phi_313_: u32;
    var phi_1482_: u32;
    var phi_989_: vec3<f32>;
    var phi_990_: u32;
    var phi_991_: vec3<f32>;
    var phi_992_: vec3<f32>;
    var phi_1158_: u32;
    var phi_1159_: type_20;
    var phi_1290_: type_21;
    var phi_1218_: f32;
    var phi_1256_: type_20;
    var phi_1259_: type_20;
    var phi_1262_: type_20;
    var phi_1266_: u32;
    var phi_1267_: type_20;
    var local_6: type_20;
    var local_7: type_20;
    var phi_1020_: vec3<f32>;
    var local_8: type_20;
    var local_9: type_20;
    var local_10: type_20;
    var phi_1350_: bool;
    var local_11: type_20;
    var phi_1436_: bool;
    var phi_1104_: vec3<f32>;
    var phi_1105_: bool;
    var phi_1110_: u32;
    var phi_1484_: u32;
    var phi_1113_: vec3<f32>;
    var phi_1114_: u32;
    var phi_1115_: vec3<f32>;
    var phi_1116_: vec3<f32>;
    var phi_1117_: bool;
    var phi_1118_: bool;
    var phi_1483_: u32;
    var phi_1121_: vec3<f32>;
    var phi_1122_: u32;
    var phi_1123_: vec3<f32>;
    var phi_1124_: vec3<f32>;
    var phi_1125_: bool;
    var phi_1126_: bool;
    var phi_1128_: bool;
    var local_12: bool;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var local_15: vec3<f32>;
    var local_16: vec3<f32>;
    var phi_1136_: vec3<f32>;
    var local_17: bool;
    var local_18: bool;
    var local_19: bool;
    var local_20: bool;
    var local_21: bool;
    var phi_311_: vec3<f32>;
    var phi_314_: u32;
    var local_22: u32;
    var local_23: vec3<f32>;
    var local_24: vec3<f32>;
    var local_25: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e72 = global;
            let _e74 = arrayLength((&global_7.member));
            let _e79 = global_6.member.member;
            if (_e72.x >= _e79) {
            } else {
                let _e83 = global_6.member.member_1;
                if (_e72.y >= _e83) {
                } else {
                    let _e85 = f32(_e79);
                    let _e86 = f32(_e83);
                    let _e90 = ((_e72.y * _e79) + _e72.x);
                    phi_310_ = vec3<f32>(0f, 0f, 0f);
                    phi_313_ = 0u;
                    loop {
                        let _e92 = phi_310_;
                        let _e94 = phi_313_;
                        local_23 = _e92;
                        local_24 = _e92;
                        local_25 = _e92;
                        let _e97 = global_6.member.member_2;
                        let _e98 = (_e94 < _e97);
                        local_22 = _e97;
                        if _e98 {
                            let _e101 = global_6.member.member_3;
                            let _e106 = (((_e101 ^ 61u) ^ (_e101 >> bitcast<u32>(16i))) * 9u);
                            let _e110 = ((_e106 ^ (_e106 >> bitcast<u32>(4i))) * 668265261u);
                            let _e114 = (_e94 ^ (_e110 ^ (_e110 >> bitcast<u32>(15i))));
                            let _e119 = (((_e114 ^ 61u) ^ (_e114 >> bitcast<u32>(16i))) * 9u);
                            let _e123 = ((_e119 ^ (_e119 >> bitcast<u32>(4i))) * 668265261u);
                            let _e127 = (_e90 ^ (_e123 ^ (_e123 >> bitcast<u32>(15i))));
                            let _e132 = (((_e127 ^ 61u) ^ (_e127 >> bitcast<u32>(16i))) * 9u);
                            let _e136 = ((_e132 ^ (_e132 >> bitcast<u32>(4i))) * 668265261u);
                            let _e139 = (_e136 ^ (_e136 >> bitcast<u32>(15i)));
                            let _e141 = select(_e139, 2654435769u, (_e139 == 0u));
                            let _e144 = (_e141 ^ (_e141 << bitcast<u32>(13i)));
                            let _e147 = (_e144 ^ (_e144 >> bitcast<u32>(17i)));
                            let _e150 = (_e147 ^ (_e147 << bitcast<u32>(5i)));
                            let _e157 = (_e150 ^ (_e150 << bitcast<u32>(13i)));
                            let _e160 = (_e157 ^ (_e157 >> bitcast<u32>(17i)));
                            let _e163 = (_e160 ^ (_e160 << bitcast<u32>(5i)));
                            phi_1482_ = _e163;
                            phi_989_ = vec3<f32>(1f, 1f, 1f);
                            phi_990_ = 0u;
                            phi_991_ = vec3<f32>(((((f32(_e72.x) + (f32((_e150 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e85) - 0.5f) * (2f * (_e85 / _e86))), ((0.5f - ((f32(_e72.y) + (f32((_e163 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e86)) * 2f), -1f);
                            phi_992_ = vec3<f32>(0f, 0f, 0f);
                            loop {
                                let _e180 = phi_1482_;
                                let _e182 = phi_989_;
                                let _e184 = phi_990_;
                                let _e186 = phi_991_;
                                let _e188 = phi_992_;
                                local_13 = _e186;
                                local_14 = _e186;
                                local_15 = _e186;
                                local_16 = _e182;
                                let _e189 = (_e184 < 8u);
                                if _e189 {
                                    phi_1158_ = 0u;
                                    phi_1159_ = type_20(1000000000000000000000000000000f, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0u, 0f);
                                    loop {
                                        let _e191 = phi_1158_;
                                        let _e193 = phi_1159_;
                                        local_6 = _e193;
                                        local_7 = _e193;
                                        local_8 = _e193;
                                        local_9 = _e193;
                                        local_10 = _e193;
                                        local_11 = _e193;
                                        let _e194 = (_e191 < 4u);
                                        if _e194 {
                                            switch bitcast<i32>(_e191) {
                                                case 0: {
                                                    phi_1290_ = type_21(vec3<f32>(0f, -100.5f, -1f), 100f, vec3<f32>(0.8f, 0.8f, 0f), 0u, 0f);
                                                    break;
                                                }
                                                case 1: {
                                                    phi_1290_ = type_21(vec3<f32>(0f, 0f, -1.2f), 0.5f, vec3<f32>(0.1f, 0.2f, 0.5f), 0u, 0f);
                                                    break;
                                                }
                                                case 2: {
                                                    phi_1290_ = type_21(vec3<f32>(-1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.8f, 0.8f), 1u, 0.05f);
                                                    break;
                                                }
                                                default: {
                                                    phi_1290_ = type_21(vec3<f32>(1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.6f, 0.2f), 1u, 0.4f);
                                                    break;
                                                }
                                            }
                                            let _e197 = phi_1290_;
                                            let _e201 = (_e188.x - _e197.member.x);
                                            let _e204 = (_e188.y - _e197.member.y);
                                            let _e207 = (_e188.z - _e197.member.z);
                                            let _e215 = (((_e186.x * _e186.x) + (_e186.y * _e186.y)) + (_e186.z * _e186.z));
                                            let _e220 = (((_e201 * _e186.x) + (_e204 * _e186.y)) + (_e207 * _e186.z));
                                            let _e231 = ((_e220 * _e220) - (_e215 * ((((_e201 * _e201) + (_e204 * _e204)) + (_e207 * _e207)) - (_e197.member_1 * _e197.member_1))));
                                            if (_e231 > 0f) {
                                                let _e233 = sqrt(_e231);
                                                let _e234 = -(_e220);
                                                let _e236 = ((_e234 - _e233) / _e215);
                                                if (_e236 < 0.001f) {
                                                    phi_1218_ = ((_e234 + _e233) / _e215);
                                                } else {
                                                    phi_1218_ = _e236;
                                                }
                                                let _e241 = phi_1218_;
                                                if (_e241 >= 0.001f) {
                                                    if (_e241 < _e193.member) {
                                                        let _e245 = (_e241 * _e186.x);
                                                        let _e246 = (_e241 * _e186.y);
                                                        let _e247 = (_e241 * _e186.z);
                                                        phi_1256_ = type_20(_e241, (_e188 + vec3<f32>(_e245, _e246, _e247)), vec3<f32>((((_e188.x + _e245) - _e197.member.x) / _e197.member_1), (((_e188.y + _e246) - _e197.member.y) / _e197.member_1), (((_e188.z + _e247) - _e197.member.z) / _e197.member_1)), _e197.member_2, _e197.member_3, _e197.member_4);
                                                    } else {
                                                        phi_1256_ = _e193;
                                                    }
                                                    let _e265 = phi_1256_;
                                                    phi_1259_ = _e265;
                                                } else {
                                                    phi_1259_ = _e193;
                                                }
                                                let _e267 = phi_1259_;
                                                phi_1262_ = _e267;
                                            } else {
                                                phi_1262_ = _e193;
                                            }
                                            let _e269 = phi_1262_;
                                            phi_1266_ = (_e191 + 1u);
                                            phi_1267_ = _e269;
                                        } else {
                                            phi_1266_ = u32();
                                            phi_1267_ = type_20();
                                        }
                                        let _e272 = phi_1266_;
                                        let _e274 = phi_1267_;
                                        continue;
                                        continuing {
                                            phi_1158_ = _e272;
                                            phi_1159_ = _e274;
                                            break if !(_e194);
                                        }
                                    }
                                    let _e277 = local_6;
                                    let _e279 = (_e277.member >= 1000000000000000000000000000000f);
                                    if _e279 {
                                        phi_1484_ = _e180;
                                        phi_1113_ = vec3<f32>();
                                        phi_1114_ = u32();
                                        phi_1115_ = vec3<f32>();
                                        phi_1116_ = vec3<f32>();
                                        phi_1117_ = false;
                                        phi_1118_ = false;
                                    } else {
                                        let _e281 = local_7;
                                        if ((((_e186.x * _e281.member_2.x) + (_e186.y * _e281.member_2.y)) + (_e186.z * _e281.member_2.z)) < 0f) {
                                            phi_1020_ = _e281.member_2;
                                        } else {
                                            phi_1020_ = -(_e281.member_2);
                                        }
                                        let _e297 = phi_1020_;
                                        let _e299 = local_8;
                                        let _e303 = local_9;
                                        let _e306 = local_10;
                                        let _e311 = (_e180 ^ (_e180 << bitcast<u32>(13i)));
                                        let _e314 = (_e311 ^ (_e311 >> bitcast<u32>(17i)));
                                        let _e317 = (_e314 ^ (_e314 << bitcast<u32>(5i)));
                                        let _e320 = (_e317 ^ (_e317 << bitcast<u32>(13i)));
                                        let _e323 = (_e320 ^ (_e320 >> bitcast<u32>(17i)));
                                        let _e326 = (_e323 ^ (_e323 << bitcast<u32>(5i)));
                                        if (_e306.member_4 == 0u) {
                                            let _e331 = (1f - (f32((_e317 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e335 = (f32((_e326 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e337 = (1f - (_e331 * _e331));
                                            if (_e337 != _e337) {
                                                phi_1350_ = true;
                                            } else {
                                                phi_1350_ = (0f >= _e337);
                                            }
                                            let _e341 = phi_1350_;
                                            let _e343 = sqrt(select(_e337, 0f, _e341));
                                            let _e345 = (_e343 * cos(_e335));
                                            let _e347 = (_e343 * sin(_e335));
                                            let _e350 = (_e297.x + _e345);
                                            let _e352 = (_e297.y + _e347);
                                            let _e354 = (_e297.z + _e331);
                                            phi_1104_ = select((_e297 + vec3<f32>(_e345, _e347, _e331)), _e297, vec3(((((_e350 * _e350) + (_e352 * _e352)) + (_e354 * _e354)) < 0.00000001f)));
                                            phi_1105_ = true;
                                        } else {
                                            let _e371 = (_e186 * (1f / sqrt((((_e186.x * _e186.x) + (_e186.y * _e186.y)) + (_e186.z * _e186.z)))));
                                            let _e383 = (2f * (((_e371.x * _e297.x) + (_e371.y * _e297.y)) + (_e371.z * _e297.z)));
                                            let _e388 = (_e371 - vec3<f32>((_e383 * _e297.x), (_e383 * _e297.y), (_e383 * _e297.z)));
                                            let _e390 = local_11;
                                            let _e396 = (1f - (f32((_e317 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e400 = (f32((_e326 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e402 = (1f - (_e396 * _e396));
                                            if (_e402 != _e402) {
                                                phi_1436_ = true;
                                            } else {
                                                phi_1436_ = (0f >= _e402);
                                            }
                                            let _e406 = phi_1436_;
                                            let _e408 = sqrt(select(_e402, 0f, _e406));
                                            let _e413 = (_e390.member_5 * (_e408 * cos(_e400)));
                                            let _e414 = (_e390.member_5 * (_e408 * sin(_e400)));
                                            let _e415 = (_e390.member_5 * _e396);
                                            phi_1104_ = (_e388 + vec3<f32>(_e413, _e414, _e415));
                                            phi_1105_ = select(true, false, (((((_e388.x + _e413) * _e297.x) + ((_e388.y + _e414) * _e297.y)) + ((_e388.z + _e415) * _e297.z)) <= 0f));
                                        }
                                        let _e432 = phi_1104_;
                                        let _e434 = phi_1105_;
                                        if _e434 {
                                            phi_1110_ = (_e184 + 1u);
                                        } else {
                                            phi_1110_ = u32();
                                        }
                                        let _e437 = phi_1110_;
                                        phi_1484_ = _e326;
                                        phi_1113_ = (_e182 * _e299.member_3);
                                        phi_1114_ = _e437;
                                        phi_1115_ = _e432;
                                        phi_1116_ = _e303.member_1;
                                        phi_1117_ = _e434;
                                        phi_1118_ = select(true, false, _e434);
                                    }
                                    let _e440 = phi_1484_;
                                    let _e442 = phi_1113_;
                                    let _e444 = phi_1114_;
                                    let _e446 = phi_1115_;
                                    let _e448 = phi_1116_;
                                    let _e450 = phi_1117_;
                                    let _e452 = phi_1118_;
                                    phi_1483_ = _e440;
                                    phi_1121_ = _e442;
                                    phi_1122_ = _e444;
                                    phi_1123_ = _e446;
                                    phi_1124_ = _e448;
                                    phi_1125_ = _e450;
                                    phi_1126_ = _e279;
                                    phi_1128_ = _e452;
                                } else {
                                    phi_1483_ = _e180;
                                    phi_1121_ = vec3<f32>();
                                    phi_1122_ = u32();
                                    phi_1123_ = vec3<f32>();
                                    phi_1124_ = vec3<f32>();
                                    phi_1125_ = false;
                                    phi_1126_ = false;
                                    phi_1128_ = false;
                                }
                                let _e454 = phi_1483_;
                                let _e456 = phi_1121_;
                                let _e458 = phi_1122_;
                                let _e460 = phi_1123_;
                                let _e462 = phi_1124_;
                                let _e464 = phi_1125_;
                                let _e466 = phi_1126_;
                                let _e468 = phi_1128_;
                                local_12 = _e466;
                                local_17 = _e466;
                                local_18 = select(true, false, _e189);
                                local_19 = _e466;
                                local_20 = _e468;
                                local_21 = _e466;
                                continue;
                                continuing {
                                    phi_1482_ = _e454;
                                    phi_989_ = _e456;
                                    phi_990_ = _e458;
                                    phi_991_ = _e460;
                                    phi_992_ = _e462;
                                    break if !(_e464);
                                }
                            }
                            let _e472 = local_12;
                            if _e472 {
                                let _e474 = local_13;
                                let _e478 = local_14;
                                let _e483 = local_15;
                                let _e490 = ((_e478.y * (1f / sqrt((((_e474.x * _e474.x) + (_e478.y * _e478.y)) + (_e483.z * _e483.z))))) + 1f);
                                let _e492 = (1f - (0.5f * _e490));
                                let _e499 = local_16;
                                phi_1136_ = (_e499 * vec3<f32>((_e492 + (_e490 * 0.25f)), (_e492 + (_e490 * 0.35f)), 1f));
                            } else {
                                phi_1136_ = vec3<f32>();
                            }
                            let _e502 = phi_1136_;
                            let _e504 = local_17;
                            let _e506 = local_18;
                            let _e507 = select(_e506, false, _e504);
                            let _e509 = local_19;
                            let _e511 = local_20;
                            let _e513 = select(select(_e511, false, _e509), false, _e507);
                            let _e515 = local_21;
                            phi_311_ = (_e92 + select(vec3<f32>(0f, 0f, 0f), select(_e502, vec3<f32>(0f, 0f, 0f), vec3(_e513)), vec3(select(select(_e515, false, _e507), true, _e513))));
                            phi_314_ = (_e94 + 1u);
                        } else {
                            phi_311_ = vec3<f32>();
                            phi_314_ = u32();
                        }
                        let _e525 = phi_311_;
                        let _e527 = phi_314_;
                        continue;
                        continuing {
                            phi_310_ = _e525;
                            phi_313_ = _e527;
                            break if !(_e98);
                        }
                    }
                    let _e530 = local_22;
                    let _e531 = f32(_e530);
                    let _e533 = local_23;
                    let _e537 = local_24;
                    let _e541 = local_25;
                    let _e544 = (_e90 * 3u);
                    if (_e544 < _e74) {
                    } else {
                        break;
                    }
                    global_7.member[_e544] = (_e533.x / _e531);
                    let _e548 = (_e544 + 1u);
                    if (_e548 < _e74) {
                    } else {
                        break;
                    }
                    global_7.member[_e548] = (_e537.y / _e531);
                    let _e552 = (_e544 + 2u);
                    if (_e552 < _e74) {
                    } else {
                        break;
                    }
                    global_7.member[_e552] = (_e541.z / _e531);
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

@compute @workgroup_size(16, 16, 1) 
fn matmul_cs(@builtin(global_invocation_id) param_1: vec3<u32>) {
    global = param_1;
    function_1();
}

@compute @workgroup_size(8, 8, 1) 
fn render_cs(@builtin(global_invocation_id) param_2: vec3<u32>) {
    global = param_2;
    function_2();
}
