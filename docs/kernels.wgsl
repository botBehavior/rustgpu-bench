struct type_4 {
    member: array<u32>,
}

struct type_9 {
    member: array<u32, 4>,
}

struct type_13 {
    member: array<f32>,
}

struct type_19 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
}

struct type_20 {
    member: type_19,
}

struct type_22 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_23 {
    member: f32,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
    member_4: u32,
    member_5: f32,
}

struct type_24 {
    member: vec3<f32>,
    member_1: f32,
    member_2: vec3<f32>,
    member_3: u32,
    member_4: f32,
}

struct type_25 {
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

struct type_26 {
    member: type_25,
}

struct type_28 {
    member: f32,
    member_1: f32,
    member_2: f32,
    member_3: f32,
}

struct type_30 {
    member: array<type_28>,
}

struct type_33 {
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
var<storage> global_4: type_20;
@group(0) @binding(0) 
var<storage> global_5: type_26;
@group(0) @binding(1) 
var<storage, read_write> global_6: type_30;
@group(0) @binding(2) 
var<storage, read_write> global_7: type_13;
@group(0) @binding(0) 
var<storage> global_8: type_33;
@group(0) @binding(1) 
var<storage> global_9: type_13;
@group(0) @binding(2) 
var<storage> global_10: type_13;
@group(0) @binding(3) 
var<storage, read_write> global_11: type_13;
@group(0) @binding(1) 
var<storage> global_12: type_13;
@group(0) @binding(2) 
var<storage, read_write> global_13: type_13;

fn function_() {
    var phi_2763_: u32;
    var phi_2764_: u32;
    var phi_2779_: u32;
    var phi_2784_: u32;
    var phi_2785_: bool;
    var phi_2790_: u32;
    var phi_2794_: u32;
    var phi_2795_: u32;
    var phi_2796_: bool;
    var phi_2797_: bool;
    var phi_2800_: u32;
    var phi_2801_: u32;
    var phi_2802_: bool;
    var phi_2803_: bool;
    var phi_2804_: bool;
    var local: bool;
    var local_1: bool;
    var local_2: bool;
    var local_3: bool;
    var local_4: u32;
    var phi_2826_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e191 = (_e187.x < arrayLength((&global_1.member)));
            if _e191 {
                if _e191 {
                } else {
                    break;
                }
                let _e194 = global_1.member[_e187.x];
                if (_e194 == 0u) {
                    phi_2826_ = 4294967295u;
                } else {
                    phi_2763_ = 0u;
                    phi_2764_ = _e194;
                    loop {
                        let _e197 = phi_2763_;
                        let _e199 = phi_2764_;
                        local_4 = _e197;
                        let _e200 = (_e199 == 1u);
                        local = _e200;
                        local_2 = _e200;
                        if _e200 {
                            phi_2800_ = u32();
                            phi_2801_ = u32();
                            phi_2802_ = false;
                            phi_2803_ = false;
                            phi_2804_ = false;
                        } else {
                            let _e201 = (_e197 >= 1000u);
                            if _e201 {
                                phi_2794_ = u32();
                                phi_2795_ = u32();
                                phi_2796_ = false;
                                phi_2797_ = false;
                            } else {
                                if ((_e199 % 2u) == 0u) {
                                    phi_2784_ = (_e199 / 2u);
                                    phi_2785_ = true;
                                } else {
                                    let _e204 = (_e199 > 1431655764u);
                                    if _e204 {
                                        phi_2779_ = u32();
                                    } else {
                                        phi_2779_ = ((3u * _e199) + 1u);
                                    }
                                    let _e208 = phi_2779_;
                                    phi_2784_ = _e208;
                                    phi_2785_ = select(true, false, _e204);
                                }
                                let _e212 = phi_2784_;
                                let _e214 = phi_2785_;
                                if _e214 {
                                    phi_2790_ = (_e197 + 1u);
                                } else {
                                    phi_2790_ = u32();
                                }
                                let _e217 = phi_2790_;
                                phi_2794_ = _e217;
                                phi_2795_ = _e212;
                                phi_2796_ = _e214;
                                phi_2797_ = select(true, false, _e214);
                            }
                            let _e220 = phi_2794_;
                            let _e222 = phi_2795_;
                            let _e224 = phi_2796_;
                            let _e226 = phi_2797_;
                            phi_2800_ = _e220;
                            phi_2801_ = _e222;
                            phi_2802_ = _e224;
                            phi_2803_ = _e201;
                            phi_2804_ = _e226;
                        }
                        let _e228 = phi_2800_;
                        let _e230 = phi_2801_;
                        let _e232 = phi_2802_;
                        let _e234 = phi_2803_;
                        let _e236 = phi_2804_;
                        local_1 = _e234;
                        local_3 = _e236;
                        continue;
                        continuing {
                            phi_2763_ = _e228;
                            phi_2764_ = _e230;
                            break if !(_e232);
                        }
                    }
                    let _e239 = local;
                    let _e241 = local_1;
                    let _e242 = select(_e241, false, _e239);
                    let _e244 = local_2;
                    let _e246 = local_3;
                    let _e251 = local_4;
                    phi_2826_ = select(_e251, 4294967295u, select(_e242, true, select(select(_e246, false, _e244), false, _e242)));
                }
                let _e254 = phi_2826_;
                if _e191 {
                } else {
                    break;
                }
                global_1.member[_e187.x] = _e254;
            }
            break;
        }
    }
    return;
}

fn function_1() {
    var phi_2844_: u32;
    var phi_2845_: f32;
    var phi_2846_: f32;
    var phi_2847_: f32;
    var phi_2848_: vec2<f32>;
    var phi_2871_: u32;
    var phi_2872_: f32;
    var phi_2873_: f32;
    var phi_2874_: f32;
    var phi_2875_: vec2<f32>;
    var local_5: f32;
    var local_6: f32;
    var phi_3036_: u32;
    var phi_3037_: f32;
    var phi_3038_: f32;
    var phi_3039_: f32;
    var phi_3040_: vec2<f32>;
    var phi_3063_: u32;
    var phi_3064_: f32;
    var phi_3065_: f32;
    var phi_3066_: f32;
    var phi_3067_: vec2<f32>;
    var local_7: f32;
    var local_8: f32;
    var phi_3228_: u32;
    var phi_3229_: f32;
    var phi_3230_: f32;
    var phi_3231_: f32;
    var phi_3232_: vec2<f32>;
    var phi_3255_: u32;
    var phi_3256_: f32;
    var phi_3257_: f32;
    var phi_3258_: f32;
    var phi_3259_: vec2<f32>;
    var local_9: f32;
    var local_10: f32;
    var phi_3420_: u32;
    var phi_3421_: f32;
    var phi_3422_: f32;
    var phi_3423_: f32;
    var phi_3424_: vec2<f32>;
    var phi_3447_: u32;
    var phi_3448_: f32;
    var phi_3449_: f32;
    var phi_3450_: f32;
    var phi_3451_: vec2<f32>;
    var local_11: f32;
    var local_12: f32;
    var phi_3698_: f32;
    var phi_3710_: f32;
    var phi_3722_: f32;
    var phi_489_: u32;
    var phi_492_: f32;
    var phi_490_: u32;
    var phi_493_: f32;
    var phi_3748_: u32;
    var phi_3749_: f32;
    var phi_3750_: f32;
    var phi_3751_: f32;
    var phi_3752_: vec2<f32>;
    var phi_3775_: u32;
    var phi_3776_: f32;
    var phi_3777_: f32;
    var phi_3778_: f32;
    var phi_3779_: vec2<f32>;
    var local_13: f32;
    var local_14: f32;
    var local_15: f32;
    var phi_3949_: u32;
    var phi_3950_: f32;
    var phi_3951_: f32;
    var phi_3952_: f32;
    var phi_3953_: vec2<f32>;
    var phi_3976_: u32;
    var phi_3977_: f32;
    var phi_3978_: f32;
    var phi_3979_: f32;
    var phi_3980_: vec2<f32>;
    var local_16: f32;
    var local_17: f32;
    var phi_4135_: bool;
    var phi_4156_: u32;
    var phi_4157_: f32;
    var phi_4158_: f32;
    var phi_4159_: f32;
    var phi_4160_: vec2<f32>;
    var phi_4183_: u32;
    var phi_4184_: f32;
    var phi_4185_: f32;
    var phi_4186_: f32;
    var phi_4187_: vec2<f32>;
    var local_18: f32;
    var local_19: f32;
    var phi_4342_: bool;
    var phi_4380_: f32;
    var phi_4392_: f32;
    var phi_4404_: f32;
    var phi_4425_: f32;
    var phi_4437_: f32;
    var phi_4449_: f32;
    var phi_645_: vec3<f32>;
    var phi_4466_: u32;
    var phi_4467_: f32;
    var phi_4468_: f32;
    var phi_4469_: f32;
    var phi_4470_: vec2<f32>;
    var phi_4493_: u32;
    var phi_4494_: f32;
    var phi_4495_: f32;
    var phi_4496_: f32;
    var phi_4497_: vec2<f32>;
    var local_20: f32;
    var local_21: f32;
    var phi_4689_: bool;
    var phi_4704_: bool;
    var phi_4845_: f32;
    var phi_4857_: f32;
    var phi_4869_: f32;
    var phi_4886_: vec2<f32>;
    var phi_4887_: u32;
    var phi_4914_: u32;
    var phi_4917_: f32;
    var phi_4918_: vec2<f32>;
    var phi_4919_: u32;
    var phi_4920_: bool;
    var phi_4921_: bool;
    var local_22: bool;
    var local_23: u32;
    var local_24: f32;
    var phi_4933_: f32;
    var local_25: bool;
    var phi_4999_: bool;
    var phi_5105_: f32;
    var phi_5117_: f32;
    var phi_5129_: f32;
    var phi_4953_: vec3<f32>;
    var phi_741_: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e189 = arrayLength((&global_3.member));
            let _e192 = global_2.member[0u];
            let _e195 = global_2.member[1u];
            if (_e187.x >= _e192) {
            } else {
                if (_e187.y >= _e195) {
                } else {
                    let _e202 = global_2.member[2u];
                    let _e203 = f32(_e202);
                    let _e206 = f32(_e195);
                    let _e213 = ((f32(_e187.x) - (0.5f * f32(_e192))) / _e206);
                    let _e214 = ((f32(_e187.y) - (0.5f * _e206)) / _e206);
                    let _e217 = global_2.member[3u];
                    switch bitcast<i32>(_e217) {
                        case 0: {
                            phi_4466_ = 0u;
                            phi_4467_ = 0f;
                            phi_4468_ = 0.5f;
                            phi_4469_ = 0f;
                            phi_4470_ = vec2<f32>(((_e213 * 3f) + (_e203 * 0.0003f)), ((_e214 * 3f) + (_e203 * -0.00020000001f)));
                            loop {
                                let _e1620 = phi_4466_;
                                let _e1622 = phi_4467_;
                                let _e1624 = phi_4468_;
                                let _e1626 = phi_4469_;
                                let _e1628 = phi_4470_;
                                local_20 = _e1626;
                                local_21 = _e1622;
                                let _e1629 = (_e1620 < 5u);
                                if _e1629 {
                                    let _e1633 = floor(_e1628.x);
                                    let _e1634 = floor(_e1628.y);
                                    let _e1641 = select(0i, select(select(i32(_e1633), i32(-2147483648), (_e1633 < -2147483600f)), 2147483647i, (_e1633 > 2147483500f)), (_e1633 == _e1633));
                                    let _e1648 = select(0i, select(select(i32(_e1634), i32(-2147483648), (_e1634 < -2147483600f)), 2147483647i, (_e1634 > 2147483500f)), (_e1634 == _e1634));
                                    let _e1649 = (_e1628.x - _e1633);
                                    let _e1653 = ((_e1649 * _e1649) * (3f - (2f * _e1649)));
                                    let _e1654 = (_e1628.y - _e1634);
                                    let _e1660 = (bitcast<u32>(_e1641) * 2376512323u);
                                    let _e1662 = (bitcast<u32>(_e1648) * 3625334849u);
                                    let _e1664 = ((7u + _e1620) * 2654435769u);
                                    let _e1665 = ((_e1660 ^ _e1662) ^ _e1664);
                                    let _e1669 = ((_e1665 ^ (_e1665 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1673 = ((_e1669 ^ (_e1669 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1680 = (f32(((_e1673 ^ (_e1673 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e1683 = (bitcast<u32>((_e1641 + 1i)) * 2376512323u);
                                    let _e1685 = ((_e1683 ^ _e1662) ^ _e1664);
                                    let _e1689 = ((_e1685 ^ (_e1685 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1693 = ((_e1689 ^ (_e1689 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1703 = (bitcast<u32>((_e1648 + 1i)) * 3625334849u);
                                    let _e1705 = ((_e1660 ^ _e1703) ^ _e1664);
                                    let _e1709 = ((_e1705 ^ (_e1705 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1713 = ((_e1709 ^ (_e1709 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1720 = (f32(((_e1713 ^ (_e1713 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e1722 = ((_e1683 ^ _e1703) ^ _e1664);
                                    let _e1726 = ((_e1722 ^ (_e1722 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1730 = ((_e1726 ^ (_e1726 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1740 = (_e1680 + (((f32(((_e1693 ^ (_e1693 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1680) * _e1653));
                                    phi_4493_ = (_e1620 + 1u);
                                    phi_4494_ = (_e1622 + _e1624);
                                    phi_4495_ = (_e1624 * 0.5f);
                                    phi_4496_ = (_e1626 + ((_e1740 + (((_e1720 + (((f32(((_e1730 ^ (_e1730 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1720) * _e1653)) - _e1740) * ((_e1654 * _e1654) * (3f - (2f * _e1654))))) * _e1624));
                                    phi_4497_ = vec2<f32>(((_e1628.x * 2.03f) + 17.13f), ((_e1628.y * 2.01f) - 9.71f));
                                } else {
                                    phi_4493_ = u32();
                                    phi_4494_ = f32();
                                    phi_4495_ = f32();
                                    phi_4496_ = f32();
                                    phi_4497_ = vec2<f32>();
                                }
                                let _e1758 = phi_4493_;
                                let _e1760 = phi_4494_;
                                let _e1762 = phi_4495_;
                                let _e1764 = phi_4496_;
                                let _e1766 = phi_4497_;
                                continue;
                                continuing {
                                    phi_4466_ = _e1758;
                                    phi_4467_ = _e1760;
                                    phi_4468_ = _e1762;
                                    phi_4469_ = _e1764;
                                    phi_4470_ = _e1766;
                                    break if !(_e1629);
                                }
                            }
                            let _e1769 = local_20;
                            let _e1771 = local_21;
                            let _e1776 = (_e213 - (0.25f * cos((_e203 * 0.00070000003f))));
                            let _e1781 = (sqrt(((_e1776 * _e1776) + (_e214 * _e214))) - 0.35f);
                            let _e1788 = (abs((_e213 + (0.25f * sin((_e203 * 0.0005f))))) - 0.28f);
                            let _e1789 = (abs(_e214) - 0.18f);
                            let _e1791 = select(0f, _e1788, (_e1788 > 0f));
                            let _e1793 = select(0f, _e1789, (_e1789 > 0f));
                            if (_e1788 != _e1788) {
                                phi_4689_ = true;
                            } else {
                                phi_4689_ = (_e1789 >= _e1788);
                            }
                            let _e1801 = phi_4689_;
                            let _e1802 = select(_e1788, _e1789, _e1801);
                            if (_e1802 != _e1802) {
                                phi_4704_ = true;
                            } else {
                                phi_4704_ = (0f <= _e1802);
                            }
                            let _e1806 = phi_4704_;
                            let _e1809 = ((sqrt(((_e1791 * _e1791) + (_e1793 * _e1793))) + select(_e1802, 0f, _e1806)) - 0.05f);
                            let _e1812 = (0.5f + ((_e1809 - _e1781) * 2f));
                            let _e1814 = select(_e1812, 0f, (_e1812 < 0f));
                            let _e1816 = select(_e1814, 1f, (_e1814 > 1f));
                            let _e1826 = exp((-6f * abs(((_e1809 + ((_e1781 - _e1809) * _e1816)) - ((0.25f * _e1816) * (1f - _e1816))))));
                            let _e1828 = ((_e1769 / _e1771) + (_e203 * 0.000050000002f));
                            let _e1841 = (vec3<f32>(0.5f, 0.5f, 0.5f) + vec3<f32>((0.5f * cos((_e1828 * 6.2831855f))), (0.5f * cos(((_e1828 + 0.33f) * 6.2831855f))), (0.5f * cos(((_e1828 + 0.67f) * 6.2831855f)))));
                            let _e1843 = (0.25f + (0.75f * _e1826));
                            let _e1851 = ((_e1826 * _e1826) * 0.6f);
                            let _e1852 = ((_e1841.x * _e1843) + _e1851);
                            let _e1853 = ((_e1841.y * _e1843) + _e1851);
                            let _e1854 = ((_e1841.z * _e1843) + _e1851);
                            let _e1876 = ((_e1852 * ((2.51f * _e1852) + 0.03f)) / ((_e1852 * ((2.43f * _e1852) + 0.59f)) + 0.14f));
                            let _e1877 = ((_e1853 * ((2.51f * _e1853) + 0.03f)) / ((_e1853 * ((2.43f * _e1853) + 0.59f)) + 0.14f));
                            let _e1878 = ((_e1854 * ((2.51f * _e1854) + 0.03f)) / ((_e1854 * ((2.43f * _e1854) + 0.59f)) + 0.14f));
                            let _e1880 = select(0f, _e1876, (_e1876 > 0f));
                            let _e1882 = select(0f, _e1877, (_e1877 > 0f));
                            let _e1884 = select(0f, _e1878, (_e1878 > 0f));
                            let _e1886 = select(1f, _e1880, (_e1880 < 1f));
                            let _e1888 = select(1f, _e1882, (_e1882 < 1f));
                            let _e1890 = select(1f, _e1884, (_e1884 < 1f));
                            if (_e1886 <= 0.0031308f) {
                                phi_4845_ = (12.92f * _e1886);
                            } else {
                                phi_4845_ = ((1.055f * pow(_e1886, 0.41666666f)) - 0.055f);
                            }
                            let _e1897 = phi_4845_;
                            if (_e1888 <= 0.0031308f) {
                                phi_4857_ = (12.92f * _e1888);
                            } else {
                                phi_4857_ = ((1.055f * pow(_e1888, 0.41666666f)) - 0.055f);
                            }
                            let _e1904 = phi_4857_;
                            if (_e1890 <= 0.0031308f) {
                                phi_4869_ = (12.92f * _e1890);
                            } else {
                                phi_4869_ = ((1.055f * pow(_e1890, 0.41666666f)) - 0.055f);
                            }
                            let _e1911 = phi_4869_;
                            phi_741_ = vec3<f32>(_e1897, _e1904, _e1911);
                            break;
                        }
                        case 1: {
                            phi_489_ = 0u;
                            phi_492_ = 1000000000f;
                            loop {
                                let _e954 = phi_489_;
                                let _e956 = phi_492_;
                                local_15 = _e956;
                                let _e957 = (_e954 < 5u);
                                if _e957 {
                                    let _e958 = f32(_e954);
                                    let _e963 = (((_e203 * 0.001f) * (0.3f + (0.07f * _e958))) + (_e958 * 2.39996f));
                                    let _e974 = (0.13f + (0.05f * sin(_e958)));
                                    let _e977 = (_e213 - (cos(_e963) * _e974));
                                    let _e978 = (_e214 - (sin(_e963) * _e974));
                                    let _e983 = (sqrt(((_e977 * _e977) + (_e978 * _e978))) - (0.16f + (0.06f * sin(((_e203 * 0.00090000004f) + (_e958 * 1.7f))))));
                                    let _e986 = (0.5f + ((_e983 - _e956) * 2.2727273f));
                                    let _e988 = select(_e986, 0f, (_e986 < 0f));
                                    let _e990 = select(_e988, 1f, (_e988 > 1f));
                                    phi_490_ = (_e954 + 1u);
                                    phi_493_ = ((_e983 + ((_e956 - _e983) * _e990)) - ((0.22f * _e990) * (1f - _e990)));
                                } else {
                                    phi_490_ = u32();
                                    phi_493_ = f32();
                                }
                                let _e1000 = phi_490_;
                                let _e1002 = phi_493_;
                                continue;
                                continuing {
                                    phi_489_ = _e1000;
                                    phi_492_ = _e1002;
                                    break if !(_e957);
                                }
                            }
                            phi_3748_ = 0u;
                            phi_3749_ = 0f;
                            phi_3750_ = 0.5f;
                            phi_3751_ = 0f;
                            phi_3752_ = vec2<f32>(((_e213 * 9f) + (_e203 * 0.0006f)), ((_e214 * 9f) + (_e203 * -0.00040000002f)));
                            loop {
                                let _e1012 = phi_3748_;
                                let _e1014 = phi_3749_;
                                let _e1016 = phi_3750_;
                                let _e1018 = phi_3751_;
                                let _e1020 = phi_3752_;
                                local_13 = _e1018;
                                local_14 = _e1014;
                                let _e1021 = (_e1012 < 4u);
                                if _e1021 {
                                    let _e1025 = floor(_e1020.x);
                                    let _e1026 = floor(_e1020.y);
                                    let _e1033 = select(0i, select(select(i32(_e1025), i32(-2147483648), (_e1025 < -2147483600f)), 2147483647i, (_e1025 > 2147483500f)), (_e1025 == _e1025));
                                    let _e1040 = select(0i, select(select(i32(_e1026), i32(-2147483648), (_e1026 < -2147483600f)), 2147483647i, (_e1026 > 2147483500f)), (_e1026 == _e1026));
                                    let _e1041 = (_e1020.x - _e1025);
                                    let _e1045 = ((_e1041 * _e1041) * (3f - (2f * _e1041)));
                                    let _e1046 = (_e1020.y - _e1026);
                                    let _e1052 = (bitcast<u32>(_e1033) * 2376512323u);
                                    let _e1054 = (bitcast<u32>(_e1040) * 3625334849u);
                                    let _e1056 = ((21u + _e1012) * 2654435769u);
                                    let _e1057 = ((_e1052 ^ _e1054) ^ _e1056);
                                    let _e1061 = ((_e1057 ^ (_e1057 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1065 = ((_e1061 ^ (_e1061 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1072 = (f32(((_e1065 ^ (_e1065 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e1075 = (bitcast<u32>((_e1033 + 1i)) * 2376512323u);
                                    let _e1077 = ((_e1075 ^ _e1054) ^ _e1056);
                                    let _e1081 = ((_e1077 ^ (_e1077 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1085 = ((_e1081 ^ (_e1081 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1095 = (bitcast<u32>((_e1040 + 1i)) * 3625334849u);
                                    let _e1097 = ((_e1052 ^ _e1095) ^ _e1056);
                                    let _e1101 = ((_e1097 ^ (_e1097 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1105 = ((_e1101 ^ (_e1101 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1112 = (f32(((_e1105 ^ (_e1105 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e1114 = ((_e1075 ^ _e1095) ^ _e1056);
                                    let _e1118 = ((_e1114 ^ (_e1114 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1122 = ((_e1118 ^ (_e1118 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1132 = (_e1072 + (((f32(((_e1085 ^ (_e1085 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1072) * _e1045));
                                    phi_3775_ = (_e1012 + 1u);
                                    phi_3776_ = (_e1014 + _e1016);
                                    phi_3777_ = (_e1016 * 0.5f);
                                    phi_3778_ = (_e1018 + ((_e1132 + (((_e1112 + (((f32(((_e1122 ^ (_e1122 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1112) * _e1045)) - _e1132) * ((_e1046 * _e1046) * (3f - (2f * _e1046))))) * _e1016));
                                    phi_3779_ = vec2<f32>(((_e1020.x * 2.03f) + 17.13f), ((_e1020.y * 2.01f) - 9.71f));
                                } else {
                                    phi_3775_ = u32();
                                    phi_3776_ = f32();
                                    phi_3777_ = f32();
                                    phi_3778_ = f32();
                                    phi_3779_ = vec2<f32>();
                                }
                                let _e1150 = phi_3775_;
                                let _e1152 = phi_3776_;
                                let _e1154 = phi_3777_;
                                let _e1156 = phi_3778_;
                                let _e1158 = phi_3779_;
                                continue;
                                continuing {
                                    phi_3748_ = _e1150;
                                    phi_3749_ = _e1152;
                                    phi_3750_ = _e1154;
                                    phi_3751_ = _e1156;
                                    phi_3752_ = _e1158;
                                    break if !(_e1021);
                                }
                            }
                            let _e1161 = local_13;
                            let _e1163 = local_14;
                            let _e1168 = local_15;
                            let _e1169 = (_e1168 + (((_e1161 / _e1163) - 0.5f) * 0.04f));
                            let _e1176 = (_e213 - (cos((_e203 * 0.00023000002f)) * 0.06f));
                            let _e1177 = (_e214 - (sin((_e203 * 0.00031f)) * 0.06f));
                            let _e1186 = (sqrt(((_e1176 * _e1176) + (_e1177 * _e1177))) - (0.07f + (0.012f * sin((_e203 * 0.0013f)))));
                            phi_3949_ = 0u;
                            phi_3950_ = 0f;
                            phi_3951_ = 0.5f;
                            phi_3952_ = 0f;
                            phi_3953_ = vec2<f32>((_e213 * 2f), (_e214 * 2f));
                            loop {
                                let _e1191 = phi_3949_;
                                let _e1193 = phi_3950_;
                                let _e1195 = phi_3951_;
                                let _e1197 = phi_3952_;
                                let _e1199 = phi_3953_;
                                local_16 = _e1197;
                                local_17 = _e1193;
                                let _e1200 = (_e1191 < 3u);
                                if _e1200 {
                                    let _e1204 = floor(_e1199.x);
                                    let _e1205 = floor(_e1199.y);
                                    let _e1212 = select(0i, select(select(i32(_e1204), i32(-2147483648), (_e1204 < -2147483600f)), 2147483647i, (_e1204 > 2147483500f)), (_e1204 == _e1204));
                                    let _e1219 = select(0i, select(select(i32(_e1205), i32(-2147483648), (_e1205 < -2147483600f)), 2147483647i, (_e1205 > 2147483500f)), (_e1205 == _e1205));
                                    let _e1220 = (_e1199.x - _e1204);
                                    let _e1224 = ((_e1220 * _e1220) * (3f - (2f * _e1220)));
                                    let _e1225 = (_e1199.y - _e1205);
                                    let _e1231 = (bitcast<u32>(_e1212) * 2376512323u);
                                    let _e1233 = (bitcast<u32>(_e1219) * 3625334849u);
                                    let _e1235 = ((5u + _e1191) * 2654435769u);
                                    let _e1236 = ((_e1231 ^ _e1233) ^ _e1235);
                                    let _e1240 = ((_e1236 ^ (_e1236 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1244 = ((_e1240 ^ (_e1240 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1251 = (f32(((_e1244 ^ (_e1244 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e1254 = (bitcast<u32>((_e1212 + 1i)) * 2376512323u);
                                    let _e1256 = ((_e1254 ^ _e1233) ^ _e1235);
                                    let _e1260 = ((_e1256 ^ (_e1256 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1264 = ((_e1260 ^ (_e1260 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1274 = (bitcast<u32>((_e1219 + 1i)) * 3625334849u);
                                    let _e1276 = ((_e1231 ^ _e1274) ^ _e1235);
                                    let _e1280 = ((_e1276 ^ (_e1276 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1284 = ((_e1280 ^ (_e1280 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1291 = (f32(((_e1284 ^ (_e1284 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e1293 = ((_e1254 ^ _e1274) ^ _e1235);
                                    let _e1297 = ((_e1293 ^ (_e1293 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e1301 = ((_e1297 ^ (_e1297 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e1311 = (_e1251 + (((f32(((_e1264 ^ (_e1264 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1251) * _e1224));
                                    phi_3976_ = (_e1191 + 1u);
                                    phi_3977_ = (_e1193 + _e1195);
                                    phi_3978_ = (_e1195 * 0.5f);
                                    phi_3979_ = (_e1197 + ((_e1311 + (((_e1291 + (((f32(((_e1301 ^ (_e1301 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1291) * _e1224)) - _e1311) * ((_e1225 * _e1225) * (3f - (2f * _e1225))))) * _e1195));
                                    phi_3980_ = vec2<f32>(((_e1199.x * 2.03f) + 17.13f), ((_e1199.y * 2.01f) - 9.71f));
                                } else {
                                    phi_3976_ = u32();
                                    phi_3977_ = f32();
                                    phi_3978_ = f32();
                                    phi_3979_ = f32();
                                    phi_3980_ = vec2<f32>();
                                }
                                let _e1329 = phi_3976_;
                                let _e1331 = phi_3977_;
                                let _e1333 = phi_3978_;
                                let _e1335 = phi_3979_;
                                let _e1337 = phi_3980_;
                                continue;
                                continuing {
                                    phi_3949_ = _e1329;
                                    phi_3950_ = _e1331;
                                    phi_3951_ = _e1333;
                                    phi_3952_ = _e1335;
                                    phi_3953_ = _e1337;
                                    break if !(_e1200);
                                }
                            }
                            let _e1340 = local_16;
                            let _e1342 = local_17;
                            let _e1344 = ((_e1340 / _e1342) * 0.03f);
                            if (_e1169 > 0f) {
                                let _e1581 = exp((-14f * _e1169));
                                let _e1585 = ((0.02f + _e1344) + (_e1581 * 0.105000004f));
                                let _e1586 = ((0.03f + _e1344) + (_e1581 * 0.315f));
                                let _e1587 = ((0.05f + _e1344) + (_e1581 * 0.21000001f));
                                if (_e1585 <= 0.0031308f) {
                                    phi_4425_ = (12.92f * _e1585);
                                } else {
                                    phi_4425_ = ((1.055f * pow(_e1585, 0.41666666f)) - 0.055f);
                                }
                                let _e1594 = phi_4425_;
                                if (_e1586 <= 0.0031308f) {
                                    phi_4437_ = (12.92f * _e1586);
                                } else {
                                    phi_4437_ = ((1.055f * pow(_e1586, 0.41666666f)) - 0.055f);
                                }
                                let _e1601 = phi_4437_;
                                if (_e1587 <= 0.0031308f) {
                                    phi_4449_ = (12.92f * _e1587);
                                } else {
                                    phi_4449_ = ((1.055f * pow(_e1587, 0.41666666f)) - 0.055f);
                                }
                                let _e1608 = phi_4449_;
                                phi_645_ = vec3<f32>(_e1594, _e1601, _e1608);
                            } else {
                                let _e1349 = -(_e1169);
                                if (_e1349 != _e1349) {
                                    phi_4135_ = true;
                                } else {
                                    phi_4135_ = (0.25f <= _e1349);
                                }
                                let _e1353 = phi_4135_;
                                let _e1354 = select(_e1349, 0.25f, _e1353);
                                phi_4156_ = 0u;
                                phi_4157_ = 0f;
                                phi_4158_ = 0.5f;
                                phi_4159_ = 0f;
                                phi_4160_ = vec2<f32>(((_e213 * 14f) + (_e203 * 0.00020000001f)), ((_e214 * 14f) + (_e203 * 0.00015f)));
                                loop {
                                    let _e1364 = phi_4156_;
                                    let _e1366 = phi_4157_;
                                    let _e1368 = phi_4158_;
                                    let _e1370 = phi_4159_;
                                    let _e1372 = phi_4160_;
                                    local_18 = _e1370;
                                    local_19 = _e1366;
                                    let _e1373 = (_e1364 < 4u);
                                    if _e1373 {
                                        let _e1377 = floor(_e1372.x);
                                        let _e1378 = floor(_e1372.y);
                                        let _e1385 = select(0i, select(select(i32(_e1377), i32(-2147483648), (_e1377 < -2147483600f)), 2147483647i, (_e1377 > 2147483500f)), (_e1377 == _e1377));
                                        let _e1392 = select(0i, select(select(i32(_e1378), i32(-2147483648), (_e1378 < -2147483600f)), 2147483647i, (_e1378 > 2147483500f)), (_e1378 == _e1378));
                                        let _e1393 = (_e1372.x - _e1377);
                                        let _e1397 = ((_e1393 * _e1393) * (3f - (2f * _e1393)));
                                        let _e1398 = (_e1372.y - _e1378);
                                        let _e1404 = (bitcast<u32>(_e1385) * 2376512323u);
                                        let _e1406 = (bitcast<u32>(_e1392) * 3625334849u);
                                        let _e1408 = ((33u + _e1364) * 2654435769u);
                                        let _e1409 = ((_e1404 ^ _e1406) ^ _e1408);
                                        let _e1413 = ((_e1409 ^ (_e1409 >> bitcast<u32>(16i))) * 2146121005u);
                                        let _e1417 = ((_e1413 ^ (_e1413 >> bitcast<u32>(15i))) * 2221713035u);
                                        let _e1424 = (f32(((_e1417 ^ (_e1417 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                        let _e1427 = (bitcast<u32>((_e1385 + 1i)) * 2376512323u);
                                        let _e1429 = ((_e1427 ^ _e1406) ^ _e1408);
                                        let _e1433 = ((_e1429 ^ (_e1429 >> bitcast<u32>(16i))) * 2146121005u);
                                        let _e1437 = ((_e1433 ^ (_e1433 >> bitcast<u32>(15i))) * 2221713035u);
                                        let _e1447 = (bitcast<u32>((_e1392 + 1i)) * 3625334849u);
                                        let _e1449 = ((_e1404 ^ _e1447) ^ _e1408);
                                        let _e1453 = ((_e1449 ^ (_e1449 >> bitcast<u32>(16i))) * 2146121005u);
                                        let _e1457 = ((_e1453 ^ (_e1453 >> bitcast<u32>(15i))) * 2221713035u);
                                        let _e1464 = (f32(((_e1457 ^ (_e1457 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                        let _e1466 = ((_e1427 ^ _e1447) ^ _e1408);
                                        let _e1470 = ((_e1466 ^ (_e1466 >> bitcast<u32>(16i))) * 2146121005u);
                                        let _e1474 = ((_e1470 ^ (_e1470 >> bitcast<u32>(15i))) * 2221713035u);
                                        let _e1484 = (_e1424 + (((f32(((_e1437 ^ (_e1437 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1424) * _e1397));
                                        phi_4183_ = (_e1364 + 1u);
                                        phi_4184_ = (_e1366 + _e1368);
                                        phi_4185_ = (_e1368 * 0.5f);
                                        phi_4186_ = (_e1370 + ((_e1484 + (((_e1464 + (((f32(((_e1474 ^ (_e1474 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e1464) * _e1397)) - _e1484) * ((_e1398 * _e1398) * (3f - (2f * _e1398))))) * _e1368));
                                        phi_4187_ = vec2<f32>(((_e1372.x * 2.03f) + 17.13f), ((_e1372.y * 2.01f) - 9.71f));
                                    } else {
                                        phi_4183_ = u32();
                                        phi_4184_ = f32();
                                        phi_4185_ = f32();
                                        phi_4186_ = f32();
                                        phi_4187_ = vec2<f32>();
                                    }
                                    let _e1502 = phi_4183_;
                                    let _e1504 = phi_4184_;
                                    let _e1506 = phi_4185_;
                                    let _e1508 = phi_4186_;
                                    let _e1510 = phi_4187_;
                                    continue;
                                    continuing {
                                        phi_4156_ = _e1502;
                                        phi_4157_ = _e1504;
                                        phi_4158_ = _e1506;
                                        phi_4159_ = _e1508;
                                        phi_4160_ = _e1510;
                                        break if !(_e1373);
                                    }
                                }
                                let _e1513 = local_18;
                                let _e1515 = local_19;
                                let _e1516 = (_e1513 / _e1515);
                                let _e1518 = (0.5f + (_e1354 * 2f));
                                let _e1529 = pow((1f - (_e1354 * 4f)), 3f);
                                if (_e1186 != _e1186) {
                                    phi_4342_ = true;
                                } else {
                                    phi_4342_ = (0f >= _e1186);
                                }
                                let _e1533 = phi_4342_;
                                let _e1536 = exp((-60f * select(_e1186, 0f, _e1533)));
                                let _e1546 = ((((0.1f * _e1518) + (0.05f * _e1516)) + (_e1529 * 0.125f)) + (0.45f * _e1536));
                                let _e1547 = ((((0.45f * _e1518) + (0.2f * _e1516)) + (_e1529 * 0.25f)) + (0.25f * _e1536));
                                let _e1548 = ((((0.3f * _e1518) + (0.12f * _e1516)) + (_e1529 * 0.175f)) + (0.5f * _e1536));
                                let _e1554 = (vec3<f32>(_e1546, _e1547, _e1548) / vec3<f32>((1f + _e1546), (1f + _e1547), (1f + _e1548)));
                                if (_e1554.x <= 0.0031308f) {
                                    phi_4380_ = (12.92f * _e1554.x);
                                } else {
                                    phi_4380_ = ((1.055f * pow(_e1554.x, 0.41666666f)) - 0.055f);
                                }
                                let _e1562 = phi_4380_;
                                if (_e1554.y <= 0.0031308f) {
                                    phi_4392_ = (12.92f * _e1554.y);
                                } else {
                                    phi_4392_ = ((1.055f * pow(_e1554.y, 0.41666666f)) - 0.055f);
                                }
                                let _e1570 = phi_4392_;
                                if (_e1554.z <= 0.0031308f) {
                                    phi_4404_ = (12.92f * _e1554.z);
                                } else {
                                    phi_4404_ = ((1.055f * pow(_e1554.z, 0.41666666f)) - 0.055f);
                                }
                                let _e1578 = phi_4404_;
                                phi_645_ = vec3<f32>(_e1562, _e1570, _e1578);
                            }
                            let _e1611 = phi_645_;
                            phi_741_ = _e1611;
                            break;
                        }
                        case 2: {
                            let _e219 = (_e203 * 0.000050000002f);
                            let _e220 = (_e203 * 0.000010000001f);
                            let _e223 = ((_e213 * 2f) + _e219);
                            let _e224 = ((_e214 * 2f) + _e220);
                            phi_2844_ = 0u;
                            phi_2845_ = 0f;
                            phi_2846_ = 0.5f;
                            phi_2847_ = 0f;
                            phi_2848_ = vec2<f32>(_e223, _e224);
                            loop {
                                let _e227 = phi_2844_;
                                let _e229 = phi_2845_;
                                let _e231 = phi_2846_;
                                let _e233 = phi_2847_;
                                let _e235 = phi_2848_;
                                local_5 = _e233;
                                local_6 = _e229;
                                let _e236 = (_e227 < 4u);
                                if _e236 {
                                    let _e240 = floor(_e235.x);
                                    let _e241 = floor(_e235.y);
                                    let _e248 = select(0i, select(select(i32(_e240), i32(-2147483648), (_e240 < -2147483600f)), 2147483647i, (_e240 > 2147483500f)), (_e240 == _e240));
                                    let _e255 = select(0i, select(select(i32(_e241), i32(-2147483648), (_e241 < -2147483600f)), 2147483647i, (_e241 > 2147483500f)), (_e241 == _e241));
                                    let _e256 = (_e235.x - _e240);
                                    let _e260 = ((_e256 * _e256) * (3f - (2f * _e256)));
                                    let _e261 = (_e235.y - _e241);
                                    let _e267 = (bitcast<u32>(_e248) * 2376512323u);
                                    let _e269 = (bitcast<u32>(_e255) * 3625334849u);
                                    let _e271 = ((11u + _e227) * 2654435769u);
                                    let _e272 = ((_e267 ^ _e269) ^ _e271);
                                    let _e276 = ((_e272 ^ (_e272 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e280 = ((_e276 ^ (_e276 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e287 = (f32(((_e280 ^ (_e280 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e290 = (bitcast<u32>((_e248 + 1i)) * 2376512323u);
                                    let _e292 = ((_e290 ^ _e269) ^ _e271);
                                    let _e296 = ((_e292 ^ (_e292 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e300 = ((_e296 ^ (_e296 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e310 = (bitcast<u32>((_e255 + 1i)) * 3625334849u);
                                    let _e312 = ((_e267 ^ _e310) ^ _e271);
                                    let _e316 = ((_e312 ^ (_e312 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e320 = ((_e316 ^ (_e316 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e327 = (f32(((_e320 ^ (_e320 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e329 = ((_e290 ^ _e310) ^ _e271);
                                    let _e333 = ((_e329 ^ (_e329 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e337 = ((_e333 ^ (_e333 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e347 = (_e287 + (((f32(((_e300 ^ (_e300 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e287) * _e260));
                                    phi_2871_ = (_e227 + 1u);
                                    phi_2872_ = (_e229 + _e231);
                                    phi_2873_ = (_e231 * 0.5f);
                                    phi_2874_ = (_e233 + ((_e347 + (((_e327 + (((f32(((_e337 ^ (_e337 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e327) * _e260)) - _e347) * ((_e261 * _e261) * (3f - (2f * _e261))))) * _e231));
                                    phi_2875_ = vec2<f32>(((_e235.x * 2.03f) + 17.13f), ((_e235.y * 2.01f) - 9.71f));
                                } else {
                                    phi_2871_ = u32();
                                    phi_2872_ = f32();
                                    phi_2873_ = f32();
                                    phi_2874_ = f32();
                                    phi_2875_ = vec2<f32>();
                                }
                                let _e365 = phi_2871_;
                                let _e367 = phi_2872_;
                                let _e369 = phi_2873_;
                                let _e371 = phi_2874_;
                                let _e373 = phi_2875_;
                                continue;
                                continuing {
                                    phi_2844_ = _e365;
                                    phi_2845_ = _e367;
                                    phi_2846_ = _e369;
                                    phi_2847_ = _e371;
                                    phi_2848_ = _e373;
                                    break if !(_e236);
                                }
                            }
                            let _e376 = local_5;
                            let _e378 = local_6;
                            phi_3036_ = 0u;
                            phi_3037_ = 0f;
                            phi_3038_ = 0.5f;
                            phi_3039_ = 0f;
                            phi_3040_ = vec2<f32>((_e223 + 5.2f), (_e224 + 1.3f));
                            loop {
                                let _e384 = phi_3036_;
                                let _e386 = phi_3037_;
                                let _e388 = phi_3038_;
                                let _e390 = phi_3039_;
                                let _e392 = phi_3040_;
                                local_7 = _e390;
                                local_8 = _e386;
                                let _e393 = (_e384 < 4u);
                                if _e393 {
                                    let _e397 = floor(_e392.x);
                                    let _e398 = floor(_e392.y);
                                    let _e405 = select(0i, select(select(i32(_e397), i32(-2147483648), (_e397 < -2147483600f)), 2147483647i, (_e397 > 2147483500f)), (_e397 == _e397));
                                    let _e412 = select(0i, select(select(i32(_e398), i32(-2147483648), (_e398 < -2147483600f)), 2147483647i, (_e398 > 2147483500f)), (_e398 == _e398));
                                    let _e413 = (_e392.x - _e397);
                                    let _e417 = ((_e413 * _e413) * (3f - (2f * _e413)));
                                    let _e418 = (_e392.y - _e398);
                                    let _e424 = (bitcast<u32>(_e405) * 2376512323u);
                                    let _e426 = (bitcast<u32>(_e412) * 3625334849u);
                                    let _e428 = ((12u + _e384) * 2654435769u);
                                    let _e429 = ((_e424 ^ _e426) ^ _e428);
                                    let _e433 = ((_e429 ^ (_e429 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e437 = ((_e433 ^ (_e433 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e444 = (f32(((_e437 ^ (_e437 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e447 = (bitcast<u32>((_e405 + 1i)) * 2376512323u);
                                    let _e449 = ((_e447 ^ _e426) ^ _e428);
                                    let _e453 = ((_e449 ^ (_e449 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e457 = ((_e453 ^ (_e453 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e467 = (bitcast<u32>((_e412 + 1i)) * 3625334849u);
                                    let _e469 = ((_e424 ^ _e467) ^ _e428);
                                    let _e473 = ((_e469 ^ (_e469 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e477 = ((_e473 ^ (_e473 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e484 = (f32(((_e477 ^ (_e477 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e486 = ((_e447 ^ _e467) ^ _e428);
                                    let _e490 = ((_e486 ^ (_e486 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e494 = ((_e490 ^ (_e490 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e504 = (_e444 + (((f32(((_e457 ^ (_e457 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e444) * _e417));
                                    phi_3063_ = (_e384 + 1u);
                                    phi_3064_ = (_e386 + _e388);
                                    phi_3065_ = (_e388 * 0.5f);
                                    phi_3066_ = (_e390 + ((_e504 + (((_e484 + (((f32(((_e494 ^ (_e494 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e484) * _e417)) - _e504) * ((_e418 * _e418) * (3f - (2f * _e418))))) * _e388));
                                    phi_3067_ = vec2<f32>(((_e392.x * 2.03f) + 17.13f), ((_e392.y * 2.01f) - 9.71f));
                                } else {
                                    phi_3063_ = u32();
                                    phi_3064_ = f32();
                                    phi_3065_ = f32();
                                    phi_3066_ = f32();
                                    phi_3067_ = vec2<f32>();
                                }
                                let _e522 = phi_3063_;
                                let _e524 = phi_3064_;
                                let _e526 = phi_3065_;
                                let _e528 = phi_3066_;
                                let _e530 = phi_3067_;
                                continue;
                                continuing {
                                    phi_3036_ = _e522;
                                    phi_3037_ = _e524;
                                    phi_3038_ = _e526;
                                    phi_3039_ = _e528;
                                    phi_3040_ = _e530;
                                    break if !(_e393);
                                }
                            }
                            let _e533 = local_7;
                            let _e535 = local_8;
                            let _e539 = ((_e376 / _e378) * 1.5f);
                            let _e540 = ((_e533 / _e535) * 1.5f);
                            phi_3228_ = 0u;
                            phi_3229_ = 0f;
                            phi_3230_ = 0.5f;
                            phi_3231_ = 0f;
                            phi_3232_ = vec2<f32>((((_e213 * 3f) + _e539) + _e219), (((_e214 * 3f) + _e540) + _e220));
                            loop {
                                let _e547 = phi_3228_;
                                let _e549 = phi_3229_;
                                let _e551 = phi_3230_;
                                let _e553 = phi_3231_;
                                let _e555 = phi_3232_;
                                local_9 = _e553;
                                local_10 = _e549;
                                let _e556 = (_e547 < 5u);
                                if _e556 {
                                    let _e560 = floor(_e555.x);
                                    let _e561 = floor(_e555.y);
                                    let _e568 = select(0i, select(select(i32(_e560), i32(-2147483648), (_e560 < -2147483600f)), 2147483647i, (_e560 > 2147483500f)), (_e560 == _e560));
                                    let _e575 = select(0i, select(select(i32(_e561), i32(-2147483648), (_e561 < -2147483600f)), 2147483647i, (_e561 > 2147483500f)), (_e561 == _e561));
                                    let _e576 = (_e555.x - _e560);
                                    let _e580 = ((_e576 * _e576) * (3f - (2f * _e576)));
                                    let _e581 = (_e555.y - _e561);
                                    let _e587 = (bitcast<u32>(_e568) * 2376512323u);
                                    let _e589 = (bitcast<u32>(_e575) * 3625334849u);
                                    let _e591 = ((13u + _e547) * 2654435769u);
                                    let _e592 = ((_e587 ^ _e589) ^ _e591);
                                    let _e596 = ((_e592 ^ (_e592 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e600 = ((_e596 ^ (_e596 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e607 = (f32(((_e600 ^ (_e600 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e610 = (bitcast<u32>((_e568 + 1i)) * 2376512323u);
                                    let _e612 = ((_e610 ^ _e589) ^ _e591);
                                    let _e616 = ((_e612 ^ (_e612 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e620 = ((_e616 ^ (_e616 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e630 = (bitcast<u32>((_e575 + 1i)) * 3625334849u);
                                    let _e632 = ((_e587 ^ _e630) ^ _e591);
                                    let _e636 = ((_e632 ^ (_e632 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e640 = ((_e636 ^ (_e636 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e647 = (f32(((_e640 ^ (_e640 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e649 = ((_e610 ^ _e630) ^ _e591);
                                    let _e653 = ((_e649 ^ (_e649 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e657 = ((_e653 ^ (_e653 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e667 = (_e607 + (((f32(((_e620 ^ (_e620 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e607) * _e580));
                                    phi_3255_ = (_e547 + 1u);
                                    phi_3256_ = (_e549 + _e551);
                                    phi_3257_ = (_e551 * 0.5f);
                                    phi_3258_ = (_e553 + ((_e667 + (((_e647 + (((f32(((_e657 ^ (_e657 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e647) * _e580)) - _e667) * ((_e581 * _e581) * (3f - (2f * _e581))))) * _e551));
                                    phi_3259_ = vec2<f32>(((_e555.x * 2.03f) + 17.13f), ((_e555.y * 2.01f) - 9.71f));
                                } else {
                                    phi_3255_ = u32();
                                    phi_3256_ = f32();
                                    phi_3257_ = f32();
                                    phi_3258_ = f32();
                                    phi_3259_ = vec2<f32>();
                                }
                                let _e685 = phi_3255_;
                                let _e687 = phi_3256_;
                                let _e689 = phi_3257_;
                                let _e691 = phi_3258_;
                                let _e693 = phi_3259_;
                                continue;
                                continuing {
                                    phi_3228_ = _e685;
                                    phi_3229_ = _e687;
                                    phi_3230_ = _e689;
                                    phi_3231_ = _e691;
                                    phi_3232_ = _e693;
                                    break if !(_e556);
                                }
                            }
                            let _e696 = local_9;
                            let _e698 = local_10;
                            let _e699 = (_e696 / _e698);
                            let _e701 = ((_e699 - 0.42f) * 4f);
                            let _e703 = select(_e701, 0f, (_e701 < 0f));
                            let _e705 = select(_e703, 1f, (_e703 > 1f));
                            phi_3420_ = 0u;
                            phi_3421_ = 0f;
                            phi_3422_ = 0.5f;
                            phi_3423_ = 0f;
                            phi_3424_ = vec2<f32>(((((_e213 + -0.04f) * 3f) + _e539) + _e219), ((((_e214 + 0.04f) * 3f) + _e540) + _e220));
                            loop {
                                let _e716 = phi_3420_;
                                let _e718 = phi_3421_;
                                let _e720 = phi_3422_;
                                let _e722 = phi_3423_;
                                let _e724 = phi_3424_;
                                local_11 = _e722;
                                local_12 = _e718;
                                let _e725 = (_e716 < 5u);
                                if _e725 {
                                    let _e729 = floor(_e724.x);
                                    let _e730 = floor(_e724.y);
                                    let _e737 = select(0i, select(select(i32(_e729), i32(-2147483648), (_e729 < -2147483600f)), 2147483647i, (_e729 > 2147483500f)), (_e729 == _e729));
                                    let _e744 = select(0i, select(select(i32(_e730), i32(-2147483648), (_e730 < -2147483600f)), 2147483647i, (_e730 > 2147483500f)), (_e730 == _e730));
                                    let _e745 = (_e724.x - _e729);
                                    let _e749 = ((_e745 * _e745) * (3f - (2f * _e745)));
                                    let _e750 = (_e724.y - _e730);
                                    let _e756 = (bitcast<u32>(_e737) * 2376512323u);
                                    let _e758 = (bitcast<u32>(_e744) * 3625334849u);
                                    let _e760 = ((13u + _e716) * 2654435769u);
                                    let _e761 = ((_e756 ^ _e758) ^ _e760);
                                    let _e765 = ((_e761 ^ (_e761 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e769 = ((_e765 ^ (_e765 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e776 = (f32(((_e769 ^ (_e769 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e779 = (bitcast<u32>((_e737 + 1i)) * 2376512323u);
                                    let _e781 = ((_e779 ^ _e758) ^ _e760);
                                    let _e785 = ((_e781 ^ (_e781 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e789 = ((_e785 ^ (_e785 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e799 = (bitcast<u32>((_e744 + 1i)) * 3625334849u);
                                    let _e801 = ((_e756 ^ _e799) ^ _e760);
                                    let _e805 = ((_e801 ^ (_e801 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e809 = ((_e805 ^ (_e805 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e816 = (f32(((_e809 ^ (_e809 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f);
                                    let _e818 = ((_e779 ^ _e799) ^ _e760);
                                    let _e822 = ((_e818 ^ (_e818 >> bitcast<u32>(16i))) * 2146121005u);
                                    let _e826 = ((_e822 ^ (_e822 >> bitcast<u32>(15i))) * 2221713035u);
                                    let _e836 = (_e776 + (((f32(((_e789 ^ (_e789 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e776) * _e749));
                                    phi_3447_ = (_e716 + 1u);
                                    phi_3448_ = (_e718 + _e720);
                                    phi_3449_ = (_e720 * 0.5f);
                                    phi_3450_ = (_e722 + ((_e836 + (((_e816 + (((f32(((_e826 ^ (_e826 >> bitcast<u32>(16i))) >> bitcast<u32>(8i))) * 0.000000059604645f) - _e816) * _e749)) - _e836) * ((_e750 * _e750) * (3f - (2f * _e750))))) * _e720));
                                    phi_3451_ = vec2<f32>(((_e724.x * 2.03f) + 17.13f), ((_e724.y * 2.01f) - 9.71f));
                                } else {
                                    phi_3447_ = u32();
                                    phi_3448_ = f32();
                                    phi_3449_ = f32();
                                    phi_3450_ = f32();
                                    phi_3451_ = vec2<f32>();
                                }
                                let _e854 = phi_3447_;
                                let _e856 = phi_3448_;
                                let _e858 = phi_3449_;
                                let _e860 = phi_3450_;
                                let _e862 = phi_3451_;
                                continue;
                                continuing {
                                    phi_3420_ = _e854;
                                    phi_3421_ = _e856;
                                    phi_3422_ = _e858;
                                    phi_3423_ = _e860;
                                    phi_3424_ = _e862;
                                    break if !(_e725);
                                }
                            }
                            let _e865 = local_11;
                            let _e867 = local_12;
                            let _e871 = (0.5f + (2f * (_e699 - (_e865 / _e867))));
                            let _e873 = select(_e871, 0f, (_e871 < 0f));
                            let _e876 = (_e214 * 0.15f);
                            let _e882 = (0.55f + (0.45f * select(_e873, 1f, (_e873 > 1f))));
                            let _e885 = (1f - _e705);
                            let _e892 = (((0.25f - _e876) * _e885) + (_e882 * _e705));
                            let _e893 = (((0.45f - (_e214 * 0.2f)) * _e885) + ((0.98f * _e882) * _e705));
                            let _e894 = (((0.75f - _e876) * _e885) + ((0.95f * _e882) * _e705));
                            let _e916 = ((_e892 * ((2.51f * _e892) + 0.03f)) / ((_e892 * ((2.43f * _e892) + 0.59f)) + 0.14f));
                            let _e917 = ((_e893 * ((2.51f * _e893) + 0.03f)) / ((_e893 * ((2.43f * _e893) + 0.59f)) + 0.14f));
                            let _e918 = ((_e894 * ((2.51f * _e894) + 0.03f)) / ((_e894 * ((2.43f * _e894) + 0.59f)) + 0.14f));
                            let _e920 = select(0f, _e916, (_e916 > 0f));
                            let _e922 = select(0f, _e917, (_e917 > 0f));
                            let _e924 = select(0f, _e918, (_e918 > 0f));
                            let _e926 = select(1f, _e920, (_e920 < 1f));
                            let _e928 = select(1f, _e922, (_e922 < 1f));
                            let _e930 = select(1f, _e924, (_e924 < 1f));
                            if (_e926 <= 0.0031308f) {
                                phi_3698_ = (12.92f * _e926);
                            } else {
                                phi_3698_ = ((1.055f * pow(_e926, 0.41666666f)) - 0.055f);
                            }
                            let _e937 = phi_3698_;
                            if (_e928 <= 0.0031308f) {
                                phi_3710_ = (12.92f * _e928);
                            } else {
                                phi_3710_ = ((1.055f * pow(_e928, 0.41666666f)) - 0.055f);
                            }
                            let _e944 = phi_3710_;
                            if (_e930 <= 0.0031308f) {
                                phi_3722_ = (12.92f * _e930);
                            } else {
                                phi_3722_ = ((1.055f * pow(_e930, 0.41666666f)) - 0.055f);
                            }
                            let _e951 = phi_3722_;
                            phi_741_ = vec3<f32>(_e937, _e944, _e951);
                            break;
                        }
                        default: {
                            let _e1918 = (0.8f / exp((1.6f + (0.9f * sin((_e203 * 0.00021f))))));
                            phi_4886_ = vec2<f32>(0f, 0f);
                            phi_4887_ = 0u;
                            loop {
                                let _e1924 = phi_4886_;
                                let _e1926 = phi_4887_;
                                local_23 = _e1926;
                                if (_e1926 < 96u) {
                                    let _e1935 = (((_e1924.x * _e1924.x) - (_e1924.y * _e1924.y)) + (-0.745f + (_e213 * _e1918)));
                                    let _e1936 = (((2f * _e1924.x) * _e1924.y) + (0.186f + (_e214 * _e1918)));
                                    let _e1940 = ((_e1935 * _e1935) + (_e1936 * _e1936));
                                    let _e1941 = (_e1940 > 64f);
                                    if _e1941 {
                                        phi_4914_ = u32();
                                    } else {
                                        phi_4914_ = (_e1926 + 1u);
                                    }
                                    let _e1944 = phi_4914_;
                                    phi_4917_ = _e1940;
                                    phi_4918_ = vec2<f32>(_e1935, _e1936);
                                    phi_4919_ = _e1944;
                                    phi_4920_ = select(true, false, _e1941);
                                    phi_4921_ = _e1941;
                                } else {
                                    phi_4917_ = f32();
                                    phi_4918_ = vec2<f32>();
                                    phi_4919_ = u32();
                                    phi_4920_ = false;
                                    phi_4921_ = false;
                                }
                                let _e1947 = phi_4917_;
                                let _e1949 = phi_4918_;
                                let _e1951 = phi_4919_;
                                let _e1953 = phi_4920_;
                                let _e1955 = phi_4921_;
                                local_22 = _e1955;
                                local_24 = _e1947;
                                local_25 = _e1955;
                                continue;
                                continuing {
                                    phi_4886_ = _e1949;
                                    phi_4887_ = _e1951;
                                    break if !(_e1953);
                                }
                            }
                            let _e1958 = local_22;
                            if _e1958 {
                                let _e1960 = local_23;
                                let _e1963 = local_24;
                                phi_4933_ = ((f32(_e1960) - (log(log(_e1963)) * 1.442695f)) + 4f);
                            } else {
                                phi_4933_ = f32();
                            }
                            let _e1970 = phi_4933_;
                            let _e1972 = local_25;
                            let _e1974 = select(_e1970, -1f, select(true, false, _e1972));
                            if (_e1974 < 0f) {
                                phi_4953_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                let _e1978 = ((_e1974 * 0.015f) + (_e203 * 0.000020000001f));
                                let _e1993 = (0.3f + (0.04f * _e1974));
                                if (_e1993 != _e1993) {
                                    phi_4999_ = true;
                                } else {
                                    phi_4999_ = (1.2f <= _e1993);
                                }
                                let _e1997 = phi_4999_;
                                let _e1999 = ((vec3<f32>(0.5f, 0.5f, 0.5f) + vec3<f32>((0.5f * cos((_e1978 * 6.2831855f))), (0.5f * cos(((_e1978 + 0.33f) * 6.2831855f))), (0.5f * cos(((_e1978 + 0.67f) * 6.2831855f))))) * select(_e1993, 1.2f, _e1997));
                                let _e2024 = ((_e1999.x * ((2.51f * _e1999.x) + 0.03f)) / ((_e1999.x * ((2.43f * _e1999.x) + 0.59f)) + 0.14f));
                                let _e2025 = ((_e1999.y * ((2.51f * _e1999.y) + 0.03f)) / ((_e1999.y * ((2.43f * _e1999.y) + 0.59f)) + 0.14f));
                                let _e2026 = ((_e1999.z * ((2.51f * _e1999.z) + 0.03f)) / ((_e1999.z * ((2.43f * _e1999.z) + 0.59f)) + 0.14f));
                                let _e2028 = select(0f, _e2024, (_e2024 > 0f));
                                let _e2030 = select(0f, _e2025, (_e2025 > 0f));
                                let _e2032 = select(0f, _e2026, (_e2026 > 0f));
                                let _e2034 = select(1f, _e2028, (_e2028 < 1f));
                                let _e2036 = select(1f, _e2030, (_e2030 < 1f));
                                let _e2038 = select(1f, _e2032, (_e2032 < 1f));
                                if (_e2034 <= 0.0031308f) {
                                    phi_5105_ = (12.92f * _e2034);
                                } else {
                                    phi_5105_ = ((1.055f * pow(_e2034, 0.41666666f)) - 0.055f);
                                }
                                let _e2045 = phi_5105_;
                                if (_e2036 <= 0.0031308f) {
                                    phi_5117_ = (12.92f * _e2036);
                                } else {
                                    phi_5117_ = ((1.055f * pow(_e2036, 0.41666666f)) - 0.055f);
                                }
                                let _e2052 = phi_5117_;
                                if (_e2038 <= 0.0031308f) {
                                    phi_5129_ = (12.92f * _e2038);
                                } else {
                                    phi_5129_ = ((1.055f * pow(_e2038, 0.41666666f)) - 0.055f);
                                }
                                let _e2059 = phi_5129_;
                                phi_4953_ = vec3<f32>(_e2045, _e2052, _e2059);
                            }
                            let _e2062 = phi_4953_;
                            phi_741_ = _e2062;
                            break;
                        }
                    }
                    let _e2064 = phi_741_;
                    let _e2067 = (((_e187.y * _e192) + _e187.x) * 3u);
                    if (_e2067 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e2067] = _e2064.x;
                    let _e2073 = (_e2067 + 1u);
                    if (_e2073 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e2073] = _e2064.y;
                    let _e2078 = (_e2067 + 2u);
                    if (_e2078 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e2078] = _e2064.z;
                }
            }
            break;
        }
    }
    return;
}

fn function_2() {
    var phi_1241_: vec3<f32>;
    var phi_1244_: u32;
    var phi_5224_: vec3<f32>;
    var phi_5225_: u32;
    var phi_5226_: u32;
    var phi_5227_: vec3<f32>;
    var phi_5228_: vec3<f32>;
    var phi_5408_: u32;
    var phi_5409_: type_23;
    var phi_5540_: type_24;
    var phi_5470_: f32;
    var phi_5509_: type_23;
    var phi_5511_: type_23;
    var phi_5513_: type_23;
    var phi_5516_: u32;
    var phi_5517_: type_23;
    var local_26: type_23;
    var local_27: type_23;
    var phi_5256_: vec3<f32>;
    var local_28: type_23;
    var local_29: type_23;
    var local_30: type_23;
    var phi_5639_: bool;
    var local_31: type_23;
    var phi_5720_: bool;
    var phi_5345_: vec3<f32>;
    var phi_5346_: u32;
    var phi_5347_: bool;
    var phi_5352_: u32;
    var phi_5356_: vec3<f32>;
    var phi_5357_: u32;
    var phi_5358_: u32;
    var phi_5359_: vec3<f32>;
    var phi_5360_: vec3<f32>;
    var phi_5361_: u32;
    var phi_5362_: bool;
    var phi_5363_: bool;
    var phi_5365_: vec3<f32>;
    var phi_5366_: u32;
    var phi_5367_: u32;
    var phi_5368_: vec3<f32>;
    var phi_5369_: vec3<f32>;
    var phi_5370_: u32;
    var phi_5371_: bool;
    var phi_5372_: bool;
    var phi_5374_: bool;
    var local_32: bool;
    var local_33: vec3<f32>;
    var local_34: vec3<f32>;
    var local_35: vec3<f32>;
    var local_36: vec3<f32>;
    var local_37: u32;
    var phi_5383_: type_22;
    var local_38: bool;
    var local_39: bool;
    var local_40: bool;
    var local_41: bool;
    var local_42: u32;
    var phi_5390_: type_22;
    var local_43: bool;
    var local_44: u32;
    var phi_5397_: type_22;
    var phi_5402_: type_22;
    var phi_1242_: vec3<f32>;
    var phi_1245_: u32;
    var local_45: u32;
    var local_46: vec3<f32>;
    var local_47: vec3<f32>;
    var local_48: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e189 = arrayLength((&global_3.member));
            let _e194 = global_4.member.member;
            if (_e187.x >= _e194) {
            } else {
                let _e198 = global_4.member.member_1;
                if (_e187.y >= _e198) {
                } else {
                    let _e200 = f32(_e194);
                    let _e201 = f32(_e198);
                    let _e205 = ((_e187.y * _e194) + _e187.x);
                    phi_1241_ = vec3<f32>(0f, 0f, 0f);
                    phi_1244_ = 0u;
                    loop {
                        let _e207 = phi_1241_;
                        let _e209 = phi_1244_;
                        local_46 = _e207;
                        local_47 = _e207;
                        local_48 = _e207;
                        let _e212 = global_4.member.member_2;
                        let _e213 = (_e209 < _e212);
                        local_45 = _e212;
                        if _e213 {
                            let _e216 = global_4.member.member_3;
                            let _e221 = (((_e216 ^ 61u) ^ (_e216 >> bitcast<u32>(16i))) * 9u);
                            let _e225 = ((_e221 ^ (_e221 >> bitcast<u32>(4i))) * 668265261u);
                            let _e229 = (_e209 ^ (_e225 ^ (_e225 >> bitcast<u32>(15i))));
                            let _e234 = (((_e229 ^ 61u) ^ (_e229 >> bitcast<u32>(16i))) * 9u);
                            let _e238 = ((_e234 ^ (_e234 >> bitcast<u32>(4i))) * 668265261u);
                            let _e242 = (_e205 ^ (_e238 ^ (_e238 >> bitcast<u32>(15i))));
                            let _e247 = (((_e242 ^ 61u) ^ (_e242 >> bitcast<u32>(16i))) * 9u);
                            let _e251 = ((_e247 ^ (_e247 >> bitcast<u32>(4i))) * 668265261u);
                            let _e254 = (_e251 ^ (_e251 >> bitcast<u32>(15i)));
                            let _e256 = select(_e254, 2654435769u, (_e254 == 0u));
                            let _e259 = (_e256 ^ (_e256 << bitcast<u32>(13i)));
                            let _e262 = (_e259 ^ (_e259 >> bitcast<u32>(17i)));
                            let _e265 = (_e262 ^ (_e262 << bitcast<u32>(5i)));
                            let _e272 = (_e265 ^ (_e265 << bitcast<u32>(13i)));
                            let _e275 = (_e272 ^ (_e272 >> bitcast<u32>(17i)));
                            let _e278 = (_e275 ^ (_e275 << bitcast<u32>(5i)));
                            phi_5224_ = vec3<f32>(1f, 1f, 1f);
                            phi_5225_ = 0u;
                            phi_5226_ = _e278;
                            phi_5227_ = vec3<f32>(((((f32(_e187.x) + (f32((_e265 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e200) - 0.5f) * (2f * (_e200 / _e201))), ((0.5f - ((f32(_e187.y) + (f32((_e278 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e201)) * 2f), -1f);
                            phi_5228_ = vec3<f32>(0f, 0f, 0f);
                            loop {
                                let _e295 = phi_5224_;
                                let _e297 = phi_5225_;
                                let _e299 = phi_5226_;
                                let _e301 = phi_5227_;
                                let _e303 = phi_5228_;
                                local_33 = _e301;
                                local_34 = _e301;
                                local_35 = _e301;
                                local_36 = _e295;
                                local_37 = _e299;
                                local_42 = _e299;
                                let _e304 = (_e297 < 8u);
                                if _e304 {
                                    phi_5408_ = 0u;
                                    phi_5409_ = type_23(1000000000000000000000000000000f, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0u, 0f);
                                    loop {
                                        let _e306 = phi_5408_;
                                        let _e308 = phi_5409_;
                                        local_26 = _e308;
                                        local_27 = _e308;
                                        local_28 = _e308;
                                        local_29 = _e308;
                                        local_30 = _e308;
                                        local_31 = _e308;
                                        let _e309 = (_e306 < 4u);
                                        if _e309 {
                                            switch bitcast<i32>(_e306) {
                                                case 0: {
                                                    phi_5540_ = type_24(vec3<f32>(0f, -100.5f, -1f), 100f, vec3<f32>(0.8f, 0.8f, 0f), 0u, 0f);
                                                    break;
                                                }
                                                case 1: {
                                                    phi_5540_ = type_24(vec3<f32>(0f, 0f, -1.2f), 0.5f, vec3<f32>(0.1f, 0.2f, 0.5f), 0u, 0f);
                                                    break;
                                                }
                                                case 2: {
                                                    phi_5540_ = type_24(vec3<f32>(-1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.8f, 0.8f), 1u, 0.05f);
                                                    break;
                                                }
                                                default: {
                                                    phi_5540_ = type_24(vec3<f32>(1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.6f, 0.2f), 1u, 0.4f);
                                                    break;
                                                }
                                            }
                                            let _e312 = phi_5540_;
                                            let _e316 = (_e303.x - _e312.member.x);
                                            let _e319 = (_e303.y - _e312.member.y);
                                            let _e322 = (_e303.z - _e312.member.z);
                                            let _e330 = (((_e301.x * _e301.x) + (_e301.y * _e301.y)) + (_e301.z * _e301.z));
                                            let _e335 = (((_e316 * _e301.x) + (_e319 * _e301.y)) + (_e322 * _e301.z));
                                            let _e346 = ((_e335 * _e335) - (_e330 * ((((_e316 * _e316) + (_e319 * _e319)) + (_e322 * _e322)) - (_e312.member_1 * _e312.member_1))));
                                            if (_e346 > 0f) {
                                                let _e348 = sqrt(_e346);
                                                let _e349 = -(_e335);
                                                let _e351 = ((_e349 - _e348) / _e330);
                                                if (_e351 < 0.001f) {
                                                    phi_5470_ = ((_e349 + _e348) / _e330);
                                                } else {
                                                    phi_5470_ = _e351;
                                                }
                                                let _e356 = phi_5470_;
                                                if (_e356 >= 0.001f) {
                                                    if (_e356 < _e308.member) {
                                                        let _e360 = (_e356 * _e301.x);
                                                        let _e361 = (_e356 * _e301.y);
                                                        let _e362 = (_e356 * _e301.z);
                                                        phi_5509_ = type_23(_e356, (_e303 + vec3<f32>(_e360, _e361, _e362)), vec3<f32>((((_e303.x + _e360) - _e312.member.x) / _e312.member_1), (((_e303.y + _e361) - _e312.member.y) / _e312.member_1), (((_e303.z + _e362) - _e312.member.z) / _e312.member_1)), _e312.member_2, _e312.member_3, _e312.member_4);
                                                    } else {
                                                        phi_5509_ = _e308;
                                                    }
                                                    let _e380 = phi_5509_;
                                                    phi_5511_ = _e380;
                                                } else {
                                                    phi_5511_ = _e308;
                                                }
                                                let _e382 = phi_5511_;
                                                phi_5513_ = _e382;
                                            } else {
                                                phi_5513_ = _e308;
                                            }
                                            let _e384 = phi_5513_;
                                            phi_5516_ = (_e306 + 1u);
                                            phi_5517_ = _e384;
                                        } else {
                                            phi_5516_ = u32();
                                            phi_5517_ = type_23();
                                        }
                                        let _e387 = phi_5516_;
                                        let _e389 = phi_5517_;
                                        continue;
                                        continuing {
                                            phi_5408_ = _e387;
                                            phi_5409_ = _e389;
                                            break if !(_e309);
                                        }
                                    }
                                    let _e392 = local_26;
                                    let _e394 = (_e392.member >= 1000000000000000000000000000000f);
                                    if _e394 {
                                        phi_5356_ = vec3<f32>();
                                        phi_5357_ = u32();
                                        phi_5358_ = u32();
                                        phi_5359_ = vec3<f32>();
                                        phi_5360_ = vec3<f32>();
                                        phi_5361_ = u32();
                                        phi_5362_ = false;
                                        phi_5363_ = false;
                                    } else {
                                        let _e396 = local_27;
                                        if ((((_e301.x * _e396.member_2.x) + (_e301.y * _e396.member_2.y)) + (_e301.z * _e396.member_2.z)) < 0f) {
                                            phi_5256_ = _e396.member_2;
                                        } else {
                                            phi_5256_ = -(_e396.member_2);
                                        }
                                        let _e412 = phi_5256_;
                                        let _e414 = local_28;
                                        let _e418 = local_29;
                                        let _e421 = local_30;
                                        let _e426 = (_e299 ^ (_e299 << bitcast<u32>(13i)));
                                        let _e429 = (_e426 ^ (_e426 >> bitcast<u32>(17i)));
                                        let _e432 = (_e429 ^ (_e429 << bitcast<u32>(5i)));
                                        let _e435 = (_e432 ^ (_e432 << bitcast<u32>(13i)));
                                        let _e438 = (_e435 ^ (_e435 >> bitcast<u32>(17i)));
                                        let _e441 = (_e438 ^ (_e438 << bitcast<u32>(5i)));
                                        if (_e421.member_4 == 0u) {
                                            let _e516 = (1f - (f32((_e432 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e517 = (f32((_e441 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e519 = (1f - (_e516 * _e516));
                                            if (_e519 != _e519) {
                                                phi_5720_ = true;
                                            } else {
                                                phi_5720_ = (0f >= _e519);
                                            }
                                            let _e523 = phi_5720_;
                                            let _e525 = sqrt(select(_e519, 0f, _e523));
                                            let _e527 = (_e525 * cos(_e517));
                                            let _e529 = (_e525 * sin(_e517));
                                            let _e532 = (_e412.x + _e527);
                                            let _e534 = (_e412.y + _e529);
                                            let _e536 = (_e412.z + _e516);
                                            phi_5345_ = select((_e412 + vec3<f32>(_e527, _e529, _e516)), _e412, vec3(((((_e532 * _e532) + (_e534 * _e534)) + (_e536 * _e536)) < 0.00000001f)));
                                            phi_5346_ = u32();
                                            phi_5347_ = true;
                                        } else {
                                            let _e449 = (_e301 * (1f / sqrt((((_e301.x * _e301.x) + (_e301.y * _e301.y)) + (_e301.z * _e301.z)))));
                                            let _e461 = (2f * (((_e449.x * _e412.x) + (_e449.y * _e412.y)) + (_e449.z * _e412.z)));
                                            let _e466 = (_e449 - vec3<f32>((_e461 * _e412.x), (_e461 * _e412.y), (_e461 * _e412.z)));
                                            let _e474 = (1f - (f32((_e432 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e475 = (f32((_e441 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e477 = (1f - (_e474 * _e474));
                                            if (_e477 != _e477) {
                                                phi_5639_ = true;
                                            } else {
                                                phi_5639_ = (0f >= _e477);
                                            }
                                            let _e481 = phi_5639_;
                                            let _e483 = sqrt(select(_e477, 0f, _e481));
                                            let _e489 = local_31;
                                            let _e491 = (_e489.member_5 * (_e483 * cos(_e475)));
                                            let _e492 = (_e489.member_5 * (_e483 * sin(_e475)));
                                            let _e493 = (_e489.member_5 * _e474);
                                            phi_5345_ = (_e466 + vec3<f32>(_e491, _e492, _e493));
                                            phi_5346_ = _e441;
                                            phi_5347_ = select(true, false, (((((_e466.x + _e491) * _e412.x) + ((_e466.y + _e492) * _e412.y)) + ((_e466.z + _e493) * _e412.z)) <= 0f));
                                        }
                                        let _e547 = phi_5345_;
                                        let _e549 = phi_5346_;
                                        let _e551 = phi_5347_;
                                        if _e551 {
                                            phi_5352_ = (_e297 + 1u);
                                        } else {
                                            phi_5352_ = u32();
                                        }
                                        let _e554 = phi_5352_;
                                        phi_5356_ = (_e295 * _e414.member_3);
                                        phi_5357_ = _e554;
                                        phi_5358_ = _e441;
                                        phi_5359_ = _e547;
                                        phi_5360_ = _e418.member_1;
                                        phi_5361_ = _e549;
                                        phi_5362_ = _e551;
                                        phi_5363_ = select(true, false, _e551);
                                    }
                                    let _e557 = phi_5356_;
                                    let _e559 = phi_5357_;
                                    let _e561 = phi_5358_;
                                    let _e563 = phi_5359_;
                                    let _e565 = phi_5360_;
                                    let _e567 = phi_5361_;
                                    let _e569 = phi_5362_;
                                    let _e571 = phi_5363_;
                                    phi_5365_ = _e557;
                                    phi_5366_ = _e559;
                                    phi_5367_ = _e561;
                                    phi_5368_ = _e563;
                                    phi_5369_ = _e565;
                                    phi_5370_ = _e567;
                                    phi_5371_ = _e569;
                                    phi_5372_ = _e394;
                                    phi_5374_ = _e571;
                                } else {
                                    phi_5365_ = vec3<f32>();
                                    phi_5366_ = u32();
                                    phi_5367_ = u32();
                                    phi_5368_ = vec3<f32>();
                                    phi_5369_ = vec3<f32>();
                                    phi_5370_ = u32();
                                    phi_5371_ = false;
                                    phi_5372_ = false;
                                    phi_5374_ = false;
                                }
                                let _e573 = phi_5365_;
                                let _e575 = phi_5366_;
                                let _e577 = phi_5367_;
                                let _e579 = phi_5368_;
                                let _e581 = phi_5369_;
                                let _e583 = phi_5370_;
                                let _e585 = phi_5371_;
                                let _e587 = phi_5372_;
                                let _e589 = phi_5374_;
                                local_32 = _e587;
                                local_38 = _e587;
                                local_39 = select(true, false, _e304);
                                local_40 = _e587;
                                local_41 = _e589;
                                local_43 = _e587;
                                local_44 = _e583;
                                continue;
                                continuing {
                                    phi_5224_ = _e573;
                                    phi_5225_ = _e575;
                                    phi_5226_ = _e577;
                                    phi_5227_ = _e579;
                                    phi_5228_ = _e581;
                                    break if !(_e585);
                                }
                            }
                            let _e593 = local_32;
                            if _e593 {
                                let _e595 = local_33;
                                let _e599 = local_34;
                                let _e604 = local_35;
                                let _e611 = ((_e599.y * (1f / sqrt((((_e595.x * _e595.x) + (_e599.y * _e599.y)) + (_e604.z * _e604.z))))) + 1f);
                                let _e613 = (1f - (0.5f * _e611));
                                let _e620 = local_36;
                                let _e623 = local_37;
                                phi_5383_ = type_22(_e623, (_e620 * vec3<f32>((_e613 + (_e611 * 0.25f)), (_e613 + (_e611 * 0.35f)), 1f)));
                            } else {
                                phi_5383_ = type_22();
                            }
                            let _e626 = phi_5383_;
                            let _e628 = local_38;
                            let _e630 = local_39;
                            let _e631 = select(_e630, false, _e628);
                            let _e633 = local_40;
                            let _e635 = local_41;
                            if _e631 {
                                let _e638 = local_42;
                                phi_5390_ = type_22(_e638, vec3<f32>(0f, 0f, 0f));
                            } else {
                                phi_5390_ = type_22();
                            }
                            let _e641 = phi_5390_;
                            let _e642 = select(select(_e635, false, _e633), false, _e631);
                            let _e644 = local_43;
                            if _e642 {
                                let _e647 = local_44;
                                phi_5397_ = type_22(_e647, vec3<f32>(0f, 0f, 0f));
                            } else {
                                phi_5397_ = _e626;
                            }
                            let _e650 = phi_5397_;
                            if select(select(_e644, false, _e631), true, _e642) {
                                phi_5402_ = _e650;
                            } else {
                                phi_5402_ = _e641;
                            }
                            let _e653 = phi_5402_;
                            phi_1242_ = (_e207 + _e653.member_1);
                            phi_1245_ = (_e209 + 1u);
                        } else {
                            phi_1242_ = vec3<f32>();
                            phi_1245_ = u32();
                        }
                        let _e658 = phi_1242_;
                        let _e660 = phi_1245_;
                        continue;
                        continuing {
                            phi_1241_ = _e658;
                            phi_1244_ = _e660;
                            break if !(_e213);
                        }
                    }
                    let _e663 = local_45;
                    let _e664 = f32(_e663);
                    let _e666 = local_46;
                    let _e670 = local_47;
                    let _e674 = local_48;
                    let _e677 = (_e205 * 3u);
                    if (_e677 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e677] = (_e666.x / _e664);
                    let _e681 = (_e677 + 1u);
                    if (_e681 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e681] = (_e670.y / _e664);
                    let _e685 = (_e677 + 2u);
                    if (_e685 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e685] = (_e674.z / _e664);
                }
            }
            break;
        }
    }
    return;
}

fn function_3() {
    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e193 = global_5.member.member_2;
            if (_e187.x < _e193) {
                let _e197 = global_5.member.member_3;
                let _e202 = (((_e197 ^ 61u) ^ (_e197 >> bitcast<u32>(16i))) * 9u);
                let _e206 = ((_e202 ^ (_e202 >> bitcast<u32>(4i))) * 668265261u);
                let _e210 = (12648430u ^ (_e206 ^ (_e206 >> bitcast<u32>(15i))));
                let _e215 = (((_e210 ^ 61u) ^ (_e210 >> bitcast<u32>(16i))) * 9u);
                let _e219 = ((_e215 ^ (_e215 >> bitcast<u32>(4i))) * 668265261u);
                let _e223 = (_e187.x ^ (_e219 ^ (_e219 >> bitcast<u32>(15i))));
                let _e228 = (((_e223 ^ 61u) ^ (_e223 >> bitcast<u32>(16i))) * 9u);
                let _e232 = ((_e228 ^ (_e228 >> bitcast<u32>(4i))) * 668265261u);
                let _e235 = (_e232 ^ (_e232 >> bitcast<u32>(15i)));
                let _e237 = select(_e235, 2654435769u, (_e235 == 0u));
                let _e240 = (_e237 ^ (_e237 << bitcast<u32>(13i)));
                let _e243 = (_e240 ^ (_e240 >> bitcast<u32>(17i)));
                let _e246 = (_e243 ^ (_e243 << bitcast<u32>(5i)));
                let _e250 = (f32((_e246 >> bitcast<u32>(8i))) * 0.00000037450704f);
                let _e253 = global_5.member.member_1;
                let _e254 = f32(_e253);
                let _e258 = (_e246 ^ (_e246 << bitcast<u32>(13i)));
                let _e261 = (_e258 ^ (_e258 >> bitcast<u32>(17i)));
                let _e270 = ((0.35f * _e254) * (0.6f + (f32(((_e261 ^ (_e261 << bitcast<u32>(5i))) >> bitcast<u32>(8i))) * 0.000000023841858f)));
                let _e273 = global_5.member.member;
                if (_e187.x < arrayLength((&global_6.member))) {
                } else {
                    break;
                }
                global_6.member[_e187.x] = type_28(((f32(_e273) * 0.5f) + (cos(_e250) * _e270)), ((_e254 * 0.5f) + (sin(_e250) * _e270)), (_e250 + 3.1415927f), 0f);
            }
            break;
        }
    }
    return;
}

fn function_4() {
    var phi_5841_: f32;
    var phi_5845_: f32;
    var phi_5855_: f32;
    var phi_5859_: f32;
    var phi_5869_: f32;
    var phi_5873_: f32;
    var phi_5883_: f32;
    var phi_5887_: f32;
    var phi_5897_: f32;
    var phi_5901_: f32;
    var phi_5911_: f32;
    var phi_5915_: f32;
    var phi_2065_: bool;
    var phi_2098_: f32;
    var phi_2099_: f32;
    var phi_2100_: f32;
    var phi_2101_: f32;
    var phi_5979_: f32;
    var phi_5983_: f32;
    var phi_5993_: f32;
    var phi_5997_: f32;
    var phi_6007_: f32;
    var phi_6011_: f32;
    var phi_6021_: f32;
    var phi_6025_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e191 = arrayLength((&global_7.member));
            let _e195 = global_5.member.member_2;
            if (_e187.x >= _e195) {
            } else {
                let _e197 = (_e187.x < arrayLength((&global_6.member)));
                if _e197 {
                } else {
                    break;
                }
                let _e202 = global_5.member.member_6;
                let _e206 = global_6.member[_e187.x].member_2;
                let _e207 = (_e206 + _e202);
                let _e211 = global_6.member[_e187.x].member;
                let _e215 = global_5.member.member_7;
                let _e217 = (_e211 + (cos(_e207) * _e215));
                let _e221 = global_6.member[_e187.x].member_1;
                let _e224 = (_e221 + (sin(_e207) * _e215));
                let _e227 = global_5.member.member;
                let _e228 = f32(_e227);
                if (_e217 < 0f) {
                    phi_5845_ = (_e217 + _e228);
                } else {
                    if (_e217 >= _e228) {
                        phi_5841_ = (_e217 - _e228);
                    } else {
                        phi_5841_ = _e217;
                    }
                    let _e233 = phi_5841_;
                    phi_5845_ = _e233;
                }
                let _e236 = phi_5845_;
                let _e242 = (_e227 == 0u);
                if _e242 {
                    break;
                }
                let _e246 = global_5.member.member_1;
                let _e247 = f32(_e246);
                if (_e224 < 0f) {
                    phi_5859_ = (_e224 + _e247);
                } else {
                    if (_e224 >= _e247) {
                        phi_5855_ = (_e224 - _e247);
                    } else {
                        phi_5855_ = _e224;
                    }
                    let _e252 = phi_5855_;
                    phi_5859_ = _e252;
                }
                let _e255 = phi_5859_;
                let _e261 = (_e246 == 0u);
                if _e261 {
                    break;
                }
                let _e264 = (((select(select(u32(_e255), 0u, (_e255 < 0f)), 4294967295u, (_e255 > 4294967000f)) % _e246) * _e227) + (select(select(u32(_e236), 0u, (_e236 < 0f)), 4294967295u, (_e236 > 4294967000f)) % _e227));
                if (_e264 < _e191) {
                } else {
                    break;
                }
                let _e268 = global_7.member[_e264];
                let _e269 = global_6.member[_e187.x].member_2;
                let _e270 = global_6.member[_e187.x].member;
                let _e273 = (_e270 + (cos(_e269) * _e215));
                let _e274 = global_6.member[_e187.x].member_1;
                let _e277 = (_e274 + (sin(_e269) * _e215));
                if (_e273 < 0f) {
                    phi_5873_ = (_e273 + _e228);
                } else {
                    if (_e273 >= _e228) {
                        phi_5869_ = (_e273 - _e228);
                    } else {
                        phi_5869_ = _e273;
                    }
                    let _e282 = phi_5869_;
                    phi_5873_ = _e282;
                }
                let _e285 = phi_5873_;
                if _e242 {
                    break;
                }
                if (_e277 < 0f) {
                    phi_5887_ = (_e277 + _e247);
                } else {
                    if (_e277 >= _e247) {
                        phi_5883_ = (_e277 - _e247);
                    } else {
                        phi_5883_ = _e277;
                    }
                    let _e296 = phi_5883_;
                    phi_5887_ = _e296;
                }
                let _e299 = phi_5887_;
                if _e261 {
                    break;
                }
                let _e307 = (((select(select(u32(_e299), 0u, (_e299 < 0f)), 4294967295u, (_e299 > 4294967000f)) % _e246) * _e227) + (select(select(u32(_e285), 0u, (_e285 < 0f)), 4294967295u, (_e285 > 4294967000f)) % _e227));
                if (_e307 < _e191) {
                } else {
                    break;
                }
                let _e311 = global_7.member[_e307];
                let _e313 = global_6.member[_e187.x].member_2;
                let _e314 = (_e313 + -(_e202));
                let _e315 = global_6.member[_e187.x].member;
                let _e318 = (_e315 + (cos(_e314) * _e215));
                let _e319 = global_6.member[_e187.x].member_1;
                let _e322 = (_e319 + (sin(_e314) * _e215));
                if (_e318 < 0f) {
                    phi_5901_ = (_e318 + _e228);
                } else {
                    if (_e318 >= _e228) {
                        phi_5897_ = (_e318 - _e228);
                    } else {
                        phi_5897_ = _e318;
                    }
                    let _e327 = phi_5897_;
                    phi_5901_ = _e327;
                }
                let _e330 = phi_5901_;
                if _e242 {
                    break;
                }
                if (_e322 < 0f) {
                    phi_5915_ = (_e322 + _e247);
                } else {
                    if (_e322 >= _e247) {
                        phi_5911_ = (_e322 - _e247);
                    } else {
                        phi_5911_ = _e322;
                    }
                    let _e341 = phi_5911_;
                    phi_5915_ = _e341;
                }
                let _e344 = phi_5915_;
                if _e261 {
                    break;
                }
                let _e352 = (((select(select(u32(_e344), 0u, (_e344 < 0f)), 4294967295u, (_e344 > 4294967000f)) % _e246) * _e227) + (select(select(u32(_e330), 0u, (_e330 < 0f)), 4294967295u, (_e330 > 4294967000f)) % _e227));
                if (_e352 < _e191) {
                } else {
                    break;
                }
                let _e356 = global_7.member[_e352];
                let _e357 = global_6.member[_e187.x].member_2;
                if (_e311 >= _e268) {
                    phi_2065_ = select(true, false, (_e311 >= _e356));
                } else {
                    phi_2065_ = true;
                }
                let _e362 = phi_2065_;
                if _e362 {
                    if (_e268 > _e356) {
                        let _e425 = global_5.member.member_5;
                        phi_2100_ = (_e357 + _e425);
                    } else {
                        if (_e356 > _e268) {
                            let _e419 = global_5.member.member_5;
                            phi_2099_ = (_e357 - _e419);
                        } else {
                            let _e367 = global_5.member.member_3;
                            let _e368 = (_e367 ^ 935473873u);
                            let _e373 = (((_e368 ^ 61u) ^ (_e368 >> bitcast<u32>(16i))) * 9u);
                            let _e377 = ((_e373 ^ (_e373 >> bitcast<u32>(4i))) * 668265261u);
                            let _e381 = (_e187.x ^ (_e377 ^ (_e377 >> bitcast<u32>(15i))));
                            let _e386 = (((_e381 ^ 61u) ^ (_e381 >> bitcast<u32>(16i))) * 9u);
                            let _e390 = ((_e386 ^ (_e386 >> bitcast<u32>(4i))) * 668265261u);
                            let _e393 = (_e390 ^ (_e390 >> bitcast<u32>(15i)));
                            let _e395 = select(_e393, 2654435769u, (_e393 == 0u));
                            let _e398 = (_e395 ^ (_e395 << bitcast<u32>(13i)));
                            let _e401 = (_e398 ^ (_e398 >> bitcast<u32>(17i)));
                            if (((_e401 ^ (_e401 << bitcast<u32>(5i))) & 1u) == 0u) {
                                let _e413 = global_5.member.member_5;
                                phi_2098_ = (_e357 + _e413);
                            } else {
                                let _e409 = global_5.member.member_5;
                                phi_2098_ = (_e357 - _e409);
                            }
                            let _e416 = phi_2098_;
                            phi_2099_ = _e416;
                        }
                        let _e422 = phi_2099_;
                        phi_2100_ = _e422;
                    }
                    let _e428 = phi_2100_;
                    phi_2101_ = _e428;
                } else {
                    phi_2101_ = _e357;
                }
                let _e430 = phi_2101_;
                let _e431 = global_6.member[_e187.x].member;
                let _e435 = global_5.member.member_4;
                let _e437 = (_e431 + (cos(_e430) * _e435));
                if (_e437 < 0f) {
                    phi_5983_ = (_e437 + _e228);
                } else {
                    if (_e437 >= _e228) {
                        phi_5979_ = (_e437 - _e228);
                    } else {
                        phi_5979_ = _e437;
                    }
                    let _e442 = phi_5979_;
                    phi_5983_ = _e442;
                }
                let _e445 = phi_5983_;
                let _e446 = global_6.member[_e187.x].member_1;
                let _e449 = (_e446 + (sin(_e430) * _e435));
                if (_e449 < 0f) {
                    phi_5997_ = (_e449 + _e247);
                } else {
                    if (_e449 >= _e247) {
                        phi_5993_ = (_e449 - _e247);
                    } else {
                        phi_5993_ = _e449;
                    }
                    let _e454 = phi_5993_;
                    phi_5997_ = _e454;
                }
                let _e457 = phi_5997_;
                if (_e445 < 0f) {
                    phi_6011_ = (_e445 + _e228);
                } else {
                    if (_e445 >= _e228) {
                        phi_6007_ = (_e445 - _e228);
                    } else {
                        phi_6007_ = _e445;
                    }
                    let _e463 = phi_6007_;
                    phi_6011_ = _e463;
                }
                let _e466 = phi_6011_;
                if _e242 {
                    break;
                }
                if (_e457 < 0f) {
                    phi_6025_ = (_e457 + _e247);
                } else {
                    if (_e457 >= _e247) {
                        phi_6021_ = (_e457 - _e247);
                    } else {
                        phi_6021_ = _e457;
                    }
                    let _e477 = phi_6021_;
                    phi_6025_ = _e477;
                }
                let _e480 = phi_6025_;
                if _e261 {
                    break;
                }
                let _e488 = (((select(select(u32(_e480), 0u, (_e480 < 0f)), 4294967295u, (_e480 > 4294967000f)) % _e246) * _e227) + (select(select(u32(_e466), 0u, (_e466 < 0f)), 4294967295u, (_e466 > 4294967000f)) % _e227));
                if _e197 {
                } else {
                    break;
                }
                global_6.member[_e187.x] = type_28(_e445, _e457, _e430, 0f);
                let _e491 = global_5.member.member_8;
                if (_e488 < _e191) {
                } else {
                    break;
                }
                let _e495 = global_7.member[_e488];
                global_7.member[_e488] = (_e495 + _e491);
            }
            break;
        }
    }
    return;
}

fn function_5() {
    var phi_2219_: u32;
    var phi_2222_: f32;
    var phi_2220_: u32;
    var phi_2223_: f32;
    var local_49: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e191 = global_8.member;
            if (_e187.y >= _e191) {
            } else {
                if (_e187.x >= _e191) {
                } else {
                    phi_2219_ = 0u;
                    phi_2222_ = 0f;
                    loop {
                        let _e197 = phi_2219_;
                        let _e199 = phi_2222_;
                        local_49 = _e199;
                        let _e200 = (_e197 < _e191);
                        if _e200 {
                            let _e209 = global_9.member[((_e187.y * _e191) + _e197)];
                            let _e210 = global_10.member[((_e197 * _e191) + _e187.x)];
                            phi_2220_ = (_e197 + 1u);
                            phi_2223_ = (_e199 + (_e209 * _e210));
                        } else {
                            phi_2220_ = u32();
                            phi_2223_ = f32();
                        }
                        let _e215 = phi_2220_;
                        let _e217 = phi_2223_;
                        continue;
                        continuing {
                            phi_2219_ = _e215;
                            phi_2222_ = _e217;
                            break if !(_e200);
                        }
                    }
                    let _e220 = ((_e187.y * _e191) + _e187.x);
                    if (_e220 < arrayLength((&global_11.member))) {
                    } else {
                        break;
                    }
                    let _e225 = local_49;
                    global_11.member[_e220] = _e225;
                }
            }
            break;
        }
    }
    return;
}

fn function_6() {
    var phi_6685_: bool;
    var phi_2291_: u32;
    var phi_2294_: f32;
    var phi_2303_: u32;
    var phi_2306_: f32;
    var phi_2304_: u32;
    var phi_2307_: f32;
    var phi_6682_: bool;
    var phi_6687_: bool;
    var phi_2292_: u32;
    var phi_2295_: f32;
    var phi_6686_: bool;
    var local_50: f32;
    var local_51: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e196 = global_5.member.member;
            if (_e187.x < _e196) {
                let _e200 = global_5.member.member_1;
                if (_e187.y < _e200) {
                    phi_6685_ = false;
                    phi_2291_ = 0u;
                    phi_2294_ = 0f;
                    loop {
                        let _e203 = phi_6685_;
                        let _e205 = phi_2291_;
                        let _e207 = phi_2294_;
                        local_50 = _e207;
                        let _e208 = (_e205 < 3u);
                        if _e208 {
                            phi_2303_ = 0u;
                            phi_2306_ = _e207;
                            loop {
                                let _e210 = phi_2303_;
                                let _e212 = phi_2306_;
                                local_51 = _e212;
                                let _e213 = (_e210 < 3u);
                                if _e213 {
                                    if (_e196 == 0u) {
                                        phi_6682_ = true;
                                        break;
                                    }
                                    if (_e200 == 0u) {
                                        phi_6682_ = true;
                                        break;
                                    }
                                    let _e225 = ((((((_e187.y + _e200) + _e205) - 1u) % _e200) * _e196) + ((((_e187.x + _e196) + _e210) - 1u) % _e196));
                                    if (_e225 < arrayLength((&global_12.member))) {
                                    } else {
                                        phi_6682_ = true;
                                        break;
                                    }
                                    let _e229 = global_12.member[_e225];
                                    phi_2304_ = (_e210 + 1u);
                                    phi_2307_ = (_e212 + _e229);
                                } else {
                                    phi_2304_ = u32();
                                    phi_2307_ = f32();
                                }
                                let _e233 = phi_2304_;
                                let _e235 = phi_2307_;
                                continue;
                                continuing {
                                    phi_2303_ = _e233;
                                    phi_2306_ = _e235;
                                    phi_6682_ = _e203;
                                    break if !(_e213);
                                }
                            }
                            let _e238 = phi_6682_;
                            phi_6686_ = _e238;
                            if _e238 {
                                break;
                            }
                            phi_6687_ = _e238;
                            phi_2292_ = (_e205 + 1u);
                            let _e273 = local_51;
                            phi_2295_ = _e273;
                        } else {
                            phi_6687_ = _e203;
                            phi_2292_ = u32();
                            phi_2295_ = f32();
                        }
                        let _e241 = phi_6687_;
                        let _e243 = phi_2292_;
                        let _e245 = phi_2295_;
                        continue;
                        continuing {
                            phi_6685_ = _e241;
                            phi_2291_ = _e243;
                            phi_2294_ = _e245;
                            phi_6686_ = _e241;
                            break if !(_e208);
                        }
                    }
                    let _e248 = phi_6686_;
                    if _e248 {
                        break;
                    }
                    let _e250 = local_50;
                    let _e254 = global_5.member.member_9;
                    let _e257 = ((_e187.y * _e196) + _e187.x);
                    if (_e257 < arrayLength((&global_13.member))) {
                    } else {
                        break;
                    }
                    global_13.member[_e257] = ((_e250 * 0.11111111f) * _e254);
                }
            }
            break;
        }
    }
    return;
}

fn function_7() {
    var phi_2377_: u32;
    var phi_2380_: f32;
    var phi_2378_: u32;
    var phi_2381_: f32;
    var phi_6688_: bool;
    var local_52: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e195 = global_8.member;
            if (_e187.y >= _e195) {
            } else {
                if (_e187.x >= _e195) {
                } else {
                    phi_2377_ = 0u;
                    phi_2380_ = 0f;
                    loop {
                        let _e201 = phi_2377_;
                        let _e203 = phi_2380_;
                        local_52 = _e203;
                        let _e204 = (_e201 < _e195);
                        if _e204 {
                            let _e206 = ((_e187.y * _e195) + _e201);
                            if (_e206 < arrayLength((&global_9.member))) {
                            } else {
                                phi_6688_ = true;
                                break;
                            }
                            let _e210 = global_9.member[_e206];
                            let _e212 = ((_e201 * _e195) + _e187.x);
                            if (_e212 < arrayLength((&global_10.member))) {
                            } else {
                                phi_6688_ = true;
                                break;
                            }
                            let _e216 = global_10.member[_e212];
                            phi_2378_ = (_e201 + 1u);
                            phi_2381_ = (_e203 + (_e210 * _e216));
                        } else {
                            phi_2378_ = u32();
                            phi_2381_ = f32();
                        }
                        let _e221 = phi_2378_;
                        let _e223 = phi_2381_;
                        continue;
                        continuing {
                            phi_2377_ = _e221;
                            phi_2380_ = _e223;
                            phi_6688_ = false;
                            break if !(_e204);
                        }
                    }
                    let _e226 = phi_6688_;
                    if _e226 {
                        break;
                    }
                    let _e228 = ((_e187.y * _e195) + _e187.x);
                    if (_e228 < arrayLength((&global_11.member))) {
                    } else {
                        break;
                    }
                    let _e233 = local_52;
                    global_11.member[_e228] = _e233;
                }
            }
            break;
        }
    }
    return;
}

fn function_8() {
    var phi_2450_: vec3<f32>;
    var phi_2453_: u32;
    var phi_6694_: u32;
    var phi_6106_: vec3<f32>;
    var phi_6107_: u32;
    var phi_6108_: vec3<f32>;
    var phi_6109_: vec3<f32>;
    var phi_6274_: u32;
    var phi_6275_: type_23;
    var phi_6405_: type_24;
    var phi_6336_: f32;
    var phi_6375_: type_23;
    var phi_6377_: type_23;
    var phi_6379_: type_23;
    var phi_6382_: u32;
    var phi_6383_: type_23;
    var local_53: type_23;
    var local_54: type_23;
    var phi_6137_: vec3<f32>;
    var local_55: type_23;
    var local_56: type_23;
    var local_57: type_23;
    var local_58: type_23;
    var phi_6487_: bool;
    var phi_6551_: bool;
    var phi_6221_: vec3<f32>;
    var phi_6222_: bool;
    var phi_6227_: u32;
    var phi_6701_: u32;
    var phi_6231_: vec3<f32>;
    var phi_6232_: u32;
    var phi_6233_: vec3<f32>;
    var phi_6234_: vec3<f32>;
    var phi_6235_: bool;
    var phi_6236_: bool;
    var phi_6700_: u32;
    var phi_6238_: vec3<f32>;
    var phi_6239_: u32;
    var phi_6240_: vec3<f32>;
    var phi_6241_: vec3<f32>;
    var phi_6242_: bool;
    var phi_6243_: bool;
    var phi_6245_: bool;
    var local_59: bool;
    var local_60: vec3<f32>;
    var local_61: vec3<f32>;
    var local_62: vec3<f32>;
    var local_63: vec3<f32>;
    var phi_6253_: vec3<f32>;
    var local_64: bool;
    var local_65: bool;
    var local_66: bool;
    var local_67: bool;
    var local_68: bool;
    var phi_2451_: vec3<f32>;
    var phi_2454_: u32;
    var local_69: u32;
    var local_70: vec3<f32>;
    var local_71: vec3<f32>;
    var local_72: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e187 = global;
            let _e189 = arrayLength((&global_3.member));
            let _e194 = global_4.member.member;
            if (_e187.x >= _e194) {
            } else {
                let _e198 = global_4.member.member_1;
                if (_e187.y >= _e198) {
                } else {
                    let _e200 = f32(_e194);
                    let _e201 = f32(_e198);
                    let _e205 = ((_e187.y * _e194) + _e187.x);
                    phi_2450_ = vec3<f32>(0f, 0f, 0f);
                    phi_2453_ = 0u;
                    loop {
                        let _e207 = phi_2450_;
                        let _e209 = phi_2453_;
                        local_70 = _e207;
                        local_71 = _e207;
                        local_72 = _e207;
                        let _e212 = global_4.member.member_2;
                        let _e213 = (_e209 < _e212);
                        local_69 = _e212;
                        if _e213 {
                            let _e216 = global_4.member.member_3;
                            let _e221 = (((_e216 ^ 61u) ^ (_e216 >> bitcast<u32>(16i))) * 9u);
                            let _e225 = ((_e221 ^ (_e221 >> bitcast<u32>(4i))) * 668265261u);
                            let _e229 = (_e209 ^ (_e225 ^ (_e225 >> bitcast<u32>(15i))));
                            let _e234 = (((_e229 ^ 61u) ^ (_e229 >> bitcast<u32>(16i))) * 9u);
                            let _e238 = ((_e234 ^ (_e234 >> bitcast<u32>(4i))) * 668265261u);
                            let _e242 = (_e205 ^ (_e238 ^ (_e238 >> bitcast<u32>(15i))));
                            let _e247 = (((_e242 ^ 61u) ^ (_e242 >> bitcast<u32>(16i))) * 9u);
                            let _e251 = ((_e247 ^ (_e247 >> bitcast<u32>(4i))) * 668265261u);
                            let _e254 = (_e251 ^ (_e251 >> bitcast<u32>(15i)));
                            let _e256 = select(_e254, 2654435769u, (_e254 == 0u));
                            let _e259 = (_e256 ^ (_e256 << bitcast<u32>(13i)));
                            let _e262 = (_e259 ^ (_e259 >> bitcast<u32>(17i)));
                            let _e265 = (_e262 ^ (_e262 << bitcast<u32>(5i)));
                            let _e272 = (_e265 ^ (_e265 << bitcast<u32>(13i)));
                            let _e275 = (_e272 ^ (_e272 >> bitcast<u32>(17i)));
                            let _e278 = (_e275 ^ (_e275 << bitcast<u32>(5i)));
                            phi_6694_ = _e278;
                            phi_6106_ = vec3<f32>(1f, 1f, 1f);
                            phi_6107_ = 0u;
                            phi_6108_ = vec3<f32>(((((f32(_e187.x) + (f32((_e265 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e200) - 0.5f) * (2f * (_e200 / _e201))), ((0.5f - ((f32(_e187.y) + (f32((_e278 >> bitcast<u32>(8i))) * 0.000000059604645f)) / _e201)) * 2f), -1f);
                            phi_6109_ = vec3<f32>(0f, 0f, 0f);
                            loop {
                                let _e295 = phi_6694_;
                                let _e297 = phi_6106_;
                                let _e299 = phi_6107_;
                                let _e301 = phi_6108_;
                                let _e303 = phi_6109_;
                                local_60 = _e301;
                                local_61 = _e301;
                                local_62 = _e301;
                                local_63 = _e297;
                                let _e304 = (_e299 < 8u);
                                if _e304 {
                                    phi_6274_ = 0u;
                                    phi_6275_ = type_23(1000000000000000000000000000000f, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), 0u, 0f);
                                    loop {
                                        let _e306 = phi_6274_;
                                        let _e308 = phi_6275_;
                                        local_53 = _e308;
                                        local_54 = _e308;
                                        local_55 = _e308;
                                        local_56 = _e308;
                                        local_57 = _e308;
                                        local_58 = _e308;
                                        let _e309 = (_e306 < 4u);
                                        if _e309 {
                                            switch bitcast<i32>(_e306) {
                                                case 0: {
                                                    phi_6405_ = type_24(vec3<f32>(0f, -100.5f, -1f), 100f, vec3<f32>(0.8f, 0.8f, 0f), 0u, 0f);
                                                    break;
                                                }
                                                case 1: {
                                                    phi_6405_ = type_24(vec3<f32>(0f, 0f, -1.2f), 0.5f, vec3<f32>(0.1f, 0.2f, 0.5f), 0u, 0f);
                                                    break;
                                                }
                                                case 2: {
                                                    phi_6405_ = type_24(vec3<f32>(-1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.8f, 0.8f), 1u, 0.05f);
                                                    break;
                                                }
                                                default: {
                                                    phi_6405_ = type_24(vec3<f32>(1f, 0f, -1f), 0.5f, vec3<f32>(0.8f, 0.6f, 0.2f), 1u, 0.4f);
                                                    break;
                                                }
                                            }
                                            let _e312 = phi_6405_;
                                            let _e316 = (_e303.x - _e312.member.x);
                                            let _e319 = (_e303.y - _e312.member.y);
                                            let _e322 = (_e303.z - _e312.member.z);
                                            let _e330 = (((_e301.x * _e301.x) + (_e301.y * _e301.y)) + (_e301.z * _e301.z));
                                            let _e335 = (((_e316 * _e301.x) + (_e319 * _e301.y)) + (_e322 * _e301.z));
                                            let _e346 = ((_e335 * _e335) - (_e330 * ((((_e316 * _e316) + (_e319 * _e319)) + (_e322 * _e322)) - (_e312.member_1 * _e312.member_1))));
                                            if (_e346 > 0f) {
                                                let _e348 = sqrt(_e346);
                                                let _e349 = -(_e335);
                                                let _e351 = ((_e349 - _e348) / _e330);
                                                if (_e351 < 0.001f) {
                                                    phi_6336_ = ((_e349 + _e348) / _e330);
                                                } else {
                                                    phi_6336_ = _e351;
                                                }
                                                let _e356 = phi_6336_;
                                                if (_e356 >= 0.001f) {
                                                    if (_e356 < _e308.member) {
                                                        let _e360 = (_e356 * _e301.x);
                                                        let _e361 = (_e356 * _e301.y);
                                                        let _e362 = (_e356 * _e301.z);
                                                        phi_6375_ = type_23(_e356, (_e303 + vec3<f32>(_e360, _e361, _e362)), vec3<f32>((((_e303.x + _e360) - _e312.member.x) / _e312.member_1), (((_e303.y + _e361) - _e312.member.y) / _e312.member_1), (((_e303.z + _e362) - _e312.member.z) / _e312.member_1)), _e312.member_2, _e312.member_3, _e312.member_4);
                                                    } else {
                                                        phi_6375_ = _e308;
                                                    }
                                                    let _e380 = phi_6375_;
                                                    phi_6377_ = _e380;
                                                } else {
                                                    phi_6377_ = _e308;
                                                }
                                                let _e382 = phi_6377_;
                                                phi_6379_ = _e382;
                                            } else {
                                                phi_6379_ = _e308;
                                            }
                                            let _e384 = phi_6379_;
                                            phi_6382_ = (_e306 + 1u);
                                            phi_6383_ = _e384;
                                        } else {
                                            phi_6382_ = u32();
                                            phi_6383_ = type_23();
                                        }
                                        let _e387 = phi_6382_;
                                        let _e389 = phi_6383_;
                                        continue;
                                        continuing {
                                            phi_6274_ = _e387;
                                            phi_6275_ = _e389;
                                            break if !(_e309);
                                        }
                                    }
                                    let _e392 = local_53;
                                    let _e394 = (_e392.member >= 1000000000000000000000000000000f);
                                    if _e394 {
                                        phi_6701_ = _e295;
                                        phi_6231_ = vec3<f32>();
                                        phi_6232_ = u32();
                                        phi_6233_ = vec3<f32>();
                                        phi_6234_ = vec3<f32>();
                                        phi_6235_ = false;
                                        phi_6236_ = false;
                                    } else {
                                        let _e396 = local_54;
                                        if ((((_e301.x * _e396.member_2.x) + (_e301.y * _e396.member_2.y)) + (_e301.z * _e396.member_2.z)) < 0f) {
                                            phi_6137_ = _e396.member_2;
                                        } else {
                                            phi_6137_ = -(_e396.member_2);
                                        }
                                        let _e412 = phi_6137_;
                                        let _e414 = local_55;
                                        let _e418 = local_56;
                                        let _e421 = local_57;
                                        let _e426 = (_e295 ^ (_e295 << bitcast<u32>(13i)));
                                        let _e429 = (_e426 ^ (_e426 >> bitcast<u32>(17i)));
                                        let _e432 = (_e429 ^ (_e429 << bitcast<u32>(5i)));
                                        let _e435 = (_e432 ^ (_e432 << bitcast<u32>(13i)));
                                        let _e438 = (_e435 ^ (_e435 >> bitcast<u32>(17i)));
                                        let _e441 = (_e438 ^ (_e438 << bitcast<u32>(5i)));
                                        if (_e421.member_4 == 0u) {
                                            let _e513 = (1f - (f32((_e432 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e517 = (f32((_e441 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e519 = (1f - (_e513 * _e513));
                                            if (_e519 != _e519) {
                                                phi_6551_ = true;
                                            } else {
                                                phi_6551_ = (0f >= _e519);
                                            }
                                            let _e523 = phi_6551_;
                                            let _e525 = sqrt(select(_e519, 0f, _e523));
                                            let _e527 = (_e525 * cos(_e517));
                                            let _e529 = (_e525 * sin(_e517));
                                            let _e532 = (_e412.x + _e527);
                                            let _e534 = (_e412.y + _e529);
                                            let _e536 = (_e412.z + _e513);
                                            phi_6221_ = select((_e412 + vec3<f32>(_e527, _e529, _e513)), _e412, vec3(((((_e532 * _e532) + (_e534 * _e534)) + (_e536 * _e536)) < 0.00000001f)));
                                            phi_6222_ = true;
                                        } else {
                                            let _e449 = (_e301 * (1f / sqrt((((_e301.x * _e301.x) + (_e301.y * _e301.y)) + (_e301.z * _e301.z)))));
                                            let _e461 = (2f * (((_e449.x * _e412.x) + (_e449.y * _e412.y)) + (_e449.z * _e412.z)));
                                            let _e466 = (_e449 - vec3<f32>((_e461 * _e412.x), (_e461 * _e412.y), (_e461 * _e412.z)));
                                            let _e468 = local_58;
                                            let _e474 = (1f - (f32((_e432 >> bitcast<u32>(8i))) * 0.00000011920929f));
                                            let _e478 = (f32((_e441 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                            let _e480 = (1f - (_e474 * _e474));
                                            if (_e480 != _e480) {
                                                phi_6487_ = true;
                                            } else {
                                                phi_6487_ = (0f >= _e480);
                                            }
                                            let _e484 = phi_6487_;
                                            let _e486 = sqrt(select(_e480, 0f, _e484));
                                            let _e491 = (_e468.member_5 * (_e486 * cos(_e478)));
                                            let _e492 = (_e468.member_5 * (_e486 * sin(_e478)));
                                            let _e493 = (_e468.member_5 * _e474);
                                            phi_6221_ = (_e466 + vec3<f32>(_e491, _e492, _e493));
                                            phi_6222_ = select(true, false, (((((_e466.x + _e491) * _e412.x) + ((_e466.y + _e492) * _e412.y)) + ((_e466.z + _e493) * _e412.z)) <= 0f));
                                        }
                                        let _e547 = phi_6221_;
                                        let _e549 = phi_6222_;
                                        if _e549 {
                                            phi_6227_ = (_e299 + 1u);
                                        } else {
                                            phi_6227_ = u32();
                                        }
                                        let _e552 = phi_6227_;
                                        phi_6701_ = _e441;
                                        phi_6231_ = (_e297 * _e414.member_3);
                                        phi_6232_ = _e552;
                                        phi_6233_ = _e547;
                                        phi_6234_ = _e418.member_1;
                                        phi_6235_ = _e549;
                                        phi_6236_ = select(true, false, _e549);
                                    }
                                    let _e555 = phi_6701_;
                                    let _e557 = phi_6231_;
                                    let _e559 = phi_6232_;
                                    let _e561 = phi_6233_;
                                    let _e563 = phi_6234_;
                                    let _e565 = phi_6235_;
                                    let _e567 = phi_6236_;
                                    phi_6700_ = _e555;
                                    phi_6238_ = _e557;
                                    phi_6239_ = _e559;
                                    phi_6240_ = _e561;
                                    phi_6241_ = _e563;
                                    phi_6242_ = _e565;
                                    phi_6243_ = _e394;
                                    phi_6245_ = _e567;
                                } else {
                                    phi_6700_ = _e295;
                                    phi_6238_ = vec3<f32>();
                                    phi_6239_ = u32();
                                    phi_6240_ = vec3<f32>();
                                    phi_6241_ = vec3<f32>();
                                    phi_6242_ = false;
                                    phi_6243_ = false;
                                    phi_6245_ = false;
                                }
                                let _e569 = phi_6700_;
                                let _e571 = phi_6238_;
                                let _e573 = phi_6239_;
                                let _e575 = phi_6240_;
                                let _e577 = phi_6241_;
                                let _e579 = phi_6242_;
                                let _e581 = phi_6243_;
                                let _e583 = phi_6245_;
                                local_59 = _e581;
                                local_64 = _e581;
                                local_65 = select(true, false, _e304);
                                local_66 = _e581;
                                local_67 = _e583;
                                local_68 = _e581;
                                continue;
                                continuing {
                                    phi_6694_ = _e569;
                                    phi_6106_ = _e571;
                                    phi_6107_ = _e573;
                                    phi_6108_ = _e575;
                                    phi_6109_ = _e577;
                                    break if !(_e579);
                                }
                            }
                            let _e587 = local_59;
                            if _e587 {
                                let _e589 = local_60;
                                let _e593 = local_61;
                                let _e598 = local_62;
                                let _e605 = ((_e593.y * (1f / sqrt((((_e589.x * _e589.x) + (_e593.y * _e593.y)) + (_e598.z * _e598.z))))) + 1f);
                                let _e607 = (1f - (0.5f * _e605));
                                let _e614 = local_63;
                                phi_6253_ = (_e614 * vec3<f32>((_e607 + (_e605 * 0.25f)), (_e607 + (_e605 * 0.35f)), 1f));
                            } else {
                                phi_6253_ = vec3<f32>();
                            }
                            let _e617 = phi_6253_;
                            let _e619 = local_64;
                            let _e621 = local_65;
                            let _e622 = select(_e621, false, _e619);
                            let _e624 = local_66;
                            let _e626 = local_67;
                            let _e628 = select(select(_e626, false, _e624), false, _e622);
                            let _e630 = local_68;
                            phi_2451_ = (_e207 + select(vec3<f32>(0f, 0f, 0f), select(_e617, vec3<f32>(0f, 0f, 0f), vec3(_e628)), vec3(select(select(_e630, false, _e622), true, _e628))));
                            phi_2454_ = (_e209 + 1u);
                        } else {
                            phi_2451_ = vec3<f32>();
                            phi_2454_ = u32();
                        }
                        let _e640 = phi_2451_;
                        let _e642 = phi_2454_;
                        continue;
                        continuing {
                            phi_2450_ = _e640;
                            phi_2453_ = _e642;
                            break if !(_e213);
                        }
                    }
                    let _e645 = local_69;
                    let _e646 = f32(_e645);
                    let _e648 = local_70;
                    let _e652 = local_71;
                    let _e656 = local_72;
                    let _e659 = (_e205 * 3u);
                    if (_e659 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e659] = (_e648.x / _e646);
                    let _e663 = (_e659 + 1u);
                    if (_e663 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e663] = (_e652.y / _e646);
                    let _e667 = (_e659 + 2u);
                    if (_e667 < _e189) {
                    } else {
                        break;
                    }
                    global_3.member[_e667] = (_e656.z / _e646);
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
fn gallery_cs(@builtin(global_invocation_id) param_1: vec3<u32>) {
    global = param_1;
    function_1();
}

@compute @workgroup_size(8, 8, 1) 
fn render_v2_cs(@builtin(global_invocation_id) param_2: vec3<u32>) {
    global = param_2;
    function_2();
}

@compute @workgroup_size(64, 1, 1) 
fn physarum_spawn_cs(@builtin(global_invocation_id) param_3: vec3<u32>) {
    global = param_3;
    function_3();
}

@compute @workgroup_size(64, 1, 1) 
fn physarum_update_cs(@builtin(global_invocation_id) param_4: vec3<u32>) {
    global = param_4;
    function_4();
}

@compute @workgroup_size(16, 16, 1) 
fn matmul_unchecked_cs(@builtin(global_invocation_id) param_5: vec3<u32>) {
    global = param_5;
    function_5();
}

@compute @workgroup_size(8, 8, 1) 
fn physarum_diffuse_cs(@builtin(global_invocation_id) param_6: vec3<u32>) {
    global = param_6;
    function_6();
}

@compute @workgroup_size(16, 16, 1) 
fn matmul_cs(@builtin(global_invocation_id) param_7: vec3<u32>) {
    global = param_7;
    function_7();
}

@compute @workgroup_size(8, 8, 1) 
fn render_cs(@builtin(global_invocation_id) param_8: vec3<u32>) {
    global = param_8;
    function_8();
}
