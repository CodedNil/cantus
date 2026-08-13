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

struct type_26 {
    member: array<render_status_StatusPill>,
}

struct isthmus_Vertex_render_text_Varyings {
    position: vec4<f32>,
    varyings: vec2<f32>,
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

struct type_36 {
    member: array<render_tempestas_WeatherSurface>,
}

struct type_38 {
    member: array<render_text_Line>,
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
var<storage> pill_1: type_26;
var<private> _isthmus_instance_index_7: u32;
var<private> out_pixel: vec2<f32>;
var<private> out_isthmus_instance_index: u32;
var<private> pixel_2: vec2<f32>;
var<private> _isthmus_instance_index_8: u32;
@group(0) @binding(2)
var<storage> placed_glyphs_1: type_15;
@group(0) @binding(3)
var<storage> glyphs_1: type_17;
@group(0) @binding(4)
var<storage> edges_1: type_19;
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
var<storage> pill_2: type_36;
var<private> out_weather: vec4<f32>;
var<private> out_isthmus_instance_index_1: u32;
var<private> weather_1: vec4<f32>;
var<private> _isthmus_instance_index_9: u32;
@group(0) @binding(2)
var<storage> text_lines: type_38;
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

    let _e31 = (param_11.start.x + (param_11.start_delta.x * param_12));
    let _e32 = (param_11.start.y + (param_11.start_delta.y * param_12));
    let _e43 = (param_11.control.x + (param_11.control_delta.x * param_12));
    let _e44 = (param_11.control.y + (param_11.control_delta.y * param_12));
    let _e55 = (param_11.end.x + (param_11.end_delta.x * param_12));
    let _e56 = (param_11.end.y + (param_11.end_delta.y * param_12));
    let _e58 = (_e44 - _e32);
    let _e59 = ((_e43 - _e31) * 2f);
    let _e60 = (_e58 * 2f);
    let _e65 = ((_e31 - (_e43 * 2f)) + _e55);
    let _e66 = ((_e32 - (_e44 * 2f)) + _e56);
    let _e68 = select(_e43, _e31, (_e31 < _e43));
    let _e70 = select(_e44, _e32, (_e32 < _e44));
    let _e72 = select(_e55, _e68, (_e68 < _e55));
    let _e74 = select(_e56, _e70, (_e70 < _e56));
    let _e76 = select(_e43, _e31, (_e31 > _e43));
    let _e78 = select(_e44, _e32, (_e32 > _e44));
    let _e80 = select(_e55, _e76, (_e76 > _e55));
    let _e82 = select(_e56, _e78, (_e78 > _e56));
    if (param_13.x >= _e80) {
        phi_8_ = i32();
        phi_9_ = true;
    } else {
        if (param_13.y < _e74) {
            phi_6_ = i32();
            phi_7_ = true;
        } else {
            let _e85 = (param_13.y >= _e82);
            if _e85 {
                phi_5_ = i32();
            } else {
                let _e86 = (_e32 - param_13.y);
                if (abs(_e66) < 0.0000001f) {
                    if (abs(_e60) < 0.0000001f) {
                        phi_1_ = 0i;
                    } else {
                        let _e116 = cantus_render_text_ray_crossing(vec2<f32>(_e31, _e32), vec2<f32>(_e59, _e60), vec2<f32>(_e65, _e66), param_13, (-(_e86) / _e60));
                        phi_1_ = _e116;
                    }
                    let _e118 = phi_1_;
                    phi_2_ = _e118;
                    phi_3_ = i32();
                    phi_4_ = true;
                } else {
                    let _e92 = ((_e60 * _e60) - ((4f * _e66) * _e86));
                    let _e93 = (_e92 <= 0f);
                    if _e93 {
                        phi_0_ = i32();
                    } else {
                        let _e94 = sqrt(_e92);
                        let _e95 = (_e66 * 2f);
                        let _e96 = (_e58 * -2f);
                        let _e99 = vec2<f32>(_e31, _e32);
                        let _e100 = vec2<f32>(_e59, _e60);
                        let _e101 = vec2<f32>(_e65, _e66);
                        let _e102 = cantus_render_text_ray_crossing(_e99, _e100, _e101, param_13, ((_e96 - _e94) / _e95));
                        let _e105 = cantus_render_text_ray_crossing(_e99, _e100, _e101, param_13, ((_e96 + _e94) / _e95));
                        phi_0_ = (_e102 + _e105);
                    }
                    let _e108 = phi_0_;
                    phi_2_ = 0i;
                    phi_3_ = _e108;
                    phi_4_ = _e93;
                }
                let _e120 = phi_2_;
                let _e122 = phi_3_;
                let _e124 = phi_4_;
                phi_5_ = select(_e122, _e120, _e124);
            }
            let _e127 = phi_5_;
            phi_6_ = _e127;
            phi_7_ = _e85;
        }
        let _e129 = phi_6_;
        let _e131 = phi_7_;
        phi_8_ = _e129;
        phi_9_ = _e131;
    }
    let _e133 = phi_8_;
    let _e135 = phi_9_;
    let _e136 = select(_e133, 0i, _e135);
    let _e138 = select(_e72, param_13.x, (param_13.x > _e72));
    let _e140 = select(_e74, param_13.y, (param_13.y > _e74));
    let _e145 = (param_13.x - select(_e80, _e138, (_e138 < _e80)));
    let _e146 = (param_13.y - select(_e82, _e140, (_e140 < _e82)));
    if (((_e145 * _e145) + (_e146 * _e146)) >= param_14) {
        phi_22_ = u0028_f32_u0020_i32_u0029_(param_14, _e136);
    } else {
        let _e151 = (_e55 - _e31);
        let _e152 = (_e56 - _e32);
        let _e153 = (param_13.x - _e31);
        let _e154 = (param_13.y - _e32);
        let _e160 = ((_e151 * _e151) + (_e152 * _e152));
        if (_e160 != _e160) {
            phi_10_ = true;
        } else {
            phi_10_ = (0.00000001f >= _e160);
        }
        let _e164 = phi_10_;
        let _e166 = (((_e153 * _e151) + (_e154 * _e152)) / select(_e160, 0.00000001f, _e164));
        if (_e166 != _e166) {
            phi_11_ = true;
        } else {
            phi_11_ = (0f >= _e166);
        }
        let _e170 = phi_11_;
        let _e171 = select(_e166, 0f, _e170);
        if (_e171 != _e171) {
            phi_12_ = true;
        } else {
            phi_12_ = (1f <= _e171);
        }
        let _e175 = phi_12_;
        let _e177 = (_e65 * 2f);
        let _e178 = (_e66 * 2f);
        phi_13_ = select(_e171, 1f, _e175);
        phi_14_ = 0i;
        loop {
            let _e180 = phi_13_;
            let _e182 = phi_14_;
            local_1 = _e180;
            let _e183 = (_e182 < 2i);
            if _e183 {
                let _e187 = cantus_render_text_curve_at(vec2<f32>(_e31, _e32), vec2<f32>(_e59, _e60), vec2<f32>(_e65, _e66), _e180);
                let _e192 = (_e59 + (_e177 * _e180));
                let _e193 = (_e60 + (_e178 * _e180));
                let _e194 = (_e187.x - param_13.x);
                let _e195 = (_e187.y - param_13.y);
                let _e202 = (((_e192 * _e192) + (_e193 * _e193)) + ((_e194 * _e177) + (_e195 * _e178)));
                let _e203 = abs(_e202);
                if (_e203 != _e203) {
                    phi_15_ = true;
                } else {
                    phi_15_ = (0.00000001f >= _e203);
                }
                let _e207 = phi_15_;
                let _e219 = (_e180 - (((_e194 * _e192) + (_e195 * _e193)) / bitcast<f32>(((bitcast<u32>(select(_e203, 0.00000001f, _e207)) & 2147483647u) | (bitcast<u32>(_e202) & 2147483648u)))));
                if (_e219 != _e219) {
                    phi_16_ = true;
                } else {
                    phi_16_ = (0f >= _e219);
                }
                let _e223 = phi_16_;
                let _e224 = select(_e219, 0f, _e223);
                if (_e224 != _e224) {
                    phi_17_ = true;
                } else {
                    phi_17_ = (1f <= _e224);
                }
                let _e228 = phi_17_;
                phi_18_ = select(_e224, 1f, _e228);
                phi_19_ = (_e182 + 1i);
            } else {
                phi_18_ = f32();
                phi_19_ = i32();
            }
            let _e232 = phi_18_;
            let _e234 = phi_19_;
            continue;
            continuing {
                phi_13_ = _e232;
                phi_14_ = _e234;
                break if !(_e183);
            }
        }
        let _e238 = ((_e153 * _e153) + (_e154 * _e154));
        let _e239 = (param_13.x - _e55);
        let _e240 = (param_13.y - _e56);
        let _e243 = ((_e239 * _e239) + (_e240 * _e240));
        if (_e238 != _e238) {
            phi_20_ = true;
        } else {
            phi_20_ = (_e243 <= _e238);
        }
        let _e247 = phi_20_;
        let _e248 = select(_e238, _e243, _e247);
        let _e253 = local_1;
        let _e254 = cantus_render_text_curve_at(vec2<f32>(_e31, _e32), vec2<f32>(_e59, _e60), vec2<f32>(_e65, _e66), _e253);
        let _e257 = (param_13.x - _e254.x);
        let _e258 = (param_13.y - _e254.y);
        let _e261 = ((_e257 * _e257) + (_e258 * _e258));
        if (_e248 != _e248) {
            phi_21_ = true;
        } else {
            phi_21_ = (_e261 <= _e248);
        }
        let _e265 = phi_21_;
        phi_22_ = u0028_f32_u0020_i32_u0029_(select(_e248, _e261, _e265), _e136);
    }
    let _e270 = phi_22_;
    return _e270;
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
    var phi_68_: bool;
    var local_10: u32;
    var phi_69_: bool;
    var phi_70_: u32;
    var phi_71_: f32;
    var phi_72_: f32;
    var phi_73_: u32;
    var phi_74_: i32;
    var phi_75_: f32;
    var phi_76_: u32;
    var phi_77_: i32;
    var phi_78_: bool;
    var local_11: f32;
    var local_12: i32;
    var phi_79_: bool;
    var phi_80_: bool;
    var phi_81_: f32;
    var phi_82_: bool;
    var phi_83_: f32;
    var phi_84_: bool;
    var phi_85_: f32;
    var phi_86_: bool;
    var phi_87_: f32;
    var phi_88_: bool;
    var phi_89_: f32;
    var phi_90_: bool;
    var phi_91_: u32;
    var phi_92_: f32;
    var phi_93_: bool;
    var phi_94_: bool;
    var local_13: f32;
    var phi_95_: bool;
    var phi_96_: f32;
    var phi_97_: bool;
    var phi_98_: f32;
    var phi_99_: bool;
    var phi_100_: bool;
    var phi_101_: f32;
    var phi_102_: bool;
    var phi_103_: bool;
    var phi_104_: f32;
    var phi_105_: bool;
    var phi_106_: u32;
    var phi_107_: u32;
    var phi_108_: u32;
    var phi_109_: u32;
    var phi_110_: u32;
    var phi_111_: bool;
    var local_14: u32;
    var phi_112_: bool;
    var phi_113_: u32;
    var phi_114_: f32;
    var phi_115_: f32;
    var phi_116_: u32;
    var phi_117_: i32;
    var phi_118_: f32;
    var phi_119_: u32;
    var phi_120_: i32;
    var phi_121_: bool;
    var local_15: f32;
    var local_16: i32;
    var phi_122_: bool;
    var phi_123_: bool;
    var phi_124_: f32;
    var phi_125_: bool;
    var phi_126_: f32;
    var phi_127_: bool;
    var phi_128_: f32;
    var phi_129_: bool;
    var phi_130_: f32;
    var phi_131_: bool;
    var phi_132_: f32;
    var phi_133_: bool;
    var phi_134_: u32;
    var phi_135_: f32;
    var phi_136_: bool;
    var phi_137_: bool;
    var local_17: f32;
    var phi_138_: f32;
    var phi_139_: f32;
    var phi_140_: bool;
    var phi_141_: f32;
    var phi_142_: bool;
    var phi_143_: f32;
    var phi_144_: bool;
    var phi_145_: bool;
    var local_18: vec4<f32>;
    var local_19: vec4<f32>;
    var local_20: vec4<f32>;
    var local_21: vec4<f32>;
    var local_22: vec4<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e165 = pixel_pos_1;
            let _e166 = pill_idx_1;
            let _e168 = arrayLength((&placed_glyphs.member));
            let _e170 = arrayLength((&glyphs.member));
            let _e172 = arrayLength((&edges.member));
            let _e178 = pill.member[_e166].x;
            let _e182 = pill.member[_e166].width;
            let _e186 = frame.member[0u].panel_height;
            let _e187 = (_e165.x - _e178);
            let _e188 = (_e165.y - 6f);
            let _e189 = (_e182 * 0.5f);
            let _e190 = (_e186 * 0.5f);
            let _e192 = (_e188 - _e190);
            let _e193 = (_e182 - _e186);
            let _e194 = (_e193 * 0.5f);
            let _e196 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e187 - _e189), _e192), _e194, _e190);
            let _e200 = frame.member[0u].mouse_pressure;
            let _e201 = (_e200 > 0f);
            if _e201 {
                let _e206 = frame.member[0u].mouse_pos[0u];
                let _e211 = frame.member[0u].mouse_pos[1u];
                let _e217 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e206 - _e178) - _e189), ((_e211 - 6f) - _e190)), _e194, _e190);
                phi_0_ = _e217;
            } else {
                phi_0_ = 1f;
            }
            let _e219 = phi_0_;
            phi_1_ = vec2<f32>(0f, 0f);
            phi_2_ = 0f;
            phi_3_ = 0u;
            loop {
                let _e221 = phi_1_;
                let _e223 = phi_2_;
                let _e225 = phi_3_;
                local_2 = _e221;
                local_3 = _e221;
                local_4 = _e221;
                local_5 = _e221;
                local_6 = _e223;
                local_7 = _e223;
                local_8 = _e223;
                local_9 = _e223;
                let _e226 = (_e225 < 4u);
                if _e226 {
                    if _e226 {
                    } else {
                        phi_13_ = true;
                        break;
                    }
                    let _e233 = frame.member[0u].ripples[_e225].origin[0u];
                    let _e240 = frame.member[0u].ripples[_e225].origin[1u];
                    let _e246 = frame.member[0u].ripples[_e225].start_time;
                    let _e252 = frame.member[0u].ripples[_e225].strength;
                    let _e256 = frame.member[0u].time;
                    let _e258 = ((_e256 - _e246) * 1.2f);
                    let _e260 = select(_e258, 0f, (_e258 < 0f));
                    let _e262 = select(_e260, 1f, (_e260 > 1f));
                    if (_e252 > 0f) {
                        if (_e262 < 1f) {
                            let _e265 = (_e165.x - _e233);
                            let _e266 = (_e165.y - _e240);
                            let _e270 = sqrt(((_e265 * _e265) + (_e266 * _e266)));
                            if (_e270 > 0.001f) {
                                phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e265 / _e270), (_e266 / _e270)), _e270);
                            } else {
                                phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e270);
                            }
                            let _e278 = phi_4_;
                            let _e288 = ((abs((_e278.unnamed_1 - (_e262 * 600f))) - 80f) * -0.0125f);
                            let _e290 = select(_e288, 0f, (_e288 < 0f));
                            let _e292 = select(_e290, 1f, (_e290 > 1f));
                            let _e298 = (1f - _e262);
                            let _e299 = ((((_e292 * _e292) * (3f - (2f * _e292))) * _e252) * _e298);
                            let _e312 = (_e223 + (_e299 * 0.5f));
                            if (_e312 != _e312) {
                                phi_5_ = true;
                            } else {
                                phi_5_ = (1f <= _e312);
                            }
                            let _e316 = phi_5_;
                            phi_6_ = vec2<f32>((_e221.x + (((_e278.unnamed.x * _e299) * _e298) * 0.5f)), (_e221.y + (((_e278.unnamed.y * _e299) * _e298) * 0.5f)));
                            phi_7_ = select(_e312, 1f, _e316);
                        } else {
                            phi_6_ = _e221;
                            phi_7_ = _e223;
                        }
                        let _e319 = phi_6_;
                        let _e321 = phi_7_;
                        phi_8_ = _e319;
                        phi_9_ = _e321;
                    } else {
                        phi_8_ = _e221;
                        phi_9_ = _e223;
                    }
                    let _e323 = phi_8_;
                    let _e325 = phi_9_;
                    phi_10_ = _e323;
                    phi_11_ = _e325;
                    phi_12_ = (_e225 + 1u);
                } else {
                    phi_10_ = vec2<f32>();
                    phi_11_ = f32();
                    phi_12_ = u32();
                }
                let _e328 = phi_10_;
                let _e330 = phi_11_;
                let _e332 = phi_12_;
                continue;
                continuing {
                    phi_1_ = _e328;
                    phi_2_ = _e330;
                    phi_3_ = _e332;
                    phi_13_ = false;
                    break if !(_e226);
                }
            }
            let _e335 = phi_13_;
            if _e335 {
                break;
            }
            if _e201 {
                let _e340 = frame.member[0u].mouse_pos[0u];
                let _e345 = frame.member[0u].mouse_pos[1u];
                let _e346 = (_e165.x - _e340);
                let _e347 = (_e165.y - _e345);
                let _e353 = ((sqrt(((_e346 * _e346) + (_e347 * _e347))) - 150f) * -0.006666667f);
                let _e355 = select(_e353, 0f, (_e353 < 0f));
                let _e357 = select(_e355, 1f, (_e355 > 1f));
                phi_14_ = ((((_e357 * _e357) * (3f - (2f * _e357))) * _e200) * 8f);
            } else {
                phi_14_ = 0f;
            }
            let _e365 = phi_14_;
            let _e367 = local_2;
            let _e370 = global[0u];
            if (_e367.x == _e370) {
                let _e373 = local_3;
                let _e376 = global[1u];
                phi_15_ = (_e373.y == _e376);
            } else {
                phi_15_ = false;
            }
            let _e379 = phi_15_;
            if _e379 {
                phi_16_ = 0f;
            } else {
                let _e381 = local_4;
                phi_16_ = (sqrt(((_e367.x * _e367.x) + (_e381.y * _e381.y))) * 22f);
            }
            let _e389 = phi_16_;
            let _e391 = local_5;
            let _e393 = (_e187 / _e182);
            let _e394 = (_e188 / _e186);
            let _e395 = (_e393 - 0.5f);
            let _e396 = (_e394 - 0.5f);
            let _e397 = (_e178 + _e189);
            let _e398 = (_e186 * 0.975f);
            let _e399 = (_e398 + 3f);
            let _e403 = pill.member[_e166].rating;
            let _e404 = (_e403 >= 0i);
            let _e405 = select(0f, 5f, _e404);
            let _e409 = pill.member[_e166].primary_playlist_count;
            let _e411 = (_e405 + f32(_e409));
            let _e417 = pill.member[_e166].secondary_expansion;
            let _e419 = (_e399 + (18f * _e417));
            let _e423 = pill.member[_e166].secondary_playlist_count;
            let _e424 = f32(_e423);
            let _e431 = frame.member[0u].mouse_pos[0u];
            let _e436 = frame.member[0u].mouse_pos[1u];
            let _e437 = vec2<f32>(_e431, _e436);
            let _e439 = (_e411 - 1f);
            let _e440 = (_e439 != _e439);
            if _e440 {
                phi_17_ = true;
            } else {
                phi_17_ = (0f >= _e439);
            }
            let _e443 = phi_17_;
            let _e446 = vec2<f32>(_e397, (_e398 + -4.4f));
            let _e448 = cantus_render_shader_sd_capsule_box((_e165 - _e446), (select(_e439, 0f, _e443) * 9f), 9f);
            if _e440 {
                phi_18_ = true;
            } else {
                phi_18_ = (0f >= _e439);
            }
            let _e451 = phi_18_;
            let _e455 = cantus_render_shader_sd_capsule_box((_e437 - _e446), (select(_e439, 0f, _e451) * 9f), 9f);
            let _e456 = (10.5f * _e417);
            let _e458 = (_e424 - 1f);
            let _e459 = (_e458 != _e458);
            if _e459 {
                phi_19_ = true;
            } else {
                phi_19_ = (0f >= _e458);
            }
            let _e462 = phi_19_;
            let _e467 = vec2<f32>(_e397, (_e419 + -5.4f));
            let _e469 = cantus_render_shader_sd_capsule_box((_e165 - _e467), (((select(_e458, 0f, _e462) * 18f) * _e417) * 0.5f), _e456);
            if _e459 {
                phi_20_ = true;
            } else {
                phi_20_ = (0f >= _e458);
            }
            let _e472 = phi_20_;
            let _e478 = cantus_render_shader_sd_capsule_box((_e437 - _e467), (((select(_e458, 0f, _e472) * 18f) * _e417) * 0.5f), _e456);
            let _e482 = pill.member[_e166].primary_alpha;
            let _e485 = (0.5f + ((_e448 - _e196) * 0.05f));
            let _e487 = select(_e485, 0f, (_e485 < 0f));
            let _e489 = select(_e487, 1f, (_e487 > 1f));
            let _e499 = (_e196 + ((((_e448 + ((_e196 - _e448) * _e489)) - ((10f * _e489) * (1f - _e489))) - _e196) * _e482));
            let _e502 = (0.5f + ((_e455 - _e219) * 0.05f));
            let _e504 = select(_e502, 0f, (_e502 < 0f));
            let _e506 = select(_e504, 1f, (_e504 > 1f));
            let _e516 = (_e219 + ((((_e455 + ((_e219 - _e455) * _e506)) - ((10f * _e506) * (1f - _e506))) - _e219) * _e482));
            let _e518 = select(0f, 1f, (_e417 > 0f));
            let _e521 = (0.5f + ((_e469 - _e499) * 0.046296295f));
            let _e523 = select(_e521, 0f, (_e521 < 0f));
            let _e525 = select(_e523, 1f, (_e523 > 1f));
            let _e538 = (0.5f + ((_e478 - _e516) * 0.046296295f));
            let _e540 = select(_e538, 0f, (_e538 < 0f));
            let _e542 = select(_e540, 1f, (_e540 > 1f));
            let _e554 = (((_e516 + ((((_e478 + ((_e516 - _e478) * _e542)) - ((10.8f * _e542) * (1f - _e542))) - _e516) * _e518)) - 0.5f) * -1f);
            let _e556 = select(_e554, 0f, (_e554 < 0f));
            let _e558 = select(_e556, 1f, (_e556 > 1f));
            let _e565 = (((_e365 * ((_e558 * _e558) * (3f - (2f * _e558)))) + _e389) * 0.5f);
            let _e566 = ((_e499 + ((((_e469 + ((_e499 - _e469) * _e525)) - ((10.8f * _e525) * (1f - _e525))) - _e499) * _e518)) - _e565);
            let _e567 = fwidth(_e566);
            if (_e567 != _e567) {
                phi_21_ = true;
            } else {
                phi_21_ = (0.55f >= _e567);
            }
            let _e571 = phi_21_;
            let _e572 = select(_e567, 0.55f, _e571);
            let _e576 = ((_e566 - _e572) / (-(_e572) - _e572));
            let _e578 = select(_e576, 0f, (_e576 < 0f));
            let _e580 = select(_e578, 1f, (_e578 > 1f));
            let _e584 = ((_e580 * _e580) * (3f - (2f * _e580)));
            let _e585 = (_e566 != _e566);
            if _e585 {
                phi_22_ = true;
            } else {
                phi_22_ = (0f >= _e566);
            }
            let _e588 = phi_22_;
            let _e592 = (exp((select(_e566, 0f, _e588) * -0.3f)) * 0.16f);
            if (_e584 != _e584) {
                phi_23_ = true;
            } else {
                phi_23_ = (_e592 >= _e584);
            }
            let _e596 = phi_23_;
            let _e597 = select(_e584, _e592, _e596);
            let _e601 = pill.member[_e166].visibility;
            if ((_e597 * _e601) <= 0.0009765625f) {
                discard;
            }
            if _e585 {
                phi_24_ = true;
            } else {
                phi_24_ = (0f <= _e566);
            }
            let _e606 = phi_24_;
            let _e609 = (1f + (select(_e566, 0f, _e606) * 0.008333334f));
            let _e611 = select(_e609, 0f, (_e609 < 0f));
            let _e613 = select(_e611, 0.6f, (_e611 > 0.6f));
            let _e623 = ((_e394 - ((_e396 * _e613) * 0.08f)) - (_e391.y * 0.04f));
            let _e624 = (((_e393 - ((_e395 * _e613) * 0.08f)) - (_e367.x * 0.04f)) * _e182);
            let _e625 = (_e623 * _e186);
            let _e629 = pill.member[_e166].effects;
            let _e633 = frame.member[0u].time;
            let _e637 = pill.member[_e166].seed;
            let _e640 = ((_e629.tempo - 0.2f) * 2.5f);
            let _e642 = select(_e640, 0f, (_e640 < 0f));
            let _e651 = ((_e633 * ((0.12f + (_e629.energy * 0.25f)) + (select(_e642, 1f, (_e642 > 1f)) * 0.12f))) + _e637);
            let _e656 = ((sin(((_e633 * _e629.tempo) * 31.415928f)) * 0.5f) + 0.5f);
            let _e662 = (((_e656 * _e656) * _e629.danceability) * (0.025f + (_e629.energy * 0.055f)));
            let _e663 = (_e629.energy * 0.55f);
            let _e668 = ((_e663 + (_e629.danceability * 0.25f)) + (_e629.loudness * 0.2f));
            if _e585 {
                phi_25_ = true;
            } else {
                phi_25_ = (0f <= _e566);
            }
            let _e671 = phi_25_;
            let _e674 = (1f + (select(_e566, 0f, _e671) * 0.008333334f));
            let _e676 = select(_e674, 0f, (_e674 < 0f));
            let _e678 = select(_e676, 1f, (_e676 > 1f));
            let _e689 = (_e637 - trunc(_e637));
            let _e694 = ((_e182 / _e186) * ((0.5f + (_e689 * 0.12f)) + (_e668 * 0.18f)));
            if (_e694 != _e694) {
                phi_26_ = true;
            } else {
                phi_26_ = (1.7f >= _e694);
            }
            let _e698 = phi_26_;
            let _e701 = select(0f, _e393, (_e393 > 0f));
            let _e703 = select(0f, _e394, (_e394 > 0f));
            let _e711 = (select(1f, _e703, (_e703 < 1f)) - (((((_e396 * _e678) * _e678) * 0.6f) + _e391.y) * 0.08f));
            let _e712 = ((select(1f, _e701, (_e701 < 1f)) - (((((_e395 * _e678) * _e678) * 0.6f) + _e367.x) * 0.08f)) * select(_e694, 1.7f, _e698));
            let _e723 = (_e651 * 0.8f);
            let _e733 = ((0.14f + (_e668 * 0.2f)) + _e662);
            let _e738 = (_e637 + 1.5707964f);
            let _e743 = pill.member[_e166].colors[0u];
            let _e745 = vec2<f32>((_e712 + ((sin(((_e711 * 4.32f) + _e651)) + cos(((_e712 * 1.3f) - (_e651 * 0.7f)))) * _e733)), ((_e711 * 1.6f) + ((cos(((_e712 * 2.3f) - _e723)) + sin(((_e711 * 2.72f) + (_e651 * 0.6f)))) * _e733)));
            let _e746 = cantus_render_track_plasma_field(_e745, unpack4x8unorm(_e743), 2.1f, 0.7f, _e651);
            let _e751 = pill.member[_e166].colors[1u];
            let _e754 = cantus_render_track_plasma_field(_e745, unpack4x8unorm(_e751), 0.6f, -2.4f, (_e738 - _e723));
            let _e771 = pill.member[_e166].colors[2u];
            let _e775 = cantus_render_track_plasma_field(_e745, unpack4x8unorm(_e771), -1.5f, 1.9f, ((_e651 * 0.65f) + 2f));
            let _e788 = pill.member[_e166].colors[3u];
            let _e789 = unpack4x8unorm(_e788);
            let _e792 = cantus_render_track_plasma_field(_e745, _e789, 2.4f, 1.6f, (_e738 - (_e651 * 0.55f)));
            let _e800 = (((_e746.w + _e754.w) + _e775.w) + _e792.w);
            let _e801 = ((((_e746.x + _e754.x) + _e775.x) + _e792.x) / _e800);
            let _e802 = ((((_e746.y + _e754.y) + _e775.y) + _e792.y) / _e800);
            let _e803 = ((((_e746.z + _e754.z) + _e775.z) + _e792.z) / _e800);
            let _e808 = (((_e801 * 0.2126f) + (_e802 * 0.7152f)) + (_e803 * 0.0722f));
            let _e812 = frame.member[0u].playhead_x;
            let _e813 = (_e812 + 3f);
            let _e817 = ((_e165.x - _e813) / ((_e812 - 3f) - _e813));
            let _e819 = select(_e817, 0f, (_e817 < 0f));
            let _e821 = select(_e819, 1f, (_e819 > 1f));
            let _e830 = pill.member[_e166].effects.valence;
            let _e831 = (_e830 * 0.4f);
            let _e832 = (1.55f + _e831);
            let _e834 = (_e808 * (-0.54999995f - _e831));
            let _e838 = (_e834 + (_e801 * _e832));
            let _e839 = (_e834 + (_e802 * _e832));
            let _e840 = (_e834 + (_e803 * _e832));
            let _e842 = select(0.035f, _e838, (_e838 > 0.035f));
            let _e844 = select(0.035f, _e839, (_e839 > 0.035f));
            let _e846 = select(0.035f, _e840, (_e840 > 0.035f));
            if (_e808 != _e808) {
                phi_27_ = true;
            } else {
                phi_27_ = (0.001f >= _e808);
            }
            let _e856 = phi_27_;
            let _e858 = (0.52f / select(_e808, 0.001f, _e856));
            if (_e858 != _e858) {
                phi_28_ = true;
            } else {
                phi_28_ = (1f <= _e858);
            }
            let _e862 = phi_28_;
            let _e863 = select(_e858, 1f, _e862);
            let _e870 = ((0.96f + (_e830 * 0.06f)) + (_e662 * 0.5f));
            let _e875 = ((_e623 - 0.45f) * 1.8181818f);
            let _e877 = select(_e875, 0f, (_e875 < 0f));
            let _e879 = select(_e877, 1f, (_e877 > 1f));
            let _e885 = (0.84f + (((_e879 * _e879) * (3f - (2f * _e879))) * 0.1f));
            let _e890 = (1f - (0.4f * ((_e821 * _e821) * (3f - (2f * _e821)))));
            let _e910 = (8f - _e629.acousticness);
            let _e914 = (_e633 * (0.35f + _e663));
            let _e917 = ((_e187 / _e910) + (_e914 * (0.16f + (_e689 * 0.08f))));
            let _e918 = ((_e188 / _e910) + (_e914 * (0.055f + (sin((_e637 * 0.7f)) * 0.025f))));
            let _e919 = floor(_e917);
            let _e920 = floor(_e918);
            let _e929 = bitcast<u32>(select(0i, select(select(i32(_e920), i32(-2147483648), (_e920 < -2147483600f)), 2147483647i, (_e920 > 2147483500f)), (_e920 == _e920)));
            let _e937 = bitcast<u32>(select(0i, select(select(i32(_e919), i32(-2147483648), (_e919 < -2147483600f)), 2147483647i, (_e919 > 2147483500f)), (_e919 == _e919)));
            let _e939 = (bitcast<u32>((_e637 + 2.71f)) * 2654435761u);
            let _e945 = (((_e937 ^ _e939) * 1664525u) + 1013904223u);
            let _e947 = ((((_e929 ^ _e939) * 1664525u) + 1013904223u) + (_e945 * 1664525u));
            let _e949 = (_e945 + (_e947 * 1664525u));
            let _e957 = ((_e947 ^ (_e947 >> bitcast<u32>(16i))) + ((_e949 ^ (_e949 >> bitcast<u32>(16i))) * 1664525u));
            let _e961 = f32((_e957 ^ (_e957 >> bitcast<u32>(16i))));
            let _e962 = (_e961 * 0.0000000016600825f);
            let _e976 = (_e629.acousticness * 0.09f);
            let _e979 = (bitcast<u32>(_e637) * 2654435761u);
            let _e985 = (((_e929 ^ _e979) * 1664525u) + 1013904223u);
            let _e987 = ((((_e937 ^ _e979) * 1664525u) + 1013904223u) + (_e985 * 1664525u));
            let _e989 = (_e985 + (_e987 * 1664525u));
            let _e997 = ((_e987 ^ (_e987 >> bitcast<u32>(16i))) + ((_e989 ^ (_e989 >> bitcast<u32>(16i))) * 1664525u));
            let _e1005 = (((f32((_e997 ^ (_e997 >> bitcast<u32>(16i)))) * 0.00000000023283064f) - (0.985f - _e976)) / (_e976 + 0.014999986f));
            let _e1007 = select(_e1005, 0f, (_e1005 < 0f));
            let _e1009 = select(_e1007, 1f, (_e1007 > 1f));
            let _e1018 = (((_e917 - _e919) - 0.5f) - ((_e961 * 0.00000000013038516f) - 0.28f));
            let _e1019 = (((_e918 - _e920) - 0.5f) - (((_e962 - trunc(_e962)) * 0.56f) - 0.28f));
            let _e1025 = ((sqrt(((_e1018 * _e1018) + (_e1019 * _e1019))) - 0.06f) * 4.5454545f);
            let _e1027 = select(_e1025, 0f, (_e1025 < 0f));
            let _e1029 = select(_e1027, 1f, (_e1027 > 1f));
            let _e1042 = (((((_e1009 * _e1009) * (3f - (2f * _e1009))) * (1f - ((_e1029 * _e1029) * (3f - (2f * _e1029))))) * ((sin(((_e633 * ((0.7f + (_e961 * 0.00000000020954757f)) + (_e629.energy * 0.8f))) + (_e961 * 0.0000000014629181f))) * 0.5f) + 0.5f)) * (0.12f + (_e629.acousticness * 0.48f)));
            let _e1046 = (((((select(0.92f, _e842, (_e842 < 0.92f)) * _e863) * _e870) * _e885) * _e890) + (((_e789.x * 0.75f) + 0.25f) * _e1042));
            let _e1047 = (((((select(0.92f, _e844, (_e844 < 0.92f)) * _e863) * _e870) * _e885) * _e890) + (((_e789.y * 0.75f) + 0.25f) * _e1042));
            let _e1048 = (((((select(0.92f, _e846, (_e846 < 0.92f)) * _e863) * _e870) * _e885) * _e890) + (((_e789.z * 0.75f) + 0.25f) * _e1042));
            let _e1055 = (_e187 / _e186);
            if (_e629.instrumentalness <= 0.00390625f) {
                phi_32_ = 0f;
            } else {
                let _e1060 = (_e633 * (0.5f + (_e629.energy * 0.35f)));
                let _e1068 = (sin(((_e394 * 1.9f) + _e1060)) * 0.35f);
                let _e1069 = (sin(((_e1055 * 1.5f) - (_e1060 * 0.8f))) * 0.35f);
                let _e1072 = ((_e1060 * 0.05f) + _e637);
                let _e1073 = ((_e1060 * -0.04f) + _e637);
                let _e1081 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1055 * 0.7f) + _e1068) + _e1072), (((_e394 * 0.7f) + _e1069) + _e1073)));
                let _e1084 = (1f - (abs(_e1081) * 2f));
                if (_e1084 != _e1084) {
                    phi_29_ = true;
                } else {
                    phi_29_ = (0f >= _e1084);
                }
                let _e1088 = phi_29_;
                let _e1089 = select(_e1084, 0f, _e1088);
                let _e1091 = ((_e1089 * _e1089) * _e1089);
                let _e1101 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1055 * 1.1f) - _e1068) - (_e1072 * 0.8f)), (((_e394 * 1.1f) - _e1069) - (_e1073 * 0.8f))));
                let _e1104 = (1f - (abs(_e1101) * 2f));
                if (_e1104 != _e1104) {
                    phi_30_ = true;
                } else {
                    phi_30_ = (0f >= _e1104);
                }
                let _e1108 = phi_30_;
                let _e1109 = select(_e1104, 0f, _e1108);
                let _e1111 = ((_e1109 * _e1109) * _e1109);
                if (_e1091 != _e1091) {
                    phi_31_ = true;
                } else {
                    phi_31_ = (_e1111 >= _e1091);
                }
                let _e1115 = phi_31_;
                phi_32_ = ((select(_e1091, _e1111, _e1115) * _e629.instrumentalness) * 0.06f);
            }
            let _e1120 = phi_32_;
            let _e1124 = (_e1046 + (((_e1046 * 0.25f) + 0.75f) * _e1120));
            let _e1125 = (_e1047 + (((_e1047 * 0.25f) + 0.75f) * _e1120));
            let _e1126 = (_e1048 + (((_e1048 * 0.25f) + 0.75f) * _e1120));
            let _e1127 = vec3<f32>(_e1124, _e1125, _e1126);
            let _e1128 = (_e193 + _e190);
            let _e1132 = pill.member[_e166].image_index;
            if (_e1132 >= 0i) {
                let _e1134 = (_e187 - _e1128);
                let _e1135 = abs(_e1134);
                let _e1136 = abs(_e192);
                if (select(_e1136, _e1135, (_e1135 > _e1136)) < _e186) {
                    let _e1140 = (_e190 + _e565);
                    let _e1146 = (_e1140 * 2f);
                    let _e1152 = vec3<f32>(((_e1134 / _e1146) + 0.5f), ((_e192 / _e1146) + 0.5f), f32(_e1132));
                    let _e1158 = textureSample(images, sampler_, vec2<f32>(_e1152.x, _e1152.y), i32(_e1152.z));
                    let _e1160 = (((sqrt(((_e1134 * _e1134) + (_e192 * _e192))) - _e1140) - -4f) * 0.25f);
                    let _e1162 = select(_e1160, 0f, (_e1160 < 0f));
                    let _e1164 = select(_e1162, 1f, (_e1162 > 1f));
                    let _e1171 = ((_e219 - 0.5f) * -1f);
                    let _e1173 = select(_e1171, 0f, (_e1171 < 0f));
                    let _e1175 = select(_e1173, 1f, (_e1173 > 1f));
                    let _e1184 = ((_e196 - (((_e365 * ((_e1175 * _e1175) * (3f - (2f * _e1175)))) + _e389) * 0.5f)) - -0.5f);
                    let _e1186 = select(_e1184, 0f, (_e1184 < 0f));
                    let _e1188 = select(_e1186, 1f, (_e1186 > 1f));
                    let _e1199 = (((1f - ((_e1164 * _e1164) * (3f - (2f * _e1164)))) * (1f - ((_e1188 * _e1188) * (3f - (2f * _e1188))))) * _e1158.w);
                    let _e1200 = (1f - _e1199);
                    phi_33_ = vec3<f32>(((_e1124 * _e1200) + (_e1158.x * _e1199)), ((_e1125 * _e1200) + (_e1158.y * _e1199)), ((_e1126 * _e1200) + (_e1158.z * _e1199)));
                } else {
                    phi_33_ = _e1127;
                }
                let _e1212 = phi_33_;
                phi_34_ = _e1212;
            } else {
                phi_34_ = _e1127;
            }
            let _e1214 = phi_34_;
            let _e1225 = ((_e623 - 0.12f) * -8.333334f);
            let _e1227 = select(_e1225, 0f, (_e1225 < 0f));
            let _e1229 = select(_e1227, 1f, (_e1227 > 1f));
            let _e1236 = ((_e566 - 5f) * -0.125f);
            let _e1238 = select(_e1236, 0f, (_e1236 < 0f));
            let _e1240 = select(_e1238, 1f, (_e1238 > 1f));
            let _e1246 = ((((_e1229 * _e1229) * (3f - (2f * _e1229))) * 0.12f) + (((_e1240 * _e1240) * (3f - (2f * _e1240))) * 0.08f));
            let _e1250 = (_e1214.x + (((_e1214.x * 0.68f) + 0.32f) * _e1246));
            let _e1251 = (_e1214.y + (((_e1214.y * 0.68f) + 0.32f) * _e1246));
            let _e1252 = (_e1214.z + (((_e1214.z * 0.68f) + 0.32f) * _e1246));
            let _e1260 = local_6;
            let _e1261 = (1f - _e1260);
            let _e1266 = local_7;
            let _e1269 = local_8;
            let _e1272 = local_9;
            let _e1280 = vec4<f32>((((_e1250 * _e1261) + (((_e1250 * 1.5f) + 0.1f) * _e1266)) * _e584), (((_e1251 * _e1261) + (((_e1251 * 1.5f) + 0.1f) * _e1269)) * _e584), (((_e1252 * _e1261) + (((_e1252 * 1.5f) + 0.1f) * _e1272)) * _e584), _e597);
            if _e404 {
                if (_e482 > 0f) {
                    phi_35_ = _e1280;
                    phi_36_ = 0u;
                    loop {
                        let _e1283 = phi_35_;
                        let _e1285 = phi_36_;
                        local_22 = _e1283;
                        let _e1286 = (_e1285 < 5u);
                        if _e1286 {
                            let _e1287 = f32(_e1285);
                            if _e440 {
                                phi_37_ = true;
                            } else {
                                phi_37_ = (0f >= _e439);
                            }
                            let _e1290 = phi_37_;
                            let _e1295 = (_e397 + ((_e1287 - (select(_e439, 0f, _e1290) * 0.5f)) * 18f));
                            let _e1296 = (_e398 + 5f);
                            let _e1297 = (_e165.x - _e1295);
                            let _e1298 = (_e165.y - _e1296);
                            let _e1299 = abs(_e1297);
                            let _e1300 = abs(_e1298);
                            if (select(_e1300, _e1299, (_e1299 > _e1300)) < 38.88f) {
                                let _e1307 = ((f32(_e403) - (_e1287 * 2f)) * 0.5f);
                                let _e1309 = select(_e1307, 0f, (_e1307 < 0f));
                                let _e1312 = (_e1295 - _e431);
                                let _e1313 = (_e1296 - _e436);
                                let _e1319 = ((sqrt(((_e1312 * _e1312) + (_e1313 * _e1313))) - 11.3f) * -1f);
                                let _e1321 = select(_e1319, 0f, (_e1319 < 0f));
                                let _e1323 = select(_e1321, 1f, (_e1321 > 1f));
                                let _e1329 = select(_e200, 0f, (_e200 < 0f));
                                let _e1332 = (((_e1323 * _e1323) * (3f - (2f * _e1323))) * select(_e1329, 1f, (_e1329 > 1f)));
                                let _e1334 = (1.05f + (0.63f * _e1332));
                                let _e1335 = (_e1312 * _e1332);
                                let _e1337 = (_e1297 - (_e1335 * 0.5f));
                                let _e1338 = (_e1335 * -0.005f);
                                let _e1339 = sin(_e1338);
                                let _e1340 = cos(_e1338);
                                let _e1343 = ((_e1340 * _e1337) - (_e1339 * _e1298));
                                let _e1346 = ((_e1339 * _e1337) + (_e1340 * _e1298));
                                let _e1350 = (_e1334 * 5.4f);
                                let _e1351 = abs(_e1343);
                                let _e1355 = ((0.809017f * _e1351) + (_e1346 * 0.58778524f));
                                if (_e1355 != _e1355) {
                                    phi_38_ = true;
                                } else {
                                    phi_38_ = (0f >= _e1355);
                                }
                                let _e1359 = phi_38_;
                                let _e1360 = select(_e1355, 0f, _e1359);
                                let _e1363 = (_e1351 - (_e1360 * 1.618034f));
                                let _e1364 = (-(_e1346) - (_e1360 * -1.1755705f));
                                let _e1367 = ((-0.809017f * _e1363) + (-0.58778524f * _e1364));
                                if (_e1367 != _e1367) {
                                    phi_39_ = true;
                                } else {
                                    phi_39_ = (0f >= _e1367);
                                }
                                let _e1371 = phi_39_;
                                let _e1372 = select(_e1367, 0f, _e1371);
                                let _e1377 = abs((_e1363 - (_e1372 * -1.618034f)));
                                let _e1378 = ((_e1364 - (_e1372 * -1.1755705f)) - _e1350);
                                let _e1379 = (_e1334 * 2.031386f);
                                let _e1381 = ((_e1334 * 2.7959628f) - _e1350);
                                let _e1388 = (((_e1377 * _e1379) + (_e1378 * _e1381)) / ((_e1379 * _e1379) + (_e1381 * _e1381)));
                                let _e1390 = select(_e1388, 0f, (_e1388 < 0f));
                                let _e1392 = select(_e1390, 1f, (_e1390 > 1f));
                                let _e1398 = (_e1377 - (_e1379 * _e1392));
                                let _e1399 = (_e1378 - (_e1381 * _e1392));
                                let _e1408 = ((sqrt(((_e1398 * _e1398) + (_e1399 * _e1399))) * select(1f, -1f, (((_e1378 * _e1379) - (_e1377 * _e1381)) < 0f))) - (_e1334 * 1.08f));
                                let _e1409 = (((_e1343 / (_e1334 * 21.6f)) + 0.5f) - select(_e1309, 1f, (_e1309 > 1f)));
                                let _e1410 = fwidth(_e1409);
                                let _e1412 = ((_e1409 / _e1410) + 0.5f);
                                let _e1414 = select(_e1412, 0f, (_e1412 < 0f));
                                let _e1416 = select(_e1414, 1f, (_e1414 > 1f));
                                let _e1417 = (1f - _e1416);
                                let _e1420 = (0.33f * _e1416);
                                let _e1424 = (0.5f - _e1408);
                                let _e1426 = select(_e1424, 0f, (_e1424 < 0f));
                                let _e1428 = select(_e1426, 1f, (_e1426 > 1f));
                                if (_e1408 != _e1408) {
                                    phi_40_ = true;
                                } else {
                                    phi_40_ = (0f >= _e1408);
                                }
                                let _e1432 = phi_40_;
                                let _e1435 = exp((select(_e1408, 0f, _e1432) * -0.5f));
                                let _e1436 = (_e1408 * -0.2f);
                                let _e1438 = select(_e1436, 0f, (_e1436 < 0f));
                                let _e1440 = select(_e1438, 1f, (_e1438 > 1f));
                                let _e1445 = (1f - ((_e1440 * _e1440) * (3f - (2f * _e1440))));
                                let _e1447 = ((_e1445 * _e1445) * 0.045f);
                                let _e1458 = ((_e1435 * _e1435) * 0.2f);
                                if (_e1428 != _e1428) {
                                    phi_41_ = true;
                                } else {
                                    phi_41_ = (_e1458 >= _e1428);
                                }
                                let _e1462 = phi_41_;
                                let _e1464 = (select(_e1428, _e1458, _e1462) * _e482);
                                let _e1465 = (1f - _e1464);
                                phi_42_ = vec4<f32>(((_e1283.x * _e1465) + ((((_e1417 + _e1420) + _e1447) * _e1428) * _e482)), ((_e1283.y * _e1465) + (((((0.85f * _e1417) + _e1420) + _e1447) * _e1428) * _e482)), ((_e1283.z * _e1465) + (((((0.2f * _e1417) + _e1420) + _e1447) * _e1428) * _e482)), ((_e1283.w * _e1465) + _e1464));
                            } else {
                                phi_42_ = _e1283;
                            }
                            let _e1480 = phi_42_;
                            phi_43_ = _e1480;
                            phi_44_ = (_e1285 + 1u);
                        } else {
                            phi_43_ = vec4<f32>();
                            phi_44_ = u32();
                        }
                        let _e1483 = phi_43_;
                        let _e1485 = phi_44_;
                        continue;
                        continuing {
                            phi_35_ = _e1483;
                            phi_36_ = _e1485;
                            break if !(_e1286);
                        }
                    }
                    if _e335 {
                        break;
                    }
                    let _e2238 = local_22;
                    phi_45_ = _e2238;
                } else {
                    phi_45_ = _e1280;
                }
                let _e1488 = phi_45_;
                phi_46_ = _e1488;
            } else {
                phi_46_ = _e1280;
            }
            let _e1490 = phi_46_;
            let _e1491 = (_e409 + _e423);
            phi_47_ = _e1490;
            phi_48_ = 0u;
            loop {
                let _e1495 = phi_47_;
                let _e1497 = phi_48_;
                local_18 = _e1495;
                local_19 = _e1495;
                local_20 = _e1495;
                local_21 = _e1495;
                let _e1498 = (_e1497 < select(_e1491, 8u, (8u < _e1491)));
                if _e1498 {
                    if (_e1497 < 8u) {
                    } else {
                        phi_62_ = true;
                        break;
                    }
                    let _e1504 = pill.member[_e166].playlist_images[_e1497];
                    if (_e1504 >= 0i) {
                        let _e1506 = (_e1497 < _e409);
                        if _e1506 {
                            phi_49_ = render_shared_RipplePulse(vec2<f32>(_e397, _e399), _e411, 1f);
                            phi_50_ = (f32(_e1497) + _e405);
                        } else {
                            phi_49_ = render_shared_RipplePulse(vec2<f32>(_e397, _e419), _e424, _e417);
                            phi_50_ = f32((_e1497 - _e409));
                        }
                        let _e1512 = phi_49_;
                        let _e1514 = phi_50_;
                        let _e1515 = select(_e417, _e482, _e1506);
                        let _e1517 = (_e1512.start_time - 1f);
                        if (_e1517 != _e1517) {
                            phi_51_ = true;
                        } else {
                            phi_51_ = (0f >= _e1517);
                        }
                        let _e1521 = phi_51_;
                        let _e1530 = (_e1512.origin.x + (((_e1514 - (select(_e1517, 0f, _e1521) * 0.5f)) * 18f) * _e1512.strength));
                        let _e1533 = (_e1512.origin.y + 2f);
                        if (_e1515 > 0f) {
                            let _e1535 = (_e165.x - _e1530);
                            let _e1536 = (_e165.y - _e1533);
                            let _e1537 = abs(_e1535);
                            let _e1538 = abs(_e1536);
                            if (select(_e1538, _e1537, (_e1537 > _e1538)) < 38.88f) {
                                let _e1542 = (_e1530 - _e431);
                                let _e1543 = (_e1533 - _e436);
                                let _e1547 = sqrt(((_e1542 * _e1542) + (_e1543 * _e1543)));
                                let _e1549 = ((_e1547 - 11.3f) * -1f);
                                let _e1551 = select(_e1549, 0f, (_e1549 < 0f));
                                let _e1553 = select(_e1551, 1f, (_e1551 > 1f));
                                let _e1559 = select(_e200, 0f, (_e200 < 0f));
                                let _e1562 = (((_e1553 * _e1553) * (3f - (2f * _e1553))) * select(_e1559, 1f, (_e1559 > 1f)));
                                let _e1564 = (1.05f + (0.63f * _e1562));
                                let _e1565 = (_e1542 * _e1562);
                                let _e1567 = (_e1535 - (_e1565 * 0.5f));
                                let _e1568 = (_e1565 * -0.005f);
                                let _e1569 = sin(_e1568);
                                let _e1570 = cos(_e1568);
                                let _e1573 = ((_e1570 * _e1567) - (_e1569 * _e1536));
                                let _e1576 = ((_e1569 * _e1567) + (_e1570 * _e1536));
                                let _e1577 = (_e1564 * 21.6f);
                                if _e1506 {
                                    phi_53_ = true;
                                } else {
                                    if _e201 {
                                        phi_52_ = select(true, false, (_e1547 <= 10.8f));
                                    } else {
                                        phi_52_ = true;
                                    }
                                    let _e1585 = phi_52_;
                                    phi_53_ = select(true, false, _e1585);
                                }
                                let _e1588 = phi_53_;
                                let _e1589 = select(0.2f, 0f, _e1588);
                                let _e1592 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1573, _e1576), 0f, (_e1564 * 6.4800005f));
                                if (_e1592 <= 7f) {
                                    let _e1595 = vec3<f32>(((_e1573 / _e1577) + 0.5f), ((_e1576 / _e1577) + 0.5f), f32(_e1504));
                                    let _e1601 = textureSample(images, sampler_, vec2<f32>(_e1595.x, _e1595.y), i32(_e1595.z));
                                    let _e1605 = (1f - _e1589);
                                    let _e1609 = (0.24f * _e1589);
                                    let _e1613 = (0.5f - _e1592);
                                    let _e1615 = select(_e1613, 0f, (_e1613 < 0f));
                                    let _e1617 = select(_e1615, 1f, (_e1615 > 1f));
                                    if (_e1592 != _e1592) {
                                        phi_54_ = true;
                                    } else {
                                        phi_54_ = (0f >= _e1592);
                                    }
                                    let _e1621 = phi_54_;
                                    let _e1624 = exp((select(_e1592, 0f, _e1621) * -0.5f));
                                    let _e1625 = (_e1592 * -0.2f);
                                    let _e1627 = select(_e1625, 0f, (_e1625 < 0f));
                                    let _e1629 = select(_e1627, 1f, (_e1627 > 1f));
                                    let _e1634 = (1f - ((_e1629 * _e1629) * (3f - (2f * _e1629))));
                                    let _e1636 = ((_e1634 * _e1634) * 0.045f);
                                    let _e1647 = ((_e1624 * _e1624) * 0.2f);
                                    if (_e1617 != _e1617) {
                                        phi_55_ = true;
                                    } else {
                                        phi_55_ = (_e1647 >= _e1617);
                                    }
                                    let _e1651 = phi_55_;
                                    let _e1653 = (select(_e1617, _e1647, _e1651) * _e1515);
                                    let _e1654 = (1f - _e1653);
                                    phi_56_ = vec4<f32>(((_e1495.x * _e1654) + (((((_e1601.x * _e1605) + _e1609) + _e1636) * _e1617) * _e1515)), ((_e1495.y * _e1654) + (((((_e1601.y * _e1605) + _e1609) + _e1636) * _e1617) * _e1515)), ((_e1495.z * _e1654) + (((((_e1601.z * _e1605) + _e1609) + _e1636) * _e1617) * _e1515)), ((_e1495.w * _e1654) + _e1653));
                                } else {
                                    phi_56_ = _e1495;
                                }
                                let _e1669 = phi_56_;
                                phi_57_ = _e1669;
                            } else {
                                phi_57_ = _e1495;
                            }
                            let _e1671 = phi_57_;
                            phi_58_ = _e1671;
                        } else {
                            phi_58_ = _e1495;
                        }
                        let _e1673 = phi_58_;
                        phi_59_ = _e1673;
                    } else {
                        phi_59_ = _e1495;
                    }
                    let _e1675 = phi_59_;
                    phi_60_ = _e1675;
                    phi_61_ = (_e1497 + 1u);
                } else {
                    phi_60_ = vec4<f32>();
                    phi_61_ = u32();
                }
                let _e1678 = phi_60_;
                let _e1680 = phi_61_;
                continue;
                continuing {
                    phi_47_ = _e1678;
                    phi_48_ = _e1680;
                    phi_62_ = _e335;
                    break if !(_e1498);
                }
            }
            let _e1683 = phi_62_;
            if _e1683 {
                break;
            }
            let _e1688 = pill.member[_e166].lines[0u];
            if (_e624 < _e1688.min.x) {
                phi_103_ = _e1683;
                phi_104_ = f32();
                phi_105_ = true;
            } else {
                if (_e624 > _e1688.max.x) {
                    phi_100_ = _e1683;
                    phi_101_ = f32();
                    phi_102_ = true;
                } else {
                    if (_e625 < _e1688.min.y) {
                        phi_97_ = _e1683;
                        phi_98_ = f32();
                        phi_99_ = true;
                    } else {
                        let _e1700 = (_e625 > _e1688.max.y);
                        if _e1700 {
                            phi_95_ = _e1683;
                            phi_96_ = f32();
                        } else {
                            let _e1703 = (1f / _e1688.size);
                            let _e1710 = ((_e624 - _e1688.origin.x) * _e1703);
                            phi_63_ = _e1688.count;
                            phi_64_ = 0u;
                            loop {
                                let _e1713 = phi_63_;
                                let _e1715 = phi_64_;
                                local_10 = _e1715;
                                let _e1716 = (_e1715 < _e1713);
                                if _e1716 {
                                    let _e1719 = (_e1715 + ((_e1713 - _e1715) / 2u));
                                    let _e1721 = (_e1688.first + _e1719);
                                    if (_e1721 < _e168) {
                                    } else {
                                        phi_68_ = true;
                                        break;
                                    }
                                    let _e1726 = placed_glyphs.member[_e1721].x;
                                    let _e1727 = (_e1726 <= _e1710);
                                    if _e1727 {
                                        phi_65_ = (_e1719 + 1u);
                                    } else {
                                        phi_65_ = _e1715;
                                    }
                                    let _e1730 = phi_65_;
                                    phi_66_ = select(_e1719, _e1713, _e1727);
                                    phi_67_ = _e1730;
                                } else {
                                    phi_66_ = u32();
                                    phi_67_ = u32();
                                }
                                let _e1733 = phi_66_;
                                let _e1735 = phi_67_;
                                continue;
                                continuing {
                                    phi_63_ = _e1733;
                                    phi_64_ = _e1735;
                                    phi_68_ = _e1683;
                                    break if !(_e1716);
                                }
                            }
                            let _e1738 = phi_68_;
                            if _e1738 {
                                break;
                            }
                            let _e1739 = (3.5f / _e1688.size);
                            let _e1741 = local_10;
                            let _e1742 = (_e1741 + 1u);
                            phi_69_ = _e1738;
                            phi_70_ = select(_e1742, _e1688.count, (_e1688.count < _e1742));
                            phi_71_ = -1000000f;
                            loop {
                                let _e1746 = phi_69_;
                                let _e1748 = phi_70_;
                                let _e1750 = phi_71_;
                                local_13 = _e1750;
                                if (_e1748 > 0u) {
                                    let _e1752 = (_e1748 - 1u);
                                    let _e1754 = (_e1688.first + _e1752);
                                    if (_e1754 < _e168) {
                                    } else {
                                        phi_94_ = true;
                                        break;
                                    }
                                    let _e1759 = placed_glyphs.member[_e1754].x;
                                    let _e1763 = placed_glyphs.member[_e1754].glyph;
                                    if (_e1763 < _e170) {
                                    } else {
                                        phi_94_ = true;
                                        break;
                                    }
                                    let _e1769 = glyphs.member[_e1763].min[0u];
                                    let _e1774 = glyphs.member[_e1763].min[1u];
                                    let _e1779 = glyphs.member[_e1763].max[0u];
                                    let _e1784 = glyphs.member[_e1763].max[1u];
                                    let _e1788 = glyphs.member[_e1763].start;
                                    let _e1792 = glyphs.member[_e1763].count;
                                    let _e1793 = (_e1710 - _e1759);
                                    let _e1794 = -(((_e625 - _e1688.origin.y) * _e1703));
                                    let _e1795 = (_e1779 + _e1739);
                                    let _e1796 = (_e1793 > _e1795);
                                    if _e1796 {
                                        phi_88_ = _e1746;
                                        phi_89_ = f32();
                                    } else {
                                        if (_e1793 >= (_e1769 - _e1739)) {
                                            if (_e1794 >= (_e1774 - _e1739)) {
                                                if (_e1793 <= _e1795) {
                                                    if (_e1794 <= (_e1784 + _e1739)) {
                                                        phi_72_ = 340282350000000000000000000000000000000f;
                                                        phi_73_ = 0u;
                                                        phi_74_ = 0i;
                                                        loop {
                                                            let _e1806 = phi_72_;
                                                            let _e1808 = phi_73_;
                                                            let _e1810 = phi_74_;
                                                            local_11 = _e1806;
                                                            local_12 = _e1810;
                                                            let _e1811 = (_e1808 < _e1792);
                                                            if _e1811 {
                                                                let _e1812 = (_e1788 + _e1808);
                                                                if (_e1812 < _e172) {
                                                                } else {
                                                                    phi_78_ = true;
                                                                    break;
                                                                }
                                                                let _e1816 = edges.member[_e1812];
                                                                let _e1818 = cantus_render_text_edge_distance(_e1816, _e1688.weight, vec2<f32>(_e1793, _e1794), _e1806);
                                                                phi_75_ = _e1818.member;
                                                                phi_76_ = (_e1808 + 1u);
                                                                phi_77_ = (_e1810 + _e1818.member_1);
                                                            } else {
                                                                phi_75_ = f32();
                                                                phi_76_ = u32();
                                                                phi_77_ = i32();
                                                            }
                                                            let _e1824 = phi_75_;
                                                            let _e1826 = phi_76_;
                                                            let _e1828 = phi_77_;
                                                            continue;
                                                            continuing {
                                                                phi_72_ = _e1824;
                                                                phi_73_ = _e1826;
                                                                phi_74_ = _e1828;
                                                                phi_78_ = _e1746;
                                                                break if !(_e1811);
                                                            }
                                                        }
                                                        let _e1831 = phi_78_;
                                                        phi_94_ = _e1831;
                                                        if _e1831 {
                                                            break;
                                                        }
                                                        let _e1833 = local_11;
                                                        let _e1837 = local_12;
                                                        let _e1840 = ((sqrt(_e1833) * _e1688.size) * select(1f, -1f, (_e1837 == 0i)));
                                                        if (_e1750 != _e1750) {
                                                            phi_79_ = true;
                                                        } else {
                                                            phi_79_ = (_e1840 >= _e1750);
                                                        }
                                                        let _e1844 = phi_79_;
                                                        phi_80_ = _e1831;
                                                        phi_81_ = select(_e1750, _e1840, _e1844);
                                                    } else {
                                                        phi_80_ = _e1746;
                                                        phi_81_ = _e1750;
                                                    }
                                                    let _e1847 = phi_80_;
                                                    let _e1849 = phi_81_;
                                                    phi_82_ = _e1847;
                                                    phi_83_ = _e1849;
                                                } else {
                                                    phi_82_ = _e1746;
                                                    phi_83_ = _e1750;
                                                }
                                                let _e1851 = phi_82_;
                                                let _e1853 = phi_83_;
                                                phi_84_ = _e1851;
                                                phi_85_ = _e1853;
                                            } else {
                                                phi_84_ = _e1746;
                                                phi_85_ = _e1750;
                                            }
                                            let _e1855 = phi_84_;
                                            let _e1857 = phi_85_;
                                            phi_86_ = _e1855;
                                            phi_87_ = _e1857;
                                        } else {
                                            phi_86_ = _e1746;
                                            phi_87_ = _e1750;
                                        }
                                        let _e1859 = phi_86_;
                                        let _e1861 = phi_87_;
                                        phi_88_ = _e1859;
                                        phi_89_ = _e1861;
                                    }
                                    let _e1863 = phi_88_;
                                    let _e1865 = phi_89_;
                                    phi_90_ = _e1863;
                                    phi_91_ = _e1752;
                                    phi_92_ = _e1865;
                                    phi_93_ = select(true, false, _e1796);
                                } else {
                                    phi_90_ = _e1746;
                                    phi_91_ = u32();
                                    phi_92_ = f32();
                                    phi_93_ = false;
                                }
                                let _e1868 = phi_90_;
                                let _e1870 = phi_91_;
                                let _e1872 = phi_92_;
                                let _e1874 = phi_93_;
                                continue;
                                continuing {
                                    phi_69_ = _e1868;
                                    phi_70_ = _e1870;
                                    phi_71_ = _e1872;
                                    phi_94_ = _e1868;
                                    break if !(_e1874);
                                }
                            }
                            let _e1877 = phi_94_;
                            if _e1877 {
                                break;
                            }
                            let _e1879 = local_13;
                            let _e1881 = ((_e1879 * 1.25f) + 0.5f);
                            let _e1883 = select(_e1881, 0f, (_e1881 < 0f));
                            let _e1885 = select(_e1883, 1f, (_e1883 > 1f));
                            phi_95_ = _e1877;
                            phi_96_ = ((_e1885 * _e1885) * (3f - (2f * _e1885)));
                        }
                        let _e1891 = phi_95_;
                        let _e1893 = phi_96_;
                        phi_97_ = _e1891;
                        phi_98_ = _e1893;
                        phi_99_ = _e1700;
                    }
                    let _e1895 = phi_97_;
                    let _e1897 = phi_98_;
                    let _e1899 = phi_99_;
                    phi_100_ = _e1895;
                    phi_101_ = _e1897;
                    phi_102_ = _e1899;
                }
                let _e1901 = phi_100_;
                let _e1903 = phi_101_;
                let _e1905 = phi_102_;
                phi_103_ = _e1901;
                phi_104_ = _e1903;
                phi_105_ = _e1905;
            }
            let _e1907 = phi_103_;
            let _e1909 = phi_104_;
            let _e1911 = phi_105_;
            let _e1912 = select(_e1909, 0f, _e1911);
            let _e1917 = pill.member[_e166].lines[1u];
            if (_e624 < _e1917.min.x) {
                phi_143_ = f32();
                phi_144_ = true;
            } else {
                if (_e624 > _e1917.max.x) {
                    phi_141_ = f32();
                    phi_142_ = true;
                } else {
                    if (_e625 < _e1917.min.y) {
                        phi_139_ = f32();
                        phi_140_ = true;
                    } else {
                        let _e1929 = (_e625 > _e1917.max.y);
                        if _e1929 {
                            phi_138_ = f32();
                        } else {
                            let _e1932 = (1f / _e1917.size);
                            let _e1939 = ((_e624 - _e1917.origin.x) * _e1932);
                            phi_106_ = _e1917.count;
                            phi_107_ = 0u;
                            loop {
                                let _e1942 = phi_106_;
                                let _e1944 = phi_107_;
                                local_14 = _e1944;
                                let _e1945 = (_e1944 < _e1942);
                                if _e1945 {
                                    let _e1948 = (_e1944 + ((_e1942 - _e1944) / 2u));
                                    let _e1950 = (_e1917.first + _e1948);
                                    if (_e1950 < _e168) {
                                    } else {
                                        phi_111_ = true;
                                        break;
                                    }
                                    let _e1955 = placed_glyphs.member[_e1950].x;
                                    let _e1956 = (_e1955 <= _e1939);
                                    if _e1956 {
                                        phi_108_ = (_e1948 + 1u);
                                    } else {
                                        phi_108_ = _e1944;
                                    }
                                    let _e1959 = phi_108_;
                                    phi_109_ = select(_e1948, _e1942, _e1956);
                                    phi_110_ = _e1959;
                                } else {
                                    phi_109_ = u32();
                                    phi_110_ = u32();
                                }
                                let _e1962 = phi_109_;
                                let _e1964 = phi_110_;
                                continue;
                                continuing {
                                    phi_106_ = _e1962;
                                    phi_107_ = _e1964;
                                    phi_111_ = _e1907;
                                    break if !(_e1945);
                                }
                            }
                            let _e1967 = phi_111_;
                            if _e1967 {
                                break;
                            }
                            let _e1968 = (3.5f / _e1917.size);
                            let _e1970 = local_14;
                            let _e1971 = (_e1970 + 1u);
                            phi_112_ = _e1967;
                            phi_113_ = select(_e1971, _e1917.count, (_e1917.count < _e1971));
                            phi_114_ = -1000000f;
                            loop {
                                let _e1975 = phi_112_;
                                let _e1977 = phi_113_;
                                let _e1979 = phi_114_;
                                local_17 = _e1979;
                                if (_e1977 > 0u) {
                                    let _e1981 = (_e1977 - 1u);
                                    let _e1983 = (_e1917.first + _e1981);
                                    if (_e1983 < _e168) {
                                    } else {
                                        phi_137_ = true;
                                        break;
                                    }
                                    let _e1988 = placed_glyphs.member[_e1983].x;
                                    let _e1992 = placed_glyphs.member[_e1983].glyph;
                                    if (_e1992 < _e170) {
                                    } else {
                                        phi_137_ = true;
                                        break;
                                    }
                                    let _e1998 = glyphs.member[_e1992].min[0u];
                                    let _e2003 = glyphs.member[_e1992].min[1u];
                                    let _e2008 = glyphs.member[_e1992].max[0u];
                                    let _e2013 = glyphs.member[_e1992].max[1u];
                                    let _e2017 = glyphs.member[_e1992].start;
                                    let _e2021 = glyphs.member[_e1992].count;
                                    let _e2022 = (_e1939 - _e1988);
                                    let _e2023 = -(((_e625 - _e1917.origin.y) * _e1932));
                                    let _e2024 = (_e2008 + _e1968);
                                    let _e2025 = (_e2022 > _e2024);
                                    if _e2025 {
                                        phi_131_ = _e1975;
                                        phi_132_ = f32();
                                    } else {
                                        if (_e2022 >= (_e1998 - _e1968)) {
                                            if (_e2023 >= (_e2003 - _e1968)) {
                                                if (_e2022 <= _e2024) {
                                                    if (_e2023 <= (_e2013 + _e1968)) {
                                                        phi_115_ = 340282350000000000000000000000000000000f;
                                                        phi_116_ = 0u;
                                                        phi_117_ = 0i;
                                                        loop {
                                                            let _e2035 = phi_115_;
                                                            let _e2037 = phi_116_;
                                                            let _e2039 = phi_117_;
                                                            local_15 = _e2035;
                                                            local_16 = _e2039;
                                                            let _e2040 = (_e2037 < _e2021);
                                                            if _e2040 {
                                                                let _e2041 = (_e2017 + _e2037);
                                                                if (_e2041 < _e172) {
                                                                } else {
                                                                    phi_121_ = true;
                                                                    break;
                                                                }
                                                                let _e2045 = edges.member[_e2041];
                                                                let _e2047 = cantus_render_text_edge_distance(_e2045, _e1917.weight, vec2<f32>(_e2022, _e2023), _e2035);
                                                                phi_118_ = _e2047.member;
                                                                phi_119_ = (_e2037 + 1u);
                                                                phi_120_ = (_e2039 + _e2047.member_1);
                                                            } else {
                                                                phi_118_ = f32();
                                                                phi_119_ = u32();
                                                                phi_120_ = i32();
                                                            }
                                                            let _e2053 = phi_118_;
                                                            let _e2055 = phi_119_;
                                                            let _e2057 = phi_120_;
                                                            continue;
                                                            continuing {
                                                                phi_115_ = _e2053;
                                                                phi_116_ = _e2055;
                                                                phi_117_ = _e2057;
                                                                phi_121_ = _e1975;
                                                                break if !(_e2040);
                                                            }
                                                        }
                                                        let _e2060 = phi_121_;
                                                        phi_137_ = _e2060;
                                                        if _e2060 {
                                                            break;
                                                        }
                                                        let _e2062 = local_15;
                                                        let _e2066 = local_16;
                                                        let _e2069 = ((sqrt(_e2062) * _e1917.size) * select(1f, -1f, (_e2066 == 0i)));
                                                        if (_e1979 != _e1979) {
                                                            phi_122_ = true;
                                                        } else {
                                                            phi_122_ = (_e2069 >= _e1979);
                                                        }
                                                        let _e2073 = phi_122_;
                                                        phi_123_ = _e2060;
                                                        phi_124_ = select(_e1979, _e2069, _e2073);
                                                    } else {
                                                        phi_123_ = _e1975;
                                                        phi_124_ = _e1979;
                                                    }
                                                    let _e2076 = phi_123_;
                                                    let _e2078 = phi_124_;
                                                    phi_125_ = _e2076;
                                                    phi_126_ = _e2078;
                                                } else {
                                                    phi_125_ = _e1975;
                                                    phi_126_ = _e1979;
                                                }
                                                let _e2080 = phi_125_;
                                                let _e2082 = phi_126_;
                                                phi_127_ = _e2080;
                                                phi_128_ = _e2082;
                                            } else {
                                                phi_127_ = _e1975;
                                                phi_128_ = _e1979;
                                            }
                                            let _e2084 = phi_127_;
                                            let _e2086 = phi_128_;
                                            phi_129_ = _e2084;
                                            phi_130_ = _e2086;
                                        } else {
                                            phi_129_ = _e1975;
                                            phi_130_ = _e1979;
                                        }
                                        let _e2088 = phi_129_;
                                        let _e2090 = phi_130_;
                                        phi_131_ = _e2088;
                                        phi_132_ = _e2090;
                                    }
                                    let _e2092 = phi_131_;
                                    let _e2094 = phi_132_;
                                    phi_133_ = _e2092;
                                    phi_134_ = _e1981;
                                    phi_135_ = _e2094;
                                    phi_136_ = select(true, false, _e2025);
                                } else {
                                    phi_133_ = _e1975;
                                    phi_134_ = u32();
                                    phi_135_ = f32();
                                    phi_136_ = false;
                                }
                                let _e2097 = phi_133_;
                                let _e2099 = phi_134_;
                                let _e2101 = phi_135_;
                                let _e2103 = phi_136_;
                                continue;
                                continuing {
                                    phi_112_ = _e2097;
                                    phi_113_ = _e2099;
                                    phi_114_ = _e2101;
                                    phi_137_ = _e2097;
                                    break if !(_e2103);
                                }
                            }
                            let _e2106 = phi_137_;
                            if _e2106 {
                                break;
                            }
                            let _e2108 = local_17;
                            let _e2110 = ((_e2108 * 1.25f) + 0.5f);
                            let _e2112 = select(_e2110, 0f, (_e2110 < 0f));
                            let _e2114 = select(_e2112, 1f, (_e2112 > 1f));
                            phi_138_ = ((_e2114 * _e2114) * (3f - (2f * _e2114)));
                        }
                        let _e2120 = phi_138_;
                        phi_139_ = _e2120;
                        phi_140_ = _e1929;
                    }
                    let _e2122 = phi_139_;
                    let _e2124 = phi_140_;
                    phi_141_ = _e2122;
                    phi_142_ = _e2124;
                }
                let _e2126 = phi_141_;
                let _e2128 = phi_142_;
                phi_143_ = _e2126;
                phi_144_ = _e2128;
            }
            let _e2130 = phi_143_;
            let _e2132 = phi_144_;
            let _e2133 = select(_e2130, 0f, _e2132);
            if (_e1912 != _e1912) {
                phi_145_ = true;
            } else {
                phi_145_ = (_e2133 >= _e1912);
            }
            let _e2137 = phi_145_;
            let _e2142 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e624 - _e1128), (_e625 - _e190)), 0f, _e190);
            let _e2144 = ((_e2142 - 2f) * 0.0625f);
            let _e2146 = select(_e2144, 0f, (_e2144 < 0f));
            let _e2148 = select(_e2146, 1f, (_e2146 > 1f));
            let _e2154 = ((select(_e1912, _e2133, _e2137) * ((_e2148 * _e2148) * (3f - (2f * _e2148)))) * _e584);
            let _e2155 = (1f - _e2154);
            let _e2157 = local_18;
            let _e2161 = local_19;
            let _e2165 = local_20;
            let _e2169 = local_21;
            let _e2172 = (0.94f * _e2154);
            let _e2180 = (((_e2169.w * _e2155) + _e2154) * _e601);
            if (_e2180 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e2157.x * _e2155) + _e2172) * _e601), (((_e2161.y * _e2155) + _e2172) * _e601), (((_e2165.z * _e2155) + _e2172) * _e601), _e2180);
            break;
        }
    }
    return;
}

fn render_status_isthmus_statuspass_vertex_impl() {
    var phi_0_: bool;
    var phi_1_: u32;
    var phi_2_: f32;
    var phi_3_: u32;
    var phi_4_: f32;
    var phi_5_: bool;
    var local_23: f32;

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
                local_23 = _e55;
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
            let _e93 = local_23;
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
    var local_24: vec2<f32>;
    var local_25: vec2<f32>;
    var phi_16_: bool;
    var local_26: vec2<f32>;
    var phi_17_: f32;
    var local_27: vec2<f32>;
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
    var local_28: f32;
    var phi_45_: vec2<f32>;
    var phi_46_: i32;
    var phi_47_: f32;
    var phi_48_: f32;
    var phi_49_: vec2<f32>;
    var phi_50_: i32;
    var phi_51_: f32;
    var phi_52_: f32;
    var phi_53_: vec2<f32>;
    var local_29: f32;
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
    var local_30: f32;
    var local_31: f32;
    var phi_83_: render_text_Line;
    var phi_84_: bool;
    var phi_85_: u32;
    var phi_86_: u32;
    var phi_87_: u32;
    var phi_88_: u32;
    var phi_89_: u32;
    var phi_90_: bool;
    var local_32: u32;
    var phi_91_: bool;
    var phi_92_: u32;
    var phi_93_: f32;
    var phi_94_: f32;
    var phi_95_: u32;
    var phi_96_: i32;
    var phi_97_: f32;
    var phi_98_: u32;
    var phi_99_: i32;
    var phi_100_: bool;
    var local_33: f32;
    var local_34: i32;
    var phi_101_: bool;
    var phi_102_: bool;
    var phi_103_: f32;
    var phi_104_: bool;
    var phi_105_: f32;
    var phi_106_: bool;
    var phi_107_: f32;
    var phi_108_: bool;
    var phi_109_: f32;
    var phi_110_: bool;
    var phi_111_: f32;
    var phi_112_: bool;
    var phi_113_: u32;
    var phi_114_: f32;
    var phi_115_: bool;
    var phi_116_: bool;
    var local_35: f32;
    var phi_117_: f32;
    var phi_118_: f32;
    var phi_119_: bool;
    var phi_120_: f32;
    var phi_121_: bool;
    var phi_122_: f32;
    var phi_123_: bool;
    var phi_124_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e230 = pixel_2;
            let _e231 = _isthmus_instance_index_8;
            let _e233 = arrayLength((&placed_glyphs_1.member));
            let _e243 = pill_1.member[_e231].battery_level;
            let _e244 = (_e243 >= -1f);
            if _e244 {
                phi_0_ = (_e243 <= 1f);
            } else {
                phi_0_ = false;
            }
            let _e247 = phi_0_;
            let _e249 = (select(0f, 40f, _e247) + 296f);
            let _e254 = frame.member[0u].screen_size[0u];
            let _e256 = ((_e254 - _e249) - 8f);
            let _e260 = frame.member[0u].panel_height;
            let _e261 = (_e230.x - _e256);
            let _e262 = (_e230.y - 6f);
            let _e263 = (_e249 * 0.5f);
            let _e264 = (_e260 * 0.5f);
            let _e268 = ((_e249 - _e260) * 0.5f);
            let _e270 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e261 - _e263), (_e262 - _e264)), _e268, _e264);
            let _e274 = frame.member[0u].mouse_pressure;
            let _e275 = (_e274 > 0f);
            if _e275 {
                let _e280 = frame.member[0u].mouse_pos[0u];
                let _e285 = frame.member[0u].mouse_pos[1u];
                let _e291 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e280 - _e256) - _e263), ((_e285 - 6f) - _e264)), _e268, _e264);
                phi_1_ = _e291;
            } else {
                phi_1_ = 1f;
            }
            let _e293 = phi_1_;
            phi_2_ = vec2<f32>(0f, 0f);
            phi_3_ = 0f;
            phi_4_ = 0u;
            loop {
                let _e295 = phi_2_;
                let _e297 = phi_3_;
                let _e299 = phi_4_;
                local_24 = _e295;
                local_25 = _e295;
                local_26 = _e295;
                local_27 = _e295;
                local_30 = _e297;
                local_31 = _e297;
                let _e300 = (_e299 < 4u);
                if _e300 {
                    if _e300 {
                    } else {
                        phi_14_ = true;
                        break;
                    }
                    let _e307 = frame.member[0u].ripples[_e299].origin[0u];
                    let _e314 = frame.member[0u].ripples[_e299].origin[1u];
                    let _e320 = frame.member[0u].ripples[_e299].start_time;
                    let _e326 = frame.member[0u].ripples[_e299].strength;
                    let _e330 = frame.member[0u].time;
                    let _e332 = ((_e330 - _e320) * 1.2f);
                    let _e334 = select(_e332, 0f, (_e332 < 0f));
                    let _e336 = select(_e334, 1f, (_e334 > 1f));
                    if (_e326 > 0f) {
                        if (_e336 < 1f) {
                            let _e340 = (_e230 - vec2<f32>(_e307, _e314));
                            let _e346 = sqrt(((_e340.x * _e340.x) + (_e340.y * _e340.y)));
                            if (_e346 > 0.001f) {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e340.x / _e346), (_e340.y / _e346)), _e346);
                            } else {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e346);
                            }
                            let _e354 = phi_5_;
                            let _e364 = ((abs((_e354.unnamed_1 - (_e336 * 600f))) - 80f) * -0.0125f);
                            let _e366 = select(_e364, 0f, (_e364 < 0f));
                            let _e368 = select(_e366, 1f, (_e366 > 1f));
                            let _e374 = (1f - _e336);
                            let _e375 = ((((_e368 * _e368) * (3f - (2f * _e368))) * _e326) * _e374);
                            let _e388 = (_e297 + (_e375 * 0.5f));
                            if (_e388 != _e388) {
                                phi_6_ = true;
                            } else {
                                phi_6_ = (1f <= _e388);
                            }
                            let _e392 = phi_6_;
                            phi_7_ = vec2<f32>((_e295.x + (((_e354.unnamed.x * _e375) * _e374) * 0.5f)), (_e295.y + (((_e354.unnamed.y * _e375) * _e374) * 0.5f)));
                            phi_8_ = select(_e388, 1f, _e392);
                        } else {
                            phi_7_ = _e295;
                            phi_8_ = _e297;
                        }
                        let _e395 = phi_7_;
                        let _e397 = phi_8_;
                        phi_9_ = _e395;
                        phi_10_ = _e397;
                    } else {
                        phi_9_ = _e295;
                        phi_10_ = _e297;
                    }
                    let _e399 = phi_9_;
                    let _e401 = phi_10_;
                    phi_11_ = _e399;
                    phi_12_ = _e401;
                    phi_13_ = (_e299 + 1u);
                } else {
                    phi_11_ = vec2<f32>();
                    phi_12_ = f32();
                    phi_13_ = u32();
                }
                let _e404 = phi_11_;
                let _e406 = phi_12_;
                let _e408 = phi_13_;
                continue;
                continuing {
                    phi_2_ = _e404;
                    phi_3_ = _e406;
                    phi_4_ = _e408;
                    phi_14_ = false;
                    break if !(_e300);
                }
            }
            let _e411 = phi_14_;
            if _e411 {
                break;
            }
            if _e275 {
                let _e416 = frame.member[0u].mouse_pos[0u];
                let _e421 = frame.member[0u].mouse_pos[1u];
                let _e422 = (_e230.x - _e416);
                let _e423 = (_e230.y - _e421);
                let _e429 = ((sqrt(((_e422 * _e422) + (_e423 * _e423))) - 150f) * -0.006666667f);
                let _e431 = select(_e429, 0f, (_e429 < 0f));
                let _e433 = select(_e431, 1f, (_e431 > 1f));
                phi_15_ = ((((_e433 * _e433) * (3f - (2f * _e433))) * _e274) * 8f);
            } else {
                phi_15_ = 0f;
            }
            let _e441 = phi_15_;
            let _e443 = local_24;
            let _e446 = global[0u];
            if (_e443.x == _e446) {
                let _e449 = local_25;
                let _e452 = global[1u];
                phi_16_ = (_e449.y == _e452);
            } else {
                phi_16_ = false;
            }
            let _e455 = phi_16_;
            if _e455 {
                phi_17_ = 0f;
            } else {
                let _e457 = local_26;
                phi_17_ = (sqrt(((_e443.x * _e443.x) + (_e457.y * _e457.y))) * 22f);
            }
            let _e465 = phi_17_;
            let _e467 = local_27;
            let _e470 = ((_e293 - 0.5f) * -1f);
            let _e472 = select(_e470, 0f, (_e470 < 0f));
            let _e474 = select(_e472, 1f, (_e472 > 1f));
            let _e482 = (_e270 - (((_e441 * ((_e474 * _e474) * (3f - (2f * _e474)))) + _e465) * 0.5f));
            let _e483 = fwidth(_e482);
            if (_e483 != _e483) {
                phi_18_ = true;
            } else {
                phi_18_ = (0.55f >= _e483);
            }
            let _e487 = phi_18_;
            let _e488 = select(_e483, 0.55f, _e487);
            let _e492 = ((_e482 - _e488) / (-(_e488) - _e488));
            let _e494 = select(_e492, 0f, (_e492 < 0f));
            let _e496 = select(_e494, 1f, (_e494 > 1f));
            let _e500 = ((_e496 * _e496) * (3f - (2f * _e496)));
            let _e501 = (_e482 != _e482);
            if _e501 {
                phi_19_ = true;
            } else {
                phi_19_ = (0f >= _e482);
            }
            let _e504 = phi_19_;
            let _e508 = (exp((select(_e482, 0f, _e504) * -0.3f)) * 0.16f);
            if (_e500 != _e500) {
                phi_20_ = true;
            } else {
                phi_20_ = (_e508 >= _e500);
            }
            let _e512 = phi_20_;
            let _e513 = select(_e500, _e508, _e512);
            if (_e513 <= 0.0009765625f) {
                discard;
            }
            let _e515 = (_e261 / _e249);
            let _e516 = (_e262 / _e260);
            if _e501 {
                phi_21_ = true;
            } else {
                phi_21_ = (0f <= _e482);
            }
            let _e521 = phi_21_;
            let _e524 = (1f + (select(_e482, 0f, _e521) * 0.008333334f));
            let _e526 = select(_e524, 0f, (_e524 < 0f));
            let _e528 = select(_e526, 0.6f, (_e526 > 0.6f));
            let _e538 = ((_e516 - (((_e516 - 0.5f) * _e528) * 0.08f)) - (_e467.y * 0.04f));
            let _e542 = pill_1.member[_e231].sun_height;
            let _e546 = pill_1.member[_e231].conditions;
            let _e550 = frame.member[0u].time;
            let _e558 = ((_e538 - 1f) * -1f);
            let _e560 = select(_e558, 0f, (_e558 < 0f));
            let _e562 = select(_e560, 1f, (_e560 > 1f));
            let _e566 = ((_e562 * _e562) * (3f - (2f * _e562)));
            let _e568 = ((_e542 - -0.04f) * 4.1666665f);
            let _e570 = select(_e568, 0f, (_e568 < 0f));
            let _e572 = select(_e570, 1f, (_e570 > 1f));
            let _e576 = ((_e572 * _e572) * (3f - (2f * _e572)));
            let _e578 = ((_e542 - -0.2f) * 4.5454545f);
            let _e580 = select(_e578, 0f, (_e578 < 0f));
            let _e582 = select(_e580, 1f, (_e580 > 1f));
            let _e587 = (1f - _e576);
            let _e588 = (((_e582 * _e582) * (3f - (2f * _e582))) * _e587);
            let _e589 = (1f - _e566);
            let _e601 = (0.65f * _e589);
            let _e625 = (1f - _e588);
            let _e639 = (((_e546.cloud * 0.34f) + (_e546.rain * 0.16f)) + (_e546.hail * 0.08f));
            let _e640 = (1f - _e639);
            let _e651 = (1f - (_e546.snow * 0.16f));
            let _e655 = (_e546.snow * 0.1312f);
            let _e660 = (1f - (_e546.fog * 0.62f));
            let _e673 = ((sin((_e550 * 2.7f)) - 0.92f) * 12.500003f);
            let _e675 = select(_e673, 0f, (_e673 < 0f));
            let _e677 = select(_e675, 1f, (_e675 > 1f));
            let _e682 = (((_e677 * _e677) * (3f - (2f * _e677))) * _e546.lightning);
            let _e684 = (1f - (_e682 * 0.45f));
            let _e695 = ((_e538 - 0.12f) * -8.333334f);
            let _e697 = select(_e695, 0f, (_e695 < 0f));
            let _e699 = select(_e697, 1f, (_e697 > 1f));
            let _e706 = ((_e482 - 5f) * -0.125f);
            let _e708 = select(_e706, 0f, (_e706 < 0f));
            let _e710 = select(_e708, 1f, (_e708 > 1f));
            let _e716 = ((((_e699 * _e699) * (3f - (2f * _e699))) * 0.12f) + (((_e710 * _e710) * (3f - (2f * _e710))) * 0.08f));
            let _e720 = (((_e515 - (((_e515 - 0.5f) * _e528) * 0.08f)) - (_e443.x * 0.04f)) * _e249);
            let _e721 = (_e538 * _e260);
            if (_e720 < 96f) {
                phi_29_ = 0u;
            } else {
                if (_e720 < 184f) {
                    phi_28_ = 1u;
                } else {
                    if _e244 {
                        phi_22_ = (_e243 <= 1f);
                    } else {
                        phi_22_ = false;
                    }
                    let _e726 = phi_22_;
                    if _e726 {
                        phi_23_ = select(true, false, (_e720 < 224f));
                    } else {
                        phi_23_ = true;
                    }
                    let _e730 = phi_23_;
                    if _e730 {
                        if _e244 {
                            phi_24_ = (_e243 <= 1f);
                        } else {
                            phi_24_ = false;
                        }
                        let _e733 = phi_24_;
                        if (_e720 < (select(0f, 40f, _e733) + 224f)) {
                            phi_26_ = 3u;
                        } else {
                            if _e244 {
                                phi_25_ = (_e243 <= 1f);
                            } else {
                                phi_25_ = false;
                            }
                            let _e739 = phi_25_;
                            phi_26_ = select(5u, 4u, (_e720 < (select(0f, 40f, _e739) + 256f)));
                        }
                        let _e745 = phi_26_;
                        phi_27_ = _e745;
                    } else {
                        phi_27_ = 2u;
                    }
                    let _e747 = phi_27_;
                    phi_28_ = _e747;
                }
                let _e749 = phi_28_;
                phi_29_ = _e749;
            }
            let _e751 = phi_29_;
            if _e244 {
                phi_30_ = (_e243 <= 1f);
            } else {
                phi_30_ = false;
            }
            let _e754 = phi_30_;
            let _e755 = select(0f, 40f, _e754);
            switch bitcast<i32>(_e751) {
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
                    phi_31_ = (188f + _e755);
                    break;
                }
                case 4: {
                    phi_31_ = (228f + _e755);
                    break;
                }
                case 5: {
                    phi_31_ = (260f + _e755);
                    break;
                }
                default: {
                    phi_31_ = f32();
                    break;
                }
            }
            let _e761 = phi_31_;
            switch bitcast<i32>(_e751) {
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
            let _e764 = phi_32_;
            let _e766 = phi_33_;
            let _e768 = phi_34_;
            let _e769 = select(_e766, false, _e764);
            let _e776 = (_e720 - (_e761 + (select(select(80f, 32f, _e769), 24f, select(select(_e768, false, _e764), false, _e769)) * 0.5f)));
            let _e777 = (_e721 - _e264);
            switch bitcast<i32>(_e751) {
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
            let _e780 = phi_35_;
            let _e782 = phi_36_;
            if _e782 {
                let _e783 = (_e720 - 52f);
                let _e788 = pill_1.member[_e231].cpu.temperature;
                if (_e788 <= 62f) {
                    phi_45_ = vec2<f32>(0f, 0f);
                } else {
                    let _e791 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e783, _e777), 13f, 13f);
                    phi_37_ = 0i;
                    phi_38_ = 0.5f;
                    phi_39_ = 0f;
                    phi_40_ = vec2<f32>(((_e783 + (_e550 * 1.8f)) * 0.035f), (((_e777 + -(_e550)) * 0.035f) + 6.1f));
                    loop {
                        let _e801 = phi_37_;
                        let _e803 = phi_38_;
                        let _e805 = phi_39_;
                        let _e807 = phi_40_;
                        local_28 = _e805;
                        let _e808 = (_e801 < 4i);
                        if _e808 {
                            let _e811 = cantus_render_shader_simplex_noise(_e807);
                            phi_41_ = (_e801 + 1i);
                            phi_42_ = (_e803 * 0.5f);
                            phi_43_ = (_e805 + (_e811 * _e803));
                            phi_44_ = vec2<f32>(((_e807.x * 1.6f) + (_e807.y * 1.2f)), ((_e807.y * 1.6f) - (_e807.x * 1.2f)));
                        } else {
                            phi_41_ = i32();
                            phi_42_ = f32();
                            phi_43_ = f32();
                            phi_44_ = vec2<f32>();
                        }
                        let _e824 = phi_41_;
                        let _e826 = phi_42_;
                        let _e828 = phi_43_;
                        let _e830 = phi_44_;
                        continue;
                        continuing {
                            phi_37_ = _e824;
                            phi_38_ = _e826;
                            phi_39_ = _e828;
                            phi_40_ = _e830;
                            break if !(_e808);
                        }
                    }
                    let _e833 = local_28;
                    let _e834 = (_e833 * 0.5f);
                    let _e837 = ((_e791 - -0.5f) * 0.5f);
                    let _e839 = select(_e837, 0f, (_e837 < 0f));
                    let _e841 = select(_e839, 1f, (_e839 > 1f));
                    let _e847 = ((_e791 - 14f) * -0.083333336f);
                    let _e849 = select(_e847, 0f, (_e847 < 0f));
                    let _e851 = select(_e849, 1f, (_e849 > 1f));
                    let _e856 = (((_e841 * _e841) * (3f - (2f * _e841))) * ((_e851 * _e851) * (3f - (2f * _e851))));
                    let _e861 = ((_e834 + 0.19999999f) * 3.125f);
                    let _e863 = select(_e861, 0f, (_e861 < 0f));
                    let _e865 = select(_e863, 1f, (_e863 > 1f));
                    let _e872 = ((_e788 - 62f) * 0.045454547f);
                    let _e874 = select(_e872, 0f, (_e872 < 0f));
                    let _e876 = select(_e874, 1f, (_e874 > 1f));
                    let _e880 = ((_e876 * _e876) * (3f - (2f * _e876)));
                    phi_45_ = vec2<f32>(((_e856 * (0.18f + ((0.5f + _e834) * 0.34f))) * _e880), ((_e856 * ((_e865 * _e865) * (3f - (2f * _e865)))) * _e880));
                }
                let _e885 = phi_45_;
                let _e888 = (_e720 - 140f);
                let _e893 = pill_1.member[_e231].gpu.temperature;
                if (_e893 <= 62f) {
                    phi_54_ = vec2<f32>(0f, 0f);
                } else {
                    let _e896 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e888, _e777), 13f, 13f);
                    phi_46_ = 0i;
                    phi_47_ = 0.5f;
                    phi_48_ = 0f;
                    phi_49_ = vec2<f32>(((_e888 + (_e550 * 1.8f)) * 0.035f), (((_e777 + -(_e550)) * 0.035f) + 6.1f));
                    loop {
                        let _e906 = phi_46_;
                        let _e908 = phi_47_;
                        let _e910 = phi_48_;
                        let _e912 = phi_49_;
                        local_29 = _e910;
                        let _e913 = (_e906 < 4i);
                        if _e913 {
                            let _e916 = cantus_render_shader_simplex_noise(_e912);
                            phi_50_ = (_e906 + 1i);
                            phi_51_ = (_e908 * 0.5f);
                            phi_52_ = (_e910 + (_e916 * _e908));
                            phi_53_ = vec2<f32>(((_e912.x * 1.6f) + (_e912.y * 1.2f)), ((_e912.y * 1.6f) - (_e912.x * 1.2f)));
                        } else {
                            phi_50_ = i32();
                            phi_51_ = f32();
                            phi_52_ = f32();
                            phi_53_ = vec2<f32>();
                        }
                        let _e929 = phi_50_;
                        let _e931 = phi_51_;
                        let _e933 = phi_52_;
                        let _e935 = phi_53_;
                        continue;
                        continuing {
                            phi_46_ = _e929;
                            phi_47_ = _e931;
                            phi_48_ = _e933;
                            phi_49_ = _e935;
                            break if !(_e913);
                        }
                    }
                    let _e938 = local_29;
                    let _e939 = (_e938 * 0.5f);
                    let _e942 = ((_e896 - -0.5f) * 0.5f);
                    let _e944 = select(_e942, 0f, (_e942 < 0f));
                    let _e946 = select(_e944, 1f, (_e944 > 1f));
                    let _e952 = ((_e896 - 14f) * -0.083333336f);
                    let _e954 = select(_e952, 0f, (_e952 < 0f));
                    let _e956 = select(_e954, 1f, (_e954 > 1f));
                    let _e961 = (((_e946 * _e946) * (3f - (2f * _e946))) * ((_e956 * _e956) * (3f - (2f * _e956))));
                    let _e966 = ((_e939 + 0.19999999f) * 3.125f);
                    let _e968 = select(_e966, 0f, (_e966 < 0f));
                    let _e970 = select(_e968, 1f, (_e968 > 1f));
                    let _e977 = ((_e893 - 62f) * 0.045454547f);
                    let _e979 = select(_e977, 0f, (_e977 < 0f));
                    let _e981 = select(_e979, 1f, (_e979 > 1f));
                    let _e985 = ((_e981 * _e981) * (3f - (2f * _e981)));
                    phi_54_ = vec2<f32>(((_e961 * (0.18f + ((0.5f + _e939) * 0.34f))) * _e985), ((_e961 * ((_e970 * _e970) * (3f - (2f * _e970)))) * _e985));
                }
                let _e990 = phi_54_;
                phi_55_ = vec2<f32>(select(_e990.x, _e885.x, (_e885.x > _e990.x)), select(_e990.y, _e885.y, (_e885.y > _e990.y)));
            } else {
                phi_55_ = _e780;
            }
            let _e999 = phi_55_;
            let _e1004 = pill_1.member[_e231].cpu.temperature;
            let _e1009 = pill_1.member[_e231].gpu.temperature;
            if (_e1004 != _e1004) {
                phi_56_ = true;
            } else {
                phi_56_ = (_e1009 >= _e1004);
            }
            let _e1013 = phi_56_;
            let _e1014 = select(_e1004, _e1009, _e1013);
            let _e1016 = ((_e1014 - 60f) * 0.083333336f);
            let _e1018 = select(_e1016, 0f, (_e1016 < 0f));
            let _e1020 = select(_e1018, 1f, (_e1018 > 1f));
            let _e1024 = ((_e1020 * _e1020) * (3f - (2f * _e1020)));
            let _e1025 = (1f - _e1024);
            let _e1034 = ((_e1014 - 72f) * 0.0625f);
            let _e1036 = select(_e1034, 0f, (_e1034 < 0f));
            let _e1038 = select(_e1036, 1f, (_e1036 > 1f));
            let _e1042 = ((_e1038 * _e1038) * (3f - (2f * _e1038)));
            let _e1043 = (1f - _e1042);
            let _e1053 = (_e999.y * 0.12f);
            let _e1054 = (0.24f + _e1053);
            let _e1055 = (0.76f - _e1053);
            let _e1067 = (1f - (_e999.x * 0.46f));
            let _e1077 = (_e999.y * 0.64f);
            let _e1078 = (1f - _e1077);
            let _e1085 = (((((((((((((((((((0.008f * _e589) + (0.03f * _e566)) * _e587) + (((0.09f * _e589) + (0.34f * _e566)) * _e576)) * _e625) + ((_e601 + (0.3f * _e566)) * _e588)) * _e640) + (0.16f * _e639)) * _e651) + _e655) * _e660) + (_e546.fog * 0.3844f)) * _e684) + (_e682 * 0.2925f)) + _e716) * _e1067) + (_e999.x * 0.0009200001f)) * _e1078) + (((0.07f * _e1055) + (((((0.22f * _e1025) + _e1024) * _e1043) + _e1042) * _e1054)) * _e1077));
            let _e1086 = (((((((((((((((((((0.015f * _e589) + (0.06f * _e566)) * _e587) + (((0.37f * _e589) + (0.7f * _e566)) * _e576)) * _e625) + (((0.25f * _e589) + (0.2f * _e566)) * _e588)) * _e640) + (0.2f * _e639)) * _e651) + _e655) * _e660) + (_e546.fog * 0.4216f)) * _e684) + (_e682 * 0.333f)) + _e716) * _e1067) + (_e999.x * 0.00276f)) * _e1078) + (((0.12f * _e1055) + (((((0.62f * _e1025) + (0.38f * _e1024)) * _e1043) + (0.08f * _e1042)) * _e1054)) * _e1077));
            let _e1087 = (((((((((((((((((((0.04f * _e589) + (0.13f * _e566)) * _e587) + ((_e601 + (0.9f * _e566)) * _e576)) * _e625) + (((0.2f * _e589) + (0.4f * _e566)) * _e588)) * _e640) + (0.27f * _e639)) * _e651) + _e655) * _e660) + (_e546.fog * 0.44640002f)) * _e684) + (_e682 * 0.43199998f)) + _e716) * _e1067) + (_e999.x * 0.00552f)) * _e1078) + (((0.18f * _e1055) + ((((_e1025 + (0.08f * _e1024)) * _e1043) + (0.035f * _e1042)) * _e1054)) * _e1077));
            switch bitcast<i32>(_e751) {
                case 0: {
                    let _e1803 = pill_1.member[_e231].history_scroll;
                    switch bitcast<i32>(_e751) {
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
                    let _e1806 = phi_63_;
                    let _e1808 = phi_64_;
                    let _e1810 = phi_65_;
                    let _e1811 = select(_e1808, false, _e1806);
                    let _e1817 = ((select(select(80f, 32f, _e1811), 24f, select(select(_e1810, false, _e1806), false, _e1811)) * 0.5f) - 4f);
                    let _e1818 = (_e264 - 8f);
                    let _e1819 = (_e1817 - _e1818);
                    let _e1821 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e776, _e777), _e1819, _e1818);
                    let _e1822 = abs(_e776);
                    let _e1823 = abs(_e777);
                    let _e1826 = (round((_e1822 * 0.11111111f)) * 9f);
                    if (_e1826 != _e1826) {
                        phi_66_ = true;
                    } else {
                        phi_66_ = (_e1817 <= _e1826);
                    }
                    let _e1830 = phi_66_;
                    let _e1831 = select(_e1826, _e1817, _e1830);
                    let _e1832 = (_e1831 - _e1819);
                    if (_e1832 != _e1832) {
                        phi_67_ = true;
                    } else {
                        phi_67_ = (0f >= _e1832);
                    }
                    let _e1836 = phi_67_;
                    let _e1837 = select(_e1832, 0f, _e1836);
                    let _e1838 = (_e1818 * _e1818);
                    let _e1841 = sqrt((_e1838 - (_e1837 * _e1837)));
                    let _e1842 = (_e1837 / _e1818);
                    let _e1843 = (_e1841 / _e1818);
                    let _e1848 = ((_e1822 - _e1831) - (_e1842 * 0.9f));
                    let _e1849 = ((_e1823 - _e1841) - (_e1843 * 0.9f));
                    let _e1858 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1848 * -(_e1843)) + (_e1849 * _e1842)), ((_e1848 * _e1842) + (_e1849 * _e1843))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e1860 = round((_e1823 * 0.125f));
                    if (_e1860 != _e1860) {
                        phi_68_ = true;
                    } else {
                        phi_68_ = (1f <= _e1860);
                    }
                    let _e1864 = phi_68_;
                    let _e1866 = (select(_e1860, 1f, _e1864) * 8f);
                    let _e1869 = sqrt((_e1838 - (_e1866 * _e1866)));
                    let _e1871 = (_e1869 / _e1818);
                    let _e1872 = (_e1866 / _e1818);
                    let _e1877 = ((_e1822 - (_e1819 + _e1869)) - (_e1871 * 0.9f));
                    let _e1878 = ((_e1823 - _e1866) - (_e1872 * 0.9f));
                    let _e1887 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1877 * -(_e1872)) + (_e1878 * _e1871)), ((_e1877 * _e1871) + (_e1878 * _e1872))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e1858 != _e1858) {
                        phi_69_ = true;
                    } else {
                        phi_69_ = (_e1887 <= _e1858);
                    }
                    let _e1891 = phi_69_;
                    let _e1892 = select(_e1858, _e1887, _e1891);
                    let _e1895 = (0.5f + ((_e1892 - _e1821) * 0.3125f));
                    let _e1897 = select(_e1895, 0f, (_e1895 < 0f));
                    let _e1899 = select(_e1897, 1f, (_e1897 > 1f));
                    let _e1908 = ((_e1821 - 0.55f) * -0.9090909f);
                    let _e1910 = select(_e1908, 0f, (_e1908 < 0f));
                    let _e1912 = select(_e1910, 1f, (_e1910 > 1f));
                    let _e1916 = ((_e1912 * _e1912) * (3f - (2f * _e1912)));
                    let _e1917 = (_e1817 * 0.051282052f);
                    let _e1918 = (_e776 + _e1817);
                    let _e1920 = ((_e1918 / _e1917) + _e1803);
                    let _e1922 = select(_e1920, 0f, (_e1920 < 0f));
                    let _e1924 = select(_e1922, 39f, (_e1922 > 39f));
                    let _e1925 = floor(_e1924);
                    let _e1930 = select(select(u32(_e1925), 0u, (_e1925 < 0f)), 4294967295u, (_e1925 > 4294967000f));
                    let _e1931 = (_e264 - 10f);
                    let _e1935 = (((f32(_e1930) - _e1803) * _e1917) - _e1817);
                    let _e1937 = select(_e1930, 39u, (39u < _e1930));
                    let _e1938 = (_e1937 < 40u);
                    if _e1938 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1945 = pill_1.member[_e231].cpu.usage.samples[_e1937];
                    let _e1948 = (_e1931 * (1f - (_e1945 * 2f)));
                    let _e1949 = (_e1930 + 1u);
                    let _e1955 = select(_e1949, 39u, (39u < _e1949));
                    let _e1956 = (_e1955 < 40u);
                    if _e1956 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1963 = pill_1.member[_e231].cpu.usage.samples[_e1955];
                    let _e1967 = ((((f32(_e1949) - _e1803) * _e1917) - _e1817) - _e1935);
                    let _e1968 = ((_e1931 * (1f - (_e1963 * 2f))) - _e1948);
                    let _e1969 = (_e776 - _e1935);
                    let _e1970 = (_e777 - _e1948);
                    let _e1971 = (_e1969 * _e1967);
                    let _e1974 = (_e1967 * _e1967);
                    let _e1976 = (_e1974 + (_e1968 * _e1968));
                    if (_e1976 != _e1976) {
                        phi_70_ = true;
                    } else {
                        phi_70_ = (0.001f >= _e1976);
                    }
                    let _e1980 = phi_70_;
                    let _e1982 = ((_e1971 + (_e1970 * _e1968)) / select(_e1976, 0.001f, _e1980));
                    let _e1984 = select(_e1982, 0f, (_e1982 < 0f));
                    let _e1986 = select(_e1984, 1f, (_e1984 > 1f));
                    let _e1989 = (_e1969 - (_e1967 * _e1986));
                    let _e1990 = (_e1970 - (_e1968 * _e1986));
                    let _e1997 = ((abs(sqrt(((_e1989 * _e1989) + (_e1990 * _e1990)))) - 1.4000001f) * -0.9090908f);
                    let _e1999 = select(_e1997, 0f, (_e1997 < 0f));
                    let _e2001 = select(_e1999, 1f, (_e1999 > 1f));
                    let _e2007 = (_e1924 - trunc(_e1924));
                    let _e2009 = select(_e2007, 0f, (_e2007 < 0f));
                    let _e2011 = select(_e2009, 1f, (_e2009 > 1f));
                    let _e2015 = ((_e2011 * _e2011) * (3f - (2f * _e2011)));
                    let _e2022 = ((((_e1948 + (_e1968 * _e2015)) - _e777) - 0.55f) * -0.9090909f);
                    let _e2024 = select(_e2022, 0f, (_e2022 < 0f));
                    let _e2026 = select(_e2024, 1f, (_e2024 > 1f));
                    let _e2032 = ((((_e2026 * _e2026) * (3f - (2f * _e2026))) * 0.156f) + ((_e2001 * _e2001) * (3f - (2f * _e2001))));
                    if _e1938 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e2041 = pill_1.member[_e231].cpu.memory.samples[_e1937];
                    let _e2044 = (_e1931 * (1f - (_e2041 * 2f)));
                    if _e1956 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e2051 = pill_1.member[_e231].cpu.memory.samples[_e1955];
                    let _e2055 = ((_e1931 * (1f - (_e2051 * 2f))) - _e2044);
                    let _e2056 = (_e777 - _e2044);
                    let _e2060 = (_e1974 + (_e2055 * _e2055));
                    if (_e2060 != _e2060) {
                        phi_71_ = true;
                    } else {
                        phi_71_ = (0.001f >= _e2060);
                    }
                    let _e2064 = phi_71_;
                    let _e2066 = ((_e1971 + (_e2056 * _e2055)) / select(_e2060, 0.001f, _e2064));
                    let _e2068 = select(_e2066, 0f, (_e2066 < 0f));
                    let _e2070 = select(_e2068, 1f, (_e2068 > 1f));
                    let _e2073 = (_e1969 - (_e1967 * _e2070));
                    let _e2074 = (_e2056 - (_e2055 * _e2070));
                    let _e2081 = ((abs(sqrt(((_e2073 * _e2073) + (_e2074 * _e2074)))) - 1.4000001f) * -0.9090908f);
                    let _e2083 = select(_e2081, 0f, (_e2081 < 0f));
                    let _e2085 = select(_e2083, 1f, (_e2083 > 1f));
                    let _e2096 = ((((_e2044 + (_e2055 * _e2015)) - _e777) - 0.55f) * -0.9090909f);
                    let _e2098 = select(_e2096, 0f, (_e2096 < 0f));
                    let _e2100 = select(_e2098, 1f, (_e2098 > 1f));
                    let _e2106 = ((((_e2100 * _e2100) * (3f - (2f * _e2100))) * 0.084f) + ((_e2085 * _e2085) * (3f - (2f * _e2085))));
                    let _e2114 = (_e1918 * 0.14285715f);
                    let _e2115 = ((_e777 + _e1818) * 0.16393442f);
                    let _e2125 = ((abs(((_e2114 - trunc(_e2114)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2127 = select(_e2125, 0f, (_e2125 < 0f));
                    let _e2129 = select(_e2127, 1f, (_e2127 > 1f));
                    let _e2133 = ((_e2129 * _e2129) * (3f - (2f * _e2129)));
                    let _e2135 = ((abs(((_e2115 - trunc(_e2115)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2137 = select(_e2135, 0f, (_e2135 < 0f));
                    let _e2139 = select(_e2137, 1f, (_e2137 > 1f));
                    let _e2143 = ((_e2139 * _e2139) * (3f - (2f * _e2139)));
                    if (_e2133 != _e2133) {
                        phi_72_ = true;
                    } else {
                        phi_72_ = (_e2143 >= _e2133);
                    }
                    let _e2147 = phi_72_;
                    let _e2155 = pill_1.member[_e231].cpu.usage.samples[39u];
                    let _e2156 = (_e2155 * 0.24f);
                    let _e2157 = (0.18f + _e2156);
                    let _e2158 = (0.82f - _e2156);
                    let _e2167 = (_e1004 - 60f);
                    let _e2168 = (_e2167 * 0.083333336f);
                    let _e2170 = select(_e2168, 0f, (_e2168 < 0f));
                    let _e2172 = select(_e2170, 1f, (_e2170 > 1f));
                    let _e2176 = ((_e2172 * _e2172) * (3f - (2f * _e2172)));
                    let _e2177 = (1f - _e2176);
                    let _e2186 = ((_e1004 - 72f) * 0.0625f);
                    let _e2188 = select(_e2186, 0f, (_e2186 < 0f));
                    let _e2190 = select(_e2188, 1f, (_e2188 > 1f));
                    let _e2194 = ((_e2190 * _e2190) * (3f - (2f * _e2190)));
                    let _e2195 = (1f - _e2194);
                    let _e2204 = (_e2167 * 0.03846154f);
                    let _e2206 = select(_e2204, 0f, (_e2204 < 0f));
                    let _e2208 = select(_e2206, 1f, (_e2206 > 1f));
                    let _e2213 = (((_e2208 * _e2208) * (3f - (2f * _e2208))) * 0.9f);
                    let _e2214 = (1f - _e2213);
                    let _e2221 = ((((0.025f * _e2158) + (0.32f * _e2157)) * _e2214) + (((((0.22f * _e2177) + _e2176) * _e2195) + _e2194) * _e2213));
                    let _e2222 = ((((0.09f * _e2158) + (0.68f * _e2157)) * _e2214) + (((((0.62f * _e2177) + (0.38f * _e2176)) * _e2195) + (0.08f * _e2194)) * _e2213));
                    let _e2223 = ((((0.15f * _e2158) + _e2157) * _e2214) + ((((_e2177 + (0.08f * _e2176)) * _e2195) + (0.035f * _e2194)) * _e2213));
                    let _e2225 = ((((_e1892 + ((_e1821 - _e1892) * _e1899)) - ((1.6f * _e1899) * (1f - _e1899))) - 0.55f) * -0.9090909f);
                    let _e2227 = select(_e2225, 0f, (_e2225 < 0f));
                    let _e2229 = select(_e2227, 1f, (_e2227 > 1f));
                    let _e2233 = ((_e2229 * _e2229) * (3f - (2f * _e2229)));
                    let _e2235 = (1f - (_e2233 * 0.82f));
                    let _e2247 = ((abs(_e1821) - 2.1f) * -0.909091f);
                    let _e2249 = select(_e2247, 0f, (_e2247 < 0f));
                    let _e2251 = select(_e2249, 1f, (_e2249 > 1f));
                    let _e2256 = (((_e2251 * _e2251) * (3f - (2f * _e2251))) * 0.92f);
                    let _e2257 = (1f - _e2256);
                    let _e2268 = ((_e1892 - 0.55f) * -0.9090909f);
                    let _e2270 = select(_e2268, 0f, (_e2268 < 0f));
                    let _e2272 = select(_e2270, 1f, (_e2270 > 1f));
                    let _e2277 = (((_e2272 * _e2272) * (3f - (2f * _e2272))) * 0.78f);
                    let _e2278 = (1f - _e2277);
                    let _e2289 = ((_e1916 * select(_e2133, _e2143, _e2147)) * 0.045f);
                    phi_73_ = _e411;
                    phi_74_ = vec3<f32>(((((((((_e1085 * _e2235) + (_e2233 * 0.00328f)) * _e2257) + (_e2221 * _e2256)) * _e2278) + (_e2221 * _e2277)) + _e2289) + (((0.32f * _e1916) * _e2032) + ((0.78f * _e1916) * _e2106))), ((((((((_e1086 * _e2235) + (_e2233 * 0.00984f)) * _e2257) + (_e2222 * _e2256)) * _e2278) + (_e2222 * _e2277)) + _e2289) + (((0.68f * _e1916) * _e2032) + ((0.3f * _e1916) * _e2106))), ((((((((_e1087 * _e2235) + (_e2233 * 0.02132f)) * _e2257) + (_e2223 * _e2256)) * _e2278) + (_e2223 * _e2277)) + _e2289) + (_e1916 * (_e2032 + _e2106))));
                    phi_75_ = false;
                    break;
                }
                case 1: {
                    let _e1422 = pill_1.member[_e231].history_scroll;
                    switch bitcast<i32>(_e751) {
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
                    let _e1425 = phi_57_;
                    let _e1427 = phi_58_;
                    let _e1429 = phi_59_;
                    let _e1430 = select(_e1427, false, _e1425);
                    let _e1436 = ((select(select(80f, 32f, _e1430), 24f, select(select(_e1429, false, _e1425), false, _e1430)) * 0.5f) - 4f);
                    let _e1437 = (_e264 - 8f);
                    let _e1440 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e776, _e777), (_e1436 - _e1437), _e1437);
                    let _e1442 = ((_e1440 - 0.55f) * -0.9090909f);
                    let _e1444 = select(_e1442, 0f, (_e1442 < 0f));
                    let _e1446 = select(_e1444, 1f, (_e1444 > 1f));
                    let _e1450 = ((_e1446 * _e1446) * (3f - (2f * _e1446)));
                    let _e1451 = (_e1436 * 0.051282052f);
                    let _e1452 = (_e776 + _e1436);
                    let _e1454 = ((_e1452 / _e1451) + _e1422);
                    let _e1456 = select(_e1454, 0f, (_e1454 < 0f));
                    let _e1458 = select(_e1456, 39f, (_e1456 > 39f));
                    let _e1459 = floor(_e1458);
                    let _e1464 = select(select(u32(_e1459), 0u, (_e1459 < 0f)), 4294967295u, (_e1459 > 4294967000f));
                    let _e1465 = (_e264 - 10f);
                    let _e1469 = (((f32(_e1464) - _e1422) * _e1451) - _e1436);
                    let _e1471 = select(_e1464, 39u, (39u < _e1464));
                    let _e1472 = (_e1471 < 40u);
                    if _e1472 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1479 = pill_1.member[_e231].gpu.usage.samples[_e1471];
                    let _e1482 = (_e1465 * (1f - (_e1479 * 2f)));
                    let _e1483 = (_e1464 + 1u);
                    let _e1489 = select(_e1483, 39u, (39u < _e1483));
                    let _e1490 = (_e1489 < 40u);
                    if _e1490 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1497 = pill_1.member[_e231].gpu.usage.samples[_e1489];
                    let _e1501 = ((((f32(_e1483) - _e1422) * _e1451) - _e1436) - _e1469);
                    let _e1502 = ((_e1465 * (1f - (_e1497 * 2f))) - _e1482);
                    let _e1503 = (_e776 - _e1469);
                    let _e1504 = (_e777 - _e1482);
                    let _e1505 = (_e1503 * _e1501);
                    let _e1508 = (_e1501 * _e1501);
                    let _e1510 = (_e1508 + (_e1502 * _e1502));
                    if (_e1510 != _e1510) {
                        phi_60_ = true;
                    } else {
                        phi_60_ = (0.001f >= _e1510);
                    }
                    let _e1514 = phi_60_;
                    let _e1516 = ((_e1505 + (_e1504 * _e1502)) / select(_e1510, 0.001f, _e1514));
                    let _e1518 = select(_e1516, 0f, (_e1516 < 0f));
                    let _e1520 = select(_e1518, 1f, (_e1518 > 1f));
                    let _e1523 = (_e1503 - (_e1501 * _e1520));
                    let _e1524 = (_e1504 - (_e1502 * _e1520));
                    let _e1531 = ((abs(sqrt(((_e1523 * _e1523) + (_e1524 * _e1524)))) - 1.4000001f) * -0.9090908f);
                    let _e1533 = select(_e1531, 0f, (_e1531 < 0f));
                    let _e1535 = select(_e1533, 1f, (_e1533 > 1f));
                    let _e1541 = (_e1458 - trunc(_e1458));
                    let _e1543 = select(_e1541, 0f, (_e1541 < 0f));
                    let _e1545 = select(_e1543, 1f, (_e1543 > 1f));
                    let _e1549 = ((_e1545 * _e1545) * (3f - (2f * _e1545)));
                    let _e1556 = ((((_e1482 + (_e1502 * _e1549)) - _e777) - 0.55f) * -0.9090909f);
                    let _e1558 = select(_e1556, 0f, (_e1556 < 0f));
                    let _e1560 = select(_e1558, 1f, (_e1558 > 1f));
                    let _e1566 = ((((_e1560 * _e1560) * (3f - (2f * _e1560))) * 0.156f) + ((_e1535 * _e1535) * (3f - (2f * _e1535))));
                    if _e1472 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1575 = pill_1.member[_e231].gpu.memory.samples[_e1471];
                    let _e1578 = (_e1465 * (1f - (_e1575 * 2f)));
                    if _e1490 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1585 = pill_1.member[_e231].gpu.memory.samples[_e1489];
                    let _e1589 = ((_e1465 * (1f - (_e1585 * 2f))) - _e1578);
                    let _e1590 = (_e777 - _e1578);
                    let _e1594 = (_e1508 + (_e1589 * _e1589));
                    if (_e1594 != _e1594) {
                        phi_61_ = true;
                    } else {
                        phi_61_ = (0.001f >= _e1594);
                    }
                    let _e1598 = phi_61_;
                    let _e1600 = ((_e1505 + (_e1590 * _e1589)) / select(_e1594, 0.001f, _e1598));
                    let _e1602 = select(_e1600, 0f, (_e1600 < 0f));
                    let _e1604 = select(_e1602, 1f, (_e1602 > 1f));
                    let _e1607 = (_e1503 - (_e1501 * _e1604));
                    let _e1608 = (_e1590 - (_e1589 * _e1604));
                    let _e1615 = ((abs(sqrt(((_e1607 * _e1607) + (_e1608 * _e1608)))) - 1.4000001f) * -0.9090908f);
                    let _e1617 = select(_e1615, 0f, (_e1615 < 0f));
                    let _e1619 = select(_e1617, 1f, (_e1617 > 1f));
                    let _e1630 = ((((_e1578 + (_e1589 * _e1549)) - _e777) - 0.55f) * -0.9090909f);
                    let _e1632 = select(_e1630, 0f, (_e1630 < 0f));
                    let _e1634 = select(_e1632, 1f, (_e1632 > 1f));
                    let _e1640 = ((((_e1634 * _e1634) * (3f - (2f * _e1634))) * 0.084f) + ((_e1619 * _e1619) * (3f - (2f * _e1619))));
                    let _e1648 = (_e1452 * 0.14285715f);
                    let _e1649 = ((_e777 + _e1437) * 0.16393442f);
                    let _e1659 = ((abs(((_e1648 - trunc(_e1648)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e1661 = select(_e1659, 0f, (_e1659 < 0f));
                    let _e1663 = select(_e1661, 1f, (_e1661 > 1f));
                    let _e1667 = ((_e1663 * _e1663) * (3f - (2f * _e1663)));
                    let _e1669 = ((abs(((_e1649 - trunc(_e1649)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e1671 = select(_e1669, 0f, (_e1669 < 0f));
                    let _e1673 = select(_e1671, 1f, (_e1671 > 1f));
                    let _e1677 = ((_e1673 * _e1673) * (3f - (2f * _e1673)));
                    if (_e1667 != _e1667) {
                        phi_62_ = true;
                    } else {
                        phi_62_ = (_e1677 >= _e1667);
                    }
                    let _e1681 = phi_62_;
                    let _e1689 = pill_1.member[_e231].gpu.usage.samples[39u];
                    let _e1690 = (_e1689 * 0.24f);
                    let _e1691 = (0.18f + _e1690);
                    let _e1692 = (0.82f - _e1690);
                    let _e1701 = (_e1009 - 60f);
                    let _e1702 = (_e1701 * 0.083333336f);
                    let _e1704 = select(_e1702, 0f, (_e1702 < 0f));
                    let _e1706 = select(_e1704, 1f, (_e1704 > 1f));
                    let _e1710 = ((_e1706 * _e1706) * (3f - (2f * _e1706)));
                    let _e1711 = (1f - _e1710);
                    let _e1720 = ((_e1009 - 72f) * 0.0625f);
                    let _e1722 = select(_e1720, 0f, (_e1720 < 0f));
                    let _e1724 = select(_e1722, 1f, (_e1722 > 1f));
                    let _e1728 = ((_e1724 * _e1724) * (3f - (2f * _e1724)));
                    let _e1729 = (1f - _e1728);
                    let _e1738 = (_e1701 * 0.03846154f);
                    let _e1740 = select(_e1738, 0f, (_e1738 < 0f));
                    let _e1742 = select(_e1740, 1f, (_e1740 > 1f));
                    let _e1747 = (((_e1742 * _e1742) * (3f - (2f * _e1742))) * 0.9f);
                    let _e1748 = (1f - _e1747);
                    let _e1759 = (1f - (_e1450 * 0.82f));
                    let _e1771 = ((abs(_e1440) - 2.1f) * -0.909091f);
                    let _e1773 = select(_e1771, 0f, (_e1771 < 0f));
                    let _e1775 = select(_e1773, 1f, (_e1773 > 1f));
                    let _e1780 = (((_e1775 * _e1775) * (3f - (2f * _e1775))) * 0.92f);
                    let _e1781 = (1f - _e1780);
                    let _e1792 = ((_e1450 * select(_e1667, _e1677, _e1681)) * 0.045f);
                    phi_73_ = _e411;
                    phi_74_ = vec3<f32>(((((((_e1085 * _e1759) + (_e1450 * 0.00328f)) * _e1781) + (((((0.025f * _e1692) + (0.32f * _e1691)) * _e1748) + (((((0.22f * _e1711) + _e1710) * _e1729) + _e1728) * _e1747)) * _e1780)) + _e1792) + (((0.32f * _e1450) * _e1566) + ((0.78f * _e1450) * _e1640))), ((((((_e1086 * _e1759) + (_e1450 * 0.00984f)) * _e1781) + (((((0.09f * _e1692) + (0.68f * _e1691)) * _e1748) + (((((0.62f * _e1711) + (0.38f * _e1710)) * _e1729) + (0.08f * _e1728)) * _e1747)) * _e1780)) + _e1792) + (((0.68f * _e1450) * _e1566) + ((0.3f * _e1450) * _e1640))), ((((((_e1087 * _e1759) + (_e1450 * 0.02132f)) * _e1781) + (((((0.15f * _e1692) + _e1691) * _e1748) + ((((_e1711 + (0.08f * _e1710)) * _e1729) + (0.035f * _e1728)) * _e1747)) * _e1780)) + _e1792) + (_e1450 * (_e1566 + _e1640))));
                    phi_75_ = false;
                    break;
                }
                case 2: {
                    let _e1214 = (_e776 * 1.25f);
                    let _e1215 = (_e777 * 1.25f);
                    let _e1217 = select(0f, 1f, (_e243 < 0f));
                    let _e1218 = abs(_e243);
                    let _e1219 = (_e1215 - 1f);
                    let _e1220 = vec2<f32>(_e1214, _e1219);
                    let _e1221 = cantus_render_shader_sd_rounded_box(_e1220, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e1224 = ((abs(_e1221) - 2.425f) * -0.909091f);
                    let _e1226 = select(_e1224, 0f, (_e1224 < 0f));
                    let _e1228 = select(_e1226, 1f, (_e1226 > 1f));
                    let _e1235 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1214, (_e1215 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e1237 = ((_e1235 - 0.55f) * -0.9090909f);
                    let _e1239 = select(_e1237, 0f, (_e1237 < 0f));
                    let _e1241 = select(_e1239, 1f, (_e1239 > 1f));
                    let _e1246 = cantus_render_shader_sd_rounded_box(_e1220, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e1248 = ((_e1246 - 0.55f) * -0.9090909f);
                    let _e1250 = select(_e1248, 0f, (_e1248 < 0f));
                    let _e1252 = select(_e1250, 1f, (_e1250 > 1f));
                    let _e1256 = ((_e1252 * _e1252) * (3f - (2f * _e1252)));
                    let _e1258 = select(_e1218, 0f, (_e1218 < 0f));
                    let _e1276 = ((12f - (select(_e1258, 1f, (_e1258 > 1f)) * 24f)) + ((sin(((_e776 * 0.775f) + (_e550 * (1.4f + (_e1217 * 1.2f))))) * 1.15f) + (sin(((_e776 * 0.3375f) - (_e550 * 0.8f))) * 0.45f)));
                    let _e1277 = (_e1276 - 0.7f);
                    let _e1281 = ((_e1219 - _e1277) / ((_e1276 + 0.7f) - _e1277));
                    let _e1283 = select(_e1281, 0f, (_e1281 < 0f));
                    let _e1285 = select(_e1283, 1f, (_e1283 > 1f));
                    let _e1290 = (_e1256 * ((_e1285 * _e1285) * (3f - (2f * _e1285))));
                    let _e1292 = ((_e1218 - 0.08f) * 5f);
                    let _e1294 = select(_e1292, 0f, (_e1292 < 0f));
                    let _e1296 = select(_e1294, 1f, (_e1294 > 1f));
                    let _e1300 = ((_e1296 * _e1296) * (3f - (2f * _e1296)));
                    let _e1301 = (1f - _e1300);
                    let _e1309 = ((_e1218 - 0.18f) * 1.8518518f);
                    let _e1311 = select(_e1309, 0f, (_e1309 < 0f));
                    let _e1313 = select(_e1311, 1f, (_e1311 > 1f));
                    let _e1317 = ((_e1313 * _e1313) * (3f - (2f * _e1313)));
                    let _e1318 = (1f - _e1317);
                    let _e1324 = (_e1318 + (0.22f * _e1317));
                    let _e1325 = ((((0.18f * _e1301) + (0.72f * _e1300)) * _e1318) + (0.95f * _e1317));
                    let _e1326 = ((((0.1f * _e1301) + (0.12f * _e1300)) * _e1318) + (0.55f * _e1317));
                    let _e1328 = floor((_e776 * 0.4166667f));
                    let _e1330 = cantus_render_shader_hash(vec2<f32>(_e1328, 0f));
                    let _e1333 = (_e1330.y * 0.5f);
                    let _e1337 = ((_e550 * (0.35f + _e1333)) + (_e1330.x * 7f));
                    let _e1339 = (_e1337 - trunc(_e1337));
                    let _e1346 = (_e1214 - (((_e1328 + 0.2f) + (_e1330.x * 0.6f)) * 3f));
                    let _e1347 = (_e1215 - (13f - (_e1339 * 24f)));
                    let _e1354 = (_e1339 * 4f);
                    let _e1356 = select(_e1354, 0f, (_e1354 < 0f));
                    let _e1358 = select(_e1356, 1f, (_e1356 > 1f));
                    let _e1364 = ((_e1339 - 1f) * -3.3333333f);
                    let _e1366 = select(_e1364, 0f, (_e1364 < 0f));
                    let _e1368 = select(_e1366, 1f, (_e1366 > 1f));
                    let _e1376 = ((abs((sqrt(((_e1346 * _e1346) + (_e1347 * _e1347))) - (0.4f + _e1333))) - 1f) * -0.9090909f);
                    let _e1378 = select(_e1376, 0f, (_e1376 < 0f));
                    let _e1380 = select(_e1378, 1f, (_e1378 > 1f));
                    let _e1387 = (((((_e1380 * _e1380) * (3f - (2f * _e1380))) * (((_e1358 * _e1358) * (3f - (2f * _e1358))) * ((_e1368 * _e1368) * (3f - (2f * _e1368))))) * _e1256) * _e1217);
                    let _e1390 = ((((_e1228 * _e1228) * (3f - (2f * _e1228))) * 0.43f) + (((_e1241 * _e1241) * (3f - (2f * _e1241))) * 0.38f));
                    phi_73_ = _e411;
                    phi_74_ = vec3<f32>((_e1085 + ((_e1390 + ((_e1324 * _e1290) * 0.78f)) + ((((_e1324 * 0.27999997f) + 0.72f) * _e1387) * 0.9f))), (_e1086 + ((_e1390 + ((_e1325 * _e1290) * 0.78f)) + ((((_e1325 * 0.27999997f) + 0.72f) * _e1387) * 0.9f))), (_e1087 + ((_e1390 + ((_e1326 * _e1290) * 0.78f)) + ((((_e1326 * 0.27999997f) + 0.72f) * _e1387) * 0.9f))));
                    phi_75_ = false;
                    break;
                }
                case 3: {
                    let _e1092 = pill_1.member[_e231].volume;
                    let _e1094 = select(0f, 1f, (_e1092 < 0f));
                    let _e1095 = abs(_e1092);
                    let _e1098 = round(((_e776 + 12f) * 0.25f));
                    let _e1100 = select(_e1098, 0f, (_e1098 < 0f));
                    let _e1102 = select(_e1100, 6f, (_e1100 > 6f));
                    let _e1107 = select(select(u32(_e1102), 0u, (_e1102 < 0f)), 4294967295u, (_e1102 > 4294967000f));
                    if (_e1107 < 7u) {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1113 = pill_1.member[_e231].audio_spectrum[_e1107];
                    let _e1114 = (1f - _e1094);
                    let _e1115 = (_e1113 * _e1114);
                    let _e1124 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e776 - (-12f + (_e1102 * 4f))), (_e777 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1115))), 1.25f);
                    let _e1127 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e776, (_e777 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1129 = ((_e1127 - 0.55f) * -0.9090909f);
                    let _e1131 = select(_e1129, 0f, (_e1129 < 0f));
                    let _e1133 = select(_e1131, 1f, (_e1131 > 1f));
                    let _e1137 = ((_e1133 * _e1133) * (3f - (2f * _e1133)));
                    let _e1139 = select(_e1095, 0f, (_e1095 < 0f));
                    let _e1142 = (select(_e1139, 1f, (_e1139 > 1f)) * 28f);
                    let _e1143 = (_e1142 + -13.2f);
                    let _e1147 = ((_e776 - _e1143) / ((_e1142 + -14.8f) - _e1143));
                    let _e1149 = select(_e1147, 0f, (_e1147 < 0f));
                    let _e1151 = select(_e1149, 1f, (_e1149 > 1f));
                    let _e1156 = (_e1137 * ((_e1151 * _e1151) * (3f - (2f * _e1151))));
                    let _e1158 = (1f - (_e1095 * 0.65f));
                    let _e1163 = ((0.08f * _e1158) + (_e1095 * 0.42249995f));
                    let _e1164 = ((0.88f * _e1158) + (_e1095 * 0.221f));
                    let _e1166 = ((_e1124 - 0.7f) * -0.71428573f);
                    let _e1168 = select(_e1166, 0f, (_e1166 < 0f));
                    let _e1170 = select(_e1168, 1f, (_e1168 > 1f));
                    let _e1179 = ((_e1124 - 3.2f) * -0.3125f);
                    let _e1181 = select(_e1179, 0f, (_e1179 < 0f));
                    let _e1183 = select(_e1181, 1f, (_e1181 > 1f));
                    let _e1190 = ((((_e1170 * _e1170) * (3f - (2f * _e1170))) * (0.58f + (_e1115 * 0.35f))) + ((((_e1183 * _e1183) * (3f - (2f * _e1183))) * _e1115) * 0.12f));
                    let _e1203 = (_e1156 + ((_e1137 * (1f - _e1156)) * 0.22f));
                    phi_73_ = _e411;
                    phi_74_ = vec3<f32>((_e1085 + ((_e1163 * _e1190) + (((_e1163 * _e1114) + _e1094) * _e1203))), (_e1086 + ((_e1164 * _e1190) + (((_e1164 * _e1114) + (0.24f * _e1094)) * _e1203))), (_e1087 + (_e1190 + ((_e1114 + (0.3f * _e1094)) * _e1203))));
                    phi_75_ = false;
                    break;
                }
                case 4: {
                    phi_73_ = _e411;
                    phi_74_ = vec3<f32>();
                    phi_75_ = true;
                    break;
                }
                case 5: {
                    phi_73_ = _e411;
                    phi_74_ = vec3<f32>();
                    phi_75_ = true;
                    break;
                }
                default: {
                    phi_73_ = _e411;
                    phi_74_ = vec3<f32>();
                    phi_75_ = bool();
                    break;
                }
            }
            let _e2298 = phi_73_;
            let _e2300 = phi_74_;
            let _e2302 = phi_75_;
            if _e2298 {
                break;
            }
            if _e2302 {
                let _e2304 = select(1f, 0f, (_e751 == 5u));
                let _e2308 = pill_1.member[_e231].power_hover;
                let _e2313 = ((abs((f32(_e2308) - _e2304)) - 0.4f) * -2.857143f);
                let _e2315 = select(_e2313, 0f, (_e2313 < 0f));
                let _e2317 = select(_e2315, 1f, (_e2315 > 1f));
                let _e2321 = ((_e2317 * _e2317) * (3f - (2f * _e2317)));
                let _e2323 = (1f + (_e2321 * 0.07f));
                let _e2324 = (_e776 / _e2323);
                let _e2325 = (_e777 / _e2323);
                let _e2329 = pill_1.member[_e231].power_action;
                let _e2334 = ((abs((f32(_e2329) - _e2304)) - 0.4f) * -2.857143f);
                let _e2336 = select(_e2334, 0f, (_e2334 < 0f));
                let _e2338 = select(_e2336, 1f, (_e2336 > 1f));
                let _e2342 = ((_e2338 * _e2338) * (3f - (2f * _e2338)));
                let _e2346 = pill_1.member[_e231].power_progress;
                let _e2347 = (_e2346 * _e2342);
                if (_e2304 < 0.5f) {
                    let _e2471 = select(_e2347, 0f, (_e2347 < 0f));
                    let _e2473 = select(_e2471, 1f, (_e2471 > 1f));
                    let _e2477 = ((_e2473 * _e2473) * (3f - (2f * _e2473)));
                    let _e2483 = (1f - _e2347);
                    let _e2492 = (_e2477 * 0.7f);
                    let _e2493 = (_e2492 + 1.5999999f);
                    let _e2498 = ((abs((sqrt(((_e2324 * _e2324) + (_e2325 * _e2325))) - ((7.5f - (_e2347 * 4.6f)) + (((sin((_e550 * 8f)) * _e2347) * _e2483) * 0.16f)))) - _e2493) / ((_e2492 + 0.49999994f) - _e2493));
                    let _e2500 = select(_e2498, 0f, (_e2498 < 0f));
                    let _e2502 = select(_e2500, 1f, (_e2500 > 1f));
                    let _e2511 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2324, (_e2325 - -7f)), vec2<f32>((3f * _e2483), 3f), 0.5f);
                    let _e2513 = ((_e2511 - 0.55f) * -0.9090909f);
                    let _e2515 = select(_e2513, 0f, (_e2513 < 0f));
                    let _e2517 = select(_e2515, 1f, (_e2515 > 1f));
                    let _e2531 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2324, (_e2325 - (-5f + (_e2347 * 3.5f)))), vec2<f32>((1.05f + (_e2477 * 0.45f)), (4.6f - (_e2347 * 3f))), 0.7f);
                    let _e2533 = ((_e2531 - 0.55f) * -0.9090909f);
                    let _e2535 = select(_e2533, 0f, (_e2533 < 0f));
                    let _e2537 = select(_e2535, 1f, (_e2535 > 1f));
                    let _e2541 = ((_e2537 * _e2537) * (3f - (2f * _e2537)));
                    let _e2543 = (((_e2502 * _e2502) * (3f - (2f * _e2502))) * (1f - ((_e2517 * _e2517) * (3f - (2f * _e2517)))));
                    if (_e2543 != _e2543) {
                        phi_79_ = true;
                    } else {
                        phi_79_ = (_e2541 >= _e2543);
                    }
                    let _e2547 = phi_79_;
                    phi_80_ = select(_e2543, _e2541, _e2547);
                } else {
                    let _e2350 = ((1f - _e2342) + _e2347);
                    let _e2354 = (((atan2(_e2325, _e2324) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e2358 = ((_e2350 * 0.82f) - 0.045f);
                    if (_e2358 != _e2358) {
                        phi_76_ = true;
                    } else {
                        phi_76_ = (0f >= _e2358);
                    }
                    let _e2362 = phi_76_;
                    let _e2363 = select(_e2358, 0f, _e2362);
                    let _e2371 = ((abs((sqrt(((_e2324 * _e2324) + (_e2325 * _e2325))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e2373 = select(_e2371, 0f, (_e2371 < 0f));
                    let _e2375 = select(_e2373, 1f, (_e2373 > 1f));
                    let _e2380 = (_e2363 + 0.008f);
                    let _e2384 = (((_e2354 - trunc(_e2354)) - _e2380) / ((_e2363 - 0.008f) - _e2380));
                    let _e2386 = select(_e2384, 0f, (_e2384 < 0f));
                    let _e2388 = select(_e2386, 1f, (_e2386 > 1f));
                    let _e2394 = (_e2350 * 50f);
                    let _e2396 = select(_e2394, 0f, (_e2394 < 0f));
                    let _e2398 = select(_e2396, 1f, (_e2396 > 1f));
                    let _e2403 = ((((_e2375 * _e2375) * (3f - (2f * _e2375))) * ((_e2388 * _e2388) * (3f - (2f * _e2388)))) * ((_e2398 * _e2398) * (3f - (2f * _e2398))));
                    let _e2405 = (0.50265485f + (5.152212f * _e2350));
                    let _e2406 = cos(_e2405);
                    let _e2407 = sin(_e2405);
                    let _e2411 = (_e2324 - (_e2406 * 7.1f));
                    let _e2412 = (_e2325 - (_e2407 * 7.1f));
                    let _e2415 = ((_e2411 * -(_e2407)) + (_e2412 * _e2406));
                    let _e2418 = ((_e2411 * _e2406) + (_e2412 * _e2407));
                    let _e2419 = (_e2415 * -3.2f);
                    let _e2422 = ((_e2419 + (_e2418 * 2.1f)) * 0.06825939f);
                    let _e2424 = select(_e2422, 0f, (_e2422 < 0f));
                    let _e2426 = select(_e2424, 1f, (_e2424 > 1f));
                    let _e2429 = (_e2415 - (-3.2f * _e2426));
                    let _e2430 = (_e2418 - (2.1f * _e2426));
                    let _e2434 = sqrt(((_e2429 * _e2429) + (_e2430 * _e2430)));
                    let _e2437 = ((_e2419 + (_e2418 * -2.1f)) * 0.06825939f);
                    let _e2439 = select(_e2437, 0f, (_e2437 < 0f));
                    let _e2441 = select(_e2439, 1f, (_e2439 > 1f));
                    let _e2444 = (_e2415 - (-3.2f * _e2441));
                    let _e2445 = (_e2418 - (-2.1f * _e2441));
                    let _e2449 = sqrt(((_e2444 * _e2444) + (_e2445 * _e2445)));
                    if (_e2434 != _e2434) {
                        phi_77_ = true;
                    } else {
                        phi_77_ = (_e2449 <= _e2434);
                    }
                    let _e2453 = phi_77_;
                    let _e2456 = ((select(_e2434, _e2449, _e2453) - 1.7f) * -0.71428573f);
                    let _e2458 = select(_e2456, 0f, (_e2456 < 0f));
                    let _e2460 = select(_e2458, 1f, (_e2458 > 1f));
                    let _e2464 = ((_e2460 * _e2460) * (3f - (2f * _e2460)));
                    if (_e2403 != _e2403) {
                        phi_78_ = true;
                    } else {
                        phi_78_ = (_e2464 >= _e2403);
                    }
                    let _e2468 = phi_78_;
                    phi_80_ = select(_e2403, _e2464, _e2468);
                }
                let _e2550 = phi_80_;
                let _e2553 = (_e2342 * (0.5f + (_e2347 * 0.5f)));
                if (_e2321 != _e2321) {
                    phi_81_ = true;
                } else {
                    phi_81_ = (_e2553 >= _e2321);
                }
                let _e2557 = phi_81_;
                let _e2558 = select(_e2321, _e2553, _e2557);
                let _e2560 = (0.48f * (1f - _e2558));
                let _e2571 = (1f + (_e2347 * 0.45f));
                phi_82_ = vec3<f32>((_e1085 + (((_e2560 + (0.78f * _e2558)) * _e2550) * _e2571)), (_e1086 + (((_e2560 + (0.3f * _e2558)) * _e2550) * _e2571)), (_e1087 + (((_e2560 + (0.28f * _e2558)) * _e2550) * _e2571)));
            } else {
                phi_82_ = _e2300;
            }
            let _e2580 = phi_82_;
            let _e2582 = local_30;
            let _e2584 = (1f - (_e2582 * 0.35f));
            let _e2592 = local_31;
            let _e2593 = (_e2592 * 0.33249998f);
            switch bitcast<i32>(_e751) {
                case 0: {
                    let _e2607 = pill_1.member[_e231].labels[0u];
                    phi_83_ = _e2607;
                    break;
                }
                case 1: {
                    let _e2602 = pill_1.member[_e231].labels[1u];
                    phi_83_ = _e2602;
                    break;
                }
                default: {
                    phi_83_ = render_text_Line(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), 0f, 0f, 0u, 0u, 0u);
                    break;
                }
            }
            let _e2609 = phi_83_;
            switch bitcast<i32>(_e751) {
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
            let _e2612 = phi_84_;
            if _e2612 {
                if (_e720 < _e2609.min.x) {
                    phi_122_ = f32();
                    phi_123_ = true;
                } else {
                    if (_e720 > _e2609.max.x) {
                        phi_120_ = f32();
                        phi_121_ = true;
                    } else {
                        if (_e721 < _e2609.min.y) {
                            phi_118_ = f32();
                            phi_119_ = true;
                        } else {
                            let _e2624 = (_e721 > _e2609.max.y);
                            if _e2624 {
                                phi_117_ = f32();
                            } else {
                                let _e2627 = (1f / _e2609.size);
                                let _e2634 = ((_e720 - _e2609.origin.x) * _e2627);
                                phi_85_ = _e2609.count;
                                phi_86_ = 0u;
                                loop {
                                    let _e2637 = phi_85_;
                                    let _e2639 = phi_86_;
                                    local_32 = _e2639;
                                    let _e2640 = (_e2639 < _e2637);
                                    if _e2640 {
                                        let _e2643 = (_e2639 + ((_e2637 - _e2639) / 2u));
                                        let _e2645 = (_e2609.first + _e2643);
                                        if (_e2645 < _e233) {
                                        } else {
                                            phi_90_ = true;
                                            break;
                                        }
                                        let _e2650 = placed_glyphs_1.member[_e2645].x;
                                        let _e2651 = (_e2650 <= _e2634);
                                        if _e2651 {
                                            phi_87_ = (_e2643 + 1u);
                                        } else {
                                            phi_87_ = _e2639;
                                        }
                                        let _e2654 = phi_87_;
                                        phi_88_ = select(_e2643, _e2637, _e2651);
                                        phi_89_ = _e2654;
                                    } else {
                                        phi_88_ = u32();
                                        phi_89_ = u32();
                                    }
                                    let _e2657 = phi_88_;
                                    let _e2659 = phi_89_;
                                    continue;
                                    continuing {
                                        phi_85_ = _e2657;
                                        phi_86_ = _e2659;
                                        phi_90_ = _e2298;
                                        break if !(_e2640);
                                    }
                                }
                                let _e2662 = phi_90_;
                                if _e2662 {
                                    break;
                                }
                                let _e2663 = (3.5f / _e2609.size);
                                let _e2665 = local_32;
                                let _e2666 = (_e2665 + 1u);
                                phi_91_ = _e2662;
                                phi_92_ = select(_e2666, _e2609.count, (_e2609.count < _e2666));
                                phi_93_ = -1000000f;
                                loop {
                                    let _e2670 = phi_91_;
                                    let _e2672 = phi_92_;
                                    let _e2674 = phi_93_;
                                    local_35 = _e2674;
                                    if (_e2672 > 0u) {
                                        let _e2676 = (_e2672 - 1u);
                                        let _e2678 = (_e2609.first + _e2676);
                                        if (_e2678 < _e233) {
                                        } else {
                                            phi_116_ = true;
                                            break;
                                        }
                                        let _e2683 = placed_glyphs_1.member[_e2678].x;
                                        let _e2687 = placed_glyphs_1.member[_e2678].glyph;
                                        if (_e2687 < arrayLength((&glyphs_1.member))) {
                                        } else {
                                            phi_116_ = true;
                                            break;
                                        }
                                        let _e2693 = glyphs_1.member[_e2687].min[0u];
                                        let _e2698 = glyphs_1.member[_e2687].min[1u];
                                        let _e2703 = glyphs_1.member[_e2687].max[0u];
                                        let _e2708 = glyphs_1.member[_e2687].max[1u];
                                        let _e2712 = glyphs_1.member[_e2687].start;
                                        let _e2716 = glyphs_1.member[_e2687].count;
                                        let _e2717 = (_e2634 - _e2683);
                                        let _e2718 = -(((_e721 - _e2609.origin.y) * _e2627));
                                        let _e2719 = (_e2703 + _e2663);
                                        let _e2720 = (_e2717 > _e2719);
                                        if _e2720 {
                                            phi_110_ = _e2670;
                                            phi_111_ = f32();
                                        } else {
                                            if (_e2717 >= (_e2693 - _e2663)) {
                                                if (_e2718 >= (_e2698 - _e2663)) {
                                                    if (_e2717 <= _e2719) {
                                                        if (_e2718 <= (_e2708 + _e2663)) {
                                                            phi_94_ = 340282350000000000000000000000000000000f;
                                                            phi_95_ = 0u;
                                                            phi_96_ = 0i;
                                                            loop {
                                                                let _e2730 = phi_94_;
                                                                let _e2732 = phi_95_;
                                                                let _e2734 = phi_96_;
                                                                local_33 = _e2730;
                                                                local_34 = _e2734;
                                                                let _e2735 = (_e2732 < _e2716);
                                                                if _e2735 {
                                                                    let _e2736 = (_e2712 + _e2732);
                                                                    if (_e2736 < arrayLength((&edges_1.member))) {
                                                                    } else {
                                                                        phi_100_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2740 = edges_1.member[_e2736];
                                                                    let _e2742 = cantus_render_text_edge_distance(_e2740, _e2609.weight, vec2<f32>(_e2717, _e2718), _e2730);
                                                                    phi_97_ = _e2742.member;
                                                                    phi_98_ = (_e2732 + 1u);
                                                                    phi_99_ = (_e2734 + _e2742.member_1);
                                                                } else {
                                                                    phi_97_ = f32();
                                                                    phi_98_ = u32();
                                                                    phi_99_ = i32();
                                                                }
                                                                let _e2748 = phi_97_;
                                                                let _e2750 = phi_98_;
                                                                let _e2752 = phi_99_;
                                                                continue;
                                                                continuing {
                                                                    phi_94_ = _e2748;
                                                                    phi_95_ = _e2750;
                                                                    phi_96_ = _e2752;
                                                                    phi_100_ = _e2670;
                                                                    break if !(_e2735);
                                                                }
                                                            }
                                                            let _e2755 = phi_100_;
                                                            phi_116_ = _e2755;
                                                            if _e2755 {
                                                                break;
                                                            }
                                                            let _e2757 = local_33;
                                                            let _e2761 = local_34;
                                                            let _e2764 = ((sqrt(_e2757) * _e2609.size) * select(1f, -1f, (_e2761 == 0i)));
                                                            if (_e2674 != _e2674) {
                                                                phi_101_ = true;
                                                            } else {
                                                                phi_101_ = (_e2764 >= _e2674);
                                                            }
                                                            let _e2768 = phi_101_;
                                                            phi_102_ = _e2755;
                                                            phi_103_ = select(_e2674, _e2764, _e2768);
                                                        } else {
                                                            phi_102_ = _e2670;
                                                            phi_103_ = _e2674;
                                                        }
                                                        let _e2771 = phi_102_;
                                                        let _e2773 = phi_103_;
                                                        phi_104_ = _e2771;
                                                        phi_105_ = _e2773;
                                                    } else {
                                                        phi_104_ = _e2670;
                                                        phi_105_ = _e2674;
                                                    }
                                                    let _e2775 = phi_104_;
                                                    let _e2777 = phi_105_;
                                                    phi_106_ = _e2775;
                                                    phi_107_ = _e2777;
                                                } else {
                                                    phi_106_ = _e2670;
                                                    phi_107_ = _e2674;
                                                }
                                                let _e2779 = phi_106_;
                                                let _e2781 = phi_107_;
                                                phi_108_ = _e2779;
                                                phi_109_ = _e2781;
                                            } else {
                                                phi_108_ = _e2670;
                                                phi_109_ = _e2674;
                                            }
                                            let _e2783 = phi_108_;
                                            let _e2785 = phi_109_;
                                            phi_110_ = _e2783;
                                            phi_111_ = _e2785;
                                        }
                                        let _e2787 = phi_110_;
                                        let _e2789 = phi_111_;
                                        phi_112_ = _e2787;
                                        phi_113_ = _e2676;
                                        phi_114_ = _e2789;
                                        phi_115_ = select(true, false, _e2720);
                                    } else {
                                        phi_112_ = _e2670;
                                        phi_113_ = u32();
                                        phi_114_ = f32();
                                        phi_115_ = false;
                                    }
                                    let _e2792 = phi_112_;
                                    let _e2794 = phi_113_;
                                    let _e2796 = phi_114_;
                                    let _e2798 = phi_115_;
                                    continue;
                                    continuing {
                                        phi_91_ = _e2792;
                                        phi_92_ = _e2794;
                                        phi_93_ = _e2796;
                                        phi_116_ = _e2792;
                                        break if !(_e2798);
                                    }
                                }
                                let _e2801 = phi_116_;
                                if _e2801 {
                                    break;
                                }
                                let _e2803 = local_35;
                                let _e2805 = ((_e2803 * 1.25f) + 0.5f);
                                let _e2807 = select(_e2805, 0f, (_e2805 < 0f));
                                let _e2809 = select(_e2807, 1f, (_e2807 > 1f));
                                phi_117_ = ((_e2809 * _e2809) * (3f - (2f * _e2809)));
                            }
                            let _e2815 = phi_117_;
                            phi_118_ = _e2815;
                            phi_119_ = _e2624;
                        }
                        let _e2817 = phi_118_;
                        let _e2819 = phi_119_;
                        phi_120_ = _e2817;
                        phi_121_ = _e2819;
                    }
                    let _e2821 = phi_120_;
                    let _e2823 = phi_121_;
                    phi_122_ = _e2821;
                    phi_123_ = _e2823;
                }
                let _e2825 = phi_122_;
                let _e2827 = phi_123_;
                phi_124_ = select(_e2825, 0f, _e2827);
            } else {
                phi_124_ = 0f;
            }
            let _e2830 = phi_124_;
            let _e2831 = (1f - _e2830);
            let _e2835 = (0.94f * _e2830);
            out_color = vec4<f32>((((((_e2580.x * _e2584) + _e2593) * _e2831) + _e2835) * _e500), (((((_e2580.y * _e2584) + _e2593) * _e2831) + _e2835) * _e500), (((((_e2580.z * _e2584) + _e2593) * _e2831) + _e2835) * _e500), _e513);
            break;
        }
    }
    return;
}

fn render_playhead_isthmus_playheadpass_vertex_impl() {
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

fn render_playhead_isthmus_playheadpass_fragment_impl() {
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

fn render_particles_isthmus_particlepass_vertex_impl() {
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
    var local_36: f32;

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
                local_36 = _e143;
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
            let _e181 = local_36;
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
    var local_37: vec2<f32>;
    var local_38: vec2<f32>;
    var phi_21_: bool;
    var local_39: vec2<f32>;
    var phi_22_: f32;
    var local_40: vec2<f32>;
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
    var local_41: f32;
    var phi_51_: i32;
    var phi_52_: f32;
    var phi_53_: f32;
    var phi_54_: vec2<f32>;
    var phi_55_: i32;
    var phi_56_: f32;
    var phi_57_: f32;
    var phi_58_: vec2<f32>;
    var local_42: f32;
    var local_43: f32;
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
    var local_44: f32;
    var phi_71_: vec3<f32>;
    var phi_72_: i32;
    var phi_73_: f32;
    var phi_74_: f32;
    var phi_75_: vec2<f32>;
    var phi_76_: i32;
    var phi_77_: f32;
    var phi_78_: f32;
    var phi_79_: vec2<f32>;
    var local_45: f32;
    var phi_80_: f32;
    var phi_81_: vec3<f32>;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var local_49: f32;
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
    var phi_101_: bool;
    var local_50: u32;
    var phi_102_: bool;
    var phi_103_: u32;
    var phi_104_: f32;
    var phi_105_: f32;
    var phi_106_: u32;
    var phi_107_: i32;
    var phi_108_: f32;
    var phi_109_: u32;
    var phi_110_: i32;
    var phi_111_: bool;
    var local_51: f32;
    var local_52: i32;
    var phi_112_: bool;
    var phi_113_: bool;
    var phi_114_: f32;
    var phi_115_: bool;
    var phi_116_: f32;
    var phi_117_: bool;
    var phi_118_: f32;
    var phi_119_: bool;
    var phi_120_: f32;
    var phi_121_: bool;
    var phi_122_: f32;
    var phi_123_: bool;
    var phi_124_: u32;
    var phi_125_: f32;
    var phi_126_: bool;
    var phi_127_: bool;
    var local_53: f32;
    var phi_128_: f32;
    var phi_129_: f32;
    var phi_130_: bool;
    var phi_131_: f32;
    var phi_132_: bool;
    var phi_133_: f32;
    var phi_134_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e217 = pixel_2;
            let _e218 = weather_1;
            let _e219 = _isthmus_instance_index_9;
            let _e223 = arrayLength((&placed_glyphs_2.member));
            let _e236 = pill_2.member[_e219].x;
            let _e240 = frame.member[0u].panel_height;
            let _e241 = (_e217.x - _e236);
            let _e242 = (_e217.y - 6f);
            let _e243 = (_e240 * 0.5f);
            let _e247 = ((308f - _e240) * 0.5f);
            let _e249 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e241 - 154f), (_e242 - _e243)), _e247, _e243);
            let _e253 = frame.member[0u].mouse_pressure;
            let _e254 = (_e253 > 0f);
            if _e254 {
                let _e259 = frame.member[0u].mouse_pos[0u];
                let _e264 = frame.member[0u].mouse_pos[1u];
                let _e270 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e259 - _e236) - 154f), ((_e264 - 6f) - _e243)), _e247, _e243);
                phi_0_ = _e270;
            } else {
                phi_0_ = 1f;
            }
            let _e272 = phi_0_;
            phi_1_ = 0u;
            loop {
                let _e274 = phi_1_;
                let _e275 = (_e274 < 4u);
                if _e275 {
                    if _e275 {
                    } else {
                        phi_3_ = true;
                        break;
                    }
                    phi_2_ = (_e274 + 1u);
                } else {
                    phi_2_ = u32();
                }
                let _e278 = phi_2_;
                continue;
                continuing {
                    phi_1_ = _e278;
                    phi_3_ = false;
                    break if !(_e275);
                }
            }
            let _e281 = phi_3_;
            if _e281 {
                break;
            }
            let _e287 = (_e236 - (_e218.w * 158f));
            let _e288 = (6f + _e240);
            let _e289 = (8f * _e218.w);
            let _e290 = ((244f * _e218.w) - _e289);
            if (_e290 != _e290) {
                phi_4_ = true;
            } else {
                phi_4_ = (0f >= _e290);
            }
            let _e294 = phi_4_;
            let _e297 = (_e217.y - _e288);
            let _e298 = ((308f + (316f * _e218.w)) * 0.5f);
            let _e299 = (select(_e290, 0f, _e294) * 0.5f);
            let _e300 = (_e289 + _e299);
            let _e303 = (_e299 != _e299);
            if _e303 {
                phi_5_ = true;
            } else {
                phi_5_ = (18f <= _e299);
            }
            let _e306 = phi_5_;
            let _e309 = vec2<f32>(_e298, _e299);
            let _e310 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e217.x - _e287) - _e298), (_e297 - _e300)), _e309, select(_e299, 18f, _e306));
            let _e315 = frame.member[0u].mouse_pos[0u];
            let _e320 = frame.member[0u].mouse_pos[1u];
            if _e303 {
                phi_6_ = true;
            } else {
                phi_6_ = (18f <= _e299);
            }
            let _e327 = phi_6_;
            let _e330 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e315 - _e287) - _e298), ((_e320 - _e288) - _e300)), _e309, select(_e299, 18f, _e327));
            let _e333 = (0.5f + ((_e310 - _e249) * 0.008928572f));
            let _e335 = select(_e333, 0f, (_e333 < 0f));
            let _e337 = select(_e335, 1f, (_e335 > 1f));
            let _e350 = (0.5f + ((_e330 - _e272) * 0.008928572f));
            let _e352 = select(_e350, 0f, (_e350 < 0f));
            let _e354 = select(_e352, 1f, (_e352 > 1f));
            phi_7_ = vec2<f32>(0f, 0f);
            phi_8_ = 0f;
            phi_9_ = 0u;
            loop {
                let _e366 = phi_7_;
                let _e368 = phi_8_;
                let _e370 = phi_9_;
                local_37 = _e366;
                local_38 = _e366;
                local_39 = _e366;
                local_40 = _e366;
                local_46 = _e368;
                local_47 = _e368;
                local_48 = _e368;
                local_49 = _e368;
                let _e371 = (_e370 < 4u);
                if _e371 {
                    if _e371 {
                    } else {
                        phi_19_ = true;
                        break;
                    }
                    let _e378 = frame.member[0u].ripples[_e370].origin[0u];
                    let _e385 = frame.member[0u].ripples[_e370].origin[1u];
                    let _e391 = frame.member[0u].ripples[_e370].start_time;
                    let _e397 = frame.member[0u].ripples[_e370].strength;
                    let _e401 = frame.member[0u].time;
                    let _e403 = ((_e401 - _e391) * 1.2f);
                    let _e405 = select(_e403, 0f, (_e403 < 0f));
                    let _e407 = select(_e405, 1f, (_e405 > 1f));
                    if (_e397 > 0f) {
                        if (_e407 < 1f) {
                            let _e410 = (_e217.x - _e378);
                            let _e411 = (_e217.y - _e385);
                            let _e415 = sqrt(((_e410 * _e410) + (_e411 * _e411)));
                            if (_e415 > 0.001f) {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e410 / _e415), (_e411 / _e415)), _e415);
                            } else {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e415);
                            }
                            let _e423 = phi_10_;
                            let _e433 = ((abs((_e423.unnamed_1 - (_e407 * 600f))) - 80f) * -0.0125f);
                            let _e435 = select(_e433, 0f, (_e433 < 0f));
                            let _e437 = select(_e435, 1f, (_e435 > 1f));
                            let _e443 = (1f - _e407);
                            let _e444 = ((((_e437 * _e437) * (3f - (2f * _e437))) * _e397) * _e443);
                            let _e457 = (_e368 + (_e444 * 0.5f));
                            if (_e457 != _e457) {
                                phi_11_ = true;
                            } else {
                                phi_11_ = (1f <= _e457);
                            }
                            let _e461 = phi_11_;
                            phi_12_ = vec2<f32>((_e366.x + (((_e423.unnamed.x * _e444) * _e443) * 0.5f)), (_e366.y + (((_e423.unnamed.y * _e444) * _e443) * 0.5f)));
                            phi_13_ = select(_e457, 1f, _e461);
                        } else {
                            phi_12_ = _e366;
                            phi_13_ = _e368;
                        }
                        let _e464 = phi_12_;
                        let _e466 = phi_13_;
                        phi_14_ = _e464;
                        phi_15_ = _e466;
                    } else {
                        phi_14_ = _e366;
                        phi_15_ = _e368;
                    }
                    let _e468 = phi_14_;
                    let _e470 = phi_15_;
                    phi_16_ = _e468;
                    phi_17_ = _e470;
                    phi_18_ = (_e370 + 1u);
                } else {
                    phi_16_ = vec2<f32>();
                    phi_17_ = f32();
                    phi_18_ = u32();
                }
                let _e473 = phi_16_;
                let _e475 = phi_17_;
                let _e477 = phi_18_;
                continue;
                continuing {
                    phi_7_ = _e473;
                    phi_8_ = _e475;
                    phi_9_ = _e477;
                    phi_19_ = _e281;
                    break if !(_e371);
                }
            }
            let _e480 = phi_19_;
            if _e480 {
                break;
            }
            if _e254 {
                let _e481 = (_e217.x - _e315);
                let _e482 = (_e217.y - _e320);
                let _e488 = ((sqrt(((_e481 * _e481) + (_e482 * _e482))) - 150f) * -0.006666667f);
                let _e490 = select(_e488, 0f, (_e488 < 0f));
                let _e492 = select(_e490, 1f, (_e490 > 1f));
                phi_20_ = ((((_e492 * _e492) * (3f - (2f * _e492))) * _e253) * 8f);
            } else {
                phi_20_ = 0f;
            }
            let _e500 = phi_20_;
            let _e502 = local_37;
            let _e504 = global[0u];
            if (_e502.x == _e504) {
                let _e507 = local_38;
                let _e510 = global[1u];
                phi_21_ = (_e507.y == _e510);
            } else {
                phi_21_ = false;
            }
            let _e513 = phi_21_;
            if _e513 {
                phi_22_ = 0f;
            } else {
                let _e515 = local_39;
                phi_22_ = (sqrt(((_e502.x * _e502.x) + (_e515.y * _e515.y))) * 22f);
            }
            let _e523 = phi_22_;
            let _e525 = local_40;
            let _e528 = (((_e272 + ((((_e330 + ((_e272 - _e330) * _e354)) - ((56f * _e354) * (1f - _e354))) - _e272) * _e218.w)) - 0.5f) * -1f);
            let _e530 = select(_e528, 0f, (_e528 < 0f));
            let _e532 = select(_e530, 1f, (_e530 > 1f));
            let _e540 = ((_e249 + ((((_e310 + ((_e249 - _e310) * _e337)) - ((56f * _e337) * (1f - _e337))) - _e249) * _e218.w)) - (((_e500 * ((_e532 * _e532) * (3f - (2f * _e532)))) + _e523) * 0.5f));
            let _e542 = (_e240 + 60f);
            let _e543 = ((_e242 - _e240) > _e542);
            let _e548 = pill_2.member[_e219].calendar_expansion;
            let _e549 = (56f + _e243);
            let _e550 = (_e240 + 8f);
            let _e552 = (_e549 + (select(0f, 1f, _e543) * _e550));
            let _e553 = (_e552 * 0.0007377049f);
            let _e554 = (0.5f + _e553);
            let _e558 = ((_e548 - _e554) / ((_e553 + 0.74f) - _e554));
            let _e560 = select(_e558, 0f, (_e558 < 0f));
            let _e562 = select(_e560, 1f, (_e560 > 1f));
            let _e566 = ((_e562 * _e562) * (3f - (2f * _e562)));
            let _e568 = (292f * _e566);
            let _e569 = (_e240 * _e566);
            let _e577 = ((_e236 + 166f) + ((292f - _e568) * 0.5f));
            let _e578 = ((_e288 + (_e552 - _e243)) + ((_e240 - _e569) * 0.5f));
            let _e579 = (_e217.x - _e577);
            let _e580 = (_e217.y - _e578);
            let _e581 = select(6u, 5u, _e543);
            if (_e568 != _e568) {
                phi_23_ = true;
            } else {
                phi_23_ = (0.001f >= _e568);
            }
            let _e585 = phi_23_;
            let _e590 = (((_e579 / select(_e568, 0.001f, _e585)) * f32(_e581)) - 0.5f);
            let _e592 = f32((_e581 - 1u));
            if (0f <= _e592) {
            } else {
                break;
            }
            let _e595 = select(_e590, 0f, (_e590 < 0f));
            let _e597 = select(_e595, _e592, (_e595 > _e592));
            let _e598 = floor(_e597);
            let _e603 = select(select(u32(_e598), 0u, (_e598 < 0f)), 4294967295u, (_e598 > 4294967000f));
            let _e605 = (_e597 - trunc(_e597));
            let _e607 = select(_e605, 0f, (_e605 < 0f));
            let _e609 = select(_e607, 1f, (_e607 > 1f));
            let _e613 = ((_e609 * _e609) * (3f - (2f * _e609)));
            if _e543 {
                if (_e603 < 5u) {
                } else {
                    break;
                }
                let _e641 = pill_2.member[_e219].daily_conditions[_e603];
                let _e642 = (_e603 + 1u);
                let _e644 = select(_e642, 4u, (4u < _e642));
                if (_e644 < 5u) {
                } else {
                    break;
                }
                let _e650 = pill_2.member[_e219].daily_conditions[_e644];
                phi_24_ = 12f;
                phi_25_ = _e650;
                phi_26_ = _e641;
            } else {
                if (_e603 < 6u) {
                } else {
                    break;
                }
                let _e619 = pill_2.member[_e219].hourly_conditions[_e603];
                let _e620 = (_e603 + 1u);
                let _e622 = select(_e620, 5u, (5u < _e620));
                if (_e622 < 6u) {
                } else {
                    break;
                }
                let _e628 = pill_2.member[_e219].hourly_conditions[_e622];
                let _e632 = pill_2.member[_e219].hourly_start;
                phi_24_ = ((_e632 + (_e597 * 4f)) % 24f);
                phi_25_ = _e628;
                phi_26_ = _e619;
            }
            let _e652 = phi_24_;
            let _e654 = phi_25_;
            let _e656 = phi_26_;
            let _e657 = (_e566 <= 0.001f);
            if _e657 {
                phi_27_ = 340282350000000000000000000000000000000f;
            } else {
                let _e659 = (_e569 * 0.5f);
                let _e665 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e579 - (_e566 * 146f)), (_e580 - _e659)), ((_e568 - _e569) * 0.5f), _e659);
                phi_27_ = _e665;
            }
            let _e667 = phi_27_;
            if _e657 {
                phi_28_ = 340282350000000000000000000000000000000f;
            } else {
                let _e671 = (_e569 * 0.5f);
                let _e677 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e315 - _e577) - (_e566 * 146f)), ((_e320 - _e578) - _e671)), ((_e568 - _e569) * 0.5f), _e671);
                phi_28_ = _e677;
            }
            let _e679 = phi_28_;
            let _e713 = pill_2.member[_e219].sun_hours;
            let _e716 = (_e713[1] - _e713[0]);
            if (_e652 >= _e713[0]) {
                let _e718 = (_e652 <= _e713[1]);
                if _e718 {
                    let _e720 = ((_e652 - _e713[0]) / _e716);
                    phi_29_ = array<f32, 2>(_e720, sin((_e720 * 3.1415927f)));
                } else {
                    phi_29_ = array<f32, 2>();
                }
                let _e725 = phi_29_;
                phi_30_ = _e725;
                phi_31_ = select(true, false, _e718);
            } else {
                phi_30_ = array<f32, 2>();
                phi_31_ = true;
            }
            let _e728 = phi_30_;
            let _e730 = phi_31_;
            if _e730 {
                let _e731 = (24f - _e716);
                if (_e652 < _e713[0]) {
                    phi_32_ = (((_e652 + 24f) - _e713[1]) / _e731);
                } else {
                    phi_32_ = ((_e652 - _e713[1]) / _e731);
                }
                let _e739 = phi_32_;
                phi_33_ = array<f32, 2>(select(0f, 1f, (_e652 >= _e713[1])), -(sin((_e739 * 3.1415927f))));
            } else {
                phi_33_ = _e728;
            }
            let _e747 = phi_33_;
            let _e750 = ((_e679 - 0.5f) * -1f);
            let _e752 = select(_e750, 0f, (_e750 < 0f));
            let _e754 = select(_e752, 1f, (_e752 > 1f));
            let _e762 = (_e667 - (((_e500 * ((_e754 * _e754) * (3f - (2f * _e754)))) + _e523) * 0.5f));
            let _e763 = (_e540 != _e540);
            if _e763 {
                phi_34_ = true;
            } else {
                phi_34_ = (_e762 <= _e540);
            }
            let _e766 = phi_34_;
            let _e767 = select(_e540, _e762, _e766);
            let _e768 = fwidth(_e767);
            if (_e768 != _e768) {
                phi_35_ = true;
            } else {
                phi_35_ = (0.55f >= _e768);
            }
            let _e772 = phi_35_;
            let _e773 = select(_e768, 0.55f, _e772);
            let _e777 = ((_e767 - _e773) / (-(_e773) - _e773));
            let _e779 = select(_e777, 0f, (_e777 < 0f));
            let _e781 = select(_e779, 1f, (_e779 > 1f));
            let _e785 = ((_e781 * _e781) * (3f - (2f * _e781)));
            if (_e767 != _e767) {
                phi_36_ = true;
            } else {
                phi_36_ = (0f >= _e767);
            }
            let _e789 = phi_36_;
            let _e793 = (exp((select(_e767, 0f, _e789) * -0.3f)) * 0.16f);
            if (_e785 != _e785) {
                phi_37_ = true;
            } else {
                phi_37_ = (_e793 >= _e785);
            }
            let _e797 = phi_37_;
            let _e798 = select(_e785, _e793, _e797);
            if (_e798 <= 0.0009765625f) {
                discard;
            }
            let _e804 = pill_2.member[_e219].hourly_conditions[0u];
            let _e805 = (_e241 * 0.0032467532f);
            let _e807 = select(_e805, 0f, (_e805 < 0f));
            let _e816 = pill_2.member[_e219].hourly_conditions[1u];
            let _e818 = ((abs((select(_e807, 1f, (_e807 > 1f)) - 0.5f)) - 0.2f) * 20.000002f);
            let _e820 = select(_e818, 0f, (_e818 < 0f));
            let _e822 = select(_e820, 1f, (_e820 > 1f));
            let _e826 = ((_e822 * _e822) * (3f - (2f * _e822)));
            let _e831 = (_e804.fog + ((_e816.fog - _e804.fog) * _e826));
            let _e836 = (_e804.cloud + ((_e816.cloud - _e804.cloud) * _e826));
            let _e841 = (_e804.rain + ((_e816.rain - _e804.rain) * _e826));
            let _e846 = (_e804.snow + ((_e816.snow - _e804.snow) * _e826));
            let _e851 = (_e804.lightning + ((_e816.lightning - _e804.lightning) * _e826));
            let _e856 = (_e804.hail + ((_e816.hail - _e804.hail) * _e826));
            let _e859 = (_e831 + ((_e804.fog - _e831) * _e218.w));
            let _e862 = (_e836 + ((_e804.cloud - _e836) * _e218.w));
            let _e865 = (_e841 + ((_e804.rain - _e841) * _e218.w));
            let _e868 = (_e846 + ((_e804.snow - _e846) * _e218.w));
            let _e871 = (_e851 + ((_e804.lightning - _e851) * _e218.w));
            let _e874 = (_e856 + ((_e804.hail - _e856) * _e218.w));
            let _e875 = (_e242 / _e240);
            if _e763 {
                phi_38_ = true;
            } else {
                phi_38_ = (0f <= _e540);
            }
            let _e880 = phi_38_;
            let _e883 = (1f + (select(_e540, 0f, _e880) * 0.008333334f));
            let _e885 = select(_e883, 0f, (_e883 < 0f));
            let _e887 = select(_e885, 0.6f, (_e885 > 0.6f));
            let _e894 = (_e502.x * 0.04f);
            let _e895 = (_e525.y * 0.04f);
            let _e896 = ((_e805 - (((_e805 - 0.5f) * _e887) * 0.08f)) - _e894);
            let _e897 = ((_e875 - (((_e875 - 0.5f) * _e887) * 0.08f)) - _e895);
            if (_e566 > 0.001f) {
                let _e900 = (_e579 / _e568);
                let _e901 = (_e580 / _e569);
                if (_e762 != _e762) {
                    phi_39_ = true;
                } else {
                    phi_39_ = (0f <= _e762);
                }
                let _e907 = phi_39_;
                let _e910 = (1f + (select(_e762, 0f, _e907) * 0.008333334f));
                let _e912 = select(_e910, 0f, (_e910 < 0f));
                let _e914 = select(_e912, 0.6f, (_e912 > 0.6f));
                phi_40_ = vec2<f32>(((_e900 - (((_e900 - 0.5f) * _e914) * 0.08f)) - _e894), ((_e901 - (((_e901 - 0.5f) * _e914) * 0.08f)) - _e895));
            } else {
                phi_40_ = vec2<f32>(_e896, _e897);
            }
            let _e925 = phi_40_;
            let _e926 = fwidth(_e762);
            if (_e926 != _e926) {
                phi_41_ = true;
            } else {
                phi_41_ = (0.55f >= _e926);
            }
            let _e930 = phi_41_;
            let _e931 = select(_e926, 0.55f, _e930);
            let _e935 = ((_e762 - _e931) / (-(_e931) - _e931));
            let _e937 = select(_e935, 0f, (_e935 < 0f));
            let _e939 = select(_e937, 1f, (_e937 > 1f));
            let _e944 = (((_e939 * _e939) * (3f - (2f * _e939))) * _e566);
            let _e951 = (1f - _e944);
            let _e956 = (((_e896 * 308f) * _e951) + ((_e925.x * _e568) * _e944));
            let _e957 = (((_e897 * _e240) * _e951) + ((_e925.y * _e569) * _e944));
            if (_e762 != _e762) {
                phi_42_ = true;
            } else {
                phi_42_ = (1000f <= _e762);
            }
            let _e964 = phi_42_;
            let _e971 = (_e859 + (((_e656.fog + ((_e654.fog - _e656.fog) * _e613)) - _e859) * _e944));
            let _e974 = (_e862 + (((_e656.cloud + ((_e654.cloud - _e656.cloud) * _e613)) - _e862) * _e944));
            let _e977 = (_e865 + (((_e656.rain + ((_e654.rain - _e656.rain) * _e613)) - _e865) * _e944));
            let _e980 = (_e868 + (((_e656.snow + ((_e654.snow - _e656.snow) * _e613)) - _e868) * _e944));
            let _e986 = (_e874 + (((_e656.hail + ((_e654.hail - _e656.hail) * _e613)) - _e874) * _e944));
            let _e988 = ((_e218.y - -0.04f) * 4.1666665f);
            let _e990 = select(_e988, 0f, (_e988 < 0f));
            let _e992 = select(_e990, 1f, (_e990 > 1f));
            let _e996 = ((_e992 * _e992) * (3f - (2f * _e992)));
            let _e998 = ((_e218.y - -0.32f) * 4.166667f);
            let _e1000 = select(_e998, 0f, (_e998 < 0f));
            let _e1002 = select(_e1000, 1f, (_e1000 > 1f));
            let _e1010 = ((_e218.y - -0.18f) * 5.5555553f);
            let _e1012 = select(_e1010, 0f, (_e1010 < 0f));
            let _e1014 = select(_e1012, 1f, (_e1012 > 1f));
            let _e1020 = ((_e218.y - 0.2f) * -5.5555553f);
            let _e1022 = select(_e1020, 0f, (_e1020 < 0f));
            let _e1024 = select(_e1022, 1f, (_e1022 > 1f));
            let _e1031 = ((_e747[1] - -0.04f) * 4.1666665f);
            let _e1033 = select(_e1031, 0f, (_e1031 < 0f));
            let _e1035 = select(_e1033, 1f, (_e1033 > 1f));
            let _e1039 = ((_e1035 * _e1035) * (3f - (2f * _e1035)));
            let _e1041 = ((_e747[1] - -0.32f) * 4.166667f);
            let _e1043 = select(_e1041, 0f, (_e1041 < 0f));
            let _e1045 = select(_e1043, 1f, (_e1043 > 1f));
            let _e1053 = ((_e747[1] - -0.18f) * 5.5555553f);
            let _e1055 = select(_e1053, 0f, (_e1053 < 0f));
            let _e1057 = select(_e1055, 1f, (_e1055 > 1f));
            let _e1063 = ((_e747[1] - 0.2f) * -5.5555553f);
            let _e1065 = select(_e1063, 0f, (_e1063 < 0f));
            let _e1067 = select(_e1065, 1f, (_e1065 > 1f));
            let _e1079 = ((_e996 * _e951) + (_e1039 * _e944));
            let _e1081 = (((((_e1014 * _e1014) * (3f - (2f * _e1014))) * ((_e1024 * _e1024) * (3f - (2f * _e1024)))) * _e951) + ((((_e1057 * _e1057) * (3f - (2f * _e1057))) * ((_e1067 * _e1067) * (3f - (2f * _e1067)))) * _e944));
            let _e1085 = frame.member[0u].time;
            let _e1086 = (_e957 / _e240);
            let _e1088 = ((_e1086 - 1f) * -1f);
            let _e1090 = select(_e1088, 0f, (_e1088 < 0f));
            let _e1092 = select(_e1090, 1f, (_e1090 > 1f));
            let _e1096 = ((_e1092 * _e1092) * (3f - (2f * _e1092)));
            let _e1097 = (1f - _e1096);
            let _e1116 = (1f - _e1079);
            let _e1128 = (0.3f * _e1097);
            let _e1129 = (0.22f * _e1096);
            let _e1135 = ((((((_e1002 * _e1002) * (3f - (2f * _e1002))) * (1f - _e996)) * _e951) + ((((_e1045 * _e1045) * (3f - (2f * _e1045))) * (1f - _e1039)) * _e944)) * 0.8f);
            let _e1136 = (1f - _e1135);
            let _e1153 = (_e1081 * 0.9f);
            let _e1154 = (1f - _e1153);
            let _e1166 = floor((_e956 * 0.055555556f));
            let _e1167 = floor((_e957 * 0.055555556f));
            let _e1171 = cantus_render_shader_hash(vec2<f32>(_e1166, _e1167));
            let _e1180 = (_e956 - (((_e1166 + 0.2f) + (_e1171.x * 0.6f)) * 18f));
            let _e1181 = (_e957 - (((_e1167 + 0.2f) + (_e1171.y * 0.6f)) * 18f));
            let _e1187 = ((sqrt(((_e1180 * _e1180) + (_e1181 * _e1181))) - 1f) * -1.6666666f);
            let _e1189 = select(_e1187, 0f, (_e1187 < 0f));
            let _e1191 = select(_e1189, 1f, (_e1189 > 1f));
            let _e1199 = cantus_render_shader_hash(vec2<f32>((_e1166 + 31.7f), (_e1167 + 31.7f)));
            let _e1202 = ((_e1199.x - 0.75f) * 4f);
            let _e1204 = select(_e1202, 0f, (_e1202 < 0f));
            let _e1206 = select(_e1204, 1f, (_e1204 > 1f));
            let _e1217 = ((((((_e1191 * _e1191) * (3f - (2f * _e1191))) * ((_e1206 * _e1206) * (3f - (2f * _e1206)))) * _e1116) * (1f - _e974)) * (0.3f + (_e1096 * 0.7f)));
            let _e1218 = (((((((((0.006f * _e1097) + (0.025f * _e1096)) * _e1116) + (((0.08f * _e1097) + (0.32f * _e1096)) * _e1079)) * _e1136) + (((0.1f * _e1097) + _e1129) * _e1135)) * _e1154) + (((0.78f * _e1097) + (0.38f * _e1096)) * _e1153)) + _e1217);
            let _e1219 = (((((((((0.012f * _e1097) + (0.04f * _e1096)) * _e1116) + (((0.34f * _e1097) + (0.67f * _e1096)) * _e1079)) * _e1136) + (((0.16f * _e1097) + (0.25f * _e1096)) * _e1135)) * _e1154) + ((_e1128 + _e1129) * _e1153)) + _e1217);
            let _e1220 = (((((((((0.035f * _e1097) + (0.095f * _e1096)) * _e1116) + (((0.62f * _e1097) + (0.87f * _e1096)) * _e1079)) * _e1136) + ((_e1128 + (0.45f * _e1096)) * _e1135)) * _e1154) + (((0.2f * _e1097) + (0.42f * _e1096)) * _e1153)) + _e1217);
            if (_e974 > 0.0009765625f) {
                let _e1223 = (_e956 / _e240);
                phi_43_ = 0i;
                phi_44_ = 0.5f;
                phi_45_ = 0f;
                phi_46_ = vec2<f32>(((_e1223 * 0.14f) + (_e1085 * 0.012f)), ((_e1086 * 0.14f) + 6.1f));
                loop {
                    let _e1231 = phi_43_;
                    let _e1233 = phi_44_;
                    let _e1235 = phi_45_;
                    let _e1237 = phi_46_;
                    local_41 = _e1235;
                    let _e1238 = (_e1231 < 4i);
                    if _e1238 {
                        let _e1241 = cantus_render_shader_simplex_noise(_e1237);
                        phi_47_ = (_e1231 + 1i);
                        phi_48_ = (_e1233 * 0.5f);
                        phi_49_ = (_e1235 + (_e1241 * _e1233));
                        phi_50_ = vec2<f32>(((_e1237.x * 1.6f) + (_e1237.y * 1.2f)), ((_e1237.y * 1.6f) - (_e1237.x * 1.2f)));
                    } else {
                        phi_47_ = i32();
                        phi_48_ = f32();
                        phi_49_ = f32();
                        phi_50_ = vec2<f32>();
                    }
                    let _e1254 = phi_47_;
                    let _e1256 = phi_48_;
                    let _e1258 = phi_49_;
                    let _e1260 = phi_50_;
                    continue;
                    continuing {
                        phi_43_ = _e1254;
                        phi_44_ = _e1256;
                        phi_45_ = _e1258;
                        phi_46_ = _e1260;
                        break if !(_e1238);
                    }
                }
                let _e1263 = local_41;
                let _e1264 = (_e1263 * 0.5f);
                phi_51_ = 0i;
                phi_52_ = 0.5f;
                phi_53_ = 0f;
                phi_54_ = vec2<f32>(((_e1223 * 0.287f) + (_e1085 * 0.018f)), ((_e1086 * 0.287f) + -3.7f));
                loop {
                    let _e1273 = phi_51_;
                    let _e1275 = phi_52_;
                    let _e1277 = phi_53_;
                    let _e1279 = phi_54_;
                    local_42 = _e1277;
                    local_43 = _e1277;
                    let _e1280 = (_e1273 < 4i);
                    if _e1280 {
                        let _e1283 = cantus_render_shader_simplex_noise(_e1279);
                        phi_55_ = (_e1273 + 1i);
                        phi_56_ = (_e1275 * 0.5f);
                        phi_57_ = (_e1277 + (_e1283 * _e1275));
                        phi_58_ = vec2<f32>(((_e1279.x * 1.6f) + (_e1279.y * 1.2f)), ((_e1279.y * 1.6f) - (_e1279.x * 1.2f)));
                    } else {
                        phi_55_ = i32();
                        phi_56_ = f32();
                        phi_57_ = f32();
                        phi_58_ = vec2<f32>();
                    }
                    let _e1296 = phi_55_;
                    let _e1298 = phi_56_;
                    let _e1300 = phi_57_;
                    let _e1302 = phi_58_;
                    continue;
                    continuing {
                        phi_51_ = _e1296;
                        phi_52_ = _e1298;
                        phi_53_ = _e1300;
                        phi_54_ = _e1302;
                        break if !(_e1280);
                    }
                }
                let _e1305 = local_42;
                let _e1308 = local_43;
                let _e1312 = ((((0.5f + _e1264) + (_e1308 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1314 = select(_e1312, 0f, (_e1312 < 0f));
                let _e1316 = select(_e1314, 1f, (_e1314 > 1f));
                let _e1322 = (((_e1305 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1324 = select(_e1322, 0f, (_e1322 < 0f));
                let _e1326 = select(_e1324, 1f, (_e1324 > 1f));
                let _e1333 = ((_e1264 + 0.02000001f) * 4.5454545f);
                let _e1335 = select(_e1333, 0f, (_e1333 < 0f));
                let _e1337 = select(_e1335, 1f, (_e1335 > 1f));
                let _e1343 = ((((_e1326 * _e1326) * (3f - (2f * _e1326))) * 0.55f) + (((_e1337 * _e1337) * (3f - (2f * _e1337))) * 0.45f));
                let _e1344 = (1f - _e1343);
                let _e1381 = (_e1081 * 0.45f);
                let _e1382 = (1f - _e1381);
                let _e1394 = (_e974 * (0.12f + (((_e1316 * _e1316) * (3f - (2f * _e1316))) * 0.7f)));
                let _e1395 = (1f - _e1394);
                phi_59_ = vec3<f32>(((_e1218 * _e1395) + (((((((0.16f * _e1344) + (0.32f * _e1343)) * _e1116) + (((0.62f * _e1344) + (0.92f * _e1343)) * _e1079)) * _e1382) + (((0.5f * _e1344) + (0.76f * _e1343)) * _e1381)) * _e1394)), ((_e1219 * _e1395) + (((((((0.2f * _e1344) + (0.36f * _e1343)) * _e1116) + (((0.7f * _e1344) + (0.94f * _e1343)) * _e1079)) * _e1382) + (((0.36f * _e1344) + (0.59f * _e1343)) * _e1381)) * _e1394)), ((_e1220 * _e1395) + (((((((0.28f * _e1344) + (0.43f * _e1343)) * _e1116) + (((0.78f * _e1344) + (0.96f * _e1343)) * _e1079)) * _e1382) + (((0.4f * _e1344) + (0.56f * _e1343)) * _e1381)) * _e1394)));
            } else {
                phi_59_ = vec3<f32>(_e1218, _e1219, _e1220);
            }
            let _e1407 = phi_59_;
            let _e1409 = (1f - (_e977 * 0.2f));
            let _e1419 = ((_e1407.x * _e1409) + (_e977 * 0.020000001f));
            let _e1420 = ((_e1407.y * _e1409) + (_e977 * 0.034f));
            let _e1421 = ((_e1407.z * _e1409) + (_e977 * 0.05f));
            if (_e977 > 0.0009765625f) {
                let _e1426 = (_e956 - (20f * _e1085));
                let _e1427 = (_e957 - (110f * _e1085));
                let _e1430 = floor((_e1426 * 0.06666667f));
                let _e1431 = floor((_e1427 * 0.04f));
                let _e1433 = cantus_render_shader_hash(vec2<f32>(_e1430, _e1431));
                let _e1444 = (_e1426 - (((_e1430 + 0.15f) + (_e1433.x * 0.7f)) * 15f));
                let _e1445 = (_e1427 - (((_e1431 + 0.15f) + (_e1433.y * 0.7f)) * 25f));
                let _e1449 = (((_e1444 * 1.8000001f) + (_e1445 * 9f)) * 0.011870845f);
                let _e1451 = select(_e1449, 0f, (_e1449 < 0f));
                let _e1453 = select(_e1451, 1f, (_e1451 > 1f));
                let _e1456 = (_e1444 - (1.8000001f * _e1453));
                let _e1457 = (_e1445 - (9f * _e1453));
                let _e1463 = ((sqrt(((_e1456 * _e1456) + (_e1457 * _e1457))) - 1.0999999f) * -1.666667f);
                let _e1465 = select(_e1463, 0f, (_e1463 < 0f));
                let _e1467 = select(_e1465, 1f, (_e1465 > 1f));
                let _e1475 = cantus_render_shader_hash(vec2<f32>((_e1430 + 19.3f), (_e1431 + 19.3f)));
                let _e1478 = ((_e1475.x - 0.22000003f) * 1.2820513f);
                let _e1480 = select(_e1478, 0f, (_e1478 < 0f));
                let _e1482 = select(_e1480, 1f, (_e1480 > 1f));
                let _e1489 = (((((_e1467 * _e1467) * (3f - (2f * _e1467))) * ((_e1482 * _e1482) * (3f - (2f * _e1482)))) * _e977) * 0.7f);
                let _e1491 = select(_e1489, 0f, (_e1489 < 0f));
                let _e1493 = select(_e1491, 1f, (_e1491 > 1f));
                let _e1494 = (1f - _e1493);
                phi_60_ = vec3<f32>(((_e1419 * _e1494) + (0.52f * _e1493)), ((_e1420 * _e1494) + (0.72f * _e1493)), ((_e1421 * _e1494) + (0.9f * _e1493)));
            } else {
                phi_60_ = vec3<f32>(_e1419, _e1420, _e1421);
            }
            let _e1506 = phi_60_;
            if (_e980 > 0.0009765625f) {
                let _e1510 = (_e956 - (5f * _e1085));
                let _e1511 = (_e957 - (14f * _e1085));
                let _e1514 = floor((_e1510 * 0.05f));
                let _e1515 = floor((_e1511 * 0.05f));
                let _e1519 = cantus_render_shader_hash(vec2<f32>((_e1514 + 31.7f), (_e1515 + 31.7f)));
                let _e1530 = (_e1510 - (((_e1514 + 0.15f) + (_e1519.x * 0.7f)) * 20f));
                let _e1531 = (_e1511 - (((_e1515 + 0.15f) + (_e1519.y * 0.7f)) * 20f));
                let _e1535 = (((_e1530 * 0.080000006f) + (_e1531 * 0.4f)) * 6.009615f);
                let _e1537 = select(_e1535, 0f, (_e1535 < 0f));
                let _e1539 = select(_e1537, 1f, (_e1537 > 1f));
                let _e1542 = (_e1530 - (0.080000006f * _e1539));
                let _e1543 = (_e1531 - (0.4f * _e1539));
                let _e1549 = ((sqrt(((_e1542 * _e1542) + (_e1543 * _e1543))) - 1.5999999f) * -1.666667f);
                let _e1551 = select(_e1549, 0f, (_e1549 < 0f));
                let _e1553 = select(_e1551, 1f, (_e1551 > 1f));
                let _e1561 = cantus_render_shader_hash(vec2<f32>((_e1514 + 19.3f), (_e1515 + 19.3f)));
                let _e1564 = ((_e1561.x - 0.3f) * 1.4285715f);
                let _e1566 = select(_e1564, 0f, (_e1564 < 0f));
                let _e1568 = select(_e1566, 1f, (_e1566 > 1f));
                let _e1575 = (((((_e1553 * _e1553) * (3f - (2f * _e1553))) * ((_e1568 * _e1568) * (3f - (2f * _e1568)))) * _e980) * 0.92f);
                let _e1577 = select(_e1575, 0f, (_e1575 < 0f));
                let _e1579 = select(_e1577, 1f, (_e1577 > 1f));
                let _e1580 = (1f - _e1579);
                let _e1587 = (0.96f * _e1579);
                phi_61_ = vec3<f32>(((_e1506.x * _e1580) + _e1587), ((_e1506.y * _e1580) + _e1587), ((_e1506.z * _e1580) + _e1587));
            } else {
                phi_61_ = _e1506;
            }
            let _e1593 = phi_61_;
            if (_e986 > 0.0009765625f) {
                let _e1597 = (_e956 - (18f * _e1085));
                let _e1598 = (_e957 - (85f * _e1085));
                let _e1601 = floor((_e1597 * 0.04347826f));
                let _e1602 = floor((_e1598 * 0.04347826f));
                let _e1606 = cantus_render_shader_hash(vec2<f32>((_e1601 + 63.4f), (_e1602 + 63.4f)));
                let _e1617 = (_e1597 - (((_e1601 + 0.15f) + (_e1606.x * 0.7f)) * 23f));
                let _e1618 = (_e1598 - (((_e1602 + 0.15f) + (_e1606.y * 0.7f)) * 23f));
                let _e1622 = (((_e1617 * 0.24000001f) + (_e1618 * 1.2f)) * 0.667735f);
                let _e1624 = select(_e1622, 0f, (_e1622 < 0f));
                let _e1626 = select(_e1624, 1f, (_e1624 > 1f));
                let _e1629 = (_e1617 - (0.24000001f * _e1626));
                let _e1630 = (_e1618 - (1.2f * _e1626));
                let _e1636 = ((sqrt(((_e1629 * _e1629) + (_e1630 * _e1630))) - 0.79999995f) * -1.6666667f);
                let _e1638 = select(_e1636, 0f, (_e1636 < 0f));
                let _e1640 = select(_e1638, 1f, (_e1638 > 1f));
                let _e1648 = cantus_render_shader_hash(vec2<f32>((_e1601 + 19.3f), (_e1602 + 19.3f)));
                let _e1651 = ((_e1648.x - 0.7f) * 3.3333333f);
                let _e1653 = select(_e1651, 0f, (_e1651 < 0f));
                let _e1655 = select(_e1653, 1f, (_e1653 > 1f));
                let _e1662 = (((((_e1640 * _e1640) * (3f - (2f * _e1640))) * ((_e1655 * _e1655) * (3f - (2f * _e1655)))) * _e986) * 0.7f);
                let _e1664 = select(_e1662, 0f, (_e1662 < 0f));
                let _e1666 = select(_e1664, 1f, (_e1664 > 1f));
                let _e1667 = (1f - _e1666);
                phi_62_ = vec3<f32>(((_e1593.x * _e1667) + (0.75f * _e1666)), ((_e1593.y * _e1667) + (0.86f * _e1666)), ((_e1593.z * _e1667) + (0.94f * _e1666)));
            } else {
                phi_62_ = _e1593;
            }
            let _e1682 = phi_62_;
            let _e1686 = ((sin((_e1085 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1688 = select(_e1686, 0f, (_e1686 < 0f));
            let _e1690 = select(_e1688, 1f, (_e1688 > 1f));
            let _e1695 = (((_e1690 * _e1690) * (3f - (2f * _e1690))) * (_e871 + (((_e656.lightning + ((_e654.lightning - _e656.lightning) * _e613)) - _e871) * _e944)));
            let _e1697 = (1f - (_e1695 * 0.55f));
            let _e1707 = ((_e1682.x * _e1697) + (_e1695 * 0.3575f));
            let _e1708 = ((_e1682.y * _e1697) + (_e1695 * 0.407f));
            let _e1709 = ((_e1682.z * _e1697) + (_e1695 * 0.528f));
            if (_e971 > 0.0009765625f) {
                phi_63_ = 0i;
                phi_64_ = 0.5f;
                phi_65_ = 0f;
                phi_66_ = vec2<f32>((((_e956 / (308f + ((_e568 - 308f) * _e944))) * 0.9f) + (_e1085 * 0.008f)), ((_e1086 * 0.32f) + 12f));
                loop {
                    let _e1720 = phi_63_;
                    let _e1722 = phi_64_;
                    let _e1724 = phi_65_;
                    let _e1726 = phi_66_;
                    local_44 = _e1724;
                    let _e1727 = (_e1720 < 4i);
                    if _e1727 {
                        let _e1730 = cantus_render_shader_simplex_noise(_e1726);
                        phi_67_ = (_e1720 + 1i);
                        phi_68_ = (_e1722 * 0.5f);
                        phi_69_ = (_e1724 + (_e1730 * _e1722));
                        phi_70_ = vec2<f32>(((_e1726.x * 1.6f) + (_e1726.y * 1.2f)), ((_e1726.y * 1.6f) - (_e1726.x * 1.2f)));
                    } else {
                        phi_67_ = i32();
                        phi_68_ = f32();
                        phi_69_ = f32();
                        phi_70_ = vec2<f32>();
                    }
                    let _e1743 = phi_67_;
                    let _e1745 = phi_68_;
                    let _e1747 = phi_69_;
                    let _e1749 = phi_70_;
                    continue;
                    continuing {
                        phi_63_ = _e1743;
                        phi_64_ = _e1745;
                        phi_65_ = _e1747;
                        phi_66_ = _e1749;
                        break if !(_e1727);
                    }
                }
                let _e1752 = local_44;
                let _e1755 = (((_e1752 * 0.5f) + 0.15f) * 2.857143f);
                let _e1757 = select(_e1755, 0f, (_e1755 < 0f));
                let _e1759 = select(_e1757, 1f, (_e1757 > 1f));
                let _e1766 = (_e971 * (0.58f + (((_e1759 * _e1759) * (3f - (2f * _e1759))) * 0.18f)));
                let _e1767 = (1f - _e1766);
                phi_71_ = vec3<f32>(((_e1707 * _e1767) + (0.63f * _e1766)), ((_e1708 * _e1767) + (0.69f * _e1766)), ((_e1709 * _e1767) + (0.73f * _e1766)));
            } else {
                phi_71_ = vec3<f32>(_e1707, _e1708, _e1709);
            }
            let _e1779 = phi_71_;
            let _e1781 = ((_e1086 - 0.12f) * -8.333334f);
            let _e1783 = select(_e1781, 0f, (_e1781 < 0f));
            let _e1785 = select(_e1783, 1f, (_e1783 > 1f));
            let _e1792 = (((_e540 + ((select(_e762, 1000f, _e964) - _e540) * _e944)) - 5f) * -0.125f);
            let _e1794 = select(_e1792, 0f, (_e1792 < 0f));
            let _e1796 = select(_e1794, 1f, (_e1794 > 1f));
            let _e1802 = ((((_e1785 * _e1785) * (3f - (2f * _e1785))) * 0.12f) + (((_e1796 * _e1796) * (3f - (2f * _e1796))) * 0.08f));
            let _e1804 = (_e1779.x + _e1802);
            let _e1806 = (_e1779.y + _e1802);
            let _e1808 = (_e1779.z + _e1802);
            if (_e249 < 1f) {
                let _e1813 = (16f + (_e218.x * 276f));
                let _e1815 = select(_e218.y, 0f, (_e218.y < 0f));
                let _e1819 = (0.72f - (select(_e1815, 1f, (_e1815 > 1f)) * 0.45f));
                let _e1822 = ((_e218.y - 0.55f) * -1.8867923f);
                let _e1824 = select(_e1822, 0f, (_e1822 < 0f));
                let _e1826 = select(_e1824, 1f, (_e1824 > 1f));
                let _e1830 = ((_e1826 * _e1826) * (3f - (2f * _e1826)));
                let _e1831 = (1f - _e1830);
                if (_e836 > 0.0009765625f) {
                    phi_72_ = 0i;
                    phi_73_ = 0.5f;
                    phi_74_ = 0f;
                    phi_75_ = vec2<f32>((((_e1813 / _e240) * 0.14f) + (_e1085 * 0.012f)), ((_e1819 * 0.14f) + 6.1f));
                    loop {
                        let _e1849 = phi_72_;
                        let _e1851 = phi_73_;
                        let _e1853 = phi_74_;
                        let _e1855 = phi_75_;
                        local_45 = _e1853;
                        let _e1856 = (_e1849 < 4i);
                        if _e1856 {
                            let _e1859 = cantus_render_shader_simplex_noise(_e1855);
                            phi_76_ = (_e1849 + 1i);
                            phi_77_ = (_e1851 * 0.5f);
                            phi_78_ = (_e1853 + (_e1859 * _e1851));
                            phi_79_ = vec2<f32>(((_e1855.x * 1.6f) + (_e1855.y * 1.2f)), ((_e1855.y * 1.6f) - (_e1855.x * 1.2f)));
                        } else {
                            phi_76_ = i32();
                            phi_77_ = f32();
                            phi_78_ = f32();
                            phi_79_ = vec2<f32>();
                        }
                        let _e1872 = phi_76_;
                        let _e1874 = phi_77_;
                        let _e1876 = phi_78_;
                        let _e1878 = phi_79_;
                        continue;
                        continuing {
                            phi_72_ = _e1872;
                            phi_73_ = _e1874;
                            phi_74_ = _e1876;
                            phi_75_ = _e1878;
                            break if !(_e1856);
                        }
                    }
                    let _e1881 = local_45;
                    let _e1884 = (((_e1881 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e1886 = select(_e1884, 0f, (_e1884 < 0f));
                    let _e1888 = select(_e1886, 1f, (_e1886 > 1f));
                    phi_80_ = ((((_e1888 * _e1888) * (3f - (2f * _e1888))) * _e836) * 0.82f);
                } else {
                    phi_80_ = 0f;
                }
                let _e1896 = phi_80_;
                let _e1898 = ((_e218.y - -0.02f) * 16.666668f);
                let _e1900 = select(_e1898, 0f, (_e1898 < 0f));
                let _e1902 = select(_e1900, 1f, (_e1900 > 1f));
                let _e1909 = (_e241 - _e1813);
                let _e1910 = (_e242 - (_e240 * _e1819));
                let _e1914 = sqrt(((_e1909 * _e1909) + (_e1910 * _e1910)));
                let _e1916 = ((_e1914 - 62f) * -0.01724138f);
                let _e1918 = select(_e1916, 0f, (_e1916 < 0f));
                let _e1920 = select(_e1918, 1f, (_e1918 > 1f));
                let _e1927 = ((_e1914 - 11f) * -0.1f);
                let _e1929 = select(_e1927, 0f, (_e1927 < 0f));
                let _e1931 = select(_e1929, 1f, (_e1929 > 1f));
                let _e1938 = (((((_e1920 * _e1920) * (3f - (2f * _e1920))) * 0.24f) + (((_e1931 * _e1931) * (3f - (2f * _e1931))) * 0.7f)) * (((_e1902 * _e1902) * (3f - (2f * _e1902))) * (1f - _e1896)));
                let _e1939 = (1f - _e1938);
                let _e1952 = ((_e249 - 1f) / ((_e240 * -0.25f) - 1f));
                let _e1954 = select(_e1952, 0f, (_e1952 < 0f));
                let _e1956 = select(_e1954, 1f, (_e1954 > 1f));
                let _e1960 = ((_e1956 * _e1956) * (3f - (2f * _e1956)));
                let _e1961 = (1f - _e1960);
                phi_81_ = vec3<f32>(((_e1804 * _e1961) + (((_e1804 * _e1939) + (((0.96f * _e1831) + (0.98f * _e1830)) * _e1938)) * _e1960)), ((_e1806 * _e1961) + (((_e1806 * _e1939) + (((0.98f * _e1831) + (0.74f * _e1830)) * _e1938)) * _e1960)), ((_e1808 * _e1961) + (((_e1808 * _e1939) + ((_e1831 + (0.66f * _e1830)) * _e1938)) * _e1960)));
            } else {
                phi_81_ = (_e1779 + vec3(_e1802));
            }
            let _e1973 = phi_81_;
            let _e1984 = local_46;
            let _e1985 = (1f - _e1984);
            let _e1990 = local_47;
            let _e1993 = local_48;
            let _e1996 = local_49;
            if (_e217.y < _e288) {
                phi_93_ = 0u;
                phi_94_ = u32();
                phi_95_ = true;
            } else {
                let _e2002 = (_e217.x - (_e236 - 158f));
                if (_e2002 >= 316f) {
                    if (_e297 < ((_e243 + 96f) * 0.5f)) {
                        phi_89_ = 4u;
                    } else {
                        let _e2047 = (_e549 + _e550);
                        let _e2048 = (_e2047 + _e243);
                        if (_e297 > _e2048) {
                            let _e2064 = cantus_render_tempestas_cell_index(_e297, _e2048, 28f, 2f);
                            phi_88_ = ((76u + (_e2064 * 2u)) + select(0u, 1u, (_e297 > (_e2048 + (4f * (3.5f + (f32(_e2064) * 7f)))))));
                        } else {
                            let _e2050 = (_e297 > _e542);
                            let _e2051 = select(6u, 5u, _e2050);
                            let _e2058 = cantus_render_tempestas_cell_index(_e2002, 316f, (308f / f32(_e2051)), f32((_e2051 - 1u)));
                            phi_88_ = ((select(5u, 17u, _e2050) + (_e2058 * 2u)) + select(0u, 1u, (_e297 > select(_e549, _e2047, _e2050))));
                        }
                        let _e2076 = phi_88_;
                        phi_89_ = _e2076;
                    }
                    let _e2078 = phi_89_;
                    phi_90_ = _e2078;
                    phi_91_ = u32();
                    phi_92_ = true;
                } else {
                    if (_e297 < 54f) {
                        let _e2017 = ((_e548 - 0.5295082f) * 4.1666665f);
                        let _e2019 = select(_e2017, 0f, (_e2017 < 0f));
                        let _e2021 = select(_e2019, 1f, (_e2019 > 1f));
                        let _e2026 = (126f * ((_e2021 * _e2021) * (3f - (2f * _e2021))));
                        if (abs((_e2002 - (154f - _e2026))) < 20f) {
                            phi_84_ = 2u;
                        } else {
                            phi_84_ = select(1u, 3u, (abs((_e2002 - (154f + _e2026))) < 20f));
                        }
                        let _e2037 = phi_84_;
                        phi_85_ = _e2037;
                        phi_86_ = u32();
                        phi_87_ = true;
                    } else {
                        let _e2005 = cantus_render_tempestas_cell_index(_e2002, 0f, 44f, 6f);
                        let _e2006 = (_e297 < 82f);
                        if _e2006 {
                            phi_82_ = (27u + _e2005);
                            phi_83_ = u32();
                        } else {
                            let _e2007 = cantus_render_tempestas_cell_index(_e297, 84f, 24f, 5f);
                            phi_82_ = u32();
                            phi_83_ = ((34u + (_e2007 * 7u)) + _e2005);
                        }
                        let _e2013 = phi_82_;
                        let _e2015 = phi_83_;
                        phi_85_ = _e2013;
                        phi_86_ = _e2015;
                        phi_87_ = _e2006;
                    }
                    let _e2039 = phi_85_;
                    let _e2041 = phi_86_;
                    let _e2043 = phi_87_;
                    phi_90_ = _e2039;
                    phi_91_ = _e2041;
                    phi_92_ = _e2043;
                }
                let _e2080 = phi_90_;
                let _e2082 = phi_91_;
                let _e2084 = phi_92_;
                phi_93_ = _e2080;
                phi_94_ = _e2082;
                phi_95_ = _e2084;
            }
            let _e2086 = phi_93_;
            let _e2088 = phi_94_;
            let _e2090 = phi_95_;
            let _e2091 = select(_e2088, _e2086, _e2090);
            if (_e2091 < arrayLength((&text_lines.member))) {
            } else {
                break;
            }
            let _e2095 = text_lines.member[_e2091];
            let _e2097 = unpack4x8unorm(_e2095.color);
            if (_e217.x < _e2095.min.x) {
                phi_133_ = f32();
                phi_134_ = true;
            } else {
                if (_e217.x > _e2095.max.x) {
                    phi_131_ = f32();
                    phi_132_ = true;
                } else {
                    if (_e217.y < _e2095.min.y) {
                        phi_129_ = f32();
                        phi_130_ = true;
                    } else {
                        let _e2109 = (_e217.y > _e2095.max.y);
                        if _e2109 {
                            phi_128_ = f32();
                        } else {
                            let _e2112 = (1f / _e2095.size);
                            let _e2119 = ((_e217.x - _e2095.origin.x) * _e2112);
                            phi_96_ = _e2095.count;
                            phi_97_ = 0u;
                            loop {
                                let _e2122 = phi_96_;
                                let _e2124 = phi_97_;
                                local_50 = _e2124;
                                let _e2125 = (_e2124 < _e2122);
                                if _e2125 {
                                    let _e2128 = (_e2124 + ((_e2122 - _e2124) / 2u));
                                    let _e2130 = (_e2095.first + _e2128);
                                    if (_e2130 < _e223) {
                                    } else {
                                        phi_101_ = true;
                                        break;
                                    }
                                    let _e2135 = placed_glyphs_2.member[_e2130].x;
                                    let _e2136 = (_e2135 <= _e2119);
                                    if _e2136 {
                                        phi_98_ = (_e2128 + 1u);
                                    } else {
                                        phi_98_ = _e2124;
                                    }
                                    let _e2139 = phi_98_;
                                    phi_99_ = select(_e2128, _e2122, _e2136);
                                    phi_100_ = _e2139;
                                } else {
                                    phi_99_ = u32();
                                    phi_100_ = u32();
                                }
                                let _e2142 = phi_99_;
                                let _e2144 = phi_100_;
                                continue;
                                continuing {
                                    phi_96_ = _e2142;
                                    phi_97_ = _e2144;
                                    phi_101_ = _e480;
                                    break if !(_e2125);
                                }
                            }
                            let _e2147 = phi_101_;
                            if _e2147 {
                                break;
                            }
                            let _e2148 = (3.5f / _e2095.size);
                            let _e2150 = local_50;
                            let _e2151 = (_e2150 + 1u);
                            phi_102_ = _e2147;
                            phi_103_ = select(_e2151, _e2095.count, (_e2095.count < _e2151));
                            phi_104_ = -1000000f;
                            loop {
                                let _e2155 = phi_102_;
                                let _e2157 = phi_103_;
                                let _e2159 = phi_104_;
                                local_53 = _e2159;
                                if (_e2157 > 0u) {
                                    let _e2161 = (_e2157 - 1u);
                                    let _e2163 = (_e2095.first + _e2161);
                                    if (_e2163 < _e223) {
                                    } else {
                                        phi_127_ = true;
                                        break;
                                    }
                                    let _e2168 = placed_glyphs_2.member[_e2163].x;
                                    let _e2172 = placed_glyphs_2.member[_e2163].glyph;
                                    if (_e2172 < arrayLength((&glyphs_2.member))) {
                                    } else {
                                        phi_127_ = true;
                                        break;
                                    }
                                    let _e2178 = glyphs_2.member[_e2172].min[0u];
                                    let _e2183 = glyphs_2.member[_e2172].min[1u];
                                    let _e2188 = glyphs_2.member[_e2172].max[0u];
                                    let _e2193 = glyphs_2.member[_e2172].max[1u];
                                    let _e2197 = glyphs_2.member[_e2172].start;
                                    let _e2201 = glyphs_2.member[_e2172].count;
                                    let _e2202 = (_e2119 - _e2168);
                                    let _e2203 = -(((_e217.y - _e2095.origin.y) * _e2112));
                                    let _e2204 = (_e2188 + _e2148);
                                    let _e2205 = (_e2202 > _e2204);
                                    if _e2205 {
                                        phi_121_ = _e2155;
                                        phi_122_ = f32();
                                    } else {
                                        if (_e2202 >= (_e2178 - _e2148)) {
                                            if (_e2203 >= (_e2183 - _e2148)) {
                                                if (_e2202 <= _e2204) {
                                                    if (_e2203 <= (_e2193 + _e2148)) {
                                                        phi_105_ = 340282350000000000000000000000000000000f;
                                                        phi_106_ = 0u;
                                                        phi_107_ = 0i;
                                                        loop {
                                                            let _e2215 = phi_105_;
                                                            let _e2217 = phi_106_;
                                                            let _e2219 = phi_107_;
                                                            local_51 = _e2215;
                                                            local_52 = _e2219;
                                                            let _e2220 = (_e2217 < _e2201);
                                                            if _e2220 {
                                                                let _e2221 = (_e2197 + _e2217);
                                                                if (_e2221 < arrayLength((&edges_2.member))) {
                                                                } else {
                                                                    phi_111_ = true;
                                                                    break;
                                                                }
                                                                let _e2225 = edges_2.member[_e2221];
                                                                let _e2227 = cantus_render_text_edge_distance(_e2225, _e2095.weight, vec2<f32>(_e2202, _e2203), _e2215);
                                                                phi_108_ = _e2227.member;
                                                                phi_109_ = (_e2217 + 1u);
                                                                phi_110_ = (_e2219 + _e2227.member_1);
                                                            } else {
                                                                phi_108_ = f32();
                                                                phi_109_ = u32();
                                                                phi_110_ = i32();
                                                            }
                                                            let _e2233 = phi_108_;
                                                            let _e2235 = phi_109_;
                                                            let _e2237 = phi_110_;
                                                            continue;
                                                            continuing {
                                                                phi_105_ = _e2233;
                                                                phi_106_ = _e2235;
                                                                phi_107_ = _e2237;
                                                                phi_111_ = _e2155;
                                                                break if !(_e2220);
                                                            }
                                                        }
                                                        let _e2240 = phi_111_;
                                                        phi_127_ = _e2240;
                                                        if _e2240 {
                                                            break;
                                                        }
                                                        let _e2242 = local_51;
                                                        let _e2246 = local_52;
                                                        let _e2249 = ((sqrt(_e2242) * _e2095.size) * select(1f, -1f, (_e2246 == 0i)));
                                                        if (_e2159 != _e2159) {
                                                            phi_112_ = true;
                                                        } else {
                                                            phi_112_ = (_e2249 >= _e2159);
                                                        }
                                                        let _e2253 = phi_112_;
                                                        phi_113_ = _e2240;
                                                        phi_114_ = select(_e2159, _e2249, _e2253);
                                                    } else {
                                                        phi_113_ = _e2155;
                                                        phi_114_ = _e2159;
                                                    }
                                                    let _e2256 = phi_113_;
                                                    let _e2258 = phi_114_;
                                                    phi_115_ = _e2256;
                                                    phi_116_ = _e2258;
                                                } else {
                                                    phi_115_ = _e2155;
                                                    phi_116_ = _e2159;
                                                }
                                                let _e2260 = phi_115_;
                                                let _e2262 = phi_116_;
                                                phi_117_ = _e2260;
                                                phi_118_ = _e2262;
                                            } else {
                                                phi_117_ = _e2155;
                                                phi_118_ = _e2159;
                                            }
                                            let _e2264 = phi_117_;
                                            let _e2266 = phi_118_;
                                            phi_119_ = _e2264;
                                            phi_120_ = _e2266;
                                        } else {
                                            phi_119_ = _e2155;
                                            phi_120_ = _e2159;
                                        }
                                        let _e2268 = phi_119_;
                                        let _e2270 = phi_120_;
                                        phi_121_ = _e2268;
                                        phi_122_ = _e2270;
                                    }
                                    let _e2272 = phi_121_;
                                    let _e2274 = phi_122_;
                                    phi_123_ = _e2272;
                                    phi_124_ = _e2161;
                                    phi_125_ = _e2274;
                                    phi_126_ = select(true, false, _e2205);
                                } else {
                                    phi_123_ = _e2155;
                                    phi_124_ = u32();
                                    phi_125_ = f32();
                                    phi_126_ = false;
                                }
                                let _e2277 = phi_123_;
                                let _e2279 = phi_124_;
                                let _e2281 = phi_125_;
                                let _e2283 = phi_126_;
                                continue;
                                continuing {
                                    phi_102_ = _e2277;
                                    phi_103_ = _e2279;
                                    phi_104_ = _e2281;
                                    phi_127_ = _e2277;
                                    break if !(_e2283);
                                }
                            }
                            let _e2286 = phi_127_;
                            if _e2286 {
                                break;
                            }
                            let _e2288 = local_53;
                            let _e2290 = ((_e2288 * 1.25f) + 0.5f);
                            let _e2292 = select(_e2290, 0f, (_e2290 < 0f));
                            let _e2294 = select(_e2292, 1f, (_e2292 > 1f));
                            phi_128_ = ((_e2294 * _e2294) * (3f - (2f * _e2294)));
                        }
                        let _e2300 = phi_128_;
                        phi_129_ = _e2300;
                        phi_130_ = _e2109;
                    }
                    let _e2302 = phi_129_;
                    let _e2304 = phi_130_;
                    phi_131_ = _e2302;
                    phi_132_ = _e2304;
                }
                let _e2306 = phi_131_;
                let _e2308 = phi_132_;
                phi_133_ = _e2306;
                phi_134_ = _e2308;
            }
            let _e2310 = phi_133_;
            let _e2312 = phi_134_;
            let _e2315 = (select(_e2310, 0f, _e2312) * _e2097.w);
            let _e2316 = (1f - _e2315);
            out_color = vec4<f32>((((((_e1973.x * _e1985) + (((_e1973.x * 1.5f) + 0.1f) * _e1990)) * _e2316) + (_e2097.x * _e2315)) * _e785), (((((_e1973.y * _e1985) + (((_e1973.y * 1.5f) + 0.1f) * _e1993)) * _e2316) + (_e2097.y * _e2315)) * _e785), (((((_e1973.z * _e1985) + (((_e1973.z * 1.5f) + 0.1f) * _e1996)) * _e2316) + (_e2097.z * _e2315)) * _e785), _e798);
            break;
        }
    }
    return;
}

@vertex
fn render_track_isthmus_trackpass_vertex(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    vertex_5 = vertex;
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
fn render_status_isthmus_statuspass_vertex(@builtin(vertex_index) vertex_1: u32, @builtin(instance_index) _isthmus_instance_index: u32) -> VertexOutput {
    vertex_5 = vertex_1;
    _isthmus_instance_index_7 = _isthmus_instance_index;
    render_status_isthmus_statuspass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_status_isthmus_statuspass_fragment(@location(0) pixel: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_1: u32) -> @location(0) vec4<f32> {
    pixel_2 = pixel;
    _isthmus_instance_index_8 = _isthmus_instance_index_1;
    render_status_isthmus_statuspass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_playhead_isthmus_playheadpass_vertex(@builtin(vertex_index) vertex_2: u32, @builtin(instance_index) _isthmus_instance_index_2: u32) -> VertexOutput {
    vertex_5 = vertex_2;
    _isthmus_instance_index_7 = _isthmus_instance_index_2;
    render_playhead_isthmus_playheadpass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_world_pos;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_playhead_isthmus_playheadpass_fragment(@location(0) world_pos: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_3: u32) -> @location(0) vec4<f32> {
    world_pos_1 = world_pos;
    _isthmus_instance_index_8 = _isthmus_instance_index_3;
    render_playhead_isthmus_playheadpass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_particles_isthmus_particlepass_vertex(@builtin(vertex_index) vertex_3: u32, @builtin(instance_index) _isthmus_instance_index_4: u32) -> VertexOutput_1 {
    vertex_5 = vertex_3;
    _isthmus_instance_index_7 = _isthmus_instance_index_4;
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
fn render_tempestas_isthmus_tempestaspass_vertex(@builtin(vertex_index) vertex_4: u32, @builtin(instance_index) _isthmus_instance_index_5: u32) -> VertexOutput_2 {
    vertex_5 = vertex_4;
    _isthmus_instance_index_7 = _isthmus_instance_index_5;
    render_tempestas_isthmus_tempestaspass_vertex_impl();
    let _e8 = out_position;
    let _e9 = out_pixel;
    let _e10 = out_weather;
    let _e11 = out_isthmus_instance_index_1;
    return VertexOutput_2(_e8, _e9, _e10, _e11);
}

@fragment
fn render_tempestas_isthmus_tempestaspass_fragment(@location(0) pixel_1: vec2<f32>, @location(1) @interpolate(flat) weather: vec4<f32>, @location(2) @interpolate(flat) _isthmus_instance_index_6: u32) -> @location(0) vec4<f32> {
    pixel_2 = pixel_1;
    weather_1 = weather;
    _isthmus_instance_index_9 = _isthmus_instance_index_6;
    render_tempestas_isthmus_tempestaspass_fragment_impl();
    let _e7 = out_color;
    return _e7;
}
