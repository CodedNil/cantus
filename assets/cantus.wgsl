struct render_shared_RipplePulse {
    origin: vec2<f32>,
    start_time: f32,
    strength: f32,
}

struct render_shared_FrameData {
    screen_size: vec2<f32>,
    mouse_pos: vec2<f32>,
    panel_height: f32,
    mouse_pressure: f32,
    playhead_x: f32,
    px_per_ms: f32,
    status_width: f32,
    time: f32,
    weather_hour: f32,
    ripples: array<render_shared_RipplePulse, 4>,
}

struct type_6 {
    member: array<render_shared_FrameData>,
}

struct render_track_AudioFeatures {
    energy: f32,
    danceability: f32,
    acousticness: f32,
    tempo: f32,
    valence: f32,
    instrumentalness: f32,
    loudness: f32,
}

struct render_text_Line {
    min: vec2<f32>,
    max: vec2<f32>,
    origin: vec2<f32>,
    size: f32,
    weight: f32,
    count: u32,
    first: u32,
}

struct render_text_PlacedGlyph {
    x: f32,
    glyph: u32,
}

struct render_text_Text_2_u0020_128_ {
    lines: array<render_text_Line, 2>,
    glyphs: array<render_text_PlacedGlyph, 128>,
    line_count: u32,
}

struct render_track_TrackPill {
    x: f32,
    width: f32,
    colors: array<u32, 4>,
    image_index: i32,
    rating: i32,
    primary_playlist_count: u32,
    secondary_playlist_count: u32,
    visibility: f32,
    primary_alpha: f32,
    secondary_expansion: f32,
    seed: f32,
    effects: render_track_AudioFeatures,
    playlist_images: array<i32, 8>,
    text: render_text_Text_2_u0020_128_,
}

struct type_13 {
    member: array<render_track_TrackPill>,
}

struct render_text_Glyph {
    min: vec2<f32>,
    max: vec2<f32>,
    start: u32,
    count: u32,
}

struct type_16 {
    member: array<render_text_Glyph>,
}

struct render_text_Edge {
    low_start: vec2<f32>,
    low_control: vec2<f32>,
    low_end: vec2<f32>,
    high_start: vec2<f32>,
    high_control: vec2<f32>,
    high_end: vec2<f32>,
}

struct type_18 {
    member: array<render_text_Edge>,
}

struct u0028_isthmus_glam_Vec2_u0020_f32_u0029_ {
    unnamed: vec2<f32>,
    unnamed_1: f32,
}

struct u0028_f32_u0020_i32_u0029_ {
    member: f32,
    member_1: i32,
}

struct render_status_UsageHistory {
    samples: array<f32, 40>,
}

struct render_status_ProcessorStatus {
    temperature: f32,
    usage: render_status_UsageHistory,
    memory: render_status_UsageHistory,
}

struct render_tempestas_WeatherCondition {
    fog: f32,
    cloud: f32,
    rain: f32,
    snow: f32,
    lightning: f32,
    hail: f32,
}

struct render_text_Text_2_u0020_32_ {
    lines: array<render_text_Line, 2>,
    glyphs: array<render_text_PlacedGlyph, 32>,
    line_count: u32,
}

struct render_status_StatusPill {
    battery_level: f32,
    volume: f32,
    audio_spectrum: array<f32, 7>,
    history_scroll: f32,
    cpu: render_status_ProcessorStatus,
    gpu: render_status_ProcessorStatus,
    power_action: i32,
    power_progress: f32,
    power_hover: i32,
    sun_height: f32,
    conditions: render_tempestas_WeatherCondition,
    text: render_text_Text_2_u0020_32_,
}

struct type_26 {
    member: array<render_status_StatusPill>,
}

struct u0028_isthmus_glam_Vec4_u0020_isthmus_glam_Vec2_u0029_ {
    unnamed: vec4<f32>,
    unnamed_1: vec2<f32>,
}

struct render_playhead_PlayheadState {
    bar_split: f32,
    icon_presence: f32,
    icon_morph: f32,
}

struct type_28 {
    member: array<render_playhead_PlayheadState>,
}

struct render_particles_Particle {
    spawn_pos: vec2<f32>,
    spawn_vel: vec2<f32>,
    end_time: f32,
    duration: f32,
    rgb: u32,
}

struct type_30 {
    member: array<render_particles_Particle>,
}

struct isthmus_Vertex_render_particles_Varyings {
    varyings: u0028_isthmus_glam_Vec4_u0020_isthmus_glam_Vec2_u0029_,
    position: vec4<f32>,
}

struct render_text_Text_76_u0020_512_ {
    lines: array<render_text_Line, 76>,
    glyphs: array<render_text_PlacedGlyph, 512>,
    line_count: u32,
}

struct render_tempestas_WeatherSurface {
    x: f32,
    calendar_expansion: f32,
    sun_hours: array<f32, 2>,
    hourly_start: f32,
    today_index: i32,
    month_range: array<u32, 2>,
    text_hover: array<f32, 3>,
    hourly_conditions: array<render_tempestas_WeatherCondition, 6>,
    daily_conditions: array<render_tempestas_WeatherCondition, 5>,
    text: render_text_Text_76_u0020_512_,
}

struct type_39 {
    member: array<render_tempestas_WeatherSurface>,
}

struct u0028_render_tempestas_WeatherCondition_u0020_f32_u0029_ {
    unnamed: render_tempestas_WeatherCondition,
    unnamed_1: f32,
}

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec2<f32>,
    @location(1) @interpolate(flat) member_2: u32,
}

struct VertexOutput_1 {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec4<f32>,
    @location(1) member_2: vec2<f32>,
}

struct VertexOutput_2 {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec2<f32>,
    @location(1) @interpolate(flat) member_2: vec4<f32>,
    @location(2) @interpolate(flat) member_3: u32,
}

var<private> vertex_5: u32;
var<private> instance_1: u32;
@group(0) @binding(0)
var<storage> frame: type_6;
@group(0) @binding(1)
var<storage> pill: type_13;
var<private> out_position: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> out_pixel_pos: vec2<f32>;
var<private> out_pill_idx: u32;
var<private> pixel_pos_1: vec2<f32>;
var<private> pill_idx_1: u32;
@group(0) @binding(2)
var<storage> glyphs: type_16;
@group(0) @binding(3)
var<storage> edges: type_18;
@group(0) @binding(5)
var sampler_: sampler;
@group(0) @binding(4)
var images: texture_2d_array<f32>;
var<private> out_color: vec4<f32>;
@group(0) @binding(1)
var<storage> pill_1: type_26;
var<private> _isthmus_instance_index_7: u32;
var<private> out_pixel: vec2<f32>;
var<private> out_isthmus_instance_index: u32;
var<private> pixel_2: vec2<f32>;
var<private> _isthmus_instance_index_8: u32;
var<private> out_world_pos: vec2<f32>;
var<private> world_pos_1: vec2<f32>;
@group(0) @binding(1)
var<storage> state: type_28;
@group(0) @binding(1)
var<storage> particle: type_30;
var<private> out_uv: vec2<f32>;
var<private> color_1: vec4<f32>;
var<private> uv_1: vec2<f32>;
@group(0) @binding(1)
var<storage> pill_2: type_39;
var<private> out_weather: vec4<f32>;
var<private> out_isthmus_instance_index_1: u32;
var<private> weather_1: vec4<f32>;
var<private> _isthmus_instance_index_9: u32;

fn cantus_render_shader_pixel_to_ndc(param: vec2<f32>, param_1: vec2<f32>) -> vec4<f32> {
    return vec4<f32>((((param.x / param_1.x) * 2f) - 1f), (1f - ((param.y / param_1.y) * 2f)), 0f, 1f);
}

fn render_track_vertex_impl() {
    var phi_0_: u32;
    var phi_1_: f32;
    var phi_2_: u32;
    var phi_3_: f32;
    var phi_4_: bool;
    var local: f32;
    var phi_5_: bool;
    var phi_6_: bool;
    var phi_7_: bool;
    var phi_8_: bool;
    var phi_9_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e32 = vertex_5;
            let _e33 = instance_1;
            let _e37 = frame.member[0u].mouse_pressure;
            phi_0_ = 0u;
            phi_1_ = (_e37 * 8f);
            loop {
                let _e40 = phi_0_;
                let _e42 = phi_1_;
                local = _e42;
                let _e43 = (_e40 < 4u);
                if _e43 {
                    if _e43 {
                    } else {
                        phi_4_ = true;
                        break;
                    }
                    let _e49 = frame.member[0u].ripples[_e40].start_time;
                    let _e55 = frame.member[0u].ripples[_e40].strength;
                    let _e59 = frame.member[0u].time;
                    let _e61 = ((_e59 - _e49) * 1.2f);
                    let _e63 = select(_e61, 0f, (_e61 < 0f));
                    let _e66 = (1f - select(_e63, 1f, (_e63 > 1f)));
                    phi_2_ = (_e40 + 1u);
                    phi_3_ = (_e42 + (((_e55 * _e66) * _e66) * 11f));
                } else {
                    phi_2_ = u32();
                    phi_3_ = f32();
                }
                let _e73 = phi_2_;
                let _e75 = phi_3_;
                continue;
                continuing {
                    phi_0_ = _e73;
                    phi_1_ = _e75;
                    phi_4_ = false;
                    break if !(_e43);
                }
            }
            let _e78 = phi_4_;
            if _e78 {
                break;
            }
            let _e80 = local;
            let _e81 = (_e80 * 0.5f);
            let _e82 = (18f + _e81);
            let _e86 = pill.member[_e33].width;
            let _e90 = frame.member[0u].panel_height;
            let _e94 = pill.member[_e33].x;
            let _e96 = (_e94 + (_e86 * 0.5f));
            let _e102 = pill.member[_e33].rating;
            let _e108 = pill.member[_e33].primary_playlist_count;
            let _e110 = (select(0f, 5f, (_e102 >= 0i)) + f32(_e108));
            let _e114 = pill.member[_e33].secondary_expansion;
            let _e120 = pill.member[_e33].secondary_playlist_count;
            let _e121 = f32(_e120);
            let _e125 = pill.member[_e33].primary_alpha;
            let _e126 = (_e110 - 1f);
            if (_e126 != _e126) {
                phi_5_ = true;
            } else {
                phi_5_ = (0f >= _e126);
            }
            let _e130 = phi_5_;
            let _e136 = select(0f, 1f, ((_e110 * _e125) > 0f));
            let _e137 = (((select(_e126, 0f, _e130) * 9f) + 32.4f) * _e136);
            let _e138 = (32.4f * _e136);
            let _e139 = (_e121 - 1f);
            if (_e139 != _e139) {
                phi_6_ = true;
            } else {
                phi_6_ = (0f >= _e139);
            }
            let _e143 = phi_6_;
            let _e151 = select(0f, 1f, ((_e121 * _e114) > 0f));
            let _e152 = (((((select(_e139, 0f, _e143) * 18f) * _e114) * 0.5f) + 32.4f) * _e151);
            let _e153 = (32.4f * _e151);
            let _e155 = select(_e152, _e137, (_e137 > _e152));
            let _e158 = (_e94 - _e82);
            let _e159 = (_e96 - _e155);
            if (_e158 != _e158) {
                phi_7_ = true;
            } else {
                phi_7_ = (_e159 <= _e158);
            }
            let _e163 = phi_7_;
            let _e164 = select(_e158, _e159, _e163);
            let _e165 = (-12f - _e81);
            let _e167 = ((_e94 + _e86) + _e82);
            let _e168 = (_e96 + _e155);
            if (_e167 != _e167) {
                phi_8_ = true;
            } else {
                phi_8_ = (_e168 >= _e167);
            }
            let _e172 = phi_8_;
            let _e175 = ((6f + _e90) + _e82);
            let _e177 = (((((_e90 * 0.975f) + 3f) + (18f * _e114)) + -5.4f) + select(_e153, _e138, (_e138 > _e153)));
            if (_e175 != _e175) {
                phi_9_ = true;
            } else {
                phi_9_ = (_e177 >= _e175);
            }
            let _e181 = phi_9_;
            let _e192 = (_e164 + (f32((_e32 & 1u)) * (select(_e167, _e168, _e172) - _e164)));
            let _e193 = (_e165 + (f32((_e32 >> bitcast<u32>(1i))) * (select(_e175, _e177, _e181) - _e165)));
            let _e198 = frame.member[0u].screen_size[0u];
            let _e203 = frame.member[0u].screen_size[1u];
            let _e206 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e192, _e193), vec2<f32>(_e198, _e203));
            out_position = _e206;
            out_pixel_pos[0u] = _e192;
            out_pixel_pos[1u] = _e193;
            out_pill_idx = _e33;
            break;
        }
    }
    return;
}

fn cantus_render_text_edge_distance(param_2: render_text_Edge, param_3: f32, param_4: vec2<f32>) -> u0028_f32_u0020_i32_u0029_ {
    var phi_0_: bool;
    var phi_1_: bool;
    var phi_2_: bool;
    var phi_3_: f32;
    var phi_4_: i32;
    var phi_5_: bool;
    var phi_6_: bool;
    var phi_7_: bool;
    var phi_8_: f32;
    var phi_9_: i32;
    var phi_10_: bool;
    var local_1: f32;
    var local_2: f32;
    var local_3: f32;
    var local_4: f32;
    var local_5: f32;
    var local_6: f32;
    var phi_11_: bool;
    var phi_12_: i32;
    var phi_13_: vec2<f32>;
    var phi_14_: i32;
    var phi_15_: i32;
    var phi_16_: i32;
    var phi_17_: i32;
    var phi_18_: i32;
    var phi_19_: i32;
    var phi_20_: i32;
    var phi_21_: vec2<f32>;
    var phi_22_: i32;
    var local_7: i32;

    let _e27 = (1f - param_3);
    let _e32 = ((param_2.low_start.x * _e27) + (param_2.high_start.x * param_3));
    let _e33 = ((param_2.low_start.y * _e27) + (param_2.high_start.y * param_3));
    let _e46 = ((param_2.low_control.x * _e27) + (param_2.high_control.x * param_3));
    let _e47 = ((param_2.low_control.y * _e27) + (param_2.high_control.y * param_3));
    let _e60 = ((param_2.low_end.x * _e27) + (param_2.high_end.x * param_3));
    let _e61 = ((param_2.low_end.y * _e27) + (param_2.high_end.y * param_3));
    let _e62 = (_e60 - _e32);
    let _e63 = (_e61 - _e33);
    let _e64 = (param_4.x - _e32);
    let _e65 = (param_4.y - _e33);
    let _e71 = ((_e62 * _e62) + (_e63 * _e63));
    if (_e71 != _e71) {
        phi_0_ = true;
    } else {
        phi_0_ = (0.00000001f >= _e71);
    }
    let _e75 = phi_0_;
    let _e77 = (((_e64 * _e62) + (_e65 * _e63)) / select(_e71, 0.00000001f, _e75));
    if (_e77 != _e77) {
        phi_1_ = true;
    } else {
        phi_1_ = (0f >= _e77);
    }
    let _e81 = phi_1_;
    let _e82 = select(_e77, 0f, _e81);
    if (_e82 != _e82) {
        phi_2_ = true;
    } else {
        phi_2_ = (1f <= _e82);
    }
    let _e86 = phi_2_;
    phi_3_ = select(_e82, 1f, _e86);
    phi_4_ = 0i;
    loop {
        let _e97 = phi_3_;
        let _e99 = phi_4_;
        local_1 = _e97;
        local_2 = _e97;
        local_3 = _e97;
        local_4 = _e97;
        local_5 = _e97;
        local_6 = _e97;
        let _e100 = (_e99 < 3i);
        if _e100 {
            let _e101 = (1f - _e97);
            let _e107 = ((2f * _e101) * _e97);
            let _e128 = ((((_e46 - _e32) * _e101) + ((_e60 - _e46) * _e97)) * 2f);
            let _e129 = ((((_e47 - _e33) * _e101) + ((_e61 - _e47) * _e97)) * 2f);
            let _e130 = (((((_e32 * _e101) * _e101) + (_e46 * _e107)) + ((_e60 * _e97) * _e97)) - param_4.x);
            let _e131 = (((((_e33 * _e101) * _e101) + (_e47 * _e107)) + ((_e61 * _e97) * _e97)) - param_4.y);
            let _e138 = (((_e128 * _e128) + (_e129 * _e129)) + ((_e130 * (((_e32 - (_e46 * 2f)) + _e60) * 2f)) + (_e131 * (((_e33 - (_e47 * 2f)) + _e61) * 2f))));
            let _e139 = abs(_e138);
            if (_e139 != _e139) {
                phi_5_ = true;
            } else {
                phi_5_ = (0.00000001f >= _e139);
            }
            let _e143 = phi_5_;
            let _e155 = (_e97 - (((_e130 * _e128) + (_e131 * _e129)) / bitcast<f32>(((bitcast<u32>(select(_e139, 0.00000001f, _e143)) & 2147483647u) | (bitcast<u32>(_e138) & 2147483648u)))));
            if (_e155 != _e155) {
                phi_6_ = true;
            } else {
                phi_6_ = (0f >= _e155);
            }
            let _e159 = phi_6_;
            let _e160 = select(_e155, 0f, _e159);
            if (_e160 != _e160) {
                phi_7_ = true;
            } else {
                phi_7_ = (1f <= _e160);
            }
            let _e164 = phi_7_;
            phi_8_ = select(_e160, 1f, _e164);
            phi_9_ = (_e99 + 1i);
        } else {
            phi_8_ = f32();
            phi_9_ = i32();
        }
        let _e168 = phi_8_;
        let _e170 = phi_9_;
        continue;
        continuing {
            phi_3_ = _e168;
            phi_4_ = _e170;
            break if !(_e100);
        }
    }
    let _e174 = ((_e64 * _e64) + (_e65 * _e65));
    let _e175 = (param_4.x - _e60);
    let _e176 = (param_4.y - _e61);
    let _e179 = ((_e175 * _e175) + (_e176 * _e176));
    if (_e174 != _e174) {
        phi_10_ = true;
    } else {
        phi_10_ = (_e179 <= _e174);
    }
    let _e183 = phi_10_;
    let _e184 = select(_e174, _e179, _e183);
    let _e187 = local_1;
    let _e188 = (1f - _e187);
    let _e195 = local_2;
    let _e196 = ((2f * _e188) * _e195);
    let _e202 = local_3;
    let _e205 = local_4;
    let _e208 = local_5;
    let _e211 = local_6;
    let _e215 = (param_4.x - ((((_e32 * _e188) * _e188) + (_e46 * _e196)) + ((_e60 * _e202) * _e208)));
    let _e216 = (param_4.y - ((((_e33 * _e188) * _e188) + (_e47 * _e196)) + ((_e61 * _e205) * _e211)));
    let _e219 = ((_e215 * _e215) + (_e216 * _e216));
    if (_e184 != _e184) {
        phi_11_ = true;
    } else {
        phi_11_ = (_e219 <= _e184);
    }
    let _e223 = phi_11_;
    phi_12_ = 1i;
    phi_13_ = vec2<f32>(_e32, _e33);
    phi_14_ = 0i;
    loop {
        let _e226 = phi_12_;
        let _e228 = phi_13_;
        let _e230 = phi_14_;
        local_7 = _e230;
        let _e231 = (_e226 <= 3i);
        if _e231 {
            let _e233 = (f32(_e226) * 0.33333334f);
            let _e234 = (1f - _e233);
            let _e240 = ((2f * _e234) * _e233);
            let _e249 = ((((_e32 * _e234) * _e234) + (_e46 * _e240)) + ((_e60 * _e233) * _e233));
            let _e250 = ((((_e33 * _e234) * _e234) + (_e47 * _e240)) + ((_e61 * _e233) * _e233));
            let _e259 = (((_e249 - _e228.x) * (param_4.y - _e228.y)) - ((_e250 - _e228.y) * (param_4.x - _e228.x)));
            if (_e228.y <= param_4.y) {
                if (_e250 > param_4.y) {
                    if (_e259 > 0f) {
                        phi_17_ = (_e230 + 1i);
                    } else {
                        phi_17_ = _e230;
                    }
                    let _e272 = phi_17_;
                    phi_18_ = _e272;
                } else {
                    phi_18_ = _e230;
                }
                let _e274 = phi_18_;
                phi_19_ = _e274;
            } else {
                if (_e250 <= param_4.y) {
                    if (_e259 < 0f) {
                        phi_15_ = (_e230 - 1i);
                    } else {
                        phi_15_ = _e230;
                    }
                    let _e265 = phi_15_;
                    phi_16_ = _e265;
                } else {
                    phi_16_ = _e230;
                }
                let _e267 = phi_16_;
                phi_19_ = _e267;
            }
            let _e276 = phi_19_;
            phi_20_ = (_e226 + 1i);
            phi_21_ = vec2<f32>(_e249, _e250);
            phi_22_ = _e276;
        } else {
            phi_20_ = i32();
            phi_21_ = vec2<f32>();
            phi_22_ = i32();
        }
        let _e280 = phi_20_;
        let _e282 = phi_21_;
        let _e284 = phi_22_;
        continue;
        continuing {
            phi_12_ = _e280;
            phi_13_ = _e282;
            phi_14_ = _e284;
            break if !(_e231);
        }
    }
    let _e287 = local_7;
    return u0028_f32_u0020_i32_u0029_(select(_e184, _e219, _e223), _e287);
}

fn cantus_render_shader_hash(param_5: vec2<f32>) -> vec2<f32> {
    let _e31 = ((bitcast<u32>(select(0i, select(select(i32(param_5.y), i32(-2147483648), (param_5.y < -2147483600f)), 2147483647i, (param_5.y > 2147483500f)), (param_5.y == param_5.y))) * 1664525u) + 1013904223u);
    let _e33 = (((bitcast<u32>(select(0i, select(select(i32(param_5.x), i32(-2147483648), (param_5.x < -2147483600f)), 2147483647i, (param_5.x > 2147483500f)), (param_5.x == param_5.x))) * 1664525u) + 1013904223u) + (_e31 * 1664525u));
    let _e35 = (_e31 + (_e33 * 1664525u));
    let _e41 = (_e35 ^ (_e35 >> bitcast<u32>(16i)));
    let _e43 = ((_e33 ^ (_e33 >> bitcast<u32>(16i))) + (_e41 * 1664525u));
    let _e45 = (_e41 + (_e43 * 1664525u));
    return vec2<f32>((f32((_e43 ^ (_e43 >> bitcast<u32>(16i)))) * 0.00000000023283064f), (f32((_e45 ^ (_e45 >> bitcast<u32>(16i)))) * 0.00000000023283064f));
}

fn cantus_render_shader_simplex_noise(param_6: vec2<f32>) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;
    var phi_2_: bool;

    let _e15 = ((param_6.x + param_6.y) * 0.36602542f);
    let _e18 = floor((param_6.x + _e15));
    let _e19 = floor((param_6.y + _e15));
    let _e23 = ((_e18 + _e19) * 0.21132487f);
    let _e24 = ((param_6.x - _e18) + _e23);
    let _e25 = ((param_6.y - _e19) + _e23);
    let _e28 = select(vec2<f32>(0f, 1f), vec2<f32>(1f, 0f), vec2((_e24 > _e25)));
    let _e33 = ((_e24 - _e28.x) + 0.21132487f);
    let _e34 = ((_e25 - _e28.y) + 0.21132487f);
    let _e35 = (_e24 + -0.57735026f);
    let _e36 = (_e25 + -0.57735026f);
    let _e40 = (0.5f - ((_e24 * _e24) + (_e25 * _e25)));
    if (_e40 != _e40) {
        phi_0_ = true;
    } else {
        phi_0_ = (0f >= _e40);
    }
    let _e44 = phi_0_;
    let _e45 = select(_e40, 0f, _e44);
    let _e50 = cantus_render_shader_hash(vec2<f32>(_e18, _e19));
    let _e64 = (0.5f - ((_e33 * _e33) + (_e34 * _e34)));
    if (_e64 != _e64) {
        phi_1_ = true;
    } else {
        phi_1_ = (0f >= _e64);
    }
    let _e68 = phi_1_;
    let _e69 = select(_e64, 0f, _e68);
    let _e76 = cantus_render_shader_hash(vec2<f32>((_e18 + _e28.x), (_e19 + _e28.y)));
    let _e91 = (0.5f - ((_e35 * _e35) + (_e36 * _e36)));
    if (_e91 != _e91) {
        phi_2_ = true;
    } else {
        phi_2_ = (0f >= _e91);
    }
    let _e95 = phi_2_;
    let _e96 = select(_e91, 0f, _e95);
    let _e103 = cantus_render_shader_hash(vec2<f32>((_e18 + 1f), (_e19 + 1f)));
    return (70f * ((((((_e45 * _e45) * _e45) * _e45) * ((_e24 * ((_e50.x * 2f) - 1f)) + (_e25 * ((_e50.y * 2f) - 1f)))) + ((((_e69 * _e69) * _e69) * _e69) * ((_e33 * ((_e76.x * 2f) - 1f)) + (_e34 * ((_e76.y * 2f) - 1f))))) + ((((_e96 * _e96) * _e96) * _e96) * ((_e35 * ((_e103.x * 2f) - 1f)) + (_e36 * ((_e103.y * 2f) - 1f))))));
}

fn cantus_render_track_plasma_field(param_7: vec2<f32>, param_8: vec4<f32>, param_9: f32, param_10: f32, param_11: f32) -> vec4<f32> {
    let _e17 = ((sin((((param_7.x * param_9) + (param_7.y * param_10)) + param_11)) * 0.5f) + 0.5f);
    let _e23 = ((0.12f + (_e17 * _e17)) * (0.25f + (param_8.w * 3f)));
    return vec4<f32>((param_8.x * _e23), (param_8.y * _e23), (param_8.z * _e23), _e23);
}

fn cantus_render_shader_sd_capsule_box(param_12: vec2<f32>, param_13: f32, param_14: f32) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;

    let _e8 = abs(param_12.y);
    let _e9 = (abs(param_12.x) - param_13);
    let _e11 = select(0f, _e9, (_e9 > 0f));
    let _e13 = select(0f, _e8, (_e8 > 0f));
    if (_e9 != _e9) {
        phi_0_ = true;
    } else {
        phi_0_ = (_e8 >= _e9);
    }
    let _e21 = phi_0_;
    let _e22 = select(_e9, _e8, _e21);
    if (_e22 != _e22) {
        phi_1_ = true;
    } else {
        phi_1_ = (0f <= _e22);
    }
    let _e26 = phi_1_;
    return ((sqrt(((_e11 * _e11) + (_e13 * _e13))) + select(_e22, 0f, _e26)) - param_14);
}

fn render_track_fragment_impl() {
    var phi_0_: vec2<f32>;
    var phi_1_: f32;
    var phi_2_: u32;
    var phi_3_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_4_: bool;
    var phi_5_: vec2<f32>;
    var phi_6_: f32;
    var phi_7_: u32;
    var phi_8_: bool;
    var phi_9_: f32;
    var local_8: vec2<f32>;
    var local_9: vec2<f32>;
    var phi_10_: bool;
    var phi_11_: bool;
    var phi_12_: bool;
    var phi_13_: bool;
    var phi_14_: bool;
    var phi_15_: bool;
    var phi_16_: bool;
    var phi_17_: bool;
    var phi_18_: bool;
    var phi_19_: bool;
    var phi_20_: bool;
    var phi_21_: bool;
    var phi_22_: bool;
    var phi_23_: bool;
    var phi_24_: bool;
    var phi_25_: f32;
    var phi_26_: vec3<f32>;
    var phi_27_: vec3<f32>;
    var local_10: f32;
    var local_11: f32;
    var local_12: f32;
    var local_13: f32;
    var phi_28_: vec4<f32>;
    var phi_29_: u32;
    var phi_30_: bool;
    var phi_31_: bool;
    var phi_32_: bool;
    var phi_33_: bool;
    var phi_34_: bool;
    var phi_35_: vec4<f32>;
    var phi_36_: vec4<f32>;
    var phi_37_: u32;
    var phi_38_: vec4<f32>;
    var phi_39_: vec4<f32>;
    var phi_40_: vec4<f32>;
    var phi_41_: u32;
    var phi_42_: render_shared_RipplePulse;
    var phi_43_: f32;
    var phi_44_: bool;
    var phi_45_: bool;
    var phi_46_: bool;
    var phi_47_: bool;
    var phi_48_: bool;
    var phi_49_: vec4<f32>;
    var phi_50_: vec4<f32>;
    var phi_51_: vec4<f32>;
    var phi_52_: vec4<f32>;
    var phi_53_: vec4<f32>;
    var phi_54_: u32;
    var phi_55_: bool;
    var phi_56_: u32;
    var phi_57_: u32;
    var phi_58_: u32;
    var phi_59_: u32;
    var phi_60_: u32;
    var phi_61_: bool;
    var local_14: u32;
    var phi_62_: bool;
    var phi_63_: u32;
    var phi_64_: f32;
    var phi_65_: u32;
    var phi_66_: i32;
    var phi_67_: f32;
    var phi_68_: bool;
    var phi_69_: u32;
    var phi_70_: i32;
    var phi_71_: f32;
    var phi_72_: bool;
    var local_15: f32;
    var local_16: i32;
    var phi_73_: bool;
    var phi_74_: bool;
    var phi_75_: f32;
    var phi_76_: bool;
    var phi_77_: f32;
    var phi_78_: bool;
    var phi_79_: f32;
    var phi_80_: bool;
    var phi_81_: f32;
    var phi_82_: bool;
    var phi_83_: f32;
    var phi_84_: bool;
    var phi_85_: u32;
    var phi_86_: f32;
    var phi_87_: bool;
    var phi_88_: bool;
    var phi_89_: bool;
    var phi_90_: f32;
    var phi_91_: bool;
    var phi_92_: f32;
    var phi_93_: bool;
    var phi_94_: bool;
    var phi_95_: f32;
    var phi_96_: bool;
    var phi_97_: bool;
    var phi_98_: f32;
    var phi_99_: bool;
    var phi_100_: u32;
    var phi_101_: u32;
    var phi_102_: u32;
    var phi_103_: u32;
    var phi_104_: u32;
    var phi_105_: bool;
    var local_17: u32;
    var phi_106_: bool;
    var phi_107_: u32;
    var phi_108_: f32;
    var phi_109_: u32;
    var phi_110_: i32;
    var phi_111_: f32;
    var phi_112_: bool;
    var phi_113_: u32;
    var phi_114_: i32;
    var phi_115_: f32;
    var phi_116_: bool;
    var local_18: f32;
    var local_19: i32;
    var phi_117_: bool;
    var phi_118_: bool;
    var phi_119_: f32;
    var phi_120_: bool;
    var phi_121_: f32;
    var phi_122_: bool;
    var phi_123_: f32;
    var phi_124_: bool;
    var phi_125_: f32;
    var phi_126_: bool;
    var phi_127_: f32;
    var phi_128_: bool;
    var phi_129_: u32;
    var phi_130_: f32;
    var phi_131_: bool;
    var phi_132_: bool;
    var phi_133_: f32;
    var phi_134_: f32;
    var phi_135_: bool;
    var phi_136_: f32;
    var phi_137_: bool;
    var phi_138_: f32;
    var phi_139_: bool;
    var phi_140_: bool;
    var local_20: vec4<f32>;
    var local_21: vec4<f32>;
    var local_22: vec4<f32>;
    var local_23: vec4<f32>;
    var local_24: vec4<f32>;
    var local_25: f32;
    var local_26: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e164 = pixel_pos_1;
            let _e165 = pill_idx_1;
            let _e167 = arrayLength((&glyphs.member));
            let _e169 = arrayLength((&edges.member));
            let _e175 = pill.member[_e165].x;
            let _e179 = pill.member[_e165].width;
            let _e183 = frame.member[0u].panel_height;
            let _e184 = (_e164.x - _e175);
            let _e185 = (_e164.y - 6f);
            let _e186 = (_e179 * 0.5f);
            let _e187 = (_e183 * 0.5f);
            let _e189 = (_e185 - _e187);
            let _e190 = (_e179 - _e183);
            let _e191 = (_e190 * 0.5f);
            let _e193 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e184 - _e186), _e189), _e191, _e187);
            let _e198 = frame.member[0u].mouse_pos[0u];
            let _e203 = frame.member[0u].mouse_pos[1u];
            let _e209 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e198 - _e175) - _e186), ((_e203 - 6f) - _e187)), _e191, _e187);
            phi_0_ = vec2<f32>(0f, 0f);
            phi_1_ = 0f;
            phi_2_ = 0u;
            loop {
                let _e211 = phi_0_;
                let _e213 = phi_1_;
                let _e215 = phi_2_;
                local_8 = _e211;
                local_9 = _e211;
                local_10 = _e213;
                local_11 = _e213;
                local_12 = _e213;
                local_13 = _e213;
                let _e216 = (_e215 < 4u);
                if _e216 {
                    if _e216 {
                    } else {
                        phi_8_ = true;
                        break;
                    }
                    let _e223 = frame.member[0u].ripples[_e215].origin[0u];
                    let _e230 = frame.member[0u].ripples[_e215].origin[1u];
                    let _e236 = frame.member[0u].ripples[_e215].start_time;
                    let _e242 = frame.member[0u].ripples[_e215].strength;
                    let _e246 = frame.member[0u].time;
                    let _e248 = ((_e246 - _e236) * 1.2f);
                    let _e250 = select(_e248, 0f, (_e248 < 0f));
                    let _e252 = select(_e250, 1f, (_e250 > 1f));
                    let _e253 = (_e164.x - _e223);
                    let _e254 = (_e164.y - _e230);
                    let _e258 = sqrt(((_e253 * _e253) + (_e254 * _e254)));
                    if (_e258 > 0.001f) {
                        phi_3_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e253 / _e258), (_e254 / _e258)), _e258);
                    } else {
                        phi_3_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e258);
                    }
                    let _e266 = phi_3_;
                    let _e276 = ((abs((_e266.unnamed_1 - (_e252 * 600f))) - 80f) * -0.0125f);
                    let _e278 = select(_e276, 0f, (_e276 < 0f));
                    let _e280 = select(_e278, 1f, (_e278 > 1f));
                    let _e286 = (1f - _e252);
                    let _e287 = ((((_e280 * _e280) * (3f - (2f * _e280))) * _e242) * _e286);
                    let _e300 = (_e213 + (_e287 * 0.5f));
                    if (_e300 != _e300) {
                        phi_4_ = true;
                    } else {
                        phi_4_ = (1f <= _e300);
                    }
                    let _e304 = phi_4_;
                    phi_5_ = vec2<f32>((_e211.x + (((_e266.unnamed.x * _e287) * _e286) * 0.5f)), (_e211.y + (((_e266.unnamed.y * _e287) * _e286) * 0.5f)));
                    phi_6_ = select(_e300, 1f, _e304);
                    phi_7_ = (_e215 + 1u);
                } else {
                    phi_5_ = vec2<f32>();
                    phi_6_ = f32();
                    phi_7_ = u32();
                }
                let _e308 = phi_5_;
                let _e310 = phi_6_;
                let _e312 = phi_7_;
                continue;
                continuing {
                    phi_0_ = _e308;
                    phi_1_ = _e310;
                    phi_2_ = _e312;
                    phi_8_ = false;
                    break if !(_e216);
                }
            }
            let _e315 = phi_8_;
            if _e315 {
                break;
            }
            let _e319 = frame.member[0u].mouse_pressure;
            let _e320 = (_e319 > 0f);
            if _e320 {
                let _e321 = (_e164.x - _e198);
                let _e322 = (_e164.y - _e203);
                let _e328 = ((sqrt(((_e321 * _e321) + (_e322 * _e322))) - 150f) * -0.006666667f);
                let _e330 = select(_e328, 0f, (_e328 < 0f));
                let _e332 = select(_e330, 1f, (_e330 > 1f));
                phi_9_ = ((((_e332 * _e332) * (3f - (2f * _e332))) * _e319) * 8f);
            } else {
                phi_9_ = 0f;
            }
            let _e340 = phi_9_;
            let _e342 = local_8;
            let _e345 = local_9;
            let _e347 = (_e184 / _e179);
            let _e348 = (_e185 / _e183);
            let _e349 = (_e347 - 0.5f);
            let _e350 = (_e348 - 0.5f);
            let _e351 = (_e175 + _e186);
            let _e352 = (_e183 * 0.975f);
            let _e353 = (_e352 + 3f);
            let _e357 = pill.member[_e165].rating;
            let _e358 = (_e357 >= 0i);
            let _e359 = select(0f, 5f, _e358);
            let _e363 = pill.member[_e165].primary_playlist_count;
            let _e365 = (_e359 + f32(_e363));
            let _e371 = pill.member[_e165].secondary_expansion;
            let _e373 = (_e353 + (18f * _e371));
            let _e377 = pill.member[_e165].secondary_playlist_count;
            let _e378 = f32(_e377);
            let _e381 = vec2<f32>(_e198, _e203);
            let _e383 = (_e365 - 1f);
            let _e384 = (_e383 != _e383);
            if _e384 {
                phi_10_ = true;
            } else {
                phi_10_ = (0f >= _e383);
            }
            let _e387 = phi_10_;
            let _e390 = vec2<f32>(_e351, (_e352 + -4.4f));
            let _e392 = cantus_render_shader_sd_capsule_box((_e164 - _e390), (select(_e383, 0f, _e387) * 9f), 9f);
            if _e384 {
                phi_11_ = true;
            } else {
                phi_11_ = (0f >= _e383);
            }
            let _e395 = phi_11_;
            let _e399 = cantus_render_shader_sd_capsule_box((_e381 - _e390), (select(_e383, 0f, _e395) * 9f), 9f);
            let _e400 = (10.5f * _e371);
            let _e402 = (_e378 - 1f);
            let _e403 = (_e402 != _e402);
            if _e403 {
                phi_12_ = true;
            } else {
                phi_12_ = (0f >= _e402);
            }
            let _e406 = phi_12_;
            let _e411 = vec2<f32>(_e351, (_e373 + -5.4f));
            let _e413 = cantus_render_shader_sd_capsule_box((_e164 - _e411), (((select(_e402, 0f, _e406) * 18f) * _e371) * 0.5f), _e400);
            if _e403 {
                phi_13_ = true;
            } else {
                phi_13_ = (0f >= _e402);
            }
            let _e416 = phi_13_;
            let _e422 = cantus_render_shader_sd_capsule_box((_e381 - _e411), (((select(_e402, 0f, _e416) * 18f) * _e371) * 0.5f), _e400);
            let _e426 = pill.member[_e165].primary_alpha;
            let _e429 = (0.5f + ((_e392 - _e193) * 0.05f));
            let _e431 = select(_e429, 0f, (_e429 < 0f));
            let _e433 = select(_e431, 1f, (_e431 > 1f));
            let _e443 = (_e193 + ((((_e392 + ((_e193 - _e392) * _e433)) - ((10f * _e433) * (1f - _e433))) - _e193) * _e426));
            let _e446 = (0.5f + ((_e399 - _e209) * 0.05f));
            let _e448 = select(_e446, 0f, (_e446 < 0f));
            let _e450 = select(_e448, 1f, (_e448 > 1f));
            let _e460 = (_e209 + ((((_e399 + ((_e209 - _e399) * _e450)) - ((10f * _e450) * (1f - _e450))) - _e209) * _e426));
            let _e462 = select(0f, 1f, (_e371 > 0f));
            let _e465 = (0.5f + ((_e413 - _e443) * 0.046296295f));
            let _e467 = select(_e465, 0f, (_e465 < 0f));
            let _e469 = select(_e467, 1f, (_e467 > 1f));
            let _e482 = (0.5f + ((_e422 - _e460) * 0.046296295f));
            let _e484 = select(_e482, 0f, (_e482 < 0f));
            let _e486 = select(_e484, 1f, (_e484 > 1f));
            let _e498 = (((_e460 + ((((_e422 + ((_e460 - _e422) * _e486)) - ((10.8f * _e486) * (1f - _e486))) - _e460) * _e462)) - 0.5f) * -1f);
            let _e500 = select(_e498, 0f, (_e498 < 0f));
            let _e502 = select(_e500, 1f, (_e500 > 1f));
            let _e512 = (sqrt(((_e342.x * _e342.x) + (_e345.y * _e345.y))) * 22f);
            let _e514 = (((_e340 * ((_e502 * _e502) * (3f - (2f * _e502)))) + _e512) * 0.5f);
            let _e515 = ((_e443 + ((((_e413 + ((_e443 - _e413) * _e469)) - ((10.8f * _e469) * (1f - _e469))) - _e443) * _e462)) - _e514);
            let _e516 = fwidth(_e515);
            if (_e516 != _e516) {
                phi_14_ = true;
            } else {
                phi_14_ = (0.55f >= _e516);
            }
            let _e520 = phi_14_;
            let _e521 = select(_e516, 0.55f, _e520);
            let _e525 = ((_e515 - _e521) / (-(_e521) - _e521));
            let _e527 = select(_e525, 0f, (_e525 < 0f));
            let _e529 = select(_e527, 1f, (_e527 > 1f));
            let _e533 = ((_e529 * _e529) * (3f - (2f * _e529)));
            let _e534 = (_e515 != _e515);
            if _e534 {
                phi_15_ = true;
            } else {
                phi_15_ = (0f >= _e515);
            }
            let _e537 = phi_15_;
            let _e541 = (exp((select(_e515, 0f, _e537) * -0.3f)) * 0.16f);
            if (_e533 != _e533) {
                phi_16_ = true;
            } else {
                phi_16_ = (_e541 >= _e533);
            }
            let _e545 = phi_16_;
            let _e546 = select(_e533, _e541, _e545);
            let _e550 = pill.member[_e165].visibility;
            if ((_e546 * _e550) <= 0.0009765625f) {
                discard;
            }
            if _e534 {
                phi_17_ = true;
            } else {
                phi_17_ = (0f <= _e515);
            }
            let _e555 = phi_17_;
            let _e558 = (1f + (select(_e515, 0f, _e555) * 0.008333334f));
            let _e560 = select(_e558, 0f, (_e558 < 0f));
            let _e562 = select(_e560, 0.6f, (_e560 > 0.6f));
            let _e572 = ((_e348 - ((_e350 * _e562) * 0.08f)) - (_e345.y * 0.04f));
            let _e573 = (((_e347 - ((_e349 * _e562) * 0.08f)) - (_e342.x * 0.04f)) * _e179);
            let _e574 = (_e572 * _e183);
            let _e578 = pill.member[_e165].effects;
            let _e582 = frame.member[0u].time;
            let _e586 = pill.member[_e165].seed;
            let _e589 = ((_e578.tempo - 0.2f) * 2.5f);
            let _e591 = select(_e589, 0f, (_e589 < 0f));
            let _e600 = ((_e582 * ((0.12f + (_e578.energy * 0.25f)) + (select(_e591, 1f, (_e591 > 1f)) * 0.12f))) + _e586);
            let _e605 = ((sin(((_e582 * _e578.tempo) * 31.415928f)) * 0.5f) + 0.5f);
            let _e611 = (((_e605 * _e605) * _e578.danceability) * (0.025f + (_e578.energy * 0.055f)));
            let _e612 = (_e578.energy * 0.55f);
            let _e617 = ((_e612 + (_e578.danceability * 0.25f)) + (_e578.loudness * 0.2f));
            if _e534 {
                phi_18_ = true;
            } else {
                phi_18_ = (0f <= _e515);
            }
            let _e620 = phi_18_;
            let _e623 = (1f + (select(_e515, 0f, _e620) * 0.008333334f));
            let _e625 = select(_e623, 0f, (_e623 < 0f));
            let _e627 = select(_e625, 1f, (_e625 > 1f));
            let _e638 = (_e586 - trunc(_e586));
            let _e643 = ((_e179 / _e183) * ((0.5f + (_e638 * 0.12f)) + (_e617 * 0.18f)));
            if (_e643 != _e643) {
                phi_19_ = true;
            } else {
                phi_19_ = (1.7f >= _e643);
            }
            let _e647 = phi_19_;
            let _e650 = select(0f, _e347, (_e347 > 0f));
            let _e652 = select(0f, _e348, (_e348 > 0f));
            let _e660 = (select(1f, _e652, (_e652 < 1f)) - (((((_e350 * _e627) * _e627) * 0.6f) + _e345.y) * 0.08f));
            let _e661 = ((select(1f, _e650, (_e650 < 1f)) - (((((_e349 * _e627) * _e627) * 0.6f) + _e342.x) * 0.08f)) * select(_e643, 1.7f, _e647));
            let _e672 = (_e600 * 0.8f);
            let _e682 = ((0.14f + (_e617 * 0.2f)) + _e611);
            let _e687 = (_e586 + 1.5707964f);
            let _e692 = pill.member[_e165].colors[0u];
            let _e694 = vec2<f32>((_e661 + ((sin(((_e660 * 4.32f) + _e600)) + cos(((_e661 * 1.3f) - (_e600 * 0.7f)))) * _e682)), ((_e660 * 1.6f) + ((cos(((_e661 * 2.3f) - _e672)) + sin(((_e660 * 2.72f) + (_e600 * 0.6f)))) * _e682)));
            let _e695 = cantus_render_track_plasma_field(_e694, unpack4x8unorm(_e692), 2.1f, 0.7f, _e600);
            let _e700 = pill.member[_e165].colors[1u];
            let _e703 = cantus_render_track_plasma_field(_e694, unpack4x8unorm(_e700), 0.6f, -2.4f, (_e687 - _e672));
            let _e720 = pill.member[_e165].colors[2u];
            let _e724 = cantus_render_track_plasma_field(_e694, unpack4x8unorm(_e720), -1.5f, 1.9f, ((_e600 * 0.65f) + 2f));
            let _e737 = pill.member[_e165].colors[3u];
            let _e738 = unpack4x8unorm(_e737);
            let _e741 = cantus_render_track_plasma_field(_e694, _e738, 2.4f, 1.6f, (_e687 - (_e600 * 0.55f)));
            let _e749 = (((_e695.w + _e703.w) + _e724.w) + _e741.w);
            let _e750 = ((((_e695.x + _e703.x) + _e724.x) + _e741.x) / _e749);
            let _e751 = ((((_e695.y + _e703.y) + _e724.y) + _e741.y) / _e749);
            let _e752 = ((((_e695.z + _e703.z) + _e724.z) + _e741.z) / _e749);
            let _e757 = (((_e750 * 0.2126f) + (_e751 * 0.7152f)) + (_e752 * 0.0722f));
            let _e761 = frame.member[0u].playhead_x;
            let _e762 = (_e761 + 3f);
            let _e766 = ((_e164.x - _e762) / ((_e761 - 3f) - _e762));
            let _e768 = select(_e766, 0f, (_e766 < 0f));
            let _e770 = select(_e768, 1f, (_e768 > 1f));
            let _e779 = pill.member[_e165].effects.valence;
            let _e780 = (_e779 * 0.4f);
            let _e781 = (1.55f + _e780);
            let _e783 = (_e757 * (-0.54999995f - _e780));
            let _e787 = (_e783 + (_e750 * _e781));
            let _e788 = (_e783 + (_e751 * _e781));
            let _e789 = (_e783 + (_e752 * _e781));
            let _e791 = select(0.035f, _e787, (_e787 > 0.035f));
            let _e793 = select(0.035f, _e788, (_e788 > 0.035f));
            let _e795 = select(0.035f, _e789, (_e789 > 0.035f));
            if (_e757 != _e757) {
                phi_20_ = true;
            } else {
                phi_20_ = (0.001f >= _e757);
            }
            let _e805 = phi_20_;
            let _e807 = (0.52f / select(_e757, 0.001f, _e805));
            if (_e807 != _e807) {
                phi_21_ = true;
            } else {
                phi_21_ = (1f <= _e807);
            }
            let _e811 = phi_21_;
            let _e812 = select(_e807, 1f, _e811);
            let _e819 = ((0.96f + (_e779 * 0.06f)) + (_e611 * 0.5f));
            let _e824 = ((_e572 - 0.45f) * 1.8181818f);
            let _e826 = select(_e824, 0f, (_e824 < 0f));
            let _e828 = select(_e826, 1f, (_e826 > 1f));
            let _e834 = (0.84f + (((_e828 * _e828) * (3f - (2f * _e828))) * 0.1f));
            let _e839 = (1f - (0.4f * ((_e770 * _e770) * (3f - (2f * _e770)))));
            let _e859 = (8f - _e578.acousticness);
            let _e863 = (_e582 * (0.35f + _e612));
            let _e866 = ((_e184 / _e859) + (_e863 * (0.16f + (_e638 * 0.08f))));
            let _e867 = ((_e185 / _e859) + (_e863 * (0.055f + (sin((_e586 * 0.7f)) * 0.025f))));
            let _e868 = floor(_e866);
            let _e869 = floor(_e867);
            let _e878 = bitcast<u32>(select(0i, select(select(i32(_e869), i32(-2147483648), (_e869 < -2147483600f)), 2147483647i, (_e869 > 2147483500f)), (_e869 == _e869)));
            let _e886 = bitcast<u32>(select(0i, select(select(i32(_e868), i32(-2147483648), (_e868 < -2147483600f)), 2147483647i, (_e868 > 2147483500f)), (_e868 == _e868)));
            let _e888 = (bitcast<u32>((_e586 + 2.71f)) * 2654435761u);
            let _e894 = (((_e886 ^ _e888) * 1664525u) + 1013904223u);
            let _e896 = ((((_e878 ^ _e888) * 1664525u) + 1013904223u) + (_e894 * 1664525u));
            let _e898 = (_e894 + (_e896 * 1664525u));
            let _e906 = ((_e896 ^ (_e896 >> bitcast<u32>(16i))) + ((_e898 ^ (_e898 >> bitcast<u32>(16i))) * 1664525u));
            let _e910 = f32((_e906 ^ (_e906 >> bitcast<u32>(16i))));
            let _e911 = (_e910 * 0.0000000016600825f);
            let _e925 = (_e578.acousticness * 0.09f);
            let _e928 = (bitcast<u32>(_e586) * 2654435761u);
            let _e934 = (((_e878 ^ _e928) * 1664525u) + 1013904223u);
            let _e936 = ((((_e886 ^ _e928) * 1664525u) + 1013904223u) + (_e934 * 1664525u));
            let _e938 = (_e934 + (_e936 * 1664525u));
            let _e946 = ((_e936 ^ (_e936 >> bitcast<u32>(16i))) + ((_e938 ^ (_e938 >> bitcast<u32>(16i))) * 1664525u));
            let _e954 = (((f32((_e946 ^ (_e946 >> bitcast<u32>(16i)))) * 0.00000000023283064f) - (0.985f - _e925)) / (_e925 + 0.014999986f));
            let _e956 = select(_e954, 0f, (_e954 < 0f));
            let _e958 = select(_e956, 1f, (_e956 > 1f));
            let _e967 = (((_e866 - _e868) - 0.5f) - ((_e910 * 0.00000000013038516f) - 0.28f));
            let _e968 = (((_e867 - _e869) - 0.5f) - (((_e911 - trunc(_e911)) * 0.56f) - 0.28f));
            let _e974 = ((sqrt(((_e967 * _e967) + (_e968 * _e968))) - 0.06f) * 4.5454545f);
            let _e976 = select(_e974, 0f, (_e974 < 0f));
            let _e978 = select(_e976, 1f, (_e976 > 1f));
            let _e991 = (((((_e958 * _e958) * (3f - (2f * _e958))) * (1f - ((_e978 * _e978) * (3f - (2f * _e978))))) * ((sin(((_e582 * ((0.7f + (_e910 * 0.00000000020954757f)) + (_e578.energy * 0.8f))) + (_e910 * 0.0000000014629181f))) * 0.5f) + 0.5f)) * (0.12f + (_e578.acousticness * 0.48f)));
            let _e995 = (((((select(0.92f, _e791, (_e791 < 0.92f)) * _e812) * _e819) * _e834) * _e839) + (((_e738.x * 0.75f) + 0.25f) * _e991));
            let _e996 = (((((select(0.92f, _e793, (_e793 < 0.92f)) * _e812) * _e819) * _e834) * _e839) + (((_e738.y * 0.75f) + 0.25f) * _e991));
            let _e997 = (((((select(0.92f, _e795, (_e795 < 0.92f)) * _e812) * _e819) * _e834) * _e839) + (((_e738.z * 0.75f) + 0.25f) * _e991));
            let _e1004 = (_e184 / _e183);
            if (_e578.instrumentalness <= 0.00390625f) {
                phi_25_ = 0f;
            } else {
                let _e1009 = (_e582 * (0.5f + (_e578.energy * 0.35f)));
                let _e1017 = (sin(((_e348 * 1.9f) + _e1009)) * 0.35f);
                let _e1018 = (sin(((_e1004 * 1.5f) - (_e1009 * 0.8f))) * 0.35f);
                let _e1021 = ((_e1009 * 0.05f) + _e586);
                let _e1022 = ((_e1009 * -0.04f) + _e586);
                let _e1030 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1004 * 0.7f) + _e1017) + _e1021), (((_e348 * 0.7f) + _e1018) + _e1022)));
                let _e1033 = (1f - (abs(_e1030) * 2f));
                if (_e1033 != _e1033) {
                    phi_22_ = true;
                } else {
                    phi_22_ = (0f >= _e1033);
                }
                let _e1037 = phi_22_;
                let _e1038 = select(_e1033, 0f, _e1037);
                let _e1040 = ((_e1038 * _e1038) * _e1038);
                let _e1050 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1004 * 1.1f) - _e1017) - (_e1021 * 0.8f)), (((_e348 * 1.1f) - _e1018) - (_e1022 * 0.8f))));
                let _e1053 = (1f - (abs(_e1050) * 2f));
                if (_e1053 != _e1053) {
                    phi_23_ = true;
                } else {
                    phi_23_ = (0f >= _e1053);
                }
                let _e1057 = phi_23_;
                let _e1058 = select(_e1053, 0f, _e1057);
                let _e1060 = ((_e1058 * _e1058) * _e1058);
                if (_e1040 != _e1040) {
                    phi_24_ = true;
                } else {
                    phi_24_ = (_e1060 >= _e1040);
                }
                let _e1064 = phi_24_;
                phi_25_ = ((select(_e1040, _e1060, _e1064) * _e578.instrumentalness) * 0.06f);
            }
            let _e1069 = phi_25_;
            let _e1073 = (_e995 + (((_e995 * 0.25f) + 0.75f) * _e1069));
            let _e1074 = (_e996 + (((_e996 * 0.25f) + 0.75f) * _e1069));
            let _e1075 = (_e997 + (((_e997 * 0.25f) + 0.75f) * _e1069));
            let _e1076 = vec3<f32>(_e1073, _e1074, _e1075);
            let _e1077 = (_e190 + _e187);
            let _e1081 = pill.member[_e165].image_index;
            if (_e1081 >= 0i) {
                let _e1083 = (_e184 - _e1077);
                let _e1084 = abs(_e1083);
                let _e1085 = abs(_e189);
                if (select(_e1085, _e1084, (_e1084 > _e1085)) < _e183) {
                    let _e1089 = (_e187 + _e514);
                    let _e1095 = (_e1089 * 2f);
                    let _e1101 = vec3<f32>(((_e1083 / _e1095) + 0.5f), ((_e189 / _e1095) + 0.5f), f32(_e1081));
                    let _e1107 = textureSample(images, sampler_, vec2<f32>(_e1101.x, _e1101.y), i32(_e1101.z));
                    let _e1109 = (((sqrt(((_e1083 * _e1083) + (_e189 * _e189))) - _e1089) - -4f) * 0.25f);
                    let _e1111 = select(_e1109, 0f, (_e1109 < 0f));
                    let _e1113 = select(_e1111, 1f, (_e1111 > 1f));
                    let _e1120 = ((_e209 - 0.5f) * -1f);
                    let _e1122 = select(_e1120, 0f, (_e1120 < 0f));
                    let _e1124 = select(_e1122, 1f, (_e1122 > 1f));
                    let _e1133 = ((_e193 - (((_e340 * ((_e1124 * _e1124) * (3f - (2f * _e1124)))) + _e512) * 0.5f)) - -0.5f);
                    let _e1135 = select(_e1133, 0f, (_e1133 < 0f));
                    let _e1137 = select(_e1135, 1f, (_e1135 > 1f));
                    let _e1148 = (((1f - ((_e1113 * _e1113) * (3f - (2f * _e1113)))) * (1f - ((_e1137 * _e1137) * (3f - (2f * _e1137))))) * _e1107.w);
                    let _e1149 = (1f - _e1148);
                    phi_26_ = vec3<f32>(((_e1073 * _e1149) + (_e1107.x * _e1148)), ((_e1074 * _e1149) + (_e1107.y * _e1148)), ((_e1075 * _e1149) + (_e1107.z * _e1148)));
                } else {
                    phi_26_ = _e1076;
                }
                let _e1161 = phi_26_;
                phi_27_ = _e1161;
            } else {
                phi_27_ = _e1076;
            }
            let _e1163 = phi_27_;
            let _e1174 = ((_e572 - 0.12f) * -8.333334f);
            let _e1176 = select(_e1174, 0f, (_e1174 < 0f));
            let _e1178 = select(_e1176, 1f, (_e1176 > 1f));
            let _e1185 = ((_e515 - 5f) * -0.125f);
            let _e1187 = select(_e1185, 0f, (_e1185 < 0f));
            let _e1189 = select(_e1187, 1f, (_e1187 > 1f));
            let _e1195 = ((((_e1178 * _e1178) * (3f - (2f * _e1178))) * 0.12f) + (((_e1189 * _e1189) * (3f - (2f * _e1189))) * 0.08f));
            let _e1199 = (_e1163.x + (((_e1163.x * 0.68f) + 0.32f) * _e1195));
            let _e1200 = (_e1163.y + (((_e1163.y * 0.68f) + 0.32f) * _e1195));
            let _e1201 = (_e1163.z + (((_e1163.z * 0.68f) + 0.32f) * _e1195));
            let _e1209 = local_10;
            let _e1210 = (1f - _e1209);
            let _e1215 = local_11;
            let _e1218 = local_12;
            let _e1221 = local_13;
            let _e1229 = vec4<f32>((((_e1199 * _e1210) + (((_e1199 * 1.5f) + 0.1f) * _e1215)) * _e533), (((_e1200 * _e1210) + (((_e1200 * 1.5f) + 0.1f) * _e1218)) * _e533), (((_e1201 * _e1210) + (((_e1201 * 1.5f) + 0.1f) * _e1221)) * _e533), _e546);
            if _e358 {
                if (_e426 > 0f) {
                    phi_28_ = _e1229;
                    phi_29_ = 0u;
                    loop {
                        let _e1232 = phi_28_;
                        let _e1234 = phi_29_;
                        local_24 = _e1232;
                        let _e1235 = (_e1234 < 5u);
                        if _e1235 {
                            let _e1236 = f32(_e1234);
                            if _e384 {
                                phi_30_ = true;
                            } else {
                                phi_30_ = (0f >= _e383);
                            }
                            let _e1239 = phi_30_;
                            let _e1244 = (_e351 + ((_e1236 - (select(_e383, 0f, _e1239) * 0.5f)) * 18f));
                            let _e1245 = (_e352 + 5f);
                            let _e1246 = (_e164.x - _e1244);
                            let _e1247 = (_e164.y - _e1245);
                            let _e1248 = abs(_e1246);
                            let _e1249 = abs(_e1247);
                            if (select(_e1249, _e1248, (_e1248 > _e1249)) < 38.88f) {
                                let _e1256 = ((f32(_e357) - (_e1236 * 2f)) * 0.5f);
                                let _e1258 = select(_e1256, 0f, (_e1256 < 0f));
                                let _e1261 = (_e1244 - _e198);
                                let _e1262 = (_e1245 - _e203);
                                let _e1268 = ((sqrt(((_e1261 * _e1261) + (_e1262 * _e1262))) - 11.3f) * -1f);
                                let _e1270 = select(_e1268, 0f, (_e1268 < 0f));
                                let _e1272 = select(_e1270, 1f, (_e1270 > 1f));
                                let _e1278 = select(_e319, 0f, (_e319 < 0f));
                                let _e1281 = (((_e1272 * _e1272) * (3f - (2f * _e1272))) * select(_e1278, 1f, (_e1278 > 1f)));
                                let _e1283 = (1.05f + (0.63f * _e1281));
                                let _e1284 = (_e1261 * _e1281);
                                let _e1286 = (_e1246 - (_e1284 * 0.5f));
                                let _e1287 = (_e1284 * -0.005f);
                                let _e1288 = sin(_e1287);
                                let _e1289 = cos(_e1287);
                                let _e1292 = ((_e1289 * _e1286) - (_e1288 * _e1247));
                                let _e1295 = ((_e1288 * _e1286) + (_e1289 * _e1247));
                                let _e1299 = (_e1283 * 5.4f);
                                let _e1300 = abs(_e1292);
                                let _e1304 = ((0.809017f * _e1300) + (_e1295 * 0.58778524f));
                                if (_e1304 != _e1304) {
                                    phi_31_ = true;
                                } else {
                                    phi_31_ = (0f >= _e1304);
                                }
                                let _e1308 = phi_31_;
                                let _e1309 = select(_e1304, 0f, _e1308);
                                let _e1312 = (_e1300 - (_e1309 * 1.618034f));
                                let _e1313 = (-(_e1295) - (_e1309 * -1.1755705f));
                                let _e1316 = ((-0.809017f * _e1312) + (-0.58778524f * _e1313));
                                if (_e1316 != _e1316) {
                                    phi_32_ = true;
                                } else {
                                    phi_32_ = (0f >= _e1316);
                                }
                                let _e1320 = phi_32_;
                                let _e1321 = select(_e1316, 0f, _e1320);
                                let _e1326 = abs((_e1312 - (_e1321 * -1.618034f)));
                                let _e1327 = ((_e1313 - (_e1321 * -1.1755705f)) - _e1299);
                                let _e1328 = (_e1283 * 2.031386f);
                                let _e1330 = ((_e1283 * 2.7959628f) - _e1299);
                                let _e1337 = (((_e1326 * _e1328) + (_e1327 * _e1330)) / ((_e1328 * _e1328) + (_e1330 * _e1330)));
                                let _e1339 = select(_e1337, 0f, (_e1337 < 0f));
                                let _e1341 = select(_e1339, 1f, (_e1339 > 1f));
                                let _e1347 = (_e1326 - (_e1328 * _e1341));
                                let _e1348 = (_e1327 - (_e1330 * _e1341));
                                let _e1357 = ((sqrt(((_e1347 * _e1347) + (_e1348 * _e1348))) * select(1f, -1f, (((_e1327 * _e1328) - (_e1326 * _e1330)) < 0f))) - (_e1283 * 1.08f));
                                let _e1358 = (((_e1292 / (_e1283 * 21.6f)) + 0.5f) - select(_e1258, 1f, (_e1258 > 1f)));
                                let _e1359 = fwidth(_e1358);
                                let _e1361 = ((_e1358 / _e1359) + 0.5f);
                                let _e1363 = select(_e1361, 0f, (_e1361 < 0f));
                                let _e1365 = select(_e1363, 1f, (_e1363 > 1f));
                                let _e1366 = (1f - _e1365);
                                let _e1369 = (0.33f * _e1365);
                                let _e1373 = (0.5f - _e1357);
                                let _e1375 = select(_e1373, 0f, (_e1373 < 0f));
                                let _e1377 = select(_e1375, 1f, (_e1375 > 1f));
                                if (_e1357 != _e1357) {
                                    phi_33_ = true;
                                } else {
                                    phi_33_ = (0f >= _e1357);
                                }
                                let _e1381 = phi_33_;
                                let _e1384 = exp((select(_e1357, 0f, _e1381) * -0.5f));
                                let _e1385 = (_e1357 * -0.2f);
                                let _e1387 = select(_e1385, 0f, (_e1385 < 0f));
                                let _e1389 = select(_e1387, 1f, (_e1387 > 1f));
                                let _e1394 = (1f - ((_e1389 * _e1389) * (3f - (2f * _e1389))));
                                let _e1396 = ((_e1394 * _e1394) * 0.045f);
                                let _e1407 = ((_e1384 * _e1384) * 0.2f);
                                if (_e1377 != _e1377) {
                                    phi_34_ = true;
                                } else {
                                    phi_34_ = (_e1407 >= _e1377);
                                }
                                let _e1411 = phi_34_;
                                let _e1413 = (select(_e1377, _e1407, _e1411) * _e426);
                                let _e1414 = (1f - _e1413);
                                phi_35_ = vec4<f32>(((_e1232.x * _e1414) + ((((_e1366 + _e1369) + _e1396) * _e1377) * _e426)), ((_e1232.y * _e1414) + (((((0.85f * _e1366) + _e1369) + _e1396) * _e1377) * _e426)), ((_e1232.z * _e1414) + (((((0.2f * _e1366) + _e1369) + _e1396) * _e1377) * _e426)), ((_e1232.w * _e1414) + _e1413));
                            } else {
                                phi_35_ = _e1232;
                            }
                            let _e1429 = phi_35_;
                            phi_36_ = _e1429;
                            phi_37_ = (_e1234 + 1u);
                        } else {
                            phi_36_ = vec4<f32>();
                            phi_37_ = u32();
                        }
                        let _e1432 = phi_36_;
                        let _e1434 = phi_37_;
                        continue;
                        continuing {
                            phi_28_ = _e1432;
                            phi_29_ = _e1434;
                            break if !(_e1235);
                        }
                    }
                    if _e315 {
                        break;
                    }
                    let _e2301 = local_24;
                    phi_38_ = _e2301;
                } else {
                    phi_38_ = _e1229;
                }
                let _e1437 = phi_38_;
                phi_39_ = _e1437;
            } else {
                phi_39_ = _e1229;
            }
            let _e1439 = phi_39_;
            let _e1440 = (_e363 + _e377);
            phi_40_ = _e1439;
            phi_41_ = 0u;
            loop {
                let _e1444 = phi_40_;
                let _e1446 = phi_41_;
                local_20 = _e1444;
                local_21 = _e1444;
                local_22 = _e1444;
                local_23 = _e1444;
                let _e1447 = (_e1446 < select(_e1440, 8u, (8u < _e1440)));
                if _e1447 {
                    if (_e1446 < 8u) {
                    } else {
                        phi_55_ = true;
                        break;
                    }
                    let _e1453 = pill.member[_e165].playlist_images[_e1446];
                    if (_e1453 >= 0i) {
                        let _e1455 = (_e1446 < _e363);
                        if _e1455 {
                            phi_42_ = render_shared_RipplePulse(vec2<f32>(_e351, _e353), _e365, 1f);
                            phi_43_ = (f32(_e1446) + _e359);
                        } else {
                            phi_42_ = render_shared_RipplePulse(vec2<f32>(_e351, _e373), _e378, _e371);
                            phi_43_ = f32((_e1446 - _e363));
                        }
                        let _e1461 = phi_42_;
                        let _e1463 = phi_43_;
                        let _e1464 = select(_e371, _e426, _e1455);
                        let _e1466 = (_e1461.start_time - 1f);
                        if (_e1466 != _e1466) {
                            phi_44_ = true;
                        } else {
                            phi_44_ = (0f >= _e1466);
                        }
                        let _e1470 = phi_44_;
                        let _e1479 = (_e1461.origin.x + (((_e1463 - (select(_e1466, 0f, _e1470) * 0.5f)) * 18f) * _e1461.strength));
                        let _e1482 = (_e1461.origin.y + 2f);
                        if (_e1464 > 0f) {
                            let _e1484 = (_e164.x - _e1479);
                            let _e1485 = (_e164.y - _e1482);
                            let _e1486 = abs(_e1484);
                            let _e1487 = abs(_e1485);
                            if (select(_e1487, _e1486, (_e1486 > _e1487)) < 38.88f) {
                                let _e1491 = (_e1479 - _e198);
                                let _e1492 = (_e1482 - _e203);
                                let _e1496 = sqrt(((_e1491 * _e1491) + (_e1492 * _e1492)));
                                let _e1498 = ((_e1496 - 11.3f) * -1f);
                                let _e1500 = select(_e1498, 0f, (_e1498 < 0f));
                                let _e1502 = select(_e1500, 1f, (_e1500 > 1f));
                                let _e1508 = select(_e319, 0f, (_e319 < 0f));
                                let _e1511 = (((_e1502 * _e1502) * (3f - (2f * _e1502))) * select(_e1508, 1f, (_e1508 > 1f)));
                                let _e1513 = (1.05f + (0.63f * _e1511));
                                let _e1514 = (_e1491 * _e1511);
                                let _e1516 = (_e1484 - (_e1514 * 0.5f));
                                let _e1517 = (_e1514 * -0.005f);
                                let _e1518 = sin(_e1517);
                                let _e1519 = cos(_e1517);
                                let _e1522 = ((_e1519 * _e1516) - (_e1518 * _e1485));
                                let _e1525 = ((_e1518 * _e1516) + (_e1519 * _e1485));
                                let _e1526 = (_e1513 * 21.6f);
                                if _e1455 {
                                    phi_46_ = true;
                                } else {
                                    if _e320 {
                                        phi_45_ = select(true, false, (_e1496 <= 10.8f));
                                    } else {
                                        phi_45_ = true;
                                    }
                                    let _e1534 = phi_45_;
                                    phi_46_ = select(true, false, _e1534);
                                }
                                let _e1537 = phi_46_;
                                let _e1538 = select(0.2f, 0f, _e1537);
                                let _e1541 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1522, _e1525), 0f, (_e1513 * 6.4800005f));
                                if (_e1541 <= 7f) {
                                    let _e1544 = vec3<f32>(((_e1522 / _e1526) + 0.5f), ((_e1525 / _e1526) + 0.5f), f32(_e1453));
                                    let _e1550 = textureSample(images, sampler_, vec2<f32>(_e1544.x, _e1544.y), i32(_e1544.z));
                                    let _e1554 = (1f - _e1538);
                                    let _e1558 = (0.24f * _e1538);
                                    let _e1562 = (0.5f - _e1541);
                                    let _e1564 = select(_e1562, 0f, (_e1562 < 0f));
                                    let _e1566 = select(_e1564, 1f, (_e1564 > 1f));
                                    if (_e1541 != _e1541) {
                                        phi_47_ = true;
                                    } else {
                                        phi_47_ = (0f >= _e1541);
                                    }
                                    let _e1570 = phi_47_;
                                    let _e1573 = exp((select(_e1541, 0f, _e1570) * -0.5f));
                                    let _e1574 = (_e1541 * -0.2f);
                                    let _e1576 = select(_e1574, 0f, (_e1574 < 0f));
                                    let _e1578 = select(_e1576, 1f, (_e1576 > 1f));
                                    let _e1583 = (1f - ((_e1578 * _e1578) * (3f - (2f * _e1578))));
                                    let _e1585 = ((_e1583 * _e1583) * 0.045f);
                                    let _e1596 = ((_e1573 * _e1573) * 0.2f);
                                    if (_e1566 != _e1566) {
                                        phi_48_ = true;
                                    } else {
                                        phi_48_ = (_e1596 >= _e1566);
                                    }
                                    let _e1600 = phi_48_;
                                    let _e1602 = (select(_e1566, _e1596, _e1600) * _e1464);
                                    let _e1603 = (1f - _e1602);
                                    phi_49_ = vec4<f32>(((_e1444.x * _e1603) + (((((_e1550.x * _e1554) + _e1558) + _e1585) * _e1566) * _e1464)), ((_e1444.y * _e1603) + (((((_e1550.y * _e1554) + _e1558) + _e1585) * _e1566) * _e1464)), ((_e1444.z * _e1603) + (((((_e1550.z * _e1554) + _e1558) + _e1585) * _e1566) * _e1464)), ((_e1444.w * _e1603) + _e1602));
                                } else {
                                    phi_49_ = _e1444;
                                }
                                let _e1618 = phi_49_;
                                phi_50_ = _e1618;
                            } else {
                                phi_50_ = _e1444;
                            }
                            let _e1620 = phi_50_;
                            phi_51_ = _e1620;
                        } else {
                            phi_51_ = _e1444;
                        }
                        let _e1622 = phi_51_;
                        phi_52_ = _e1622;
                    } else {
                        phi_52_ = _e1444;
                    }
                    let _e1624 = phi_52_;
                    phi_53_ = _e1624;
                    phi_54_ = (_e1446 + 1u);
                } else {
                    phi_53_ = vec4<f32>();
                    phi_54_ = u32();
                }
                let _e1627 = phi_53_;
                let _e1629 = phi_54_;
                continue;
                continuing {
                    phi_40_ = _e1627;
                    phi_41_ = _e1629;
                    phi_55_ = _e315;
                    break if !(_e1447);
                }
            }
            let _e1632 = phi_55_;
            if _e1632 {
                break;
            }
            let _e1640 = pill.member[_e165].text.lines[0u].min[0u];
            let _e1648 = pill.member[_e165].text.lines[0u].min[1u];
            let _e1656 = pill.member[_e165].text.lines[0u].max[0u];
            let _e1664 = pill.member[_e165].text.lines[0u].max[1u];
            let _e1672 = pill.member[_e165].text.lines[0u].origin[0u];
            let _e1680 = pill.member[_e165].text.lines[0u].origin[1u];
            let _e1687 = pill.member[_e165].text.lines[0u].size;
            let _e1694 = pill.member[_e165].text.lines[0u].weight;
            let _e1701 = pill.member[_e165].text.lines[0u].count;
            let _e1708 = pill.member[_e165].text.lines[0u].first;
            if (_e573 < _e1640) {
                phi_97_ = _e1632;
                phi_98_ = f32();
                phi_99_ = true;
            } else {
                if (_e573 > _e1656) {
                    phi_94_ = _e1632;
                    phi_95_ = f32();
                    phi_96_ = true;
                } else {
                    if (_e574 < _e1648) {
                        phi_91_ = _e1632;
                        phi_92_ = f32();
                        phi_93_ = true;
                    } else {
                        let _e1712 = (_e574 > _e1664);
                        if _e1712 {
                            phi_89_ = _e1632;
                            phi_90_ = f32();
                        } else {
                            phi_56_ = _e1701;
                            phi_57_ = 0u;
                            loop {
                                let _e1714 = phi_56_;
                                let _e1716 = phi_57_;
                                local_14 = _e1716;
                                let _e1717 = (_e1716 < _e1714);
                                if _e1717 {
                                    let _e1720 = (_e1716 + ((_e1714 - _e1716) / 2u));
                                    let _e1721 = (_e1708 + _e1720);
                                    if (_e1721 < 128u) {
                                    } else {
                                        phi_61_ = true;
                                        break;
                                    }
                                    let _e1729 = pill.member[_e165].text.glyphs[_e1721].x;
                                    let _e1732 = (_e1729 <= ((_e573 - _e1672) / _e1687));
                                    if _e1732 {
                                        phi_58_ = (_e1720 + 1u);
                                    } else {
                                        phi_58_ = _e1716;
                                    }
                                    let _e1735 = phi_58_;
                                    phi_59_ = select(_e1720, _e1714, _e1732);
                                    phi_60_ = _e1735;
                                } else {
                                    phi_59_ = u32();
                                    phi_60_ = u32();
                                }
                                let _e1738 = phi_59_;
                                let _e1740 = phi_60_;
                                continue;
                                continuing {
                                    phi_56_ = _e1738;
                                    phi_57_ = _e1740;
                                    phi_61_ = _e1632;
                                    break if !(_e1717);
                                }
                            }
                            let _e1743 = phi_61_;
                            if _e1743 {
                                break;
                            }
                            let _e1745 = local_14;
                            let _e1746 = (_e1745 + 1u);
                            phi_62_ = _e1743;
                            phi_63_ = select(_e1746, _e1701, (_e1701 < _e1746));
                            phi_64_ = -1000000f;
                            loop {
                                let _e1750 = phi_62_;
                                let _e1752 = phi_63_;
                                let _e1754 = phi_64_;
                                local_25 = _e1754;
                                if (_e1752 > 0u) {
                                    let _e1756 = (_e1752 - 1u);
                                    let _e1757 = (_e1708 + _e1756);
                                    if (_e1757 < 128u) {
                                    } else {
                                        phi_88_ = true;
                                        break;
                                    }
                                    let _e1765 = pill.member[_e165].text.glyphs[_e1757].x;
                                    let _e1772 = pill.member[_e165].text.glyphs[_e1757].glyph;
                                    if (_e1772 < _e167) {
                                    } else {
                                        phi_88_ = true;
                                        break;
                                    }
                                    let _e1778 = glyphs.member[_e1772].min[0u];
                                    let _e1783 = glyphs.member[_e1772].min[1u];
                                    let _e1788 = glyphs.member[_e1772].max[0u];
                                    let _e1793 = glyphs.member[_e1772].max[1u];
                                    let _e1797 = glyphs.member[_e1772].start;
                                    let _e1801 = glyphs.member[_e1772].count;
                                    let _e1804 = (((_e573 - _e1672) / _e1687) - _e1765);
                                    let _e1807 = (-((_e574 - _e1680)) / _e1687);
                                    let _e1808 = (3.5f / _e1687);
                                    let _e1809 = (_e1788 + _e1808);
                                    let _e1810 = (_e1804 > _e1809);
                                    if _e1810 {
                                        phi_82_ = _e1750;
                                        phi_83_ = f32();
                                    } else {
                                        if (_e1804 >= (_e1778 - _e1808)) {
                                            if (_e1807 >= (_e1783 - _e1808)) {
                                                if (_e1804 <= _e1809) {
                                                    if (_e1807 <= (_e1793 + _e1808)) {
                                                        phi_65_ = 0u;
                                                        phi_66_ = 0i;
                                                        phi_67_ = 340282350000000000000000000000000000000f;
                                                        loop {
                                                            let _e1819 = phi_65_;
                                                            let _e1821 = phi_66_;
                                                            let _e1823 = phi_67_;
                                                            local_15 = _e1823;
                                                            local_16 = _e1821;
                                                            let _e1824 = (_e1819 < _e1801);
                                                            if _e1824 {
                                                                let _e1825 = (_e1797 + _e1819);
                                                                if (_e1825 < _e169) {
                                                                } else {
                                                                    phi_72_ = true;
                                                                    break;
                                                                }
                                                                let _e1829 = edges.member[_e1825];
                                                                let _e1831 = cantus_render_text_edge_distance(_e1829, _e1694, vec2<f32>(_e1804, _e1807));
                                                                if (_e1823 != _e1823) {
                                                                    phi_68_ = true;
                                                                } else {
                                                                    phi_68_ = (_e1831.member <= _e1823);
                                                                }
                                                                let _e1837 = phi_68_;
                                                                phi_69_ = (_e1819 + 1u);
                                                                phi_70_ = (_e1821 + _e1831.member_1);
                                                                phi_71_ = select(_e1823, _e1831.member, _e1837);
                                                            } else {
                                                                phi_69_ = u32();
                                                                phi_70_ = i32();
                                                                phi_71_ = f32();
                                                            }
                                                            let _e1842 = phi_69_;
                                                            let _e1844 = phi_70_;
                                                            let _e1846 = phi_71_;
                                                            continue;
                                                            continuing {
                                                                phi_65_ = _e1842;
                                                                phi_66_ = _e1844;
                                                                phi_67_ = _e1846;
                                                                phi_72_ = _e1750;
                                                                break if !(_e1824);
                                                            }
                                                        }
                                                        let _e1849 = phi_72_;
                                                        phi_88_ = _e1849;
                                                        if _e1849 {
                                                            break;
                                                        }
                                                        let _e1851 = local_15;
                                                        let _e1855 = local_16;
                                                        let _e1858 = ((sqrt(_e1851) * _e1687) * select(1f, -1f, (_e1855 == 0i)));
                                                        if (_e1754 != _e1754) {
                                                            phi_73_ = true;
                                                        } else {
                                                            phi_73_ = (_e1858 >= _e1754);
                                                        }
                                                        let _e1862 = phi_73_;
                                                        phi_74_ = _e1849;
                                                        phi_75_ = select(_e1754, _e1858, _e1862);
                                                    } else {
                                                        phi_74_ = _e1750;
                                                        phi_75_ = _e1754;
                                                    }
                                                    let _e1865 = phi_74_;
                                                    let _e1867 = phi_75_;
                                                    phi_76_ = _e1865;
                                                    phi_77_ = _e1867;
                                                } else {
                                                    phi_76_ = _e1750;
                                                    phi_77_ = _e1754;
                                                }
                                                let _e1869 = phi_76_;
                                                let _e1871 = phi_77_;
                                                phi_78_ = _e1869;
                                                phi_79_ = _e1871;
                                            } else {
                                                phi_78_ = _e1750;
                                                phi_79_ = _e1754;
                                            }
                                            let _e1873 = phi_78_;
                                            let _e1875 = phi_79_;
                                            phi_80_ = _e1873;
                                            phi_81_ = _e1875;
                                        } else {
                                            phi_80_ = _e1750;
                                            phi_81_ = _e1754;
                                        }
                                        let _e1877 = phi_80_;
                                        let _e1879 = phi_81_;
                                        phi_82_ = _e1877;
                                        phi_83_ = _e1879;
                                    }
                                    let _e1881 = phi_82_;
                                    let _e1883 = phi_83_;
                                    phi_84_ = _e1881;
                                    phi_85_ = _e1756;
                                    phi_86_ = _e1883;
                                    phi_87_ = select(true, false, _e1810);
                                } else {
                                    phi_84_ = _e1750;
                                    phi_85_ = u32();
                                    phi_86_ = f32();
                                    phi_87_ = false;
                                }
                                let _e1886 = phi_84_;
                                let _e1888 = phi_85_;
                                let _e1890 = phi_86_;
                                let _e1892 = phi_87_;
                                continue;
                                continuing {
                                    phi_62_ = _e1886;
                                    phi_63_ = _e1888;
                                    phi_64_ = _e1890;
                                    phi_88_ = _e1886;
                                    break if !(_e1892);
                                }
                            }
                            let _e1895 = phi_88_;
                            if _e1895 {
                                break;
                            }
                            phi_89_ = _e1895;
                            let _e2358 = local_25;
                            phi_90_ = _e2358;
                        }
                        let _e1897 = phi_89_;
                        let _e1899 = phi_90_;
                        phi_91_ = _e1897;
                        phi_92_ = _e1899;
                        phi_93_ = _e1712;
                    }
                    let _e1901 = phi_91_;
                    let _e1903 = phi_92_;
                    let _e1905 = phi_93_;
                    phi_94_ = _e1901;
                    phi_95_ = _e1903;
                    phi_96_ = _e1905;
                }
                let _e1907 = phi_94_;
                let _e1909 = phi_95_;
                let _e1911 = phi_96_;
                phi_97_ = _e1907;
                phi_98_ = _e1909;
                phi_99_ = _e1911;
            }
            let _e1913 = phi_97_;
            let _e1915 = phi_98_;
            let _e1917 = phi_99_;
            let _e1918 = select(_e1915, -1000000f, _e1917);
            let _e1926 = pill.member[_e165].text.lines[1u].min[0u];
            let _e1934 = pill.member[_e165].text.lines[1u].min[1u];
            let _e1942 = pill.member[_e165].text.lines[1u].max[0u];
            let _e1950 = pill.member[_e165].text.lines[1u].max[1u];
            let _e1958 = pill.member[_e165].text.lines[1u].origin[0u];
            let _e1966 = pill.member[_e165].text.lines[1u].origin[1u];
            let _e1973 = pill.member[_e165].text.lines[1u].size;
            let _e1980 = pill.member[_e165].text.lines[1u].weight;
            let _e1987 = pill.member[_e165].text.lines[1u].count;
            let _e1994 = pill.member[_e165].text.lines[1u].first;
            if (_e573 < _e1926) {
                phi_138_ = f32();
                phi_139_ = true;
            } else {
                if (_e573 > _e1942) {
                    phi_136_ = f32();
                    phi_137_ = true;
                } else {
                    if (_e574 < _e1934) {
                        phi_134_ = f32();
                        phi_135_ = true;
                    } else {
                        let _e1998 = (_e574 > _e1950);
                        if _e1998 {
                            phi_133_ = f32();
                        } else {
                            phi_100_ = _e1987;
                            phi_101_ = 0u;
                            loop {
                                let _e2000 = phi_100_;
                                let _e2002 = phi_101_;
                                local_17 = _e2002;
                                let _e2003 = (_e2002 < _e2000);
                                if _e2003 {
                                    let _e2006 = (_e2002 + ((_e2000 - _e2002) / 2u));
                                    let _e2007 = (_e1994 + _e2006);
                                    if (_e2007 < 128u) {
                                    } else {
                                        phi_105_ = true;
                                        break;
                                    }
                                    let _e2015 = pill.member[_e165].text.glyphs[_e2007].x;
                                    let _e2018 = (_e2015 <= ((_e573 - _e1958) / _e1973));
                                    if _e2018 {
                                        phi_102_ = (_e2006 + 1u);
                                    } else {
                                        phi_102_ = _e2002;
                                    }
                                    let _e2021 = phi_102_;
                                    phi_103_ = select(_e2006, _e2000, _e2018);
                                    phi_104_ = _e2021;
                                } else {
                                    phi_103_ = u32();
                                    phi_104_ = u32();
                                }
                                let _e2024 = phi_103_;
                                let _e2026 = phi_104_;
                                continue;
                                continuing {
                                    phi_100_ = _e2024;
                                    phi_101_ = _e2026;
                                    phi_105_ = _e1913;
                                    break if !(_e2003);
                                }
                            }
                            let _e2029 = phi_105_;
                            if _e2029 {
                                break;
                            }
                            let _e2031 = local_17;
                            let _e2032 = (_e2031 + 1u);
                            phi_106_ = _e2029;
                            phi_107_ = select(_e2032, _e1987, (_e1987 < _e2032));
                            phi_108_ = -1000000f;
                            loop {
                                let _e2036 = phi_106_;
                                let _e2038 = phi_107_;
                                let _e2040 = phi_108_;
                                local_26 = _e2040;
                                if (_e2038 > 0u) {
                                    let _e2042 = (_e2038 - 1u);
                                    let _e2043 = (_e1994 + _e2042);
                                    if (_e2043 < 128u) {
                                    } else {
                                        phi_132_ = true;
                                        break;
                                    }
                                    let _e2051 = pill.member[_e165].text.glyphs[_e2043].x;
                                    let _e2058 = pill.member[_e165].text.glyphs[_e2043].glyph;
                                    if (_e2058 < _e167) {
                                    } else {
                                        phi_132_ = true;
                                        break;
                                    }
                                    let _e2064 = glyphs.member[_e2058].min[0u];
                                    let _e2069 = glyphs.member[_e2058].min[1u];
                                    let _e2074 = glyphs.member[_e2058].max[0u];
                                    let _e2079 = glyphs.member[_e2058].max[1u];
                                    let _e2083 = glyphs.member[_e2058].start;
                                    let _e2087 = glyphs.member[_e2058].count;
                                    let _e2090 = (((_e573 - _e1958) / _e1973) - _e2051);
                                    let _e2093 = (-((_e574 - _e1966)) / _e1973);
                                    let _e2094 = (3.5f / _e1973);
                                    let _e2095 = (_e2074 + _e2094);
                                    let _e2096 = (_e2090 > _e2095);
                                    if _e2096 {
                                        phi_126_ = _e2036;
                                        phi_127_ = f32();
                                    } else {
                                        if (_e2090 >= (_e2064 - _e2094)) {
                                            if (_e2093 >= (_e2069 - _e2094)) {
                                                if (_e2090 <= _e2095) {
                                                    if (_e2093 <= (_e2079 + _e2094)) {
                                                        phi_109_ = 0u;
                                                        phi_110_ = 0i;
                                                        phi_111_ = 340282350000000000000000000000000000000f;
                                                        loop {
                                                            let _e2105 = phi_109_;
                                                            let _e2107 = phi_110_;
                                                            let _e2109 = phi_111_;
                                                            local_18 = _e2109;
                                                            local_19 = _e2107;
                                                            let _e2110 = (_e2105 < _e2087);
                                                            if _e2110 {
                                                                let _e2111 = (_e2083 + _e2105);
                                                                if (_e2111 < _e169) {
                                                                } else {
                                                                    phi_116_ = true;
                                                                    break;
                                                                }
                                                                let _e2115 = edges.member[_e2111];
                                                                let _e2117 = cantus_render_text_edge_distance(_e2115, _e1980, vec2<f32>(_e2090, _e2093));
                                                                if (_e2109 != _e2109) {
                                                                    phi_112_ = true;
                                                                } else {
                                                                    phi_112_ = (_e2117.member <= _e2109);
                                                                }
                                                                let _e2123 = phi_112_;
                                                                phi_113_ = (_e2105 + 1u);
                                                                phi_114_ = (_e2107 + _e2117.member_1);
                                                                phi_115_ = select(_e2109, _e2117.member, _e2123);
                                                            } else {
                                                                phi_113_ = u32();
                                                                phi_114_ = i32();
                                                                phi_115_ = f32();
                                                            }
                                                            let _e2128 = phi_113_;
                                                            let _e2130 = phi_114_;
                                                            let _e2132 = phi_115_;
                                                            continue;
                                                            continuing {
                                                                phi_109_ = _e2128;
                                                                phi_110_ = _e2130;
                                                                phi_111_ = _e2132;
                                                                phi_116_ = _e2036;
                                                                break if !(_e2110);
                                                            }
                                                        }
                                                        let _e2135 = phi_116_;
                                                        phi_132_ = _e2135;
                                                        if _e2135 {
                                                            break;
                                                        }
                                                        let _e2137 = local_18;
                                                        let _e2141 = local_19;
                                                        let _e2144 = ((sqrt(_e2137) * _e1973) * select(1f, -1f, (_e2141 == 0i)));
                                                        if (_e2040 != _e2040) {
                                                            phi_117_ = true;
                                                        } else {
                                                            phi_117_ = (_e2144 >= _e2040);
                                                        }
                                                        let _e2148 = phi_117_;
                                                        phi_118_ = _e2135;
                                                        phi_119_ = select(_e2040, _e2144, _e2148);
                                                    } else {
                                                        phi_118_ = _e2036;
                                                        phi_119_ = _e2040;
                                                    }
                                                    let _e2151 = phi_118_;
                                                    let _e2153 = phi_119_;
                                                    phi_120_ = _e2151;
                                                    phi_121_ = _e2153;
                                                } else {
                                                    phi_120_ = _e2036;
                                                    phi_121_ = _e2040;
                                                }
                                                let _e2155 = phi_120_;
                                                let _e2157 = phi_121_;
                                                phi_122_ = _e2155;
                                                phi_123_ = _e2157;
                                            } else {
                                                phi_122_ = _e2036;
                                                phi_123_ = _e2040;
                                            }
                                            let _e2159 = phi_122_;
                                            let _e2161 = phi_123_;
                                            phi_124_ = _e2159;
                                            phi_125_ = _e2161;
                                        } else {
                                            phi_124_ = _e2036;
                                            phi_125_ = _e2040;
                                        }
                                        let _e2163 = phi_124_;
                                        let _e2165 = phi_125_;
                                        phi_126_ = _e2163;
                                        phi_127_ = _e2165;
                                    }
                                    let _e2167 = phi_126_;
                                    let _e2169 = phi_127_;
                                    phi_128_ = _e2167;
                                    phi_129_ = _e2042;
                                    phi_130_ = _e2169;
                                    phi_131_ = select(true, false, _e2096);
                                } else {
                                    phi_128_ = _e2036;
                                    phi_129_ = u32();
                                    phi_130_ = f32();
                                    phi_131_ = false;
                                }
                                let _e2172 = phi_128_;
                                let _e2174 = phi_129_;
                                let _e2176 = phi_130_;
                                let _e2178 = phi_131_;
                                continue;
                                continuing {
                                    phi_106_ = _e2172;
                                    phi_107_ = _e2174;
                                    phi_108_ = _e2176;
                                    phi_132_ = _e2172;
                                    break if !(_e2178);
                                }
                            }
                            let _e2181 = phi_132_;
                            if _e2181 {
                                break;
                            }
                            let _e2406 = local_26;
                            phi_133_ = _e2406;
                        }
                        let _e2183 = phi_133_;
                        phi_134_ = _e2183;
                        phi_135_ = _e1998;
                    }
                    let _e2185 = phi_134_;
                    let _e2187 = phi_135_;
                    phi_136_ = _e2185;
                    phi_137_ = _e2187;
                }
                let _e2189 = phi_136_;
                let _e2191 = phi_137_;
                phi_138_ = _e2189;
                phi_139_ = _e2191;
            }
            let _e2193 = phi_138_;
            let _e2195 = phi_139_;
            let _e2196 = select(_e2193, -1000000f, _e2195);
            if (_e1918 != _e1918) {
                phi_140_ = true;
            } else {
                phi_140_ = (_e2196 >= _e1918);
            }
            let _e2200 = phi_140_;
            let _e2205 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e573 - _e1077), (_e574 - _e187)), 0f, _e187);
            let _e2206 = (_e2205 * 0.125f);
            let _e2208 = select(_e2206, 0f, (_e2206 < 0f));
            let _e2210 = select(_e2208, 1f, (_e2208 > 1f));
            let _e2216 = ((select(_e1918, _e2196, _e2200) * 1.25f) + 0.5f);
            let _e2218 = select(_e2216, 0f, (_e2216 < 0f));
            let _e2220 = select(_e2218, 1f, (_e2218 > 1f));
            let _e2226 = ((((_e2220 * _e2220) * (3f - (2f * _e2220))) * ((_e2210 * _e2210) * (3f - (2f * _e2210)))) * _e533);
            let _e2227 = (1f - _e2226);
            let _e2229 = local_20;
            let _e2233 = local_21;
            let _e2237 = local_22;
            let _e2241 = local_23;
            let _e2244 = (0.94f * _e2226);
            let _e2252 = (((_e2241.w * _e2227) + _e2226) * _e550);
            if (_e2252 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e2229.x * _e2227) + _e2244) * _e550), (((_e2233.y * _e2227) + _e2244) * _e550), (((_e2237.z * _e2227) + _e2244) * _e550), _e2252);
            break;
        }
    }
    return;
}

fn render_status_vertex_impl() {
    var phi_0_: bool;
    var phi_1_: u32;
    var phi_2_: f32;
    var phi_3_: u32;
    var phi_4_: f32;
    var phi_5_: bool;
    var local_27: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e28 = vertex_5;
            let _e29 = _isthmus_instance_index_7;
            let _e33 = pill_1.member[_e29].battery_level;
            if (_e33 >= -1f) {
                phi_0_ = (_e33 <= 1f);
            } else {
                phi_0_ = false;
            }
            let _e37 = phi_0_;
            let _e39 = (select(0f, 40f, _e37) + 296f);
            let _e44 = frame.member[0u].screen_size[0u];
            let _e50 = frame.member[0u].mouse_pressure;
            phi_1_ = 0u;
            phi_2_ = (_e50 * 8f);
            loop {
                let _e53 = phi_1_;
                let _e55 = phi_2_;
                local_27 = _e55;
                let _e56 = (_e53 < 4u);
                if _e56 {
                    if _e56 {
                    } else {
                        phi_5_ = true;
                        break;
                    }
                    let _e62 = frame.member[0u].ripples[_e53].start_time;
                    let _e68 = frame.member[0u].ripples[_e53].strength;
                    let _e72 = frame.member[0u].time;
                    let _e74 = ((_e72 - _e62) * 1.2f);
                    let _e76 = select(_e74, 0f, (_e74 < 0f));
                    let _e79 = (1f - select(_e76, 1f, (_e76 > 1f)));
                    phi_3_ = (_e53 + 1u);
                    phi_4_ = (_e55 + (((_e68 * _e79) * _e79) * 11f));
                } else {
                    phi_3_ = u32();
                    phi_4_ = f32();
                }
                let _e86 = phi_3_;
                let _e88 = phi_4_;
                continue;
                continuing {
                    phi_1_ = _e86;
                    phi_2_ = _e88;
                    phi_5_ = false;
                    break if !(_e56);
                }
            }
            let _e91 = phi_5_;
            if _e91 {
                break;
            }
            let _e93 = local_27;
            let _e94 = (_e93 * 0.5f);
            let _e95 = (18f + _e94);
            let _e106 = frame.member[0u].panel_height;
            let _e113 = ((((_e44 - _e39) - 8f) - _e95) + (f32((_e28 & 1u)) * (_e39 + (_e95 * 2f))));
            let _e114 = ((-12f - _e94) + (f32((_e28 >> bitcast<u32>(1i))) * ((_e106 + _e95) * 2f)));
            let _e119 = frame.member[0u].screen_size[1u];
            let _e122 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e113, _e114), vec2<f32>(_e44, _e119));
            out_position = _e122;
            out_pixel[0u] = _e113;
            out_pixel[1u] = _e114;
            out_isthmus_instance_index = _e29;
            break;
        }
    }
    return;
}

fn cantus_render_shader_sd_rounded_box(param_15: vec2<f32>, param_16: vec2<f32>, param_17: f32) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;

    let _e13 = ((abs(param_15.x) - param_16.x) + param_17);
    let _e14 = ((abs(param_15.y) - param_16.y) + param_17);
    let _e16 = select(0f, _e13, (_e13 > 0f));
    let _e18 = select(0f, _e14, (_e14 > 0f));
    if (_e13 != _e13) {
        phi_0_ = true;
    } else {
        phi_0_ = (_e14 >= _e13);
    }
    let _e26 = phi_0_;
    let _e27 = select(_e13, _e14, _e26);
    if (_e27 != _e27) {
        phi_1_ = true;
    } else {
        phi_1_ = (0f <= _e27);
    }
    let _e31 = phi_1_;
    return ((sqrt(((_e16 * _e16) + (_e18 * _e18))) + select(_e27, 0f, _e31)) - param_17);
}

fn render_status_fragment_impl() {
    var phi_0_: bool;
    var phi_1_: vec2<f32>;
    var phi_2_: f32;
    var phi_3_: u32;
    var phi_4_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_5_: bool;
    var phi_6_: vec2<f32>;
    var phi_7_: f32;
    var phi_8_: u32;
    var phi_9_: bool;
    var phi_10_: f32;
    var local_28: vec2<f32>;
    var local_29: vec2<f32>;
    var phi_11_: bool;
    var phi_12_: bool;
    var phi_13_: bool;
    var phi_14_: bool;
    var phi_15_: bool;
    var phi_16_: bool;
    var phi_17_: bool;
    var phi_18_: bool;
    var phi_19_: u32;
    var phi_20_: u32;
    var phi_21_: u32;
    var phi_22_: u32;
    var phi_23_: bool;
    var phi_24_: f32;
    var phi_25_: bool;
    var phi_26_: bool;
    var phi_27_: bool;
    var phi_28_: vec2<f32>;
    var phi_29_: bool;
    var phi_30_: i32;
    var phi_31_: f32;
    var phi_32_: f32;
    var phi_33_: vec2<f32>;
    var phi_34_: i32;
    var phi_35_: f32;
    var phi_36_: f32;
    var phi_37_: vec2<f32>;
    var local_30: f32;
    var phi_38_: vec2<f32>;
    var phi_39_: i32;
    var phi_40_: f32;
    var phi_41_: f32;
    var phi_42_: vec2<f32>;
    var phi_43_: i32;
    var phi_44_: f32;
    var phi_45_: f32;
    var phi_46_: vec2<f32>;
    var local_31: f32;
    var phi_47_: vec2<f32>;
    var phi_48_: vec2<f32>;
    var phi_49_: bool;
    var phi_50_: bool;
    var phi_51_: bool;
    var phi_52_: bool;
    var phi_53_: bool;
    var phi_54_: bool;
    var phi_55_: bool;
    var phi_56_: bool;
    var phi_57_: bool;
    var phi_58_: bool;
    var phi_59_: bool;
    var phi_60_: bool;
    var phi_61_: bool;
    var phi_62_: bool;
    var phi_63_: bool;
    var phi_64_: bool;
    var phi_65_: bool;
    var phi_66_: bool;
    var phi_67_: vec3<f32>;
    var phi_68_: bool;
    var phi_69_: bool;
    var phi_70_: bool;
    var phi_71_: bool;
    var phi_72_: bool;
    var phi_73_: f32;
    var phi_74_: bool;
    var phi_75_: vec3<f32>;
    var phi_76_: bool;
    var phi_77_: u32;
    var phi_78_: u32;
    var phi_79_: u32;
    var phi_80_: u32;
    var phi_81_: u32;
    var phi_82_: bool;
    var local_32: u32;
    var phi_83_: bool;
    var phi_84_: u32;
    var phi_85_: f32;
    var phi_86_: u32;
    var phi_87_: i32;
    var phi_88_: f32;
    var phi_89_: bool;
    var phi_90_: u32;
    var phi_91_: i32;
    var phi_92_: f32;
    var phi_93_: bool;
    var local_33: f32;
    var local_34: i32;
    var phi_94_: bool;
    var phi_95_: bool;
    var phi_96_: f32;
    var phi_97_: bool;
    var phi_98_: f32;
    var phi_99_: bool;
    var phi_100_: f32;
    var phi_101_: bool;
    var phi_102_: f32;
    var phi_103_: bool;
    var phi_104_: f32;
    var phi_105_: bool;
    var phi_106_: u32;
    var phi_107_: f32;
    var phi_108_: bool;
    var phi_109_: bool;
    var phi_110_: f32;
    var phi_111_: f32;
    var phi_112_: bool;
    var phi_113_: f32;
    var phi_114_: bool;
    var phi_115_: f32;
    var phi_116_: bool;
    var phi_117_: f32;
    var local_35: f32;
    var local_36: f32;
    var local_37: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e230 = pixel_2;
            let _e231 = _isthmus_instance_index_8;
            let _e241 = pill_1.member[_e231].battery_level;
            let _e242 = (_e241 >= -1f);
            if _e242 {
                phi_0_ = (_e241 <= 1f);
            } else {
                phi_0_ = false;
            }
            let _e245 = phi_0_;
            let _e247 = (select(0f, 40f, _e245) + 296f);
            let _e252 = frame.member[0u].screen_size[0u];
            let _e254 = ((_e252 - _e247) - 8f);
            let _e258 = frame.member[0u].panel_height;
            let _e259 = (_e230.x - _e254);
            let _e260 = (_e230.y - 6f);
            let _e261 = (_e247 * 0.5f);
            let _e262 = (_e258 * 0.5f);
            let _e266 = ((_e247 - _e258) * 0.5f);
            let _e268 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e259 - _e261), (_e260 - _e262)), _e266, _e262);
            let _e273 = frame.member[0u].mouse_pos[0u];
            let _e278 = frame.member[0u].mouse_pos[1u];
            let _e284 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e273 - _e254) - _e261), ((_e278 - 6f) - _e262)), _e266, _e262);
            phi_1_ = vec2<f32>(0f, 0f);
            phi_2_ = 0f;
            phi_3_ = 0u;
            loop {
                let _e286 = phi_1_;
                let _e288 = phi_2_;
                let _e290 = phi_3_;
                local_28 = _e286;
                local_29 = _e286;
                local_35 = _e288;
                local_36 = _e288;
                let _e291 = (_e290 < 4u);
                if _e291 {
                    if _e291 {
                    } else {
                        phi_9_ = true;
                        break;
                    }
                    let _e298 = frame.member[0u].ripples[_e290].origin[0u];
                    let _e305 = frame.member[0u].ripples[_e290].origin[1u];
                    let _e311 = frame.member[0u].ripples[_e290].start_time;
                    let _e317 = frame.member[0u].ripples[_e290].strength;
                    let _e321 = frame.member[0u].time;
                    let _e323 = ((_e321 - _e311) * 1.2f);
                    let _e325 = select(_e323, 0f, (_e323 < 0f));
                    let _e327 = select(_e325, 1f, (_e325 > 1f));
                    let _e329 = (_e230 - vec2<f32>(_e298, _e305));
                    let _e335 = sqrt(((_e329.x * _e329.x) + (_e329.y * _e329.y)));
                    if (_e335 > 0.001f) {
                        phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e329.x / _e335), (_e329.y / _e335)), _e335);
                    } else {
                        phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e335);
                    }
                    let _e343 = phi_4_;
                    let _e353 = ((abs((_e343.unnamed_1 - (_e327 * 600f))) - 80f) * -0.0125f);
                    let _e355 = select(_e353, 0f, (_e353 < 0f));
                    let _e357 = select(_e355, 1f, (_e355 > 1f));
                    let _e363 = (1f - _e327);
                    let _e364 = ((((_e357 * _e357) * (3f - (2f * _e357))) * _e317) * _e363);
                    let _e377 = (_e288 + (_e364 * 0.5f));
                    if (_e377 != _e377) {
                        phi_5_ = true;
                    } else {
                        phi_5_ = (1f <= _e377);
                    }
                    let _e381 = phi_5_;
                    phi_6_ = vec2<f32>((_e286.x + (((_e343.unnamed.x * _e364) * _e363) * 0.5f)), (_e286.y + (((_e343.unnamed.y * _e364) * _e363) * 0.5f)));
                    phi_7_ = select(_e377, 1f, _e381);
                    phi_8_ = (_e290 + 1u);
                } else {
                    phi_6_ = vec2<f32>();
                    phi_7_ = f32();
                    phi_8_ = u32();
                }
                let _e385 = phi_6_;
                let _e387 = phi_7_;
                let _e389 = phi_8_;
                continue;
                continuing {
                    phi_1_ = _e385;
                    phi_2_ = _e387;
                    phi_3_ = _e389;
                    phi_9_ = false;
                    break if !(_e291);
                }
            }
            let _e392 = phi_9_;
            if _e392 {
                break;
            }
            let _e396 = frame.member[0u].mouse_pressure;
            if (_e396 > 0f) {
                let _e398 = (_e230.x - _e273);
                let _e399 = (_e230.y - _e278);
                let _e405 = ((sqrt(((_e398 * _e398) + (_e399 * _e399))) - 150f) * -0.006666667f);
                let _e407 = select(_e405, 0f, (_e405 < 0f));
                let _e409 = select(_e407, 1f, (_e407 > 1f));
                phi_10_ = ((((_e409 * _e409) * (3f - (2f * _e409))) * _e396) * 8f);
            } else {
                phi_10_ = 0f;
            }
            let _e417 = phi_10_;
            let _e419 = local_28;
            let _e422 = local_29;
            let _e425 = ((_e284 - 0.5f) * -1f);
            let _e427 = select(_e425, 0f, (_e425 < 0f));
            let _e429 = select(_e427, 1f, (_e427 > 1f));
            let _e442 = (_e268 - (((_e417 * ((_e429 * _e429) * (3f - (2f * _e429)))) + (sqrt(((_e419.x * _e419.x) + (_e422.y * _e422.y))) * 22f)) * 0.5f));
            let _e443 = fwidth(_e442);
            if (_e443 != _e443) {
                phi_11_ = true;
            } else {
                phi_11_ = (0.55f >= _e443);
            }
            let _e447 = phi_11_;
            let _e448 = select(_e443, 0.55f, _e447);
            let _e452 = ((_e442 - _e448) / (-(_e448) - _e448));
            let _e454 = select(_e452, 0f, (_e452 < 0f));
            let _e456 = select(_e454, 1f, (_e454 > 1f));
            let _e460 = ((_e456 * _e456) * (3f - (2f * _e456)));
            let _e461 = (_e442 != _e442);
            if _e461 {
                phi_12_ = true;
            } else {
                phi_12_ = (0f >= _e442);
            }
            let _e464 = phi_12_;
            let _e468 = (exp((select(_e442, 0f, _e464) * -0.3f)) * 0.16f);
            if (_e460 != _e460) {
                phi_13_ = true;
            } else {
                phi_13_ = (_e468 >= _e460);
            }
            let _e472 = phi_13_;
            let _e473 = select(_e460, _e468, _e472);
            if (_e473 <= 0.0009765625f) {
                discard;
            }
            let _e475 = (_e259 / _e247);
            let _e476 = (_e260 / _e258);
            if _e461 {
                phi_14_ = true;
            } else {
                phi_14_ = (0f <= _e442);
            }
            let _e481 = phi_14_;
            let _e484 = (1f + (select(_e442, 0f, _e481) * 0.008333334f));
            let _e486 = select(_e484, 0f, (_e484 < 0f));
            let _e488 = select(_e486, 0.6f, (_e486 > 0.6f));
            let _e498 = ((_e476 - (((_e476 - 0.5f) * _e488) * 0.08f)) - (_e422.y * 0.04f));
            let _e502 = pill_1.member[_e231].sun_height;
            let _e506 = pill_1.member[_e231].conditions;
            let _e510 = frame.member[0u].time;
            let _e518 = ((_e498 - 1f) * -1f);
            let _e520 = select(_e518, 0f, (_e518 < 0f));
            let _e522 = select(_e520, 1f, (_e520 > 1f));
            let _e526 = ((_e522 * _e522) * (3f - (2f * _e522)));
            let _e528 = ((_e502 - -0.04f) * 4.1666665f);
            let _e530 = select(_e528, 0f, (_e528 < 0f));
            let _e532 = select(_e530, 1f, (_e530 > 1f));
            let _e536 = ((_e532 * _e532) * (3f - (2f * _e532)));
            let _e538 = ((_e502 - -0.2f) * 4.5454545f);
            let _e540 = select(_e538, 0f, (_e538 < 0f));
            let _e542 = select(_e540, 1f, (_e540 > 1f));
            let _e547 = (1f - _e536);
            let _e548 = (((_e542 * _e542) * (3f - (2f * _e542))) * _e547);
            let _e549 = (1f - _e526);
            let _e561 = (0.65f * _e549);
            let _e585 = (1f - _e548);
            let _e599 = (((_e506.cloud * 0.34f) + (_e506.rain * 0.16f)) + (_e506.hail * 0.08f));
            let _e600 = (1f - _e599);
            let _e611 = (1f - (_e506.snow * 0.16f));
            let _e615 = (_e506.snow * 0.1312f);
            let _e620 = (1f - (_e506.fog * 0.62f));
            let _e633 = ((sin((_e510 * 2.7f)) - 0.92f) * 12.500003f);
            let _e635 = select(_e633, 0f, (_e633 < 0f));
            let _e637 = select(_e635, 1f, (_e635 > 1f));
            let _e642 = (((_e637 * _e637) * (3f - (2f * _e637))) * _e506.lightning);
            let _e644 = (1f - (_e642 * 0.45f));
            let _e655 = ((_e498 - 0.12f) * -8.333334f);
            let _e657 = select(_e655, 0f, (_e655 < 0f));
            let _e659 = select(_e657, 1f, (_e657 > 1f));
            let _e666 = ((_e442 - 5f) * -0.125f);
            let _e668 = select(_e666, 0f, (_e666 < 0f));
            let _e670 = select(_e668, 1f, (_e668 > 1f));
            let _e676 = ((((_e659 * _e659) * (3f - (2f * _e659))) * 0.12f) + (((_e670 * _e670) * (3f - (2f * _e670))) * 0.08f));
            let _e680 = (((_e475 - (((_e475 - 0.5f) * _e488) * 0.08f)) - (_e419.x * 0.04f)) * _e247);
            if (_e680 < 96f) {
                phi_22_ = 0u;
            } else {
                if (_e680 < 184f) {
                    phi_21_ = 1u;
                } else {
                    if _e242 {
                        phi_15_ = (_e241 <= 1f);
                    } else {
                        phi_15_ = false;
                    }
                    let _e686 = phi_15_;
                    if _e686 {
                        phi_16_ = select(true, false, (_e680 < 224f));
                    } else {
                        phi_16_ = true;
                    }
                    let _e690 = phi_16_;
                    if _e690 {
                        if _e242 {
                            phi_17_ = (_e241 <= 1f);
                        } else {
                            phi_17_ = false;
                        }
                        let _e693 = phi_17_;
                        if (_e680 < (select(0f, 40f, _e693) + 224f)) {
                            phi_19_ = 3u;
                        } else {
                            if _e242 {
                                phi_18_ = (_e241 <= 1f);
                            } else {
                                phi_18_ = false;
                            }
                            let _e699 = phi_18_;
                            phi_19_ = select(5u, 4u, (_e680 < (select(0f, 40f, _e699) + 256f)));
                        }
                        let _e705 = phi_19_;
                        phi_20_ = _e705;
                    } else {
                        phi_20_ = 2u;
                    }
                    let _e707 = phi_20_;
                    phi_21_ = _e707;
                }
                let _e709 = phi_21_;
                phi_22_ = _e709;
            }
            let _e711 = phi_22_;
            if _e242 {
                phi_23_ = (_e241 <= 1f);
            } else {
                phi_23_ = false;
            }
            let _e714 = phi_23_;
            let _e715 = select(0f, 40f, _e714);
            switch bitcast<i32>(_e711) {
                case 0: {
                    phi_24_ = 12f;
                    break;
                }
                case 1: {
                    phi_24_ = 100f;
                    break;
                }
                case 2: {
                    phi_24_ = 188f;
                    break;
                }
                case 3: {
                    phi_24_ = (188f + _e715);
                    break;
                }
                case 4: {
                    phi_24_ = (228f + _e715);
                    break;
                }
                case 5: {
                    phi_24_ = (260f + _e715);
                    break;
                }
                default: {
                    phi_24_ = f32();
                    break;
                }
            }
            let _e721 = phi_24_;
            switch bitcast<i32>(_e711) {
                case 0: {
                    phi_25_ = true;
                    phi_26_ = false;
                    phi_27_ = false;
                    break;
                }
                case 1: {
                    phi_25_ = true;
                    phi_26_ = false;
                    phi_27_ = false;
                    break;
                }
                case 2: {
                    phi_25_ = false;
                    phi_26_ = true;
                    phi_27_ = false;
                    break;
                }
                case 3: {
                    phi_25_ = false;
                    phi_26_ = true;
                    phi_27_ = false;
                    break;
                }
                case 4: {
                    phi_25_ = false;
                    phi_26_ = false;
                    phi_27_ = true;
                    break;
                }
                case 5: {
                    phi_25_ = false;
                    phi_26_ = false;
                    phi_27_ = true;
                    break;
                }
                default: {
                    phi_25_ = bool();
                    phi_26_ = bool();
                    phi_27_ = bool();
                    break;
                }
            }
            let _e724 = phi_25_;
            let _e726 = phi_26_;
            let _e728 = phi_27_;
            let _e729 = select(_e726, false, _e724);
            let _e736 = (_e680 - (_e721 + (select(select(80f, 32f, _e729), 24f, select(select(_e728, false, _e724), false, _e729)) * 0.5f)));
            let _e737 = ((_e498 * _e258) - _e262);
            switch bitcast<i32>(_e711) {
                case 0: {
                    phi_28_ = vec2<f32>();
                    phi_29_ = true;
                    break;
                }
                case 1: {
                    phi_28_ = vec2<f32>();
                    phi_29_ = true;
                    break;
                }
                default: {
                    phi_28_ = vec2<f32>(0f, 0f);
                    phi_29_ = false;
                    break;
                }
            }
            let _e740 = phi_28_;
            let _e742 = phi_29_;
            if _e742 {
                let _e743 = (_e680 - 52f);
                let _e748 = pill_1.member[_e231].cpu.temperature;
                if (_e748 <= 62f) {
                    phi_38_ = vec2<f32>(0f, 0f);
                } else {
                    let _e751 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e743, _e737), 13f, 13f);
                    phi_30_ = 0i;
                    phi_31_ = 0.5f;
                    phi_32_ = 0f;
                    phi_33_ = vec2<f32>(((_e743 + (_e510 * 1.8f)) * 0.035f), (((_e737 + -(_e510)) * 0.035f) + 6.1f));
                    loop {
                        let _e761 = phi_30_;
                        let _e763 = phi_31_;
                        let _e765 = phi_32_;
                        let _e767 = phi_33_;
                        local_30 = _e765;
                        let _e768 = (_e761 < 4i);
                        if _e768 {
                            let _e771 = cantus_render_shader_simplex_noise(_e767);
                            phi_34_ = (_e761 + 1i);
                            phi_35_ = (_e763 * 0.5f);
                            phi_36_ = (_e765 + (_e771 * _e763));
                            phi_37_ = vec2<f32>(((_e767.x * 1.6f) + (_e767.y * 1.2f)), ((_e767.y * 1.6f) - (_e767.x * 1.2f)));
                        } else {
                            phi_34_ = i32();
                            phi_35_ = f32();
                            phi_36_ = f32();
                            phi_37_ = vec2<f32>();
                        }
                        let _e784 = phi_34_;
                        let _e786 = phi_35_;
                        let _e788 = phi_36_;
                        let _e790 = phi_37_;
                        continue;
                        continuing {
                            phi_30_ = _e784;
                            phi_31_ = _e786;
                            phi_32_ = _e788;
                            phi_33_ = _e790;
                            break if !(_e768);
                        }
                    }
                    let _e793 = local_30;
                    let _e794 = (_e793 * 0.5f);
                    let _e797 = ((_e751 - -0.5f) * 0.5f);
                    let _e799 = select(_e797, 0f, (_e797 < 0f));
                    let _e801 = select(_e799, 1f, (_e799 > 1f));
                    let _e807 = ((_e751 - 14f) * -0.083333336f);
                    let _e809 = select(_e807, 0f, (_e807 < 0f));
                    let _e811 = select(_e809, 1f, (_e809 > 1f));
                    let _e816 = (((_e801 * _e801) * (3f - (2f * _e801))) * ((_e811 * _e811) * (3f - (2f * _e811))));
                    let _e821 = ((_e794 + 0.19999999f) * 3.125f);
                    let _e823 = select(_e821, 0f, (_e821 < 0f));
                    let _e825 = select(_e823, 1f, (_e823 > 1f));
                    let _e832 = ((_e748 - 62f) * 0.045454547f);
                    let _e834 = select(_e832, 0f, (_e832 < 0f));
                    let _e836 = select(_e834, 1f, (_e834 > 1f));
                    let _e840 = ((_e836 * _e836) * (3f - (2f * _e836)));
                    phi_38_ = vec2<f32>(((_e816 * (0.18f + ((0.5f + _e794) * 0.34f))) * _e840), ((_e816 * ((_e825 * _e825) * (3f - (2f * _e825)))) * _e840));
                }
                let _e845 = phi_38_;
                let _e848 = (_e680 - 140f);
                let _e853 = pill_1.member[_e231].gpu.temperature;
                if (_e853 <= 62f) {
                    phi_47_ = vec2<f32>(0f, 0f);
                } else {
                    let _e856 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e848, _e737), 13f, 13f);
                    phi_39_ = 0i;
                    phi_40_ = 0.5f;
                    phi_41_ = 0f;
                    phi_42_ = vec2<f32>(((_e848 + (_e510 * 1.8f)) * 0.035f), (((_e737 + -(_e510)) * 0.035f) + 6.1f));
                    loop {
                        let _e866 = phi_39_;
                        let _e868 = phi_40_;
                        let _e870 = phi_41_;
                        let _e872 = phi_42_;
                        local_31 = _e870;
                        let _e873 = (_e866 < 4i);
                        if _e873 {
                            let _e876 = cantus_render_shader_simplex_noise(_e872);
                            phi_43_ = (_e866 + 1i);
                            phi_44_ = (_e868 * 0.5f);
                            phi_45_ = (_e870 + (_e876 * _e868));
                            phi_46_ = vec2<f32>(((_e872.x * 1.6f) + (_e872.y * 1.2f)), ((_e872.y * 1.6f) - (_e872.x * 1.2f)));
                        } else {
                            phi_43_ = i32();
                            phi_44_ = f32();
                            phi_45_ = f32();
                            phi_46_ = vec2<f32>();
                        }
                        let _e889 = phi_43_;
                        let _e891 = phi_44_;
                        let _e893 = phi_45_;
                        let _e895 = phi_46_;
                        continue;
                        continuing {
                            phi_39_ = _e889;
                            phi_40_ = _e891;
                            phi_41_ = _e893;
                            phi_42_ = _e895;
                            break if !(_e873);
                        }
                    }
                    let _e898 = local_31;
                    let _e899 = (_e898 * 0.5f);
                    let _e902 = ((_e856 - -0.5f) * 0.5f);
                    let _e904 = select(_e902, 0f, (_e902 < 0f));
                    let _e906 = select(_e904, 1f, (_e904 > 1f));
                    let _e912 = ((_e856 - 14f) * -0.083333336f);
                    let _e914 = select(_e912, 0f, (_e912 < 0f));
                    let _e916 = select(_e914, 1f, (_e914 > 1f));
                    let _e921 = (((_e906 * _e906) * (3f - (2f * _e906))) * ((_e916 * _e916) * (3f - (2f * _e916))));
                    let _e926 = ((_e899 + 0.19999999f) * 3.125f);
                    let _e928 = select(_e926, 0f, (_e926 < 0f));
                    let _e930 = select(_e928, 1f, (_e928 > 1f));
                    let _e937 = ((_e853 - 62f) * 0.045454547f);
                    let _e939 = select(_e937, 0f, (_e937 < 0f));
                    let _e941 = select(_e939, 1f, (_e939 > 1f));
                    let _e945 = ((_e941 * _e941) * (3f - (2f * _e941)));
                    phi_47_ = vec2<f32>(((_e921 * (0.18f + ((0.5f + _e899) * 0.34f))) * _e945), ((_e921 * ((_e930 * _e930) * (3f - (2f * _e930)))) * _e945));
                }
                let _e950 = phi_47_;
                phi_48_ = vec2<f32>(select(_e950.x, _e845.x, (_e845.x > _e950.x)), select(_e950.y, _e845.y, (_e845.y > _e950.y)));
            } else {
                phi_48_ = _e740;
            }
            let _e959 = phi_48_;
            let _e964 = pill_1.member[_e231].cpu.temperature;
            let _e969 = pill_1.member[_e231].gpu.temperature;
            if (_e964 != _e964) {
                phi_49_ = true;
            } else {
                phi_49_ = (_e969 >= _e964);
            }
            let _e973 = phi_49_;
            let _e974 = select(_e964, _e969, _e973);
            let _e976 = ((_e974 - 60f) * 0.083333336f);
            let _e978 = select(_e976, 0f, (_e976 < 0f));
            let _e980 = select(_e978, 1f, (_e978 > 1f));
            let _e984 = ((_e980 * _e980) * (3f - (2f * _e980)));
            let _e985 = (1f - _e984);
            let _e994 = ((_e974 - 72f) * 0.0625f);
            let _e996 = select(_e994, 0f, (_e994 < 0f));
            let _e998 = select(_e996, 1f, (_e996 > 1f));
            let _e1002 = ((_e998 * _e998) * (3f - (2f * _e998)));
            let _e1003 = (1f - _e1002);
            let _e1013 = (_e959.y * 0.12f);
            let _e1014 = (0.24f + _e1013);
            let _e1015 = (0.76f - _e1013);
            let _e1027 = (1f - (_e959.x * 0.46f));
            let _e1037 = (_e959.y * 0.64f);
            let _e1038 = (1f - _e1037);
            let _e1045 = (((((((((((((((((((0.008f * _e549) + (0.03f * _e526)) * _e547) + (((0.09f * _e549) + (0.34f * _e526)) * _e536)) * _e585) + ((_e561 + (0.3f * _e526)) * _e548)) * _e600) + (0.16f * _e599)) * _e611) + _e615) * _e620) + (_e506.fog * 0.3844f)) * _e644) + (_e642 * 0.2925f)) + _e676) * _e1027) + (_e959.x * 0.0009200001f)) * _e1038) + (((0.07f * _e1015) + (((((0.22f * _e985) + _e984) * _e1003) + _e1002) * _e1014)) * _e1037));
            let _e1046 = (((((((((((((((((((0.015f * _e549) + (0.06f * _e526)) * _e547) + (((0.37f * _e549) + (0.7f * _e526)) * _e536)) * _e585) + (((0.25f * _e549) + (0.2f * _e526)) * _e548)) * _e600) + (0.2f * _e599)) * _e611) + _e615) * _e620) + (_e506.fog * 0.4216f)) * _e644) + (_e642 * 0.333f)) + _e676) * _e1027) + (_e959.x * 0.00276f)) * _e1038) + (((0.12f * _e1015) + (((((0.62f * _e985) + (0.38f * _e984)) * _e1003) + (0.08f * _e1002)) * _e1014)) * _e1037));
            let _e1047 = (((((((((((((((((((0.04f * _e549) + (0.13f * _e526)) * _e547) + ((_e561 + (0.9f * _e526)) * _e536)) * _e585) + (((0.2f * _e549) + (0.4f * _e526)) * _e548)) * _e600) + (0.27f * _e599)) * _e611) + _e615) * _e620) + (_e506.fog * 0.44640002f)) * _e644) + (_e642 * 0.43199998f)) + _e676) * _e1027) + (_e959.x * 0.00552f)) * _e1038) + (((0.18f * _e1015) + ((((_e985 + (0.08f * _e984)) * _e1003) + (0.035f * _e1002)) * _e1014)) * _e1037));
            switch bitcast<i32>(_e711) {
                case 0: {
                    let _e1769 = pill_1.member[_e231].history_scroll;
                    switch bitcast<i32>(_e711) {
                        case 0: {
                            phi_56_ = true;
                            phi_57_ = false;
                            phi_58_ = false;
                            break;
                        }
                        case 1: {
                            phi_56_ = true;
                            phi_57_ = false;
                            phi_58_ = false;
                            break;
                        }
                        case 2: {
                            phi_56_ = false;
                            phi_57_ = true;
                            phi_58_ = false;
                            break;
                        }
                        case 3: {
                            phi_56_ = false;
                            phi_57_ = true;
                            phi_58_ = false;
                            break;
                        }
                        case 4: {
                            phi_56_ = false;
                            phi_57_ = false;
                            phi_58_ = true;
                            break;
                        }
                        case 5: {
                            phi_56_ = false;
                            phi_57_ = false;
                            phi_58_ = true;
                            break;
                        }
                        default: {
                            phi_56_ = bool();
                            phi_57_ = bool();
                            phi_58_ = bool();
                            break;
                        }
                    }
                    let _e1772 = phi_56_;
                    let _e1774 = phi_57_;
                    let _e1776 = phi_58_;
                    let _e1777 = select(_e1774, false, _e1772);
                    let _e1783 = ((select(select(80f, 32f, _e1777), 24f, select(select(_e1776, false, _e1772), false, _e1777)) * 0.5f) - 4f);
                    let _e1784 = (_e262 - 8f);
                    let _e1785 = (_e1783 - _e1784);
                    let _e1787 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e736, _e737), _e1785, _e1784);
                    let _e1788 = abs(_e736);
                    let _e1789 = abs(_e737);
                    let _e1792 = (round((_e1788 * 0.11111111f)) * 9f);
                    if (_e1792 != _e1792) {
                        phi_59_ = true;
                    } else {
                        phi_59_ = (_e1783 <= _e1792);
                    }
                    let _e1796 = phi_59_;
                    let _e1797 = select(_e1792, _e1783, _e1796);
                    let _e1798 = (_e1797 - _e1785);
                    if (_e1798 != _e1798) {
                        phi_60_ = true;
                    } else {
                        phi_60_ = (0f >= _e1798);
                    }
                    let _e1802 = phi_60_;
                    let _e1803 = select(_e1798, 0f, _e1802);
                    let _e1804 = (_e1784 * _e1784);
                    let _e1807 = sqrt((_e1804 - (_e1803 * _e1803)));
                    let _e1808 = (_e1803 / _e1784);
                    let _e1809 = (_e1807 / _e1784);
                    let _e1814 = ((_e1788 - _e1797) - (_e1808 * 0.9f));
                    let _e1815 = ((_e1789 - _e1807) - (_e1809 * 0.9f));
                    let _e1824 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1814 * -(_e1809)) + (_e1815 * _e1808)), ((_e1814 * _e1808) + (_e1815 * _e1809))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e1826 = round((_e1789 * 0.125f));
                    if (_e1826 != _e1826) {
                        phi_61_ = true;
                    } else {
                        phi_61_ = (1f <= _e1826);
                    }
                    let _e1830 = phi_61_;
                    let _e1832 = (select(_e1826, 1f, _e1830) * 8f);
                    let _e1835 = sqrt((_e1804 - (_e1832 * _e1832)));
                    let _e1837 = (_e1835 / _e1784);
                    let _e1838 = (_e1832 / _e1784);
                    let _e1843 = ((_e1788 - (_e1785 + _e1835)) - (_e1837 * 0.9f));
                    let _e1844 = ((_e1789 - _e1832) - (_e1838 * 0.9f));
                    let _e1853 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1843 * -(_e1838)) + (_e1844 * _e1837)), ((_e1843 * _e1837) + (_e1844 * _e1838))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e1824 != _e1824) {
                        phi_62_ = true;
                    } else {
                        phi_62_ = (_e1853 <= _e1824);
                    }
                    let _e1857 = phi_62_;
                    let _e1858 = select(_e1824, _e1853, _e1857);
                    let _e1861 = (0.5f + ((_e1858 - _e1787) * 0.3125f));
                    let _e1863 = select(_e1861, 0f, (_e1861 < 0f));
                    let _e1865 = select(_e1863, 1f, (_e1863 > 1f));
                    let _e1874 = ((_e1787 - 0.55f) * -0.9090909f);
                    let _e1876 = select(_e1874, 0f, (_e1874 < 0f));
                    let _e1878 = select(_e1876, 1f, (_e1876 > 1f));
                    let _e1882 = ((_e1878 * _e1878) * (3f - (2f * _e1878)));
                    let _e1883 = (_e1783 * 0.051282052f);
                    let _e1884 = (_e736 + _e1783);
                    let _e1886 = ((_e1884 / _e1883) + _e1769);
                    let _e1888 = select(_e1886, 0f, (_e1886 < 0f));
                    let _e1890 = select(_e1888, 39f, (_e1888 > 39f));
                    let _e1891 = floor(_e1890);
                    let _e1896 = select(select(u32(_e1891), 0u, (_e1891 < 0f)), 4294967295u, (_e1891 > 4294967000f));
                    let _e1897 = (_e262 - 10f);
                    let _e1901 = (((f32(_e1896) - _e1769) * _e1883) - _e1783);
                    let _e1903 = select(_e1896, 39u, (39u < _e1896));
                    let _e1904 = (_e1903 < 40u);
                    if _e1904 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e1911 = pill_1.member[_e231].cpu.usage.samples[_e1903];
                    let _e1914 = (_e1897 * (1f - (_e1911 * 2f)));
                    let _e1915 = (_e1896 + 1u);
                    let _e1921 = select(_e1915, 39u, (39u < _e1915));
                    let _e1922 = (_e1921 < 40u);
                    if _e1922 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e1929 = pill_1.member[_e231].cpu.usage.samples[_e1921];
                    let _e1933 = ((((f32(_e1915) - _e1769) * _e1883) - _e1783) - _e1901);
                    let _e1934 = ((_e1897 * (1f - (_e1929 * 2f))) - _e1914);
                    let _e1935 = (_e736 - _e1901);
                    let _e1936 = (_e737 - _e1914);
                    let _e1937 = (_e1935 * _e1933);
                    let _e1940 = (_e1933 * _e1933);
                    let _e1942 = (_e1940 + (_e1934 * _e1934));
                    if (_e1942 != _e1942) {
                        phi_63_ = true;
                    } else {
                        phi_63_ = (0.001f >= _e1942);
                    }
                    let _e1946 = phi_63_;
                    let _e1948 = ((_e1937 + (_e1936 * _e1934)) / select(_e1942, 0.001f, _e1946));
                    let _e1950 = select(_e1948, 0f, (_e1948 < 0f));
                    let _e1952 = select(_e1950, 1f, (_e1950 > 1f));
                    let _e1955 = (_e1935 - (_e1933 * _e1952));
                    let _e1956 = (_e1936 - (_e1934 * _e1952));
                    let _e1963 = ((abs(sqrt(((_e1955 * _e1955) + (_e1956 * _e1956)))) - 1.4000001f) * -0.9090908f);
                    let _e1965 = select(_e1963, 0f, (_e1963 < 0f));
                    let _e1967 = select(_e1965, 1f, (_e1965 > 1f));
                    let _e1973 = (_e1890 - trunc(_e1890));
                    let _e1975 = select(_e1973, 0f, (_e1973 < 0f));
                    let _e1977 = select(_e1975, 1f, (_e1975 > 1f));
                    let _e1981 = ((_e1977 * _e1977) * (3f - (2f * _e1977)));
                    let _e1988 = ((((_e1914 + (_e1934 * _e1981)) - _e737) - 0.55f) * -0.9090909f);
                    let _e1990 = select(_e1988, 0f, (_e1988 < 0f));
                    let _e1992 = select(_e1990, 1f, (_e1990 > 1f));
                    let _e1998 = ((((_e1992 * _e1992) * (3f - (2f * _e1992))) * 0.156f) + ((_e1967 * _e1967) * (3f - (2f * _e1967))));
                    if _e1904 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e2007 = pill_1.member[_e231].cpu.memory.samples[_e1903];
                    let _e2010 = (_e1897 * (1f - (_e2007 * 2f)));
                    if _e1922 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e2017 = pill_1.member[_e231].cpu.memory.samples[_e1921];
                    let _e2021 = ((_e1897 * (1f - (_e2017 * 2f))) - _e2010);
                    let _e2022 = (_e737 - _e2010);
                    let _e2026 = (_e1940 + (_e2021 * _e2021));
                    if (_e2026 != _e2026) {
                        phi_64_ = true;
                    } else {
                        phi_64_ = (0.001f >= _e2026);
                    }
                    let _e2030 = phi_64_;
                    let _e2032 = ((_e1937 + (_e2022 * _e2021)) / select(_e2026, 0.001f, _e2030));
                    let _e2034 = select(_e2032, 0f, (_e2032 < 0f));
                    let _e2036 = select(_e2034, 1f, (_e2034 > 1f));
                    let _e2039 = (_e1935 - (_e1933 * _e2036));
                    let _e2040 = (_e2022 - (_e2021 * _e2036));
                    let _e2047 = ((abs(sqrt(((_e2039 * _e2039) + (_e2040 * _e2040)))) - 1.4000001f) * -0.9090908f);
                    let _e2049 = select(_e2047, 0f, (_e2047 < 0f));
                    let _e2051 = select(_e2049, 1f, (_e2049 > 1f));
                    let _e2062 = ((((_e2010 + (_e2021 * _e1981)) - _e737) - 0.55f) * -0.9090909f);
                    let _e2064 = select(_e2062, 0f, (_e2062 < 0f));
                    let _e2066 = select(_e2064, 1f, (_e2064 > 1f));
                    let _e2072 = ((((_e2066 * _e2066) * (3f - (2f * _e2066))) * 0.084f) + ((_e2051 * _e2051) * (3f - (2f * _e2051))));
                    let _e2080 = (_e1884 * 0.14285715f);
                    let _e2081 = ((_e737 + _e1784) * 0.16393442f);
                    let _e2091 = ((abs(((_e2080 - trunc(_e2080)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2093 = select(_e2091, 0f, (_e2091 < 0f));
                    let _e2095 = select(_e2093, 1f, (_e2093 > 1f));
                    let _e2099 = ((_e2095 * _e2095) * (3f - (2f * _e2095)));
                    let _e2101 = ((abs(((_e2081 - trunc(_e2081)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2103 = select(_e2101, 0f, (_e2101 < 0f));
                    let _e2105 = select(_e2103, 1f, (_e2103 > 1f));
                    let _e2109 = ((_e2105 * _e2105) * (3f - (2f * _e2105)));
                    if (_e2099 != _e2099) {
                        phi_65_ = true;
                    } else {
                        phi_65_ = (_e2109 >= _e2099);
                    }
                    let _e2113 = phi_65_;
                    let _e2121 = pill_1.member[_e231].cpu.usage.samples[39u];
                    let _e2122 = (_e2121 * 0.24f);
                    let _e2123 = (0.18f + _e2122);
                    let _e2124 = (0.82f - _e2122);
                    let _e2133 = (_e964 - 60f);
                    let _e2134 = (_e2133 * 0.083333336f);
                    let _e2136 = select(_e2134, 0f, (_e2134 < 0f));
                    let _e2138 = select(_e2136, 1f, (_e2136 > 1f));
                    let _e2142 = ((_e2138 * _e2138) * (3f - (2f * _e2138)));
                    let _e2143 = (1f - _e2142);
                    let _e2152 = ((_e964 - 72f) * 0.0625f);
                    let _e2154 = select(_e2152, 0f, (_e2152 < 0f));
                    let _e2156 = select(_e2154, 1f, (_e2154 > 1f));
                    let _e2160 = ((_e2156 * _e2156) * (3f - (2f * _e2156)));
                    let _e2161 = (1f - _e2160);
                    let _e2170 = (_e2133 * 0.03846154f);
                    let _e2172 = select(_e2170, 0f, (_e2170 < 0f));
                    let _e2174 = select(_e2172, 1f, (_e2172 > 1f));
                    let _e2179 = (((_e2174 * _e2174) * (3f - (2f * _e2174))) * 0.9f);
                    let _e2180 = (1f - _e2179);
                    let _e2187 = ((((0.025f * _e2124) + (0.32f * _e2123)) * _e2180) + (((((0.22f * _e2143) + _e2142) * _e2161) + _e2160) * _e2179));
                    let _e2188 = ((((0.09f * _e2124) + (0.68f * _e2123)) * _e2180) + (((((0.62f * _e2143) + (0.38f * _e2142)) * _e2161) + (0.08f * _e2160)) * _e2179));
                    let _e2189 = ((((0.15f * _e2124) + _e2123) * _e2180) + ((((_e2143 + (0.08f * _e2142)) * _e2161) + (0.035f * _e2160)) * _e2179));
                    let _e2191 = ((((_e1858 + ((_e1787 - _e1858) * _e1865)) - ((1.6f * _e1865) * (1f - _e1865))) - 0.55f) * -0.9090909f);
                    let _e2193 = select(_e2191, 0f, (_e2191 < 0f));
                    let _e2195 = select(_e2193, 1f, (_e2193 > 1f));
                    let _e2199 = ((_e2195 * _e2195) * (3f - (2f * _e2195)));
                    let _e2201 = (1f - (_e2199 * 0.82f));
                    let _e2213 = ((abs(_e1787) - 2.1f) * -0.909091f);
                    let _e2215 = select(_e2213, 0f, (_e2213 < 0f));
                    let _e2217 = select(_e2215, 1f, (_e2215 > 1f));
                    let _e2222 = (((_e2217 * _e2217) * (3f - (2f * _e2217))) * 0.92f);
                    let _e2223 = (1f - _e2222);
                    let _e2234 = ((_e1858 - 0.55f) * -0.9090909f);
                    let _e2236 = select(_e2234, 0f, (_e2234 < 0f));
                    let _e2238 = select(_e2236, 1f, (_e2236 > 1f));
                    let _e2243 = (((_e2238 * _e2238) * (3f - (2f * _e2238))) * 0.78f);
                    let _e2244 = (1f - _e2243);
                    let _e2255 = ((_e1882 * select(_e2099, _e2109, _e2113)) * 0.045f);
                    phi_66_ = _e392;
                    phi_67_ = vec3<f32>(((((((((_e1045 * _e2201) + (_e2199 * 0.00328f)) * _e2223) + (_e2187 * _e2222)) * _e2244) + (_e2187 * _e2243)) + _e2255) + (((0.32f * _e1882) * _e1998) + ((0.78f * _e1882) * _e2072))), ((((((((_e1046 * _e2201) + (_e2199 * 0.00984f)) * _e2223) + (_e2188 * _e2222)) * _e2244) + (_e2188 * _e2243)) + _e2255) + (((0.68f * _e1882) * _e1998) + ((0.3f * _e1882) * _e2072))), ((((((((_e1047 * _e2201) + (_e2199 * 0.02132f)) * _e2223) + (_e2189 * _e2222)) * _e2244) + (_e2189 * _e2243)) + _e2255) + (_e1882 * (_e1998 + _e2072))));
                    phi_68_ = false;
                    break;
                }
                case 1: {
                    let _e1388 = pill_1.member[_e231].history_scroll;
                    switch bitcast<i32>(_e711) {
                        case 0: {
                            phi_50_ = true;
                            phi_51_ = false;
                            phi_52_ = false;
                            break;
                        }
                        case 1: {
                            phi_50_ = true;
                            phi_51_ = false;
                            phi_52_ = false;
                            break;
                        }
                        case 2: {
                            phi_50_ = false;
                            phi_51_ = true;
                            phi_52_ = false;
                            break;
                        }
                        case 3: {
                            phi_50_ = false;
                            phi_51_ = true;
                            phi_52_ = false;
                            break;
                        }
                        case 4: {
                            phi_50_ = false;
                            phi_51_ = false;
                            phi_52_ = true;
                            break;
                        }
                        case 5: {
                            phi_50_ = false;
                            phi_51_ = false;
                            phi_52_ = true;
                            break;
                        }
                        default: {
                            phi_50_ = bool();
                            phi_51_ = bool();
                            phi_52_ = bool();
                            break;
                        }
                    }
                    let _e1391 = phi_50_;
                    let _e1393 = phi_51_;
                    let _e1395 = phi_52_;
                    let _e1396 = select(_e1393, false, _e1391);
                    let _e1402 = ((select(select(80f, 32f, _e1396), 24f, select(select(_e1395, false, _e1391), false, _e1396)) * 0.5f) - 4f);
                    let _e1403 = (_e262 - 8f);
                    let _e1406 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e736, _e737), (_e1402 - _e1403), _e1403);
                    let _e1408 = ((_e1406 - 0.55f) * -0.9090909f);
                    let _e1410 = select(_e1408, 0f, (_e1408 < 0f));
                    let _e1412 = select(_e1410, 1f, (_e1410 > 1f));
                    let _e1416 = ((_e1412 * _e1412) * (3f - (2f * _e1412)));
                    let _e1417 = (_e1402 * 0.051282052f);
                    let _e1418 = (_e736 + _e1402);
                    let _e1420 = ((_e1418 / _e1417) + _e1388);
                    let _e1422 = select(_e1420, 0f, (_e1420 < 0f));
                    let _e1424 = select(_e1422, 39f, (_e1422 > 39f));
                    let _e1425 = floor(_e1424);
                    let _e1430 = select(select(u32(_e1425), 0u, (_e1425 < 0f)), 4294967295u, (_e1425 > 4294967000f));
                    let _e1431 = (_e262 - 10f);
                    let _e1435 = (((f32(_e1430) - _e1388) * _e1417) - _e1402);
                    let _e1437 = select(_e1430, 39u, (39u < _e1430));
                    let _e1438 = (_e1437 < 40u);
                    if _e1438 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e1445 = pill_1.member[_e231].gpu.usage.samples[_e1437];
                    let _e1448 = (_e1431 * (1f - (_e1445 * 2f)));
                    let _e1449 = (_e1430 + 1u);
                    let _e1455 = select(_e1449, 39u, (39u < _e1449));
                    let _e1456 = (_e1455 < 40u);
                    if _e1456 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e1463 = pill_1.member[_e231].gpu.usage.samples[_e1455];
                    let _e1467 = ((((f32(_e1449) - _e1388) * _e1417) - _e1402) - _e1435);
                    let _e1468 = ((_e1431 * (1f - (_e1463 * 2f))) - _e1448);
                    let _e1469 = (_e736 - _e1435);
                    let _e1470 = (_e737 - _e1448);
                    let _e1471 = (_e1469 * _e1467);
                    let _e1474 = (_e1467 * _e1467);
                    let _e1476 = (_e1474 + (_e1468 * _e1468));
                    if (_e1476 != _e1476) {
                        phi_53_ = true;
                    } else {
                        phi_53_ = (0.001f >= _e1476);
                    }
                    let _e1480 = phi_53_;
                    let _e1482 = ((_e1471 + (_e1470 * _e1468)) / select(_e1476, 0.001f, _e1480));
                    let _e1484 = select(_e1482, 0f, (_e1482 < 0f));
                    let _e1486 = select(_e1484, 1f, (_e1484 > 1f));
                    let _e1489 = (_e1469 - (_e1467 * _e1486));
                    let _e1490 = (_e1470 - (_e1468 * _e1486));
                    let _e1497 = ((abs(sqrt(((_e1489 * _e1489) + (_e1490 * _e1490)))) - 1.4000001f) * -0.9090908f);
                    let _e1499 = select(_e1497, 0f, (_e1497 < 0f));
                    let _e1501 = select(_e1499, 1f, (_e1499 > 1f));
                    let _e1507 = (_e1424 - trunc(_e1424));
                    let _e1509 = select(_e1507, 0f, (_e1507 < 0f));
                    let _e1511 = select(_e1509, 1f, (_e1509 > 1f));
                    let _e1515 = ((_e1511 * _e1511) * (3f - (2f * _e1511)));
                    let _e1522 = ((((_e1448 + (_e1468 * _e1515)) - _e737) - 0.55f) * -0.9090909f);
                    let _e1524 = select(_e1522, 0f, (_e1522 < 0f));
                    let _e1526 = select(_e1524, 1f, (_e1524 > 1f));
                    let _e1532 = ((((_e1526 * _e1526) * (3f - (2f * _e1526))) * 0.156f) + ((_e1501 * _e1501) * (3f - (2f * _e1501))));
                    if _e1438 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e1541 = pill_1.member[_e231].gpu.memory.samples[_e1437];
                    let _e1544 = (_e1431 * (1f - (_e1541 * 2f)));
                    if _e1456 {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e1551 = pill_1.member[_e231].gpu.memory.samples[_e1455];
                    let _e1555 = ((_e1431 * (1f - (_e1551 * 2f))) - _e1544);
                    let _e1556 = (_e737 - _e1544);
                    let _e1560 = (_e1474 + (_e1555 * _e1555));
                    if (_e1560 != _e1560) {
                        phi_54_ = true;
                    } else {
                        phi_54_ = (0.001f >= _e1560);
                    }
                    let _e1564 = phi_54_;
                    let _e1566 = ((_e1471 + (_e1556 * _e1555)) / select(_e1560, 0.001f, _e1564));
                    let _e1568 = select(_e1566, 0f, (_e1566 < 0f));
                    let _e1570 = select(_e1568, 1f, (_e1568 > 1f));
                    let _e1573 = (_e1469 - (_e1467 * _e1570));
                    let _e1574 = (_e1556 - (_e1555 * _e1570));
                    let _e1581 = ((abs(sqrt(((_e1573 * _e1573) + (_e1574 * _e1574)))) - 1.4000001f) * -0.9090908f);
                    let _e1583 = select(_e1581, 0f, (_e1581 < 0f));
                    let _e1585 = select(_e1583, 1f, (_e1583 > 1f));
                    let _e1596 = ((((_e1544 + (_e1555 * _e1515)) - _e737) - 0.55f) * -0.9090909f);
                    let _e1598 = select(_e1596, 0f, (_e1596 < 0f));
                    let _e1600 = select(_e1598, 1f, (_e1598 > 1f));
                    let _e1606 = ((((_e1600 * _e1600) * (3f - (2f * _e1600))) * 0.084f) + ((_e1585 * _e1585) * (3f - (2f * _e1585))));
                    let _e1614 = (_e1418 * 0.14285715f);
                    let _e1615 = ((_e737 + _e1403) * 0.16393442f);
                    let _e1625 = ((abs(((_e1614 - trunc(_e1614)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e1627 = select(_e1625, 0f, (_e1625 < 0f));
                    let _e1629 = select(_e1627, 1f, (_e1627 > 1f));
                    let _e1633 = ((_e1629 * _e1629) * (3f - (2f * _e1629)));
                    let _e1635 = ((abs(((_e1615 - trunc(_e1615)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e1637 = select(_e1635, 0f, (_e1635 < 0f));
                    let _e1639 = select(_e1637, 1f, (_e1637 > 1f));
                    let _e1643 = ((_e1639 * _e1639) * (3f - (2f * _e1639)));
                    if (_e1633 != _e1633) {
                        phi_55_ = true;
                    } else {
                        phi_55_ = (_e1643 >= _e1633);
                    }
                    let _e1647 = phi_55_;
                    let _e1655 = pill_1.member[_e231].gpu.usage.samples[39u];
                    let _e1656 = (_e1655 * 0.24f);
                    let _e1657 = (0.18f + _e1656);
                    let _e1658 = (0.82f - _e1656);
                    let _e1667 = (_e969 - 60f);
                    let _e1668 = (_e1667 * 0.083333336f);
                    let _e1670 = select(_e1668, 0f, (_e1668 < 0f));
                    let _e1672 = select(_e1670, 1f, (_e1670 > 1f));
                    let _e1676 = ((_e1672 * _e1672) * (3f - (2f * _e1672)));
                    let _e1677 = (1f - _e1676);
                    let _e1686 = ((_e969 - 72f) * 0.0625f);
                    let _e1688 = select(_e1686, 0f, (_e1686 < 0f));
                    let _e1690 = select(_e1688, 1f, (_e1688 > 1f));
                    let _e1694 = ((_e1690 * _e1690) * (3f - (2f * _e1690)));
                    let _e1695 = (1f - _e1694);
                    let _e1704 = (_e1667 * 0.03846154f);
                    let _e1706 = select(_e1704, 0f, (_e1704 < 0f));
                    let _e1708 = select(_e1706, 1f, (_e1706 > 1f));
                    let _e1713 = (((_e1708 * _e1708) * (3f - (2f * _e1708))) * 0.9f);
                    let _e1714 = (1f - _e1713);
                    let _e1725 = (1f - (_e1416 * 0.82f));
                    let _e1737 = ((abs(_e1406) - 2.1f) * -0.909091f);
                    let _e1739 = select(_e1737, 0f, (_e1737 < 0f));
                    let _e1741 = select(_e1739, 1f, (_e1739 > 1f));
                    let _e1746 = (((_e1741 * _e1741) * (3f - (2f * _e1741))) * 0.92f);
                    let _e1747 = (1f - _e1746);
                    let _e1758 = ((_e1416 * select(_e1633, _e1643, _e1647)) * 0.045f);
                    phi_66_ = _e392;
                    phi_67_ = vec3<f32>(((((((_e1045 * _e1725) + (_e1416 * 0.00328f)) * _e1747) + (((((0.025f * _e1658) + (0.32f * _e1657)) * _e1714) + (((((0.22f * _e1677) + _e1676) * _e1695) + _e1694) * _e1713)) * _e1746)) + _e1758) + (((0.32f * _e1416) * _e1532) + ((0.78f * _e1416) * _e1606))), ((((((_e1046 * _e1725) + (_e1416 * 0.00984f)) * _e1747) + (((((0.09f * _e1658) + (0.68f * _e1657)) * _e1714) + (((((0.62f * _e1677) + (0.38f * _e1676)) * _e1695) + (0.08f * _e1694)) * _e1713)) * _e1746)) + _e1758) + (((0.68f * _e1416) * _e1532) + ((0.3f * _e1416) * _e1606))), ((((((_e1047 * _e1725) + (_e1416 * 0.02132f)) * _e1747) + (((((0.15f * _e1658) + _e1657) * _e1714) + ((((_e1677 + (0.08f * _e1676)) * _e1695) + (0.035f * _e1694)) * _e1713)) * _e1746)) + _e1758) + (_e1416 * (_e1532 + _e1606))));
                    phi_68_ = false;
                    break;
                }
                case 2: {
                    let _e1174 = (_e736 * 1.25f);
                    let _e1175 = (_e737 * 1.25f);
                    let _e1177 = select(0f, 1f, (_e241 < 0f));
                    let _e1178 = abs(_e241);
                    let _e1179 = (_e1175 - 1f);
                    let _e1180 = vec2<f32>(_e1174, _e1179);
                    let _e1181 = cantus_render_shader_sd_rounded_box(_e1180, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e1184 = ((abs(_e1181) - 2.425f) * -0.909091f);
                    let _e1186 = select(_e1184, 0f, (_e1184 < 0f));
                    let _e1188 = select(_e1186, 1f, (_e1186 > 1f));
                    let _e1195 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1174, (_e1175 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e1197 = ((_e1195 - 0.55f) * -0.9090909f);
                    let _e1199 = select(_e1197, 0f, (_e1197 < 0f));
                    let _e1201 = select(_e1199, 1f, (_e1199 > 1f));
                    let _e1206 = cantus_render_shader_sd_rounded_box(_e1180, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e1208 = ((_e1206 - 0.55f) * -0.9090909f);
                    let _e1210 = select(_e1208, 0f, (_e1208 < 0f));
                    let _e1212 = select(_e1210, 1f, (_e1210 > 1f));
                    let _e1218 = select(_e1178, 0f, (_e1178 < 0f));
                    let _e1236 = ((12f - (select(_e1218, 1f, (_e1218 > 1f)) * 24f)) + ((sin(((_e736 * 0.775f) + (_e510 * (1.4f + (_e1177 * 1.2f))))) * 1.15f) + (sin(((_e736 * 0.3375f) - (_e510 * 0.8f))) * 0.45f)));
                    let _e1237 = (_e1236 - 0.7f);
                    let _e1241 = ((_e1179 - _e1237) / ((_e1236 + 0.7f) - _e1237));
                    let _e1243 = select(_e1241, 0f, (_e1241 < 0f));
                    let _e1245 = select(_e1243, 1f, (_e1243 > 1f));
                    let _e1250 = (((_e1212 * _e1212) * (3f - (2f * _e1212))) * ((_e1245 * _e1245) * (3f - (2f * _e1245))));
                    let _e1252 = ((_e1178 - 0.08f) * 5f);
                    let _e1254 = select(_e1252, 0f, (_e1252 < 0f));
                    let _e1256 = select(_e1254, 1f, (_e1254 > 1f));
                    let _e1260 = ((_e1256 * _e1256) * (3f - (2f * _e1256)));
                    let _e1261 = (1f - _e1260);
                    let _e1269 = ((_e1178 - 0.18f) * 1.8518518f);
                    let _e1271 = select(_e1269, 0f, (_e1269 < 0f));
                    let _e1273 = select(_e1271, 1f, (_e1271 > 1f));
                    let _e1277 = ((_e1273 * _e1273) * (3f - (2f * _e1273)));
                    let _e1278 = (1f - _e1277);
                    let _e1284 = (_e1278 + (0.22f * _e1277));
                    let _e1285 = ((((0.18f * _e1261) + (0.72f * _e1260)) * _e1278) + (0.95f * _e1277));
                    let _e1286 = ((((0.1f * _e1261) + (0.12f * _e1260)) * _e1278) + (0.55f * _e1277));
                    let _e1289 = floor((_e736 * 0.4166667f));
                    let _e1290 = floor((_e737 * 0.36764705f));
                    let _e1292 = cantus_render_shader_hash(vec2<f32>(_e1289, _e1290));
                    let _e1306 = ((_e510 * (0.5f + _e1292.y)) + (_e1292.x * 11f));
                    let _e1308 = (_e1306 - trunc(_e1306));
                    let _e1309 = (_e1174 - (((_e1289 + 0.2f) + (_e1292.x * 0.6f)) * 3f));
                    let _e1312 = ((_e1175 - (((_e1290 + 0.2f) + (_e1292.y * 0.6f)) * 3.4f)) + (_e1308 * 5f));
                    let _e1320 = (_e1308 * 4f);
                    let _e1322 = select(_e1320, 0f, (_e1320 < 0f));
                    let _e1324 = select(_e1322, 1f, (_e1322 > 1f));
                    let _e1330 = ((_e1308 - 1f) * -3.3333333f);
                    let _e1332 = select(_e1330, 0f, (_e1330 < 0f));
                    let _e1334 = select(_e1332, 1f, (_e1332 > 1f));
                    let _e1342 = ((abs((sqrt(((_e1309 * _e1309) + (_e1312 * _e1312))) - (0.4f + (_e1292.y * 0.5f)))) - 1f) * -0.9090909f);
                    let _e1344 = select(_e1342, 0f, (_e1342 < 0f));
                    let _e1346 = select(_e1344, 1f, (_e1344 > 1f));
                    let _e1353 = (((((_e1346 * _e1346) * (3f - (2f * _e1346))) * (((_e1324 * _e1324) * (3f - (2f * _e1324))) * ((_e1334 * _e1334) * (3f - (2f * _e1334))))) * _e1250) * _e1177);
                    let _e1356 = ((((_e1188 * _e1188) * (3f - (2f * _e1188))) * 0.43f) + (((_e1201 * _e1201) * (3f - (2f * _e1201))) * 0.38f));
                    phi_66_ = _e392;
                    phi_67_ = vec3<f32>((_e1045 + ((_e1356 + ((_e1284 * _e1250) * 0.78f)) + ((((_e1284 * 0.27999997f) + 0.72f) * _e1353) * 0.9f))), (_e1046 + ((_e1356 + ((_e1285 * _e1250) * 0.78f)) + ((((_e1285 * 0.27999997f) + 0.72f) * _e1353) * 0.9f))), (_e1047 + ((_e1356 + ((_e1286 * _e1250) * 0.78f)) + ((((_e1286 * 0.27999997f) + 0.72f) * _e1353) * 0.9f))));
                    phi_68_ = false;
                    break;
                }
                case 3: {
                    let _e1052 = pill_1.member[_e231].volume;
                    let _e1054 = select(0f, 1f, (_e1052 < 0f));
                    let _e1055 = abs(_e1052);
                    let _e1058 = round(((_e736 + 12f) * 0.25f));
                    let _e1060 = select(_e1058, 0f, (_e1058 < 0f));
                    let _e1062 = select(_e1060, 6f, (_e1060 > 6f));
                    let _e1067 = select(select(u32(_e1062), 0u, (_e1062 < 0f)), 4294967295u, (_e1062 > 4294967000f));
                    if (_e1067 < 7u) {
                    } else {
                        phi_66_ = true;
                        phi_67_ = vec3<f32>();
                        phi_68_ = bool();
                        break;
                    }
                    let _e1073 = pill_1.member[_e231].audio_spectrum[_e1067];
                    let _e1074 = (1f - _e1054);
                    let _e1075 = (_e1073 * _e1074);
                    let _e1084 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e736 - (-12f + (_e1062 * 4f))), (_e737 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1075))), 1.25f);
                    let _e1087 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e736, (_e737 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1089 = ((_e1087 - 0.55f) * -0.9090909f);
                    let _e1091 = select(_e1089, 0f, (_e1089 < 0f));
                    let _e1093 = select(_e1091, 1f, (_e1091 > 1f));
                    let _e1097 = ((_e1093 * _e1093) * (3f - (2f * _e1093)));
                    let _e1099 = select(_e1055, 0f, (_e1055 < 0f));
                    let _e1102 = (select(_e1099, 1f, (_e1099 > 1f)) * 28f);
                    let _e1103 = (_e1102 + -13.2f);
                    let _e1107 = ((_e736 - _e1103) / ((_e1102 + -14.8f) - _e1103));
                    let _e1109 = select(_e1107, 0f, (_e1107 < 0f));
                    let _e1111 = select(_e1109, 1f, (_e1109 > 1f));
                    let _e1116 = (_e1097 * ((_e1111 * _e1111) * (3f - (2f * _e1111))));
                    let _e1118 = (1f - (_e1055 * 0.65f));
                    let _e1123 = ((0.08f * _e1118) + (_e1055 * 0.42249995f));
                    let _e1124 = ((0.88f * _e1118) + (_e1055 * 0.221f));
                    let _e1126 = ((_e1084 - 0.7f) * -0.71428573f);
                    let _e1128 = select(_e1126, 0f, (_e1126 < 0f));
                    let _e1130 = select(_e1128, 1f, (_e1128 > 1f));
                    let _e1139 = ((_e1084 - 3.2f) * -0.3125f);
                    let _e1141 = select(_e1139, 0f, (_e1139 < 0f));
                    let _e1143 = select(_e1141, 1f, (_e1141 > 1f));
                    let _e1150 = ((((_e1130 * _e1130) * (3f - (2f * _e1130))) * (0.58f + (_e1075 * 0.35f))) + ((((_e1143 * _e1143) * (3f - (2f * _e1143))) * _e1075) * 0.12f));
                    let _e1163 = (_e1116 + ((_e1097 * (1f - _e1116)) * 0.22f));
                    phi_66_ = _e392;
                    phi_67_ = vec3<f32>((_e1045 + ((_e1123 * _e1150) + (((_e1123 * _e1074) + _e1054) * _e1163))), (_e1046 + ((_e1124 * _e1150) + (((_e1124 * _e1074) + (0.24f * _e1054)) * _e1163))), (_e1047 + (_e1150 + ((_e1074 + (0.3f * _e1054)) * _e1163))));
                    phi_68_ = false;
                    break;
                }
                case 4: {
                    phi_66_ = _e392;
                    phi_67_ = vec3<f32>();
                    phi_68_ = true;
                    break;
                }
                case 5: {
                    phi_66_ = _e392;
                    phi_67_ = vec3<f32>();
                    phi_68_ = true;
                    break;
                }
                default: {
                    phi_66_ = _e392;
                    phi_67_ = vec3<f32>();
                    phi_68_ = bool();
                    break;
                }
            }
            let _e2264 = phi_66_;
            let _e2266 = phi_67_;
            let _e2268 = phi_68_;
            if _e2264 {
                break;
            }
            if _e2268 {
                let _e2270 = select(1f, 0f, (_e711 == 5u));
                let _e2274 = pill_1.member[_e231].power_hover;
                let _e2279 = ((abs((f32(_e2274) - _e2270)) - 0.4f) * -2.857143f);
                let _e2281 = select(_e2279, 0f, (_e2279 < 0f));
                let _e2283 = select(_e2281, 1f, (_e2281 > 1f));
                let _e2287 = ((_e2283 * _e2283) * (3f - (2f * _e2283)));
                let _e2289 = (1f + (_e2287 * 0.07f));
                let _e2290 = (_e736 / _e2289);
                let _e2291 = (_e737 / _e2289);
                let _e2295 = pill_1.member[_e231].power_action;
                let _e2300 = ((abs((f32(_e2295) - _e2270)) - 0.4f) * -2.857143f);
                let _e2302 = select(_e2300, 0f, (_e2300 < 0f));
                let _e2304 = select(_e2302, 1f, (_e2302 > 1f));
                let _e2308 = ((_e2304 * _e2304) * (3f - (2f * _e2304)));
                let _e2312 = pill_1.member[_e231].power_progress;
                let _e2313 = (_e2312 * _e2308);
                if (_e2270 < 0.5f) {
                    let _e2437 = select(_e2313, 0f, (_e2313 < 0f));
                    let _e2439 = select(_e2437, 1f, (_e2437 > 1f));
                    let _e2443 = ((_e2439 * _e2439) * (3f - (2f * _e2439)));
                    let _e2449 = (1f - _e2313);
                    let _e2458 = (_e2443 * 0.7f);
                    let _e2459 = (_e2458 + 1.5999999f);
                    let _e2464 = ((abs((sqrt(((_e2290 * _e2290) + (_e2291 * _e2291))) - ((7.5f - (_e2313 * 4.6f)) + (((sin((_e510 * 8f)) * _e2313) * _e2449) * 0.16f)))) - _e2459) / ((_e2458 + 0.49999994f) - _e2459));
                    let _e2466 = select(_e2464, 0f, (_e2464 < 0f));
                    let _e2468 = select(_e2466, 1f, (_e2466 > 1f));
                    let _e2477 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2290, (_e2291 - -7f)), vec2<f32>((3f * _e2449), 3f), 0.5f);
                    let _e2479 = ((_e2477 - 0.55f) * -0.9090909f);
                    let _e2481 = select(_e2479, 0f, (_e2479 < 0f));
                    let _e2483 = select(_e2481, 1f, (_e2481 > 1f));
                    let _e2497 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2290, (_e2291 - (-5f + (_e2313 * 3.5f)))), vec2<f32>((1.05f + (_e2443 * 0.45f)), (4.6f - (_e2313 * 3f))), 0.7f);
                    let _e2499 = ((_e2497 - 0.55f) * -0.9090909f);
                    let _e2501 = select(_e2499, 0f, (_e2499 < 0f));
                    let _e2503 = select(_e2501, 1f, (_e2501 > 1f));
                    let _e2507 = ((_e2503 * _e2503) * (3f - (2f * _e2503)));
                    let _e2509 = (((_e2468 * _e2468) * (3f - (2f * _e2468))) * (1f - ((_e2483 * _e2483) * (3f - (2f * _e2483)))));
                    if (_e2509 != _e2509) {
                        phi_72_ = true;
                    } else {
                        phi_72_ = (_e2507 >= _e2509);
                    }
                    let _e2513 = phi_72_;
                    phi_73_ = select(_e2509, _e2507, _e2513);
                } else {
                    let _e2316 = ((1f - _e2308) + _e2313);
                    let _e2320 = (((atan2(_e2291, _e2290) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e2324 = ((_e2316 * 0.82f) - 0.045f);
                    if (_e2324 != _e2324) {
                        phi_69_ = true;
                    } else {
                        phi_69_ = (0f >= _e2324);
                    }
                    let _e2328 = phi_69_;
                    let _e2329 = select(_e2324, 0f, _e2328);
                    let _e2337 = ((abs((sqrt(((_e2290 * _e2290) + (_e2291 * _e2291))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e2339 = select(_e2337, 0f, (_e2337 < 0f));
                    let _e2341 = select(_e2339, 1f, (_e2339 > 1f));
                    let _e2346 = (_e2329 + 0.008f);
                    let _e2350 = (((_e2320 - trunc(_e2320)) - _e2346) / ((_e2329 - 0.008f) - _e2346));
                    let _e2352 = select(_e2350, 0f, (_e2350 < 0f));
                    let _e2354 = select(_e2352, 1f, (_e2352 > 1f));
                    let _e2360 = (_e2316 * 50f);
                    let _e2362 = select(_e2360, 0f, (_e2360 < 0f));
                    let _e2364 = select(_e2362, 1f, (_e2362 > 1f));
                    let _e2369 = ((((_e2341 * _e2341) * (3f - (2f * _e2341))) * ((_e2354 * _e2354) * (3f - (2f * _e2354)))) * ((_e2364 * _e2364) * (3f - (2f * _e2364))));
                    let _e2371 = (0.50265485f + (5.152212f * _e2316));
                    let _e2372 = cos(_e2371);
                    let _e2373 = sin(_e2371);
                    let _e2377 = (_e2290 - (_e2372 * 7.1f));
                    let _e2378 = (_e2291 - (_e2373 * 7.1f));
                    let _e2381 = ((_e2377 * -(_e2373)) + (_e2378 * _e2372));
                    let _e2384 = ((_e2377 * _e2372) + (_e2378 * _e2373));
                    let _e2385 = (_e2381 * -3.2f);
                    let _e2388 = ((_e2385 + (_e2384 * 2.1f)) * 0.06825939f);
                    let _e2390 = select(_e2388, 0f, (_e2388 < 0f));
                    let _e2392 = select(_e2390, 1f, (_e2390 > 1f));
                    let _e2395 = (_e2381 - (-3.2f * _e2392));
                    let _e2396 = (_e2384 - (2.1f * _e2392));
                    let _e2400 = sqrt(((_e2395 * _e2395) + (_e2396 * _e2396)));
                    let _e2403 = ((_e2385 + (_e2384 * -2.1f)) * 0.06825939f);
                    let _e2405 = select(_e2403, 0f, (_e2403 < 0f));
                    let _e2407 = select(_e2405, 1f, (_e2405 > 1f));
                    let _e2410 = (_e2381 - (-3.2f * _e2407));
                    let _e2411 = (_e2384 - (-2.1f * _e2407));
                    let _e2415 = sqrt(((_e2410 * _e2410) + (_e2411 * _e2411)));
                    if (_e2400 != _e2400) {
                        phi_70_ = true;
                    } else {
                        phi_70_ = (_e2415 <= _e2400);
                    }
                    let _e2419 = phi_70_;
                    let _e2422 = ((select(_e2400, _e2415, _e2419) - 1.7f) * -0.71428573f);
                    let _e2424 = select(_e2422, 0f, (_e2422 < 0f));
                    let _e2426 = select(_e2424, 1f, (_e2424 > 1f));
                    let _e2430 = ((_e2426 * _e2426) * (3f - (2f * _e2426)));
                    if (_e2369 != _e2369) {
                        phi_71_ = true;
                    } else {
                        phi_71_ = (_e2430 >= _e2369);
                    }
                    let _e2434 = phi_71_;
                    phi_73_ = select(_e2369, _e2430, _e2434);
                }
                let _e2516 = phi_73_;
                let _e2519 = (_e2308 * (0.5f + (_e2313 * 0.5f)));
                if (_e2287 != _e2287) {
                    phi_74_ = true;
                } else {
                    phi_74_ = (_e2519 >= _e2287);
                }
                let _e2523 = phi_74_;
                let _e2524 = select(_e2287, _e2519, _e2523);
                let _e2526 = (0.48f * (1f - _e2524));
                let _e2537 = (1f + (_e2313 * 0.45f));
                phi_75_ = vec3<f32>((_e1045 + (((_e2526 + (0.78f * _e2524)) * _e2516) * _e2537)), (_e1046 + (((_e2526 + (0.3f * _e2524)) * _e2516) * _e2537)), (_e1047 + (((_e2526 + (0.28f * _e2524)) * _e2516) * _e2537)));
            } else {
                phi_75_ = _e2266;
            }
            let _e2546 = phi_75_;
            let _e2548 = select(1u, 0u, (_e711 == 0u));
            switch bitcast<i32>(_e711) {
                case 0: {
                    phi_76_ = true;
                    break;
                }
                case 1: {
                    phi_76_ = true;
                    break;
                }
                default: {
                    phi_76_ = false;
                    break;
                }
            }
            let _e2551 = phi_76_;
            if _e2551 {
                if (_e2548 < 2u) {
                } else {
                    break;
                }
                let _e2560 = pill_1.member[_e231].text.lines[_e2548].min[0u];
                let _e2568 = pill_1.member[_e231].text.lines[_e2548].min[1u];
                let _e2576 = pill_1.member[_e231].text.lines[_e2548].max[0u];
                let _e2584 = pill_1.member[_e231].text.lines[_e2548].max[1u];
                let _e2592 = pill_1.member[_e231].text.lines[_e2548].origin[0u];
                let _e2600 = pill_1.member[_e231].text.lines[_e2548].origin[1u];
                let _e2607 = pill_1.member[_e231].text.lines[_e2548].size;
                let _e2614 = pill_1.member[_e231].text.lines[_e2548].weight;
                let _e2621 = pill_1.member[_e231].text.lines[_e2548].count;
                let _e2628 = pill_1.member[_e231].text.lines[_e2548].first;
                if (_e259 < _e2560) {
                    phi_115_ = f32();
                    phi_116_ = true;
                } else {
                    if (_e259 > _e2576) {
                        phi_113_ = f32();
                        phi_114_ = true;
                    } else {
                        if (_e260 < _e2568) {
                            phi_111_ = f32();
                            phi_112_ = true;
                        } else {
                            let _e2632 = (_e260 > _e2584);
                            if _e2632 {
                                phi_110_ = f32();
                            } else {
                                phi_77_ = _e2621;
                                phi_78_ = 0u;
                                loop {
                                    let _e2634 = phi_77_;
                                    let _e2636 = phi_78_;
                                    local_32 = _e2636;
                                    let _e2637 = (_e2636 < _e2634);
                                    if _e2637 {
                                        let _e2640 = (_e2636 + ((_e2634 - _e2636) / 2u));
                                        let _e2641 = (_e2628 + _e2640);
                                        if (_e2641 < 32u) {
                                        } else {
                                            phi_82_ = true;
                                            break;
                                        }
                                        let _e2649 = pill_1.member[_e231].text.glyphs[_e2641].x;
                                        let _e2652 = (_e2649 <= ((_e259 - _e2592) / _e2607));
                                        if _e2652 {
                                            phi_79_ = (_e2640 + 1u);
                                        } else {
                                            phi_79_ = _e2636;
                                        }
                                        let _e2655 = phi_79_;
                                        phi_80_ = select(_e2640, _e2634, _e2652);
                                        phi_81_ = _e2655;
                                    } else {
                                        phi_80_ = u32();
                                        phi_81_ = u32();
                                    }
                                    let _e2658 = phi_80_;
                                    let _e2660 = phi_81_;
                                    continue;
                                    continuing {
                                        phi_77_ = _e2658;
                                        phi_78_ = _e2660;
                                        phi_82_ = _e2264;
                                        break if !(_e2637);
                                    }
                                }
                                let _e2663 = phi_82_;
                                if _e2663 {
                                    break;
                                }
                                let _e2665 = local_32;
                                let _e2666 = (_e2665 + 1u);
                                phi_83_ = _e2663;
                                phi_84_ = select(_e2666, _e2621, (_e2621 < _e2666));
                                phi_85_ = -1000000f;
                                loop {
                                    let _e2670 = phi_83_;
                                    let _e2672 = phi_84_;
                                    let _e2674 = phi_85_;
                                    local_37 = _e2674;
                                    if (_e2672 > 0u) {
                                        let _e2676 = (_e2672 - 1u);
                                        let _e2677 = (_e2628 + _e2676);
                                        if (_e2677 < 32u) {
                                        } else {
                                            phi_109_ = true;
                                            break;
                                        }
                                        let _e2685 = pill_1.member[_e231].text.glyphs[_e2677].x;
                                        let _e2692 = pill_1.member[_e231].text.glyphs[_e2677].glyph;
                                        if (_e2692 < arrayLength((&glyphs.member))) {
                                        } else {
                                            phi_109_ = true;
                                            break;
                                        }
                                        let _e2698 = glyphs.member[_e2692].min[0u];
                                        let _e2703 = glyphs.member[_e2692].min[1u];
                                        let _e2708 = glyphs.member[_e2692].max[0u];
                                        let _e2713 = glyphs.member[_e2692].max[1u];
                                        let _e2717 = glyphs.member[_e2692].start;
                                        let _e2721 = glyphs.member[_e2692].count;
                                        let _e2724 = (((_e259 - _e2592) / _e2607) - _e2685);
                                        let _e2727 = (-((_e260 - _e2600)) / _e2607);
                                        let _e2728 = (3.5f / _e2607);
                                        let _e2729 = (_e2708 + _e2728);
                                        let _e2730 = (_e2724 > _e2729);
                                        if _e2730 {
                                            phi_103_ = _e2670;
                                            phi_104_ = f32();
                                        } else {
                                            if (_e2724 >= (_e2698 - _e2728)) {
                                                if (_e2727 >= (_e2703 - _e2728)) {
                                                    if (_e2724 <= _e2729) {
                                                        if (_e2727 <= (_e2713 + _e2728)) {
                                                            phi_86_ = 0u;
                                                            phi_87_ = 0i;
                                                            phi_88_ = 340282350000000000000000000000000000000f;
                                                            loop {
                                                                let _e2739 = phi_86_;
                                                                let _e2741 = phi_87_;
                                                                let _e2743 = phi_88_;
                                                                local_33 = _e2743;
                                                                local_34 = _e2741;
                                                                let _e2744 = (_e2739 < _e2721);
                                                                if _e2744 {
                                                                    let _e2745 = (_e2717 + _e2739);
                                                                    if (_e2745 < arrayLength((&edges.member))) {
                                                                    } else {
                                                                        phi_93_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2749 = edges.member[_e2745];
                                                                    let _e2751 = cantus_render_text_edge_distance(_e2749, _e2614, vec2<f32>(_e2724, _e2727));
                                                                    if (_e2743 != _e2743) {
                                                                        phi_89_ = true;
                                                                    } else {
                                                                        phi_89_ = (_e2751.member <= _e2743);
                                                                    }
                                                                    let _e2757 = phi_89_;
                                                                    phi_90_ = (_e2739 + 1u);
                                                                    phi_91_ = (_e2741 + _e2751.member_1);
                                                                    phi_92_ = select(_e2743, _e2751.member, _e2757);
                                                                } else {
                                                                    phi_90_ = u32();
                                                                    phi_91_ = i32();
                                                                    phi_92_ = f32();
                                                                }
                                                                let _e2762 = phi_90_;
                                                                let _e2764 = phi_91_;
                                                                let _e2766 = phi_92_;
                                                                continue;
                                                                continuing {
                                                                    phi_86_ = _e2762;
                                                                    phi_87_ = _e2764;
                                                                    phi_88_ = _e2766;
                                                                    phi_93_ = _e2670;
                                                                    break if !(_e2744);
                                                                }
                                                            }
                                                            let _e2769 = phi_93_;
                                                            phi_109_ = _e2769;
                                                            if _e2769 {
                                                                break;
                                                            }
                                                            let _e2771 = local_33;
                                                            let _e2775 = local_34;
                                                            let _e2778 = ((sqrt(_e2771) * _e2607) * select(1f, -1f, (_e2775 == 0i)));
                                                            if (_e2674 != _e2674) {
                                                                phi_94_ = true;
                                                            } else {
                                                                phi_94_ = (_e2778 >= _e2674);
                                                            }
                                                            let _e2782 = phi_94_;
                                                            phi_95_ = _e2769;
                                                            phi_96_ = select(_e2674, _e2778, _e2782);
                                                        } else {
                                                            phi_95_ = _e2670;
                                                            phi_96_ = _e2674;
                                                        }
                                                        let _e2785 = phi_95_;
                                                        let _e2787 = phi_96_;
                                                        phi_97_ = _e2785;
                                                        phi_98_ = _e2787;
                                                    } else {
                                                        phi_97_ = _e2670;
                                                        phi_98_ = _e2674;
                                                    }
                                                    let _e2789 = phi_97_;
                                                    let _e2791 = phi_98_;
                                                    phi_99_ = _e2789;
                                                    phi_100_ = _e2791;
                                                } else {
                                                    phi_99_ = _e2670;
                                                    phi_100_ = _e2674;
                                                }
                                                let _e2793 = phi_99_;
                                                let _e2795 = phi_100_;
                                                phi_101_ = _e2793;
                                                phi_102_ = _e2795;
                                            } else {
                                                phi_101_ = _e2670;
                                                phi_102_ = _e2674;
                                            }
                                            let _e2797 = phi_101_;
                                            let _e2799 = phi_102_;
                                            phi_103_ = _e2797;
                                            phi_104_ = _e2799;
                                        }
                                        let _e2801 = phi_103_;
                                        let _e2803 = phi_104_;
                                        phi_105_ = _e2801;
                                        phi_106_ = _e2676;
                                        phi_107_ = _e2803;
                                        phi_108_ = select(true, false, _e2730);
                                    } else {
                                        phi_105_ = _e2670;
                                        phi_106_ = u32();
                                        phi_107_ = f32();
                                        phi_108_ = false;
                                    }
                                    let _e2806 = phi_105_;
                                    let _e2808 = phi_106_;
                                    let _e2810 = phi_107_;
                                    let _e2812 = phi_108_;
                                    continue;
                                    continuing {
                                        phi_83_ = _e2806;
                                        phi_84_ = _e2808;
                                        phi_85_ = _e2810;
                                        phi_109_ = _e2806;
                                        break if !(_e2812);
                                    }
                                }
                                let _e2815 = phi_109_;
                                if _e2815 {
                                    break;
                                }
                                let _e2990 = local_37;
                                phi_110_ = _e2990;
                            }
                            let _e2817 = phi_110_;
                            phi_111_ = _e2817;
                            phi_112_ = _e2632;
                        }
                        let _e2819 = phi_111_;
                        let _e2821 = phi_112_;
                        phi_113_ = _e2819;
                        phi_114_ = _e2821;
                    }
                    let _e2823 = phi_113_;
                    let _e2825 = phi_114_;
                    phi_115_ = _e2823;
                    phi_116_ = _e2825;
                }
                let _e2827 = phi_115_;
                let _e2829 = phi_116_;
                phi_117_ = select(_e2827, -1000000f, _e2829);
            } else {
                phi_117_ = -1000000f;
            }
            let _e2832 = phi_117_;
            let _e2834 = ((_e2832 * 1.25f) + 0.5f);
            let _e2836 = select(_e2834, 0f, (_e2834 < 0f));
            let _e2838 = select(_e2836, 1f, (_e2836 > 1f));
            let _e2842 = ((_e2838 * _e2838) * (3f - (2f * _e2838)));
            let _e2843 = (1f - _e2842);
            let _e2850 = (0.94f * _e2842);
            let _e2855 = local_35;
            let _e2857 = (1f - (_e2855 * 0.35f));
            let _e2862 = local_36;
            let _e2863 = (_e2862 * 0.33249998f);
            out_color = vec4<f32>((((((_e2546.x * _e2843) + _e2850) * _e2857) + _e2863) * _e460), (((((_e2546.y * _e2843) + _e2850) * _e2857) + _e2863) * _e460), (((((_e2546.z * _e2843) + _e2850) * _e2857) + _e2863) * _e460), _e473);
            break;
        }
    }
    return;
}

fn render_playhead_vertex_impl() {
    let _e13 = vertex_5;
    let _e14 = _isthmus_instance_index_7;
    let _e23 = frame.member[0u].playhead_x;
    let _e29 = frame.member[0u].panel_height;
    let _e32 = (_e23 + ((((f32((_e13 & 1u)) * 2f) - 1f) * _e29) * 0.4f));
    let _e35 = (1f + (f32((_e13 >> bitcast<u32>(1i))) * (_e29 + 10f)));
    let _e40 = frame.member[0u].screen_size[0u];
    let _e45 = frame.member[0u].screen_size[1u];
    let _e48 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e32, _e35), vec2<f32>(_e40, _e45));
    out_position = _e48;
    out_world_pos[0u] = _e32;
    out_world_pos[1u] = _e35;
    out_isthmus_instance_index = _e14;
    return;
}

fn render_playhead_fragment_impl() {
    var phi_0_: bool;
    var phi_1_: bool;
    var phi_2_: bool;
    var phi_3_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e31 = world_pos_1;
            let _e32 = _isthmus_instance_index_8;
            let _e38 = frame.member[0u].playhead_x;
            let _e42 = frame.member[0u].panel_height;
            let _e45 = (_e31.x - _e38);
            let _e46 = (_e31.y - (6f + (_e42 * 0.5f)));
            let _e47 = abs(_e45);
            let _e48 = abs(_e46);
            let _e52 = state.member[_e32].bar_split;
            let _e55 = (_e42 * (0.5f - (0.375f * _e52)));
            let _e61 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e48 - ((_e42 - _e55) * 0.5f)), _e47), (_e55 * 0.5f), 4.5f);
            let _e64 = abs((_e47 - (4f * _e52)));
            let _e66 = (_e48 - (_e42 * 0.1f));
            if (_e66 != _e66) {
                phi_0_ = true;
            } else {
                phi_0_ = (0f >= _e66);
            }
            let _e70 = phi_0_;
            let _e71 = select(_e66, 0f, _e70);
            let _e76 = (sqrt(((_e64 * _e64) + (_e71 * _e71))) - 3.5f);
            let _e81 = state.member[_e32].icon_morph;
            let _e85 = state.member[_e32].icon_presence;
            let _e89 = ((_e42 * 0.18f) * (1f + (_e81 * (1f - _e85))));
            let _e91 = (_e89 * 0.5f);
            let _e92 = abs(-(_e46));
            let _e94 = (_e92 + (1.7320508f * _e45));
            if (_e94 != _e94) {
                phi_1_ = true;
            } else {
                phi_1_ = (0f >= _e94);
            }
            let _e98 = phi_1_;
            let _e99 = select(_e94, 0f, _e98);
            let _e102 = (_e92 - (0.5f * _e99));
            let _e104 = (_e89 - _e91);
            let _e106 = (_e104 * -0.8660254f);
            let _e107 = (_e104 * 0.8660254f);
            if (_e106 <= _e107) {
            } else {
                break;
            }
            let _e110 = select(_e102, _e106, (_e102 < _e106));
            let _e113 = (_e102 - select(_e110, _e107, (_e110 > _e107)));
            let _e114 = ((_e45 - (_e99 * 0.8660254f)) - (-0.5f * _e104));
            let _e125 = (_e76 + ((((sqrt(((_e113 * _e113) + (_e114 * _e114))) * select(1f, -1f, (_e114 > 0f))) - _e91) - _e76) * _e81));
            let _e126 = (_e61 - -0.8f);
            let _e128 = select(_e126, 0f, (_e126 < 0f));
            let _e130 = select(_e128, 1f, (_e128 > 1f));
            let _e135 = (1f - ((_e130 * _e130) * (3f - (2f * _e130))));
            let _e136 = (_e125 - -0.8f);
            let _e138 = select(_e136, 0f, (_e136 < 0f));
            let _e140 = select(_e138, 1f, (_e138 > 1f));
            let _e146 = ((1f - ((_e140 * _e140) * (3f - (2f * _e140)))) * _e85);
            if (_e146 != _e146) {
                phi_2_ = true;
            } else {
                phi_2_ = (_e135 >= _e146);
            }
            let _e150 = phi_2_;
            let _e151 = select(_e146, _e135, _e150);
            if (_e151 <= 0f) {
                discard;
            }
            if (_e61 != _e61) {
                phi_3_ = true;
            } else {
                phi_3_ = (_e125 <= _e61);
            }
            let _e156 = phi_3_;
            let _e159 = ((select(_e61, _e125, _e156) - -2.5f) * 0.6666667f);
            let _e161 = select(_e159, 0f, (_e159 < 0f));
            let _e163 = select(_e161, 1f, (_e161 > 1f));
            let _e167 = ((_e163 * _e163) * (3f - (2f * _e163)));
            let _e168 = (1f - _e167);
            let _e171 = (0.15f * _e167);
            out_color = vec4<f32>((_e168 + _e171), ((0.878f * _e168) + _e171), ((0.824f * _e168) + _e171), _e151);
            break;
        }
    }
    return;
}

fn render_particles_vertex_impl() {
    var phi_0_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_1_: isthmus_Vertex_render_particles_Varyings;
    var phi_2_: isthmus_Vertex_render_particles_Varyings;
    var phi_3_: bool;
    var phi_4_: isthmus_Vertex_render_particles_Varyings;

    let _e30 = vertex_5;
    let _e31 = _isthmus_instance_index_7;
    let _e35 = frame.member[0u].time;
    let _e39 = particle.member[_e31].end_time;
    let _e43 = particle.member[_e31].duration;
    let _e45 = (_e35 - (_e39 - _e43));
    if (_e45 < 0f) {
        phi_2_ = isthmus_Vertex_render_particles_Varyings();
        phi_3_ = true;
    } else {
        let _e47 = (_e45 > _e43);
        if _e47 {
            phi_1_ = isthmus_Vertex_render_particles_Varyings();
        } else {
            let _e48 = (_e45 / _e43);
            let _e53 = particle.member[_e31].spawn_vel[0u];
            let _e58 = particle.member[_e31].spawn_vel[1u];
            let _e62 = sqrt(((_e53 * _e53) + (_e58 * _e58)));
            if (_e62 > 0.001f) {
                phi_0_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e53 / _e62), (_e58 / _e62)), _e62);
            } else {
                phi_0_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e62);
            }
            let _e70 = phi_0_;
            let _e82 = ((f32((_e30 & 1u)) * 2f) - 1f);
            let _e83 = ((f32((_e30 >> bitcast<u32>(1i))) * 2f) - 1f);
            let _e86 = (_e48 + 0.5f);
            let _e87 = ((_e82 * 5f) * _e86);
            let _e88 = ((_e83 * 2.5f) * _e86);
            let _e93 = particle.member[_e31].spawn_pos[0u];
            let _e98 = particle.member[_e31].spawn_pos[1u];
            let _e115 = particle.member[_e31].rgb;
            let _e116 = unpack4x8unorm(_e115);
            let _e125 = ((((_e116.x * 0.299f) + (_e116.y * 0.587f)) + (_e116.z * 0.114f)) * -1f);
            let _e145 = frame.member[0u].screen_size[0u];
            let _e150 = frame.member[0u].screen_size[1u];
            let _e153 = cantus_render_shader_pixel_to_ndc(vec2<f32>((((_e93 + (_e53 * _e45)) + (_e70.unnamed.x * _e87)) + (-(_e70.unnamed.y) * _e88)), (((_e98 + (_e58 * _e45)) + (_e70.unnamed.y * _e87)) + (_e70.unnamed.x * _e88))), vec2<f32>(_e145, _e150));
            let _e155 = (_e45 * 6.6666665f);
            let _e157 = select(_e155, 0f, (_e155 < 0f));
            let _e159 = select(_e157, 1f, (_e157 > 1f));
            phi_1_ = isthmus_Vertex_render_particles_Varyings(u0028_isthmus_glam_Vec4_u0020_isthmus_glam_Vec2_u0029_(vec4<f32>(((((_e125 + (_e116.x * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.y * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.z * 2f)) * 0.8f) + 0.2f) * 2f), (((1f - _e48) * ((_e159 * _e159) * (3f - (2f * _e159)))) * 0.3f)), vec2<f32>(_e82, _e83)), _e153);
        }
        let _e171 = phi_1_;
        phi_2_ = _e171;
        phi_3_ = _e47;
    }
    let _e173 = phi_2_;
    let _e175 = phi_3_;
    if _e175 {
        phi_4_ = isthmus_Vertex_render_particles_Varyings(u0028_isthmus_glam_Vec4_u0020_isthmus_glam_Vec2_u0029_(vec4<f32>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f)), vec4<f32>(0f, 0f, 0f, 0f));
    } else {
        phi_4_ = _e173;
    }
    let _e177 = phi_4_;
    out_position = _e177.position;
    out_color = _e177.varyings.unnamed;
    out_uv[0u] = _e177.varyings.unnamed_1.x;
    out_uv[1u] = _e177.varyings.unnamed_1.y;
    return;
}

fn render_particles_fragment_impl() {
    let _e9 = color_1;
    let _e10 = uv_1;
    let _e14 = (_e10.x * 0.8f);
    let _e20 = ((sqrt(((_e14 * _e14) + (_e10.y * _e10.y))) - 1f) * -1.25f);
    let _e22 = select(_e20, 0f, (_e20 < 0f));
    let _e24 = select(_e22, 1f, (_e22 > 1f));
    let _e29 = (_e9.w * ((_e24 * _e24) * (3f - (2f * _e24))));
    if (_e29 <= 0f) {
        discard;
    }
    out_color = vec4<f32>((_e9.x * _e29), (_e9.y * _e29), (_e9.z * _e29), _e29);
    return;
}

fn render_tempestas_vertex_impl() {
    var phi_0_: array<f32, 2>;
    var phi_1_: array<f32, 2>;
    var phi_2_: bool;
    var phi_3_: f32;
    var phi_4_: array<f32, 2>;
    var phi_5_: array<f32, 2>;
    var phi_6_: array<f32, 2>;
    var phi_7_: bool;
    var phi_8_: f32;
    var phi_9_: array<f32, 2>;
    var phi_10_: u32;
    var phi_11_: f32;
    var phi_12_: u32;
    var phi_13_: f32;
    var phi_14_: bool;
    var local_38: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e36 = vertex_5;
            let _e37 = _isthmus_instance_index_7;
            let _e41 = pill_2.member[_e37].calendar_expansion;
            let _e43 = select(_e41, 0f, (_e41 < 0f));
            let _e45 = select(_e43, 1f, (_e43 > 1f));
            let _e49 = ((_e45 * _e45) * (3f - (2f * _e45)));
            let _e53 = frame.member[0u].weather_hour;
            let _e57 = pill_2.member[_e37].sun_hours;
            let _e60 = (_e57[1] - _e57[0]);
            if (_e53 >= _e57[0]) {
                let _e62 = (_e53 <= _e57[1]);
                if _e62 {
                    let _e64 = ((_e53 - _e57[0]) / _e60);
                    phi_0_ = array<f32, 2>(_e64, sin((_e64 * 3.1415927f)));
                } else {
                    phi_0_ = array<f32, 2>();
                }
                let _e69 = phi_0_;
                phi_1_ = _e69;
                phi_2_ = select(true, false, _e62);
            } else {
                phi_1_ = array<f32, 2>();
                phi_2_ = true;
            }
            let _e72 = phi_1_;
            let _e74 = phi_2_;
            if _e74 {
                let _e75 = (24f - _e60);
                if (_e53 < _e57[0]) {
                    phi_3_ = (((_e53 + 24f) - _e57[1]) / _e75);
                } else {
                    phi_3_ = ((_e53 - _e57[1]) / _e75);
                }
                let _e83 = phi_3_;
                phi_4_ = array<f32, 2>(select(0f, 1f, (_e53 >= _e57[1])), -(sin((_e83 * 3.1415927f))));
            } else {
                phi_4_ = _e72;
            }
            let _e91 = phi_4_;
            if (12f >= _e57[0]) {
                let _e95 = (12f <= _e57[1]);
                if _e95 {
                    let _e97 = ((12f - _e57[0]) / _e60);
                    phi_5_ = array<f32, 2>(_e97, sin((_e97 * 3.1415927f)));
                } else {
                    phi_5_ = array<f32, 2>();
                }
                let _e102 = phi_5_;
                phi_6_ = _e102;
                phi_7_ = select(true, false, _e95);
            } else {
                phi_6_ = array<f32, 2>();
                phi_7_ = true;
            }
            let _e105 = phi_6_;
            let _e107 = phi_7_;
            if _e107 {
                let _e108 = (24f - _e60);
                if (12f < _e57[0]) {
                    phi_8_ = ((36f - _e57[1]) / _e108);
                } else {
                    phi_8_ = ((12f - _e57[1]) / _e108);
                }
                let _e115 = phi_8_;
                phi_9_ = array<f32, 2>(select(0f, 1f, (12f >= _e57[1])), -(sin((_e115 * 3.1415927f))));
            } else {
                phi_9_ = _e105;
            }
            let _e123 = phi_9_;
            let _e129 = pill_2.member[_e37].x;
            let _e138 = frame.member[0u].mouse_pressure;
            phi_10_ = 0u;
            phi_11_ = (_e138 * 8f);
            loop {
                let _e141 = phi_10_;
                let _e143 = phi_11_;
                local_38 = _e143;
                let _e144 = (_e141 < 4u);
                if _e144 {
                    if _e144 {
                    } else {
                        phi_14_ = true;
                        break;
                    }
                    let _e150 = frame.member[0u].ripples[_e141].start_time;
                    let _e156 = frame.member[0u].ripples[_e141].strength;
                    let _e160 = frame.member[0u].time;
                    let _e162 = ((_e160 - _e150) * 1.2f);
                    let _e164 = select(_e162, 0f, (_e162 < 0f));
                    let _e167 = (1f - select(_e164, 1f, (_e164 > 1f)));
                    phi_12_ = (_e141 + 1u);
                    phi_13_ = (_e143 + (((_e156 * _e167) * _e167) * 11f));
                } else {
                    phi_12_ = u32();
                    phi_13_ = f32();
                }
                let _e174 = phi_12_;
                let _e176 = phi_13_;
                continue;
                continuing {
                    phi_10_ = _e174;
                    phi_11_ = _e176;
                    phi_14_ = false;
                    break if !(_e144);
                }
            }
            let _e179 = phi_14_;
            if _e179 {
                break;
            }
            let _e181 = local_38;
            let _e182 = (_e181 * 0.5f);
            let _e183 = (18f + _e182);
            let _e194 = frame.member[0u].panel_height;
            let _e202 = (((_e129 - (_e49 * 158f)) - _e183) + (f32((_e36 & 1u)) * ((308f + (316f * _e49)) + (_e183 * 2f))));
            let _e203 = ((-12f - _e182) + (f32((_e36 >> bitcast<u32>(1i))) * ((244f * _e49) + ((_e194 + _e183) * 2f))));
            let _e208 = frame.member[0u].screen_size[0u];
            let _e213 = frame.member[0u].screen_size[1u];
            let _e216 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e202, _e203), vec2<f32>(_e208, _e213));
            out_position = _e216;
            out_pixel[0u] = _e202;
            out_pixel[1u] = _e203;
            out_weather = vec4<f32>(_e91[0], _e91[1], _e123[1], _e49);
            out_isthmus_instance_index_1 = _e37;
            break;
        }
    }
    return;
}

fn render_tempestas_fragment_impl() {
    var phi_0_: u32;
    var phi_1_: u32;
    var phi_2_: bool;
    var phi_3_: bool;
    var phi_4_: bool;
    var phi_5_: bool;
    var phi_6_: vec2<f32>;
    var phi_7_: f32;
    var phi_8_: u32;
    var phi_9_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_10_: bool;
    var phi_11_: vec2<f32>;
    var phi_12_: f32;
    var phi_13_: u32;
    var phi_14_: bool;
    var phi_15_: f32;
    var local_39: vec2<f32>;
    var local_40: vec2<f32>;
    var phi_16_: bool;
    var phi_17_: f32;
    var phi_18_: f32;
    var phi_19_: f32;
    var phi_20_: bool;
    var phi_21_: bool;
    var phi_22_: bool;
    var phi_23_: bool;
    var phi_24_: bool;
    var phi_25_: bool;
    var phi_26_: vec2<f32>;
    var phi_27_: bool;
    var phi_28_: array<f32, 2>;
    var phi_29_: array<f32, 2>;
    var phi_30_: bool;
    var phi_31_: f32;
    var phi_32_: array<f32, 2>;
    var phi_33_: array<f32, 2>;
    var phi_34_: array<f32, 2>;
    var phi_35_: bool;
    var phi_36_: f32;
    var phi_37_: array<f32, 2>;
    var phi_38_: u0028_render_tempestas_WeatherCondition_u0020_f32_u0029_;
    var phi_39_: bool;
    var phi_40_: i32;
    var phi_41_: f32;
    var phi_42_: f32;
    var phi_43_: vec2<f32>;
    var phi_44_: i32;
    var phi_45_: f32;
    var phi_46_: f32;
    var phi_47_: vec2<f32>;
    var local_41: f32;
    var phi_48_: i32;
    var phi_49_: f32;
    var phi_50_: f32;
    var phi_51_: vec2<f32>;
    var phi_52_: i32;
    var phi_53_: f32;
    var phi_54_: f32;
    var phi_55_: vec2<f32>;
    var local_42: f32;
    var local_43: f32;
    var phi_56_: vec3<f32>;
    var phi_57_: vec3<f32>;
    var phi_58_: vec3<f32>;
    var phi_59_: vec3<f32>;
    var phi_60_: i32;
    var phi_61_: f32;
    var phi_62_: f32;
    var phi_63_: vec2<f32>;
    var phi_64_: i32;
    var phi_65_: f32;
    var phi_66_: f32;
    var phi_67_: vec2<f32>;
    var local_44: f32;
    var phi_68_: vec3<f32>;
    var phi_69_: i32;
    var phi_70_: f32;
    var phi_71_: f32;
    var phi_72_: vec2<f32>;
    var phi_73_: i32;
    var phi_74_: f32;
    var phi_75_: f32;
    var phi_76_: vec2<f32>;
    var local_45: f32;
    var phi_77_: f32;
    var phi_78_: vec3<f32>;
    var phi_79_: f32;
    var phi_80_: u32;
    var phi_81_: f32;
    var phi_82_: u32;
    var phi_83_: bool;
    var phi_84_: f32;
    var phi_85_: u32;
    var phi_86_: f32;
    var phi_87_: vec3<f32>;
    var phi_88_: u32;
    var phi_89_: f32;
    var phi_90_: u32;
    var phi_91_: f32;
    var phi_92_: vec3<f32>;
    var phi_93_: f32;
    var phi_94_: u32;
    var phi_95_: f32;
    var phi_96_: vec3<f32>;
    var phi_97_: u32;
    var phi_98_: f32;
    var phi_99_: vec3<f32>;
    var phi_100_: vec2<f32>;
    var phi_101_: u32;
    var phi_102_: f32;
    var phi_103_: vec3<f32>;
    var phi_104_: vec2<f32>;
    var phi_105_: bool;
    var phi_106_: u32;
    var phi_107_: u32;
    var phi_108_: u32;
    var phi_109_: u32;
    var phi_110_: u32;
    var phi_111_: bool;
    var local_46: u32;
    var phi_112_: bool;
    var phi_113_: u32;
    var phi_114_: f32;
    var phi_115_: u32;
    var phi_116_: i32;
    var phi_117_: f32;
    var phi_118_: bool;
    var phi_119_: u32;
    var phi_120_: i32;
    var phi_121_: f32;
    var phi_122_: bool;
    var local_47: f32;
    var local_48: i32;
    var phi_123_: bool;
    var phi_124_: bool;
    var phi_125_: f32;
    var phi_126_: bool;
    var phi_127_: f32;
    var phi_128_: bool;
    var phi_129_: f32;
    var phi_130_: bool;
    var phi_131_: f32;
    var phi_132_: bool;
    var phi_133_: f32;
    var phi_134_: bool;
    var phi_135_: u32;
    var phi_136_: f32;
    var phi_137_: bool;
    var phi_138_: bool;
    var phi_139_: f32;
    var phi_140_: f32;
    var phi_141_: bool;
    var phi_142_: f32;
    var phi_143_: bool;
    var phi_144_: f32;
    var phi_145_: bool;
    var local_49: f32;
    var local_50: f32;
    var local_51: f32;
    var local_52: f32;
    var local_53: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e218 = pixel_2;
            let _e219 = weather_1;
            let _e220 = _isthmus_instance_index_9;
            let _e233 = pill_2.member[_e220].x;
            let _e237 = frame.member[0u].panel_height;
            let _e238 = (_e218.x - _e233);
            let _e239 = (_e218.y - 6f);
            let _e240 = (_e237 * 0.5f);
            let _e244 = ((308f - _e237) * 0.5f);
            let _e246 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e238 - 154f), (_e239 - _e240)), _e244, _e240);
            let _e251 = frame.member[0u].mouse_pos[0u];
            let _e256 = frame.member[0u].mouse_pos[1u];
            let _e262 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e251 - _e233) - 154f), ((_e256 - 6f) - _e240)), _e244, _e240);
            phi_0_ = 0u;
            loop {
                let _e264 = phi_0_;
                let _e265 = (_e264 < 4u);
                if _e265 {
                    if _e265 {
                    } else {
                        phi_2_ = true;
                        break;
                    }
                    phi_1_ = (_e264 + 1u);
                } else {
                    phi_1_ = u32();
                }
                let _e268 = phi_1_;
                continue;
                continuing {
                    phi_0_ = _e268;
                    phi_2_ = false;
                    break if !(_e265);
                }
            }
            let _e271 = phi_2_;
            if _e271 {
                break;
            }
            let _e275 = frame.member[0u].mouse_pressure;
            let _e278 = (6f + _e237);
            let _e283 = (_e233 - (_e219.w * 158f));
            let _e285 = (_e218.y - _e278);
            let _e286 = (_e233 - 158f);
            let _e287 = (_e218.x - _e286);
            let _e288 = (8f * _e219.w);
            let _e289 = ((244f * _e219.w) - _e288);
            if (_e289 != _e289) {
                phi_3_ = true;
            } else {
                phi_3_ = (0f >= _e289);
            }
            let _e293 = phi_3_;
            let _e295 = ((308f + (316f * _e219.w)) * 0.5f);
            let _e296 = (select(_e289, 0f, _e293) * 0.5f);
            let _e297 = (_e288 + _e296);
            let _e300 = (_e296 != _e296);
            if _e300 {
                phi_4_ = true;
            } else {
                phi_4_ = (18f <= _e296);
            }
            let _e303 = phi_4_;
            let _e306 = vec2<f32>(_e295, _e296);
            let _e307 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e218.x - _e283) - _e295), (_e285 - _e297)), _e306, select(_e296, 18f, _e303));
            let _e309 = (_e256 - _e278);
            if _e300 {
                phi_5_ = true;
            } else {
                phi_5_ = (18f <= _e296);
            }
            let _e314 = phi_5_;
            let _e317 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e251 - _e283) - _e295), (_e309 - _e297)), _e306, select(_e296, 18f, _e314));
            let _e320 = (0.5f + ((_e307 - _e246) * 0.008928572f));
            let _e322 = select(_e320, 0f, (_e320 < 0f));
            let _e324 = select(_e322, 1f, (_e322 > 1f));
            let _e337 = (0.5f + ((_e317 - _e262) * 0.008928572f));
            let _e339 = select(_e337, 0f, (_e337 < 0f));
            let _e341 = select(_e339, 1f, (_e339 > 1f));
            phi_6_ = vec2<f32>(0f, 0f);
            phi_7_ = 0f;
            phi_8_ = 0u;
            loop {
                let _e353 = phi_6_;
                let _e355 = phi_7_;
                let _e357 = phi_8_;
                local_39 = _e353;
                local_40 = _e353;
                local_49 = _e355;
                local_50 = _e355;
                local_51 = _e355;
                local_52 = _e355;
                let _e358 = (_e357 < 4u);
                if _e358 {
                    if _e358 {
                    } else {
                        phi_14_ = true;
                        break;
                    }
                    let _e365 = frame.member[0u].ripples[_e357].origin[0u];
                    let _e372 = frame.member[0u].ripples[_e357].origin[1u];
                    let _e378 = frame.member[0u].ripples[_e357].start_time;
                    let _e384 = frame.member[0u].ripples[_e357].strength;
                    let _e388 = frame.member[0u].time;
                    let _e390 = ((_e388 - _e378) * 1.2f);
                    let _e392 = select(_e390, 0f, (_e390 < 0f));
                    let _e394 = select(_e392, 1f, (_e392 > 1f));
                    let _e395 = (_e218.x - _e365);
                    let _e396 = (_e218.y - _e372);
                    let _e400 = sqrt(((_e395 * _e395) + (_e396 * _e396)));
                    if (_e400 > 0.001f) {
                        phi_9_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e395 / _e400), (_e396 / _e400)), _e400);
                    } else {
                        phi_9_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e400);
                    }
                    let _e408 = phi_9_;
                    let _e418 = ((abs((_e408.unnamed_1 - (_e394 * 600f))) - 80f) * -0.0125f);
                    let _e420 = select(_e418, 0f, (_e418 < 0f));
                    let _e422 = select(_e420, 1f, (_e420 > 1f));
                    let _e428 = (1f - _e394);
                    let _e429 = ((((_e422 * _e422) * (3f - (2f * _e422))) * _e384) * _e428);
                    let _e442 = (_e355 + (_e429 * 0.5f));
                    if (_e442 != _e442) {
                        phi_10_ = true;
                    } else {
                        phi_10_ = (1f <= _e442);
                    }
                    let _e446 = phi_10_;
                    phi_11_ = vec2<f32>((_e353.x + (((_e408.unnamed.x * _e429) * _e428) * 0.5f)), (_e353.y + (((_e408.unnamed.y * _e429) * _e428) * 0.5f)));
                    phi_12_ = select(_e442, 1f, _e446);
                    phi_13_ = (_e357 + 1u);
                } else {
                    phi_11_ = vec2<f32>();
                    phi_12_ = f32();
                    phi_13_ = u32();
                }
                let _e450 = phi_11_;
                let _e452 = phi_12_;
                let _e454 = phi_13_;
                continue;
                continuing {
                    phi_6_ = _e450;
                    phi_7_ = _e452;
                    phi_8_ = _e454;
                    phi_14_ = _e271;
                    break if !(_e358);
                }
            }
            let _e457 = phi_14_;
            if _e457 {
                break;
            }
            if (_e275 > 0f) {
                let _e458 = (_e218.x - _e251);
                let _e459 = (_e218.y - _e256);
                let _e465 = ((sqrt(((_e458 * _e458) + (_e459 * _e459))) - 150f) * -0.006666667f);
                let _e467 = select(_e465, 0f, (_e465 < 0f));
                let _e469 = select(_e467, 1f, (_e467 > 1f));
                phi_15_ = ((((_e469 * _e469) * (3f - (2f * _e469))) * _e275) * 8f);
            } else {
                phi_15_ = 0f;
            }
            let _e477 = phi_15_;
            let _e479 = local_39;
            let _e482 = local_40;
            let _e485 = (((_e262 + ((((_e317 + ((_e262 - _e317) * _e341)) - ((56f * _e341) * (1f - _e341))) - _e262) * _e219.w)) - 0.5f) * -1f);
            let _e487 = select(_e485, 0f, (_e485 < 0f));
            let _e489 = select(_e487, 1f, (_e487 > 1f));
            let _e499 = (sqrt(((_e479.x * _e479.x) + (_e482.y * _e482.y))) * 22f);
            let _e502 = ((_e246 + ((((_e307 + ((_e246 - _e307) * _e324)) - ((56f * _e324) * (1f - _e324))) - _e246) * _e219.w)) - (((_e477 * ((_e489 * _e489) * (3f - (2f * _e489)))) + _e499) * 0.5f));
            let _e503 = (56f + _e240);
            let _e504 = (_e237 + 8f);
            let _e508 = (_e285 > ((_e503 + (_e503 + _e504)) * 0.5f));
            let _e513 = pill_2.member[_e220].calendar_expansion;
            let _e515 = (_e503 + (select(0f, 1f, _e508) * _e504));
            let _e516 = (_e515 * 0.0007377049f);
            let _e517 = (0.5f + _e516);
            let _e521 = ((_e513 - _e517) / ((_e516 + 0.74f) - _e517));
            let _e523 = select(_e521, 0f, (_e521 < 0f));
            let _e525 = select(_e523, 1f, (_e523 > 1f));
            let _e529 = ((_e525 * _e525) * (3f - (2f * _e525)));
            let _e531 = (292f * _e529);
            let _e532 = (_e237 * _e529);
            let _e537 = (324f + ((292f - _e531) * 0.5f));
            let _e538 = ((_e515 - _e240) + ((_e237 - _e532) * 0.5f));
            let _e539 = (_e287 - _e537);
            let _e540 = (_e285 - _e538);
            if (_e531 != _e531) {
                phi_16_ = true;
            } else {
                phi_16_ = (0.001f >= _e531);
            }
            let _e544 = phi_16_;
            let _e546 = (_e539 / select(_e531, 0.001f, _e544));
            if _e508 {
                let _e554 = ((_e546 * 5f) - 0.5f);
                let _e556 = select(_e554, 0f, (_e554 < 0f));
                phi_17_ = select(_e556, 4f, (_e556 > 4f));
            } else {
                let _e548 = ((_e546 * 6f) - 0.5f);
                let _e550 = select(_e548, 0f, (_e548 < 0f));
                phi_17_ = select(_e550, 5f, (_e550 > 5f));
            }
            let _e560 = phi_17_;
            let _e561 = (_e529 <= 0.001f);
            if _e561 {
                phi_18_ = 340282350000000000000000000000000000000f;
            } else {
                let _e563 = (_e532 * 0.5f);
                let _e569 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e539 - (_e529 * 146f)), (_e540 - _e563)), ((_e531 - _e532) * 0.5f), _e563);
                phi_18_ = _e569;
            }
            let _e571 = phi_18_;
            if _e561 {
                phi_19_ = 340282350000000000000000000000000000000f;
            } else {
                let _e576 = (_e532 * 0.5f);
                let _e582 = cantus_render_shader_sd_capsule_box(vec2<f32>((((_e251 - _e286) - _e537) - (_e529 * 146f)), ((_e309 - _e538) - _e576)), ((_e531 - _e532) * 0.5f), _e576);
                phi_19_ = _e582;
            }
            let _e584 = phi_19_;
            let _e586 = ((_e584 - 0.5f) * -1f);
            let _e588 = select(_e586, 0f, (_e586 < 0f));
            let _e590 = select(_e588, 1f, (_e588 > 1f));
            let _e598 = (_e571 - (((_e477 * ((_e590 * _e590) * (3f - (2f * _e590)))) + _e499) * 0.5f));
            let _e599 = (_e502 != _e502);
            if _e599 {
                phi_20_ = true;
            } else {
                phi_20_ = (_e598 <= _e502);
            }
            let _e602 = phi_20_;
            let _e603 = select(_e502, _e598, _e602);
            let _e604 = fwidth(_e603);
            if (_e604 != _e604) {
                phi_21_ = true;
            } else {
                phi_21_ = (0.55f >= _e604);
            }
            let _e608 = phi_21_;
            let _e609 = select(_e604, 0.55f, _e608);
            let _e613 = ((_e603 - _e609) / (-(_e609) - _e609));
            let _e615 = select(_e613, 0f, (_e613 < 0f));
            let _e617 = select(_e615, 1f, (_e615 > 1f));
            let _e621 = ((_e617 * _e617) * (3f - (2f * _e617)));
            if (_e603 != _e603) {
                phi_22_ = true;
            } else {
                phi_22_ = (0f >= _e603);
            }
            let _e625 = phi_22_;
            let _e629 = (exp((select(_e603, 0f, _e625) * -0.3f)) * 0.16f);
            if (_e621 != _e621) {
                phi_23_ = true;
            } else {
                phi_23_ = (_e629 >= _e621);
            }
            let _e633 = phi_23_;
            let _e634 = select(_e621, _e629, _e633);
            if (_e634 <= 0.0009765625f) {
                discard;
            }
            let _e640 = pill_2.member[_e220].hourly_conditions[0u];
            let _e641 = (_e238 * 0.0032467532f);
            let _e643 = select(_e641, 0f, (_e641 < 0f));
            let _e652 = pill_2.member[_e220].hourly_conditions[1u];
            let _e654 = ((abs((select(_e643, 1f, (_e643 > 1f)) - 0.5f)) - 0.2f) * 20.000002f);
            let _e656 = select(_e654, 0f, (_e654 < 0f));
            let _e658 = select(_e656, 1f, (_e656 > 1f));
            let _e662 = ((_e658 * _e658) * (3f - (2f * _e658)));
            let _e667 = (_e640.fog + ((_e652.fog - _e640.fog) * _e662));
            let _e672 = (_e640.cloud + ((_e652.cloud - _e640.cloud) * _e662));
            let _e677 = (_e640.rain + ((_e652.rain - _e640.rain) * _e662));
            let _e682 = (_e640.snow + ((_e652.snow - _e640.snow) * _e662));
            let _e687 = (_e640.lightning + ((_e652.lightning - _e640.lightning) * _e662));
            let _e692 = (_e640.hail + ((_e652.hail - _e640.hail) * _e662));
            let _e695 = (_e667 + ((_e640.fog - _e667) * _e219.w));
            let _e698 = (_e672 + ((_e640.cloud - _e672) * _e219.w));
            let _e701 = (_e677 + ((_e640.rain - _e677) * _e219.w));
            let _e704 = (_e682 + ((_e640.snow - _e682) * _e219.w));
            let _e707 = (_e687 + ((_e640.lightning - _e687) * _e219.w));
            let _e710 = (_e692 + ((_e640.hail - _e692) * _e219.w));
            let _e711 = (_e239 / _e237);
            if _e599 {
                phi_24_ = true;
            } else {
                phi_24_ = (0f <= _e502);
            }
            let _e716 = phi_24_;
            let _e719 = (1f + (select(_e502, 0f, _e716) * 0.008333334f));
            let _e721 = select(_e719, 0f, (_e719 < 0f));
            let _e723 = select(_e721, 0.6f, (_e721 > 0.6f));
            let _e730 = (_e479.x * 0.04f);
            let _e731 = (_e482.y * 0.04f);
            let _e732 = ((_e641 - (((_e641 - 0.5f) * _e723) * 0.08f)) - _e730);
            let _e733 = ((_e711 - (((_e711 - 0.5f) * _e723) * 0.08f)) - _e731);
            if (_e529 > 0.001f) {
                let _e736 = (_e539 / _e531);
                let _e737 = (_e540 / _e532);
                if (_e598 != _e598) {
                    phi_25_ = true;
                } else {
                    phi_25_ = (0f <= _e598);
                }
                let _e743 = phi_25_;
                let _e746 = (1f + (select(_e598, 0f, _e743) * 0.008333334f));
                let _e748 = select(_e746, 0f, (_e746 < 0f));
                let _e750 = select(_e748, 0.6f, (_e748 > 0.6f));
                phi_26_ = vec2<f32>(((_e736 - (((_e736 - 0.5f) * _e750) * 0.08f)) - _e730), ((_e737 - (((_e737 - 0.5f) * _e750) * 0.08f)) - _e731));
            } else {
                phi_26_ = vec2<f32>(_e732, _e733);
            }
            let _e761 = phi_26_;
            let _e762 = fwidth(_e598);
            if (_e762 != _e762) {
                phi_27_ = true;
            } else {
                phi_27_ = (0.55f >= _e762);
            }
            let _e766 = phi_27_;
            let _e767 = select(_e762, 0.55f, _e766);
            let _e771 = ((_e598 - _e767) / (-(_e767) - _e767));
            let _e773 = select(_e771, 0f, (_e771 < 0f));
            let _e775 = select(_e773, 1f, (_e773 > 1f));
            let _e780 = (((_e775 * _e775) * (3f - (2f * _e775))) * _e529);
            let _e781 = floor(_e560);
            let _e786 = select(select(u32(_e781), 0u, (_e781 < 0f)), 4294967295u, (_e781 > 4294967000f));
            if _e508 {
                if (_e786 < 5u) {
                } else {
                    break;
                }
                let _e895 = pill_2.member[_e220].daily_conditions[_e786];
                let _e896 = (_e786 + 1u);
                let _e898 = select(_e896, 4u, (4u < _e896));
                if (_e898 < 5u) {
                } else {
                    break;
                }
                let _e904 = pill_2.member[_e220].daily_conditions[_e898];
                let _e906 = (_e560 - trunc(_e560));
                let _e908 = select(_e906, 0f, (_e906 < 0f));
                let _e910 = select(_e908, 1f, (_e908 > 1f));
                let _e914 = ((_e910 * _e910) * (3f - (2f * _e910)));
                let _e949 = pill_2.member[_e220].sun_hours;
                let _e952 = (_e949[1] - _e949[0]);
                if (12f >= _e949[0]) {
                    let _e954 = (12f <= _e949[1]);
                    if _e954 {
                        let _e956 = ((12f - _e949[0]) / _e952);
                        phi_33_ = array<f32, 2>(_e956, sin((_e956 * 3.1415927f)));
                    } else {
                        phi_33_ = array<f32, 2>();
                    }
                    let _e961 = phi_33_;
                    phi_34_ = _e961;
                    phi_35_ = select(true, false, _e954);
                } else {
                    phi_34_ = array<f32, 2>();
                    phi_35_ = true;
                }
                let _e964 = phi_34_;
                let _e966 = phi_35_;
                if _e966 {
                    let _e967 = (24f - _e952);
                    if (12f < _e949[0]) {
                        phi_36_ = ((36f - _e949[1]) / _e967);
                    } else {
                        phi_36_ = ((12f - _e949[1]) / _e967);
                    }
                    let _e974 = phi_36_;
                    phi_37_ = array<f32, 2>(select(0f, 1f, (12f >= _e949[1])), -(sin((_e974 * 3.1415927f))));
                } else {
                    phi_37_ = _e964;
                }
                let _e982 = phi_37_;
                phi_38_ = u0028_render_tempestas_WeatherCondition_u0020_f32_u0029_(render_tempestas_WeatherCondition((_e895.fog + ((_e904.fog - _e895.fog) * _e914)), (_e895.cloud + ((_e904.cloud - _e895.cloud) * _e914)), (_e895.rain + ((_e904.rain - _e895.rain) * _e914)), (_e895.snow + ((_e904.snow - _e895.snow) * _e914)), (_e895.lightning + ((_e904.lightning - _e895.lightning) * _e914)), (_e895.hail + ((_e904.hail - _e895.hail) * _e914))), _e982[1]);
            } else {
                if (_e786 < 6u) {
                } else {
                    break;
                }
                let _e792 = pill_2.member[_e220].hourly_conditions[_e786];
                let _e793 = (_e786 + 1u);
                let _e795 = select(_e793, 5u, (5u < _e793));
                if (_e795 < 6u) {
                } else {
                    break;
                }
                let _e801 = pill_2.member[_e220].hourly_conditions[_e795];
                let _e803 = (_e560 - trunc(_e560));
                let _e805 = select(_e803, 0f, (_e803 < 0f));
                let _e807 = select(_e805, 1f, (_e805 > 1f));
                let _e811 = ((_e807 * _e807) * (3f - (2f * _e807)));
                let _e846 = pill_2.member[_e220].hourly_start;
                let _e849 = ((_e846 + (_e560 * 4f)) % 24f);
                let _e853 = pill_2.member[_e220].sun_hours;
                let _e856 = (_e853[1] - _e853[0]);
                if (_e849 >= _e853[0]) {
                    let _e858 = (_e849 <= _e853[1]);
                    if _e858 {
                        let _e860 = ((_e849 - _e853[0]) / _e856);
                        phi_28_ = array<f32, 2>(_e860, sin((_e860 * 3.1415927f)));
                    } else {
                        phi_28_ = array<f32, 2>();
                    }
                    let _e865 = phi_28_;
                    phi_29_ = _e865;
                    phi_30_ = select(true, false, _e858);
                } else {
                    phi_29_ = array<f32, 2>();
                    phi_30_ = true;
                }
                let _e868 = phi_29_;
                let _e870 = phi_30_;
                if _e870 {
                    let _e871 = (24f - _e856);
                    if (_e849 < _e853[0]) {
                        phi_31_ = (((_e849 + 24f) - _e853[1]) / _e871);
                    } else {
                        phi_31_ = ((_e849 - _e853[1]) / _e871);
                    }
                    let _e879 = phi_31_;
                    phi_32_ = array<f32, 2>(select(0f, 1f, (_e849 >= _e853[1])), -(sin((_e879 * 3.1415927f))));
                } else {
                    phi_32_ = _e868;
                }
                let _e887 = phi_32_;
                phi_38_ = u0028_render_tempestas_WeatherCondition_u0020_f32_u0029_(render_tempestas_WeatherCondition((_e792.fog + ((_e801.fog - _e792.fog) * _e811)), (_e792.cloud + ((_e801.cloud - _e792.cloud) * _e811)), (_e792.rain + ((_e801.rain - _e792.rain) * _e811)), (_e792.snow + ((_e801.snow - _e792.snow) * _e811)), (_e792.lightning + ((_e801.lightning - _e792.lightning) * _e811)), (_e792.hail + ((_e801.hail - _e792.hail) * _e811))), _e887[1]);
            }
            let _e986 = phi_38_;
            let _e995 = (1f - _e780);
            let _e1000 = (((_e732 * 308f) * _e995) + ((_e761.x * _e531) * _e780));
            let _e1001 = (((_e733 * _e237) * _e995) + ((_e761.y * _e532) * _e780));
            if (_e598 != _e598) {
                phi_39_ = true;
            } else {
                phi_39_ = (1000f <= _e598);
            }
            let _e1008 = phi_39_;
            let _e1016 = (_e695 + ((_e986.unnamed.fog - _e695) * _e780));
            let _e1020 = (_e698 + ((_e986.unnamed.cloud - _e698) * _e780));
            let _e1024 = (_e701 + ((_e986.unnamed.rain - _e701) * _e780));
            let _e1028 = (_e704 + ((_e986.unnamed.snow - _e704) * _e780));
            let _e1036 = (_e710 + ((_e986.unnamed.hail - _e710) * _e780));
            let _e1038 = ((_e219.y - -0.04f) * 4.1666665f);
            let _e1040 = select(_e1038, 0f, (_e1038 < 0f));
            let _e1042 = select(_e1040, 1f, (_e1040 > 1f));
            let _e1046 = ((_e1042 * _e1042) * (3f - (2f * _e1042)));
            let _e1048 = ((_e219.y - -0.32f) * 4.166667f);
            let _e1050 = select(_e1048, 0f, (_e1048 < 0f));
            let _e1052 = select(_e1050, 1f, (_e1050 > 1f));
            let _e1058 = (((_e1052 * _e1052) * (3f - (2f * _e1052))) * (1f - _e1046));
            let _e1060 = ((_e219.y - -0.18f) * 5.5555553f);
            let _e1062 = select(_e1060, 0f, (_e1060 < 0f));
            let _e1064 = select(_e1062, 1f, (_e1062 > 1f));
            let _e1070 = ((_e219.y - 0.2f) * -5.5555553f);
            let _e1072 = select(_e1070, 0f, (_e1070 < 0f));
            let _e1074 = select(_e1072, 1f, (_e1072 > 1f));
            let _e1079 = (((_e1064 * _e1064) * (3f - (2f * _e1064))) * ((_e1074 * _e1074) * (3f - (2f * _e1074))));
            let _e1081 = ((_e986.unnamed_1 - -0.04f) * 4.1666665f);
            let _e1083 = select(_e1081, 0f, (_e1081 < 0f));
            let _e1085 = select(_e1083, 1f, (_e1083 > 1f));
            let _e1089 = ((_e1085 * _e1085) * (3f - (2f * _e1085)));
            let _e1091 = ((_e986.unnamed_1 - -0.32f) * 4.166667f);
            let _e1093 = select(_e1091, 0f, (_e1091 < 0f));
            let _e1095 = select(_e1093, 1f, (_e1093 > 1f));
            let _e1103 = ((_e986.unnamed_1 - -0.18f) * 5.5555553f);
            let _e1105 = select(_e1103, 0f, (_e1103 < 0f));
            let _e1107 = select(_e1105, 1f, (_e1105 > 1f));
            let _e1113 = ((_e986.unnamed_1 - 0.2f) * -5.5555553f);
            let _e1115 = select(_e1113, 0f, (_e1113 < 0f));
            let _e1117 = select(_e1115, 1f, (_e1115 > 1f));
            let _e1125 = (_e1046 + ((_e1089 - _e1046) * _e780));
            let _e1131 = (_e1079 + (((((_e1107 * _e1107) * (3f - (2f * _e1107))) * ((_e1117 * _e1117) * (3f - (2f * _e1117)))) - _e1079) * _e780));
            let _e1135 = frame.member[0u].time;
            let _e1136 = (_e1001 / _e237);
            let _e1138 = ((_e1136 - 1f) * -1f);
            let _e1140 = select(_e1138, 0f, (_e1138 < 0f));
            let _e1142 = select(_e1140, 1f, (_e1140 > 1f));
            let _e1146 = ((_e1142 * _e1142) * (3f - (2f * _e1142)));
            let _e1147 = (1f - _e1146);
            let _e1166 = (1f - _e1125);
            let _e1178 = (0.3f * _e1147);
            let _e1179 = (0.22f * _e1146);
            let _e1185 = ((_e1058 + (((((_e1095 * _e1095) * (3f - (2f * _e1095))) * (1f - _e1089)) - _e1058) * _e780)) * 0.8f);
            let _e1186 = (1f - _e1185);
            let _e1203 = (_e1131 * 0.9f);
            let _e1204 = (1f - _e1203);
            let _e1216 = floor((_e1000 * 0.055555556f));
            let _e1217 = floor((_e1001 * 0.055555556f));
            let _e1221 = cantus_render_shader_hash(vec2<f32>(_e1216, _e1217));
            let _e1230 = (_e1000 - (((_e1216 + 0.2f) + (_e1221.x * 0.6f)) * 18f));
            let _e1231 = (_e1001 - (((_e1217 + 0.2f) + (_e1221.y * 0.6f)) * 18f));
            let _e1237 = ((sqrt(((_e1230 * _e1230) + (_e1231 * _e1231))) - 1f) * -1.6666666f);
            let _e1239 = select(_e1237, 0f, (_e1237 < 0f));
            let _e1241 = select(_e1239, 1f, (_e1239 > 1f));
            let _e1249 = cantus_render_shader_hash(vec2<f32>((_e1216 + 31.7f), (_e1217 + 31.7f)));
            let _e1252 = ((_e1249.x - 0.75f) * 4f);
            let _e1254 = select(_e1252, 0f, (_e1252 < 0f));
            let _e1256 = select(_e1254, 1f, (_e1254 > 1f));
            let _e1267 = ((((((_e1241 * _e1241) * (3f - (2f * _e1241))) * ((_e1256 * _e1256) * (3f - (2f * _e1256)))) * _e1166) * (1f - _e1020)) * (0.3f + (_e1146 * 0.7f)));
            let _e1268 = (((((((((0.006f * _e1147) + (0.025f * _e1146)) * _e1166) + (((0.08f * _e1147) + (0.32f * _e1146)) * _e1125)) * _e1186) + (((0.1f * _e1147) + _e1179) * _e1185)) * _e1204) + (((0.78f * _e1147) + (0.38f * _e1146)) * _e1203)) + _e1267);
            let _e1269 = (((((((((0.012f * _e1147) + (0.04f * _e1146)) * _e1166) + (((0.34f * _e1147) + (0.67f * _e1146)) * _e1125)) * _e1186) + (((0.16f * _e1147) + (0.25f * _e1146)) * _e1185)) * _e1204) + ((_e1178 + _e1179) * _e1203)) + _e1267);
            let _e1270 = (((((((((0.035f * _e1147) + (0.095f * _e1146)) * _e1166) + (((0.62f * _e1147) + (0.87f * _e1146)) * _e1125)) * _e1186) + ((_e1178 + (0.45f * _e1146)) * _e1185)) * _e1204) + (((0.2f * _e1147) + (0.42f * _e1146)) * _e1203)) + _e1267);
            if (_e1020 > 0.0009765625f) {
                let _e1273 = (_e1000 / _e237);
                phi_40_ = 0i;
                phi_41_ = 0.5f;
                phi_42_ = 0f;
                phi_43_ = vec2<f32>(((_e1273 * 0.14f) + (_e1135 * 0.012f)), ((_e1136 * 0.14f) + 6.1f));
                loop {
                    let _e1281 = phi_40_;
                    let _e1283 = phi_41_;
                    let _e1285 = phi_42_;
                    let _e1287 = phi_43_;
                    local_41 = _e1285;
                    let _e1288 = (_e1281 < 4i);
                    if _e1288 {
                        let _e1291 = cantus_render_shader_simplex_noise(_e1287);
                        phi_44_ = (_e1281 + 1i);
                        phi_45_ = (_e1283 * 0.5f);
                        phi_46_ = (_e1285 + (_e1291 * _e1283));
                        phi_47_ = vec2<f32>(((_e1287.x * 1.6f) + (_e1287.y * 1.2f)), ((_e1287.y * 1.6f) - (_e1287.x * 1.2f)));
                    } else {
                        phi_44_ = i32();
                        phi_45_ = f32();
                        phi_46_ = f32();
                        phi_47_ = vec2<f32>();
                    }
                    let _e1304 = phi_44_;
                    let _e1306 = phi_45_;
                    let _e1308 = phi_46_;
                    let _e1310 = phi_47_;
                    continue;
                    continuing {
                        phi_40_ = _e1304;
                        phi_41_ = _e1306;
                        phi_42_ = _e1308;
                        phi_43_ = _e1310;
                        break if !(_e1288);
                    }
                }
                let _e1313 = local_41;
                let _e1314 = (_e1313 * 0.5f);
                phi_48_ = 0i;
                phi_49_ = 0.5f;
                phi_50_ = 0f;
                phi_51_ = vec2<f32>(((_e1273 * 0.287f) + (_e1135 * 0.018f)), ((_e1136 * 0.287f) + -3.7f));
                loop {
                    let _e1323 = phi_48_;
                    let _e1325 = phi_49_;
                    let _e1327 = phi_50_;
                    let _e1329 = phi_51_;
                    local_42 = _e1327;
                    local_43 = _e1327;
                    let _e1330 = (_e1323 < 4i);
                    if _e1330 {
                        let _e1333 = cantus_render_shader_simplex_noise(_e1329);
                        phi_52_ = (_e1323 + 1i);
                        phi_53_ = (_e1325 * 0.5f);
                        phi_54_ = (_e1327 + (_e1333 * _e1325));
                        phi_55_ = vec2<f32>(((_e1329.x * 1.6f) + (_e1329.y * 1.2f)), ((_e1329.y * 1.6f) - (_e1329.x * 1.2f)));
                    } else {
                        phi_52_ = i32();
                        phi_53_ = f32();
                        phi_54_ = f32();
                        phi_55_ = vec2<f32>();
                    }
                    let _e1346 = phi_52_;
                    let _e1348 = phi_53_;
                    let _e1350 = phi_54_;
                    let _e1352 = phi_55_;
                    continue;
                    continuing {
                        phi_48_ = _e1346;
                        phi_49_ = _e1348;
                        phi_50_ = _e1350;
                        phi_51_ = _e1352;
                        break if !(_e1330);
                    }
                }
                let _e1355 = local_42;
                let _e1358 = local_43;
                let _e1362 = ((((0.5f + _e1314) + (_e1358 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1364 = select(_e1362, 0f, (_e1362 < 0f));
                let _e1366 = select(_e1364, 1f, (_e1364 > 1f));
                let _e1372 = (((_e1355 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1374 = select(_e1372, 0f, (_e1372 < 0f));
                let _e1376 = select(_e1374, 1f, (_e1374 > 1f));
                let _e1383 = ((_e1314 + 0.02000001f) * 4.5454545f);
                let _e1385 = select(_e1383, 0f, (_e1383 < 0f));
                let _e1387 = select(_e1385, 1f, (_e1385 > 1f));
                let _e1393 = ((((_e1376 * _e1376) * (3f - (2f * _e1376))) * 0.55f) + (((_e1387 * _e1387) * (3f - (2f * _e1387))) * 0.45f));
                let _e1394 = (1f - _e1393);
                let _e1431 = (_e1131 * 0.45f);
                let _e1432 = (1f - _e1431);
                let _e1444 = (_e1020 * (0.12f + (((_e1366 * _e1366) * (3f - (2f * _e1366))) * 0.7f)));
                let _e1445 = (1f - _e1444);
                phi_56_ = vec3<f32>(((_e1268 * _e1445) + (((((((0.16f * _e1394) + (0.32f * _e1393)) * _e1166) + (((0.62f * _e1394) + (0.92f * _e1393)) * _e1125)) * _e1432) + (((0.5f * _e1394) + (0.76f * _e1393)) * _e1431)) * _e1444)), ((_e1269 * _e1445) + (((((((0.2f * _e1394) + (0.36f * _e1393)) * _e1166) + (((0.7f * _e1394) + (0.94f * _e1393)) * _e1125)) * _e1432) + (((0.36f * _e1394) + (0.59f * _e1393)) * _e1431)) * _e1444)), ((_e1270 * _e1445) + (((((((0.28f * _e1394) + (0.43f * _e1393)) * _e1166) + (((0.78f * _e1394) + (0.96f * _e1393)) * _e1125)) * _e1432) + (((0.4f * _e1394) + (0.56f * _e1393)) * _e1431)) * _e1444)));
            } else {
                phi_56_ = vec3<f32>(_e1268, _e1269, _e1270);
            }
            let _e1457 = phi_56_;
            let _e1459 = (1f - (_e1024 * 0.2f));
            let _e1469 = ((_e1457.x * _e1459) + (_e1024 * 0.020000001f));
            let _e1470 = ((_e1457.y * _e1459) + (_e1024 * 0.034f));
            let _e1471 = ((_e1457.z * _e1459) + (_e1024 * 0.05f));
            if (_e1024 > 0.0009765625f) {
                let _e1476 = (_e1000 - (20f * _e1135));
                let _e1477 = (_e1001 - (110f * _e1135));
                let _e1480 = floor((_e1476 * 0.06666667f));
                let _e1481 = floor((_e1477 * 0.04f));
                let _e1483 = cantus_render_shader_hash(vec2<f32>(_e1480, _e1481));
                let _e1494 = (_e1476 - (((_e1480 + 0.15f) + (_e1483.x * 0.7f)) * 15f));
                let _e1495 = (_e1477 - (((_e1481 + 0.15f) + (_e1483.y * 0.7f)) * 25f));
                let _e1499 = (((_e1494 * 1.8000001f) + (_e1495 * 9f)) * 0.011870845f);
                let _e1501 = select(_e1499, 0f, (_e1499 < 0f));
                let _e1503 = select(_e1501, 1f, (_e1501 > 1f));
                let _e1506 = (_e1494 - (1.8000001f * _e1503));
                let _e1507 = (_e1495 - (9f * _e1503));
                let _e1513 = ((sqrt(((_e1506 * _e1506) + (_e1507 * _e1507))) - 1.0999999f) * -1.666667f);
                let _e1515 = select(_e1513, 0f, (_e1513 < 0f));
                let _e1517 = select(_e1515, 1f, (_e1515 > 1f));
                let _e1525 = cantus_render_shader_hash(vec2<f32>((_e1480 + 19.3f), (_e1481 + 19.3f)));
                let _e1528 = ((_e1525.x - 0.22000003f) * 1.2820513f);
                let _e1530 = select(_e1528, 0f, (_e1528 < 0f));
                let _e1532 = select(_e1530, 1f, (_e1530 > 1f));
                let _e1539 = (((((_e1517 * _e1517) * (3f - (2f * _e1517))) * ((_e1532 * _e1532) * (3f - (2f * _e1532)))) * _e1024) * 0.7f);
                let _e1541 = select(_e1539, 0f, (_e1539 < 0f));
                let _e1543 = select(_e1541, 1f, (_e1541 > 1f));
                let _e1544 = (1f - _e1543);
                phi_57_ = vec3<f32>(((_e1469 * _e1544) + (0.52f * _e1543)), ((_e1470 * _e1544) + (0.72f * _e1543)), ((_e1471 * _e1544) + (0.9f * _e1543)));
            } else {
                phi_57_ = vec3<f32>(_e1469, _e1470, _e1471);
            }
            let _e1556 = phi_57_;
            if (_e1028 > 0.0009765625f) {
                let _e1560 = (_e1000 - (5f * _e1135));
                let _e1561 = (_e1001 - (14f * _e1135));
                let _e1564 = floor((_e1560 * 0.05f));
                let _e1565 = floor((_e1561 * 0.05f));
                let _e1569 = cantus_render_shader_hash(vec2<f32>((_e1564 + 31.7f), (_e1565 + 31.7f)));
                let _e1580 = (_e1560 - (((_e1564 + 0.15f) + (_e1569.x * 0.7f)) * 20f));
                let _e1581 = (_e1561 - (((_e1565 + 0.15f) + (_e1569.y * 0.7f)) * 20f));
                let _e1585 = (((_e1580 * 0.080000006f) + (_e1581 * 0.4f)) * 6.009615f);
                let _e1587 = select(_e1585, 0f, (_e1585 < 0f));
                let _e1589 = select(_e1587, 1f, (_e1587 > 1f));
                let _e1592 = (_e1580 - (0.080000006f * _e1589));
                let _e1593 = (_e1581 - (0.4f * _e1589));
                let _e1599 = ((sqrt(((_e1592 * _e1592) + (_e1593 * _e1593))) - 1.5999999f) * -1.666667f);
                let _e1601 = select(_e1599, 0f, (_e1599 < 0f));
                let _e1603 = select(_e1601, 1f, (_e1601 > 1f));
                let _e1611 = cantus_render_shader_hash(vec2<f32>((_e1564 + 19.3f), (_e1565 + 19.3f)));
                let _e1614 = ((_e1611.x - 0.3f) * 1.4285715f);
                let _e1616 = select(_e1614, 0f, (_e1614 < 0f));
                let _e1618 = select(_e1616, 1f, (_e1616 > 1f));
                let _e1625 = (((((_e1603 * _e1603) * (3f - (2f * _e1603))) * ((_e1618 * _e1618) * (3f - (2f * _e1618)))) * _e1028) * 0.92f);
                let _e1627 = select(_e1625, 0f, (_e1625 < 0f));
                let _e1629 = select(_e1627, 1f, (_e1627 > 1f));
                let _e1630 = (1f - _e1629);
                let _e1637 = (0.96f * _e1629);
                phi_58_ = vec3<f32>(((_e1556.x * _e1630) + _e1637), ((_e1556.y * _e1630) + _e1637), ((_e1556.z * _e1630) + _e1637));
            } else {
                phi_58_ = _e1556;
            }
            let _e1643 = phi_58_;
            if (_e1036 > 0.0009765625f) {
                let _e1647 = (_e1000 - (18f * _e1135));
                let _e1648 = (_e1001 - (85f * _e1135));
                let _e1651 = floor((_e1647 * 0.04347826f));
                let _e1652 = floor((_e1648 * 0.04347826f));
                let _e1656 = cantus_render_shader_hash(vec2<f32>((_e1651 + 63.4f), (_e1652 + 63.4f)));
                let _e1667 = (_e1647 - (((_e1651 + 0.15f) + (_e1656.x * 0.7f)) * 23f));
                let _e1668 = (_e1648 - (((_e1652 + 0.15f) + (_e1656.y * 0.7f)) * 23f));
                let _e1672 = (((_e1667 * 0.24000001f) + (_e1668 * 1.2f)) * 0.667735f);
                let _e1674 = select(_e1672, 0f, (_e1672 < 0f));
                let _e1676 = select(_e1674, 1f, (_e1674 > 1f));
                let _e1679 = (_e1667 - (0.24000001f * _e1676));
                let _e1680 = (_e1668 - (1.2f * _e1676));
                let _e1686 = ((sqrt(((_e1679 * _e1679) + (_e1680 * _e1680))) - 0.79999995f) * -1.6666667f);
                let _e1688 = select(_e1686, 0f, (_e1686 < 0f));
                let _e1690 = select(_e1688, 1f, (_e1688 > 1f));
                let _e1698 = cantus_render_shader_hash(vec2<f32>((_e1651 + 19.3f), (_e1652 + 19.3f)));
                let _e1701 = ((_e1698.x - 0.7f) * 3.3333333f);
                let _e1703 = select(_e1701, 0f, (_e1701 < 0f));
                let _e1705 = select(_e1703, 1f, (_e1703 > 1f));
                let _e1712 = (((((_e1690 * _e1690) * (3f - (2f * _e1690))) * ((_e1705 * _e1705) * (3f - (2f * _e1705)))) * _e1036) * 0.7f);
                let _e1714 = select(_e1712, 0f, (_e1712 < 0f));
                let _e1716 = select(_e1714, 1f, (_e1714 > 1f));
                let _e1717 = (1f - _e1716);
                phi_59_ = vec3<f32>(((_e1643.x * _e1717) + (0.75f * _e1716)), ((_e1643.y * _e1717) + (0.86f * _e1716)), ((_e1643.z * _e1717) + (0.94f * _e1716)));
            } else {
                phi_59_ = _e1643;
            }
            let _e1732 = phi_59_;
            let _e1736 = ((sin((_e1135 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1738 = select(_e1736, 0f, (_e1736 < 0f));
            let _e1740 = select(_e1738, 1f, (_e1738 > 1f));
            let _e1745 = (((_e1740 * _e1740) * (3f - (2f * _e1740))) * (_e707 + ((_e986.unnamed.lightning - _e707) * _e780)));
            let _e1747 = (1f - (_e1745 * 0.55f));
            let _e1757 = ((_e1732.x * _e1747) + (_e1745 * 0.3575f));
            let _e1758 = ((_e1732.y * _e1747) + (_e1745 * 0.407f));
            let _e1759 = ((_e1732.z * _e1747) + (_e1745 * 0.528f));
            if (_e1016 > 0.0009765625f) {
                phi_60_ = 0i;
                phi_61_ = 0.5f;
                phi_62_ = 0f;
                phi_63_ = vec2<f32>((((_e1000 / (308f + ((_e531 - 308f) * _e780))) * 0.9f) + (_e1135 * 0.008f)), ((_e1136 * 0.32f) + 12f));
                loop {
                    let _e1770 = phi_60_;
                    let _e1772 = phi_61_;
                    let _e1774 = phi_62_;
                    let _e1776 = phi_63_;
                    local_44 = _e1774;
                    let _e1777 = (_e1770 < 4i);
                    if _e1777 {
                        let _e1780 = cantus_render_shader_simplex_noise(_e1776);
                        phi_64_ = (_e1770 + 1i);
                        phi_65_ = (_e1772 * 0.5f);
                        phi_66_ = (_e1774 + (_e1780 * _e1772));
                        phi_67_ = vec2<f32>(((_e1776.x * 1.6f) + (_e1776.y * 1.2f)), ((_e1776.y * 1.6f) - (_e1776.x * 1.2f)));
                    } else {
                        phi_64_ = i32();
                        phi_65_ = f32();
                        phi_66_ = f32();
                        phi_67_ = vec2<f32>();
                    }
                    let _e1793 = phi_64_;
                    let _e1795 = phi_65_;
                    let _e1797 = phi_66_;
                    let _e1799 = phi_67_;
                    continue;
                    continuing {
                        phi_60_ = _e1793;
                        phi_61_ = _e1795;
                        phi_62_ = _e1797;
                        phi_63_ = _e1799;
                        break if !(_e1777);
                    }
                }
                let _e1802 = local_44;
                let _e1805 = (((_e1802 * 0.5f) + 0.15f) * 2.857143f);
                let _e1807 = select(_e1805, 0f, (_e1805 < 0f));
                let _e1809 = select(_e1807, 1f, (_e1807 > 1f));
                let _e1816 = (_e1016 * (0.58f + (((_e1809 * _e1809) * (3f - (2f * _e1809))) * 0.18f)));
                let _e1817 = (1f - _e1816);
                phi_68_ = vec3<f32>(((_e1757 * _e1817) + (0.63f * _e1816)), ((_e1758 * _e1817) + (0.69f * _e1816)), ((_e1759 * _e1817) + (0.73f * _e1816)));
            } else {
                phi_68_ = vec3<f32>(_e1757, _e1758, _e1759);
            }
            let _e1829 = phi_68_;
            let _e1831 = ((_e1136 - 0.12f) * -8.333334f);
            let _e1833 = select(_e1831, 0f, (_e1831 < 0f));
            let _e1835 = select(_e1833, 1f, (_e1833 > 1f));
            let _e1842 = (((_e502 + ((select(_e598, 1000f, _e1008) - _e502) * _e780)) - 5f) * -0.125f);
            let _e1844 = select(_e1842, 0f, (_e1842 < 0f));
            let _e1846 = select(_e1844, 1f, (_e1844 > 1f));
            let _e1852 = ((((_e1835 * _e1835) * (3f - (2f * _e1835))) * 0.12f) + (((_e1846 * _e1846) * (3f - (2f * _e1846))) * 0.08f));
            let _e1854 = (_e1829.x + _e1852);
            let _e1856 = (_e1829.y + _e1852);
            let _e1858 = (_e1829.z + _e1852);
            if (_e246 < 1f) {
                let _e1863 = (16f + (_e219.x * 276f));
                let _e1865 = select(_e219.y, 0f, (_e219.y < 0f));
                let _e1869 = (0.72f - (select(_e1865, 1f, (_e1865 > 1f)) * 0.45f));
                let _e1872 = ((_e219.y - 0.55f) * -1.8867923f);
                let _e1874 = select(_e1872, 0f, (_e1872 < 0f));
                let _e1876 = select(_e1874, 1f, (_e1874 > 1f));
                let _e1880 = ((_e1876 * _e1876) * (3f - (2f * _e1876)));
                let _e1881 = (1f - _e1880);
                if (_e672 > 0.0009765625f) {
                    phi_69_ = 0i;
                    phi_70_ = 0.5f;
                    phi_71_ = 0f;
                    phi_72_ = vec2<f32>((((_e1863 / _e237) * 0.14f) + (_e1135 * 0.012f)), ((_e1869 * 0.14f) + 6.1f));
                    loop {
                        let _e1899 = phi_69_;
                        let _e1901 = phi_70_;
                        let _e1903 = phi_71_;
                        let _e1905 = phi_72_;
                        local_45 = _e1903;
                        let _e1906 = (_e1899 < 4i);
                        if _e1906 {
                            let _e1909 = cantus_render_shader_simplex_noise(_e1905);
                            phi_73_ = (_e1899 + 1i);
                            phi_74_ = (_e1901 * 0.5f);
                            phi_75_ = (_e1903 + (_e1909 * _e1901));
                            phi_76_ = vec2<f32>(((_e1905.x * 1.6f) + (_e1905.y * 1.2f)), ((_e1905.y * 1.6f) - (_e1905.x * 1.2f)));
                        } else {
                            phi_73_ = i32();
                            phi_74_ = f32();
                            phi_75_ = f32();
                            phi_76_ = vec2<f32>();
                        }
                        let _e1922 = phi_73_;
                        let _e1924 = phi_74_;
                        let _e1926 = phi_75_;
                        let _e1928 = phi_76_;
                        continue;
                        continuing {
                            phi_69_ = _e1922;
                            phi_70_ = _e1924;
                            phi_71_ = _e1926;
                            phi_72_ = _e1928;
                            break if !(_e1906);
                        }
                    }
                    let _e1931 = local_45;
                    let _e1934 = (((_e1931 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e1936 = select(_e1934, 0f, (_e1934 < 0f));
                    let _e1938 = select(_e1936, 1f, (_e1936 > 1f));
                    phi_77_ = ((((_e1938 * _e1938) * (3f - (2f * _e1938))) * _e672) * 0.82f);
                } else {
                    phi_77_ = 0f;
                }
                let _e1946 = phi_77_;
                let _e1948 = ((_e219.y - -0.02f) * 16.666668f);
                let _e1950 = select(_e1948, 0f, (_e1948 < 0f));
                let _e1952 = select(_e1950, 1f, (_e1950 > 1f));
                let _e1959 = (_e238 - _e1863);
                let _e1960 = (_e239 - (_e237 * _e1869));
                let _e1964 = sqrt(((_e1959 * _e1959) + (_e1960 * _e1960)));
                let _e1966 = ((_e1964 - 62f) * -0.01724138f);
                let _e1968 = select(_e1966, 0f, (_e1966 < 0f));
                let _e1970 = select(_e1968, 1f, (_e1968 > 1f));
                let _e1977 = ((_e1964 - 11f) * -0.1f);
                let _e1979 = select(_e1977, 0f, (_e1977 < 0f));
                let _e1981 = select(_e1979, 1f, (_e1979 > 1f));
                let _e1988 = (((((_e1970 * _e1970) * (3f - (2f * _e1970))) * 0.24f) + (((_e1981 * _e1981) * (3f - (2f * _e1981))) * 0.7f)) * (((_e1952 * _e1952) * (3f - (2f * _e1952))) * (1f - _e1946)));
                let _e1989 = (1f - _e1988);
                let _e2002 = ((_e246 - 1f) / ((_e237 * -0.25f) - 1f));
                let _e2004 = select(_e2002, 0f, (_e2002 < 0f));
                let _e2006 = select(_e2004, 1f, (_e2004 > 1f));
                let _e2010 = ((_e2006 * _e2006) * (3f - (2f * _e2006)));
                let _e2011 = (1f - _e2010);
                phi_78_ = vec3<f32>(((_e1854 * _e2011) + (((_e1854 * _e1989) + (((0.96f * _e1881) + (0.98f * _e1880)) * _e1988)) * _e2010)), ((_e1856 * _e2011) + (((_e1856 * _e1989) + (((0.98f * _e1881) + (0.74f * _e1880)) * _e1988)) * _e2010)), ((_e1858 * _e2011) + (((_e1858 * _e1989) + ((_e1881 + (0.66f * _e1880)) * _e1988)) * _e2010)));
            } else {
                phi_78_ = (_e1829 + vec3(_e1852));
            }
            let _e2023 = phi_78_;
            if (_e513 > 0f) {
                let _e2025 = (_e285 >= 0f);
                if _e2025 {
                    if (_e287 < 308f) {
                        if (_e285 < 54f) {
                            let _e2143 = (_e287 - 154f);
                            if (_e2143 < -104f) {
                                phi_88_ = 2u;
                            } else {
                                phi_88_ = select(1u, 3u, (_e2143 > 104f));
                            }
                            let _e2148 = phi_88_;
                            phi_89_ = 40f;
                            phi_90_ = _e2148;
                            phi_91_ = 1f;
                            phi_92_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                        } else {
                            if (_e285 < 82f) {
                                let _e2124 = floor((_e287 * 0.022727273f));
                                let _e2126 = select(_e2124, 0f, (_e2124 < 0f));
                                let _e2128 = select(_e2126, 6f, (_e2126 > 6f));
                                phi_84_ = 68f;
                                phi_85_ = (27u + select(select(u32(_e2128), 0u, (_e2128 < 0f)), 4294967295u, (_e2128 > 4294967000f)));
                                phi_86_ = 0.75f;
                                phi_87_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                            } else {
                                let _e2072 = floor((((_e285 - 96f) * 0.041666668f) + 0.5f));
                                let _e2074 = select(_e2072, 0f, (_e2072 < 0f));
                                let _e2076 = select(_e2074, 5f, (_e2074 > 5f));
                                let _e2084 = floor((_e287 * 0.022727273f));
                                let _e2086 = select(_e2084, 0f, (_e2084 < 0f));
                                let _e2088 = select(_e2086, 6f, (_e2086 > 6f));
                                let _e2094 = ((select(select(u32(_e2076), 0u, (_e2076 < 0f)), 4294967295u, (_e2076 > 4294967000f)) * 7u) + select(select(u32(_e2088), 0u, (_e2088 < 0f)), 4294967295u, (_e2088 > 4294967000f)));
                                let _e2104 = pill_2.member[_e220].today_index;
                                let _e2112 = pill_2.member[_e220].month_range[0u];
                                if (_e2094 < _e2112) {
                                    phi_83_ = true;
                                } else {
                                    let _e2118 = pill_2.member[_e220].month_range[1u];
                                    phi_83_ = (_e2094 >= _e2118);
                                }
                                let _e2121 = phi_83_;
                                phi_84_ = (96f + (f32((_e2094 / 7u)) * 24f));
                                phi_85_ = (34u + _e2094);
                                phi_86_ = select(1f, 0.32f, _e2121);
                                phi_87_ = select(vec3<f32>(0.94f, 0.94f, 0.94f), vec3<f32>(1f, 0.68f, 0.68f), vec3((bitcast<i32>(_e2094) == _e2104)));
                            }
                            let _e2136 = phi_84_;
                            let _e2138 = phi_85_;
                            let _e2140 = phi_86_;
                            let _e2142 = phi_87_;
                            phi_89_ = _e2136;
                            phi_90_ = _e2138;
                            phi_91_ = _e2140;
                            phi_92_ = _e2142;
                        }
                        let _e2150 = phi_89_;
                        let _e2152 = phi_90_;
                        let _e2154 = phi_91_;
                        let _e2156 = phi_92_;
                        phi_93_ = _e2150;
                        phi_94_ = _e2152;
                        phi_95_ = _e2154;
                        phi_96_ = _e2156;
                    } else {
                        if (_e287 >= 316f) {
                            if (_e285 >= 56f) {
                                let _e2030 = select(6u, 5u, _e508);
                                let _e2035 = ((((_e287 - 316f) * 0.0032467532f) * f32(_e2030)) - 0.5f);
                                let _e2037 = f32((_e2030 - 1u));
                                let _e2038 = (0f <= _e2037);
                                if _e2038 {
                                } else {
                                    break;
                                }
                                let _e2040 = select(_e2035, 0f, (_e2035 < 0f));
                                let _e2043 = round(select(_e2040, _e2037, (_e2040 > _e2037)));
                                if _e2038 {
                                } else {
                                    break;
                                }
                                let _e2045 = select(_e2043, 0f, (_e2043 < 0f));
                                let _e2047 = select(_e2045, _e2037, (_e2045 > _e2037));
                                phi_79_ = _e515;
                                phi_80_ = (select(5u, 17u, _e508) + ((select(select(u32(_e2047), 0u, (_e2047 < 0f)), 4294967295u, (_e2047 > 4294967000f)) * 2u) + select(0u, 1u, (_e285 >= _e515))));
                            } else {
                                phi_79_ = 40f;
                                phi_80_ = 4u;
                            }
                            let _e2060 = phi_79_;
                            let _e2062 = phi_80_;
                            phi_81_ = _e2060;
                            phi_82_ = _e2062;
                        } else {
                            phi_81_ = 40f;
                            phi_82_ = 4u;
                        }
                        let _e2064 = phi_81_;
                        let _e2066 = phi_82_;
                        phi_93_ = _e2064;
                        phi_94_ = _e2066;
                        phi_95_ = 1f;
                        phi_96_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                    }
                    let _e2158 = phi_93_;
                    let _e2160 = phi_94_;
                    let _e2162 = phi_95_;
                    let _e2164 = phi_96_;
                    let _e2165 = (_e2158 * 0.0007377049f);
                    let _e2166 = (0.5f + _e2165);
                    let _e2170 = ((_e513 - _e2166) / ((_e2165 + 0.74f) - _e2166));
                    let _e2172 = select(_e2170, 0f, (_e2170 < 0f));
                    let _e2174 = select(_e2172, 1f, (_e2172 > 1f));
                    phi_97_ = _e2160;
                    phi_98_ = (((_e2174 * _e2174) * (3f - (2f * _e2174))) * _e2162);
                    phi_99_ = _e2164;
                    phi_100_ = vec2<f32>(_e287, _e285);
                } else {
                    phi_97_ = u32();
                    phi_98_ = f32();
                    phi_99_ = vec3<f32>();
                    phi_100_ = vec2<f32>();
                }
                let _e2181 = phi_97_;
                let _e2183 = phi_98_;
                let _e2185 = phi_99_;
                let _e2187 = phi_100_;
                phi_101_ = _e2181;
                phi_102_ = _e2183;
                phi_103_ = _e2185;
                phi_104_ = _e2187;
                phi_105_ = select(true, false, _e2025);
            } else {
                phi_101_ = u32();
                phi_102_ = f32();
                phi_103_ = vec3<f32>();
                phi_104_ = vec2<f32>();
                phi_105_ = true;
            }
            let _e2190 = phi_101_;
            let _e2192 = phi_102_;
            let _e2194 = phi_103_;
            let _e2196 = phi_104_;
            let _e2198 = phi_105_;
            let _e2199 = select(_e2190, 0u, _e2198);
            let _e2202 = select(_e2194, vec3<f32>(0.94f, 0.94f, 0.94f), vec3(_e2198));
            let _e2204 = select(_e2196, vec2<f32>(_e238, _e239), vec2(_e2198));
            if (_e2199 < 76u) {
            } else {
                break;
            }
            let _e2215 = pill_2.member[_e220].text.lines[_e2199].min[0u];
            let _e2223 = pill_2.member[_e220].text.lines[_e2199].min[1u];
            let _e2231 = pill_2.member[_e220].text.lines[_e2199].max[0u];
            let _e2239 = pill_2.member[_e220].text.lines[_e2199].max[1u];
            let _e2247 = pill_2.member[_e220].text.lines[_e2199].origin[0u];
            let _e2255 = pill_2.member[_e220].text.lines[_e2199].origin[1u];
            let _e2262 = pill_2.member[_e220].text.lines[_e2199].size;
            let _e2269 = pill_2.member[_e220].text.lines[_e2199].weight;
            let _e2276 = pill_2.member[_e220].text.lines[_e2199].count;
            let _e2283 = pill_2.member[_e220].text.lines[_e2199].first;
            if (_e2204.x < _e2215) {
                phi_144_ = f32();
                phi_145_ = true;
            } else {
                if (_e2204.x > _e2231) {
                    phi_142_ = f32();
                    phi_143_ = true;
                } else {
                    if (_e2204.y < _e2223) {
                        phi_140_ = f32();
                        phi_141_ = true;
                    } else {
                        let _e2287 = (_e2204.y > _e2239);
                        if _e2287 {
                            phi_139_ = f32();
                        } else {
                            phi_106_ = _e2276;
                            phi_107_ = 0u;
                            loop {
                                let _e2289 = phi_106_;
                                let _e2291 = phi_107_;
                                local_46 = _e2291;
                                let _e2292 = (_e2291 < _e2289);
                                if _e2292 {
                                    let _e2295 = (_e2291 + ((_e2289 - _e2291) / 2u));
                                    let _e2296 = (_e2283 + _e2295);
                                    if (_e2296 < 512u) {
                                    } else {
                                        phi_111_ = true;
                                        break;
                                    }
                                    let _e2304 = pill_2.member[_e220].text.glyphs[_e2296].x;
                                    let _e2307 = (_e2304 <= ((_e2204.x - _e2247) / _e2262));
                                    if _e2307 {
                                        phi_108_ = (_e2295 + 1u);
                                    } else {
                                        phi_108_ = _e2291;
                                    }
                                    let _e2310 = phi_108_;
                                    phi_109_ = select(_e2295, _e2289, _e2307);
                                    phi_110_ = _e2310;
                                } else {
                                    phi_109_ = u32();
                                    phi_110_ = u32();
                                }
                                let _e2313 = phi_109_;
                                let _e2315 = phi_110_;
                                continue;
                                continuing {
                                    phi_106_ = _e2313;
                                    phi_107_ = _e2315;
                                    phi_111_ = _e457;
                                    break if !(_e2292);
                                }
                            }
                            let _e2318 = phi_111_;
                            if _e2318 {
                                break;
                            }
                            let _e2320 = local_46;
                            let _e2321 = (_e2320 + 1u);
                            phi_112_ = _e2318;
                            phi_113_ = select(_e2321, _e2276, (_e2276 < _e2321));
                            phi_114_ = -1000000f;
                            loop {
                                let _e2325 = phi_112_;
                                let _e2327 = phi_113_;
                                let _e2329 = phi_114_;
                                local_53 = _e2329;
                                if (_e2327 > 0u) {
                                    let _e2331 = (_e2327 - 1u);
                                    let _e2332 = (_e2283 + _e2331);
                                    if (_e2332 < 512u) {
                                    } else {
                                        phi_138_ = true;
                                        break;
                                    }
                                    let _e2340 = pill_2.member[_e220].text.glyphs[_e2332].x;
                                    let _e2347 = pill_2.member[_e220].text.glyphs[_e2332].glyph;
                                    if (_e2347 < arrayLength((&glyphs.member))) {
                                    } else {
                                        phi_138_ = true;
                                        break;
                                    }
                                    let _e2353 = glyphs.member[_e2347].min[0u];
                                    let _e2358 = glyphs.member[_e2347].min[1u];
                                    let _e2363 = glyphs.member[_e2347].max[0u];
                                    let _e2368 = glyphs.member[_e2347].max[1u];
                                    let _e2372 = glyphs.member[_e2347].start;
                                    let _e2376 = glyphs.member[_e2347].count;
                                    let _e2379 = (((_e2204.x - _e2247) / _e2262) - _e2340);
                                    let _e2382 = (-((_e2204.y - _e2255)) / _e2262);
                                    let _e2383 = (3.5f / _e2262);
                                    let _e2384 = (_e2363 + _e2383);
                                    let _e2385 = (_e2379 > _e2384);
                                    if _e2385 {
                                        phi_132_ = _e2325;
                                        phi_133_ = f32();
                                    } else {
                                        if (_e2379 >= (_e2353 - _e2383)) {
                                            if (_e2382 >= (_e2358 - _e2383)) {
                                                if (_e2379 <= _e2384) {
                                                    if (_e2382 <= (_e2368 + _e2383)) {
                                                        phi_115_ = 0u;
                                                        phi_116_ = 0i;
                                                        phi_117_ = 340282350000000000000000000000000000000f;
                                                        loop {
                                                            let _e2394 = phi_115_;
                                                            let _e2396 = phi_116_;
                                                            let _e2398 = phi_117_;
                                                            local_47 = _e2398;
                                                            local_48 = _e2396;
                                                            let _e2399 = (_e2394 < _e2376);
                                                            if _e2399 {
                                                                let _e2400 = (_e2372 + _e2394);
                                                                if (_e2400 < arrayLength((&edges.member))) {
                                                                } else {
                                                                    phi_122_ = true;
                                                                    break;
                                                                }
                                                                let _e2404 = edges.member[_e2400];
                                                                let _e2406 = cantus_render_text_edge_distance(_e2404, _e2269, vec2<f32>(_e2379, _e2382));
                                                                if (_e2398 != _e2398) {
                                                                    phi_118_ = true;
                                                                } else {
                                                                    phi_118_ = (_e2406.member <= _e2398);
                                                                }
                                                                let _e2412 = phi_118_;
                                                                phi_119_ = (_e2394 + 1u);
                                                                phi_120_ = (_e2396 + _e2406.member_1);
                                                                phi_121_ = select(_e2398, _e2406.member, _e2412);
                                                            } else {
                                                                phi_119_ = u32();
                                                                phi_120_ = i32();
                                                                phi_121_ = f32();
                                                            }
                                                            let _e2417 = phi_119_;
                                                            let _e2419 = phi_120_;
                                                            let _e2421 = phi_121_;
                                                            continue;
                                                            continuing {
                                                                phi_115_ = _e2417;
                                                                phi_116_ = _e2419;
                                                                phi_117_ = _e2421;
                                                                phi_122_ = _e2325;
                                                                break if !(_e2399);
                                                            }
                                                        }
                                                        let _e2424 = phi_122_;
                                                        phi_138_ = _e2424;
                                                        if _e2424 {
                                                            break;
                                                        }
                                                        let _e2426 = local_47;
                                                        let _e2430 = local_48;
                                                        let _e2433 = ((sqrt(_e2426) * _e2262) * select(1f, -1f, (_e2430 == 0i)));
                                                        if (_e2329 != _e2329) {
                                                            phi_123_ = true;
                                                        } else {
                                                            phi_123_ = (_e2433 >= _e2329);
                                                        }
                                                        let _e2437 = phi_123_;
                                                        phi_124_ = _e2424;
                                                        phi_125_ = select(_e2329, _e2433, _e2437);
                                                    } else {
                                                        phi_124_ = _e2325;
                                                        phi_125_ = _e2329;
                                                    }
                                                    let _e2440 = phi_124_;
                                                    let _e2442 = phi_125_;
                                                    phi_126_ = _e2440;
                                                    phi_127_ = _e2442;
                                                } else {
                                                    phi_126_ = _e2325;
                                                    phi_127_ = _e2329;
                                                }
                                                let _e2444 = phi_126_;
                                                let _e2446 = phi_127_;
                                                phi_128_ = _e2444;
                                                phi_129_ = _e2446;
                                            } else {
                                                phi_128_ = _e2325;
                                                phi_129_ = _e2329;
                                            }
                                            let _e2448 = phi_128_;
                                            let _e2450 = phi_129_;
                                            phi_130_ = _e2448;
                                            phi_131_ = _e2450;
                                        } else {
                                            phi_130_ = _e2325;
                                            phi_131_ = _e2329;
                                        }
                                        let _e2452 = phi_130_;
                                        let _e2454 = phi_131_;
                                        phi_132_ = _e2452;
                                        phi_133_ = _e2454;
                                    }
                                    let _e2456 = phi_132_;
                                    let _e2458 = phi_133_;
                                    phi_134_ = _e2456;
                                    phi_135_ = _e2331;
                                    phi_136_ = _e2458;
                                    phi_137_ = select(true, false, _e2385);
                                } else {
                                    phi_134_ = _e2325;
                                    phi_135_ = u32();
                                    phi_136_ = f32();
                                    phi_137_ = false;
                                }
                                let _e2461 = phi_134_;
                                let _e2463 = phi_135_;
                                let _e2465 = phi_136_;
                                let _e2467 = phi_137_;
                                continue;
                                continuing {
                                    phi_112_ = _e2461;
                                    phi_113_ = _e2463;
                                    phi_114_ = _e2465;
                                    phi_138_ = _e2461;
                                    break if !(_e2467);
                                }
                            }
                            let _e2470 = phi_138_;
                            if _e2470 {
                                break;
                            }
                            let _e2692 = local_53;
                            phi_139_ = _e2692;
                        }
                        let _e2472 = phi_139_;
                        phi_140_ = _e2472;
                        phi_141_ = _e2287;
                    }
                    let _e2474 = phi_140_;
                    let _e2476 = phi_141_;
                    phi_142_ = _e2474;
                    phi_143_ = _e2476;
                }
                let _e2478 = phi_142_;
                let _e2480 = phi_143_;
                phi_144_ = _e2478;
                phi_145_ = _e2480;
            }
            let _e2482 = phi_144_;
            let _e2484 = phi_145_;
            let _e2487 = ((select(_e2482, -1000000f, _e2484) * 1.25f) + 0.5f);
            let _e2489 = select(_e2487, 0f, (_e2487 < 0f));
            let _e2491 = select(_e2489, 1f, (_e2489 > 1f));
            let _e2496 = (((_e2491 * _e2491) * (3f - (2f * _e2491))) * select(_e2192, 1f, _e2198));
            let _e2497 = (1f - _e2496);
            let _e2510 = ((_e2023.x * _e2497) + (_e2202.x * _e2496));
            let _e2511 = ((_e2023.y * _e2497) + (_e2202.y * _e2496));
            let _e2512 = ((_e2023.z * _e2497) + (_e2202.z * _e2496));
            let _e2520 = local_49;
            let _e2521 = (1f - _e2520);
            let _e2526 = local_50;
            let _e2529 = local_51;
            let _e2532 = local_52;
            out_color = vec4<f32>((((_e2510 * _e2521) + (((_e2510 * 1.5f) + 0.1f) * _e2526)) * _e621), (((_e2511 * _e2521) + (((_e2511 * 1.5f) + 0.1f) * _e2529)) * _e621), (((_e2512 * _e2521) + (((_e2512 * 1.5f) + 0.1f) * _e2532)) * _e621), _e634);
            break;
        }
    }
    return;
}

@vertex
fn render_track_vertex(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    vertex_5 = vertex;
    instance_1 = instance;
    render_track_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel_pos;
    let _e9 = out_pill_idx;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_track_fragment(@location(0) pixel_pos: vec2<f32>, @location(1) @interpolate(flat) pill_idx: u32) -> @location(0) vec4<f32> {
    pixel_pos_1 = pixel_pos;
    pill_idx_1 = pill_idx;
    render_track_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_status_vertex(@builtin(vertex_index) vertex_1: u32, @builtin(instance_index) _isthmus_instance_index: u32) -> VertexOutput {
    vertex_5 = vertex_1;
    _isthmus_instance_index_7 = _isthmus_instance_index;
    render_status_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_status_fragment(@location(0) pixel: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_1: u32) -> @location(0) vec4<f32> {
    pixel_2 = pixel;
    _isthmus_instance_index_8 = _isthmus_instance_index_1;
    render_status_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_playhead_vertex(@builtin(vertex_index) vertex_2: u32, @builtin(instance_index) _isthmus_instance_index_2: u32) -> VertexOutput {
    vertex_5 = vertex_2;
    _isthmus_instance_index_7 = _isthmus_instance_index_2;
    render_playhead_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_world_pos;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_playhead_fragment(@location(0) world_pos: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_3: u32) -> @location(0) vec4<f32> {
    world_pos_1 = world_pos;
    _isthmus_instance_index_8 = _isthmus_instance_index_3;
    render_playhead_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_particles_vertex(@builtin(vertex_index) vertex_3: u32, @builtin(instance_index) _isthmus_instance_index_4: u32) -> VertexOutput_1 {
    vertex_5 = vertex_3;
    _isthmus_instance_index_7 = _isthmus_instance_index_4;
    render_particles_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_color;
    let _e9 = out_uv;
    return VertexOutput_1(_e7, _e8, _e9);
}

@fragment
fn render_particles_fragment(@location(0) color: vec4<f32>, @location(1) uv: vec2<f32>) -> @location(0) vec4<f32> {
    color_1 = color;
    uv_1 = uv;
    render_particles_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_tempestas_vertex(@builtin(vertex_index) vertex_4: u32, @builtin(instance_index) _isthmus_instance_index_5: u32) -> VertexOutput_2 {
    vertex_5 = vertex_4;
    _isthmus_instance_index_7 = _isthmus_instance_index_5;
    render_tempestas_vertex_impl();
    let _e8 = out_position;
    let _e9 = out_pixel;
    let _e10 = out_weather;
    let _e11 = out_isthmus_instance_index_1;
    return VertexOutput_2(_e8, _e9, _e10, _e11);
}

@fragment
fn render_tempestas_fragment(@location(0) pixel_1: vec2<f32>, @location(1) @interpolate(flat) weather: vec4<f32>, @location(2) @interpolate(flat) _isthmus_instance_index_6: u32) -> @location(0) vec4<f32> {
    pixel_2 = pixel_1;
    weather_1 = weather;
    _isthmus_instance_index_9 = _isthmus_instance_index_6;
    render_tempestas_fragment_impl();
    let _e7 = out_color;
    return _e7;
}
