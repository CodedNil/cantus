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
    color: u32,
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
    lines: array<render_text_Line, 2>,
}

struct type_12 {
    member: array<render_track_TrackPill>,
}

struct render_text_PlacedGlyph {
    x: f32,
    glyph: u32,
}

struct type_15 {
    member: array<render_text_PlacedGlyph>,
}

struct render_text_Glyph {
    min: vec2<f32>,
    max: vec2<f32>,
    start: u32,
    count: u32,
}

struct type_17 {
    member: array<render_text_Glyph>,
}

struct render_text_Edge {
    start: vec2<f32>,
    control: vec2<f32>,
    end: vec2<f32>,
    start_delta: vec2<f32>,
    control_delta: vec2<f32>,
    end_delta: vec2<f32>,
}

struct type_19 {
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

struct core_ops_Range_f32_ {
    start: f32,
    end: f32,
}

struct type_24 {
    member: array<render_text_Line>,
}

struct isthmus_Vertex_render_text_Varyings {
    position: vec4<f32>,
    varyings: vec2<f32>,
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
    labels: array<render_text_Line, 2>,
}

struct type_28 {
    member: array<render_status_StatusPill>,
}

struct render_playhead_PlayheadState {
    bar_split: f32,
    icon_presence: f32,
    icon_morph: f32,
}

struct type_30 {
    member: array<render_playhead_PlayheadState>,
}

struct render_particles_Particle {
    spawn_pos: vec2<f32>,
    spawn_vel: vec2<f32>,
    end_time: f32,
    duration: f32,
    rgb: u32,
}

struct type_32 {
    member: array<render_particles_Particle>,
}

struct isthmus_Vertex_render_particles_Varyings {
    varyings: isthmus_Vertex_render_text_Varyings,
    position: vec4<f32>,
}

struct render_tempestas_WeatherSurface {
    x: f32,
    calendar_expansion: f32,
    sun_hours: array<f32, 2>,
    hourly_start: f32,
    text_hover: array<f32, 3>,
    hourly_conditions: array<render_tempestas_WeatherCondition, 6>,
    daily_conditions: array<render_tempestas_WeatherCondition, 5>,
}

struct type_38 {
    member: array<render_tempestas_WeatherSurface>,
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

var<private> vertex_6: u32;
var<private> instance_1: u32;
@group(0) @binding(0)
var<storage> frame: type_6;
@group(0) @binding(1)
var<storage> pill: type_12;
var<private> out_position: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> out_pixel_pos: vec2<f32>;
var<private> out_pill_idx: u32;
var<private> pixel_pos_1: vec2<f32>;
var<private> pill_idx_1: u32;
@group(0) @binding(4)
var<storage> placed_glyphs: type_15;
@group(0) @binding(5)
var<storage> glyphs: type_17;
@group(0) @binding(6)
var<storage> edges: type_19;
var<private> global: vec2<f32> = vec2<f32>(0f, 0f);
@group(0) @binding(3)
var sampler_: sampler;
@group(0) @binding(2)
var images: texture_2d_array<f32>;
var<private> global_1: core_ops_Range_f32_ = core_ops_Range_f32_(0f, 1f);
var<private> out_color: vec4<f32>;
@group(0) @binding(1)
var<storage> line: type_24;
var<private> _isthmus_instance_index_9: u32;
var<private> out_pixel: vec2<f32>;
var<private> out_isthmus_instance_index: u32;
var<private> pixel_3: vec2<f32>;
var<private> _isthmus_instance_index_10: u32;
@group(0) @binding(2)
var<storage> placed_glyphs_1: type_15;
@group(0) @binding(3)
var<storage> glyphs_1: type_17;
@group(0) @binding(4)
var<storage> edges_1: type_19;
@group(0) @binding(1)
var<storage> pill_1: type_28;
var<private> out_world_pos: vec2<f32>;
var<private> world_pos_1: vec2<f32>;
@group(0) @binding(1)
var<storage> state: type_30;
@group(0) @binding(1)
var<storage> particle: type_32;
var<private> out_uv: vec2<f32>;
var<private> color_1: vec4<f32>;
var<private> uv_1: vec2<f32>;
@group(0) @binding(1)
var<storage> pill_2: type_38;
var<private> out_weather: vec4<f32>;
var<private> out_isthmus_instance_index_1: u32;
var<private> weather_1: vec4<f32>;
var<private> _isthmus_instance_index_11: u32;
@group(0) @binding(2)
var<storage> text_lines: type_24;
@group(0) @binding(3)
var<storage> placed_glyphs_2: type_15;
@group(0) @binding(4)
var<storage> glyphs_2: type_17;
@group(0) @binding(5)
var<storage> edges_2: type_19;

fn cantus_render_shader_pixel_to_ndc(param: vec2<f32>, param_1: vec2<f32>) -> vec4<f32> {
    return vec4<f32>((((param.x / param_1.x) * 2f) - 1f), (1f - ((param.y / param_1.y) * 2f)), 0f, 1f);
}

fn render_track_isthmus_trackpass_vertex_impl() {
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
            let _e32 = vertex_6;
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

fn cantus_render_text_curve_at(param_2: vec2<f32>, param_3: vec2<f32>, param_4: vec2<f32>, param_5: f32) -> vec2<f32> {
    return (param_2 + vec2<f32>(((param_3.x + (param_4.x * param_5)) * param_5), ((param_3.y + (param_4.y * param_5)) * param_5)));
}

fn cantus_render_text_ray_crossing(param_6: vec2<f32>, param_7: vec2<f32>, param_8: vec2<f32>, param_9: vec2<f32>, param_10: f32) -> i32 {
    var phi_0_: bool;
    var phi_1_: i32;
    var phi_2_: i32;
    var phi_3_: i32;

    let _e19 = global_1.start;
    if (_e19 <= param_10) {
        let _e22 = global_1.end;
        phi_0_ = (param_10 < _e22);
    } else {
        phi_0_ = false;
    }
    let _e25 = phi_0_;
    if _e25 {
        if ((param_6.x + ((param_7.x + (param_8.x * param_10)) * param_10)) <= param_9.x) {
            phi_2_ = 0i;
        } else {
            let _e33 = (param_7.y + (param_8.y * (2f * param_10)));
            if (_e33 > 0f) {
                phi_1_ = 1i;
            } else {
                phi_1_ = select(0i, -1i, (_e33 < 0f));
            }
            let _e38 = phi_1_;
            phi_2_ = _e38;
        }
        let _e40 = phi_2_;
        phi_3_ = _e40;
    } else {
        phi_3_ = 0i;
    }
    let _e42 = phi_3_;
    return _e42;
}

fn cantus_render_text_edge_distance(param_11: render_text_Edge, param_12: f32, param_13: vec2<f32>, param_14: f32) -> u0028_f32_u0020_i32_u0029_ {
    var phi_0_: i32;
    var phi_1_: i32;
    var phi_2_: i32;
    var phi_3_: i32;
    var phi_4_: bool;
    var phi_5_: i32;
    var phi_6_: i32;
    var phi_7_: bool;
    var phi_8_: i32;
    var phi_9_: bool;
    var phi_10_: bool;
    var phi_11_: bool;
    var phi_12_: bool;
    var phi_13_: f32;
    var phi_14_: i32;
    var phi_15_: bool;
    var phi_16_: bool;
    var phi_17_: bool;
    var phi_18_: f32;
    var phi_19_: i32;
    var phi_20_: bool;
    var local_1: f32;
    var phi_21_: bool;
    var phi_22_: u0028_f32_u0020_i32_u0029_;
    var phi_23_: u0028_f32_u0020_i32_u0029_;
    var phi_24_: u0028_f32_u0020_i32_u0029_;
    var phi_25_: u0028_f32_u0020_i32_u0029_;
    var phi_26_: bool;
    var phi_27_: u0028_f32_u0020_i32_u0029_;

    let _e33 = (param_11.start.x + (param_11.start_delta.x * param_12));
    let _e34 = (param_11.start.y + (param_11.start_delta.y * param_12));
    let _e45 = (param_11.control.x + (param_11.control_delta.x * param_12));
    let _e46 = (param_11.control.y + (param_11.control_delta.y * param_12));
    let _e57 = (param_11.end.x + (param_11.end_delta.x * param_12));
    let _e58 = (param_11.end.y + (param_11.end_delta.y * param_12));
    let _e60 = (_e46 - _e34);
    let _e61 = ((_e45 - _e33) * 2f);
    let _e62 = (_e60 * 2f);
    let _e67 = ((_e33 - (_e45 * 2f)) + _e57);
    let _e68 = ((_e34 - (_e46 * 2f)) + _e58);
    let _e70 = select(_e45, _e33, (_e33 < _e45));
    let _e72 = select(_e46, _e34, (_e34 < _e46));
    let _e74 = select(_e57, _e70, (_e70 < _e57));
    let _e76 = select(_e58, _e72, (_e72 < _e58));
    let _e78 = select(_e45, _e33, (_e33 > _e45));
    let _e80 = select(_e46, _e34, (_e34 > _e46));
    let _e82 = select(_e57, _e78, (_e78 > _e57));
    let _e84 = select(_e58, _e80, (_e80 > _e58));
    if (param_13.x >= _e82) {
        phi_8_ = i32();
        phi_9_ = true;
    } else {
        if (param_13.y < _e76) {
            phi_6_ = i32();
            phi_7_ = true;
        } else {
            let _e87 = (param_13.y >= _e84);
            if _e87 {
                phi_5_ = i32();
            } else {
                let _e88 = (_e34 - param_13.y);
                if (abs(_e68) < 0.0000001f) {
                    if (abs(_e62) < 0.0000001f) {
                        phi_1_ = 0i;
                    } else {
                        let _e118 = cantus_render_text_ray_crossing(vec2<f32>(_e33, _e34), vec2<f32>(_e61, _e62), vec2<f32>(_e67, _e68), param_13, (-(_e88) / _e62));
                        phi_1_ = _e118;
                    }
                    let _e120 = phi_1_;
                    phi_2_ = _e120;
                    phi_3_ = i32();
                    phi_4_ = true;
                } else {
                    let _e94 = ((_e62 * _e62) - ((4f * _e68) * _e88));
                    let _e95 = (_e94 <= 0f);
                    if _e95 {
                        phi_0_ = i32();
                    } else {
                        let _e96 = sqrt(_e94);
                        let _e97 = (_e68 * 2f);
                        let _e98 = (_e60 * -2f);
                        let _e101 = vec2<f32>(_e33, _e34);
                        let _e102 = vec2<f32>(_e61, _e62);
                        let _e103 = vec2<f32>(_e67, _e68);
                        let _e104 = cantus_render_text_ray_crossing(_e101, _e102, _e103, param_13, ((_e98 - _e96) / _e97));
                        let _e107 = cantus_render_text_ray_crossing(_e101, _e102, _e103, param_13, ((_e98 + _e96) / _e97));
                        phi_0_ = (_e104 + _e107);
                    }
                    let _e110 = phi_0_;
                    phi_2_ = 0i;
                    phi_3_ = _e110;
                    phi_4_ = _e95;
                }
                let _e122 = phi_2_;
                let _e124 = phi_3_;
                let _e126 = phi_4_;
                phi_5_ = select(_e124, _e122, _e126);
            }
            let _e129 = phi_5_;
            phi_6_ = _e129;
            phi_7_ = _e87;
        }
        let _e131 = phi_6_;
        let _e133 = phi_7_;
        phi_8_ = _e131;
        phi_9_ = _e133;
    }
    let _e135 = phi_8_;
    let _e137 = phi_9_;
    let _e138 = select(_e135, 0i, _e137);
    let _e140 = select(_e74, param_13.x, (param_13.x > _e74));
    let _e142 = select(_e76, param_13.y, (param_13.y > _e76));
    let _e147 = (param_13.x - select(_e82, _e140, (_e140 < _e82)));
    let _e148 = (param_13.y - select(_e84, _e142, (_e142 < _e84)));
    if (((_e147 * _e147) + (_e148 * _e148)) >= param_14) {
        phi_24_ = u0028_f32_u0020_i32_u0029_(param_14, _e138);
        phi_25_ = u0028_f32_u0020_i32_u0029_();
        phi_26_ = true;
    } else {
        let _e153 = (_e57 - _e33);
        let _e154 = (_e58 - _e34);
        let _e155 = (param_13.x - _e33);
        let _e156 = (param_13.y - _e34);
        let _e162 = ((_e153 * _e153) + (_e154 * _e154));
        if (_e162 != _e162) {
            phi_10_ = true;
        } else {
            phi_10_ = (0.00000001f >= _e162);
        }
        let _e166 = phi_10_;
        let _e168 = (((_e155 * _e153) + (_e156 * _e154)) / select(_e162, 0.00000001f, _e166));
        if (_e168 != _e168) {
            phi_11_ = true;
        } else {
            phi_11_ = (0f >= _e168);
        }
        let _e172 = phi_11_;
        let _e173 = select(_e168, 0f, _e172);
        if (_e173 != _e173) {
            phi_12_ = true;
        } else {
            phi_12_ = (1f <= _e173);
        }
        let _e177 = phi_12_;
        let _e178 = select(_e173, 1f, _e177);
        let _e182 = (((_e67 * _e67) + (_e68 * _e68)) < 0.000000000001f);
        if _e182 {
            let _e278 = (param_13.x - (_e33 + (_e61 * _e178)));
            let _e279 = (param_13.y - (_e34 + (_e62 * _e178)));
            phi_22_ = u0028_f32_u0020_i32_u0029_(((_e278 * _e278) + (_e279 * _e279)), _e138);
            phi_23_ = u0028_f32_u0020_i32_u0029_();
        } else {
            let _e183 = (_e67 * 2f);
            let _e184 = (_e68 * 2f);
            phi_13_ = _e178;
            phi_14_ = 0i;
            loop {
                let _e186 = phi_13_;
                let _e188 = phi_14_;
                local_1 = _e186;
                let _e189 = (_e188 < 2i);
                if _e189 {
                    let _e193 = cantus_render_text_curve_at(vec2<f32>(_e33, _e34), vec2<f32>(_e61, _e62), vec2<f32>(_e67, _e68), _e186);
                    let _e198 = (_e61 + (_e183 * _e186));
                    let _e199 = (_e62 + (_e184 * _e186));
                    let _e200 = (_e193.x - param_13.x);
                    let _e201 = (_e193.y - param_13.y);
                    let _e208 = (((_e198 * _e198) + (_e199 * _e199)) + ((_e200 * _e183) + (_e201 * _e184)));
                    let _e209 = abs(_e208);
                    if (_e209 != _e209) {
                        phi_15_ = true;
                    } else {
                        phi_15_ = (0.00000001f >= _e209);
                    }
                    let _e213 = phi_15_;
                    let _e225 = (_e186 - (((_e200 * _e198) + (_e201 * _e199)) / bitcast<f32>(((bitcast<u32>(select(_e209, 0.00000001f, _e213)) & 2147483647u) | (bitcast<u32>(_e208) & 2147483648u)))));
                    if (_e225 != _e225) {
                        phi_16_ = true;
                    } else {
                        phi_16_ = (0f >= _e225);
                    }
                    let _e229 = phi_16_;
                    let _e230 = select(_e225, 0f, _e229);
                    if (_e230 != _e230) {
                        phi_17_ = true;
                    } else {
                        phi_17_ = (1f <= _e230);
                    }
                    let _e234 = phi_17_;
                    phi_18_ = select(_e230, 1f, _e234);
                    phi_19_ = (_e188 + 1i);
                } else {
                    phi_18_ = f32();
                    phi_19_ = i32();
                }
                let _e238 = phi_18_;
                let _e240 = phi_19_;
                continue;
                continuing {
                    phi_13_ = _e238;
                    phi_14_ = _e240;
                    break if !(_e189);
                }
            }
            let _e244 = ((_e155 * _e155) + (_e156 * _e156));
            let _e245 = (param_13.x - _e57);
            let _e246 = (param_13.y - _e58);
            let _e249 = ((_e245 * _e245) + (_e246 * _e246));
            if (_e244 != _e244) {
                phi_20_ = true;
            } else {
                phi_20_ = (_e249 <= _e244);
            }
            let _e253 = phi_20_;
            let _e254 = select(_e244, _e249, _e253);
            let _e259 = local_1;
            let _e260 = cantus_render_text_curve_at(vec2<f32>(_e33, _e34), vec2<f32>(_e61, _e62), vec2<f32>(_e67, _e68), _e259);
            let _e263 = (param_13.x - _e260.x);
            let _e264 = (param_13.y - _e260.y);
            let _e267 = ((_e263 * _e263) + (_e264 * _e264));
            if (_e254 != _e254) {
                phi_21_ = true;
            } else {
                phi_21_ = (_e267 <= _e254);
            }
            let _e271 = phi_21_;
            phi_22_ = u0028_f32_u0020_i32_u0029_();
            phi_23_ = u0028_f32_u0020_i32_u0029_(select(_e254, _e267, _e271), _e138);
        }
        let _e285 = phi_22_;
        let _e287 = phi_23_;
        phi_24_ = _e285;
        phi_25_ = _e287;
        phi_26_ = _e182;
    }
    let _e290 = phi_24_;
    let _e292 = phi_25_;
    let _e294 = phi_26_;
    if _e294 {
        phi_27_ = _e290;
    } else {
        phi_27_ = _e292;
    }
    let _e296 = phi_27_;
    return _e296;
}

fn cantus_render_shader_hash(param_15: vec2<f32>) -> vec2<f32> {
    let _e31 = ((bitcast<u32>(select(0i, select(select(i32(param_15.y), i32(-2147483648), (param_15.y < -2147483600f)), 2147483647i, (param_15.y > 2147483500f)), (param_15.y == param_15.y))) * 1664525u) + 1013904223u);
    let _e33 = (((bitcast<u32>(select(0i, select(select(i32(param_15.x), i32(-2147483648), (param_15.x < -2147483600f)), 2147483647i, (param_15.x > 2147483500f)), (param_15.x == param_15.x))) * 1664525u) + 1013904223u) + (_e31 * 1664525u));
    let _e35 = (_e31 + (_e33 * 1664525u));
    let _e41 = (_e35 ^ (_e35 >> bitcast<u32>(16i)));
    let _e43 = ((_e33 ^ (_e33 >> bitcast<u32>(16i))) + (_e41 * 1664525u));
    let _e45 = (_e41 + (_e43 * 1664525u));
    return vec2<f32>((f32((_e43 ^ (_e43 >> bitcast<u32>(16i)))) * 0.00000000023283064f), (f32((_e45 ^ (_e45 >> bitcast<u32>(16i)))) * 0.00000000023283064f));
}

fn cantus_render_shader_simplex_noise(param_16: vec2<f32>) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;
    var phi_2_: bool;

    let _e15 = ((param_16.x + param_16.y) * 0.36602542f);
    let _e18 = floor((param_16.x + _e15));
    let _e19 = floor((param_16.y + _e15));
    let _e23 = ((_e18 + _e19) * 0.21132487f);
    let _e24 = ((param_16.x - _e18) + _e23);
    let _e25 = ((param_16.y - _e19) + _e23);
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

fn cantus_render_track_plasma_field(param_17: vec2<f32>, param_18: vec4<f32>, param_19: f32, param_20: f32, param_21: f32) -> vec4<f32> {
    let _e17 = ((sin((((param_17.x * param_19) + (param_17.y * param_20)) + param_21)) * 0.5f) + 0.5f);
    let _e23 = ((0.12f + (_e17 * _e17)) * (0.25f + (param_18.w * 3f)));
    return vec4<f32>((param_18.x * _e23), (param_18.y * _e23), (param_18.z * _e23), _e23);
}

fn cantus_render_shader_sd_capsule_box(param_22: vec2<f32>, param_23: f32, param_24: f32) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;

    let _e8 = abs(param_22.y);
    let _e9 = (abs(param_22.x) - param_23);
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
    return ((sqrt(((_e11 * _e11) + (_e13 * _e13))) + select(_e22, 0f, _e26)) - param_24);
}

fn render_track_isthmus_trackpass_fragment_impl() {
    var phi_0_: f32;
    var phi_1_: vec2<f32>;
    var phi_2_: f32;
    var phi_3_: u32;
    var phi_4_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_5_: bool;
    var phi_6_: vec2<f32>;
    var phi_7_: f32;
    var phi_8_: vec2<f32>;
    var phi_9_: f32;
    var phi_10_: vec2<f32>;
    var phi_11_: f32;
    var phi_12_: u32;
    var phi_13_: bool;
    var phi_14_: f32;
    var local_2: vec2<f32>;
    var local_3: vec2<f32>;
    var phi_15_: bool;
    var local_4: vec2<f32>;
    var phi_16_: f32;
    var local_5: vec2<f32>;
    var phi_17_: bool;
    var phi_18_: bool;
    var phi_19_: bool;
    var phi_20_: bool;
    var phi_21_: bool;
    var phi_22_: bool;
    var phi_23_: bool;
    var phi_24_: bool;
    var phi_25_: bool;
    var phi_26_: bool;
    var phi_27_: bool;
    var phi_28_: bool;
    var phi_29_: bool;
    var phi_30_: bool;
    var phi_31_: bool;
    var phi_32_: f32;
    var phi_33_: vec3<f32>;
    var phi_34_: vec3<f32>;
    var local_6: f32;
    var local_7: f32;
    var local_8: f32;
    var local_9: f32;
    var phi_35_: vec4<f32>;
    var phi_36_: u32;
    var phi_37_: bool;
    var phi_38_: bool;
    var phi_39_: bool;
    var phi_40_: bool;
    var phi_41_: bool;
    var phi_42_: vec4<f32>;
    var phi_43_: vec4<f32>;
    var phi_44_: u32;
    var phi_45_: vec4<f32>;
    var phi_46_: vec4<f32>;
    var phi_47_: vec4<f32>;
    var phi_48_: u32;
    var phi_49_: render_shared_RipplePulse;
    var phi_50_: f32;
    var phi_51_: bool;
    var phi_52_: bool;
    var phi_53_: bool;
    var phi_54_: bool;
    var phi_55_: bool;
    var phi_56_: vec4<f32>;
    var phi_57_: vec4<f32>;
    var phi_58_: vec4<f32>;
    var phi_59_: vec4<f32>;
    var phi_60_: vec4<f32>;
    var phi_61_: u32;
    var phi_62_: bool;
    var phi_63_: u32;
    var phi_64_: u32;
    var phi_65_: u32;
    var phi_66_: u32;
    var phi_67_: u32;
    var local_10: u32;
    var phi_68_: u32;
    var phi_69_: f32;
    var phi_70_: f32;
    var phi_71_: u32;
    var phi_72_: i32;
    var phi_73_: f32;
    var phi_74_: u32;
    var phi_75_: i32;
    var local_11: f32;
    var phi_76_: f32;
    var local_12: i32;
    var phi_77_: bool;
    var phi_78_: f32;
    var phi_79_: f32;
    var phi_80_: f32;
    var phi_81_: f32;
    var phi_82_: f32;
    var phi_83_: u32;
    var phi_84_: f32;
    var phi_85_: bool;
    var phi_86_: f32;
    var phi_87_: f32;
    var phi_88_: bool;
    var phi_89_: f32;
    var phi_90_: bool;
    var phi_91_: f32;
    var phi_92_: bool;
    var phi_93_: u32;
    var phi_94_: u32;
    var phi_95_: u32;
    var phi_96_: u32;
    var phi_97_: u32;
    var local_13: u32;
    var phi_98_: u32;
    var phi_99_: f32;
    var phi_100_: f32;
    var phi_101_: u32;
    var phi_102_: i32;
    var phi_103_: f32;
    var phi_104_: u32;
    var phi_105_: i32;
    var local_14: f32;
    var phi_106_: f32;
    var local_15: i32;
    var phi_107_: bool;
    var phi_108_: f32;
    var phi_109_: f32;
    var phi_110_: f32;
    var phi_111_: f32;
    var phi_112_: f32;
    var phi_113_: u32;
    var phi_114_: f32;
    var phi_115_: bool;
    var phi_116_: f32;
    var phi_117_: f32;
    var phi_118_: bool;
    var phi_119_: f32;
    var phi_120_: bool;
    var phi_121_: f32;
    var phi_122_: bool;
    var phi_123_: bool;
    var local_16: vec4<f32>;
    var local_17: vec4<f32>;
    var local_18: vec4<f32>;
    var local_19: vec4<f32>;
    var local_20: vec4<f32>;
    var local_21: f32;
    var local_22: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e166 = pixel_pos_1;
            let _e167 = pill_idx_1;
            let _e173 = pill.member[_e167].x;
            let _e177 = pill.member[_e167].width;
            let _e181 = frame.member[0u].panel_height;
            let _e182 = (_e166.x - _e173);
            let _e183 = (_e166.y - 6f);
            let _e184 = (_e177 * 0.5f);
            let _e185 = (_e181 * 0.5f);
            let _e187 = (_e183 - _e185);
            let _e188 = (_e177 - _e181);
            let _e189 = (_e188 * 0.5f);
            let _e191 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e182 - _e184), _e187), _e189, _e185);
            let _e195 = frame.member[0u].mouse_pressure;
            let _e196 = (_e195 > 0f);
            if _e196 {
                let _e201 = frame.member[0u].mouse_pos[0u];
                let _e206 = frame.member[0u].mouse_pos[1u];
                let _e212 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e201 - _e173) - _e184), ((_e206 - 6f) - _e185)), _e189, _e185);
                phi_0_ = _e212;
            } else {
                phi_0_ = 1f;
            }
            let _e214 = phi_0_;
            phi_1_ = vec2<f32>(0f, 0f);
            phi_2_ = 0f;
            phi_3_ = 0u;
            loop {
                let _e216 = phi_1_;
                let _e218 = phi_2_;
                let _e220 = phi_3_;
                local_2 = _e216;
                local_3 = _e216;
                local_4 = _e216;
                local_5 = _e216;
                local_6 = _e218;
                local_7 = _e218;
                local_8 = _e218;
                local_9 = _e218;
                let _e221 = (_e220 < 4u);
                if _e221 {
                    if _e221 {
                    } else {
                        phi_13_ = true;
                        break;
                    }
                    let _e228 = frame.member[0u].ripples[_e220].origin[0u];
                    let _e235 = frame.member[0u].ripples[_e220].origin[1u];
                    let _e241 = frame.member[0u].ripples[_e220].start_time;
                    let _e247 = frame.member[0u].ripples[_e220].strength;
                    let _e251 = frame.member[0u].time;
                    let _e253 = ((_e251 - _e241) * 1.2f);
                    let _e255 = select(_e253, 0f, (_e253 < 0f));
                    let _e257 = select(_e255, 1f, (_e255 > 1f));
                    if (_e247 > 0f) {
                        if (_e257 < 1f) {
                            let _e260 = (_e166.x - _e228);
                            let _e261 = (_e166.y - _e235);
                            let _e265 = sqrt(((_e260 * _e260) + (_e261 * _e261)));
                            if (_e265 > 0.001f) {
                                phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e260 / _e265), (_e261 / _e265)), _e265);
                            } else {
                                phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e265);
                            }
                            let _e273 = phi_4_;
                            let _e283 = ((abs((_e273.unnamed_1 - (_e257 * 600f))) - 80f) * -0.0125f);
                            let _e285 = select(_e283, 0f, (_e283 < 0f));
                            let _e287 = select(_e285, 1f, (_e285 > 1f));
                            let _e293 = (1f - _e257);
                            let _e294 = ((((_e287 * _e287) * (3f - (2f * _e287))) * _e247) * _e293);
                            let _e307 = (_e218 + (_e294 * 0.5f));
                            if (_e307 != _e307) {
                                phi_5_ = true;
                            } else {
                                phi_5_ = (1f <= _e307);
                            }
                            let _e311 = phi_5_;
                            phi_6_ = vec2<f32>((_e216.x + (((_e273.unnamed.x * _e294) * _e293) * 0.5f)), (_e216.y + (((_e273.unnamed.y * _e294) * _e293) * 0.5f)));
                            phi_7_ = select(_e307, 1f, _e311);
                        } else {
                            phi_6_ = _e216;
                            phi_7_ = _e218;
                        }
                        let _e314 = phi_6_;
                        let _e316 = phi_7_;
                        phi_8_ = _e314;
                        phi_9_ = _e316;
                    } else {
                        phi_8_ = _e216;
                        phi_9_ = _e218;
                    }
                    let _e318 = phi_8_;
                    let _e320 = phi_9_;
                    phi_10_ = _e318;
                    phi_11_ = _e320;
                    phi_12_ = (_e220 + 1u);
                } else {
                    phi_10_ = vec2<f32>();
                    phi_11_ = f32();
                    phi_12_ = u32();
                }
                let _e323 = phi_10_;
                let _e325 = phi_11_;
                let _e327 = phi_12_;
                continue;
                continuing {
                    phi_1_ = _e323;
                    phi_2_ = _e325;
                    phi_3_ = _e327;
                    phi_13_ = false;
                    break if !(_e221);
                }
            }
            let _e330 = phi_13_;
            if _e330 {
                break;
            }
            if _e196 {
                let _e335 = frame.member[0u].mouse_pos[0u];
                let _e340 = frame.member[0u].mouse_pos[1u];
                let _e341 = (_e166.x - _e335);
                let _e342 = (_e166.y - _e340);
                let _e348 = ((sqrt(((_e341 * _e341) + (_e342 * _e342))) - 150f) * -0.006666667f);
                let _e350 = select(_e348, 0f, (_e348 < 0f));
                let _e352 = select(_e350, 1f, (_e350 > 1f));
                phi_14_ = ((((_e352 * _e352) * (3f - (2f * _e352))) * _e195) * 8f);
            } else {
                phi_14_ = 0f;
            }
            let _e360 = phi_14_;
            let _e362 = local_2;
            let _e365 = global[0u];
            if (_e362.x == _e365) {
                let _e368 = local_3;
                let _e371 = global[1u];
                phi_15_ = (_e368.y == _e371);
            } else {
                phi_15_ = false;
            }
            let _e374 = phi_15_;
            if _e374 {
                phi_16_ = 0f;
            } else {
                let _e376 = local_4;
                phi_16_ = (sqrt(((_e362.x * _e362.x) + (_e376.y * _e376.y))) * 22f);
            }
            let _e384 = phi_16_;
            let _e386 = local_5;
            let _e388 = (_e182 / _e177);
            let _e389 = (_e183 / _e181);
            let _e390 = (_e388 - 0.5f);
            let _e391 = (_e389 - 0.5f);
            let _e392 = (_e173 + _e184);
            let _e393 = (_e181 * 0.975f);
            let _e394 = (_e393 + 3f);
            let _e398 = pill.member[_e167].rating;
            let _e399 = (_e398 >= 0i);
            let _e400 = select(0f, 5f, _e399);
            let _e404 = pill.member[_e167].primary_playlist_count;
            let _e406 = (_e400 + f32(_e404));
            let _e412 = pill.member[_e167].secondary_expansion;
            let _e414 = (_e394 + (18f * _e412));
            let _e418 = pill.member[_e167].secondary_playlist_count;
            let _e419 = f32(_e418);
            let _e426 = frame.member[0u].mouse_pos[0u];
            let _e431 = frame.member[0u].mouse_pos[1u];
            let _e432 = vec2<f32>(_e426, _e431);
            let _e434 = (_e406 - 1f);
            let _e435 = (_e434 != _e434);
            if _e435 {
                phi_17_ = true;
            } else {
                phi_17_ = (0f >= _e434);
            }
            let _e438 = phi_17_;
            let _e441 = vec2<f32>(_e392, (_e393 + -4.4f));
            let _e443 = cantus_render_shader_sd_capsule_box((_e166 - _e441), (select(_e434, 0f, _e438) * 9f), 9f);
            if _e435 {
                phi_18_ = true;
            } else {
                phi_18_ = (0f >= _e434);
            }
            let _e446 = phi_18_;
            let _e450 = cantus_render_shader_sd_capsule_box((_e432 - _e441), (select(_e434, 0f, _e446) * 9f), 9f);
            let _e451 = (10.5f * _e412);
            let _e453 = (_e419 - 1f);
            let _e454 = (_e453 != _e453);
            if _e454 {
                phi_19_ = true;
            } else {
                phi_19_ = (0f >= _e453);
            }
            let _e457 = phi_19_;
            let _e462 = vec2<f32>(_e392, (_e414 + -5.4f));
            let _e464 = cantus_render_shader_sd_capsule_box((_e166 - _e462), (((select(_e453, 0f, _e457) * 18f) * _e412) * 0.5f), _e451);
            if _e454 {
                phi_20_ = true;
            } else {
                phi_20_ = (0f >= _e453);
            }
            let _e467 = phi_20_;
            let _e473 = cantus_render_shader_sd_capsule_box((_e432 - _e462), (((select(_e453, 0f, _e467) * 18f) * _e412) * 0.5f), _e451);
            let _e477 = pill.member[_e167].primary_alpha;
            let _e480 = (0.5f + ((_e443 - _e191) * 0.05f));
            let _e482 = select(_e480, 0f, (_e480 < 0f));
            let _e484 = select(_e482, 1f, (_e482 > 1f));
            let _e494 = (_e191 + ((((_e443 + ((_e191 - _e443) * _e484)) - ((10f * _e484) * (1f - _e484))) - _e191) * _e477));
            let _e497 = (0.5f + ((_e450 - _e214) * 0.05f));
            let _e499 = select(_e497, 0f, (_e497 < 0f));
            let _e501 = select(_e499, 1f, (_e499 > 1f));
            let _e511 = (_e214 + ((((_e450 + ((_e214 - _e450) * _e501)) - ((10f * _e501) * (1f - _e501))) - _e214) * _e477));
            let _e513 = select(0f, 1f, (_e412 > 0f));
            let _e516 = (0.5f + ((_e464 - _e494) * 0.046296295f));
            let _e518 = select(_e516, 0f, (_e516 < 0f));
            let _e520 = select(_e518, 1f, (_e518 > 1f));
            let _e533 = (0.5f + ((_e473 - _e511) * 0.046296295f));
            let _e535 = select(_e533, 0f, (_e533 < 0f));
            let _e537 = select(_e535, 1f, (_e535 > 1f));
            let _e549 = (((_e511 + ((((_e473 + ((_e511 - _e473) * _e537)) - ((10.8f * _e537) * (1f - _e537))) - _e511) * _e513)) - 0.5f) * -1f);
            let _e551 = select(_e549, 0f, (_e549 < 0f));
            let _e553 = select(_e551, 1f, (_e551 > 1f));
            let _e560 = (((_e360 * ((_e553 * _e553) * (3f - (2f * _e553)))) + _e384) * 0.5f);
            let _e561 = ((_e494 + ((((_e464 + ((_e494 - _e464) * _e520)) - ((10.8f * _e520) * (1f - _e520))) - _e494) * _e513)) - _e560);
            let _e562 = fwidth(_e561);
            if (_e562 != _e562) {
                phi_21_ = true;
            } else {
                phi_21_ = (0.55f >= _e562);
            }
            let _e566 = phi_21_;
            let _e567 = select(_e562, 0.55f, _e566);
            let _e571 = ((_e561 - _e567) / (-(_e567) - _e567));
            let _e573 = select(_e571, 0f, (_e571 < 0f));
            let _e575 = select(_e573, 1f, (_e573 > 1f));
            let _e579 = ((_e575 * _e575) * (3f - (2f * _e575)));
            let _e580 = (_e561 != _e561);
            if _e580 {
                phi_22_ = true;
            } else {
                phi_22_ = (0f >= _e561);
            }
            let _e583 = phi_22_;
            let _e587 = (exp((select(_e561, 0f, _e583) * -0.3f)) * 0.16f);
            if (_e579 != _e579) {
                phi_23_ = true;
            } else {
                phi_23_ = (_e587 >= _e579);
            }
            let _e591 = phi_23_;
            let _e592 = select(_e579, _e587, _e591);
            let _e596 = pill.member[_e167].visibility;
            if ((_e592 * _e596) <= 0.0009765625f) {
                discard;
            }
            if _e580 {
                phi_24_ = true;
            } else {
                phi_24_ = (0f <= _e561);
            }
            let _e601 = phi_24_;
            let _e604 = (1f + (select(_e561, 0f, _e601) * 0.008333334f));
            let _e606 = select(_e604, 0f, (_e604 < 0f));
            let _e608 = select(_e606, 0.6f, (_e606 > 0.6f));
            let _e618 = ((_e389 - ((_e391 * _e608) * 0.08f)) - (_e386.y * 0.04f));
            let _e619 = (((_e388 - ((_e390 * _e608) * 0.08f)) - (_e362.x * 0.04f)) * _e177);
            let _e620 = (_e618 * _e181);
            let _e624 = pill.member[_e167].effects;
            let _e628 = frame.member[0u].time;
            let _e632 = pill.member[_e167].seed;
            let _e635 = ((_e624.tempo - 0.2f) * 2.5f);
            let _e637 = select(_e635, 0f, (_e635 < 0f));
            let _e646 = ((_e628 * ((0.12f + (_e624.energy * 0.25f)) + (select(_e637, 1f, (_e637 > 1f)) * 0.12f))) + _e632);
            let _e651 = ((sin(((_e628 * _e624.tempo) * 31.415928f)) * 0.5f) + 0.5f);
            let _e657 = (((_e651 * _e651) * _e624.danceability) * (0.025f + (_e624.energy * 0.055f)));
            let _e658 = (_e624.energy * 0.55f);
            let _e663 = ((_e658 + (_e624.danceability * 0.25f)) + (_e624.loudness * 0.2f));
            if _e580 {
                phi_25_ = true;
            } else {
                phi_25_ = (0f <= _e561);
            }
            let _e666 = phi_25_;
            let _e669 = (1f + (select(_e561, 0f, _e666) * 0.008333334f));
            let _e671 = select(_e669, 0f, (_e669 < 0f));
            let _e673 = select(_e671, 1f, (_e671 > 1f));
            let _e684 = (_e632 - trunc(_e632));
            let _e689 = ((_e177 / _e181) * ((0.5f + (_e684 * 0.12f)) + (_e663 * 0.18f)));
            if (_e689 != _e689) {
                phi_26_ = true;
            } else {
                phi_26_ = (1.7f >= _e689);
            }
            let _e693 = phi_26_;
            let _e696 = select(0f, _e388, (_e388 > 0f));
            let _e698 = select(0f, _e389, (_e389 > 0f));
            let _e706 = (select(1f, _e698, (_e698 < 1f)) - (((((_e391 * _e673) * _e673) * 0.6f) + _e386.y) * 0.08f));
            let _e707 = ((select(1f, _e696, (_e696 < 1f)) - (((((_e390 * _e673) * _e673) * 0.6f) + _e362.x) * 0.08f)) * select(_e689, 1.7f, _e693));
            let _e718 = (_e646 * 0.8f);
            let _e728 = ((0.14f + (_e663 * 0.2f)) + _e657);
            let _e733 = (_e632 + 1.5707964f);
            let _e738 = pill.member[_e167].colors[0u];
            let _e740 = vec2<f32>((_e707 + ((sin(((_e706 * 4.32f) + _e646)) + cos(((_e707 * 1.3f) - (_e646 * 0.7f)))) * _e728)), ((_e706 * 1.6f) + ((cos(((_e707 * 2.3f) - _e718)) + sin(((_e706 * 2.72f) + (_e646 * 0.6f)))) * _e728)));
            let _e741 = cantus_render_track_plasma_field(_e740, unpack4x8unorm(_e738), 2.1f, 0.7f, _e646);
            let _e746 = pill.member[_e167].colors[1u];
            let _e749 = cantus_render_track_plasma_field(_e740, unpack4x8unorm(_e746), 0.6f, -2.4f, (_e733 - _e718));
            let _e766 = pill.member[_e167].colors[2u];
            let _e770 = cantus_render_track_plasma_field(_e740, unpack4x8unorm(_e766), -1.5f, 1.9f, ((_e646 * 0.65f) + 2f));
            let _e783 = pill.member[_e167].colors[3u];
            let _e784 = unpack4x8unorm(_e783);
            let _e787 = cantus_render_track_plasma_field(_e740, _e784, 2.4f, 1.6f, (_e733 - (_e646 * 0.55f)));
            let _e795 = (((_e741.w + _e749.w) + _e770.w) + _e787.w);
            let _e796 = ((((_e741.x + _e749.x) + _e770.x) + _e787.x) / _e795);
            let _e797 = ((((_e741.y + _e749.y) + _e770.y) + _e787.y) / _e795);
            let _e798 = ((((_e741.z + _e749.z) + _e770.z) + _e787.z) / _e795);
            let _e803 = (((_e796 * 0.2126f) + (_e797 * 0.7152f)) + (_e798 * 0.0722f));
            let _e807 = frame.member[0u].playhead_x;
            let _e808 = (_e807 + 3f);
            let _e812 = ((_e166.x - _e808) / ((_e807 - 3f) - _e808));
            let _e814 = select(_e812, 0f, (_e812 < 0f));
            let _e816 = select(_e814, 1f, (_e814 > 1f));
            let _e825 = pill.member[_e167].effects.valence;
            let _e826 = (_e825 * 0.4f);
            let _e827 = (1.55f + _e826);
            let _e829 = (_e803 * (-0.54999995f - _e826));
            let _e833 = (_e829 + (_e796 * _e827));
            let _e834 = (_e829 + (_e797 * _e827));
            let _e835 = (_e829 + (_e798 * _e827));
            let _e837 = select(0.035f, _e833, (_e833 > 0.035f));
            let _e839 = select(0.035f, _e834, (_e834 > 0.035f));
            let _e841 = select(0.035f, _e835, (_e835 > 0.035f));
            if (_e803 != _e803) {
                phi_27_ = true;
            } else {
                phi_27_ = (0.001f >= _e803);
            }
            let _e851 = phi_27_;
            let _e853 = (0.52f / select(_e803, 0.001f, _e851));
            if (_e853 != _e853) {
                phi_28_ = true;
            } else {
                phi_28_ = (1f <= _e853);
            }
            let _e857 = phi_28_;
            let _e858 = select(_e853, 1f, _e857);
            let _e865 = ((0.96f + (_e825 * 0.06f)) + (_e657 * 0.5f));
            let _e870 = ((_e618 - 0.45f) * 1.8181818f);
            let _e872 = select(_e870, 0f, (_e870 < 0f));
            let _e874 = select(_e872, 1f, (_e872 > 1f));
            let _e880 = (0.84f + (((_e874 * _e874) * (3f - (2f * _e874))) * 0.1f));
            let _e885 = (1f - (0.4f * ((_e816 * _e816) * (3f - (2f * _e816)))));
            let _e905 = (8f - _e624.acousticness);
            let _e909 = (_e628 * (0.35f + _e658));
            let _e912 = ((_e182 / _e905) + (_e909 * (0.16f + (_e684 * 0.08f))));
            let _e913 = ((_e183 / _e905) + (_e909 * (0.055f + (sin((_e632 * 0.7f)) * 0.025f))));
            let _e914 = floor(_e912);
            let _e915 = floor(_e913);
            let _e924 = bitcast<u32>(select(0i, select(select(i32(_e915), i32(-2147483648), (_e915 < -2147483600f)), 2147483647i, (_e915 > 2147483500f)), (_e915 == _e915)));
            let _e932 = bitcast<u32>(select(0i, select(select(i32(_e914), i32(-2147483648), (_e914 < -2147483600f)), 2147483647i, (_e914 > 2147483500f)), (_e914 == _e914)));
            let _e934 = (bitcast<u32>((_e632 + 2.71f)) * 2654435761u);
            let _e940 = (((_e932 ^ _e934) * 1664525u) + 1013904223u);
            let _e942 = ((((_e924 ^ _e934) * 1664525u) + 1013904223u) + (_e940 * 1664525u));
            let _e944 = (_e940 + (_e942 * 1664525u));
            let _e952 = ((_e942 ^ (_e942 >> bitcast<u32>(16i))) + ((_e944 ^ (_e944 >> bitcast<u32>(16i))) * 1664525u));
            let _e956 = f32((_e952 ^ (_e952 >> bitcast<u32>(16i))));
            let _e957 = (_e956 * 0.0000000016600825f);
            let _e971 = (_e624.acousticness * 0.09f);
            let _e974 = (bitcast<u32>(_e632) * 2654435761u);
            let _e980 = (((_e924 ^ _e974) * 1664525u) + 1013904223u);
            let _e982 = ((((_e932 ^ _e974) * 1664525u) + 1013904223u) + (_e980 * 1664525u));
            let _e984 = (_e980 + (_e982 * 1664525u));
            let _e992 = ((_e982 ^ (_e982 >> bitcast<u32>(16i))) + ((_e984 ^ (_e984 >> bitcast<u32>(16i))) * 1664525u));
            let _e1000 = (((f32((_e992 ^ (_e992 >> bitcast<u32>(16i)))) * 0.00000000023283064f) - (0.985f - _e971)) / (_e971 + 0.014999986f));
            let _e1002 = select(_e1000, 0f, (_e1000 < 0f));
            let _e1004 = select(_e1002, 1f, (_e1002 > 1f));
            let _e1013 = (((_e912 - _e914) - 0.5f) - ((_e956 * 0.00000000013038516f) - 0.28f));
            let _e1014 = (((_e913 - _e915) - 0.5f) - (((_e957 - trunc(_e957)) * 0.56f) - 0.28f));
            let _e1020 = ((sqrt(((_e1013 * _e1013) + (_e1014 * _e1014))) - 0.06f) * 4.5454545f);
            let _e1022 = select(_e1020, 0f, (_e1020 < 0f));
            let _e1024 = select(_e1022, 1f, (_e1022 > 1f));
            let _e1037 = (((((_e1004 * _e1004) * (3f - (2f * _e1004))) * (1f - ((_e1024 * _e1024) * (3f - (2f * _e1024))))) * ((sin(((_e628 * ((0.7f + (_e956 * 0.00000000020954757f)) + (_e624.energy * 0.8f))) + (_e956 * 0.0000000014629181f))) * 0.5f) + 0.5f)) * (0.12f + (_e624.acousticness * 0.48f)));
            let _e1041 = (((((select(0.92f, _e837, (_e837 < 0.92f)) * _e858) * _e865) * _e880) * _e885) + (((_e784.x * 0.75f) + 0.25f) * _e1037));
            let _e1042 = (((((select(0.92f, _e839, (_e839 < 0.92f)) * _e858) * _e865) * _e880) * _e885) + (((_e784.y * 0.75f) + 0.25f) * _e1037));
            let _e1043 = (((((select(0.92f, _e841, (_e841 < 0.92f)) * _e858) * _e865) * _e880) * _e885) + (((_e784.z * 0.75f) + 0.25f) * _e1037));
            let _e1050 = (_e182 / _e181);
            if (_e624.instrumentalness <= 0.00390625f) {
                phi_32_ = 0f;
            } else {
                let _e1055 = (_e628 * (0.5f + (_e624.energy * 0.35f)));
                let _e1063 = (sin(((_e389 * 1.9f) + _e1055)) * 0.35f);
                let _e1064 = (sin(((_e1050 * 1.5f) - (_e1055 * 0.8f))) * 0.35f);
                let _e1067 = ((_e1055 * 0.05f) + _e632);
                let _e1068 = ((_e1055 * -0.04f) + _e632);
                let _e1076 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1050 * 0.7f) + _e1063) + _e1067), (((_e389 * 0.7f) + _e1064) + _e1068)));
                let _e1079 = (1f - (abs(_e1076) * 2f));
                if (_e1079 != _e1079) {
                    phi_29_ = true;
                } else {
                    phi_29_ = (0f >= _e1079);
                }
                let _e1083 = phi_29_;
                let _e1084 = select(_e1079, 0f, _e1083);
                let _e1086 = ((_e1084 * _e1084) * _e1084);
                let _e1096 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1050 * 1.1f) - _e1063) - (_e1067 * 0.8f)), (((_e389 * 1.1f) - _e1064) - (_e1068 * 0.8f))));
                let _e1099 = (1f - (abs(_e1096) * 2f));
                if (_e1099 != _e1099) {
                    phi_30_ = true;
                } else {
                    phi_30_ = (0f >= _e1099);
                }
                let _e1103 = phi_30_;
                let _e1104 = select(_e1099, 0f, _e1103);
                let _e1106 = ((_e1104 * _e1104) * _e1104);
                if (_e1086 != _e1086) {
                    phi_31_ = true;
                } else {
                    phi_31_ = (_e1106 >= _e1086);
                }
                let _e1110 = phi_31_;
                phi_32_ = ((select(_e1086, _e1106, _e1110) * _e624.instrumentalness) * 0.06f);
            }
            let _e1115 = phi_32_;
            let _e1119 = (_e1041 + (((_e1041 * 0.25f) + 0.75f) * _e1115));
            let _e1120 = (_e1042 + (((_e1042 * 0.25f) + 0.75f) * _e1115));
            let _e1121 = (_e1043 + (((_e1043 * 0.25f) + 0.75f) * _e1115));
            let _e1122 = vec3<f32>(_e1119, _e1120, _e1121);
            let _e1123 = (_e188 + _e185);
            let _e1127 = pill.member[_e167].image_index;
            if (_e1127 >= 0i) {
                let _e1129 = (_e182 - _e1123);
                let _e1130 = abs(_e1129);
                let _e1131 = abs(_e187);
                if (select(_e1131, _e1130, (_e1130 > _e1131)) < _e181) {
                    let _e1135 = (_e185 + _e560);
                    let _e1141 = (_e1135 * 2f);
                    let _e1147 = vec3<f32>(((_e1129 / _e1141) + 0.5f), ((_e187 / _e1141) + 0.5f), f32(_e1127));
                    let _e1153 = textureSample(images, sampler_, vec2<f32>(_e1147.x, _e1147.y), i32(_e1147.z));
                    let _e1155 = (((sqrt(((_e1129 * _e1129) + (_e187 * _e187))) - _e1135) - -4f) * 0.25f);
                    let _e1157 = select(_e1155, 0f, (_e1155 < 0f));
                    let _e1159 = select(_e1157, 1f, (_e1157 > 1f));
                    let _e1166 = ((_e214 - 0.5f) * -1f);
                    let _e1168 = select(_e1166, 0f, (_e1166 < 0f));
                    let _e1170 = select(_e1168, 1f, (_e1168 > 1f));
                    let _e1179 = ((_e191 - (((_e360 * ((_e1170 * _e1170) * (3f - (2f * _e1170)))) + _e384) * 0.5f)) - -0.5f);
                    let _e1181 = select(_e1179, 0f, (_e1179 < 0f));
                    let _e1183 = select(_e1181, 1f, (_e1181 > 1f));
                    let _e1194 = (((1f - ((_e1159 * _e1159) * (3f - (2f * _e1159)))) * (1f - ((_e1183 * _e1183) * (3f - (2f * _e1183))))) * _e1153.w);
                    let _e1195 = (1f - _e1194);
                    phi_33_ = vec3<f32>(((_e1119 * _e1195) + (_e1153.x * _e1194)), ((_e1120 * _e1195) + (_e1153.y * _e1194)), ((_e1121 * _e1195) + (_e1153.z * _e1194)));
                } else {
                    phi_33_ = _e1122;
                }
                let _e1207 = phi_33_;
                phi_34_ = _e1207;
            } else {
                phi_34_ = _e1122;
            }
            let _e1209 = phi_34_;
            let _e1220 = ((_e618 - 0.12f) * -8.333334f);
            let _e1222 = select(_e1220, 0f, (_e1220 < 0f));
            let _e1224 = select(_e1222, 1f, (_e1222 > 1f));
            let _e1231 = ((_e561 - 5f) * -0.125f);
            let _e1233 = select(_e1231, 0f, (_e1231 < 0f));
            let _e1235 = select(_e1233, 1f, (_e1233 > 1f));
            let _e1241 = ((((_e1224 * _e1224) * (3f - (2f * _e1224))) * 0.12f) + (((_e1235 * _e1235) * (3f - (2f * _e1235))) * 0.08f));
            let _e1245 = (_e1209.x + (((_e1209.x * 0.68f) + 0.32f) * _e1241));
            let _e1246 = (_e1209.y + (((_e1209.y * 0.68f) + 0.32f) * _e1241));
            let _e1247 = (_e1209.z + (((_e1209.z * 0.68f) + 0.32f) * _e1241));
            let _e1255 = local_6;
            let _e1256 = (1f - _e1255);
            let _e1261 = local_7;
            let _e1264 = local_8;
            let _e1267 = local_9;
            let _e1275 = vec4<f32>((((_e1245 * _e1256) + (((_e1245 * 1.5f) + 0.1f) * _e1261)) * _e579), (((_e1246 * _e1256) + (((_e1246 * 1.5f) + 0.1f) * _e1264)) * _e579), (((_e1247 * _e1256) + (((_e1247 * 1.5f) + 0.1f) * _e1267)) * _e579), _e592);
            if _e399 {
                if (_e477 > 0f) {
                    phi_35_ = _e1275;
                    phi_36_ = 0u;
                    loop {
                        let _e1278 = phi_35_;
                        let _e1280 = phi_36_;
                        local_20 = _e1278;
                        let _e1281 = (_e1280 < 5u);
                        if _e1281 {
                            let _e1282 = f32(_e1280);
                            if _e435 {
                                phi_37_ = true;
                            } else {
                                phi_37_ = (0f >= _e434);
                            }
                            let _e1285 = phi_37_;
                            let _e1290 = (_e392 + ((_e1282 - (select(_e434, 0f, _e1285) * 0.5f)) * 18f));
                            let _e1291 = (_e393 + 5f);
                            let _e1292 = (_e166.x - _e1290);
                            let _e1293 = (_e166.y - _e1291);
                            let _e1294 = abs(_e1292);
                            let _e1295 = abs(_e1293);
                            if (select(_e1295, _e1294, (_e1294 > _e1295)) < 38.88f) {
                                let _e1302 = ((f32(_e398) - (_e1282 * 2f)) * 0.5f);
                                let _e1304 = select(_e1302, 0f, (_e1302 < 0f));
                                let _e1307 = (_e1290 - _e426);
                                let _e1308 = (_e1291 - _e431);
                                let _e1314 = ((sqrt(((_e1307 * _e1307) + (_e1308 * _e1308))) - 11.3f) * -1f);
                                let _e1316 = select(_e1314, 0f, (_e1314 < 0f));
                                let _e1318 = select(_e1316, 1f, (_e1316 > 1f));
                                let _e1324 = select(_e195, 0f, (_e195 < 0f));
                                let _e1327 = (((_e1318 * _e1318) * (3f - (2f * _e1318))) * select(_e1324, 1f, (_e1324 > 1f)));
                                let _e1329 = (1.05f + (0.63f * _e1327));
                                let _e1330 = (_e1307 * _e1327);
                                let _e1332 = (_e1292 - (_e1330 * 0.5f));
                                let _e1333 = (_e1330 * -0.005f);
                                let _e1334 = sin(_e1333);
                                let _e1335 = cos(_e1333);
                                let _e1338 = ((_e1335 * _e1332) - (_e1334 * _e1293));
                                let _e1341 = ((_e1334 * _e1332) + (_e1335 * _e1293));
                                let _e1345 = (_e1329 * 5.4f);
                                let _e1346 = abs(_e1338);
                                let _e1350 = ((0.809017f * _e1346) + (_e1341 * 0.58778524f));
                                if (_e1350 != _e1350) {
                                    phi_38_ = true;
                                } else {
                                    phi_38_ = (0f >= _e1350);
                                }
                                let _e1354 = phi_38_;
                                let _e1355 = select(_e1350, 0f, _e1354);
                                let _e1358 = (_e1346 - (_e1355 * 1.618034f));
                                let _e1359 = (-(_e1341) - (_e1355 * -1.1755705f));
                                let _e1362 = ((-0.809017f * _e1358) + (-0.58778524f * _e1359));
                                if (_e1362 != _e1362) {
                                    phi_39_ = true;
                                } else {
                                    phi_39_ = (0f >= _e1362);
                                }
                                let _e1366 = phi_39_;
                                let _e1367 = select(_e1362, 0f, _e1366);
                                let _e1372 = abs((_e1358 - (_e1367 * -1.618034f)));
                                let _e1373 = ((_e1359 - (_e1367 * -1.1755705f)) - _e1345);
                                let _e1374 = (_e1329 * 2.031386f);
                                let _e1376 = ((_e1329 * 2.7959628f) - _e1345);
                                let _e1383 = (((_e1372 * _e1374) + (_e1373 * _e1376)) / ((_e1374 * _e1374) + (_e1376 * _e1376)));
                                let _e1385 = select(_e1383, 0f, (_e1383 < 0f));
                                let _e1387 = select(_e1385, 1f, (_e1385 > 1f));
                                let _e1393 = (_e1372 - (_e1374 * _e1387));
                                let _e1394 = (_e1373 - (_e1376 * _e1387));
                                let _e1403 = ((sqrt(((_e1393 * _e1393) + (_e1394 * _e1394))) * select(1f, -1f, (((_e1373 * _e1374) - (_e1372 * _e1376)) < 0f))) - (_e1329 * 1.08f));
                                let _e1404 = (((_e1338 / (_e1329 * 21.6f)) + 0.5f) - select(_e1304, 1f, (_e1304 > 1f)));
                                let _e1405 = fwidth(_e1404);
                                let _e1407 = ((_e1404 / _e1405) + 0.5f);
                                let _e1409 = select(_e1407, 0f, (_e1407 < 0f));
                                let _e1411 = select(_e1409, 1f, (_e1409 > 1f));
                                let _e1412 = (1f - _e1411);
                                let _e1415 = (0.33f * _e1411);
                                let _e1419 = (0.5f - _e1403);
                                let _e1421 = select(_e1419, 0f, (_e1419 < 0f));
                                let _e1423 = select(_e1421, 1f, (_e1421 > 1f));
                                if (_e1403 != _e1403) {
                                    phi_40_ = true;
                                } else {
                                    phi_40_ = (0f >= _e1403);
                                }
                                let _e1427 = phi_40_;
                                let _e1430 = exp((select(_e1403, 0f, _e1427) * -0.5f));
                                let _e1431 = (_e1403 * -0.2f);
                                let _e1433 = select(_e1431, 0f, (_e1431 < 0f));
                                let _e1435 = select(_e1433, 1f, (_e1433 > 1f));
                                let _e1440 = (1f - ((_e1435 * _e1435) * (3f - (2f * _e1435))));
                                let _e1442 = ((_e1440 * _e1440) * 0.045f);
                                let _e1453 = ((_e1430 * _e1430) * 0.2f);
                                if (_e1423 != _e1423) {
                                    phi_41_ = true;
                                } else {
                                    phi_41_ = (_e1453 >= _e1423);
                                }
                                let _e1457 = phi_41_;
                                let _e1459 = (select(_e1423, _e1453, _e1457) * _e477);
                                let _e1460 = (1f - _e1459);
                                phi_42_ = vec4<f32>(((_e1278.x * _e1460) + ((((_e1412 + _e1415) + _e1442) * _e1423) * _e477)), ((_e1278.y * _e1460) + (((((0.85f * _e1412) + _e1415) + _e1442) * _e1423) * _e477)), ((_e1278.z * _e1460) + (((((0.2f * _e1412) + _e1415) + _e1442) * _e1423) * _e477)), ((_e1278.w * _e1460) + _e1459));
                            } else {
                                phi_42_ = _e1278;
                            }
                            let _e1475 = phi_42_;
                            phi_43_ = _e1475;
                            phi_44_ = (_e1280 + 1u);
                        } else {
                            phi_43_ = vec4<f32>();
                            phi_44_ = u32();
                        }
                        let _e1478 = phi_43_;
                        let _e1480 = phi_44_;
                        continue;
                        continuing {
                            phi_35_ = _e1478;
                            phi_36_ = _e1480;
                            break if !(_e1281);
                        }
                    }
                    if _e330 {
                        break;
                    }
                    let _e2179 = local_20;
                    phi_45_ = _e2179;
                } else {
                    phi_45_ = _e1275;
                }
                let _e1483 = phi_45_;
                phi_46_ = _e1483;
            } else {
                phi_46_ = _e1275;
            }
            let _e1485 = phi_46_;
            let _e1486 = (_e404 + _e418);
            phi_47_ = _e1485;
            phi_48_ = 0u;
            loop {
                let _e1490 = phi_47_;
                let _e1492 = phi_48_;
                local_16 = _e1490;
                local_17 = _e1490;
                local_18 = _e1490;
                local_19 = _e1490;
                let _e1493 = (_e1492 < select(_e1486, 8u, (8u < _e1486)));
                if _e1493 {
                    if (_e1492 < 8u) {
                    } else {
                        phi_62_ = true;
                        break;
                    }
                    let _e1499 = pill.member[_e167].playlist_images[_e1492];
                    if (_e1499 >= 0i) {
                        let _e1501 = (_e1492 < _e404);
                        if _e1501 {
                            phi_49_ = render_shared_RipplePulse(vec2<f32>(_e392, _e394), _e406, 1f);
                            phi_50_ = (f32(_e1492) + _e400);
                        } else {
                            phi_49_ = render_shared_RipplePulse(vec2<f32>(_e392, _e414), _e419, _e412);
                            phi_50_ = f32((_e1492 - _e404));
                        }
                        let _e1507 = phi_49_;
                        let _e1509 = phi_50_;
                        let _e1510 = select(_e412, _e477, _e1501);
                        let _e1512 = (_e1507.start_time - 1f);
                        if (_e1512 != _e1512) {
                            phi_51_ = true;
                        } else {
                            phi_51_ = (0f >= _e1512);
                        }
                        let _e1516 = phi_51_;
                        let _e1525 = (_e1507.origin.x + (((_e1509 - (select(_e1512, 0f, _e1516) * 0.5f)) * 18f) * _e1507.strength));
                        let _e1528 = (_e1507.origin.y + 2f);
                        if (_e1510 > 0f) {
                            let _e1530 = (_e166.x - _e1525);
                            let _e1531 = (_e166.y - _e1528);
                            let _e1532 = abs(_e1530);
                            let _e1533 = abs(_e1531);
                            if (select(_e1533, _e1532, (_e1532 > _e1533)) < 38.88f) {
                                let _e1537 = (_e1525 - _e426);
                                let _e1538 = (_e1528 - _e431);
                                let _e1542 = sqrt(((_e1537 * _e1537) + (_e1538 * _e1538)));
                                let _e1544 = ((_e1542 - 11.3f) * -1f);
                                let _e1546 = select(_e1544, 0f, (_e1544 < 0f));
                                let _e1548 = select(_e1546, 1f, (_e1546 > 1f));
                                let _e1554 = select(_e195, 0f, (_e195 < 0f));
                                let _e1557 = (((_e1548 * _e1548) * (3f - (2f * _e1548))) * select(_e1554, 1f, (_e1554 > 1f)));
                                let _e1559 = (1.05f + (0.63f * _e1557));
                                let _e1560 = (_e1537 * _e1557);
                                let _e1562 = (_e1530 - (_e1560 * 0.5f));
                                let _e1563 = (_e1560 * -0.005f);
                                let _e1564 = sin(_e1563);
                                let _e1565 = cos(_e1563);
                                let _e1568 = ((_e1565 * _e1562) - (_e1564 * _e1531));
                                let _e1571 = ((_e1564 * _e1562) + (_e1565 * _e1531));
                                let _e1572 = (_e1559 * 21.6f);
                                if _e1501 {
                                    phi_53_ = true;
                                } else {
                                    if _e196 {
                                        phi_52_ = select(true, false, (_e1542 <= 10.8f));
                                    } else {
                                        phi_52_ = true;
                                    }
                                    let _e1580 = phi_52_;
                                    phi_53_ = select(true, false, _e1580);
                                }
                                let _e1583 = phi_53_;
                                let _e1584 = select(0.2f, 0f, _e1583);
                                let _e1587 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1568, _e1571), 0f, (_e1559 * 6.4800005f));
                                if (_e1587 <= 7f) {
                                    let _e1590 = vec3<f32>(((_e1568 / _e1572) + 0.5f), ((_e1571 / _e1572) + 0.5f), f32(_e1499));
                                    let _e1596 = textureSample(images, sampler_, vec2<f32>(_e1590.x, _e1590.y), i32(_e1590.z));
                                    let _e1600 = (1f - _e1584);
                                    let _e1604 = (0.24f * _e1584);
                                    let _e1608 = (0.5f - _e1587);
                                    let _e1610 = select(_e1608, 0f, (_e1608 < 0f));
                                    let _e1612 = select(_e1610, 1f, (_e1610 > 1f));
                                    if (_e1587 != _e1587) {
                                        phi_54_ = true;
                                    } else {
                                        phi_54_ = (0f >= _e1587);
                                    }
                                    let _e1616 = phi_54_;
                                    let _e1619 = exp((select(_e1587, 0f, _e1616) * -0.5f));
                                    let _e1620 = (_e1587 * -0.2f);
                                    let _e1622 = select(_e1620, 0f, (_e1620 < 0f));
                                    let _e1624 = select(_e1622, 1f, (_e1622 > 1f));
                                    let _e1629 = (1f - ((_e1624 * _e1624) * (3f - (2f * _e1624))));
                                    let _e1631 = ((_e1629 * _e1629) * 0.045f);
                                    let _e1642 = ((_e1619 * _e1619) * 0.2f);
                                    if (_e1612 != _e1612) {
                                        phi_55_ = true;
                                    } else {
                                        phi_55_ = (_e1642 >= _e1612);
                                    }
                                    let _e1646 = phi_55_;
                                    let _e1648 = (select(_e1612, _e1642, _e1646) * _e1510);
                                    let _e1649 = (1f - _e1648);
                                    phi_56_ = vec4<f32>(((_e1490.x * _e1649) + (((((_e1596.x * _e1600) + _e1604) + _e1631) * _e1612) * _e1510)), ((_e1490.y * _e1649) + (((((_e1596.y * _e1600) + _e1604) + _e1631) * _e1612) * _e1510)), ((_e1490.z * _e1649) + (((((_e1596.z * _e1600) + _e1604) + _e1631) * _e1612) * _e1510)), ((_e1490.w * _e1649) + _e1648));
                                } else {
                                    phi_56_ = _e1490;
                                }
                                let _e1664 = phi_56_;
                                phi_57_ = _e1664;
                            } else {
                                phi_57_ = _e1490;
                            }
                            let _e1666 = phi_57_;
                            phi_58_ = _e1666;
                        } else {
                            phi_58_ = _e1490;
                        }
                        let _e1668 = phi_58_;
                        phi_59_ = _e1668;
                    } else {
                        phi_59_ = _e1490;
                    }
                    let _e1670 = phi_59_;
                    phi_60_ = _e1670;
                    phi_61_ = (_e1492 + 1u);
                } else {
                    phi_60_ = vec4<f32>();
                    phi_61_ = u32();
                }
                let _e1673 = phi_60_;
                let _e1675 = phi_61_;
                continue;
                continuing {
                    phi_47_ = _e1673;
                    phi_48_ = _e1675;
                    phi_62_ = _e330;
                    break if !(_e1493);
                }
            }
            let _e1678 = phi_62_;
            if _e1678 {
                break;
            }
            let _e1683 = pill.member[_e167].lines[0u];
            if (_e619 < _e1683.min.x) {
                phi_91_ = f32();
                phi_92_ = true;
            } else {
                if (_e619 > _e1683.max.x) {
                    phi_89_ = f32();
                    phi_90_ = true;
                } else {
                    if (_e620 < _e1683.min.y) {
                        phi_87_ = f32();
                        phi_88_ = true;
                    } else {
                        let _e1695 = (_e620 > _e1683.max.y);
                        if _e1695 {
                            phi_86_ = f32();
                        } else {
                            let _e1697 = (1f / _e1683.size);
                            let _e1704 = ((_e619 - _e1683.origin.x) * _e1697);
                            phi_63_ = 0u;
                            phi_64_ = _e1683.count;
                            loop {
                                let _e1709 = phi_63_;
                                let _e1711 = phi_64_;
                                local_10 = _e1709;
                                let _e1712 = (_e1709 < _e1711);
                                if _e1712 {
                                    let _e1715 = (_e1709 + ((_e1711 - _e1709) / 2u));
                                    let _e1720 = placed_glyphs.member[(_e1683.first + _e1715)].x;
                                    let _e1721 = (_e1720 <= _e1704);
                                    if _e1721 {
                                        phi_65_ = (_e1715 + 1u);
                                    } else {
                                        phi_65_ = _e1709;
                                    }
                                    let _e1724 = phi_65_;
                                    phi_66_ = _e1724;
                                    phi_67_ = select(_e1715, _e1711, _e1721);
                                } else {
                                    phi_66_ = u32();
                                    phi_67_ = u32();
                                }
                                let _e1727 = phi_66_;
                                let _e1729 = phi_67_;
                                continue;
                                continuing {
                                    phi_63_ = _e1727;
                                    phi_64_ = _e1729;
                                    break if !(_e1712);
                                }
                            }
                            let _e1731 = (3.5f / _e1683.size);
                            let _e1733 = local_10;
                            let _e1734 = (_e1733 + 1u);
                            phi_68_ = select(_e1734, _e1683.count, (_e1683.count < _e1734));
                            phi_69_ = -1000000f;
                            loop {
                                let _e1738 = phi_68_;
                                let _e1740 = phi_69_;
                                local_21 = _e1740;
                                if (_e1738 > 0u) {
                                    let _e1742 = (_e1738 - 1u);
                                    let _e1743 = (_e1683.first + _e1742);
                                    let _e1747 = placed_glyphs.member[_e1743].x;
                                    let _e1751 = placed_glyphs.member[_e1743].glyph;
                                    let _e1756 = glyphs.member[_e1751].min[0u];
                                    let _e1761 = glyphs.member[_e1751].min[1u];
                                    let _e1766 = glyphs.member[_e1751].max[0u];
                                    let _e1771 = glyphs.member[_e1751].max[1u];
                                    let _e1775 = glyphs.member[_e1751].start;
                                    let _e1779 = glyphs.member[_e1751].count;
                                    let _e1780 = (_e1704 - _e1747);
                                    let _e1781 = -(((_e620 - _e1683.origin.y) * _e1697));
                                    let _e1782 = (_e1766 + _e1731);
                                    let _e1783 = (_e1780 > _e1782);
                                    if _e1783 {
                                        phi_82_ = f32();
                                    } else {
                                        if (_e1780 >= (_e1756 - _e1731)) {
                                            if (_e1781 >= (_e1761 - _e1731)) {
                                                if (_e1780 <= _e1782) {
                                                    if (_e1781 <= (_e1771 + _e1731)) {
                                                        phi_70_ = 340282350000000000000000000000000000000f;
                                                        phi_71_ = 0u;
                                                        phi_72_ = 0i;
                                                        loop {
                                                            let _e1793 = phi_70_;
                                                            let _e1795 = phi_71_;
                                                            let _e1797 = phi_72_;
                                                            local_11 = _e1793;
                                                            local_12 = _e1797;
                                                            let _e1798 = (_e1795 < _e1779);
                                                            if _e1798 {
                                                                let _e1802 = edges.member[(_e1775 + _e1795)];
                                                                let _e1804 = cantus_render_text_edge_distance(_e1802, _e1683.weight, vec2<f32>(_e1780, _e1781), _e1793);
                                                                phi_73_ = _e1804.member;
                                                                phi_74_ = (_e1795 + 1u);
                                                                phi_75_ = (_e1797 + _e1804.member_1);
                                                            } else {
                                                                phi_73_ = f32();
                                                                phi_74_ = u32();
                                                                phi_75_ = i32();
                                                            }
                                                            let _e1810 = phi_73_;
                                                            let _e1812 = phi_74_;
                                                            let _e1814 = phi_75_;
                                                            continue;
                                                            continuing {
                                                                phi_70_ = _e1810;
                                                                phi_71_ = _e1812;
                                                                phi_72_ = _e1814;
                                                                break if !(_e1798);
                                                            }
                                                        }
                                                        let _e1817 = local_11;
                                                        let _e1819 = ((_e1817 * _e1683.size) * _e1683.size);
                                                        if (_e1819 >= 12.25f) {
                                                            phi_76_ = 3.5f;
                                                        } else {
                                                            phi_76_ = sqrt(_e1819);
                                                        }
                                                        let _e1823 = phi_76_;
                                                        let _e1825 = local_12;
                                                        let _e1828 = (_e1823 * select(1f, -1f, (_e1825 == 0i)));
                                                        if (_e1740 != _e1740) {
                                                            phi_77_ = true;
                                                        } else {
                                                            phi_77_ = (_e1828 >= _e1740);
                                                        }
                                                        let _e1832 = phi_77_;
                                                        phi_78_ = select(_e1740, _e1828, _e1832);
                                                    } else {
                                                        phi_78_ = _e1740;
                                                    }
                                                    let _e1835 = phi_78_;
                                                    phi_79_ = _e1835;
                                                } else {
                                                    phi_79_ = _e1740;
                                                }
                                                let _e1837 = phi_79_;
                                                phi_80_ = _e1837;
                                            } else {
                                                phi_80_ = _e1740;
                                            }
                                            let _e1839 = phi_80_;
                                            phi_81_ = _e1839;
                                        } else {
                                            phi_81_ = _e1740;
                                        }
                                        let _e1841 = phi_81_;
                                        phi_82_ = _e1841;
                                    }
                                    let _e1843 = phi_82_;
                                    phi_83_ = _e1742;
                                    phi_84_ = _e1843;
                                    phi_85_ = select(true, false, _e1783);
                                } else {
                                    phi_83_ = u32();
                                    phi_84_ = f32();
                                    phi_85_ = false;
                                }
                                let _e1846 = phi_83_;
                                let _e1848 = phi_84_;
                                let _e1850 = phi_85_;
                                continue;
                                continuing {
                                    phi_68_ = _e1846;
                                    phi_69_ = _e1848;
                                    break if !(_e1850);
                                }
                            }
                            let _e2225 = local_21;
                            phi_86_ = _e2225;
                        }
                        let _e1853 = phi_86_;
                        phi_87_ = _e1853;
                        phi_88_ = _e1695;
                    }
                    let _e1855 = phi_87_;
                    let _e1857 = phi_88_;
                    phi_89_ = _e1855;
                    phi_90_ = _e1857;
                }
                let _e1859 = phi_89_;
                let _e1861 = phi_90_;
                phi_91_ = _e1859;
                phi_92_ = _e1861;
            }
            let _e1863 = phi_91_;
            let _e1865 = phi_92_;
            let _e1868 = ((select(_e1863, -1000000f, _e1865) * 1.25f) + 0.5f);
            let _e1870 = select(_e1868, 0f, (_e1868 < 0f));
            let _e1872 = select(_e1870, 1f, (_e1870 > 1f));
            let _e1876 = ((_e1872 * _e1872) * (3f - (2f * _e1872)));
            let _e1881 = pill.member[_e167].lines[1u];
            if (_e619 < _e1881.min.x) {
                phi_121_ = f32();
                phi_122_ = true;
            } else {
                if (_e619 > _e1881.max.x) {
                    phi_119_ = f32();
                    phi_120_ = true;
                } else {
                    if (_e620 < _e1881.min.y) {
                        phi_117_ = f32();
                        phi_118_ = true;
                    } else {
                        let _e1893 = (_e620 > _e1881.max.y);
                        if _e1893 {
                            phi_116_ = f32();
                        } else {
                            let _e1895 = (1f / _e1881.size);
                            let _e1902 = ((_e619 - _e1881.origin.x) * _e1895);
                            phi_93_ = 0u;
                            phi_94_ = _e1881.count;
                            loop {
                                let _e1907 = phi_93_;
                                let _e1909 = phi_94_;
                                local_13 = _e1907;
                                let _e1910 = (_e1907 < _e1909);
                                if _e1910 {
                                    let _e1913 = (_e1907 + ((_e1909 - _e1907) / 2u));
                                    let _e1918 = placed_glyphs.member[(_e1881.first + _e1913)].x;
                                    let _e1919 = (_e1918 <= _e1902);
                                    if _e1919 {
                                        phi_95_ = (_e1913 + 1u);
                                    } else {
                                        phi_95_ = _e1907;
                                    }
                                    let _e1922 = phi_95_;
                                    phi_96_ = _e1922;
                                    phi_97_ = select(_e1913, _e1909, _e1919);
                                } else {
                                    phi_96_ = u32();
                                    phi_97_ = u32();
                                }
                                let _e1925 = phi_96_;
                                let _e1927 = phi_97_;
                                continue;
                                continuing {
                                    phi_93_ = _e1925;
                                    phi_94_ = _e1927;
                                    break if !(_e1910);
                                }
                            }
                            let _e1929 = (3.5f / _e1881.size);
                            let _e1931 = local_13;
                            let _e1932 = (_e1931 + 1u);
                            phi_98_ = select(_e1932, _e1881.count, (_e1881.count < _e1932));
                            phi_99_ = -1000000f;
                            loop {
                                let _e1936 = phi_98_;
                                let _e1938 = phi_99_;
                                local_22 = _e1938;
                                if (_e1936 > 0u) {
                                    let _e1940 = (_e1936 - 1u);
                                    let _e1941 = (_e1881.first + _e1940);
                                    let _e1945 = placed_glyphs.member[_e1941].x;
                                    let _e1949 = placed_glyphs.member[_e1941].glyph;
                                    let _e1954 = glyphs.member[_e1949].min[0u];
                                    let _e1959 = glyphs.member[_e1949].min[1u];
                                    let _e1964 = glyphs.member[_e1949].max[0u];
                                    let _e1969 = glyphs.member[_e1949].max[1u];
                                    let _e1973 = glyphs.member[_e1949].start;
                                    let _e1977 = glyphs.member[_e1949].count;
                                    let _e1978 = (_e1902 - _e1945);
                                    let _e1979 = -(((_e620 - _e1881.origin.y) * _e1895));
                                    let _e1980 = (_e1964 + _e1929);
                                    let _e1981 = (_e1978 > _e1980);
                                    if _e1981 {
                                        phi_112_ = f32();
                                    } else {
                                        if (_e1978 >= (_e1954 - _e1929)) {
                                            if (_e1979 >= (_e1959 - _e1929)) {
                                                if (_e1978 <= _e1980) {
                                                    if (_e1979 <= (_e1969 + _e1929)) {
                                                        phi_100_ = 340282350000000000000000000000000000000f;
                                                        phi_101_ = 0u;
                                                        phi_102_ = 0i;
                                                        loop {
                                                            let _e1991 = phi_100_;
                                                            let _e1993 = phi_101_;
                                                            let _e1995 = phi_102_;
                                                            local_14 = _e1991;
                                                            local_15 = _e1995;
                                                            let _e1996 = (_e1993 < _e1977);
                                                            if _e1996 {
                                                                let _e2000 = edges.member[(_e1973 + _e1993)];
                                                                let _e2002 = cantus_render_text_edge_distance(_e2000, _e1881.weight, vec2<f32>(_e1978, _e1979), _e1991);
                                                                phi_103_ = _e2002.member;
                                                                phi_104_ = (_e1993 + 1u);
                                                                phi_105_ = (_e1995 + _e2002.member_1);
                                                            } else {
                                                                phi_103_ = f32();
                                                                phi_104_ = u32();
                                                                phi_105_ = i32();
                                                            }
                                                            let _e2008 = phi_103_;
                                                            let _e2010 = phi_104_;
                                                            let _e2012 = phi_105_;
                                                            continue;
                                                            continuing {
                                                                phi_100_ = _e2008;
                                                                phi_101_ = _e2010;
                                                                phi_102_ = _e2012;
                                                                break if !(_e1996);
                                                            }
                                                        }
                                                        let _e2015 = local_14;
                                                        let _e2017 = ((_e2015 * _e1881.size) * _e1881.size);
                                                        if (_e2017 >= 12.25f) {
                                                            phi_106_ = 3.5f;
                                                        } else {
                                                            phi_106_ = sqrt(_e2017);
                                                        }
                                                        let _e2021 = phi_106_;
                                                        let _e2023 = local_15;
                                                        let _e2026 = (_e2021 * select(1f, -1f, (_e2023 == 0i)));
                                                        if (_e1938 != _e1938) {
                                                            phi_107_ = true;
                                                        } else {
                                                            phi_107_ = (_e2026 >= _e1938);
                                                        }
                                                        let _e2030 = phi_107_;
                                                        phi_108_ = select(_e1938, _e2026, _e2030);
                                                    } else {
                                                        phi_108_ = _e1938;
                                                    }
                                                    let _e2033 = phi_108_;
                                                    phi_109_ = _e2033;
                                                } else {
                                                    phi_109_ = _e1938;
                                                }
                                                let _e2035 = phi_109_;
                                                phi_110_ = _e2035;
                                            } else {
                                                phi_110_ = _e1938;
                                            }
                                            let _e2037 = phi_110_;
                                            phi_111_ = _e2037;
                                        } else {
                                            phi_111_ = _e1938;
                                        }
                                        let _e2039 = phi_111_;
                                        phi_112_ = _e2039;
                                    }
                                    let _e2041 = phi_112_;
                                    phi_113_ = _e1940;
                                    phi_114_ = _e2041;
                                    phi_115_ = select(true, false, _e1981);
                                } else {
                                    phi_113_ = u32();
                                    phi_114_ = f32();
                                    phi_115_ = false;
                                }
                                let _e2044 = phi_113_;
                                let _e2046 = phi_114_;
                                let _e2048 = phi_115_;
                                continue;
                                continuing {
                                    phi_98_ = _e2044;
                                    phi_99_ = _e2046;
                                    break if !(_e2048);
                                }
                            }
                            let _e2260 = local_22;
                            phi_116_ = _e2260;
                        }
                        let _e2051 = phi_116_;
                        phi_117_ = _e2051;
                        phi_118_ = _e1893;
                    }
                    let _e2053 = phi_117_;
                    let _e2055 = phi_118_;
                    phi_119_ = _e2053;
                    phi_120_ = _e2055;
                }
                let _e2057 = phi_119_;
                let _e2059 = phi_120_;
                phi_121_ = _e2057;
                phi_122_ = _e2059;
            }
            let _e2061 = phi_121_;
            let _e2063 = phi_122_;
            let _e2066 = ((select(_e2061, -1000000f, _e2063) * 1.25f) + 0.5f);
            let _e2068 = select(_e2066, 0f, (_e2066 < 0f));
            let _e2070 = select(_e2068, 1f, (_e2068 > 1f));
            let _e2074 = ((_e2070 * _e2070) * (3f - (2f * _e2070)));
            if (_e1876 != _e1876) {
                phi_123_ = true;
            } else {
                phi_123_ = (_e2074 >= _e1876);
            }
            let _e2078 = phi_123_;
            let _e2083 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e619 - _e1123), (_e620 - _e185)), 0f, _e185);
            let _e2085 = ((_e2083 - 2f) * 0.0625f);
            let _e2087 = select(_e2085, 0f, (_e2085 < 0f));
            let _e2089 = select(_e2087, 1f, (_e2087 > 1f));
            let _e2095 = ((select(_e1876, _e2074, _e2078) * ((_e2089 * _e2089) * (3f - (2f * _e2089)))) * _e579);
            let _e2096 = (1f - _e2095);
            let _e2098 = local_16;
            let _e2102 = local_17;
            let _e2106 = local_18;
            let _e2110 = local_19;
            let _e2113 = (0.94f * _e2095);
            let _e2121 = (((_e2110.w * _e2096) + _e2095) * _e596);
            if (_e2121 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e2098.x * _e2096) + _e2113) * _e596), (((_e2102.y * _e2096) + _e2113) * _e596), (((_e2106.z * _e2096) + _e2113) * _e596), _e2121);
            break;
        }
    }
    return;
}

fn render_lyrics_isthmus_lyricspass_vertex_impl() {
    let _e10 = vertex_6;
    let _e11 = _isthmus_instance_index_9;
    let _e14 = line.member[_e11];
    let _e32 = (_e14.min.x + (f32((_e10 & 1u)) * (_e14.max.x - _e14.min.x)));
    let _e33 = (_e14.min.y + (f32((_e10 >> bitcast<u32>(1i))) * (_e14.max.y - _e14.min.y)));
    let _e38 = frame.member[0u].screen_size[0u];
    let _e43 = frame.member[0u].screen_size[1u];
    let _e46 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e32, _e33), vec2<f32>(_e38, _e43));
    out_position = _e46;
    out_pixel[0u] = _e32;
    out_pixel[1u] = _e33;
    out_isthmus_instance_index = _e11;
    return;
}

fn render_lyrics_isthmus_lyricspass_fragment_impl() {
    var phi_0_: u32;
    var phi_1_: u32;
    var phi_2_: u32;
    var phi_3_: u32;
    var phi_4_: u32;
    var local_23: u32;
    var phi_5_: u32;
    var phi_6_: f32;
    var phi_7_: f32;
    var phi_8_: u32;
    var phi_9_: i32;
    var phi_10_: f32;
    var phi_11_: u32;
    var phi_12_: i32;
    var local_24: f32;
    var phi_13_: f32;
    var local_25: i32;
    var phi_14_: bool;
    var phi_15_: f32;
    var phi_16_: f32;
    var phi_17_: f32;
    var phi_18_: f32;
    var phi_19_: f32;
    var phi_20_: u32;
    var phi_21_: f32;
    var phi_22_: bool;
    var phi_23_: f32;
    var phi_24_: f32;
    var phi_25_: bool;
    var phi_26_: f32;
    var phi_27_: bool;
    var phi_28_: f32;
    var phi_29_: bool;
    var local_26: f32;

    let _e36 = pixel_3;
    let _e37 = _isthmus_instance_index_10;
    let _e42 = (_e36.x * 0.03125f);
    let _e44 = select(_e42, 0f, (_e42 < 0f));
    let _e46 = select(_e44, 1f, (_e44 > 1f));
    let _e55 = frame.member[0u].screen_size[0u];
    let _e59 = ((_e36.x - _e55) / ((_e55 - 32f) - _e55));
    let _e61 = select(_e59, 0f, (_e59 < 0f));
    let _e63 = select(_e61, 1f, (_e61 > 1f));
    let _e68 = (((_e46 * _e46) * (3f - (2f * _e46))) * ((_e63 * _e63) * (3f - (2f * _e63))));
    let _e72 = frame.member[0u].playhead_x;
    let _e73 = (_e36.x - _e72);
    let _e76 = ((abs(_e73) - 110f) * -0.009090909f);
    let _e78 = select(_e76, 0f, (_e76 < 0f));
    let _e80 = select(_e78, 1f, (_e78 > 1f));
    let _e86 = (1f + (((_e80 * _e80) * (3f - (2f * _e80))) * 0.24f));
    let _e88 = (_e72 + (_e73 / _e86));
    let _e93 = line.member[_e37].origin[1u];
    let _e96 = (_e93 + ((_e36.y - _e93) / _e86));
    let _e97 = line.member[_e37];
    if (_e88 < _e97.min.x) {
        phi_28_ = f32();
        phi_29_ = true;
    } else {
        if (_e88 > _e97.max.x) {
            phi_26_ = f32();
            phi_27_ = true;
        } else {
            if (_e96 < _e97.min.y) {
                phi_24_ = f32();
                phi_25_ = true;
            } else {
                let _e109 = (_e96 > _e97.max.y);
                if _e109 {
                    phi_23_ = f32();
                } else {
                    let _e111 = (1f / _e97.size);
                    let _e118 = ((_e88 - _e97.origin.x) * _e111);
                    phi_0_ = 0u;
                    phi_1_ = _e97.count;
                    loop {
                        let _e123 = phi_0_;
                        let _e125 = phi_1_;
                        local_23 = _e123;
                        let _e126 = (_e123 < _e125);
                        if _e126 {
                            let _e129 = (_e123 + ((_e125 - _e123) / 2u));
                            let _e134 = placed_glyphs_1.member[(_e97.first + _e129)].x;
                            let _e135 = (_e134 <= _e118);
                            if _e135 {
                                phi_2_ = (_e129 + 1u);
                            } else {
                                phi_2_ = _e123;
                            }
                            let _e138 = phi_2_;
                            phi_3_ = _e138;
                            phi_4_ = select(_e129, _e125, _e135);
                        } else {
                            phi_3_ = u32();
                            phi_4_ = u32();
                        }
                        let _e141 = phi_3_;
                        let _e143 = phi_4_;
                        continue;
                        continuing {
                            phi_0_ = _e141;
                            phi_1_ = _e143;
                            break if !(_e126);
                        }
                    }
                    let _e145 = (3.5f / _e97.size);
                    let _e147 = local_23;
                    let _e148 = (_e147 + 1u);
                    phi_5_ = select(_e148, _e97.count, (_e97.count < _e148));
                    phi_6_ = -1000000f;
                    loop {
                        let _e152 = phi_5_;
                        let _e154 = phi_6_;
                        local_26 = _e154;
                        if (_e152 > 0u) {
                            let _e156 = (_e152 - 1u);
                            let _e157 = (_e97.first + _e156);
                            let _e161 = placed_glyphs_1.member[_e157].x;
                            let _e165 = placed_glyphs_1.member[_e157].glyph;
                            let _e170 = glyphs_1.member[_e165].min[0u];
                            let _e175 = glyphs_1.member[_e165].min[1u];
                            let _e180 = glyphs_1.member[_e165].max[0u];
                            let _e185 = glyphs_1.member[_e165].max[1u];
                            let _e189 = glyphs_1.member[_e165].start;
                            let _e193 = glyphs_1.member[_e165].count;
                            let _e194 = (_e118 - _e161);
                            let _e195 = -(((_e96 - _e97.origin.y) * _e111));
                            let _e196 = (_e180 + _e145);
                            let _e197 = (_e194 > _e196);
                            if _e197 {
                                phi_19_ = f32();
                            } else {
                                if (_e194 >= (_e170 - _e145)) {
                                    if (_e195 >= (_e175 - _e145)) {
                                        if (_e194 <= _e196) {
                                            if (_e195 <= (_e185 + _e145)) {
                                                phi_7_ = 340282350000000000000000000000000000000f;
                                                phi_8_ = 0u;
                                                phi_9_ = 0i;
                                                loop {
                                                    let _e207 = phi_7_;
                                                    let _e209 = phi_8_;
                                                    let _e211 = phi_9_;
                                                    local_24 = _e207;
                                                    local_25 = _e211;
                                                    let _e212 = (_e209 < _e193);
                                                    if _e212 {
                                                        let _e216 = edges_1.member[(_e189 + _e209)];
                                                        let _e218 = cantus_render_text_edge_distance(_e216, _e97.weight, vec2<f32>(_e194, _e195), _e207);
                                                        phi_10_ = _e218.member;
                                                        phi_11_ = (_e209 + 1u);
                                                        phi_12_ = (_e211 + _e218.member_1);
                                                    } else {
                                                        phi_10_ = f32();
                                                        phi_11_ = u32();
                                                        phi_12_ = i32();
                                                    }
                                                    let _e224 = phi_10_;
                                                    let _e226 = phi_11_;
                                                    let _e228 = phi_12_;
                                                    continue;
                                                    continuing {
                                                        phi_7_ = _e224;
                                                        phi_8_ = _e226;
                                                        phi_9_ = _e228;
                                                        break if !(_e212);
                                                    }
                                                }
                                                let _e231 = local_24;
                                                let _e233 = ((_e231 * _e97.size) * _e97.size);
                                                if (_e233 >= 12.25f) {
                                                    phi_13_ = 3.5f;
                                                } else {
                                                    phi_13_ = sqrt(_e233);
                                                }
                                                let _e237 = phi_13_;
                                                let _e239 = local_25;
                                                let _e242 = (_e237 * select(1f, -1f, (_e239 == 0i)));
                                                if (_e154 != _e154) {
                                                    phi_14_ = true;
                                                } else {
                                                    phi_14_ = (_e242 >= _e154);
                                                }
                                                let _e246 = phi_14_;
                                                phi_15_ = select(_e154, _e242, _e246);
                                            } else {
                                                phi_15_ = _e154;
                                            }
                                            let _e249 = phi_15_;
                                            phi_16_ = _e249;
                                        } else {
                                            phi_16_ = _e154;
                                        }
                                        let _e251 = phi_16_;
                                        phi_17_ = _e251;
                                    } else {
                                        phi_17_ = _e154;
                                    }
                                    let _e253 = phi_17_;
                                    phi_18_ = _e253;
                                } else {
                                    phi_18_ = _e154;
                                }
                                let _e255 = phi_18_;
                                phi_19_ = _e255;
                            }
                            let _e257 = phi_19_;
                            phi_20_ = _e156;
                            phi_21_ = _e257;
                            phi_22_ = select(true, false, _e197);
                        } else {
                            phi_20_ = u32();
                            phi_21_ = f32();
                            phi_22_ = false;
                        }
                        let _e260 = phi_20_;
                        let _e262 = phi_21_;
                        let _e264 = phi_22_;
                        continue;
                        continuing {
                            phi_5_ = _e260;
                            phi_6_ = _e262;
                            break if !(_e264);
                        }
                    }
                    let _e375 = local_26;
                    phi_23_ = _e375;
                }
                let _e267 = phi_23_;
                phi_24_ = _e267;
                phi_25_ = _e109;
            }
            let _e269 = phi_24_;
            let _e271 = phi_25_;
            phi_26_ = _e269;
            phi_27_ = _e271;
        }
        let _e273 = phi_26_;
        let _e275 = phi_27_;
        phi_28_ = _e273;
        phi_29_ = _e275;
    }
    let _e277 = phi_28_;
    let _e279 = phi_29_;
    let _e280 = select(_e277, -1000000f, _e279);
    let _e282 = ((_e280 * 1.25f) + 0.5f);
    let _e284 = select(_e282, 0f, (_e282 < 0f));
    let _e286 = select(_e284, 1f, (_e284 > 1f));
    let _e290 = ((_e286 * _e286) * (3f - (2f * _e286)));
    let _e293 = (((_e280 + 0.65f) * 1.25f) + 0.5f);
    let _e295 = select(_e293, 0f, (_e293 < 0f));
    let _e297 = select(_e295, 1f, (_e295 > 1f));
    let _e306 = (_e72 + 4f);
    let _e310 = ((_e36.x - _e306) / ((_e72 - 4f) - _e306));
    let _e312 = select(_e310, 0f, (_e310 < 0f));
    let _e314 = select(_e312, 1f, (_e312 > 1f));
    let _e318 = ((_e314 * _e314) * (3f - (2f * _e314)));
    let _e322 = line.member[_e37].color;
    let _e323 = unpack4x8unorm(_e322);
    let _e330 = (1f - _e318);
    out_color = vec4<f32>(((((_e323.x * _e330) + ((_e323.x * 0.42f) * _e318)) * _e290) * _e68), ((((_e323.y * _e330) + ((_e323.y * 0.42f) * _e318)) * _e290) * _e68), ((((_e323.z * _e330) + ((_e323.z * 0.42f) * _e318)) * _e290) * _e68), ((_e290 + ((((_e297 * _e297) * (3f - (2f * _e297))) - _e290) * 0.5f)) * _e68));
    return;
}

fn render_status_isthmus_statuspass_vertex_impl() {
    var phi_0_: bool;
    var phi_1_: u32;
    var phi_2_: f32;
    var phi_3_: u32;
    var phi_4_: f32;
    var phi_5_: bool;
    var local_27: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e28 = vertex_6;
            let _e29 = _isthmus_instance_index_9;
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

fn cantus_render_shader_sd_rounded_box(param_25: vec2<f32>, param_26: vec2<f32>, param_27: f32) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;

    let _e13 = ((abs(param_25.x) - param_26.x) + param_27);
    let _e14 = ((abs(param_25.y) - param_26.y) + param_27);
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
    return ((sqrt(((_e16 * _e16) + (_e18 * _e18))) + select(_e27, 0f, _e31)) - param_27);
}

fn render_status_isthmus_statuspass_fragment_impl() {
    var phi_0_: bool;
    var phi_1_: f32;
    var phi_2_: vec2<f32>;
    var phi_3_: f32;
    var phi_4_: u32;
    var phi_5_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_6_: bool;
    var phi_7_: vec2<f32>;
    var phi_8_: f32;
    var phi_9_: vec2<f32>;
    var phi_10_: f32;
    var phi_11_: vec2<f32>;
    var phi_12_: f32;
    var phi_13_: u32;
    var phi_14_: bool;
    var phi_15_: f32;
    var local_28: vec2<f32>;
    var local_29: vec2<f32>;
    var phi_16_: bool;
    var local_30: vec2<f32>;
    var phi_17_: f32;
    var local_31: vec2<f32>;
    var phi_18_: bool;
    var phi_19_: bool;
    var phi_20_: bool;
    var phi_21_: bool;
    var phi_22_: bool;
    var phi_23_: bool;
    var phi_24_: bool;
    var phi_25_: bool;
    var phi_26_: u32;
    var phi_27_: u32;
    var phi_28_: u32;
    var phi_29_: u32;
    var phi_30_: bool;
    var phi_31_: f32;
    var phi_32_: bool;
    var phi_33_: bool;
    var phi_34_: bool;
    var phi_35_: vec2<f32>;
    var phi_36_: bool;
    var phi_37_: i32;
    var phi_38_: f32;
    var phi_39_: f32;
    var phi_40_: vec2<f32>;
    var phi_41_: i32;
    var phi_42_: f32;
    var phi_43_: f32;
    var phi_44_: vec2<f32>;
    var local_32: f32;
    var phi_45_: vec2<f32>;
    var phi_46_: i32;
    var phi_47_: f32;
    var phi_48_: f32;
    var phi_49_: vec2<f32>;
    var phi_50_: i32;
    var phi_51_: f32;
    var phi_52_: f32;
    var phi_53_: vec2<f32>;
    var local_33: f32;
    var phi_54_: vec2<f32>;
    var phi_55_: vec2<f32>;
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
    var phi_67_: bool;
    var phi_68_: bool;
    var phi_69_: bool;
    var phi_70_: bool;
    var phi_71_: bool;
    var phi_72_: bool;
    var phi_73_: bool;
    var phi_74_: vec3<f32>;
    var phi_75_: bool;
    var phi_76_: bool;
    var phi_77_: bool;
    var phi_78_: bool;
    var phi_79_: bool;
    var phi_80_: f32;
    var phi_81_: bool;
    var phi_82_: vec3<f32>;
    var local_34: f32;
    var local_35: f32;
    var phi_83_: render_text_Line;
    var phi_84_: bool;
    var phi_85_: u32;
    var phi_86_: u32;
    var phi_87_: u32;
    var phi_88_: u32;
    var phi_89_: u32;
    var local_36: u32;
    var phi_90_: u32;
    var phi_91_: f32;
    var phi_92_: f32;
    var phi_93_: u32;
    var phi_94_: i32;
    var phi_95_: f32;
    var phi_96_: u32;
    var phi_97_: i32;
    var local_37: f32;
    var phi_98_: f32;
    var local_38: i32;
    var phi_99_: bool;
    var phi_100_: f32;
    var phi_101_: f32;
    var phi_102_: f32;
    var phi_103_: f32;
    var phi_104_: f32;
    var phi_105_: u32;
    var phi_106_: f32;
    var phi_107_: bool;
    var phi_108_: f32;
    var phi_109_: f32;
    var phi_110_: bool;
    var phi_111_: f32;
    var phi_112_: bool;
    var phi_113_: f32;
    var phi_114_: bool;
    var phi_115_: f32;
    var local_39: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e231 = pixel_3;
            let _e232 = _isthmus_instance_index_10;
            let _e238 = pill_1.member[_e232].battery_level;
            let _e239 = (_e238 >= -1f);
            if _e239 {
                phi_0_ = (_e238 <= 1f);
            } else {
                phi_0_ = false;
            }
            let _e242 = phi_0_;
            let _e244 = (select(0f, 40f, _e242) + 296f);
            let _e249 = frame.member[0u].screen_size[0u];
            let _e251 = ((_e249 - _e244) - 8f);
            let _e255 = frame.member[0u].panel_height;
            let _e256 = (_e231.x - _e251);
            let _e257 = (_e231.y - 6f);
            let _e258 = (_e244 * 0.5f);
            let _e259 = (_e255 * 0.5f);
            let _e263 = ((_e244 - _e255) * 0.5f);
            let _e265 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e256 - _e258), (_e257 - _e259)), _e263, _e259);
            let _e269 = frame.member[0u].mouse_pressure;
            let _e270 = (_e269 > 0f);
            if _e270 {
                let _e275 = frame.member[0u].mouse_pos[0u];
                let _e280 = frame.member[0u].mouse_pos[1u];
                let _e286 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e275 - _e251) - _e258), ((_e280 - 6f) - _e259)), _e263, _e259);
                phi_1_ = _e286;
            } else {
                phi_1_ = 1f;
            }
            let _e288 = phi_1_;
            phi_2_ = vec2<f32>(0f, 0f);
            phi_3_ = 0f;
            phi_4_ = 0u;
            loop {
                let _e290 = phi_2_;
                let _e292 = phi_3_;
                let _e294 = phi_4_;
                local_28 = _e290;
                local_29 = _e290;
                local_30 = _e290;
                local_31 = _e290;
                local_34 = _e292;
                local_35 = _e292;
                let _e295 = (_e294 < 4u);
                if _e295 {
                    if _e295 {
                    } else {
                        phi_14_ = true;
                        break;
                    }
                    let _e302 = frame.member[0u].ripples[_e294].origin[0u];
                    let _e309 = frame.member[0u].ripples[_e294].origin[1u];
                    let _e315 = frame.member[0u].ripples[_e294].start_time;
                    let _e321 = frame.member[0u].ripples[_e294].strength;
                    let _e325 = frame.member[0u].time;
                    let _e327 = ((_e325 - _e315) * 1.2f);
                    let _e329 = select(_e327, 0f, (_e327 < 0f));
                    let _e331 = select(_e329, 1f, (_e329 > 1f));
                    if (_e321 > 0f) {
                        if (_e331 < 1f) {
                            let _e335 = (_e231 - vec2<f32>(_e302, _e309));
                            let _e341 = sqrt(((_e335.x * _e335.x) + (_e335.y * _e335.y)));
                            if (_e341 > 0.001f) {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e335.x / _e341), (_e335.y / _e341)), _e341);
                            } else {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e341);
                            }
                            let _e349 = phi_5_;
                            let _e359 = ((abs((_e349.unnamed_1 - (_e331 * 600f))) - 80f) * -0.0125f);
                            let _e361 = select(_e359, 0f, (_e359 < 0f));
                            let _e363 = select(_e361, 1f, (_e361 > 1f));
                            let _e369 = (1f - _e331);
                            let _e370 = ((((_e363 * _e363) * (3f - (2f * _e363))) * _e321) * _e369);
                            let _e383 = (_e292 + (_e370 * 0.5f));
                            if (_e383 != _e383) {
                                phi_6_ = true;
                            } else {
                                phi_6_ = (1f <= _e383);
                            }
                            let _e387 = phi_6_;
                            phi_7_ = vec2<f32>((_e290.x + (((_e349.unnamed.x * _e370) * _e369) * 0.5f)), (_e290.y + (((_e349.unnamed.y * _e370) * _e369) * 0.5f)));
                            phi_8_ = select(_e383, 1f, _e387);
                        } else {
                            phi_7_ = _e290;
                            phi_8_ = _e292;
                        }
                        let _e390 = phi_7_;
                        let _e392 = phi_8_;
                        phi_9_ = _e390;
                        phi_10_ = _e392;
                    } else {
                        phi_9_ = _e290;
                        phi_10_ = _e292;
                    }
                    let _e394 = phi_9_;
                    let _e396 = phi_10_;
                    phi_11_ = _e394;
                    phi_12_ = _e396;
                    phi_13_ = (_e294 + 1u);
                } else {
                    phi_11_ = vec2<f32>();
                    phi_12_ = f32();
                    phi_13_ = u32();
                }
                let _e399 = phi_11_;
                let _e401 = phi_12_;
                let _e403 = phi_13_;
                continue;
                continuing {
                    phi_2_ = _e399;
                    phi_3_ = _e401;
                    phi_4_ = _e403;
                    phi_14_ = false;
                    break if !(_e295);
                }
            }
            let _e406 = phi_14_;
            if _e406 {
                break;
            }
            if _e270 {
                let _e411 = frame.member[0u].mouse_pos[0u];
                let _e416 = frame.member[0u].mouse_pos[1u];
                let _e417 = (_e231.x - _e411);
                let _e418 = (_e231.y - _e416);
                let _e424 = ((sqrt(((_e417 * _e417) + (_e418 * _e418))) - 150f) * -0.006666667f);
                let _e426 = select(_e424, 0f, (_e424 < 0f));
                let _e428 = select(_e426, 1f, (_e426 > 1f));
                phi_15_ = ((((_e428 * _e428) * (3f - (2f * _e428))) * _e269) * 8f);
            } else {
                phi_15_ = 0f;
            }
            let _e436 = phi_15_;
            let _e438 = local_28;
            let _e441 = global[0u];
            if (_e438.x == _e441) {
                let _e444 = local_29;
                let _e447 = global[1u];
                phi_16_ = (_e444.y == _e447);
            } else {
                phi_16_ = false;
            }
            let _e450 = phi_16_;
            if _e450 {
                phi_17_ = 0f;
            } else {
                let _e452 = local_30;
                phi_17_ = (sqrt(((_e438.x * _e438.x) + (_e452.y * _e452.y))) * 22f);
            }
            let _e460 = phi_17_;
            let _e462 = local_31;
            let _e465 = ((_e288 - 0.5f) * -1f);
            let _e467 = select(_e465, 0f, (_e465 < 0f));
            let _e469 = select(_e467, 1f, (_e467 > 1f));
            let _e477 = (_e265 - (((_e436 * ((_e469 * _e469) * (3f - (2f * _e469)))) + _e460) * 0.5f));
            let _e478 = fwidth(_e477);
            if (_e478 != _e478) {
                phi_18_ = true;
            } else {
                phi_18_ = (0.55f >= _e478);
            }
            let _e482 = phi_18_;
            let _e483 = select(_e478, 0.55f, _e482);
            let _e487 = ((_e477 - _e483) / (-(_e483) - _e483));
            let _e489 = select(_e487, 0f, (_e487 < 0f));
            let _e491 = select(_e489, 1f, (_e489 > 1f));
            let _e495 = ((_e491 * _e491) * (3f - (2f * _e491)));
            let _e496 = (_e477 != _e477);
            if _e496 {
                phi_19_ = true;
            } else {
                phi_19_ = (0f >= _e477);
            }
            let _e499 = phi_19_;
            let _e503 = (exp((select(_e477, 0f, _e499) * -0.3f)) * 0.16f);
            if (_e495 != _e495) {
                phi_20_ = true;
            } else {
                phi_20_ = (_e503 >= _e495);
            }
            let _e507 = phi_20_;
            let _e508 = select(_e495, _e503, _e507);
            if (_e508 <= 0.0009765625f) {
                discard;
            }
            let _e510 = (_e256 / _e244);
            let _e511 = (_e257 / _e255);
            if _e496 {
                phi_21_ = true;
            } else {
                phi_21_ = (0f <= _e477);
            }
            let _e516 = phi_21_;
            let _e519 = (1f + (select(_e477, 0f, _e516) * 0.008333334f));
            let _e521 = select(_e519, 0f, (_e519 < 0f));
            let _e523 = select(_e521, 0.6f, (_e521 > 0.6f));
            let _e533 = ((_e511 - (((_e511 - 0.5f) * _e523) * 0.08f)) - (_e462.y * 0.04f));
            let _e537 = pill_1.member[_e232].sun_height;
            let _e541 = pill_1.member[_e232].conditions;
            let _e545 = frame.member[0u].time;
            let _e553 = ((_e533 - 1f) * -1f);
            let _e555 = select(_e553, 0f, (_e553 < 0f));
            let _e557 = select(_e555, 1f, (_e555 > 1f));
            let _e561 = ((_e557 * _e557) * (3f - (2f * _e557)));
            let _e563 = ((_e537 - -0.04f) * 4.1666665f);
            let _e565 = select(_e563, 0f, (_e563 < 0f));
            let _e567 = select(_e565, 1f, (_e565 > 1f));
            let _e571 = ((_e567 * _e567) * (3f - (2f * _e567)));
            let _e573 = ((_e537 - -0.2f) * 4.5454545f);
            let _e575 = select(_e573, 0f, (_e573 < 0f));
            let _e577 = select(_e575, 1f, (_e575 > 1f));
            let _e582 = (1f - _e571);
            let _e583 = (((_e577 * _e577) * (3f - (2f * _e577))) * _e582);
            let _e584 = (1f - _e561);
            let _e596 = (0.65f * _e584);
            let _e620 = (1f - _e583);
            let _e634 = (((_e541.cloud * 0.34f) + (_e541.rain * 0.16f)) + (_e541.hail * 0.08f));
            let _e635 = (1f - _e634);
            let _e646 = (1f - (_e541.snow * 0.16f));
            let _e650 = (_e541.snow * 0.1312f);
            let _e655 = (1f - (_e541.fog * 0.62f));
            let _e668 = ((sin((_e545 * 2.7f)) - 0.92f) * 12.500003f);
            let _e670 = select(_e668, 0f, (_e668 < 0f));
            let _e672 = select(_e670, 1f, (_e670 > 1f));
            let _e677 = (((_e672 * _e672) * (3f - (2f * _e672))) * _e541.lightning);
            let _e679 = (1f - (_e677 * 0.45f));
            let _e690 = ((_e533 - 0.12f) * -8.333334f);
            let _e692 = select(_e690, 0f, (_e690 < 0f));
            let _e694 = select(_e692, 1f, (_e692 > 1f));
            let _e701 = ((_e477 - 5f) * -0.125f);
            let _e703 = select(_e701, 0f, (_e701 < 0f));
            let _e705 = select(_e703, 1f, (_e703 > 1f));
            let _e711 = ((((_e694 * _e694) * (3f - (2f * _e694))) * 0.12f) + (((_e705 * _e705) * (3f - (2f * _e705))) * 0.08f));
            let _e715 = (((_e510 - (((_e510 - 0.5f) * _e523) * 0.08f)) - (_e438.x * 0.04f)) * _e244);
            let _e716 = (_e533 * _e255);
            if (_e715 < 96f) {
                phi_29_ = 0u;
            } else {
                if (_e715 < 184f) {
                    phi_28_ = 1u;
                } else {
                    if _e239 {
                        phi_22_ = (_e238 <= 1f);
                    } else {
                        phi_22_ = false;
                    }
                    let _e721 = phi_22_;
                    if _e721 {
                        phi_23_ = select(true, false, (_e715 < 224f));
                    } else {
                        phi_23_ = true;
                    }
                    let _e725 = phi_23_;
                    if _e725 {
                        if _e239 {
                            phi_24_ = (_e238 <= 1f);
                        } else {
                            phi_24_ = false;
                        }
                        let _e728 = phi_24_;
                        if (_e715 < (select(0f, 40f, _e728) + 224f)) {
                            phi_26_ = 3u;
                        } else {
                            if _e239 {
                                phi_25_ = (_e238 <= 1f);
                            } else {
                                phi_25_ = false;
                            }
                            let _e734 = phi_25_;
                            phi_26_ = select(5u, 4u, (_e715 < (select(0f, 40f, _e734) + 256f)));
                        }
                        let _e740 = phi_26_;
                        phi_27_ = _e740;
                    } else {
                        phi_27_ = 2u;
                    }
                    let _e742 = phi_27_;
                    phi_28_ = _e742;
                }
                let _e744 = phi_28_;
                phi_29_ = _e744;
            }
            let _e746 = phi_29_;
            if _e239 {
                phi_30_ = (_e238 <= 1f);
            } else {
                phi_30_ = false;
            }
            let _e749 = phi_30_;
            let _e750 = select(0f, 40f, _e749);
            switch bitcast<i32>(_e746) {
                case 0: {
                    phi_31_ = 12f;
                    break;
                }
                case 1: {
                    phi_31_ = 100f;
                    break;
                }
                case 2: {
                    phi_31_ = 188f;
                    break;
                }
                case 3: {
                    phi_31_ = (188f + _e750);
                    break;
                }
                case 4: {
                    phi_31_ = (228f + _e750);
                    break;
                }
                case 5: {
                    phi_31_ = (260f + _e750);
                    break;
                }
                default: {
                    phi_31_ = f32();
                    break;
                }
            }
            let _e756 = phi_31_;
            switch bitcast<i32>(_e746) {
                case 0: {
                    phi_32_ = true;
                    phi_33_ = false;
                    phi_34_ = false;
                    break;
                }
                case 1: {
                    phi_32_ = true;
                    phi_33_ = false;
                    phi_34_ = false;
                    break;
                }
                case 2: {
                    phi_32_ = false;
                    phi_33_ = true;
                    phi_34_ = false;
                    break;
                }
                case 3: {
                    phi_32_ = false;
                    phi_33_ = true;
                    phi_34_ = false;
                    break;
                }
                case 4: {
                    phi_32_ = false;
                    phi_33_ = false;
                    phi_34_ = true;
                    break;
                }
                case 5: {
                    phi_32_ = false;
                    phi_33_ = false;
                    phi_34_ = true;
                    break;
                }
                default: {
                    phi_32_ = bool();
                    phi_33_ = bool();
                    phi_34_ = bool();
                    break;
                }
            }
            let _e759 = phi_32_;
            let _e761 = phi_33_;
            let _e763 = phi_34_;
            let _e764 = select(_e761, false, _e759);
            let _e771 = (_e715 - (_e756 + (select(select(80f, 32f, _e764), 24f, select(select(_e763, false, _e759), false, _e764)) * 0.5f)));
            let _e772 = (_e716 - _e259);
            switch bitcast<i32>(_e746) {
                case 0: {
                    phi_35_ = vec2<f32>();
                    phi_36_ = true;
                    break;
                }
                case 1: {
                    phi_35_ = vec2<f32>();
                    phi_36_ = true;
                    break;
                }
                default: {
                    phi_35_ = vec2<f32>(0f, 0f);
                    phi_36_ = false;
                    break;
                }
            }
            let _e775 = phi_35_;
            let _e777 = phi_36_;
            if _e777 {
                let _e778 = (_e715 - 52f);
                let _e783 = pill_1.member[_e232].cpu.temperature;
                if (_e783 <= 62f) {
                    phi_45_ = vec2<f32>(0f, 0f);
                } else {
                    let _e786 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e778, _e772), 13f, 13f);
                    phi_37_ = 0i;
                    phi_38_ = 0.5f;
                    phi_39_ = 0f;
                    phi_40_ = vec2<f32>(((_e778 + (_e545 * 1.8f)) * 0.035f), (((_e772 + -(_e545)) * 0.035f) + 6.1f));
                    loop {
                        let _e796 = phi_37_;
                        let _e798 = phi_38_;
                        let _e800 = phi_39_;
                        let _e802 = phi_40_;
                        local_32 = _e800;
                        let _e803 = (_e796 < 4i);
                        if _e803 {
                            let _e806 = cantus_render_shader_simplex_noise(_e802);
                            phi_41_ = (_e796 + 1i);
                            phi_42_ = (_e798 * 0.5f);
                            phi_43_ = (_e800 + (_e806 * _e798));
                            phi_44_ = vec2<f32>(((_e802.x * 1.6f) + (_e802.y * 1.2f)), ((_e802.y * 1.6f) - (_e802.x * 1.2f)));
                        } else {
                            phi_41_ = i32();
                            phi_42_ = f32();
                            phi_43_ = f32();
                            phi_44_ = vec2<f32>();
                        }
                        let _e819 = phi_41_;
                        let _e821 = phi_42_;
                        let _e823 = phi_43_;
                        let _e825 = phi_44_;
                        continue;
                        continuing {
                            phi_37_ = _e819;
                            phi_38_ = _e821;
                            phi_39_ = _e823;
                            phi_40_ = _e825;
                            break if !(_e803);
                        }
                    }
                    let _e828 = local_32;
                    let _e829 = (_e828 * 0.5f);
                    let _e832 = ((_e786 - -0.5f) * 0.5f);
                    let _e834 = select(_e832, 0f, (_e832 < 0f));
                    let _e836 = select(_e834, 1f, (_e834 > 1f));
                    let _e842 = ((_e786 - 14f) * -0.083333336f);
                    let _e844 = select(_e842, 0f, (_e842 < 0f));
                    let _e846 = select(_e844, 1f, (_e844 > 1f));
                    let _e851 = (((_e836 * _e836) * (3f - (2f * _e836))) * ((_e846 * _e846) * (3f - (2f * _e846))));
                    let _e856 = ((_e829 + 0.19999999f) * 3.125f);
                    let _e858 = select(_e856, 0f, (_e856 < 0f));
                    let _e860 = select(_e858, 1f, (_e858 > 1f));
                    let _e867 = ((_e783 - 62f) * 0.045454547f);
                    let _e869 = select(_e867, 0f, (_e867 < 0f));
                    let _e871 = select(_e869, 1f, (_e869 > 1f));
                    let _e875 = ((_e871 * _e871) * (3f - (2f * _e871)));
                    phi_45_ = vec2<f32>(((_e851 * (0.18f + ((0.5f + _e829) * 0.34f))) * _e875), ((_e851 * ((_e860 * _e860) * (3f - (2f * _e860)))) * _e875));
                }
                let _e880 = phi_45_;
                let _e883 = (_e715 - 140f);
                let _e888 = pill_1.member[_e232].gpu.temperature;
                if (_e888 <= 62f) {
                    phi_54_ = vec2<f32>(0f, 0f);
                } else {
                    let _e891 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e883, _e772), 13f, 13f);
                    phi_46_ = 0i;
                    phi_47_ = 0.5f;
                    phi_48_ = 0f;
                    phi_49_ = vec2<f32>(((_e883 + (_e545 * 1.8f)) * 0.035f), (((_e772 + -(_e545)) * 0.035f) + 6.1f));
                    loop {
                        let _e901 = phi_46_;
                        let _e903 = phi_47_;
                        let _e905 = phi_48_;
                        let _e907 = phi_49_;
                        local_33 = _e905;
                        let _e908 = (_e901 < 4i);
                        if _e908 {
                            let _e911 = cantus_render_shader_simplex_noise(_e907);
                            phi_50_ = (_e901 + 1i);
                            phi_51_ = (_e903 * 0.5f);
                            phi_52_ = (_e905 + (_e911 * _e903));
                            phi_53_ = vec2<f32>(((_e907.x * 1.6f) + (_e907.y * 1.2f)), ((_e907.y * 1.6f) - (_e907.x * 1.2f)));
                        } else {
                            phi_50_ = i32();
                            phi_51_ = f32();
                            phi_52_ = f32();
                            phi_53_ = vec2<f32>();
                        }
                        let _e924 = phi_50_;
                        let _e926 = phi_51_;
                        let _e928 = phi_52_;
                        let _e930 = phi_53_;
                        continue;
                        continuing {
                            phi_46_ = _e924;
                            phi_47_ = _e926;
                            phi_48_ = _e928;
                            phi_49_ = _e930;
                            break if !(_e908);
                        }
                    }
                    let _e933 = local_33;
                    let _e934 = (_e933 * 0.5f);
                    let _e937 = ((_e891 - -0.5f) * 0.5f);
                    let _e939 = select(_e937, 0f, (_e937 < 0f));
                    let _e941 = select(_e939, 1f, (_e939 > 1f));
                    let _e947 = ((_e891 - 14f) * -0.083333336f);
                    let _e949 = select(_e947, 0f, (_e947 < 0f));
                    let _e951 = select(_e949, 1f, (_e949 > 1f));
                    let _e956 = (((_e941 * _e941) * (3f - (2f * _e941))) * ((_e951 * _e951) * (3f - (2f * _e951))));
                    let _e961 = ((_e934 + 0.19999999f) * 3.125f);
                    let _e963 = select(_e961, 0f, (_e961 < 0f));
                    let _e965 = select(_e963, 1f, (_e963 > 1f));
                    let _e972 = ((_e888 - 62f) * 0.045454547f);
                    let _e974 = select(_e972, 0f, (_e972 < 0f));
                    let _e976 = select(_e974, 1f, (_e974 > 1f));
                    let _e980 = ((_e976 * _e976) * (3f - (2f * _e976)));
                    phi_54_ = vec2<f32>(((_e956 * (0.18f + ((0.5f + _e934) * 0.34f))) * _e980), ((_e956 * ((_e965 * _e965) * (3f - (2f * _e965)))) * _e980));
                }
                let _e985 = phi_54_;
                phi_55_ = vec2<f32>(select(_e985.x, _e880.x, (_e880.x > _e985.x)), select(_e985.y, _e880.y, (_e880.y > _e985.y)));
            } else {
                phi_55_ = _e775;
            }
            let _e994 = phi_55_;
            let _e999 = pill_1.member[_e232].cpu.temperature;
            let _e1004 = pill_1.member[_e232].gpu.temperature;
            if (_e999 != _e999) {
                phi_56_ = true;
            } else {
                phi_56_ = (_e1004 >= _e999);
            }
            let _e1008 = phi_56_;
            let _e1009 = select(_e999, _e1004, _e1008);
            let _e1011 = ((_e1009 - 60f) * 0.083333336f);
            let _e1013 = select(_e1011, 0f, (_e1011 < 0f));
            let _e1015 = select(_e1013, 1f, (_e1013 > 1f));
            let _e1019 = ((_e1015 * _e1015) * (3f - (2f * _e1015)));
            let _e1020 = (1f - _e1019);
            let _e1029 = ((_e1009 - 72f) * 0.0625f);
            let _e1031 = select(_e1029, 0f, (_e1029 < 0f));
            let _e1033 = select(_e1031, 1f, (_e1031 > 1f));
            let _e1037 = ((_e1033 * _e1033) * (3f - (2f * _e1033)));
            let _e1038 = (1f - _e1037);
            let _e1048 = (_e994.y * 0.12f);
            let _e1049 = (0.24f + _e1048);
            let _e1050 = (0.76f - _e1048);
            let _e1062 = (1f - (_e994.x * 0.46f));
            let _e1072 = (_e994.y * 0.64f);
            let _e1073 = (1f - _e1072);
            let _e1080 = (((((((((((((((((((0.008f * _e584) + (0.03f * _e561)) * _e582) + (((0.09f * _e584) + (0.34f * _e561)) * _e571)) * _e620) + ((_e596 + (0.3f * _e561)) * _e583)) * _e635) + (0.16f * _e634)) * _e646) + _e650) * _e655) + (_e541.fog * 0.3844f)) * _e679) + (_e677 * 0.2925f)) + _e711) * _e1062) + (_e994.x * 0.0009200001f)) * _e1073) + (((0.07f * _e1050) + (((((0.22f * _e1020) + _e1019) * _e1038) + _e1037) * _e1049)) * _e1072));
            let _e1081 = (((((((((((((((((((0.015f * _e584) + (0.06f * _e561)) * _e582) + (((0.37f * _e584) + (0.7f * _e561)) * _e571)) * _e620) + (((0.25f * _e584) + (0.2f * _e561)) * _e583)) * _e635) + (0.2f * _e634)) * _e646) + _e650) * _e655) + (_e541.fog * 0.4216f)) * _e679) + (_e677 * 0.333f)) + _e711) * _e1062) + (_e994.x * 0.00276f)) * _e1073) + (((0.12f * _e1050) + (((((0.62f * _e1020) + (0.38f * _e1019)) * _e1038) + (0.08f * _e1037)) * _e1049)) * _e1072));
            let _e1082 = (((((((((((((((((((0.04f * _e584) + (0.13f * _e561)) * _e582) + ((_e596 + (0.9f * _e561)) * _e571)) * _e620) + (((0.2f * _e584) + (0.4f * _e561)) * _e583)) * _e635) + (0.27f * _e634)) * _e646) + _e650) * _e655) + (_e541.fog * 0.44640002f)) * _e679) + (_e677 * 0.43199998f)) + _e711) * _e1062) + (_e994.x * 0.00552f)) * _e1073) + (((0.18f * _e1050) + ((((_e1020 + (0.08f * _e1019)) * _e1038) + (0.035f * _e1037)) * _e1049)) * _e1072));
            switch bitcast<i32>(_e746) {
                case 0: {
                    let _e1798 = pill_1.member[_e232].history_scroll;
                    switch bitcast<i32>(_e746) {
                        case 0: {
                            phi_63_ = true;
                            phi_64_ = false;
                            phi_65_ = false;
                            break;
                        }
                        case 1: {
                            phi_63_ = true;
                            phi_64_ = false;
                            phi_65_ = false;
                            break;
                        }
                        case 2: {
                            phi_63_ = false;
                            phi_64_ = true;
                            phi_65_ = false;
                            break;
                        }
                        case 3: {
                            phi_63_ = false;
                            phi_64_ = true;
                            phi_65_ = false;
                            break;
                        }
                        case 4: {
                            phi_63_ = false;
                            phi_64_ = false;
                            phi_65_ = true;
                            break;
                        }
                        case 5: {
                            phi_63_ = false;
                            phi_64_ = false;
                            phi_65_ = true;
                            break;
                        }
                        default: {
                            phi_63_ = bool();
                            phi_64_ = bool();
                            phi_65_ = bool();
                            break;
                        }
                    }
                    let _e1801 = phi_63_;
                    let _e1803 = phi_64_;
                    let _e1805 = phi_65_;
                    let _e1806 = select(_e1803, false, _e1801);
                    let _e1812 = ((select(select(80f, 32f, _e1806), 24f, select(select(_e1805, false, _e1801), false, _e1806)) * 0.5f) - 4f);
                    let _e1813 = (_e259 - 8f);
                    let _e1814 = (_e1812 - _e1813);
                    let _e1816 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e771, _e772), _e1814, _e1813);
                    let _e1817 = abs(_e771);
                    let _e1818 = abs(_e772);
                    let _e1821 = (round((_e1817 * 0.11111111f)) * 9f);
                    if (_e1821 != _e1821) {
                        phi_66_ = true;
                    } else {
                        phi_66_ = (_e1812 <= _e1821);
                    }
                    let _e1825 = phi_66_;
                    let _e1826 = select(_e1821, _e1812, _e1825);
                    let _e1827 = (_e1826 - _e1814);
                    if (_e1827 != _e1827) {
                        phi_67_ = true;
                    } else {
                        phi_67_ = (0f >= _e1827);
                    }
                    let _e1831 = phi_67_;
                    let _e1832 = select(_e1827, 0f, _e1831);
                    let _e1833 = (_e1813 * _e1813);
                    let _e1836 = sqrt((_e1833 - (_e1832 * _e1832)));
                    let _e1837 = (_e1832 / _e1813);
                    let _e1838 = (_e1836 / _e1813);
                    let _e1843 = ((_e1817 - _e1826) - (_e1837 * 0.9f));
                    let _e1844 = ((_e1818 - _e1836) - (_e1838 * 0.9f));
                    let _e1853 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1843 * -(_e1838)) + (_e1844 * _e1837)), ((_e1843 * _e1837) + (_e1844 * _e1838))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e1855 = round((_e1818 * 0.125f));
                    if (_e1855 != _e1855) {
                        phi_68_ = true;
                    } else {
                        phi_68_ = (1f <= _e1855);
                    }
                    let _e1859 = phi_68_;
                    let _e1861 = (select(_e1855, 1f, _e1859) * 8f);
                    let _e1864 = sqrt((_e1833 - (_e1861 * _e1861)));
                    let _e1866 = (_e1864 / _e1813);
                    let _e1867 = (_e1861 / _e1813);
                    let _e1872 = ((_e1817 - (_e1814 + _e1864)) - (_e1866 * 0.9f));
                    let _e1873 = ((_e1818 - _e1861) - (_e1867 * 0.9f));
                    let _e1882 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1872 * -(_e1867)) + (_e1873 * _e1866)), ((_e1872 * _e1866) + (_e1873 * _e1867))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e1853 != _e1853) {
                        phi_69_ = true;
                    } else {
                        phi_69_ = (_e1882 <= _e1853);
                    }
                    let _e1886 = phi_69_;
                    let _e1887 = select(_e1853, _e1882, _e1886);
                    let _e1890 = (0.5f + ((_e1887 - _e1816) * 0.3125f));
                    let _e1892 = select(_e1890, 0f, (_e1890 < 0f));
                    let _e1894 = select(_e1892, 1f, (_e1892 > 1f));
                    let _e1903 = ((_e1816 - 0.55f) * -0.9090909f);
                    let _e1905 = select(_e1903, 0f, (_e1903 < 0f));
                    let _e1907 = select(_e1905, 1f, (_e1905 > 1f));
                    let _e1911 = ((_e1907 * _e1907) * (3f - (2f * _e1907)));
                    let _e1912 = (_e1812 * 0.051282052f);
                    let _e1913 = (_e771 + _e1812);
                    let _e1915 = ((_e1913 / _e1912) + _e1798);
                    let _e1917 = select(_e1915, 0f, (_e1915 < 0f));
                    let _e1919 = select(_e1917, 39f, (_e1917 > 39f));
                    let _e1920 = floor(_e1919);
                    let _e1925 = select(select(u32(_e1920), 0u, (_e1920 < 0f)), 4294967295u, (_e1920 > 4294967000f));
                    let _e1926 = (_e259 - 10f);
                    let _e1930 = (((f32(_e1925) - _e1798) * _e1912) - _e1812);
                    let _e1932 = select(_e1925, 39u, (39u < _e1925));
                    let _e1933 = (_e1932 < 40u);
                    if _e1933 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1940 = pill_1.member[_e232].cpu.usage.samples[_e1932];
                    let _e1943 = (_e1926 * (1f - (_e1940 * 2f)));
                    let _e1944 = (_e1925 + 1u);
                    let _e1950 = select(_e1944, 39u, (39u < _e1944));
                    let _e1951 = (_e1950 < 40u);
                    if _e1951 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1958 = pill_1.member[_e232].cpu.usage.samples[_e1950];
                    let _e1962 = ((((f32(_e1944) - _e1798) * _e1912) - _e1812) - _e1930);
                    let _e1963 = ((_e1926 * (1f - (_e1958 * 2f))) - _e1943);
                    let _e1964 = (_e771 - _e1930);
                    let _e1965 = (_e772 - _e1943);
                    let _e1966 = (_e1964 * _e1962);
                    let _e1969 = (_e1962 * _e1962);
                    let _e1971 = (_e1969 + (_e1963 * _e1963));
                    if (_e1971 != _e1971) {
                        phi_70_ = true;
                    } else {
                        phi_70_ = (0.001f >= _e1971);
                    }
                    let _e1975 = phi_70_;
                    let _e1977 = ((_e1966 + (_e1965 * _e1963)) / select(_e1971, 0.001f, _e1975));
                    let _e1979 = select(_e1977, 0f, (_e1977 < 0f));
                    let _e1981 = select(_e1979, 1f, (_e1979 > 1f));
                    let _e1984 = (_e1964 - (_e1962 * _e1981));
                    let _e1985 = (_e1965 - (_e1963 * _e1981));
                    let _e1992 = ((abs(sqrt(((_e1984 * _e1984) + (_e1985 * _e1985)))) - 1.4000001f) * -0.9090908f);
                    let _e1994 = select(_e1992, 0f, (_e1992 < 0f));
                    let _e1996 = select(_e1994, 1f, (_e1994 > 1f));
                    let _e2002 = (_e1919 - trunc(_e1919));
                    let _e2004 = select(_e2002, 0f, (_e2002 < 0f));
                    let _e2006 = select(_e2004, 1f, (_e2004 > 1f));
                    let _e2010 = ((_e2006 * _e2006) * (3f - (2f * _e2006)));
                    let _e2017 = ((((_e1943 + (_e1963 * _e2010)) - _e772) - 0.55f) * -0.9090909f);
                    let _e2019 = select(_e2017, 0f, (_e2017 < 0f));
                    let _e2021 = select(_e2019, 1f, (_e2019 > 1f));
                    let _e2027 = ((((_e2021 * _e2021) * (3f - (2f * _e2021))) * 0.156f) + ((_e1996 * _e1996) * (3f - (2f * _e1996))));
                    if _e1933 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e2036 = pill_1.member[_e232].cpu.memory.samples[_e1932];
                    let _e2039 = (_e1926 * (1f - (_e2036 * 2f)));
                    if _e1951 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e2046 = pill_1.member[_e232].cpu.memory.samples[_e1950];
                    let _e2050 = ((_e1926 * (1f - (_e2046 * 2f))) - _e2039);
                    let _e2051 = (_e772 - _e2039);
                    let _e2055 = (_e1969 + (_e2050 * _e2050));
                    if (_e2055 != _e2055) {
                        phi_71_ = true;
                    } else {
                        phi_71_ = (0.001f >= _e2055);
                    }
                    let _e2059 = phi_71_;
                    let _e2061 = ((_e1966 + (_e2051 * _e2050)) / select(_e2055, 0.001f, _e2059));
                    let _e2063 = select(_e2061, 0f, (_e2061 < 0f));
                    let _e2065 = select(_e2063, 1f, (_e2063 > 1f));
                    let _e2068 = (_e1964 - (_e1962 * _e2065));
                    let _e2069 = (_e2051 - (_e2050 * _e2065));
                    let _e2076 = ((abs(sqrt(((_e2068 * _e2068) + (_e2069 * _e2069)))) - 1.4000001f) * -0.9090908f);
                    let _e2078 = select(_e2076, 0f, (_e2076 < 0f));
                    let _e2080 = select(_e2078, 1f, (_e2078 > 1f));
                    let _e2091 = ((((_e2039 + (_e2050 * _e2010)) - _e772) - 0.55f) * -0.9090909f);
                    let _e2093 = select(_e2091, 0f, (_e2091 < 0f));
                    let _e2095 = select(_e2093, 1f, (_e2093 > 1f));
                    let _e2101 = ((((_e2095 * _e2095) * (3f - (2f * _e2095))) * 0.084f) + ((_e2080 * _e2080) * (3f - (2f * _e2080))));
                    let _e2109 = (_e1913 * 0.14285715f);
                    let _e2110 = ((_e772 + _e1813) * 0.16393442f);
                    let _e2120 = ((abs(((_e2109 - trunc(_e2109)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2122 = select(_e2120, 0f, (_e2120 < 0f));
                    let _e2124 = select(_e2122, 1f, (_e2122 > 1f));
                    let _e2128 = ((_e2124 * _e2124) * (3f - (2f * _e2124)));
                    let _e2130 = ((abs(((_e2110 - trunc(_e2110)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2132 = select(_e2130, 0f, (_e2130 < 0f));
                    let _e2134 = select(_e2132, 1f, (_e2132 > 1f));
                    let _e2138 = ((_e2134 * _e2134) * (3f - (2f * _e2134)));
                    if (_e2128 != _e2128) {
                        phi_72_ = true;
                    } else {
                        phi_72_ = (_e2138 >= _e2128);
                    }
                    let _e2142 = phi_72_;
                    let _e2150 = pill_1.member[_e232].cpu.usage.samples[39u];
                    let _e2151 = (_e2150 * 0.24f);
                    let _e2152 = (0.18f + _e2151);
                    let _e2153 = (0.82f - _e2151);
                    let _e2162 = (_e999 - 60f);
                    let _e2163 = (_e2162 * 0.083333336f);
                    let _e2165 = select(_e2163, 0f, (_e2163 < 0f));
                    let _e2167 = select(_e2165, 1f, (_e2165 > 1f));
                    let _e2171 = ((_e2167 * _e2167) * (3f - (2f * _e2167)));
                    let _e2172 = (1f - _e2171);
                    let _e2181 = ((_e999 - 72f) * 0.0625f);
                    let _e2183 = select(_e2181, 0f, (_e2181 < 0f));
                    let _e2185 = select(_e2183, 1f, (_e2183 > 1f));
                    let _e2189 = ((_e2185 * _e2185) * (3f - (2f * _e2185)));
                    let _e2190 = (1f - _e2189);
                    let _e2199 = (_e2162 * 0.03846154f);
                    let _e2201 = select(_e2199, 0f, (_e2199 < 0f));
                    let _e2203 = select(_e2201, 1f, (_e2201 > 1f));
                    let _e2208 = (((_e2203 * _e2203) * (3f - (2f * _e2203))) * 0.9f);
                    let _e2209 = (1f - _e2208);
                    let _e2216 = ((((0.025f * _e2153) + (0.32f * _e2152)) * _e2209) + (((((0.22f * _e2172) + _e2171) * _e2190) + _e2189) * _e2208));
                    let _e2217 = ((((0.09f * _e2153) + (0.68f * _e2152)) * _e2209) + (((((0.62f * _e2172) + (0.38f * _e2171)) * _e2190) + (0.08f * _e2189)) * _e2208));
                    let _e2218 = ((((0.15f * _e2153) + _e2152) * _e2209) + ((((_e2172 + (0.08f * _e2171)) * _e2190) + (0.035f * _e2189)) * _e2208));
                    let _e2220 = ((((_e1887 + ((_e1816 - _e1887) * _e1894)) - ((1.6f * _e1894) * (1f - _e1894))) - 0.55f) * -0.9090909f);
                    let _e2222 = select(_e2220, 0f, (_e2220 < 0f));
                    let _e2224 = select(_e2222, 1f, (_e2222 > 1f));
                    let _e2228 = ((_e2224 * _e2224) * (3f - (2f * _e2224)));
                    let _e2230 = (1f - (_e2228 * 0.82f));
                    let _e2242 = ((abs(_e1816) - 2.1f) * -0.909091f);
                    let _e2244 = select(_e2242, 0f, (_e2242 < 0f));
                    let _e2246 = select(_e2244, 1f, (_e2244 > 1f));
                    let _e2251 = (((_e2246 * _e2246) * (3f - (2f * _e2246))) * 0.92f);
                    let _e2252 = (1f - _e2251);
                    let _e2263 = ((_e1887 - 0.55f) * -0.9090909f);
                    let _e2265 = select(_e2263, 0f, (_e2263 < 0f));
                    let _e2267 = select(_e2265, 1f, (_e2265 > 1f));
                    let _e2272 = (((_e2267 * _e2267) * (3f - (2f * _e2267))) * 0.78f);
                    let _e2273 = (1f - _e2272);
                    let _e2284 = ((_e1911 * select(_e2128, _e2138, _e2142)) * 0.045f);
                    phi_73_ = _e406;
                    phi_74_ = vec3<f32>(((((((((_e1080 * _e2230) + (_e2228 * 0.00328f)) * _e2252) + (_e2216 * _e2251)) * _e2273) + (_e2216 * _e2272)) + _e2284) + (((0.32f * _e1911) * _e2027) + ((0.78f * _e1911) * _e2101))), ((((((((_e1081 * _e2230) + (_e2228 * 0.00984f)) * _e2252) + (_e2217 * _e2251)) * _e2273) + (_e2217 * _e2272)) + _e2284) + (((0.68f * _e1911) * _e2027) + ((0.3f * _e1911) * _e2101))), ((((((((_e1082 * _e2230) + (_e2228 * 0.02132f)) * _e2252) + (_e2218 * _e2251)) * _e2273) + (_e2218 * _e2272)) + _e2284) + (_e1911 * (_e2027 + _e2101))));
                    phi_75_ = false;
                    break;
                }
                case 1: {
                    let _e1417 = pill_1.member[_e232].history_scroll;
                    switch bitcast<i32>(_e746) {
                        case 0: {
                            phi_57_ = true;
                            phi_58_ = false;
                            phi_59_ = false;
                            break;
                        }
                        case 1: {
                            phi_57_ = true;
                            phi_58_ = false;
                            phi_59_ = false;
                            break;
                        }
                        case 2: {
                            phi_57_ = false;
                            phi_58_ = true;
                            phi_59_ = false;
                            break;
                        }
                        case 3: {
                            phi_57_ = false;
                            phi_58_ = true;
                            phi_59_ = false;
                            break;
                        }
                        case 4: {
                            phi_57_ = false;
                            phi_58_ = false;
                            phi_59_ = true;
                            break;
                        }
                        case 5: {
                            phi_57_ = false;
                            phi_58_ = false;
                            phi_59_ = true;
                            break;
                        }
                        default: {
                            phi_57_ = bool();
                            phi_58_ = bool();
                            phi_59_ = bool();
                            break;
                        }
                    }
                    let _e1420 = phi_57_;
                    let _e1422 = phi_58_;
                    let _e1424 = phi_59_;
                    let _e1425 = select(_e1422, false, _e1420);
                    let _e1431 = ((select(select(80f, 32f, _e1425), 24f, select(select(_e1424, false, _e1420), false, _e1425)) * 0.5f) - 4f);
                    let _e1432 = (_e259 - 8f);
                    let _e1435 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e771, _e772), (_e1431 - _e1432), _e1432);
                    let _e1437 = ((_e1435 - 0.55f) * -0.9090909f);
                    let _e1439 = select(_e1437, 0f, (_e1437 < 0f));
                    let _e1441 = select(_e1439, 1f, (_e1439 > 1f));
                    let _e1445 = ((_e1441 * _e1441) * (3f - (2f * _e1441)));
                    let _e1446 = (_e1431 * 0.051282052f);
                    let _e1447 = (_e771 + _e1431);
                    let _e1449 = ((_e1447 / _e1446) + _e1417);
                    let _e1451 = select(_e1449, 0f, (_e1449 < 0f));
                    let _e1453 = select(_e1451, 39f, (_e1451 > 39f));
                    let _e1454 = floor(_e1453);
                    let _e1459 = select(select(u32(_e1454), 0u, (_e1454 < 0f)), 4294967295u, (_e1454 > 4294967000f));
                    let _e1460 = (_e259 - 10f);
                    let _e1464 = (((f32(_e1459) - _e1417) * _e1446) - _e1431);
                    let _e1466 = select(_e1459, 39u, (39u < _e1459));
                    let _e1467 = (_e1466 < 40u);
                    if _e1467 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1474 = pill_1.member[_e232].gpu.usage.samples[_e1466];
                    let _e1477 = (_e1460 * (1f - (_e1474 * 2f)));
                    let _e1478 = (_e1459 + 1u);
                    let _e1484 = select(_e1478, 39u, (39u < _e1478));
                    let _e1485 = (_e1484 < 40u);
                    if _e1485 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1492 = pill_1.member[_e232].gpu.usage.samples[_e1484];
                    let _e1496 = ((((f32(_e1478) - _e1417) * _e1446) - _e1431) - _e1464);
                    let _e1497 = ((_e1460 * (1f - (_e1492 * 2f))) - _e1477);
                    let _e1498 = (_e771 - _e1464);
                    let _e1499 = (_e772 - _e1477);
                    let _e1500 = (_e1498 * _e1496);
                    let _e1503 = (_e1496 * _e1496);
                    let _e1505 = (_e1503 + (_e1497 * _e1497));
                    if (_e1505 != _e1505) {
                        phi_60_ = true;
                    } else {
                        phi_60_ = (0.001f >= _e1505);
                    }
                    let _e1509 = phi_60_;
                    let _e1511 = ((_e1500 + (_e1499 * _e1497)) / select(_e1505, 0.001f, _e1509));
                    let _e1513 = select(_e1511, 0f, (_e1511 < 0f));
                    let _e1515 = select(_e1513, 1f, (_e1513 > 1f));
                    let _e1518 = (_e1498 - (_e1496 * _e1515));
                    let _e1519 = (_e1499 - (_e1497 * _e1515));
                    let _e1526 = ((abs(sqrt(((_e1518 * _e1518) + (_e1519 * _e1519)))) - 1.4000001f) * -0.9090908f);
                    let _e1528 = select(_e1526, 0f, (_e1526 < 0f));
                    let _e1530 = select(_e1528, 1f, (_e1528 > 1f));
                    let _e1536 = (_e1453 - trunc(_e1453));
                    let _e1538 = select(_e1536, 0f, (_e1536 < 0f));
                    let _e1540 = select(_e1538, 1f, (_e1538 > 1f));
                    let _e1544 = ((_e1540 * _e1540) * (3f - (2f * _e1540)));
                    let _e1551 = ((((_e1477 + (_e1497 * _e1544)) - _e772) - 0.55f) * -0.9090909f);
                    let _e1553 = select(_e1551, 0f, (_e1551 < 0f));
                    let _e1555 = select(_e1553, 1f, (_e1553 > 1f));
                    let _e1561 = ((((_e1555 * _e1555) * (3f - (2f * _e1555))) * 0.156f) + ((_e1530 * _e1530) * (3f - (2f * _e1530))));
                    if _e1467 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1570 = pill_1.member[_e232].gpu.memory.samples[_e1466];
                    let _e1573 = (_e1460 * (1f - (_e1570 * 2f)));
                    if _e1485 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1580 = pill_1.member[_e232].gpu.memory.samples[_e1484];
                    let _e1584 = ((_e1460 * (1f - (_e1580 * 2f))) - _e1573);
                    let _e1585 = (_e772 - _e1573);
                    let _e1589 = (_e1503 + (_e1584 * _e1584));
                    if (_e1589 != _e1589) {
                        phi_61_ = true;
                    } else {
                        phi_61_ = (0.001f >= _e1589);
                    }
                    let _e1593 = phi_61_;
                    let _e1595 = ((_e1500 + (_e1585 * _e1584)) / select(_e1589, 0.001f, _e1593));
                    let _e1597 = select(_e1595, 0f, (_e1595 < 0f));
                    let _e1599 = select(_e1597, 1f, (_e1597 > 1f));
                    let _e1602 = (_e1498 - (_e1496 * _e1599));
                    let _e1603 = (_e1585 - (_e1584 * _e1599));
                    let _e1610 = ((abs(sqrt(((_e1602 * _e1602) + (_e1603 * _e1603)))) - 1.4000001f) * -0.9090908f);
                    let _e1612 = select(_e1610, 0f, (_e1610 < 0f));
                    let _e1614 = select(_e1612, 1f, (_e1612 > 1f));
                    let _e1625 = ((((_e1573 + (_e1584 * _e1544)) - _e772) - 0.55f) * -0.9090909f);
                    let _e1627 = select(_e1625, 0f, (_e1625 < 0f));
                    let _e1629 = select(_e1627, 1f, (_e1627 > 1f));
                    let _e1635 = ((((_e1629 * _e1629) * (3f - (2f * _e1629))) * 0.084f) + ((_e1614 * _e1614) * (3f - (2f * _e1614))));
                    let _e1643 = (_e1447 * 0.14285715f);
                    let _e1644 = ((_e772 + _e1432) * 0.16393442f);
                    let _e1654 = ((abs(((_e1643 - trunc(_e1643)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e1656 = select(_e1654, 0f, (_e1654 < 0f));
                    let _e1658 = select(_e1656, 1f, (_e1656 > 1f));
                    let _e1662 = ((_e1658 * _e1658) * (3f - (2f * _e1658)));
                    let _e1664 = ((abs(((_e1644 - trunc(_e1644)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e1666 = select(_e1664, 0f, (_e1664 < 0f));
                    let _e1668 = select(_e1666, 1f, (_e1666 > 1f));
                    let _e1672 = ((_e1668 * _e1668) * (3f - (2f * _e1668)));
                    if (_e1662 != _e1662) {
                        phi_62_ = true;
                    } else {
                        phi_62_ = (_e1672 >= _e1662);
                    }
                    let _e1676 = phi_62_;
                    let _e1684 = pill_1.member[_e232].gpu.usage.samples[39u];
                    let _e1685 = (_e1684 * 0.24f);
                    let _e1686 = (0.18f + _e1685);
                    let _e1687 = (0.82f - _e1685);
                    let _e1696 = (_e1004 - 60f);
                    let _e1697 = (_e1696 * 0.083333336f);
                    let _e1699 = select(_e1697, 0f, (_e1697 < 0f));
                    let _e1701 = select(_e1699, 1f, (_e1699 > 1f));
                    let _e1705 = ((_e1701 * _e1701) * (3f - (2f * _e1701)));
                    let _e1706 = (1f - _e1705);
                    let _e1715 = ((_e1004 - 72f) * 0.0625f);
                    let _e1717 = select(_e1715, 0f, (_e1715 < 0f));
                    let _e1719 = select(_e1717, 1f, (_e1717 > 1f));
                    let _e1723 = ((_e1719 * _e1719) * (3f - (2f * _e1719)));
                    let _e1724 = (1f - _e1723);
                    let _e1733 = (_e1696 * 0.03846154f);
                    let _e1735 = select(_e1733, 0f, (_e1733 < 0f));
                    let _e1737 = select(_e1735, 1f, (_e1735 > 1f));
                    let _e1742 = (((_e1737 * _e1737) * (3f - (2f * _e1737))) * 0.9f);
                    let _e1743 = (1f - _e1742);
                    let _e1754 = (1f - (_e1445 * 0.82f));
                    let _e1766 = ((abs(_e1435) - 2.1f) * -0.909091f);
                    let _e1768 = select(_e1766, 0f, (_e1766 < 0f));
                    let _e1770 = select(_e1768, 1f, (_e1768 > 1f));
                    let _e1775 = (((_e1770 * _e1770) * (3f - (2f * _e1770))) * 0.92f);
                    let _e1776 = (1f - _e1775);
                    let _e1787 = ((_e1445 * select(_e1662, _e1672, _e1676)) * 0.045f);
                    phi_73_ = _e406;
                    phi_74_ = vec3<f32>(((((((_e1080 * _e1754) + (_e1445 * 0.00328f)) * _e1776) + (((((0.025f * _e1687) + (0.32f * _e1686)) * _e1743) + (((((0.22f * _e1706) + _e1705) * _e1724) + _e1723) * _e1742)) * _e1775)) + _e1787) + (((0.32f * _e1445) * _e1561) + ((0.78f * _e1445) * _e1635))), ((((((_e1081 * _e1754) + (_e1445 * 0.00984f)) * _e1776) + (((((0.09f * _e1687) + (0.68f * _e1686)) * _e1743) + (((((0.62f * _e1706) + (0.38f * _e1705)) * _e1724) + (0.08f * _e1723)) * _e1742)) * _e1775)) + _e1787) + (((0.68f * _e1445) * _e1561) + ((0.3f * _e1445) * _e1635))), ((((((_e1082 * _e1754) + (_e1445 * 0.02132f)) * _e1776) + (((((0.15f * _e1687) + _e1686) * _e1743) + ((((_e1706 + (0.08f * _e1705)) * _e1724) + (0.035f * _e1723)) * _e1742)) * _e1775)) + _e1787) + (_e1445 * (_e1561 + _e1635))));
                    phi_75_ = false;
                    break;
                }
                case 2: {
                    let _e1209 = (_e771 * 1.25f);
                    let _e1210 = (_e772 * 1.25f);
                    let _e1212 = select(0f, 1f, (_e238 < 0f));
                    let _e1213 = abs(_e238);
                    let _e1214 = (_e1210 - 1f);
                    let _e1215 = vec2<f32>(_e1209, _e1214);
                    let _e1216 = cantus_render_shader_sd_rounded_box(_e1215, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e1219 = ((abs(_e1216) - 2.425f) * -0.909091f);
                    let _e1221 = select(_e1219, 0f, (_e1219 < 0f));
                    let _e1223 = select(_e1221, 1f, (_e1221 > 1f));
                    let _e1230 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1209, (_e1210 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e1232 = ((_e1230 - 0.55f) * -0.9090909f);
                    let _e1234 = select(_e1232, 0f, (_e1232 < 0f));
                    let _e1236 = select(_e1234, 1f, (_e1234 > 1f));
                    let _e1241 = cantus_render_shader_sd_rounded_box(_e1215, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e1243 = ((_e1241 - 0.55f) * -0.9090909f);
                    let _e1245 = select(_e1243, 0f, (_e1243 < 0f));
                    let _e1247 = select(_e1245, 1f, (_e1245 > 1f));
                    let _e1251 = ((_e1247 * _e1247) * (3f - (2f * _e1247)));
                    let _e1253 = select(_e1213, 0f, (_e1213 < 0f));
                    let _e1271 = ((12f - (select(_e1253, 1f, (_e1253 > 1f)) * 24f)) + ((sin(((_e771 * 0.775f) + (_e545 * (1.4f + (_e1212 * 1.2f))))) * 1.15f) + (sin(((_e771 * 0.3375f) - (_e545 * 0.8f))) * 0.45f)));
                    let _e1272 = (_e1271 - 0.7f);
                    let _e1276 = ((_e1214 - _e1272) / ((_e1271 + 0.7f) - _e1272));
                    let _e1278 = select(_e1276, 0f, (_e1276 < 0f));
                    let _e1280 = select(_e1278, 1f, (_e1278 > 1f));
                    let _e1285 = (_e1251 * ((_e1280 * _e1280) * (3f - (2f * _e1280))));
                    let _e1287 = ((_e1213 - 0.08f) * 5f);
                    let _e1289 = select(_e1287, 0f, (_e1287 < 0f));
                    let _e1291 = select(_e1289, 1f, (_e1289 > 1f));
                    let _e1295 = ((_e1291 * _e1291) * (3f - (2f * _e1291)));
                    let _e1296 = (1f - _e1295);
                    let _e1304 = ((_e1213 - 0.18f) * 1.8518518f);
                    let _e1306 = select(_e1304, 0f, (_e1304 < 0f));
                    let _e1308 = select(_e1306, 1f, (_e1306 > 1f));
                    let _e1312 = ((_e1308 * _e1308) * (3f - (2f * _e1308)));
                    let _e1313 = (1f - _e1312);
                    let _e1319 = (_e1313 + (0.22f * _e1312));
                    let _e1320 = ((((0.18f * _e1296) + (0.72f * _e1295)) * _e1313) + (0.95f * _e1312));
                    let _e1321 = ((((0.1f * _e1296) + (0.12f * _e1295)) * _e1313) + (0.55f * _e1312));
                    let _e1323 = floor((_e771 * 0.4166667f));
                    let _e1325 = cantus_render_shader_hash(vec2<f32>(_e1323, 0f));
                    let _e1328 = (_e1325.y * 0.5f);
                    let _e1332 = ((_e545 * (0.35f + _e1328)) + (_e1325.x * 7f));
                    let _e1334 = (_e1332 - trunc(_e1332));
                    let _e1341 = (_e1209 - (((_e1323 + 0.2f) + (_e1325.x * 0.6f)) * 3f));
                    let _e1342 = (_e1210 - (13f - (_e1334 * 24f)));
                    let _e1349 = (_e1334 * 4f);
                    let _e1351 = select(_e1349, 0f, (_e1349 < 0f));
                    let _e1353 = select(_e1351, 1f, (_e1351 > 1f));
                    let _e1359 = ((_e1334 - 1f) * -3.3333333f);
                    let _e1361 = select(_e1359, 0f, (_e1359 < 0f));
                    let _e1363 = select(_e1361, 1f, (_e1361 > 1f));
                    let _e1371 = ((abs((sqrt(((_e1341 * _e1341) + (_e1342 * _e1342))) - (0.4f + _e1328))) - 1f) * -0.9090909f);
                    let _e1373 = select(_e1371, 0f, (_e1371 < 0f));
                    let _e1375 = select(_e1373, 1f, (_e1373 > 1f));
                    let _e1382 = (((((_e1375 * _e1375) * (3f - (2f * _e1375))) * (((_e1353 * _e1353) * (3f - (2f * _e1353))) * ((_e1363 * _e1363) * (3f - (2f * _e1363))))) * _e1251) * _e1212);
                    let _e1385 = ((((_e1223 * _e1223) * (3f - (2f * _e1223))) * 0.43f) + (((_e1236 * _e1236) * (3f - (2f * _e1236))) * 0.38f));
                    phi_73_ = _e406;
                    phi_74_ = vec3<f32>((_e1080 + ((_e1385 + ((_e1319 * _e1285) * 0.78f)) + ((((_e1319 * 0.27999997f) + 0.72f) * _e1382) * 0.9f))), (_e1081 + ((_e1385 + ((_e1320 * _e1285) * 0.78f)) + ((((_e1320 * 0.27999997f) + 0.72f) * _e1382) * 0.9f))), (_e1082 + ((_e1385 + ((_e1321 * _e1285) * 0.78f)) + ((((_e1321 * 0.27999997f) + 0.72f) * _e1382) * 0.9f))));
                    phi_75_ = false;
                    break;
                }
                case 3: {
                    let _e1087 = pill_1.member[_e232].volume;
                    let _e1089 = select(0f, 1f, (_e1087 < 0f));
                    let _e1090 = abs(_e1087);
                    let _e1093 = round(((_e771 + 12f) * 0.25f));
                    let _e1095 = select(_e1093, 0f, (_e1093 < 0f));
                    let _e1097 = select(_e1095, 6f, (_e1095 > 6f));
                    let _e1102 = select(select(u32(_e1097), 0u, (_e1097 < 0f)), 4294967295u, (_e1097 > 4294967000f));
                    if (_e1102 < 7u) {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1108 = pill_1.member[_e232].audio_spectrum[_e1102];
                    let _e1109 = (1f - _e1089);
                    let _e1110 = (_e1108 * _e1109);
                    let _e1119 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e771 - (-12f + (_e1097 * 4f))), (_e772 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1110))), 1.25f);
                    let _e1122 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e771, (_e772 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1124 = ((_e1122 - 0.55f) * -0.9090909f);
                    let _e1126 = select(_e1124, 0f, (_e1124 < 0f));
                    let _e1128 = select(_e1126, 1f, (_e1126 > 1f));
                    let _e1132 = ((_e1128 * _e1128) * (3f - (2f * _e1128)));
                    let _e1134 = select(_e1090, 0f, (_e1090 < 0f));
                    let _e1137 = (select(_e1134, 1f, (_e1134 > 1f)) * 28f);
                    let _e1138 = (_e1137 + -13.2f);
                    let _e1142 = ((_e771 - _e1138) / ((_e1137 + -14.8f) - _e1138));
                    let _e1144 = select(_e1142, 0f, (_e1142 < 0f));
                    let _e1146 = select(_e1144, 1f, (_e1144 > 1f));
                    let _e1151 = (_e1132 * ((_e1146 * _e1146) * (3f - (2f * _e1146))));
                    let _e1153 = (1f - (_e1090 * 0.65f));
                    let _e1158 = ((0.08f * _e1153) + (_e1090 * 0.42249995f));
                    let _e1159 = ((0.88f * _e1153) + (_e1090 * 0.221f));
                    let _e1161 = ((_e1119 - 0.7f) * -0.71428573f);
                    let _e1163 = select(_e1161, 0f, (_e1161 < 0f));
                    let _e1165 = select(_e1163, 1f, (_e1163 > 1f));
                    let _e1174 = ((_e1119 - 3.2f) * -0.3125f);
                    let _e1176 = select(_e1174, 0f, (_e1174 < 0f));
                    let _e1178 = select(_e1176, 1f, (_e1176 > 1f));
                    let _e1185 = ((((_e1165 * _e1165) * (3f - (2f * _e1165))) * (0.58f + (_e1110 * 0.35f))) + ((((_e1178 * _e1178) * (3f - (2f * _e1178))) * _e1110) * 0.12f));
                    let _e1198 = (_e1151 + ((_e1132 * (1f - _e1151)) * 0.22f));
                    phi_73_ = _e406;
                    phi_74_ = vec3<f32>((_e1080 + ((_e1158 * _e1185) + (((_e1158 * _e1109) + _e1089) * _e1198))), (_e1081 + ((_e1159 * _e1185) + (((_e1159 * _e1109) + (0.24f * _e1089)) * _e1198))), (_e1082 + (_e1185 + ((_e1109 + (0.3f * _e1089)) * _e1198))));
                    phi_75_ = false;
                    break;
                }
                case 4: {
                    phi_73_ = _e406;
                    phi_74_ = vec3<f32>();
                    phi_75_ = true;
                    break;
                }
                case 5: {
                    phi_73_ = _e406;
                    phi_74_ = vec3<f32>();
                    phi_75_ = true;
                    break;
                }
                default: {
                    phi_73_ = _e406;
                    phi_74_ = vec3<f32>();
                    phi_75_ = bool();
                    break;
                }
            }
            let _e2293 = phi_73_;
            let _e2295 = phi_74_;
            let _e2297 = phi_75_;
            if _e2293 {
                break;
            }
            if _e2297 {
                let _e2299 = select(1f, 0f, (_e746 == 5u));
                let _e2303 = pill_1.member[_e232].power_hover;
                let _e2308 = ((abs((f32(_e2303) - _e2299)) - 0.4f) * -2.857143f);
                let _e2310 = select(_e2308, 0f, (_e2308 < 0f));
                let _e2312 = select(_e2310, 1f, (_e2310 > 1f));
                let _e2316 = ((_e2312 * _e2312) * (3f - (2f * _e2312)));
                let _e2318 = (1f + (_e2316 * 0.07f));
                let _e2319 = (_e771 / _e2318);
                let _e2320 = (_e772 / _e2318);
                let _e2324 = pill_1.member[_e232].power_action;
                let _e2329 = ((abs((f32(_e2324) - _e2299)) - 0.4f) * -2.857143f);
                let _e2331 = select(_e2329, 0f, (_e2329 < 0f));
                let _e2333 = select(_e2331, 1f, (_e2331 > 1f));
                let _e2337 = ((_e2333 * _e2333) * (3f - (2f * _e2333)));
                let _e2341 = pill_1.member[_e232].power_progress;
                let _e2342 = (_e2341 * _e2337);
                if (_e2299 < 0.5f) {
                    let _e2466 = select(_e2342, 0f, (_e2342 < 0f));
                    let _e2468 = select(_e2466, 1f, (_e2466 > 1f));
                    let _e2472 = ((_e2468 * _e2468) * (3f - (2f * _e2468)));
                    let _e2478 = (1f - _e2342);
                    let _e2487 = (_e2472 * 0.7f);
                    let _e2488 = (_e2487 + 1.5999999f);
                    let _e2493 = ((abs((sqrt(((_e2319 * _e2319) + (_e2320 * _e2320))) - ((7.5f - (_e2342 * 4.6f)) + (((sin((_e545 * 8f)) * _e2342) * _e2478) * 0.16f)))) - _e2488) / ((_e2487 + 0.49999994f) - _e2488));
                    let _e2495 = select(_e2493, 0f, (_e2493 < 0f));
                    let _e2497 = select(_e2495, 1f, (_e2495 > 1f));
                    let _e2506 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2319, (_e2320 - -7f)), vec2<f32>((3f * _e2478), 3f), 0.5f);
                    let _e2508 = ((_e2506 - 0.55f) * -0.9090909f);
                    let _e2510 = select(_e2508, 0f, (_e2508 < 0f));
                    let _e2512 = select(_e2510, 1f, (_e2510 > 1f));
                    let _e2526 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2319, (_e2320 - (-5f + (_e2342 * 3.5f)))), vec2<f32>((1.05f + (_e2472 * 0.45f)), (4.6f - (_e2342 * 3f))), 0.7f);
                    let _e2528 = ((_e2526 - 0.55f) * -0.9090909f);
                    let _e2530 = select(_e2528, 0f, (_e2528 < 0f));
                    let _e2532 = select(_e2530, 1f, (_e2530 > 1f));
                    let _e2536 = ((_e2532 * _e2532) * (3f - (2f * _e2532)));
                    let _e2538 = (((_e2497 * _e2497) * (3f - (2f * _e2497))) * (1f - ((_e2512 * _e2512) * (3f - (2f * _e2512)))));
                    if (_e2538 != _e2538) {
                        phi_79_ = true;
                    } else {
                        phi_79_ = (_e2536 >= _e2538);
                    }
                    let _e2542 = phi_79_;
                    phi_80_ = select(_e2538, _e2536, _e2542);
                } else {
                    let _e2345 = ((1f - _e2337) + _e2342);
                    let _e2349 = (((atan2(_e2320, _e2319) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e2353 = ((_e2345 * 0.82f) - 0.045f);
                    if (_e2353 != _e2353) {
                        phi_76_ = true;
                    } else {
                        phi_76_ = (0f >= _e2353);
                    }
                    let _e2357 = phi_76_;
                    let _e2358 = select(_e2353, 0f, _e2357);
                    let _e2366 = ((abs((sqrt(((_e2319 * _e2319) + (_e2320 * _e2320))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e2368 = select(_e2366, 0f, (_e2366 < 0f));
                    let _e2370 = select(_e2368, 1f, (_e2368 > 1f));
                    let _e2375 = (_e2358 + 0.008f);
                    let _e2379 = (((_e2349 - trunc(_e2349)) - _e2375) / ((_e2358 - 0.008f) - _e2375));
                    let _e2381 = select(_e2379, 0f, (_e2379 < 0f));
                    let _e2383 = select(_e2381, 1f, (_e2381 > 1f));
                    let _e2389 = (_e2345 * 50f);
                    let _e2391 = select(_e2389, 0f, (_e2389 < 0f));
                    let _e2393 = select(_e2391, 1f, (_e2391 > 1f));
                    let _e2398 = ((((_e2370 * _e2370) * (3f - (2f * _e2370))) * ((_e2383 * _e2383) * (3f - (2f * _e2383)))) * ((_e2393 * _e2393) * (3f - (2f * _e2393))));
                    let _e2400 = (0.50265485f + (5.152212f * _e2345));
                    let _e2401 = cos(_e2400);
                    let _e2402 = sin(_e2400);
                    let _e2406 = (_e2319 - (_e2401 * 7.1f));
                    let _e2407 = (_e2320 - (_e2402 * 7.1f));
                    let _e2410 = ((_e2406 * -(_e2402)) + (_e2407 * _e2401));
                    let _e2413 = ((_e2406 * _e2401) + (_e2407 * _e2402));
                    let _e2414 = (_e2410 * -3.2f);
                    let _e2417 = ((_e2414 + (_e2413 * 2.1f)) * 0.06825939f);
                    let _e2419 = select(_e2417, 0f, (_e2417 < 0f));
                    let _e2421 = select(_e2419, 1f, (_e2419 > 1f));
                    let _e2424 = (_e2410 - (-3.2f * _e2421));
                    let _e2425 = (_e2413 - (2.1f * _e2421));
                    let _e2429 = sqrt(((_e2424 * _e2424) + (_e2425 * _e2425)));
                    let _e2432 = ((_e2414 + (_e2413 * -2.1f)) * 0.06825939f);
                    let _e2434 = select(_e2432, 0f, (_e2432 < 0f));
                    let _e2436 = select(_e2434, 1f, (_e2434 > 1f));
                    let _e2439 = (_e2410 - (-3.2f * _e2436));
                    let _e2440 = (_e2413 - (-2.1f * _e2436));
                    let _e2444 = sqrt(((_e2439 * _e2439) + (_e2440 * _e2440)));
                    if (_e2429 != _e2429) {
                        phi_77_ = true;
                    } else {
                        phi_77_ = (_e2444 <= _e2429);
                    }
                    let _e2448 = phi_77_;
                    let _e2451 = ((select(_e2429, _e2444, _e2448) - 1.7f) * -0.71428573f);
                    let _e2453 = select(_e2451, 0f, (_e2451 < 0f));
                    let _e2455 = select(_e2453, 1f, (_e2453 > 1f));
                    let _e2459 = ((_e2455 * _e2455) * (3f - (2f * _e2455)));
                    if (_e2398 != _e2398) {
                        phi_78_ = true;
                    } else {
                        phi_78_ = (_e2459 >= _e2398);
                    }
                    let _e2463 = phi_78_;
                    phi_80_ = select(_e2398, _e2459, _e2463);
                }
                let _e2545 = phi_80_;
                let _e2548 = (_e2337 * (0.5f + (_e2342 * 0.5f)));
                if (_e2316 != _e2316) {
                    phi_81_ = true;
                } else {
                    phi_81_ = (_e2548 >= _e2316);
                }
                let _e2552 = phi_81_;
                let _e2553 = select(_e2316, _e2548, _e2552);
                let _e2555 = (0.48f * (1f - _e2553));
                let _e2566 = (1f + (_e2342 * 0.45f));
                phi_82_ = vec3<f32>((_e1080 + (((_e2555 + (0.78f * _e2553)) * _e2545) * _e2566)), (_e1081 + (((_e2555 + (0.3f * _e2553)) * _e2545) * _e2566)), (_e1082 + (((_e2555 + (0.28f * _e2553)) * _e2545) * _e2566)));
            } else {
                phi_82_ = _e2295;
            }
            let _e2575 = phi_82_;
            let _e2577 = local_34;
            let _e2579 = (1f - (_e2577 * 0.35f));
            let _e2587 = local_35;
            let _e2588 = (_e2587 * 0.33249998f);
            switch bitcast<i32>(_e746) {
                case 0: {
                    let _e2602 = pill_1.member[_e232].labels[0u];
                    phi_83_ = _e2602;
                    break;
                }
                case 1: {
                    let _e2597 = pill_1.member[_e232].labels[1u];
                    phi_83_ = _e2597;
                    break;
                }
                default: {
                    phi_83_ = render_text_Line(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), 0f, 0f, 0u, 0u, 0u);
                    break;
                }
            }
            let _e2604 = phi_83_;
            switch bitcast<i32>(_e746) {
                case 0: {
                    phi_84_ = true;
                    break;
                }
                case 1: {
                    phi_84_ = true;
                    break;
                }
                default: {
                    phi_84_ = false;
                    break;
                }
            }
            let _e2607 = phi_84_;
            if _e2607 {
                if (_e715 < _e2604.min.x) {
                    phi_113_ = f32();
                    phi_114_ = true;
                } else {
                    if (_e715 > _e2604.max.x) {
                        phi_111_ = f32();
                        phi_112_ = true;
                    } else {
                        if (_e716 < _e2604.min.y) {
                            phi_109_ = f32();
                            phi_110_ = true;
                        } else {
                            let _e2619 = (_e716 > _e2604.max.y);
                            if _e2619 {
                                phi_108_ = f32();
                            } else {
                                let _e2621 = (1f / _e2604.size);
                                let _e2628 = ((_e715 - _e2604.origin.x) * _e2621);
                                phi_85_ = 0u;
                                phi_86_ = _e2604.count;
                                loop {
                                    let _e2633 = phi_85_;
                                    let _e2635 = phi_86_;
                                    local_36 = _e2633;
                                    let _e2636 = (_e2633 < _e2635);
                                    if _e2636 {
                                        let _e2639 = (_e2633 + ((_e2635 - _e2633) / 2u));
                                        let _e2644 = placed_glyphs_1.member[(_e2604.first + _e2639)].x;
                                        let _e2645 = (_e2644 <= _e2628);
                                        if _e2645 {
                                            phi_87_ = (_e2639 + 1u);
                                        } else {
                                            phi_87_ = _e2633;
                                        }
                                        let _e2648 = phi_87_;
                                        phi_88_ = _e2648;
                                        phi_89_ = select(_e2639, _e2635, _e2645);
                                    } else {
                                        phi_88_ = u32();
                                        phi_89_ = u32();
                                    }
                                    let _e2651 = phi_88_;
                                    let _e2653 = phi_89_;
                                    continue;
                                    continuing {
                                        phi_85_ = _e2651;
                                        phi_86_ = _e2653;
                                        break if !(_e2636);
                                    }
                                }
                                let _e2655 = (3.5f / _e2604.size);
                                let _e2657 = local_36;
                                let _e2658 = (_e2657 + 1u);
                                phi_90_ = select(_e2658, _e2604.count, (_e2604.count < _e2658));
                                phi_91_ = -1000000f;
                                loop {
                                    let _e2662 = phi_90_;
                                    let _e2664 = phi_91_;
                                    local_39 = _e2664;
                                    if (_e2662 > 0u) {
                                        let _e2666 = (_e2662 - 1u);
                                        let _e2667 = (_e2604.first + _e2666);
                                        let _e2671 = placed_glyphs_1.member[_e2667].x;
                                        let _e2675 = placed_glyphs_1.member[_e2667].glyph;
                                        let _e2680 = glyphs_1.member[_e2675].min[0u];
                                        let _e2685 = glyphs_1.member[_e2675].min[1u];
                                        let _e2690 = glyphs_1.member[_e2675].max[0u];
                                        let _e2695 = glyphs_1.member[_e2675].max[1u];
                                        let _e2699 = glyphs_1.member[_e2675].start;
                                        let _e2703 = glyphs_1.member[_e2675].count;
                                        let _e2704 = (_e2628 - _e2671);
                                        let _e2705 = -(((_e716 - _e2604.origin.y) * _e2621));
                                        let _e2706 = (_e2690 + _e2655);
                                        let _e2707 = (_e2704 > _e2706);
                                        if _e2707 {
                                            phi_104_ = f32();
                                        } else {
                                            if (_e2704 >= (_e2680 - _e2655)) {
                                                if (_e2705 >= (_e2685 - _e2655)) {
                                                    if (_e2704 <= _e2706) {
                                                        if (_e2705 <= (_e2695 + _e2655)) {
                                                            phi_92_ = 340282350000000000000000000000000000000f;
                                                            phi_93_ = 0u;
                                                            phi_94_ = 0i;
                                                            loop {
                                                                let _e2717 = phi_92_;
                                                                let _e2719 = phi_93_;
                                                                let _e2721 = phi_94_;
                                                                local_37 = _e2717;
                                                                local_38 = _e2721;
                                                                let _e2722 = (_e2719 < _e2703);
                                                                if _e2722 {
                                                                    let _e2726 = edges_1.member[(_e2699 + _e2719)];
                                                                    let _e2728 = cantus_render_text_edge_distance(_e2726, _e2604.weight, vec2<f32>(_e2704, _e2705), _e2717);
                                                                    phi_95_ = _e2728.member;
                                                                    phi_96_ = (_e2719 + 1u);
                                                                    phi_97_ = (_e2721 + _e2728.member_1);
                                                                } else {
                                                                    phi_95_ = f32();
                                                                    phi_96_ = u32();
                                                                    phi_97_ = i32();
                                                                }
                                                                let _e2734 = phi_95_;
                                                                let _e2736 = phi_96_;
                                                                let _e2738 = phi_97_;
                                                                continue;
                                                                continuing {
                                                                    phi_92_ = _e2734;
                                                                    phi_93_ = _e2736;
                                                                    phi_94_ = _e2738;
                                                                    break if !(_e2722);
                                                                }
                                                            }
                                                            let _e2741 = local_37;
                                                            let _e2743 = ((_e2741 * _e2604.size) * _e2604.size);
                                                            if (_e2743 >= 12.25f) {
                                                                phi_98_ = 3.5f;
                                                            } else {
                                                                phi_98_ = sqrt(_e2743);
                                                            }
                                                            let _e2747 = phi_98_;
                                                            let _e2749 = local_38;
                                                            let _e2752 = (_e2747 * select(1f, -1f, (_e2749 == 0i)));
                                                            if (_e2664 != _e2664) {
                                                                phi_99_ = true;
                                                            } else {
                                                                phi_99_ = (_e2752 >= _e2664);
                                                            }
                                                            let _e2756 = phi_99_;
                                                            phi_100_ = select(_e2664, _e2752, _e2756);
                                                        } else {
                                                            phi_100_ = _e2664;
                                                        }
                                                        let _e2759 = phi_100_;
                                                        phi_101_ = _e2759;
                                                    } else {
                                                        phi_101_ = _e2664;
                                                    }
                                                    let _e2761 = phi_101_;
                                                    phi_102_ = _e2761;
                                                } else {
                                                    phi_102_ = _e2664;
                                                }
                                                let _e2763 = phi_102_;
                                                phi_103_ = _e2763;
                                            } else {
                                                phi_103_ = _e2664;
                                            }
                                            let _e2765 = phi_103_;
                                            phi_104_ = _e2765;
                                        }
                                        let _e2767 = phi_104_;
                                        phi_105_ = _e2666;
                                        phi_106_ = _e2767;
                                        phi_107_ = select(true, false, _e2707);
                                    } else {
                                        phi_105_ = u32();
                                        phi_106_ = f32();
                                        phi_107_ = false;
                                    }
                                    let _e2770 = phi_105_;
                                    let _e2772 = phi_106_;
                                    let _e2774 = phi_107_;
                                    continue;
                                    continuing {
                                        phi_90_ = _e2770;
                                        phi_91_ = _e2772;
                                        break if !(_e2774);
                                    }
                                }
                                let _e2936 = local_39;
                                phi_108_ = _e2936;
                            }
                            let _e2777 = phi_108_;
                            phi_109_ = _e2777;
                            phi_110_ = _e2619;
                        }
                        let _e2779 = phi_109_;
                        let _e2781 = phi_110_;
                        phi_111_ = _e2779;
                        phi_112_ = _e2781;
                    }
                    let _e2783 = phi_111_;
                    let _e2785 = phi_112_;
                    phi_113_ = _e2783;
                    phi_114_ = _e2785;
                }
                let _e2787 = phi_113_;
                let _e2789 = phi_114_;
                let _e2792 = ((select(_e2787, -1000000f, _e2789) * 1.25f) + 0.5f);
                let _e2794 = select(_e2792, 0f, (_e2792 < 0f));
                let _e2796 = select(_e2794, 1f, (_e2794 > 1f));
                phi_115_ = ((_e2796 * _e2796) * (3f - (2f * _e2796)));
            } else {
                phi_115_ = 0f;
            }
            let _e2802 = phi_115_;
            let _e2803 = (1f - _e2802);
            let _e2807 = (0.94f * _e2802);
            out_color = vec4<f32>((((((_e2575.x * _e2579) + _e2588) * _e2803) + _e2807) * _e495), (((((_e2575.y * _e2579) + _e2588) * _e2803) + _e2807) * _e495), (((((_e2575.z * _e2579) + _e2588) * _e2803) + _e2807) * _e495), _e508);
            break;
        }
    }
    return;
}

fn render_playhead_isthmus_playheadpass_vertex_impl() {
    let _e13 = vertex_6;
    let _e14 = _isthmus_instance_index_9;
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

fn render_playhead_isthmus_playheadpass_fragment_impl() {
    var phi_0_: bool;
    var phi_1_: bool;
    var phi_2_: bool;
    var phi_3_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e31 = world_pos_1;
            let _e32 = _isthmus_instance_index_10;
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

fn render_particles_isthmus_particlepass_vertex_impl() {
    var phi_0_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_1_: isthmus_Vertex_render_particles_Varyings;
    var phi_2_: isthmus_Vertex_render_particles_Varyings;
    var phi_3_: bool;
    var phi_4_: isthmus_Vertex_render_particles_Varyings;

    let _e30 = vertex_6;
    let _e31 = _isthmus_instance_index_9;
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
            phi_1_ = isthmus_Vertex_render_particles_Varyings(isthmus_Vertex_render_text_Varyings(vec4<f32>(((((_e125 + (_e116.x * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.y * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.z * 2f)) * 0.8f) + 0.2f) * 2f), (((1f - _e48) * ((_e159 * _e159) * (3f - (2f * _e159)))) * 0.3f)), vec2<f32>(_e82, _e83)), _e153);
        }
        let _e171 = phi_1_;
        phi_2_ = _e171;
        phi_3_ = _e47;
    }
    let _e173 = phi_2_;
    let _e175 = phi_3_;
    if _e175 {
        phi_4_ = isthmus_Vertex_render_particles_Varyings(isthmus_Vertex_render_text_Varyings(vec4<f32>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f)), vec4<f32>(0f, 0f, 0f, 0f));
    } else {
        phi_4_ = _e173;
    }
    let _e177 = phi_4_;
    out_position = _e177.position;
    out_color = _e177.varyings.position;
    out_uv[0u] = _e177.varyings.varyings.x;
    out_uv[1u] = _e177.varyings.varyings.y;
    return;
}

fn render_particles_isthmus_particlepass_fragment_impl() {
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

fn render_tempestas_isthmus_tempestaspass_vertex_impl() {
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
    var local_40: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e36 = vertex_6;
            let _e37 = _isthmus_instance_index_9;
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
                local_40 = _e143;
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
            let _e181 = local_40;
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

fn cantus_render_tempestas_cell_index(param_28: f32, param_29: f32, param_30: f32, param_31: f32) -> u32 {
    var phi_0_: bool;
    var phi_1_: bool;

    let _e11 = floor(((param_28 - param_29) / param_30));
    if (_e11 != _e11) {
        phi_0_ = true;
    } else {
        phi_0_ = (0f >= _e11);
    }
    let _e15 = phi_0_;
    let _e16 = select(_e11, 0f, _e15);
    if (_e16 != _e16) {
        phi_1_ = true;
    } else {
        phi_1_ = (param_31 <= _e16);
    }
    let _e20 = phi_1_;
    let _e21 = select(_e16, param_31, _e20);
    return select(select(u32(_e21), 0u, (_e21 < 0f)), 4294967295u, (_e21 > 4294967000f));
}

fn render_tempestas_isthmus_tempestaspass_fragment_impl() {
    var phi_0_: f32;
    var phi_1_: u32;
    var phi_2_: u32;
    var phi_3_: bool;
    var phi_4_: bool;
    var phi_5_: bool;
    var phi_6_: bool;
    var phi_7_: vec2<f32>;
    var phi_8_: f32;
    var phi_9_: u32;
    var phi_10_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_11_: bool;
    var phi_12_: vec2<f32>;
    var phi_13_: f32;
    var phi_14_: vec2<f32>;
    var phi_15_: f32;
    var phi_16_: vec2<f32>;
    var phi_17_: f32;
    var phi_18_: u32;
    var phi_19_: bool;
    var phi_20_: f32;
    var local_41: vec2<f32>;
    var local_42: vec2<f32>;
    var phi_21_: bool;
    var local_43: vec2<f32>;
    var phi_22_: f32;
    var local_44: vec2<f32>;
    var phi_23_: bool;
    var phi_24_: f32;
    var phi_25_: render_tempestas_WeatherCondition;
    var phi_26_: render_tempestas_WeatherCondition;
    var phi_27_: f32;
    var phi_28_: f32;
    var phi_29_: array<f32, 2>;
    var phi_30_: array<f32, 2>;
    var phi_31_: bool;
    var phi_32_: f32;
    var phi_33_: array<f32, 2>;
    var phi_34_: bool;
    var phi_35_: bool;
    var phi_36_: bool;
    var phi_37_: bool;
    var phi_38_: bool;
    var phi_39_: bool;
    var phi_40_: vec2<f32>;
    var phi_41_: bool;
    var phi_42_: bool;
    var phi_43_: i32;
    var phi_44_: f32;
    var phi_45_: f32;
    var phi_46_: vec2<f32>;
    var phi_47_: i32;
    var phi_48_: f32;
    var phi_49_: f32;
    var phi_50_: vec2<f32>;
    var local_45: f32;
    var phi_51_: i32;
    var phi_52_: f32;
    var phi_53_: f32;
    var phi_54_: vec2<f32>;
    var phi_55_: i32;
    var phi_56_: f32;
    var phi_57_: f32;
    var phi_58_: vec2<f32>;
    var local_46: f32;
    var local_47: f32;
    var phi_59_: vec3<f32>;
    var phi_60_: vec3<f32>;
    var phi_61_: vec3<f32>;
    var phi_62_: vec3<f32>;
    var phi_63_: i32;
    var phi_64_: f32;
    var phi_65_: f32;
    var phi_66_: vec2<f32>;
    var phi_67_: i32;
    var phi_68_: f32;
    var phi_69_: f32;
    var phi_70_: vec2<f32>;
    var local_48: f32;
    var phi_71_: vec3<f32>;
    var phi_72_: i32;
    var phi_73_: f32;
    var phi_74_: f32;
    var phi_75_: vec2<f32>;
    var phi_76_: i32;
    var phi_77_: f32;
    var phi_78_: f32;
    var phi_79_: vec2<f32>;
    var local_49: f32;
    var phi_80_: f32;
    var phi_81_: vec3<f32>;
    var local_50: f32;
    var local_51: f32;
    var local_52: f32;
    var local_53: f32;
    var phi_82_: u32;
    var phi_83_: u32;
    var phi_84_: u32;
    var phi_85_: u32;
    var phi_86_: u32;
    var phi_87_: bool;
    var phi_88_: u32;
    var phi_89_: u32;
    var phi_90_: u32;
    var phi_91_: u32;
    var phi_92_: bool;
    var phi_93_: u32;
    var phi_94_: u32;
    var phi_95_: bool;
    var phi_96_: u32;
    var phi_97_: u32;
    var phi_98_: u32;
    var phi_99_: u32;
    var phi_100_: u32;
    var local_54: u32;
    var phi_101_: u32;
    var phi_102_: f32;
    var phi_103_: f32;
    var phi_104_: u32;
    var phi_105_: i32;
    var phi_106_: f32;
    var phi_107_: u32;
    var phi_108_: i32;
    var local_55: f32;
    var phi_109_: f32;
    var local_56: i32;
    var phi_110_: bool;
    var phi_111_: f32;
    var phi_112_: f32;
    var phi_113_: f32;
    var phi_114_: f32;
    var phi_115_: f32;
    var phi_116_: u32;
    var phi_117_: f32;
    var phi_118_: bool;
    var phi_119_: f32;
    var phi_120_: f32;
    var phi_121_: bool;
    var phi_122_: f32;
    var phi_123_: bool;
    var phi_124_: f32;
    var phi_125_: bool;
    var local_57: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e218 = pixel_3;
            let _e219 = weather_1;
            let _e220 = _isthmus_instance_index_11;
            let _e231 = pill_2.member[_e220].x;
            let _e235 = frame.member[0u].panel_height;
            let _e236 = (_e218.x - _e231);
            let _e237 = (_e218.y - 6f);
            let _e238 = (_e235 * 0.5f);
            let _e242 = ((308f - _e235) * 0.5f);
            let _e244 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e236 - 154f), (_e237 - _e238)), _e242, _e238);
            let _e248 = frame.member[0u].mouse_pressure;
            let _e249 = (_e248 > 0f);
            if _e249 {
                let _e254 = frame.member[0u].mouse_pos[0u];
                let _e259 = frame.member[0u].mouse_pos[1u];
                let _e265 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e254 - _e231) - 154f), ((_e259 - 6f) - _e238)), _e242, _e238);
                phi_0_ = _e265;
            } else {
                phi_0_ = 1f;
            }
            let _e267 = phi_0_;
            phi_1_ = 0u;
            loop {
                let _e269 = phi_1_;
                let _e270 = (_e269 < 4u);
                if _e270 {
                    if _e270 {
                    } else {
                        phi_3_ = true;
                        break;
                    }
                    phi_2_ = (_e269 + 1u);
                } else {
                    phi_2_ = u32();
                }
                let _e273 = phi_2_;
                continue;
                continuing {
                    phi_1_ = _e273;
                    phi_3_ = false;
                    break if !(_e270);
                }
            }
            let _e276 = phi_3_;
            if _e276 {
                break;
            }
            let _e282 = (_e231 - (_e219.w * 158f));
            let _e283 = (6f + _e235);
            let _e284 = (8f * _e219.w);
            let _e285 = ((244f * _e219.w) - _e284);
            if (_e285 != _e285) {
                phi_4_ = true;
            } else {
                phi_4_ = (0f >= _e285);
            }
            let _e289 = phi_4_;
            let _e292 = (_e218.y - _e283);
            let _e293 = ((308f + (316f * _e219.w)) * 0.5f);
            let _e294 = (select(_e285, 0f, _e289) * 0.5f);
            let _e295 = (_e284 + _e294);
            let _e298 = (_e294 != _e294);
            if _e298 {
                phi_5_ = true;
            } else {
                phi_5_ = (18f <= _e294);
            }
            let _e301 = phi_5_;
            let _e304 = vec2<f32>(_e293, _e294);
            let _e305 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e218.x - _e282) - _e293), (_e292 - _e295)), _e304, select(_e294, 18f, _e301));
            let _e310 = frame.member[0u].mouse_pos[0u];
            let _e315 = frame.member[0u].mouse_pos[1u];
            if _e298 {
                phi_6_ = true;
            } else {
                phi_6_ = (18f <= _e294);
            }
            let _e322 = phi_6_;
            let _e325 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e310 - _e282) - _e293), ((_e315 - _e283) - _e295)), _e304, select(_e294, 18f, _e322));
            let _e328 = (0.5f + ((_e305 - _e244) * 0.008928572f));
            let _e330 = select(_e328, 0f, (_e328 < 0f));
            let _e332 = select(_e330, 1f, (_e330 > 1f));
            let _e345 = (0.5f + ((_e325 - _e267) * 0.008928572f));
            let _e347 = select(_e345, 0f, (_e345 < 0f));
            let _e349 = select(_e347, 1f, (_e347 > 1f));
            phi_7_ = vec2<f32>(0f, 0f);
            phi_8_ = 0f;
            phi_9_ = 0u;
            loop {
                let _e361 = phi_7_;
                let _e363 = phi_8_;
                let _e365 = phi_9_;
                local_41 = _e361;
                local_42 = _e361;
                local_43 = _e361;
                local_44 = _e361;
                local_50 = _e363;
                local_51 = _e363;
                local_52 = _e363;
                local_53 = _e363;
                let _e366 = (_e365 < 4u);
                if _e366 {
                    if _e366 {
                    } else {
                        phi_19_ = true;
                        break;
                    }
                    let _e373 = frame.member[0u].ripples[_e365].origin[0u];
                    let _e380 = frame.member[0u].ripples[_e365].origin[1u];
                    let _e386 = frame.member[0u].ripples[_e365].start_time;
                    let _e392 = frame.member[0u].ripples[_e365].strength;
                    let _e396 = frame.member[0u].time;
                    let _e398 = ((_e396 - _e386) * 1.2f);
                    let _e400 = select(_e398, 0f, (_e398 < 0f));
                    let _e402 = select(_e400, 1f, (_e400 > 1f));
                    if (_e392 > 0f) {
                        if (_e402 < 1f) {
                            let _e405 = (_e218.x - _e373);
                            let _e406 = (_e218.y - _e380);
                            let _e410 = sqrt(((_e405 * _e405) + (_e406 * _e406)));
                            if (_e410 > 0.001f) {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e405 / _e410), (_e406 / _e410)), _e410);
                            } else {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e410);
                            }
                            let _e418 = phi_10_;
                            let _e428 = ((abs((_e418.unnamed_1 - (_e402 * 600f))) - 80f) * -0.0125f);
                            let _e430 = select(_e428, 0f, (_e428 < 0f));
                            let _e432 = select(_e430, 1f, (_e430 > 1f));
                            let _e438 = (1f - _e402);
                            let _e439 = ((((_e432 * _e432) * (3f - (2f * _e432))) * _e392) * _e438);
                            let _e452 = (_e363 + (_e439 * 0.5f));
                            if (_e452 != _e452) {
                                phi_11_ = true;
                            } else {
                                phi_11_ = (1f <= _e452);
                            }
                            let _e456 = phi_11_;
                            phi_12_ = vec2<f32>((_e361.x + (((_e418.unnamed.x * _e439) * _e438) * 0.5f)), (_e361.y + (((_e418.unnamed.y * _e439) * _e438) * 0.5f)));
                            phi_13_ = select(_e452, 1f, _e456);
                        } else {
                            phi_12_ = _e361;
                            phi_13_ = _e363;
                        }
                        let _e459 = phi_12_;
                        let _e461 = phi_13_;
                        phi_14_ = _e459;
                        phi_15_ = _e461;
                    } else {
                        phi_14_ = _e361;
                        phi_15_ = _e363;
                    }
                    let _e463 = phi_14_;
                    let _e465 = phi_15_;
                    phi_16_ = _e463;
                    phi_17_ = _e465;
                    phi_18_ = (_e365 + 1u);
                } else {
                    phi_16_ = vec2<f32>();
                    phi_17_ = f32();
                    phi_18_ = u32();
                }
                let _e468 = phi_16_;
                let _e470 = phi_17_;
                let _e472 = phi_18_;
                continue;
                continuing {
                    phi_7_ = _e468;
                    phi_8_ = _e470;
                    phi_9_ = _e472;
                    phi_19_ = _e276;
                    break if !(_e366);
                }
            }
            let _e475 = phi_19_;
            if _e475 {
                break;
            }
            if _e249 {
                let _e476 = (_e218.x - _e310);
                let _e477 = (_e218.y - _e315);
                let _e483 = ((sqrt(((_e476 * _e476) + (_e477 * _e477))) - 150f) * -0.006666667f);
                let _e485 = select(_e483, 0f, (_e483 < 0f));
                let _e487 = select(_e485, 1f, (_e485 > 1f));
                phi_20_ = ((((_e487 * _e487) * (3f - (2f * _e487))) * _e248) * 8f);
            } else {
                phi_20_ = 0f;
            }
            let _e495 = phi_20_;
            let _e497 = local_41;
            let _e499 = global[0u];
            if (_e497.x == _e499) {
                let _e502 = local_42;
                let _e505 = global[1u];
                phi_21_ = (_e502.y == _e505);
            } else {
                phi_21_ = false;
            }
            let _e508 = phi_21_;
            if _e508 {
                phi_22_ = 0f;
            } else {
                let _e510 = local_43;
                phi_22_ = (sqrt(((_e497.x * _e497.x) + (_e510.y * _e510.y))) * 22f);
            }
            let _e518 = phi_22_;
            let _e520 = local_44;
            let _e523 = (((_e267 + ((((_e325 + ((_e267 - _e325) * _e349)) - ((56f * _e349) * (1f - _e349))) - _e267) * _e219.w)) - 0.5f) * -1f);
            let _e525 = select(_e523, 0f, (_e523 < 0f));
            let _e527 = select(_e525, 1f, (_e525 > 1f));
            let _e535 = ((_e244 + ((((_e305 + ((_e244 - _e305) * _e332)) - ((56f * _e332) * (1f - _e332))) - _e244) * _e219.w)) - (((_e495 * ((_e527 * _e527) * (3f - (2f * _e527)))) + _e518) * 0.5f));
            let _e537 = (_e235 + 60f);
            let _e538 = ((_e237 - _e235) > _e537);
            let _e543 = pill_2.member[_e220].calendar_expansion;
            let _e544 = (56f + _e238);
            let _e545 = (_e235 + 8f);
            let _e547 = (_e544 + (select(0f, 1f, _e538) * _e545));
            let _e548 = (_e547 * 0.0007377049f);
            let _e549 = (0.5f + _e548);
            let _e553 = ((_e543 - _e549) / ((_e548 + 0.74f) - _e549));
            let _e555 = select(_e553, 0f, (_e553 < 0f));
            let _e557 = select(_e555, 1f, (_e555 > 1f));
            let _e561 = ((_e557 * _e557) * (3f - (2f * _e557)));
            let _e563 = (292f * _e561);
            let _e564 = (_e235 * _e561);
            let _e572 = ((_e231 + 166f) + ((292f - _e563) * 0.5f));
            let _e573 = ((_e283 + (_e547 - _e238)) + ((_e235 - _e564) * 0.5f));
            let _e574 = (_e218.x - _e572);
            let _e575 = (_e218.y - _e573);
            let _e576 = select(6u, 5u, _e538);
            if (_e563 != _e563) {
                phi_23_ = true;
            } else {
                phi_23_ = (0.001f >= _e563);
            }
            let _e580 = phi_23_;
            let _e585 = (((_e574 / select(_e563, 0.001f, _e580)) * f32(_e576)) - 0.5f);
            let _e587 = f32((_e576 - 1u));
            if (0f <= _e587) {
            } else {
                break;
            }
            let _e590 = select(_e585, 0f, (_e585 < 0f));
            let _e592 = select(_e590, _e587, (_e590 > _e587));
            let _e593 = floor(_e592);
            let _e598 = select(select(u32(_e593), 0u, (_e593 < 0f)), 4294967295u, (_e593 > 4294967000f));
            let _e600 = (_e592 - trunc(_e592));
            let _e602 = select(_e600, 0f, (_e600 < 0f));
            let _e604 = select(_e602, 1f, (_e602 > 1f));
            let _e608 = ((_e604 * _e604) * (3f - (2f * _e604)));
            if _e538 {
                if (_e598 < 5u) {
                } else {
                    break;
                }
                let _e636 = pill_2.member[_e220].daily_conditions[_e598];
                let _e637 = (_e598 + 1u);
                let _e639 = select(_e637, 4u, (4u < _e637));
                if (_e639 < 5u) {
                } else {
                    break;
                }
                let _e645 = pill_2.member[_e220].daily_conditions[_e639];
                phi_24_ = 12f;
                phi_25_ = _e645;
                phi_26_ = _e636;
            } else {
                if (_e598 < 6u) {
                } else {
                    break;
                }
                let _e614 = pill_2.member[_e220].hourly_conditions[_e598];
                let _e615 = (_e598 + 1u);
                let _e617 = select(_e615, 5u, (5u < _e615));
                if (_e617 < 6u) {
                } else {
                    break;
                }
                let _e623 = pill_2.member[_e220].hourly_conditions[_e617];
                let _e627 = pill_2.member[_e220].hourly_start;
                phi_24_ = ((_e627 + (_e592 * 4f)) % 24f);
                phi_25_ = _e623;
                phi_26_ = _e614;
            }
            let _e647 = phi_24_;
            let _e649 = phi_25_;
            let _e651 = phi_26_;
            let _e652 = (_e561 <= 0.001f);
            if _e652 {
                phi_27_ = 340282350000000000000000000000000000000f;
            } else {
                let _e654 = (_e564 * 0.5f);
                let _e660 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e574 - (_e561 * 146f)), (_e575 - _e654)), ((_e563 - _e564) * 0.5f), _e654);
                phi_27_ = _e660;
            }
            let _e662 = phi_27_;
            if _e652 {
                phi_28_ = 340282350000000000000000000000000000000f;
            } else {
                let _e666 = (_e564 * 0.5f);
                let _e672 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e310 - _e572) - (_e561 * 146f)), ((_e315 - _e573) - _e666)), ((_e563 - _e564) * 0.5f), _e666);
                phi_28_ = _e672;
            }
            let _e674 = phi_28_;
            let _e708 = pill_2.member[_e220].sun_hours;
            let _e711 = (_e708[1] - _e708[0]);
            if (_e647 >= _e708[0]) {
                let _e713 = (_e647 <= _e708[1]);
                if _e713 {
                    let _e715 = ((_e647 - _e708[0]) / _e711);
                    phi_29_ = array<f32, 2>(_e715, sin((_e715 * 3.1415927f)));
                } else {
                    phi_29_ = array<f32, 2>();
                }
                let _e720 = phi_29_;
                phi_30_ = _e720;
                phi_31_ = select(true, false, _e713);
            } else {
                phi_30_ = array<f32, 2>();
                phi_31_ = true;
            }
            let _e723 = phi_30_;
            let _e725 = phi_31_;
            if _e725 {
                let _e726 = (24f - _e711);
                if (_e647 < _e708[0]) {
                    phi_32_ = (((_e647 + 24f) - _e708[1]) / _e726);
                } else {
                    phi_32_ = ((_e647 - _e708[1]) / _e726);
                }
                let _e734 = phi_32_;
                phi_33_ = array<f32, 2>(select(0f, 1f, (_e647 >= _e708[1])), -(sin((_e734 * 3.1415927f))));
            } else {
                phi_33_ = _e723;
            }
            let _e742 = phi_33_;
            let _e745 = ((_e674 - 0.5f) * -1f);
            let _e747 = select(_e745, 0f, (_e745 < 0f));
            let _e749 = select(_e747, 1f, (_e747 > 1f));
            let _e757 = (_e662 - (((_e495 * ((_e749 * _e749) * (3f - (2f * _e749)))) + _e518) * 0.5f));
            let _e758 = (_e535 != _e535);
            if _e758 {
                phi_34_ = true;
            } else {
                phi_34_ = (_e757 <= _e535);
            }
            let _e761 = phi_34_;
            let _e762 = select(_e535, _e757, _e761);
            let _e763 = fwidth(_e762);
            if (_e763 != _e763) {
                phi_35_ = true;
            } else {
                phi_35_ = (0.55f >= _e763);
            }
            let _e767 = phi_35_;
            let _e768 = select(_e763, 0.55f, _e767);
            let _e772 = ((_e762 - _e768) / (-(_e768) - _e768));
            let _e774 = select(_e772, 0f, (_e772 < 0f));
            let _e776 = select(_e774, 1f, (_e774 > 1f));
            let _e780 = ((_e776 * _e776) * (3f - (2f * _e776)));
            if (_e762 != _e762) {
                phi_36_ = true;
            } else {
                phi_36_ = (0f >= _e762);
            }
            let _e784 = phi_36_;
            let _e788 = (exp((select(_e762, 0f, _e784) * -0.3f)) * 0.16f);
            if (_e780 != _e780) {
                phi_37_ = true;
            } else {
                phi_37_ = (_e788 >= _e780);
            }
            let _e792 = phi_37_;
            let _e793 = select(_e780, _e788, _e792);
            if (_e793 <= 0.0009765625f) {
                discard;
            }
            let _e799 = pill_2.member[_e220].hourly_conditions[0u];
            let _e800 = (_e236 * 0.0032467532f);
            let _e802 = select(_e800, 0f, (_e800 < 0f));
            let _e811 = pill_2.member[_e220].hourly_conditions[1u];
            let _e813 = ((abs((select(_e802, 1f, (_e802 > 1f)) - 0.5f)) - 0.2f) * 20.000002f);
            let _e815 = select(_e813, 0f, (_e813 < 0f));
            let _e817 = select(_e815, 1f, (_e815 > 1f));
            let _e821 = ((_e817 * _e817) * (3f - (2f * _e817)));
            let _e826 = (_e799.fog + ((_e811.fog - _e799.fog) * _e821));
            let _e831 = (_e799.cloud + ((_e811.cloud - _e799.cloud) * _e821));
            let _e836 = (_e799.rain + ((_e811.rain - _e799.rain) * _e821));
            let _e841 = (_e799.snow + ((_e811.snow - _e799.snow) * _e821));
            let _e846 = (_e799.lightning + ((_e811.lightning - _e799.lightning) * _e821));
            let _e851 = (_e799.hail + ((_e811.hail - _e799.hail) * _e821));
            let _e854 = (_e826 + ((_e799.fog - _e826) * _e219.w));
            let _e857 = (_e831 + ((_e799.cloud - _e831) * _e219.w));
            let _e860 = (_e836 + ((_e799.rain - _e836) * _e219.w));
            let _e863 = (_e841 + ((_e799.snow - _e841) * _e219.w));
            let _e866 = (_e846 + ((_e799.lightning - _e846) * _e219.w));
            let _e869 = (_e851 + ((_e799.hail - _e851) * _e219.w));
            let _e870 = (_e237 / _e235);
            if _e758 {
                phi_38_ = true;
            } else {
                phi_38_ = (0f <= _e535);
            }
            let _e875 = phi_38_;
            let _e878 = (1f + (select(_e535, 0f, _e875) * 0.008333334f));
            let _e880 = select(_e878, 0f, (_e878 < 0f));
            let _e882 = select(_e880, 0.6f, (_e880 > 0.6f));
            let _e889 = (_e497.x * 0.04f);
            let _e890 = (_e520.y * 0.04f);
            let _e891 = ((_e800 - (((_e800 - 0.5f) * _e882) * 0.08f)) - _e889);
            let _e892 = ((_e870 - (((_e870 - 0.5f) * _e882) * 0.08f)) - _e890);
            if (_e561 > 0.001f) {
                let _e895 = (_e574 / _e563);
                let _e896 = (_e575 / _e564);
                if (_e757 != _e757) {
                    phi_39_ = true;
                } else {
                    phi_39_ = (0f <= _e757);
                }
                let _e902 = phi_39_;
                let _e905 = (1f + (select(_e757, 0f, _e902) * 0.008333334f));
                let _e907 = select(_e905, 0f, (_e905 < 0f));
                let _e909 = select(_e907, 0.6f, (_e907 > 0.6f));
                phi_40_ = vec2<f32>(((_e895 - (((_e895 - 0.5f) * _e909) * 0.08f)) - _e889), ((_e896 - (((_e896 - 0.5f) * _e909) * 0.08f)) - _e890));
            } else {
                phi_40_ = vec2<f32>(_e891, _e892);
            }
            let _e920 = phi_40_;
            let _e921 = fwidth(_e757);
            if (_e921 != _e921) {
                phi_41_ = true;
            } else {
                phi_41_ = (0.55f >= _e921);
            }
            let _e925 = phi_41_;
            let _e926 = select(_e921, 0.55f, _e925);
            let _e930 = ((_e757 - _e926) / (-(_e926) - _e926));
            let _e932 = select(_e930, 0f, (_e930 < 0f));
            let _e934 = select(_e932, 1f, (_e932 > 1f));
            let _e939 = (((_e934 * _e934) * (3f - (2f * _e934))) * _e561);
            let _e946 = (1f - _e939);
            let _e951 = (((_e891 * 308f) * _e946) + ((_e920.x * _e563) * _e939));
            let _e952 = (((_e892 * _e235) * _e946) + ((_e920.y * _e564) * _e939));
            if (_e757 != _e757) {
                phi_42_ = true;
            } else {
                phi_42_ = (1000f <= _e757);
            }
            let _e959 = phi_42_;
            let _e966 = (_e854 + (((_e651.fog + ((_e649.fog - _e651.fog) * _e608)) - _e854) * _e939));
            let _e969 = (_e857 + (((_e651.cloud + ((_e649.cloud - _e651.cloud) * _e608)) - _e857) * _e939));
            let _e972 = (_e860 + (((_e651.rain + ((_e649.rain - _e651.rain) * _e608)) - _e860) * _e939));
            let _e975 = (_e863 + (((_e651.snow + ((_e649.snow - _e651.snow) * _e608)) - _e863) * _e939));
            let _e981 = (_e869 + (((_e651.hail + ((_e649.hail - _e651.hail) * _e608)) - _e869) * _e939));
            let _e983 = ((_e219.y - -0.04f) * 4.1666665f);
            let _e985 = select(_e983, 0f, (_e983 < 0f));
            let _e987 = select(_e985, 1f, (_e985 > 1f));
            let _e991 = ((_e987 * _e987) * (3f - (2f * _e987)));
            let _e993 = ((_e219.y - -0.32f) * 4.166667f);
            let _e995 = select(_e993, 0f, (_e993 < 0f));
            let _e997 = select(_e995, 1f, (_e995 > 1f));
            let _e1005 = ((_e219.y - -0.18f) * 5.5555553f);
            let _e1007 = select(_e1005, 0f, (_e1005 < 0f));
            let _e1009 = select(_e1007, 1f, (_e1007 > 1f));
            let _e1015 = ((_e219.y - 0.2f) * -5.5555553f);
            let _e1017 = select(_e1015, 0f, (_e1015 < 0f));
            let _e1019 = select(_e1017, 1f, (_e1017 > 1f));
            let _e1026 = ((_e742[1] - -0.04f) * 4.1666665f);
            let _e1028 = select(_e1026, 0f, (_e1026 < 0f));
            let _e1030 = select(_e1028, 1f, (_e1028 > 1f));
            let _e1034 = ((_e1030 * _e1030) * (3f - (2f * _e1030)));
            let _e1036 = ((_e742[1] - -0.32f) * 4.166667f);
            let _e1038 = select(_e1036, 0f, (_e1036 < 0f));
            let _e1040 = select(_e1038, 1f, (_e1038 > 1f));
            let _e1048 = ((_e742[1] - -0.18f) * 5.5555553f);
            let _e1050 = select(_e1048, 0f, (_e1048 < 0f));
            let _e1052 = select(_e1050, 1f, (_e1050 > 1f));
            let _e1058 = ((_e742[1] - 0.2f) * -5.5555553f);
            let _e1060 = select(_e1058, 0f, (_e1058 < 0f));
            let _e1062 = select(_e1060, 1f, (_e1060 > 1f));
            let _e1074 = ((_e991 * _e946) + (_e1034 * _e939));
            let _e1076 = (((((_e1009 * _e1009) * (3f - (2f * _e1009))) * ((_e1019 * _e1019) * (3f - (2f * _e1019)))) * _e946) + ((((_e1052 * _e1052) * (3f - (2f * _e1052))) * ((_e1062 * _e1062) * (3f - (2f * _e1062)))) * _e939));
            let _e1080 = frame.member[0u].time;
            let _e1081 = (_e952 / _e235);
            let _e1083 = ((_e1081 - 1f) * -1f);
            let _e1085 = select(_e1083, 0f, (_e1083 < 0f));
            let _e1087 = select(_e1085, 1f, (_e1085 > 1f));
            let _e1091 = ((_e1087 * _e1087) * (3f - (2f * _e1087)));
            let _e1092 = (1f - _e1091);
            let _e1111 = (1f - _e1074);
            let _e1123 = (0.3f * _e1092);
            let _e1124 = (0.22f * _e1091);
            let _e1130 = ((((((_e997 * _e997) * (3f - (2f * _e997))) * (1f - _e991)) * _e946) + ((((_e1040 * _e1040) * (3f - (2f * _e1040))) * (1f - _e1034)) * _e939)) * 0.8f);
            let _e1131 = (1f - _e1130);
            let _e1148 = (_e1076 * 0.9f);
            let _e1149 = (1f - _e1148);
            let _e1161 = floor((_e951 * 0.055555556f));
            let _e1162 = floor((_e952 * 0.055555556f));
            let _e1166 = cantus_render_shader_hash(vec2<f32>(_e1161, _e1162));
            let _e1175 = (_e951 - (((_e1161 + 0.2f) + (_e1166.x * 0.6f)) * 18f));
            let _e1176 = (_e952 - (((_e1162 + 0.2f) + (_e1166.y * 0.6f)) * 18f));
            let _e1182 = ((sqrt(((_e1175 * _e1175) + (_e1176 * _e1176))) - 1f) * -1.6666666f);
            let _e1184 = select(_e1182, 0f, (_e1182 < 0f));
            let _e1186 = select(_e1184, 1f, (_e1184 > 1f));
            let _e1194 = cantus_render_shader_hash(vec2<f32>((_e1161 + 31.7f), (_e1162 + 31.7f)));
            let _e1197 = ((_e1194.x - 0.75f) * 4f);
            let _e1199 = select(_e1197, 0f, (_e1197 < 0f));
            let _e1201 = select(_e1199, 1f, (_e1199 > 1f));
            let _e1212 = ((((((_e1186 * _e1186) * (3f - (2f * _e1186))) * ((_e1201 * _e1201) * (3f - (2f * _e1201)))) * _e1111) * (1f - _e969)) * (0.3f + (_e1091 * 0.7f)));
            let _e1213 = (((((((((0.006f * _e1092) + (0.025f * _e1091)) * _e1111) + (((0.08f * _e1092) + (0.32f * _e1091)) * _e1074)) * _e1131) + (((0.1f * _e1092) + _e1124) * _e1130)) * _e1149) + (((0.78f * _e1092) + (0.38f * _e1091)) * _e1148)) + _e1212);
            let _e1214 = (((((((((0.012f * _e1092) + (0.04f * _e1091)) * _e1111) + (((0.34f * _e1092) + (0.67f * _e1091)) * _e1074)) * _e1131) + (((0.16f * _e1092) + (0.25f * _e1091)) * _e1130)) * _e1149) + ((_e1123 + _e1124) * _e1148)) + _e1212);
            let _e1215 = (((((((((0.035f * _e1092) + (0.095f * _e1091)) * _e1111) + (((0.62f * _e1092) + (0.87f * _e1091)) * _e1074)) * _e1131) + ((_e1123 + (0.45f * _e1091)) * _e1130)) * _e1149) + (((0.2f * _e1092) + (0.42f * _e1091)) * _e1148)) + _e1212);
            if (_e969 > 0.0009765625f) {
                let _e1218 = (_e951 / _e235);
                phi_43_ = 0i;
                phi_44_ = 0.5f;
                phi_45_ = 0f;
                phi_46_ = vec2<f32>(((_e1218 * 0.14f) + (_e1080 * 0.012f)), ((_e1081 * 0.14f) + 6.1f));
                loop {
                    let _e1226 = phi_43_;
                    let _e1228 = phi_44_;
                    let _e1230 = phi_45_;
                    let _e1232 = phi_46_;
                    local_45 = _e1230;
                    let _e1233 = (_e1226 < 4i);
                    if _e1233 {
                        let _e1236 = cantus_render_shader_simplex_noise(_e1232);
                        phi_47_ = (_e1226 + 1i);
                        phi_48_ = (_e1228 * 0.5f);
                        phi_49_ = (_e1230 + (_e1236 * _e1228));
                        phi_50_ = vec2<f32>(((_e1232.x * 1.6f) + (_e1232.y * 1.2f)), ((_e1232.y * 1.6f) - (_e1232.x * 1.2f)));
                    } else {
                        phi_47_ = i32();
                        phi_48_ = f32();
                        phi_49_ = f32();
                        phi_50_ = vec2<f32>();
                    }
                    let _e1249 = phi_47_;
                    let _e1251 = phi_48_;
                    let _e1253 = phi_49_;
                    let _e1255 = phi_50_;
                    continue;
                    continuing {
                        phi_43_ = _e1249;
                        phi_44_ = _e1251;
                        phi_45_ = _e1253;
                        phi_46_ = _e1255;
                        break if !(_e1233);
                    }
                }
                let _e1258 = local_45;
                let _e1259 = (_e1258 * 0.5f);
                phi_51_ = 0i;
                phi_52_ = 0.5f;
                phi_53_ = 0f;
                phi_54_ = vec2<f32>(((_e1218 * 0.287f) + (_e1080 * 0.018f)), ((_e1081 * 0.287f) + -3.7f));
                loop {
                    let _e1268 = phi_51_;
                    let _e1270 = phi_52_;
                    let _e1272 = phi_53_;
                    let _e1274 = phi_54_;
                    local_46 = _e1272;
                    local_47 = _e1272;
                    let _e1275 = (_e1268 < 4i);
                    if _e1275 {
                        let _e1278 = cantus_render_shader_simplex_noise(_e1274);
                        phi_55_ = (_e1268 + 1i);
                        phi_56_ = (_e1270 * 0.5f);
                        phi_57_ = (_e1272 + (_e1278 * _e1270));
                        phi_58_ = vec2<f32>(((_e1274.x * 1.6f) + (_e1274.y * 1.2f)), ((_e1274.y * 1.6f) - (_e1274.x * 1.2f)));
                    } else {
                        phi_55_ = i32();
                        phi_56_ = f32();
                        phi_57_ = f32();
                        phi_58_ = vec2<f32>();
                    }
                    let _e1291 = phi_55_;
                    let _e1293 = phi_56_;
                    let _e1295 = phi_57_;
                    let _e1297 = phi_58_;
                    continue;
                    continuing {
                        phi_51_ = _e1291;
                        phi_52_ = _e1293;
                        phi_53_ = _e1295;
                        phi_54_ = _e1297;
                        break if !(_e1275);
                    }
                }
                let _e1300 = local_46;
                let _e1303 = local_47;
                let _e1307 = ((((0.5f + _e1259) + (_e1303 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1309 = select(_e1307, 0f, (_e1307 < 0f));
                let _e1311 = select(_e1309, 1f, (_e1309 > 1f));
                let _e1317 = (((_e1300 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1319 = select(_e1317, 0f, (_e1317 < 0f));
                let _e1321 = select(_e1319, 1f, (_e1319 > 1f));
                let _e1328 = ((_e1259 + 0.02000001f) * 4.5454545f);
                let _e1330 = select(_e1328, 0f, (_e1328 < 0f));
                let _e1332 = select(_e1330, 1f, (_e1330 > 1f));
                let _e1338 = ((((_e1321 * _e1321) * (3f - (2f * _e1321))) * 0.55f) + (((_e1332 * _e1332) * (3f - (2f * _e1332))) * 0.45f));
                let _e1339 = (1f - _e1338);
                let _e1376 = (_e1076 * 0.45f);
                let _e1377 = (1f - _e1376);
                let _e1389 = (_e969 * (0.12f + (((_e1311 * _e1311) * (3f - (2f * _e1311))) * 0.7f)));
                let _e1390 = (1f - _e1389);
                phi_59_ = vec3<f32>(((_e1213 * _e1390) + (((((((0.16f * _e1339) + (0.32f * _e1338)) * _e1111) + (((0.62f * _e1339) + (0.92f * _e1338)) * _e1074)) * _e1377) + (((0.5f * _e1339) + (0.76f * _e1338)) * _e1376)) * _e1389)), ((_e1214 * _e1390) + (((((((0.2f * _e1339) + (0.36f * _e1338)) * _e1111) + (((0.7f * _e1339) + (0.94f * _e1338)) * _e1074)) * _e1377) + (((0.36f * _e1339) + (0.59f * _e1338)) * _e1376)) * _e1389)), ((_e1215 * _e1390) + (((((((0.28f * _e1339) + (0.43f * _e1338)) * _e1111) + (((0.78f * _e1339) + (0.96f * _e1338)) * _e1074)) * _e1377) + (((0.4f * _e1339) + (0.56f * _e1338)) * _e1376)) * _e1389)));
            } else {
                phi_59_ = vec3<f32>(_e1213, _e1214, _e1215);
            }
            let _e1402 = phi_59_;
            let _e1404 = (1f - (_e972 * 0.2f));
            let _e1414 = ((_e1402.x * _e1404) + (_e972 * 0.020000001f));
            let _e1415 = ((_e1402.y * _e1404) + (_e972 * 0.034f));
            let _e1416 = ((_e1402.z * _e1404) + (_e972 * 0.05f));
            if (_e972 > 0.0009765625f) {
                let _e1421 = (_e951 - (20f * _e1080));
                let _e1422 = (_e952 - (110f * _e1080));
                let _e1425 = floor((_e1421 * 0.06666667f));
                let _e1426 = floor((_e1422 * 0.04f));
                let _e1428 = cantus_render_shader_hash(vec2<f32>(_e1425, _e1426));
                let _e1439 = (_e1421 - (((_e1425 + 0.15f) + (_e1428.x * 0.7f)) * 15f));
                let _e1440 = (_e1422 - (((_e1426 + 0.15f) + (_e1428.y * 0.7f)) * 25f));
                let _e1444 = (((_e1439 * 1.8000001f) + (_e1440 * 9f)) * 0.011870845f);
                let _e1446 = select(_e1444, 0f, (_e1444 < 0f));
                let _e1448 = select(_e1446, 1f, (_e1446 > 1f));
                let _e1451 = (_e1439 - (1.8000001f * _e1448));
                let _e1452 = (_e1440 - (9f * _e1448));
                let _e1458 = ((sqrt(((_e1451 * _e1451) + (_e1452 * _e1452))) - 1.0999999f) * -1.666667f);
                let _e1460 = select(_e1458, 0f, (_e1458 < 0f));
                let _e1462 = select(_e1460, 1f, (_e1460 > 1f));
                let _e1470 = cantus_render_shader_hash(vec2<f32>((_e1425 + 19.3f), (_e1426 + 19.3f)));
                let _e1473 = ((_e1470.x - 0.22000003f) * 1.2820513f);
                let _e1475 = select(_e1473, 0f, (_e1473 < 0f));
                let _e1477 = select(_e1475, 1f, (_e1475 > 1f));
                let _e1484 = (((((_e1462 * _e1462) * (3f - (2f * _e1462))) * ((_e1477 * _e1477) * (3f - (2f * _e1477)))) * _e972) * 0.7f);
                let _e1486 = select(_e1484, 0f, (_e1484 < 0f));
                let _e1488 = select(_e1486, 1f, (_e1486 > 1f));
                let _e1489 = (1f - _e1488);
                phi_60_ = vec3<f32>(((_e1414 * _e1489) + (0.52f * _e1488)), ((_e1415 * _e1489) + (0.72f * _e1488)), ((_e1416 * _e1489) + (0.9f * _e1488)));
            } else {
                phi_60_ = vec3<f32>(_e1414, _e1415, _e1416);
            }
            let _e1501 = phi_60_;
            if (_e975 > 0.0009765625f) {
                let _e1505 = (_e951 - (5f * _e1080));
                let _e1506 = (_e952 - (14f * _e1080));
                let _e1509 = floor((_e1505 * 0.05f));
                let _e1510 = floor((_e1506 * 0.05f));
                let _e1514 = cantus_render_shader_hash(vec2<f32>((_e1509 + 31.7f), (_e1510 + 31.7f)));
                let _e1525 = (_e1505 - (((_e1509 + 0.15f) + (_e1514.x * 0.7f)) * 20f));
                let _e1526 = (_e1506 - (((_e1510 + 0.15f) + (_e1514.y * 0.7f)) * 20f));
                let _e1530 = (((_e1525 * 0.080000006f) + (_e1526 * 0.4f)) * 6.009615f);
                let _e1532 = select(_e1530, 0f, (_e1530 < 0f));
                let _e1534 = select(_e1532, 1f, (_e1532 > 1f));
                let _e1537 = (_e1525 - (0.080000006f * _e1534));
                let _e1538 = (_e1526 - (0.4f * _e1534));
                let _e1544 = ((sqrt(((_e1537 * _e1537) + (_e1538 * _e1538))) - 1.5999999f) * -1.666667f);
                let _e1546 = select(_e1544, 0f, (_e1544 < 0f));
                let _e1548 = select(_e1546, 1f, (_e1546 > 1f));
                let _e1556 = cantus_render_shader_hash(vec2<f32>((_e1509 + 19.3f), (_e1510 + 19.3f)));
                let _e1559 = ((_e1556.x - 0.3f) * 1.4285715f);
                let _e1561 = select(_e1559, 0f, (_e1559 < 0f));
                let _e1563 = select(_e1561, 1f, (_e1561 > 1f));
                let _e1570 = (((((_e1548 * _e1548) * (3f - (2f * _e1548))) * ((_e1563 * _e1563) * (3f - (2f * _e1563)))) * _e975) * 0.92f);
                let _e1572 = select(_e1570, 0f, (_e1570 < 0f));
                let _e1574 = select(_e1572, 1f, (_e1572 > 1f));
                let _e1575 = (1f - _e1574);
                let _e1582 = (0.96f * _e1574);
                phi_61_ = vec3<f32>(((_e1501.x * _e1575) + _e1582), ((_e1501.y * _e1575) + _e1582), ((_e1501.z * _e1575) + _e1582));
            } else {
                phi_61_ = _e1501;
            }
            let _e1588 = phi_61_;
            if (_e981 > 0.0009765625f) {
                let _e1592 = (_e951 - (18f * _e1080));
                let _e1593 = (_e952 - (85f * _e1080));
                let _e1596 = floor((_e1592 * 0.04347826f));
                let _e1597 = floor((_e1593 * 0.04347826f));
                let _e1601 = cantus_render_shader_hash(vec2<f32>((_e1596 + 63.4f), (_e1597 + 63.4f)));
                let _e1612 = (_e1592 - (((_e1596 + 0.15f) + (_e1601.x * 0.7f)) * 23f));
                let _e1613 = (_e1593 - (((_e1597 + 0.15f) + (_e1601.y * 0.7f)) * 23f));
                let _e1617 = (((_e1612 * 0.24000001f) + (_e1613 * 1.2f)) * 0.667735f);
                let _e1619 = select(_e1617, 0f, (_e1617 < 0f));
                let _e1621 = select(_e1619, 1f, (_e1619 > 1f));
                let _e1624 = (_e1612 - (0.24000001f * _e1621));
                let _e1625 = (_e1613 - (1.2f * _e1621));
                let _e1631 = ((sqrt(((_e1624 * _e1624) + (_e1625 * _e1625))) - 0.79999995f) * -1.6666667f);
                let _e1633 = select(_e1631, 0f, (_e1631 < 0f));
                let _e1635 = select(_e1633, 1f, (_e1633 > 1f));
                let _e1643 = cantus_render_shader_hash(vec2<f32>((_e1596 + 19.3f), (_e1597 + 19.3f)));
                let _e1646 = ((_e1643.x - 0.7f) * 3.3333333f);
                let _e1648 = select(_e1646, 0f, (_e1646 < 0f));
                let _e1650 = select(_e1648, 1f, (_e1648 > 1f));
                let _e1657 = (((((_e1635 * _e1635) * (3f - (2f * _e1635))) * ((_e1650 * _e1650) * (3f - (2f * _e1650)))) * _e981) * 0.7f);
                let _e1659 = select(_e1657, 0f, (_e1657 < 0f));
                let _e1661 = select(_e1659, 1f, (_e1659 > 1f));
                let _e1662 = (1f - _e1661);
                phi_62_ = vec3<f32>(((_e1588.x * _e1662) + (0.75f * _e1661)), ((_e1588.y * _e1662) + (0.86f * _e1661)), ((_e1588.z * _e1662) + (0.94f * _e1661)));
            } else {
                phi_62_ = _e1588;
            }
            let _e1677 = phi_62_;
            let _e1681 = ((sin((_e1080 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1683 = select(_e1681, 0f, (_e1681 < 0f));
            let _e1685 = select(_e1683, 1f, (_e1683 > 1f));
            let _e1690 = (((_e1685 * _e1685) * (3f - (2f * _e1685))) * (_e866 + (((_e651.lightning + ((_e649.lightning - _e651.lightning) * _e608)) - _e866) * _e939)));
            let _e1692 = (1f - (_e1690 * 0.55f));
            let _e1702 = ((_e1677.x * _e1692) + (_e1690 * 0.3575f));
            let _e1703 = ((_e1677.y * _e1692) + (_e1690 * 0.407f));
            let _e1704 = ((_e1677.z * _e1692) + (_e1690 * 0.528f));
            if (_e966 > 0.0009765625f) {
                phi_63_ = 0i;
                phi_64_ = 0.5f;
                phi_65_ = 0f;
                phi_66_ = vec2<f32>((((_e951 / (308f + ((_e563 - 308f) * _e939))) * 0.9f) + (_e1080 * 0.008f)), ((_e1081 * 0.32f) + 12f));
                loop {
                    let _e1715 = phi_63_;
                    let _e1717 = phi_64_;
                    let _e1719 = phi_65_;
                    let _e1721 = phi_66_;
                    local_48 = _e1719;
                    let _e1722 = (_e1715 < 4i);
                    if _e1722 {
                        let _e1725 = cantus_render_shader_simplex_noise(_e1721);
                        phi_67_ = (_e1715 + 1i);
                        phi_68_ = (_e1717 * 0.5f);
                        phi_69_ = (_e1719 + (_e1725 * _e1717));
                        phi_70_ = vec2<f32>(((_e1721.x * 1.6f) + (_e1721.y * 1.2f)), ((_e1721.y * 1.6f) - (_e1721.x * 1.2f)));
                    } else {
                        phi_67_ = i32();
                        phi_68_ = f32();
                        phi_69_ = f32();
                        phi_70_ = vec2<f32>();
                    }
                    let _e1738 = phi_67_;
                    let _e1740 = phi_68_;
                    let _e1742 = phi_69_;
                    let _e1744 = phi_70_;
                    continue;
                    continuing {
                        phi_63_ = _e1738;
                        phi_64_ = _e1740;
                        phi_65_ = _e1742;
                        phi_66_ = _e1744;
                        break if !(_e1722);
                    }
                }
                let _e1747 = local_48;
                let _e1750 = (((_e1747 * 0.5f) + 0.15f) * 2.857143f);
                let _e1752 = select(_e1750, 0f, (_e1750 < 0f));
                let _e1754 = select(_e1752, 1f, (_e1752 > 1f));
                let _e1761 = (_e966 * (0.58f + (((_e1754 * _e1754) * (3f - (2f * _e1754))) * 0.18f)));
                let _e1762 = (1f - _e1761);
                phi_71_ = vec3<f32>(((_e1702 * _e1762) + (0.63f * _e1761)), ((_e1703 * _e1762) + (0.69f * _e1761)), ((_e1704 * _e1762) + (0.73f * _e1761)));
            } else {
                phi_71_ = vec3<f32>(_e1702, _e1703, _e1704);
            }
            let _e1774 = phi_71_;
            let _e1776 = ((_e1081 - 0.12f) * -8.333334f);
            let _e1778 = select(_e1776, 0f, (_e1776 < 0f));
            let _e1780 = select(_e1778, 1f, (_e1778 > 1f));
            let _e1787 = (((_e535 + ((select(_e757, 1000f, _e959) - _e535) * _e939)) - 5f) * -0.125f);
            let _e1789 = select(_e1787, 0f, (_e1787 < 0f));
            let _e1791 = select(_e1789, 1f, (_e1789 > 1f));
            let _e1797 = ((((_e1780 * _e1780) * (3f - (2f * _e1780))) * 0.12f) + (((_e1791 * _e1791) * (3f - (2f * _e1791))) * 0.08f));
            let _e1799 = (_e1774.x + _e1797);
            let _e1801 = (_e1774.y + _e1797);
            let _e1803 = (_e1774.z + _e1797);
            if (_e244 < 1f) {
                let _e1808 = (16f + (_e219.x * 276f));
                let _e1810 = select(_e219.y, 0f, (_e219.y < 0f));
                let _e1814 = (0.72f - (select(_e1810, 1f, (_e1810 > 1f)) * 0.45f));
                let _e1817 = ((_e219.y - 0.55f) * -1.8867923f);
                let _e1819 = select(_e1817, 0f, (_e1817 < 0f));
                let _e1821 = select(_e1819, 1f, (_e1819 > 1f));
                let _e1825 = ((_e1821 * _e1821) * (3f - (2f * _e1821)));
                let _e1826 = (1f - _e1825);
                if (_e831 > 0.0009765625f) {
                    phi_72_ = 0i;
                    phi_73_ = 0.5f;
                    phi_74_ = 0f;
                    phi_75_ = vec2<f32>((((_e1808 / _e235) * 0.14f) + (_e1080 * 0.012f)), ((_e1814 * 0.14f) + 6.1f));
                    loop {
                        let _e1844 = phi_72_;
                        let _e1846 = phi_73_;
                        let _e1848 = phi_74_;
                        let _e1850 = phi_75_;
                        local_49 = _e1848;
                        let _e1851 = (_e1844 < 4i);
                        if _e1851 {
                            let _e1854 = cantus_render_shader_simplex_noise(_e1850);
                            phi_76_ = (_e1844 + 1i);
                            phi_77_ = (_e1846 * 0.5f);
                            phi_78_ = (_e1848 + (_e1854 * _e1846));
                            phi_79_ = vec2<f32>(((_e1850.x * 1.6f) + (_e1850.y * 1.2f)), ((_e1850.y * 1.6f) - (_e1850.x * 1.2f)));
                        } else {
                            phi_76_ = i32();
                            phi_77_ = f32();
                            phi_78_ = f32();
                            phi_79_ = vec2<f32>();
                        }
                        let _e1867 = phi_76_;
                        let _e1869 = phi_77_;
                        let _e1871 = phi_78_;
                        let _e1873 = phi_79_;
                        continue;
                        continuing {
                            phi_72_ = _e1867;
                            phi_73_ = _e1869;
                            phi_74_ = _e1871;
                            phi_75_ = _e1873;
                            break if !(_e1851);
                        }
                    }
                    let _e1876 = local_49;
                    let _e1879 = (((_e1876 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e1881 = select(_e1879, 0f, (_e1879 < 0f));
                    let _e1883 = select(_e1881, 1f, (_e1881 > 1f));
                    phi_80_ = ((((_e1883 * _e1883) * (3f - (2f * _e1883))) * _e831) * 0.82f);
                } else {
                    phi_80_ = 0f;
                }
                let _e1891 = phi_80_;
                let _e1893 = ((_e219.y - -0.02f) * 16.666668f);
                let _e1895 = select(_e1893, 0f, (_e1893 < 0f));
                let _e1897 = select(_e1895, 1f, (_e1895 > 1f));
                let _e1904 = (_e236 - _e1808);
                let _e1905 = (_e237 - (_e235 * _e1814));
                let _e1909 = sqrt(((_e1904 * _e1904) + (_e1905 * _e1905)));
                let _e1911 = ((_e1909 - 62f) * -0.01724138f);
                let _e1913 = select(_e1911, 0f, (_e1911 < 0f));
                let _e1915 = select(_e1913, 1f, (_e1913 > 1f));
                let _e1922 = ((_e1909 - 11f) * -0.1f);
                let _e1924 = select(_e1922, 0f, (_e1922 < 0f));
                let _e1926 = select(_e1924, 1f, (_e1924 > 1f));
                let _e1933 = (((((_e1915 * _e1915) * (3f - (2f * _e1915))) * 0.24f) + (((_e1926 * _e1926) * (3f - (2f * _e1926))) * 0.7f)) * (((_e1897 * _e1897) * (3f - (2f * _e1897))) * (1f - _e1891)));
                let _e1934 = (1f - _e1933);
                let _e1947 = ((_e244 - 1f) / ((_e235 * -0.25f) - 1f));
                let _e1949 = select(_e1947, 0f, (_e1947 < 0f));
                let _e1951 = select(_e1949, 1f, (_e1949 > 1f));
                let _e1955 = ((_e1951 * _e1951) * (3f - (2f * _e1951)));
                let _e1956 = (1f - _e1955);
                phi_81_ = vec3<f32>(((_e1799 * _e1956) + (((_e1799 * _e1934) + (((0.96f * _e1826) + (0.98f * _e1825)) * _e1933)) * _e1955)), ((_e1801 * _e1956) + (((_e1801 * _e1934) + (((0.98f * _e1826) + (0.74f * _e1825)) * _e1933)) * _e1955)), ((_e1803 * _e1956) + (((_e1803 * _e1934) + ((_e1826 + (0.66f * _e1825)) * _e1933)) * _e1955)));
            } else {
                phi_81_ = (_e1774 + vec3(_e1797));
            }
            let _e1968 = phi_81_;
            let _e1979 = local_50;
            let _e1980 = (1f - _e1979);
            let _e1985 = local_51;
            let _e1988 = local_52;
            let _e1991 = local_53;
            if (_e218.y < _e283) {
                phi_93_ = 0u;
                phi_94_ = u32();
                phi_95_ = true;
            } else {
                let _e1997 = (_e218.x - (_e231 - 158f));
                if (_e1997 >= 316f) {
                    if (_e292 < ((_e238 + 96f) * 0.5f)) {
                        phi_89_ = 4u;
                    } else {
                        let _e2042 = (_e544 + _e545);
                        let _e2043 = (_e2042 + _e238);
                        if (_e292 > _e2043) {
                            let _e2059 = cantus_render_tempestas_cell_index(_e292, _e2043, 28f, 2f);
                            phi_88_ = ((76u + (_e2059 * 2u)) + select(0u, 1u, (_e292 > (_e2043 + (4f * (3.5f + (f32(_e2059) * 7f)))))));
                        } else {
                            let _e2045 = (_e292 > _e537);
                            let _e2046 = select(6u, 5u, _e2045);
                            let _e2053 = cantus_render_tempestas_cell_index(_e1997, 316f, (308f / f32(_e2046)), f32((_e2046 - 1u)));
                            phi_88_ = ((select(5u, 17u, _e2045) + (_e2053 * 2u)) + select(0u, 1u, (_e292 > select(_e544, _e2042, _e2045))));
                        }
                        let _e2071 = phi_88_;
                        phi_89_ = _e2071;
                    }
                    let _e2073 = phi_89_;
                    phi_90_ = _e2073;
                    phi_91_ = u32();
                    phi_92_ = true;
                } else {
                    if (_e292 < 54f) {
                        let _e2012 = ((_e543 - 0.5295082f) * 4.1666665f);
                        let _e2014 = select(_e2012, 0f, (_e2012 < 0f));
                        let _e2016 = select(_e2014, 1f, (_e2014 > 1f));
                        let _e2021 = (126f * ((_e2016 * _e2016) * (3f - (2f * _e2016))));
                        if (abs((_e1997 - (154f - _e2021))) < 20f) {
                            phi_84_ = 2u;
                        } else {
                            phi_84_ = select(1u, 3u, (abs((_e1997 - (154f + _e2021))) < 20f));
                        }
                        let _e2032 = phi_84_;
                        phi_85_ = _e2032;
                        phi_86_ = u32();
                        phi_87_ = true;
                    } else {
                        let _e2000 = cantus_render_tempestas_cell_index(_e1997, 0f, 44f, 6f);
                        let _e2001 = (_e292 < 82f);
                        if _e2001 {
                            phi_82_ = (27u + _e2000);
                            phi_83_ = u32();
                        } else {
                            let _e2002 = cantus_render_tempestas_cell_index(_e292, 84f, 24f, 5f);
                            phi_82_ = u32();
                            phi_83_ = ((34u + (_e2002 * 7u)) + _e2000);
                        }
                        let _e2008 = phi_82_;
                        let _e2010 = phi_83_;
                        phi_85_ = _e2008;
                        phi_86_ = _e2010;
                        phi_87_ = _e2001;
                    }
                    let _e2034 = phi_85_;
                    let _e2036 = phi_86_;
                    let _e2038 = phi_87_;
                    phi_90_ = _e2034;
                    phi_91_ = _e2036;
                    phi_92_ = _e2038;
                }
                let _e2075 = phi_90_;
                let _e2077 = phi_91_;
                let _e2079 = phi_92_;
                phi_93_ = _e2075;
                phi_94_ = _e2077;
                phi_95_ = _e2079;
            }
            let _e2081 = phi_93_;
            let _e2083 = phi_94_;
            let _e2085 = phi_95_;
            let _e2086 = select(_e2083, _e2081, _e2085);
            if (_e2086 < arrayLength((&text_lines.member))) {
            } else {
                break;
            }
            let _e2090 = text_lines.member[_e2086];
            let _e2092 = unpack4x8unorm(_e2090.color);
            if (_e218.x < _e2090.min.x) {
                phi_124_ = f32();
                phi_125_ = true;
            } else {
                if (_e218.x > _e2090.max.x) {
                    phi_122_ = f32();
                    phi_123_ = true;
                } else {
                    if (_e218.y < _e2090.min.y) {
                        phi_120_ = f32();
                        phi_121_ = true;
                    } else {
                        let _e2104 = (_e218.y > _e2090.max.y);
                        if _e2104 {
                            phi_119_ = f32();
                        } else {
                            let _e2106 = (1f / _e2090.size);
                            let _e2113 = ((_e218.x - _e2090.origin.x) * _e2106);
                            phi_96_ = 0u;
                            phi_97_ = _e2090.count;
                            loop {
                                let _e2118 = phi_96_;
                                let _e2120 = phi_97_;
                                local_54 = _e2118;
                                let _e2121 = (_e2118 < _e2120);
                                if _e2121 {
                                    let _e2124 = (_e2118 + ((_e2120 - _e2118) / 2u));
                                    let _e2129 = placed_glyphs_2.member[(_e2090.first + _e2124)].x;
                                    let _e2130 = (_e2129 <= _e2113);
                                    if _e2130 {
                                        phi_98_ = (_e2124 + 1u);
                                    } else {
                                        phi_98_ = _e2118;
                                    }
                                    let _e2133 = phi_98_;
                                    phi_99_ = _e2133;
                                    phi_100_ = select(_e2124, _e2120, _e2130);
                                } else {
                                    phi_99_ = u32();
                                    phi_100_ = u32();
                                }
                                let _e2136 = phi_99_;
                                let _e2138 = phi_100_;
                                continue;
                                continuing {
                                    phi_96_ = _e2136;
                                    phi_97_ = _e2138;
                                    break if !(_e2121);
                                }
                            }
                            let _e2140 = (3.5f / _e2090.size);
                            let _e2142 = local_54;
                            let _e2143 = (_e2142 + 1u);
                            phi_101_ = select(_e2143, _e2090.count, (_e2090.count < _e2143));
                            phi_102_ = -1000000f;
                            loop {
                                let _e2147 = phi_101_;
                                let _e2149 = phi_102_;
                                local_57 = _e2149;
                                if (_e2147 > 0u) {
                                    let _e2151 = (_e2147 - 1u);
                                    let _e2152 = (_e2090.first + _e2151);
                                    let _e2156 = placed_glyphs_2.member[_e2152].x;
                                    let _e2160 = placed_glyphs_2.member[_e2152].glyph;
                                    let _e2165 = glyphs_2.member[_e2160].min[0u];
                                    let _e2170 = glyphs_2.member[_e2160].min[1u];
                                    let _e2175 = glyphs_2.member[_e2160].max[0u];
                                    let _e2180 = glyphs_2.member[_e2160].max[1u];
                                    let _e2184 = glyphs_2.member[_e2160].start;
                                    let _e2188 = glyphs_2.member[_e2160].count;
                                    let _e2189 = (_e2113 - _e2156);
                                    let _e2190 = -(((_e218.y - _e2090.origin.y) * _e2106));
                                    let _e2191 = (_e2175 + _e2140);
                                    let _e2192 = (_e2189 > _e2191);
                                    if _e2192 {
                                        phi_115_ = f32();
                                    } else {
                                        if (_e2189 >= (_e2165 - _e2140)) {
                                            if (_e2190 >= (_e2170 - _e2140)) {
                                                if (_e2189 <= _e2191) {
                                                    if (_e2190 <= (_e2180 + _e2140)) {
                                                        phi_103_ = 340282350000000000000000000000000000000f;
                                                        phi_104_ = 0u;
                                                        phi_105_ = 0i;
                                                        loop {
                                                            let _e2202 = phi_103_;
                                                            let _e2204 = phi_104_;
                                                            let _e2206 = phi_105_;
                                                            local_55 = _e2202;
                                                            local_56 = _e2206;
                                                            let _e2207 = (_e2204 < _e2188);
                                                            if _e2207 {
                                                                let _e2211 = edges_2.member[(_e2184 + _e2204)];
                                                                let _e2213 = cantus_render_text_edge_distance(_e2211, _e2090.weight, vec2<f32>(_e2189, _e2190), _e2202);
                                                                phi_106_ = _e2213.member;
                                                                phi_107_ = (_e2204 + 1u);
                                                                phi_108_ = (_e2206 + _e2213.member_1);
                                                            } else {
                                                                phi_106_ = f32();
                                                                phi_107_ = u32();
                                                                phi_108_ = i32();
                                                            }
                                                            let _e2219 = phi_106_;
                                                            let _e2221 = phi_107_;
                                                            let _e2223 = phi_108_;
                                                            continue;
                                                            continuing {
                                                                phi_103_ = _e2219;
                                                                phi_104_ = _e2221;
                                                                phi_105_ = _e2223;
                                                                break if !(_e2207);
                                                            }
                                                        }
                                                        let _e2226 = local_55;
                                                        let _e2228 = ((_e2226 * _e2090.size) * _e2090.size);
                                                        if (_e2228 >= 12.25f) {
                                                            phi_109_ = 3.5f;
                                                        } else {
                                                            phi_109_ = sqrt(_e2228);
                                                        }
                                                        let _e2232 = phi_109_;
                                                        let _e2234 = local_56;
                                                        let _e2237 = (_e2232 * select(1f, -1f, (_e2234 == 0i)));
                                                        if (_e2149 != _e2149) {
                                                            phi_110_ = true;
                                                        } else {
                                                            phi_110_ = (_e2237 >= _e2149);
                                                        }
                                                        let _e2241 = phi_110_;
                                                        phi_111_ = select(_e2149, _e2237, _e2241);
                                                    } else {
                                                        phi_111_ = _e2149;
                                                    }
                                                    let _e2244 = phi_111_;
                                                    phi_112_ = _e2244;
                                                } else {
                                                    phi_112_ = _e2149;
                                                }
                                                let _e2246 = phi_112_;
                                                phi_113_ = _e2246;
                                            } else {
                                                phi_113_ = _e2149;
                                            }
                                            let _e2248 = phi_113_;
                                            phi_114_ = _e2248;
                                        } else {
                                            phi_114_ = _e2149;
                                        }
                                        let _e2250 = phi_114_;
                                        phi_115_ = _e2250;
                                    }
                                    let _e2252 = phi_115_;
                                    phi_116_ = _e2151;
                                    phi_117_ = _e2252;
                                    phi_118_ = select(true, false, _e2192);
                                } else {
                                    phi_116_ = u32();
                                    phi_117_ = f32();
                                    phi_118_ = false;
                                }
                                let _e2255 = phi_116_;
                                let _e2257 = phi_117_;
                                let _e2259 = phi_118_;
                                continue;
                                continuing {
                                    phi_101_ = _e2255;
                                    phi_102_ = _e2257;
                                    break if !(_e2259);
                                }
                            }
                            let _e2442 = local_57;
                            phi_119_ = _e2442;
                        }
                        let _e2262 = phi_119_;
                        phi_120_ = _e2262;
                        phi_121_ = _e2104;
                    }
                    let _e2264 = phi_120_;
                    let _e2266 = phi_121_;
                    phi_122_ = _e2264;
                    phi_123_ = _e2266;
                }
                let _e2268 = phi_122_;
                let _e2270 = phi_123_;
                phi_124_ = _e2268;
                phi_125_ = _e2270;
            }
            let _e2272 = phi_124_;
            let _e2274 = phi_125_;
            let _e2277 = ((select(_e2272, -1000000f, _e2274) * 1.25f) + 0.5f);
            let _e2279 = select(_e2277, 0f, (_e2277 < 0f));
            let _e2281 = select(_e2279, 1f, (_e2279 > 1f));
            let _e2287 = (((_e2281 * _e2281) * (3f - (2f * _e2281))) * _e2092.w);
            let _e2288 = (1f - _e2287);
            out_color = vec4<f32>((((((_e1968.x * _e1980) + (((_e1968.x * 1.5f) + 0.1f) * _e1985)) * _e2288) + (_e2092.x * _e2287)) * _e780), (((((_e1968.y * _e1980) + (((_e1968.y * 1.5f) + 0.1f) * _e1988)) * _e2288) + (_e2092.y * _e2287)) * _e780), (((((_e1968.z * _e1980) + (((_e1968.z * 1.5f) + 0.1f) * _e1991)) * _e2288) + (_e2092.z * _e2287)) * _e780), _e793);
            break;
        }
    }
    return;
}

@vertex
fn render_track_isthmus_trackpass_vertex(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    vertex_6 = vertex;
    instance_1 = instance;
    render_track_isthmus_trackpass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel_pos;
    let _e9 = out_pill_idx;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_track_isthmus_trackpass_fragment(@location(0) pixel_pos: vec2<f32>, @location(1) @interpolate(flat) pill_idx: u32) -> @location(0) vec4<f32> {
    pixel_pos_1 = pixel_pos;
    pill_idx_1 = pill_idx;
    render_track_isthmus_trackpass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_lyrics_isthmus_lyricspass_vertex(@builtin(vertex_index) vertex_1: u32, @builtin(instance_index) _isthmus_instance_index: u32) -> VertexOutput {
    vertex_6 = vertex_1;
    _isthmus_instance_index_9 = _isthmus_instance_index;
    render_lyrics_isthmus_lyricspass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_lyrics_isthmus_lyricspass_fragment(@location(0) pixel: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_1: u32) -> @location(0) vec4<f32> {
    pixel_3 = pixel;
    _isthmus_instance_index_10 = _isthmus_instance_index_1;
    render_lyrics_isthmus_lyricspass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_status_isthmus_statuspass_vertex(@builtin(vertex_index) vertex_2: u32, @builtin(instance_index) _isthmus_instance_index_2: u32) -> VertexOutput {
    vertex_6 = vertex_2;
    _isthmus_instance_index_9 = _isthmus_instance_index_2;
    render_status_isthmus_statuspass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_status_isthmus_statuspass_fragment(@location(0) pixel_1: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_3: u32) -> @location(0) vec4<f32> {
    pixel_3 = pixel_1;
    _isthmus_instance_index_10 = _isthmus_instance_index_3;
    render_status_isthmus_statuspass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_playhead_isthmus_playheadpass_vertex(@builtin(vertex_index) vertex_3: u32, @builtin(instance_index) _isthmus_instance_index_4: u32) -> VertexOutput {
    vertex_6 = vertex_3;
    _isthmus_instance_index_9 = _isthmus_instance_index_4;
    render_playhead_isthmus_playheadpass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_world_pos;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_playhead_isthmus_playheadpass_fragment(@location(0) world_pos: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_5: u32) -> @location(0) vec4<f32> {
    world_pos_1 = world_pos;
    _isthmus_instance_index_10 = _isthmus_instance_index_5;
    render_playhead_isthmus_playheadpass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_particles_isthmus_particlepass_vertex(@builtin(vertex_index) vertex_4: u32, @builtin(instance_index) _isthmus_instance_index_6: u32) -> VertexOutput_1 {
    vertex_6 = vertex_4;
    _isthmus_instance_index_9 = _isthmus_instance_index_6;
    render_particles_isthmus_particlepass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_color;
    let _e9 = out_uv;
    return VertexOutput_1(_e7, _e8, _e9);
}

@fragment
fn render_particles_isthmus_particlepass_fragment(@location(0) color: vec4<f32>, @location(1) uv: vec2<f32>) -> @location(0) vec4<f32> {
    color_1 = color;
    uv_1 = uv;
    render_particles_isthmus_particlepass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_tempestas_isthmus_tempestaspass_vertex(@builtin(vertex_index) vertex_5: u32, @builtin(instance_index) _isthmus_instance_index_7: u32) -> VertexOutput_2 {
    vertex_6 = vertex_5;
    _isthmus_instance_index_9 = _isthmus_instance_index_7;
    render_tempestas_isthmus_tempestaspass_vertex_impl();
    let _e8 = out_position;
    let _e9 = out_pixel;
    let _e10 = out_weather;
    let _e11 = out_isthmus_instance_index_1;
    return VertexOutput_2(_e8, _e9, _e10, _e11);
}

@fragment
fn render_tempestas_isthmus_tempestaspass_fragment(@location(0) pixel_2: vec2<f32>, @location(1) @interpolate(flat) weather: vec4<f32>, @location(2) @interpolate(flat) _isthmus_instance_index_8: u32) -> @location(0) vec4<f32> {
    pixel_3 = pixel_2;
    weather_1 = weather;
    _isthmus_instance_index_11 = _isthmus_instance_index_8;
    render_tempestas_isthmus_tempestaspass_fragment_impl();
    let _e7 = out_color;
    return _e7;
}
