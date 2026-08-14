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
    var local_12: i32;
    var phi_76_: bool;
    var phi_77_: f32;
    var phi_78_: f32;
    var phi_79_: f32;
    var phi_80_: f32;
    var phi_81_: f32;
    var phi_82_: u32;
    var phi_83_: f32;
    var phi_84_: bool;
    var local_13: f32;
    var phi_85_: f32;
    var phi_86_: f32;
    var phi_87_: bool;
    var phi_88_: f32;
    var phi_89_: bool;
    var phi_90_: f32;
    var phi_91_: bool;
    var phi_92_: u32;
    var phi_93_: u32;
    var phi_94_: u32;
    var phi_95_: u32;
    var phi_96_: u32;
    var local_14: u32;
    var phi_97_: u32;
    var phi_98_: f32;
    var phi_99_: f32;
    var phi_100_: u32;
    var phi_101_: i32;
    var phi_102_: f32;
    var phi_103_: u32;
    var phi_104_: i32;
    var local_15: f32;
    var local_16: i32;
    var phi_105_: bool;
    var phi_106_: f32;
    var phi_107_: f32;
    var phi_108_: f32;
    var phi_109_: f32;
    var phi_110_: f32;
    var phi_111_: u32;
    var phi_112_: f32;
    var phi_113_: bool;
    var local_17: f32;
    var phi_114_: f32;
    var phi_115_: f32;
    var phi_116_: bool;
    var phi_117_: f32;
    var phi_118_: bool;
    var phi_119_: f32;
    var phi_120_: bool;
    var phi_121_: bool;
    var local_18: vec4<f32>;
    var local_19: vec4<f32>;
    var local_20: vec4<f32>;
    var local_21: vec4<f32>;
    var local_22: vec4<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e165 = pixel_pos_1;
            let _e166 = pill_idx_1;
            let _e172 = pill.member[_e166].x;
            let _e176 = pill.member[_e166].width;
            let _e180 = frame.member[0u].panel_height;
            let _e181 = (_e165.x - _e172);
            let _e182 = (_e165.y - 6f);
            let _e183 = (_e176 * 0.5f);
            let _e184 = (_e180 * 0.5f);
            let _e186 = (_e182 - _e184);
            let _e187 = (_e176 - _e180);
            let _e188 = (_e187 * 0.5f);
            let _e190 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e181 - _e183), _e186), _e188, _e184);
            let _e194 = frame.member[0u].mouse_pressure;
            let _e195 = (_e194 > 0f);
            if _e195 {
                let _e200 = frame.member[0u].mouse_pos[0u];
                let _e205 = frame.member[0u].mouse_pos[1u];
                let _e211 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e200 - _e172) - _e183), ((_e205 - 6f) - _e184)), _e188, _e184);
                phi_0_ = _e211;
            } else {
                phi_0_ = 1f;
            }
            let _e213 = phi_0_;
            phi_1_ = vec2<f32>(0f, 0f);
            phi_2_ = 0f;
            phi_3_ = 0u;
            loop {
                let _e215 = phi_1_;
                let _e217 = phi_2_;
                let _e219 = phi_3_;
                local_2 = _e215;
                local_3 = _e215;
                local_4 = _e215;
                local_5 = _e215;
                local_6 = _e217;
                local_7 = _e217;
                local_8 = _e217;
                local_9 = _e217;
                let _e220 = (_e219 < 4u);
                if _e220 {
                    if _e220 {
                    } else {
                        phi_13_ = true;
                        break;
                    }
                    let _e227 = frame.member[0u].ripples[_e219].origin[0u];
                    let _e234 = frame.member[0u].ripples[_e219].origin[1u];
                    let _e240 = frame.member[0u].ripples[_e219].start_time;
                    let _e246 = frame.member[0u].ripples[_e219].strength;
                    let _e250 = frame.member[0u].time;
                    let _e252 = ((_e250 - _e240) * 1.2f);
                    let _e254 = select(_e252, 0f, (_e252 < 0f));
                    let _e256 = select(_e254, 1f, (_e254 > 1f));
                    if (_e246 > 0f) {
                        if (_e256 < 1f) {
                            let _e259 = (_e165.x - _e227);
                            let _e260 = (_e165.y - _e234);
                            let _e264 = sqrt(((_e259 * _e259) + (_e260 * _e260)));
                            if (_e264 > 0.001f) {
                                phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e259 / _e264), (_e260 / _e264)), _e264);
                            } else {
                                phi_4_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e264);
                            }
                            let _e272 = phi_4_;
                            let _e282 = ((abs((_e272.unnamed_1 - (_e256 * 600f))) - 80f) * -0.0125f);
                            let _e284 = select(_e282, 0f, (_e282 < 0f));
                            let _e286 = select(_e284, 1f, (_e284 > 1f));
                            let _e292 = (1f - _e256);
                            let _e293 = ((((_e286 * _e286) * (3f - (2f * _e286))) * _e246) * _e292);
                            let _e306 = (_e217 + (_e293 * 0.5f));
                            if (_e306 != _e306) {
                                phi_5_ = true;
                            } else {
                                phi_5_ = (1f <= _e306);
                            }
                            let _e310 = phi_5_;
                            phi_6_ = vec2<f32>((_e215.x + (((_e272.unnamed.x * _e293) * _e292) * 0.5f)), (_e215.y + (((_e272.unnamed.y * _e293) * _e292) * 0.5f)));
                            phi_7_ = select(_e306, 1f, _e310);
                        } else {
                            phi_6_ = _e215;
                            phi_7_ = _e217;
                        }
                        let _e313 = phi_6_;
                        let _e315 = phi_7_;
                        phi_8_ = _e313;
                        phi_9_ = _e315;
                    } else {
                        phi_8_ = _e215;
                        phi_9_ = _e217;
                    }
                    let _e317 = phi_8_;
                    let _e319 = phi_9_;
                    phi_10_ = _e317;
                    phi_11_ = _e319;
                    phi_12_ = (_e219 + 1u);
                } else {
                    phi_10_ = vec2<f32>();
                    phi_11_ = f32();
                    phi_12_ = u32();
                }
                let _e322 = phi_10_;
                let _e324 = phi_11_;
                let _e326 = phi_12_;
                continue;
                continuing {
                    phi_1_ = _e322;
                    phi_2_ = _e324;
                    phi_3_ = _e326;
                    phi_13_ = false;
                    break if !(_e220);
                }
            }
            let _e329 = phi_13_;
            if _e329 {
                break;
            }
            if _e195 {
                let _e334 = frame.member[0u].mouse_pos[0u];
                let _e339 = frame.member[0u].mouse_pos[1u];
                let _e340 = (_e165.x - _e334);
                let _e341 = (_e165.y - _e339);
                let _e347 = ((sqrt(((_e340 * _e340) + (_e341 * _e341))) - 150f) * -0.006666667f);
                let _e349 = select(_e347, 0f, (_e347 < 0f));
                let _e351 = select(_e349, 1f, (_e349 > 1f));
                phi_14_ = ((((_e351 * _e351) * (3f - (2f * _e351))) * _e194) * 8f);
            } else {
                phi_14_ = 0f;
            }
            let _e359 = phi_14_;
            let _e361 = local_2;
            let _e364 = global[0u];
            if (_e361.x == _e364) {
                let _e367 = local_3;
                let _e370 = global[1u];
                phi_15_ = (_e367.y == _e370);
            } else {
                phi_15_ = false;
            }
            let _e373 = phi_15_;
            if _e373 {
                phi_16_ = 0f;
            } else {
                let _e375 = local_4;
                phi_16_ = (sqrt(((_e361.x * _e361.x) + (_e375.y * _e375.y))) * 22f);
            }
            let _e383 = phi_16_;
            let _e385 = local_5;
            let _e387 = (_e181 / _e176);
            let _e388 = (_e182 / _e180);
            let _e389 = (_e387 - 0.5f);
            let _e390 = (_e388 - 0.5f);
            let _e391 = (_e172 + _e183);
            let _e392 = (_e180 * 0.975f);
            let _e393 = (_e392 + 3f);
            let _e397 = pill.member[_e166].rating;
            let _e398 = (_e397 >= 0i);
            let _e399 = select(0f, 5f, _e398);
            let _e403 = pill.member[_e166].primary_playlist_count;
            let _e405 = (_e399 + f32(_e403));
            let _e411 = pill.member[_e166].secondary_expansion;
            let _e413 = (_e393 + (18f * _e411));
            let _e417 = pill.member[_e166].secondary_playlist_count;
            let _e418 = f32(_e417);
            let _e425 = frame.member[0u].mouse_pos[0u];
            let _e430 = frame.member[0u].mouse_pos[1u];
            let _e431 = vec2<f32>(_e425, _e430);
            let _e433 = (_e405 - 1f);
            let _e434 = (_e433 != _e433);
            if _e434 {
                phi_17_ = true;
            } else {
                phi_17_ = (0f >= _e433);
            }
            let _e437 = phi_17_;
            let _e440 = vec2<f32>(_e391, (_e392 + -4.4f));
            let _e442 = cantus_render_shader_sd_capsule_box((_e165 - _e440), (select(_e433, 0f, _e437) * 9f), 9f);
            if _e434 {
                phi_18_ = true;
            } else {
                phi_18_ = (0f >= _e433);
            }
            let _e445 = phi_18_;
            let _e449 = cantus_render_shader_sd_capsule_box((_e431 - _e440), (select(_e433, 0f, _e445) * 9f), 9f);
            let _e450 = (10.5f * _e411);
            let _e452 = (_e418 - 1f);
            let _e453 = (_e452 != _e452);
            if _e453 {
                phi_19_ = true;
            } else {
                phi_19_ = (0f >= _e452);
            }
            let _e456 = phi_19_;
            let _e461 = vec2<f32>(_e391, (_e413 + -5.4f));
            let _e463 = cantus_render_shader_sd_capsule_box((_e165 - _e461), (((select(_e452, 0f, _e456) * 18f) * _e411) * 0.5f), _e450);
            if _e453 {
                phi_20_ = true;
            } else {
                phi_20_ = (0f >= _e452);
            }
            let _e466 = phi_20_;
            let _e472 = cantus_render_shader_sd_capsule_box((_e431 - _e461), (((select(_e452, 0f, _e466) * 18f) * _e411) * 0.5f), _e450);
            let _e476 = pill.member[_e166].primary_alpha;
            let _e479 = (0.5f + ((_e442 - _e190) * 0.05f));
            let _e481 = select(_e479, 0f, (_e479 < 0f));
            let _e483 = select(_e481, 1f, (_e481 > 1f));
            let _e493 = (_e190 + ((((_e442 + ((_e190 - _e442) * _e483)) - ((10f * _e483) * (1f - _e483))) - _e190) * _e476));
            let _e496 = (0.5f + ((_e449 - _e213) * 0.05f));
            let _e498 = select(_e496, 0f, (_e496 < 0f));
            let _e500 = select(_e498, 1f, (_e498 > 1f));
            let _e510 = (_e213 + ((((_e449 + ((_e213 - _e449) * _e500)) - ((10f * _e500) * (1f - _e500))) - _e213) * _e476));
            let _e512 = select(0f, 1f, (_e411 > 0f));
            let _e515 = (0.5f + ((_e463 - _e493) * 0.046296295f));
            let _e517 = select(_e515, 0f, (_e515 < 0f));
            let _e519 = select(_e517, 1f, (_e517 > 1f));
            let _e532 = (0.5f + ((_e472 - _e510) * 0.046296295f));
            let _e534 = select(_e532, 0f, (_e532 < 0f));
            let _e536 = select(_e534, 1f, (_e534 > 1f));
            let _e548 = (((_e510 + ((((_e472 + ((_e510 - _e472) * _e536)) - ((10.8f * _e536) * (1f - _e536))) - _e510) * _e512)) - 0.5f) * -1f);
            let _e550 = select(_e548, 0f, (_e548 < 0f));
            let _e552 = select(_e550, 1f, (_e550 > 1f));
            let _e559 = (((_e359 * ((_e552 * _e552) * (3f - (2f * _e552)))) + _e383) * 0.5f);
            let _e560 = ((_e493 + ((((_e463 + ((_e493 - _e463) * _e519)) - ((10.8f * _e519) * (1f - _e519))) - _e493) * _e512)) - _e559);
            let _e561 = fwidth(_e560);
            if (_e561 != _e561) {
                phi_21_ = true;
            } else {
                phi_21_ = (0.55f >= _e561);
            }
            let _e565 = phi_21_;
            let _e566 = select(_e561, 0.55f, _e565);
            let _e570 = ((_e560 - _e566) / (-(_e566) - _e566));
            let _e572 = select(_e570, 0f, (_e570 < 0f));
            let _e574 = select(_e572, 1f, (_e572 > 1f));
            let _e578 = ((_e574 * _e574) * (3f - (2f * _e574)));
            let _e579 = (_e560 != _e560);
            if _e579 {
                phi_22_ = true;
            } else {
                phi_22_ = (0f >= _e560);
            }
            let _e582 = phi_22_;
            let _e586 = (exp((select(_e560, 0f, _e582) * -0.3f)) * 0.16f);
            if (_e578 != _e578) {
                phi_23_ = true;
            } else {
                phi_23_ = (_e586 >= _e578);
            }
            let _e590 = phi_23_;
            let _e591 = select(_e578, _e586, _e590);
            let _e595 = pill.member[_e166].visibility;
            if ((_e591 * _e595) <= 0.0009765625f) {
                discard;
            }
            if _e579 {
                phi_24_ = true;
            } else {
                phi_24_ = (0f <= _e560);
            }
            let _e600 = phi_24_;
            let _e603 = (1f + (select(_e560, 0f, _e600) * 0.008333334f));
            let _e605 = select(_e603, 0f, (_e603 < 0f));
            let _e607 = select(_e605, 0.6f, (_e605 > 0.6f));
            let _e617 = ((_e388 - ((_e390 * _e607) * 0.08f)) - (_e385.y * 0.04f));
            let _e618 = (((_e387 - ((_e389 * _e607) * 0.08f)) - (_e361.x * 0.04f)) * _e176);
            let _e619 = (_e617 * _e180);
            let _e623 = pill.member[_e166].effects;
            let _e627 = frame.member[0u].time;
            let _e631 = pill.member[_e166].seed;
            let _e634 = ((_e623.tempo - 0.2f) * 2.5f);
            let _e636 = select(_e634, 0f, (_e634 < 0f));
            let _e645 = ((_e627 * ((0.12f + (_e623.energy * 0.25f)) + (select(_e636, 1f, (_e636 > 1f)) * 0.12f))) + _e631);
            let _e650 = ((sin(((_e627 * _e623.tempo) * 31.415928f)) * 0.5f) + 0.5f);
            let _e656 = (((_e650 * _e650) * _e623.danceability) * (0.025f + (_e623.energy * 0.055f)));
            let _e657 = (_e623.energy * 0.55f);
            let _e662 = ((_e657 + (_e623.danceability * 0.25f)) + (_e623.loudness * 0.2f));
            if _e579 {
                phi_25_ = true;
            } else {
                phi_25_ = (0f <= _e560);
            }
            let _e665 = phi_25_;
            let _e668 = (1f + (select(_e560, 0f, _e665) * 0.008333334f));
            let _e670 = select(_e668, 0f, (_e668 < 0f));
            let _e672 = select(_e670, 1f, (_e670 > 1f));
            let _e683 = (_e631 - trunc(_e631));
            let _e688 = ((_e176 / _e180) * ((0.5f + (_e683 * 0.12f)) + (_e662 * 0.18f)));
            if (_e688 != _e688) {
                phi_26_ = true;
            } else {
                phi_26_ = (1.7f >= _e688);
            }
            let _e692 = phi_26_;
            let _e695 = select(0f, _e387, (_e387 > 0f));
            let _e697 = select(0f, _e388, (_e388 > 0f));
            let _e705 = (select(1f, _e697, (_e697 < 1f)) - (((((_e390 * _e672) * _e672) * 0.6f) + _e385.y) * 0.08f));
            let _e706 = ((select(1f, _e695, (_e695 < 1f)) - (((((_e389 * _e672) * _e672) * 0.6f) + _e361.x) * 0.08f)) * select(_e688, 1.7f, _e692));
            let _e717 = (_e645 * 0.8f);
            let _e727 = ((0.14f + (_e662 * 0.2f)) + _e656);
            let _e732 = (_e631 + 1.5707964f);
            let _e737 = pill.member[_e166].colors[0u];
            let _e739 = vec2<f32>((_e706 + ((sin(((_e705 * 4.32f) + _e645)) + cos(((_e706 * 1.3f) - (_e645 * 0.7f)))) * _e727)), ((_e705 * 1.6f) + ((cos(((_e706 * 2.3f) - _e717)) + sin(((_e705 * 2.72f) + (_e645 * 0.6f)))) * _e727)));
            let _e740 = cantus_render_track_plasma_field(_e739, unpack4x8unorm(_e737), 2.1f, 0.7f, _e645);
            let _e745 = pill.member[_e166].colors[1u];
            let _e748 = cantus_render_track_plasma_field(_e739, unpack4x8unorm(_e745), 0.6f, -2.4f, (_e732 - _e717));
            let _e765 = pill.member[_e166].colors[2u];
            let _e769 = cantus_render_track_plasma_field(_e739, unpack4x8unorm(_e765), -1.5f, 1.9f, ((_e645 * 0.65f) + 2f));
            let _e782 = pill.member[_e166].colors[3u];
            let _e783 = unpack4x8unorm(_e782);
            let _e786 = cantus_render_track_plasma_field(_e739, _e783, 2.4f, 1.6f, (_e732 - (_e645 * 0.55f)));
            let _e794 = (((_e740.w + _e748.w) + _e769.w) + _e786.w);
            let _e795 = ((((_e740.x + _e748.x) + _e769.x) + _e786.x) / _e794);
            let _e796 = ((((_e740.y + _e748.y) + _e769.y) + _e786.y) / _e794);
            let _e797 = ((((_e740.z + _e748.z) + _e769.z) + _e786.z) / _e794);
            let _e802 = (((_e795 * 0.2126f) + (_e796 * 0.7152f)) + (_e797 * 0.0722f));
            let _e806 = frame.member[0u].playhead_x;
            let _e807 = (_e806 + 3f);
            let _e811 = ((_e165.x - _e807) / ((_e806 - 3f) - _e807));
            let _e813 = select(_e811, 0f, (_e811 < 0f));
            let _e815 = select(_e813, 1f, (_e813 > 1f));
            let _e824 = pill.member[_e166].effects.valence;
            let _e825 = (_e824 * 0.4f);
            let _e826 = (1.55f + _e825);
            let _e828 = (_e802 * (-0.54999995f - _e825));
            let _e832 = (_e828 + (_e795 * _e826));
            let _e833 = (_e828 + (_e796 * _e826));
            let _e834 = (_e828 + (_e797 * _e826));
            let _e836 = select(0.035f, _e832, (_e832 > 0.035f));
            let _e838 = select(0.035f, _e833, (_e833 > 0.035f));
            let _e840 = select(0.035f, _e834, (_e834 > 0.035f));
            if (_e802 != _e802) {
                phi_27_ = true;
            } else {
                phi_27_ = (0.001f >= _e802);
            }
            let _e850 = phi_27_;
            let _e852 = (0.52f / select(_e802, 0.001f, _e850));
            if (_e852 != _e852) {
                phi_28_ = true;
            } else {
                phi_28_ = (1f <= _e852);
            }
            let _e856 = phi_28_;
            let _e857 = select(_e852, 1f, _e856);
            let _e864 = ((0.96f + (_e824 * 0.06f)) + (_e656 * 0.5f));
            let _e869 = ((_e617 - 0.45f) * 1.8181818f);
            let _e871 = select(_e869, 0f, (_e869 < 0f));
            let _e873 = select(_e871, 1f, (_e871 > 1f));
            let _e879 = (0.84f + (((_e873 * _e873) * (3f - (2f * _e873))) * 0.1f));
            let _e884 = (1f - (0.4f * ((_e815 * _e815) * (3f - (2f * _e815)))));
            let _e904 = (8f - _e623.acousticness);
            let _e908 = (_e627 * (0.35f + _e657));
            let _e911 = ((_e181 / _e904) + (_e908 * (0.16f + (_e683 * 0.08f))));
            let _e912 = ((_e182 / _e904) + (_e908 * (0.055f + (sin((_e631 * 0.7f)) * 0.025f))));
            let _e913 = floor(_e911);
            let _e914 = floor(_e912);
            let _e923 = bitcast<u32>(select(0i, select(select(i32(_e914), i32(-2147483648), (_e914 < -2147483600f)), 2147483647i, (_e914 > 2147483500f)), (_e914 == _e914)));
            let _e931 = bitcast<u32>(select(0i, select(select(i32(_e913), i32(-2147483648), (_e913 < -2147483600f)), 2147483647i, (_e913 > 2147483500f)), (_e913 == _e913)));
            let _e933 = (bitcast<u32>((_e631 + 2.71f)) * 2654435761u);
            let _e939 = (((_e931 ^ _e933) * 1664525u) + 1013904223u);
            let _e941 = ((((_e923 ^ _e933) * 1664525u) + 1013904223u) + (_e939 * 1664525u));
            let _e943 = (_e939 + (_e941 * 1664525u));
            let _e951 = ((_e941 ^ (_e941 >> bitcast<u32>(16i))) + ((_e943 ^ (_e943 >> bitcast<u32>(16i))) * 1664525u));
            let _e955 = f32((_e951 ^ (_e951 >> bitcast<u32>(16i))));
            let _e956 = (_e955 * 0.0000000016600825f);
            let _e970 = (_e623.acousticness * 0.09f);
            let _e973 = (bitcast<u32>(_e631) * 2654435761u);
            let _e979 = (((_e923 ^ _e973) * 1664525u) + 1013904223u);
            let _e981 = ((((_e931 ^ _e973) * 1664525u) + 1013904223u) + (_e979 * 1664525u));
            let _e983 = (_e979 + (_e981 * 1664525u));
            let _e991 = ((_e981 ^ (_e981 >> bitcast<u32>(16i))) + ((_e983 ^ (_e983 >> bitcast<u32>(16i))) * 1664525u));
            let _e999 = (((f32((_e991 ^ (_e991 >> bitcast<u32>(16i)))) * 0.00000000023283064f) - (0.985f - _e970)) / (_e970 + 0.014999986f));
            let _e1001 = select(_e999, 0f, (_e999 < 0f));
            let _e1003 = select(_e1001, 1f, (_e1001 > 1f));
            let _e1012 = (((_e911 - _e913) - 0.5f) - ((_e955 * 0.00000000013038516f) - 0.28f));
            let _e1013 = (((_e912 - _e914) - 0.5f) - (((_e956 - trunc(_e956)) * 0.56f) - 0.28f));
            let _e1019 = ((sqrt(((_e1012 * _e1012) + (_e1013 * _e1013))) - 0.06f) * 4.5454545f);
            let _e1021 = select(_e1019, 0f, (_e1019 < 0f));
            let _e1023 = select(_e1021, 1f, (_e1021 > 1f));
            let _e1036 = (((((_e1003 * _e1003) * (3f - (2f * _e1003))) * (1f - ((_e1023 * _e1023) * (3f - (2f * _e1023))))) * ((sin(((_e627 * ((0.7f + (_e955 * 0.00000000020954757f)) + (_e623.energy * 0.8f))) + (_e955 * 0.0000000014629181f))) * 0.5f) + 0.5f)) * (0.12f + (_e623.acousticness * 0.48f)));
            let _e1040 = (((((select(0.92f, _e836, (_e836 < 0.92f)) * _e857) * _e864) * _e879) * _e884) + (((_e783.x * 0.75f) + 0.25f) * _e1036));
            let _e1041 = (((((select(0.92f, _e838, (_e838 < 0.92f)) * _e857) * _e864) * _e879) * _e884) + (((_e783.y * 0.75f) + 0.25f) * _e1036));
            let _e1042 = (((((select(0.92f, _e840, (_e840 < 0.92f)) * _e857) * _e864) * _e879) * _e884) + (((_e783.z * 0.75f) + 0.25f) * _e1036));
            let _e1049 = (_e181 / _e180);
            if (_e623.instrumentalness <= 0.00390625f) {
                phi_32_ = 0f;
            } else {
                let _e1054 = (_e627 * (0.5f + (_e623.energy * 0.35f)));
                let _e1062 = (sin(((_e388 * 1.9f) + _e1054)) * 0.35f);
                let _e1063 = (sin(((_e1049 * 1.5f) - (_e1054 * 0.8f))) * 0.35f);
                let _e1066 = ((_e1054 * 0.05f) + _e631);
                let _e1067 = ((_e1054 * -0.04f) + _e631);
                let _e1075 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1049 * 0.7f) + _e1062) + _e1066), (((_e388 * 0.7f) + _e1063) + _e1067)));
                let _e1078 = (1f - (abs(_e1075) * 2f));
                if (_e1078 != _e1078) {
                    phi_29_ = true;
                } else {
                    phi_29_ = (0f >= _e1078);
                }
                let _e1082 = phi_29_;
                let _e1083 = select(_e1078, 0f, _e1082);
                let _e1085 = ((_e1083 * _e1083) * _e1083);
                let _e1095 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1049 * 1.1f) - _e1062) - (_e1066 * 0.8f)), (((_e388 * 1.1f) - _e1063) - (_e1067 * 0.8f))));
                let _e1098 = (1f - (abs(_e1095) * 2f));
                if (_e1098 != _e1098) {
                    phi_30_ = true;
                } else {
                    phi_30_ = (0f >= _e1098);
                }
                let _e1102 = phi_30_;
                let _e1103 = select(_e1098, 0f, _e1102);
                let _e1105 = ((_e1103 * _e1103) * _e1103);
                if (_e1085 != _e1085) {
                    phi_31_ = true;
                } else {
                    phi_31_ = (_e1105 >= _e1085);
                }
                let _e1109 = phi_31_;
                phi_32_ = ((select(_e1085, _e1105, _e1109) * _e623.instrumentalness) * 0.06f);
            }
            let _e1114 = phi_32_;
            let _e1118 = (_e1040 + (((_e1040 * 0.25f) + 0.75f) * _e1114));
            let _e1119 = (_e1041 + (((_e1041 * 0.25f) + 0.75f) * _e1114));
            let _e1120 = (_e1042 + (((_e1042 * 0.25f) + 0.75f) * _e1114));
            let _e1121 = vec3<f32>(_e1118, _e1119, _e1120);
            let _e1122 = (_e187 + _e184);
            let _e1126 = pill.member[_e166].image_index;
            if (_e1126 >= 0i) {
                let _e1128 = (_e181 - _e1122);
                let _e1129 = abs(_e1128);
                let _e1130 = abs(_e186);
                if (select(_e1130, _e1129, (_e1129 > _e1130)) < _e180) {
                    let _e1134 = (_e184 + _e559);
                    let _e1140 = (_e1134 * 2f);
                    let _e1146 = vec3<f32>(((_e1128 / _e1140) + 0.5f), ((_e186 / _e1140) + 0.5f), f32(_e1126));
                    let _e1152 = textureSample(images, sampler_, vec2<f32>(_e1146.x, _e1146.y), i32(_e1146.z));
                    let _e1154 = (((sqrt(((_e1128 * _e1128) + (_e186 * _e186))) - _e1134) - -4f) * 0.25f);
                    let _e1156 = select(_e1154, 0f, (_e1154 < 0f));
                    let _e1158 = select(_e1156, 1f, (_e1156 > 1f));
                    let _e1165 = ((_e213 - 0.5f) * -1f);
                    let _e1167 = select(_e1165, 0f, (_e1165 < 0f));
                    let _e1169 = select(_e1167, 1f, (_e1167 > 1f));
                    let _e1178 = ((_e190 - (((_e359 * ((_e1169 * _e1169) * (3f - (2f * _e1169)))) + _e383) * 0.5f)) - -0.5f);
                    let _e1180 = select(_e1178, 0f, (_e1178 < 0f));
                    let _e1182 = select(_e1180, 1f, (_e1180 > 1f));
                    let _e1193 = (((1f - ((_e1158 * _e1158) * (3f - (2f * _e1158)))) * (1f - ((_e1182 * _e1182) * (3f - (2f * _e1182))))) * _e1152.w);
                    let _e1194 = (1f - _e1193);
                    phi_33_ = vec3<f32>(((_e1118 * _e1194) + (_e1152.x * _e1193)), ((_e1119 * _e1194) + (_e1152.y * _e1193)), ((_e1120 * _e1194) + (_e1152.z * _e1193)));
                } else {
                    phi_33_ = _e1121;
                }
                let _e1206 = phi_33_;
                phi_34_ = _e1206;
            } else {
                phi_34_ = _e1121;
            }
            let _e1208 = phi_34_;
            let _e1219 = ((_e617 - 0.12f) * -8.333334f);
            let _e1221 = select(_e1219, 0f, (_e1219 < 0f));
            let _e1223 = select(_e1221, 1f, (_e1221 > 1f));
            let _e1230 = ((_e560 - 5f) * -0.125f);
            let _e1232 = select(_e1230, 0f, (_e1230 < 0f));
            let _e1234 = select(_e1232, 1f, (_e1232 > 1f));
            let _e1240 = ((((_e1223 * _e1223) * (3f - (2f * _e1223))) * 0.12f) + (((_e1234 * _e1234) * (3f - (2f * _e1234))) * 0.08f));
            let _e1244 = (_e1208.x + (((_e1208.x * 0.68f) + 0.32f) * _e1240));
            let _e1245 = (_e1208.y + (((_e1208.y * 0.68f) + 0.32f) * _e1240));
            let _e1246 = (_e1208.z + (((_e1208.z * 0.68f) + 0.32f) * _e1240));
            let _e1254 = local_6;
            let _e1255 = (1f - _e1254);
            let _e1260 = local_7;
            let _e1263 = local_8;
            let _e1266 = local_9;
            let _e1274 = vec4<f32>((((_e1244 * _e1255) + (((_e1244 * 1.5f) + 0.1f) * _e1260)) * _e578), (((_e1245 * _e1255) + (((_e1245 * 1.5f) + 0.1f) * _e1263)) * _e578), (((_e1246 * _e1255) + (((_e1246 * 1.5f) + 0.1f) * _e1266)) * _e578), _e591);
            if _e398 {
                if (_e476 > 0f) {
                    phi_35_ = _e1274;
                    phi_36_ = 0u;
                    loop {
                        let _e1277 = phi_35_;
                        let _e1279 = phi_36_;
                        local_22 = _e1277;
                        let _e1280 = (_e1279 < 5u);
                        if _e1280 {
                            let _e1281 = f32(_e1279);
                            if _e434 {
                                phi_37_ = true;
                            } else {
                                phi_37_ = (0f >= _e433);
                            }
                            let _e1284 = phi_37_;
                            let _e1289 = (_e391 + ((_e1281 - (select(_e433, 0f, _e1284) * 0.5f)) * 18f));
                            let _e1290 = (_e392 + 5f);
                            let _e1291 = (_e165.x - _e1289);
                            let _e1292 = (_e165.y - _e1290);
                            let _e1293 = abs(_e1291);
                            let _e1294 = abs(_e1292);
                            if (select(_e1294, _e1293, (_e1293 > _e1294)) < 38.88f) {
                                let _e1301 = ((f32(_e397) - (_e1281 * 2f)) * 0.5f);
                                let _e1303 = select(_e1301, 0f, (_e1301 < 0f));
                                let _e1306 = (_e1289 - _e425);
                                let _e1307 = (_e1290 - _e430);
                                let _e1313 = ((sqrt(((_e1306 * _e1306) + (_e1307 * _e1307))) - 11.3f) * -1f);
                                let _e1315 = select(_e1313, 0f, (_e1313 < 0f));
                                let _e1317 = select(_e1315, 1f, (_e1315 > 1f));
                                let _e1323 = select(_e194, 0f, (_e194 < 0f));
                                let _e1326 = (((_e1317 * _e1317) * (3f - (2f * _e1317))) * select(_e1323, 1f, (_e1323 > 1f)));
                                let _e1328 = (1.05f + (0.63f * _e1326));
                                let _e1329 = (_e1306 * _e1326);
                                let _e1331 = (_e1291 - (_e1329 * 0.5f));
                                let _e1332 = (_e1329 * -0.005f);
                                let _e1333 = sin(_e1332);
                                let _e1334 = cos(_e1332);
                                let _e1337 = ((_e1334 * _e1331) - (_e1333 * _e1292));
                                let _e1340 = ((_e1333 * _e1331) + (_e1334 * _e1292));
                                let _e1344 = (_e1328 * 5.4f);
                                let _e1345 = abs(_e1337);
                                let _e1349 = ((0.809017f * _e1345) + (_e1340 * 0.58778524f));
                                if (_e1349 != _e1349) {
                                    phi_38_ = true;
                                } else {
                                    phi_38_ = (0f >= _e1349);
                                }
                                let _e1353 = phi_38_;
                                let _e1354 = select(_e1349, 0f, _e1353);
                                let _e1357 = (_e1345 - (_e1354 * 1.618034f));
                                let _e1358 = (-(_e1340) - (_e1354 * -1.1755705f));
                                let _e1361 = ((-0.809017f * _e1357) + (-0.58778524f * _e1358));
                                if (_e1361 != _e1361) {
                                    phi_39_ = true;
                                } else {
                                    phi_39_ = (0f >= _e1361);
                                }
                                let _e1365 = phi_39_;
                                let _e1366 = select(_e1361, 0f, _e1365);
                                let _e1371 = abs((_e1357 - (_e1366 * -1.618034f)));
                                let _e1372 = ((_e1358 - (_e1366 * -1.1755705f)) - _e1344);
                                let _e1373 = (_e1328 * 2.031386f);
                                let _e1375 = ((_e1328 * 2.7959628f) - _e1344);
                                let _e1382 = (((_e1371 * _e1373) + (_e1372 * _e1375)) / ((_e1373 * _e1373) + (_e1375 * _e1375)));
                                let _e1384 = select(_e1382, 0f, (_e1382 < 0f));
                                let _e1386 = select(_e1384, 1f, (_e1384 > 1f));
                                let _e1392 = (_e1371 - (_e1373 * _e1386));
                                let _e1393 = (_e1372 - (_e1375 * _e1386));
                                let _e1402 = ((sqrt(((_e1392 * _e1392) + (_e1393 * _e1393))) * select(1f, -1f, (((_e1372 * _e1373) - (_e1371 * _e1375)) < 0f))) - (_e1328 * 1.08f));
                                let _e1403 = (((_e1337 / (_e1328 * 21.6f)) + 0.5f) - select(_e1303, 1f, (_e1303 > 1f)));
                                let _e1404 = fwidth(_e1403);
                                let _e1406 = ((_e1403 / _e1404) + 0.5f);
                                let _e1408 = select(_e1406, 0f, (_e1406 < 0f));
                                let _e1410 = select(_e1408, 1f, (_e1408 > 1f));
                                let _e1411 = (1f - _e1410);
                                let _e1414 = (0.33f * _e1410);
                                let _e1418 = (0.5f - _e1402);
                                let _e1420 = select(_e1418, 0f, (_e1418 < 0f));
                                let _e1422 = select(_e1420, 1f, (_e1420 > 1f));
                                if (_e1402 != _e1402) {
                                    phi_40_ = true;
                                } else {
                                    phi_40_ = (0f >= _e1402);
                                }
                                let _e1426 = phi_40_;
                                let _e1429 = exp((select(_e1402, 0f, _e1426) * -0.5f));
                                let _e1430 = (_e1402 * -0.2f);
                                let _e1432 = select(_e1430, 0f, (_e1430 < 0f));
                                let _e1434 = select(_e1432, 1f, (_e1432 > 1f));
                                let _e1439 = (1f - ((_e1434 * _e1434) * (3f - (2f * _e1434))));
                                let _e1441 = ((_e1439 * _e1439) * 0.045f);
                                let _e1452 = ((_e1429 * _e1429) * 0.2f);
                                if (_e1422 != _e1422) {
                                    phi_41_ = true;
                                } else {
                                    phi_41_ = (_e1452 >= _e1422);
                                }
                                let _e1456 = phi_41_;
                                let _e1458 = (select(_e1422, _e1452, _e1456) * _e476);
                                let _e1459 = (1f - _e1458);
                                phi_42_ = vec4<f32>(((_e1277.x * _e1459) + ((((_e1411 + _e1414) + _e1441) * _e1422) * _e476)), ((_e1277.y * _e1459) + (((((0.85f * _e1411) + _e1414) + _e1441) * _e1422) * _e476)), ((_e1277.z * _e1459) + (((((0.2f * _e1411) + _e1414) + _e1441) * _e1422) * _e476)), ((_e1277.w * _e1459) + _e1458));
                            } else {
                                phi_42_ = _e1277;
                            }
                            let _e1474 = phi_42_;
                            phi_43_ = _e1474;
                            phi_44_ = (_e1279 + 1u);
                        } else {
                            phi_43_ = vec4<f32>();
                            phi_44_ = u32();
                        }
                        let _e1477 = phi_43_;
                        let _e1479 = phi_44_;
                        continue;
                        continuing {
                            phi_35_ = _e1477;
                            phi_36_ = _e1479;
                            break if !(_e1280);
                        }
                    }
                    if _e329 {
                        break;
                    }
                    let _e2174 = local_22;
                    phi_45_ = _e2174;
                } else {
                    phi_45_ = _e1274;
                }
                let _e1482 = phi_45_;
                phi_46_ = _e1482;
            } else {
                phi_46_ = _e1274;
            }
            let _e1484 = phi_46_;
            let _e1485 = (_e403 + _e417);
            phi_47_ = _e1484;
            phi_48_ = 0u;
            loop {
                let _e1489 = phi_47_;
                let _e1491 = phi_48_;
                local_18 = _e1489;
                local_19 = _e1489;
                local_20 = _e1489;
                local_21 = _e1489;
                let _e1492 = (_e1491 < select(_e1485, 8u, (8u < _e1485)));
                if _e1492 {
                    if (_e1491 < 8u) {
                    } else {
                        phi_62_ = true;
                        break;
                    }
                    let _e1498 = pill.member[_e166].playlist_images[_e1491];
                    if (_e1498 >= 0i) {
                        let _e1500 = (_e1491 < _e403);
                        if _e1500 {
                            phi_49_ = render_shared_RipplePulse(vec2<f32>(_e391, _e393), _e405, 1f);
                            phi_50_ = (f32(_e1491) + _e399);
                        } else {
                            phi_49_ = render_shared_RipplePulse(vec2<f32>(_e391, _e413), _e418, _e411);
                            phi_50_ = f32((_e1491 - _e403));
                        }
                        let _e1506 = phi_49_;
                        let _e1508 = phi_50_;
                        let _e1509 = select(_e411, _e476, _e1500);
                        let _e1511 = (_e1506.start_time - 1f);
                        if (_e1511 != _e1511) {
                            phi_51_ = true;
                        } else {
                            phi_51_ = (0f >= _e1511);
                        }
                        let _e1515 = phi_51_;
                        let _e1524 = (_e1506.origin.x + (((_e1508 - (select(_e1511, 0f, _e1515) * 0.5f)) * 18f) * _e1506.strength));
                        let _e1527 = (_e1506.origin.y + 2f);
                        if (_e1509 > 0f) {
                            let _e1529 = (_e165.x - _e1524);
                            let _e1530 = (_e165.y - _e1527);
                            let _e1531 = abs(_e1529);
                            let _e1532 = abs(_e1530);
                            if (select(_e1532, _e1531, (_e1531 > _e1532)) < 38.88f) {
                                let _e1536 = (_e1524 - _e425);
                                let _e1537 = (_e1527 - _e430);
                                let _e1541 = sqrt(((_e1536 * _e1536) + (_e1537 * _e1537)));
                                let _e1543 = ((_e1541 - 11.3f) * -1f);
                                let _e1545 = select(_e1543, 0f, (_e1543 < 0f));
                                let _e1547 = select(_e1545, 1f, (_e1545 > 1f));
                                let _e1553 = select(_e194, 0f, (_e194 < 0f));
                                let _e1556 = (((_e1547 * _e1547) * (3f - (2f * _e1547))) * select(_e1553, 1f, (_e1553 > 1f)));
                                let _e1558 = (1.05f + (0.63f * _e1556));
                                let _e1559 = (_e1536 * _e1556);
                                let _e1561 = (_e1529 - (_e1559 * 0.5f));
                                let _e1562 = (_e1559 * -0.005f);
                                let _e1563 = sin(_e1562);
                                let _e1564 = cos(_e1562);
                                let _e1567 = ((_e1564 * _e1561) - (_e1563 * _e1530));
                                let _e1570 = ((_e1563 * _e1561) + (_e1564 * _e1530));
                                let _e1571 = (_e1558 * 21.6f);
                                if _e1500 {
                                    phi_53_ = true;
                                } else {
                                    if _e195 {
                                        phi_52_ = select(true, false, (_e1541 <= 10.8f));
                                    } else {
                                        phi_52_ = true;
                                    }
                                    let _e1579 = phi_52_;
                                    phi_53_ = select(true, false, _e1579);
                                }
                                let _e1582 = phi_53_;
                                let _e1583 = select(0.2f, 0f, _e1582);
                                let _e1586 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1567, _e1570), 0f, (_e1558 * 6.4800005f));
                                if (_e1586 <= 7f) {
                                    let _e1589 = vec3<f32>(((_e1567 / _e1571) + 0.5f), ((_e1570 / _e1571) + 0.5f), f32(_e1498));
                                    let _e1595 = textureSample(images, sampler_, vec2<f32>(_e1589.x, _e1589.y), i32(_e1589.z));
                                    let _e1599 = (1f - _e1583);
                                    let _e1603 = (0.24f * _e1583);
                                    let _e1607 = (0.5f - _e1586);
                                    let _e1609 = select(_e1607, 0f, (_e1607 < 0f));
                                    let _e1611 = select(_e1609, 1f, (_e1609 > 1f));
                                    if (_e1586 != _e1586) {
                                        phi_54_ = true;
                                    } else {
                                        phi_54_ = (0f >= _e1586);
                                    }
                                    let _e1615 = phi_54_;
                                    let _e1618 = exp((select(_e1586, 0f, _e1615) * -0.5f));
                                    let _e1619 = (_e1586 * -0.2f);
                                    let _e1621 = select(_e1619, 0f, (_e1619 < 0f));
                                    let _e1623 = select(_e1621, 1f, (_e1621 > 1f));
                                    let _e1628 = (1f - ((_e1623 * _e1623) * (3f - (2f * _e1623))));
                                    let _e1630 = ((_e1628 * _e1628) * 0.045f);
                                    let _e1641 = ((_e1618 * _e1618) * 0.2f);
                                    if (_e1611 != _e1611) {
                                        phi_55_ = true;
                                    } else {
                                        phi_55_ = (_e1641 >= _e1611);
                                    }
                                    let _e1645 = phi_55_;
                                    let _e1647 = (select(_e1611, _e1641, _e1645) * _e1509);
                                    let _e1648 = (1f - _e1647);
                                    phi_56_ = vec4<f32>(((_e1489.x * _e1648) + (((((_e1595.x * _e1599) + _e1603) + _e1630) * _e1611) * _e1509)), ((_e1489.y * _e1648) + (((((_e1595.y * _e1599) + _e1603) + _e1630) * _e1611) * _e1509)), ((_e1489.z * _e1648) + (((((_e1595.z * _e1599) + _e1603) + _e1630) * _e1611) * _e1509)), ((_e1489.w * _e1648) + _e1647));
                                } else {
                                    phi_56_ = _e1489;
                                }
                                let _e1663 = phi_56_;
                                phi_57_ = _e1663;
                            } else {
                                phi_57_ = _e1489;
                            }
                            let _e1665 = phi_57_;
                            phi_58_ = _e1665;
                        } else {
                            phi_58_ = _e1489;
                        }
                        let _e1667 = phi_58_;
                        phi_59_ = _e1667;
                    } else {
                        phi_59_ = _e1489;
                    }
                    let _e1669 = phi_59_;
                    phi_60_ = _e1669;
                    phi_61_ = (_e1491 + 1u);
                } else {
                    phi_60_ = vec4<f32>();
                    phi_61_ = u32();
                }
                let _e1672 = phi_60_;
                let _e1674 = phi_61_;
                continue;
                continuing {
                    phi_47_ = _e1672;
                    phi_48_ = _e1674;
                    phi_62_ = _e329;
                    break if !(_e1492);
                }
            }
            let _e1677 = phi_62_;
            if _e1677 {
                break;
            }
            let _e1682 = pill.member[_e166].lines[0u];
            if (_e618 < _e1682.min.x) {
                phi_90_ = f32();
                phi_91_ = true;
            } else {
                if (_e618 > _e1682.max.x) {
                    phi_88_ = f32();
                    phi_89_ = true;
                } else {
                    if (_e619 < _e1682.min.y) {
                        phi_86_ = f32();
                        phi_87_ = true;
                    } else {
                        let _e1694 = (_e619 > _e1682.max.y);
                        if _e1694 {
                            phi_85_ = f32();
                        } else {
                            let _e1696 = (1f / _e1682.size);
                            let _e1703 = ((_e618 - _e1682.origin.x) * _e1696);
                            phi_63_ = 0u;
                            phi_64_ = _e1682.count;
                            loop {
                                let _e1708 = phi_63_;
                                let _e1710 = phi_64_;
                                local_10 = _e1708;
                                let _e1711 = (_e1708 < _e1710);
                                if _e1711 {
                                    let _e1714 = (_e1708 + ((_e1710 - _e1708) / 2u));
                                    let _e1719 = placed_glyphs.member[(_e1682.first + _e1714)].x;
                                    let _e1720 = (_e1719 <= _e1703);
                                    if _e1720 {
                                        phi_65_ = (_e1714 + 1u);
                                    } else {
                                        phi_65_ = _e1708;
                                    }
                                    let _e1723 = phi_65_;
                                    phi_66_ = _e1723;
                                    phi_67_ = select(_e1714, _e1710, _e1720);
                                } else {
                                    phi_66_ = u32();
                                    phi_67_ = u32();
                                }
                                let _e1726 = phi_66_;
                                let _e1728 = phi_67_;
                                continue;
                                continuing {
                                    phi_63_ = _e1726;
                                    phi_64_ = _e1728;
                                    break if !(_e1711);
                                }
                            }
                            let _e1730 = (3.5f / _e1682.size);
                            let _e1732 = local_10;
                            let _e1733 = (_e1732 + 1u);
                            phi_68_ = select(_e1733, _e1682.count, (_e1682.count < _e1733));
                            phi_69_ = -1000000f;
                            loop {
                                let _e1737 = phi_68_;
                                let _e1739 = phi_69_;
                                local_13 = _e1739;
                                if (_e1737 > 0u) {
                                    let _e1741 = (_e1737 - 1u);
                                    let _e1742 = (_e1682.first + _e1741);
                                    let _e1746 = placed_glyphs.member[_e1742].x;
                                    let _e1750 = placed_glyphs.member[_e1742].glyph;
                                    let _e1755 = glyphs.member[_e1750].min[0u];
                                    let _e1760 = glyphs.member[_e1750].min[1u];
                                    let _e1765 = glyphs.member[_e1750].max[0u];
                                    let _e1770 = glyphs.member[_e1750].max[1u];
                                    let _e1774 = glyphs.member[_e1750].start;
                                    let _e1778 = glyphs.member[_e1750].count;
                                    let _e1779 = (_e1703 - _e1746);
                                    let _e1780 = -(((_e619 - _e1682.origin.y) * _e1696));
                                    let _e1781 = (_e1765 + _e1730);
                                    let _e1782 = (_e1779 > _e1781);
                                    if _e1782 {
                                        phi_81_ = f32();
                                    } else {
                                        if (_e1779 >= (_e1755 - _e1730)) {
                                            if (_e1780 >= (_e1760 - _e1730)) {
                                                if (_e1779 <= _e1781) {
                                                    if (_e1780 <= (_e1770 + _e1730)) {
                                                        phi_70_ = 340282350000000000000000000000000000000f;
                                                        phi_71_ = 0u;
                                                        phi_72_ = 0i;
                                                        loop {
                                                            let _e1792 = phi_70_;
                                                            let _e1794 = phi_71_;
                                                            let _e1796 = phi_72_;
                                                            local_11 = _e1792;
                                                            local_12 = _e1796;
                                                            let _e1797 = (_e1794 < _e1778);
                                                            if _e1797 {
                                                                let _e1801 = edges.member[(_e1774 + _e1794)];
                                                                let _e1803 = cantus_render_text_edge_distance(_e1801, _e1682.weight, vec2<f32>(_e1779, _e1780), _e1792);
                                                                phi_73_ = _e1803.member;
                                                                phi_74_ = (_e1794 + 1u);
                                                                phi_75_ = (_e1796 + _e1803.member_1);
                                                            } else {
                                                                phi_73_ = f32();
                                                                phi_74_ = u32();
                                                                phi_75_ = i32();
                                                            }
                                                            let _e1809 = phi_73_;
                                                            let _e1811 = phi_74_;
                                                            let _e1813 = phi_75_;
                                                            continue;
                                                            continuing {
                                                                phi_70_ = _e1809;
                                                                phi_71_ = _e1811;
                                                                phi_72_ = _e1813;
                                                                break if !(_e1797);
                                                            }
                                                        }
                                                        let _e1816 = local_11;
                                                        let _e1820 = local_12;
                                                        let _e1823 = ((sqrt(_e1816) * _e1682.size) * select(1f, -1f, (_e1820 == 0i)));
                                                        if (_e1739 != _e1739) {
                                                            phi_76_ = true;
                                                        } else {
                                                            phi_76_ = (_e1823 >= _e1739);
                                                        }
                                                        let _e1827 = phi_76_;
                                                        phi_77_ = select(_e1739, _e1823, _e1827);
                                                    } else {
                                                        phi_77_ = _e1739;
                                                    }
                                                    let _e1830 = phi_77_;
                                                    phi_78_ = _e1830;
                                                } else {
                                                    phi_78_ = _e1739;
                                                }
                                                let _e1832 = phi_78_;
                                                phi_79_ = _e1832;
                                            } else {
                                                phi_79_ = _e1739;
                                            }
                                            let _e1834 = phi_79_;
                                            phi_80_ = _e1834;
                                        } else {
                                            phi_80_ = _e1739;
                                        }
                                        let _e1836 = phi_80_;
                                        phi_81_ = _e1836;
                                    }
                                    let _e1838 = phi_81_;
                                    phi_82_ = _e1741;
                                    phi_83_ = _e1838;
                                    phi_84_ = select(true, false, _e1782);
                                } else {
                                    phi_82_ = u32();
                                    phi_83_ = f32();
                                    phi_84_ = false;
                                }
                                let _e1841 = phi_82_;
                                let _e1843 = phi_83_;
                                let _e1845 = phi_84_;
                                continue;
                                continuing {
                                    phi_68_ = _e1841;
                                    phi_69_ = _e1843;
                                    break if !(_e1845);
                                }
                            }
                            let _e1848 = local_13;
                            let _e1850 = ((_e1848 * 1.25f) + 0.5f);
                            let _e1852 = select(_e1850, 0f, (_e1850 < 0f));
                            let _e1854 = select(_e1852, 1f, (_e1852 > 1f));
                            phi_85_ = ((_e1854 * _e1854) * (3f - (2f * _e1854)));
                        }
                        let _e1860 = phi_85_;
                        phi_86_ = _e1860;
                        phi_87_ = _e1694;
                    }
                    let _e1862 = phi_86_;
                    let _e1864 = phi_87_;
                    phi_88_ = _e1862;
                    phi_89_ = _e1864;
                }
                let _e1866 = phi_88_;
                let _e1868 = phi_89_;
                phi_90_ = _e1866;
                phi_91_ = _e1868;
            }
            let _e1870 = phi_90_;
            let _e1872 = phi_91_;
            let _e1873 = select(_e1870, 0f, _e1872);
            let _e1878 = pill.member[_e166].lines[1u];
            if (_e618 < _e1878.min.x) {
                phi_119_ = f32();
                phi_120_ = true;
            } else {
                if (_e618 > _e1878.max.x) {
                    phi_117_ = f32();
                    phi_118_ = true;
                } else {
                    if (_e619 < _e1878.min.y) {
                        phi_115_ = f32();
                        phi_116_ = true;
                    } else {
                        let _e1890 = (_e619 > _e1878.max.y);
                        if _e1890 {
                            phi_114_ = f32();
                        } else {
                            let _e1892 = (1f / _e1878.size);
                            let _e1899 = ((_e618 - _e1878.origin.x) * _e1892);
                            phi_92_ = 0u;
                            phi_93_ = _e1878.count;
                            loop {
                                let _e1904 = phi_92_;
                                let _e1906 = phi_93_;
                                local_14 = _e1904;
                                let _e1907 = (_e1904 < _e1906);
                                if _e1907 {
                                    let _e1910 = (_e1904 + ((_e1906 - _e1904) / 2u));
                                    let _e1915 = placed_glyphs.member[(_e1878.first + _e1910)].x;
                                    let _e1916 = (_e1915 <= _e1899);
                                    if _e1916 {
                                        phi_94_ = (_e1910 + 1u);
                                    } else {
                                        phi_94_ = _e1904;
                                    }
                                    let _e1919 = phi_94_;
                                    phi_95_ = _e1919;
                                    phi_96_ = select(_e1910, _e1906, _e1916);
                                } else {
                                    phi_95_ = u32();
                                    phi_96_ = u32();
                                }
                                let _e1922 = phi_95_;
                                let _e1924 = phi_96_;
                                continue;
                                continuing {
                                    phi_92_ = _e1922;
                                    phi_93_ = _e1924;
                                    break if !(_e1907);
                                }
                            }
                            let _e1926 = (3.5f / _e1878.size);
                            let _e1928 = local_14;
                            let _e1929 = (_e1928 + 1u);
                            phi_97_ = select(_e1929, _e1878.count, (_e1878.count < _e1929));
                            phi_98_ = -1000000f;
                            loop {
                                let _e1933 = phi_97_;
                                let _e1935 = phi_98_;
                                local_17 = _e1935;
                                if (_e1933 > 0u) {
                                    let _e1937 = (_e1933 - 1u);
                                    let _e1938 = (_e1878.first + _e1937);
                                    let _e1942 = placed_glyphs.member[_e1938].x;
                                    let _e1946 = placed_glyphs.member[_e1938].glyph;
                                    let _e1951 = glyphs.member[_e1946].min[0u];
                                    let _e1956 = glyphs.member[_e1946].min[1u];
                                    let _e1961 = glyphs.member[_e1946].max[0u];
                                    let _e1966 = glyphs.member[_e1946].max[1u];
                                    let _e1970 = glyphs.member[_e1946].start;
                                    let _e1974 = glyphs.member[_e1946].count;
                                    let _e1975 = (_e1899 - _e1942);
                                    let _e1976 = -(((_e619 - _e1878.origin.y) * _e1892));
                                    let _e1977 = (_e1961 + _e1926);
                                    let _e1978 = (_e1975 > _e1977);
                                    if _e1978 {
                                        phi_110_ = f32();
                                    } else {
                                        if (_e1975 >= (_e1951 - _e1926)) {
                                            if (_e1976 >= (_e1956 - _e1926)) {
                                                if (_e1975 <= _e1977) {
                                                    if (_e1976 <= (_e1966 + _e1926)) {
                                                        phi_99_ = 340282350000000000000000000000000000000f;
                                                        phi_100_ = 0u;
                                                        phi_101_ = 0i;
                                                        loop {
                                                            let _e1988 = phi_99_;
                                                            let _e1990 = phi_100_;
                                                            let _e1992 = phi_101_;
                                                            local_15 = _e1988;
                                                            local_16 = _e1992;
                                                            let _e1993 = (_e1990 < _e1974);
                                                            if _e1993 {
                                                                let _e1997 = edges.member[(_e1970 + _e1990)];
                                                                let _e1999 = cantus_render_text_edge_distance(_e1997, _e1878.weight, vec2<f32>(_e1975, _e1976), _e1988);
                                                                phi_102_ = _e1999.member;
                                                                phi_103_ = (_e1990 + 1u);
                                                                phi_104_ = (_e1992 + _e1999.member_1);
                                                            } else {
                                                                phi_102_ = f32();
                                                                phi_103_ = u32();
                                                                phi_104_ = i32();
                                                            }
                                                            let _e2005 = phi_102_;
                                                            let _e2007 = phi_103_;
                                                            let _e2009 = phi_104_;
                                                            continue;
                                                            continuing {
                                                                phi_99_ = _e2005;
                                                                phi_100_ = _e2007;
                                                                phi_101_ = _e2009;
                                                                break if !(_e1993);
                                                            }
                                                        }
                                                        let _e2012 = local_15;
                                                        let _e2016 = local_16;
                                                        let _e2019 = ((sqrt(_e2012) * _e1878.size) * select(1f, -1f, (_e2016 == 0i)));
                                                        if (_e1935 != _e1935) {
                                                            phi_105_ = true;
                                                        } else {
                                                            phi_105_ = (_e2019 >= _e1935);
                                                        }
                                                        let _e2023 = phi_105_;
                                                        phi_106_ = select(_e1935, _e2019, _e2023);
                                                    } else {
                                                        phi_106_ = _e1935;
                                                    }
                                                    let _e2026 = phi_106_;
                                                    phi_107_ = _e2026;
                                                } else {
                                                    phi_107_ = _e1935;
                                                }
                                                let _e2028 = phi_107_;
                                                phi_108_ = _e2028;
                                            } else {
                                                phi_108_ = _e1935;
                                            }
                                            let _e2030 = phi_108_;
                                            phi_109_ = _e2030;
                                        } else {
                                            phi_109_ = _e1935;
                                        }
                                        let _e2032 = phi_109_;
                                        phi_110_ = _e2032;
                                    }
                                    let _e2034 = phi_110_;
                                    phi_111_ = _e1937;
                                    phi_112_ = _e2034;
                                    phi_113_ = select(true, false, _e1978);
                                } else {
                                    phi_111_ = u32();
                                    phi_112_ = f32();
                                    phi_113_ = false;
                                }
                                let _e2037 = phi_111_;
                                let _e2039 = phi_112_;
                                let _e2041 = phi_113_;
                                continue;
                                continuing {
                                    phi_97_ = _e2037;
                                    phi_98_ = _e2039;
                                    break if !(_e2041);
                                }
                            }
                            let _e2044 = local_17;
                            let _e2046 = ((_e2044 * 1.25f) + 0.5f);
                            let _e2048 = select(_e2046, 0f, (_e2046 < 0f));
                            let _e2050 = select(_e2048, 1f, (_e2048 > 1f));
                            phi_114_ = ((_e2050 * _e2050) * (3f - (2f * _e2050)));
                        }
                        let _e2056 = phi_114_;
                        phi_115_ = _e2056;
                        phi_116_ = _e1890;
                    }
                    let _e2058 = phi_115_;
                    let _e2060 = phi_116_;
                    phi_117_ = _e2058;
                    phi_118_ = _e2060;
                }
                let _e2062 = phi_117_;
                let _e2064 = phi_118_;
                phi_119_ = _e2062;
                phi_120_ = _e2064;
            }
            let _e2066 = phi_119_;
            let _e2068 = phi_120_;
            let _e2069 = select(_e2066, 0f, _e2068);
            if (_e1873 != _e1873) {
                phi_121_ = true;
            } else {
                phi_121_ = (_e2069 >= _e1873);
            }
            let _e2073 = phi_121_;
            let _e2078 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e618 - _e1122), (_e619 - _e184)), 0f, _e184);
            let _e2080 = ((_e2078 - 2f) * 0.0625f);
            let _e2082 = select(_e2080, 0f, (_e2080 < 0f));
            let _e2084 = select(_e2082, 1f, (_e2082 > 1f));
            let _e2090 = ((select(_e1873, _e2069, _e2073) * ((_e2084 * _e2084) * (3f - (2f * _e2084)))) * _e578);
            let _e2091 = (1f - _e2090);
            let _e2093 = local_18;
            let _e2097 = local_19;
            let _e2101 = local_20;
            let _e2105 = local_21;
            let _e2108 = (0.94f * _e2090);
            let _e2116 = (((_e2105.w * _e2091) + _e2090) * _e595);
            if (_e2116 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e2093.x * _e2091) + _e2108) * _e595), (((_e2097.y * _e2091) + _e2108) * _e595), (((_e2101.z * _e2091) + _e2108) * _e595), _e2116);
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
    var local_25: i32;
    var phi_13_: bool;
    var phi_14_: f32;
    var phi_15_: f32;
    var phi_16_: f32;
    var phi_17_: f32;
    var phi_18_: f32;
    var phi_19_: u32;
    var phi_20_: f32;
    var phi_21_: bool;
    var local_26: f32;
    var phi_22_: f32;
    var phi_23_: f32;
    var phi_24_: bool;
    var phi_25_: f32;
    var phi_26_: bool;
    var phi_27_: f32;
    var phi_28_: bool;

    let _e31 = pixel_3;
    let _e32 = _isthmus_instance_index_10;
    let _e37 = (_e31.x * 0.03125f);
    let _e39 = select(_e37, 0f, (_e37 < 0f));
    let _e41 = select(_e39, 1f, (_e39 > 1f));
    let _e50 = frame.member[0u].screen_size[0u];
    let _e54 = ((_e31.x - _e50) / ((_e50 - 32f) - _e50));
    let _e56 = select(_e54, 0f, (_e54 < 0f));
    let _e58 = select(_e56, 1f, (_e56 > 1f));
    let _e64 = line.member[_e32];
    if (_e31.x < _e64.min.x) {
        phi_27_ = f32();
        phi_28_ = true;
    } else {
        if (_e31.x > _e64.max.x) {
            phi_25_ = f32();
            phi_26_ = true;
        } else {
            if (_e31.y < _e64.min.y) {
                phi_23_ = f32();
                phi_24_ = true;
            } else {
                let _e76 = (_e31.y > _e64.max.y);
                if _e76 {
                    phi_22_ = f32();
                } else {
                    let _e78 = (1f / _e64.size);
                    let _e85 = ((_e31.x - _e64.origin.x) * _e78);
                    phi_0_ = 0u;
                    phi_1_ = _e64.count;
                    loop {
                        let _e90 = phi_0_;
                        let _e92 = phi_1_;
                        local_23 = _e90;
                        let _e93 = (_e90 < _e92);
                        if _e93 {
                            let _e96 = (_e90 + ((_e92 - _e90) / 2u));
                            let _e101 = placed_glyphs_1.member[(_e64.first + _e96)].x;
                            let _e102 = (_e101 <= _e85);
                            if _e102 {
                                phi_2_ = (_e96 + 1u);
                            } else {
                                phi_2_ = _e90;
                            }
                            let _e105 = phi_2_;
                            phi_3_ = _e105;
                            phi_4_ = select(_e96, _e92, _e102);
                        } else {
                            phi_3_ = u32();
                            phi_4_ = u32();
                        }
                        let _e108 = phi_3_;
                        let _e110 = phi_4_;
                        continue;
                        continuing {
                            phi_0_ = _e108;
                            phi_1_ = _e110;
                            break if !(_e93);
                        }
                    }
                    let _e112 = (3.5f / _e64.size);
                    let _e114 = local_23;
                    let _e115 = (_e114 + 1u);
                    phi_5_ = select(_e115, _e64.count, (_e64.count < _e115));
                    phi_6_ = -1000000f;
                    loop {
                        let _e119 = phi_5_;
                        let _e121 = phi_6_;
                        local_26 = _e121;
                        if (_e119 > 0u) {
                            let _e123 = (_e119 - 1u);
                            let _e124 = (_e64.first + _e123);
                            let _e128 = placed_glyphs_1.member[_e124].x;
                            let _e132 = placed_glyphs_1.member[_e124].glyph;
                            let _e137 = glyphs_1.member[_e132].min[0u];
                            let _e142 = glyphs_1.member[_e132].min[1u];
                            let _e147 = glyphs_1.member[_e132].max[0u];
                            let _e152 = glyphs_1.member[_e132].max[1u];
                            let _e156 = glyphs_1.member[_e132].start;
                            let _e160 = glyphs_1.member[_e132].count;
                            let _e161 = (_e85 - _e128);
                            let _e162 = -(((_e31.y - _e64.origin.y) * _e78));
                            let _e163 = (_e147 + _e112);
                            let _e164 = (_e161 > _e163);
                            if _e164 {
                                phi_18_ = f32();
                            } else {
                                if (_e161 >= (_e137 - _e112)) {
                                    if (_e162 >= (_e142 - _e112)) {
                                        if (_e161 <= _e163) {
                                            if (_e162 <= (_e152 + _e112)) {
                                                phi_7_ = 340282350000000000000000000000000000000f;
                                                phi_8_ = 0u;
                                                phi_9_ = 0i;
                                                loop {
                                                    let _e174 = phi_7_;
                                                    let _e176 = phi_8_;
                                                    let _e178 = phi_9_;
                                                    local_24 = _e174;
                                                    local_25 = _e178;
                                                    let _e179 = (_e176 < _e160);
                                                    if _e179 {
                                                        let _e183 = edges_1.member[(_e156 + _e176)];
                                                        let _e185 = cantus_render_text_edge_distance(_e183, _e64.weight, vec2<f32>(_e161, _e162), _e174);
                                                        phi_10_ = _e185.member;
                                                        phi_11_ = (_e176 + 1u);
                                                        phi_12_ = (_e178 + _e185.member_1);
                                                    } else {
                                                        phi_10_ = f32();
                                                        phi_11_ = u32();
                                                        phi_12_ = i32();
                                                    }
                                                    let _e191 = phi_10_;
                                                    let _e193 = phi_11_;
                                                    let _e195 = phi_12_;
                                                    continue;
                                                    continuing {
                                                        phi_7_ = _e191;
                                                        phi_8_ = _e193;
                                                        phi_9_ = _e195;
                                                        break if !(_e179);
                                                    }
                                                }
                                                let _e198 = local_24;
                                                let _e202 = local_25;
                                                let _e205 = ((sqrt(_e198) * _e64.size) * select(1f, -1f, (_e202 == 0i)));
                                                if (_e121 != _e121) {
                                                    phi_13_ = true;
                                                } else {
                                                    phi_13_ = (_e205 >= _e121);
                                                }
                                                let _e209 = phi_13_;
                                                phi_14_ = select(_e121, _e205, _e209);
                                            } else {
                                                phi_14_ = _e121;
                                            }
                                            let _e212 = phi_14_;
                                            phi_15_ = _e212;
                                        } else {
                                            phi_15_ = _e121;
                                        }
                                        let _e214 = phi_15_;
                                        phi_16_ = _e214;
                                    } else {
                                        phi_16_ = _e121;
                                    }
                                    let _e216 = phi_16_;
                                    phi_17_ = _e216;
                                } else {
                                    phi_17_ = _e121;
                                }
                                let _e218 = phi_17_;
                                phi_18_ = _e218;
                            }
                            let _e220 = phi_18_;
                            phi_19_ = _e123;
                            phi_20_ = _e220;
                            phi_21_ = select(true, false, _e164);
                        } else {
                            phi_19_ = u32();
                            phi_20_ = f32();
                            phi_21_ = false;
                        }
                        let _e223 = phi_19_;
                        let _e225 = phi_20_;
                        let _e227 = phi_21_;
                        continue;
                        continuing {
                            phi_5_ = _e223;
                            phi_6_ = _e225;
                            break if !(_e227);
                        }
                    }
                    let _e230 = local_26;
                    let _e232 = ((_e230 * 1.25f) + 0.5f);
                    let _e234 = select(_e232, 0f, (_e232 < 0f));
                    let _e236 = select(_e234, 1f, (_e234 > 1f));
                    phi_22_ = ((_e236 * _e236) * (3f - (2f * _e236)));
                }
                let _e242 = phi_22_;
                phi_23_ = _e242;
                phi_24_ = _e76;
            }
            let _e244 = phi_23_;
            let _e246 = phi_24_;
            phi_25_ = _e244;
            phi_26_ = _e246;
        }
        let _e248 = phi_25_;
        let _e250 = phi_26_;
        phi_27_ = _e248;
        phi_28_ = _e250;
    }
    let _e252 = phi_27_;
    let _e254 = phi_28_;
    let _e256 = (select(_e252, 0f, _e254) * (((_e41 * _e41) * (3f - (2f * _e41))) * ((_e58 * _e58) * (3f - (2f * _e58)))));
    let _e260 = frame.member[0u].playhead_x;
    let _e261 = (_e260 + 4f);
    let _e265 = ((_e31.x - _e261) / ((_e260 - 4f) - _e261));
    let _e267 = select(_e265, 0f, (_e265 < 0f));
    let _e269 = select(_e267, 1f, (_e267 > 1f));
    let _e273 = ((_e269 * _e269) * (3f - (2f * _e269)));
    let _e277 = line.member[_e32].color;
    let _e278 = unpack4x8unorm(_e277);
    let _e285 = (1f - _e273);
    out_color = vec4<f32>((((_e278.x * _e285) + ((_e278.x * 0.42f) * _e273)) * _e256), (((_e278.y * _e285) + ((_e278.y * 0.42f) * _e273)) * _e256), (((_e278.z * _e285) + ((_e278.z * 0.42f) * _e273)) * _e256), _e256);
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
    var local_38: i32;
    var phi_98_: bool;
    var phi_99_: f32;
    var phi_100_: f32;
    var phi_101_: f32;
    var phi_102_: f32;
    var phi_103_: f32;
    var phi_104_: u32;
    var phi_105_: f32;
    var phi_106_: bool;
    var local_39: f32;
    var phi_107_: f32;
    var phi_108_: f32;
    var phi_109_: bool;
    var phi_110_: f32;
    var phi_111_: bool;
    var phi_112_: f32;
    var phi_113_: bool;
    var phi_114_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e230 = pixel_3;
            let _e231 = _isthmus_instance_index_10;
            let _e237 = pill_1.member[_e231].battery_level;
            let _e238 = (_e237 >= -1f);
            if _e238 {
                phi_0_ = (_e237 <= 1f);
            } else {
                phi_0_ = false;
            }
            let _e241 = phi_0_;
            let _e243 = (select(0f, 40f, _e241) + 296f);
            let _e248 = frame.member[0u].screen_size[0u];
            let _e250 = ((_e248 - _e243) - 8f);
            let _e254 = frame.member[0u].panel_height;
            let _e255 = (_e230.x - _e250);
            let _e256 = (_e230.y - 6f);
            let _e257 = (_e243 * 0.5f);
            let _e258 = (_e254 * 0.5f);
            let _e262 = ((_e243 - _e254) * 0.5f);
            let _e264 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e255 - _e257), (_e256 - _e258)), _e262, _e258);
            let _e268 = frame.member[0u].mouse_pressure;
            let _e269 = (_e268 > 0f);
            if _e269 {
                let _e274 = frame.member[0u].mouse_pos[0u];
                let _e279 = frame.member[0u].mouse_pos[1u];
                let _e285 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e274 - _e250) - _e257), ((_e279 - 6f) - _e258)), _e262, _e258);
                phi_1_ = _e285;
            } else {
                phi_1_ = 1f;
            }
            let _e287 = phi_1_;
            phi_2_ = vec2<f32>(0f, 0f);
            phi_3_ = 0f;
            phi_4_ = 0u;
            loop {
                let _e289 = phi_2_;
                let _e291 = phi_3_;
                let _e293 = phi_4_;
                local_28 = _e289;
                local_29 = _e289;
                local_30 = _e289;
                local_31 = _e289;
                local_34 = _e291;
                local_35 = _e291;
                let _e294 = (_e293 < 4u);
                if _e294 {
                    if _e294 {
                    } else {
                        phi_14_ = true;
                        break;
                    }
                    let _e301 = frame.member[0u].ripples[_e293].origin[0u];
                    let _e308 = frame.member[0u].ripples[_e293].origin[1u];
                    let _e314 = frame.member[0u].ripples[_e293].start_time;
                    let _e320 = frame.member[0u].ripples[_e293].strength;
                    let _e324 = frame.member[0u].time;
                    let _e326 = ((_e324 - _e314) * 1.2f);
                    let _e328 = select(_e326, 0f, (_e326 < 0f));
                    let _e330 = select(_e328, 1f, (_e328 > 1f));
                    if (_e320 > 0f) {
                        if (_e330 < 1f) {
                            let _e334 = (_e230 - vec2<f32>(_e301, _e308));
                            let _e340 = sqrt(((_e334.x * _e334.x) + (_e334.y * _e334.y)));
                            if (_e340 > 0.001f) {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e334.x / _e340), (_e334.y / _e340)), _e340);
                            } else {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e340);
                            }
                            let _e348 = phi_5_;
                            let _e358 = ((abs((_e348.unnamed_1 - (_e330 * 600f))) - 80f) * -0.0125f);
                            let _e360 = select(_e358, 0f, (_e358 < 0f));
                            let _e362 = select(_e360, 1f, (_e360 > 1f));
                            let _e368 = (1f - _e330);
                            let _e369 = ((((_e362 * _e362) * (3f - (2f * _e362))) * _e320) * _e368);
                            let _e382 = (_e291 + (_e369 * 0.5f));
                            if (_e382 != _e382) {
                                phi_6_ = true;
                            } else {
                                phi_6_ = (1f <= _e382);
                            }
                            let _e386 = phi_6_;
                            phi_7_ = vec2<f32>((_e289.x + (((_e348.unnamed.x * _e369) * _e368) * 0.5f)), (_e289.y + (((_e348.unnamed.y * _e369) * _e368) * 0.5f)));
                            phi_8_ = select(_e382, 1f, _e386);
                        } else {
                            phi_7_ = _e289;
                            phi_8_ = _e291;
                        }
                        let _e389 = phi_7_;
                        let _e391 = phi_8_;
                        phi_9_ = _e389;
                        phi_10_ = _e391;
                    } else {
                        phi_9_ = _e289;
                        phi_10_ = _e291;
                    }
                    let _e393 = phi_9_;
                    let _e395 = phi_10_;
                    phi_11_ = _e393;
                    phi_12_ = _e395;
                    phi_13_ = (_e293 + 1u);
                } else {
                    phi_11_ = vec2<f32>();
                    phi_12_ = f32();
                    phi_13_ = u32();
                }
                let _e398 = phi_11_;
                let _e400 = phi_12_;
                let _e402 = phi_13_;
                continue;
                continuing {
                    phi_2_ = _e398;
                    phi_3_ = _e400;
                    phi_4_ = _e402;
                    phi_14_ = false;
                    break if !(_e294);
                }
            }
            let _e405 = phi_14_;
            if _e405 {
                break;
            }
            if _e269 {
                let _e410 = frame.member[0u].mouse_pos[0u];
                let _e415 = frame.member[0u].mouse_pos[1u];
                let _e416 = (_e230.x - _e410);
                let _e417 = (_e230.y - _e415);
                let _e423 = ((sqrt(((_e416 * _e416) + (_e417 * _e417))) - 150f) * -0.006666667f);
                let _e425 = select(_e423, 0f, (_e423 < 0f));
                let _e427 = select(_e425, 1f, (_e425 > 1f));
                phi_15_ = ((((_e427 * _e427) * (3f - (2f * _e427))) * _e268) * 8f);
            } else {
                phi_15_ = 0f;
            }
            let _e435 = phi_15_;
            let _e437 = local_28;
            let _e440 = global[0u];
            if (_e437.x == _e440) {
                let _e443 = local_29;
                let _e446 = global[1u];
                phi_16_ = (_e443.y == _e446);
            } else {
                phi_16_ = false;
            }
            let _e449 = phi_16_;
            if _e449 {
                phi_17_ = 0f;
            } else {
                let _e451 = local_30;
                phi_17_ = (sqrt(((_e437.x * _e437.x) + (_e451.y * _e451.y))) * 22f);
            }
            let _e459 = phi_17_;
            let _e461 = local_31;
            let _e464 = ((_e287 - 0.5f) * -1f);
            let _e466 = select(_e464, 0f, (_e464 < 0f));
            let _e468 = select(_e466, 1f, (_e466 > 1f));
            let _e476 = (_e264 - (((_e435 * ((_e468 * _e468) * (3f - (2f * _e468)))) + _e459) * 0.5f));
            let _e477 = fwidth(_e476);
            if (_e477 != _e477) {
                phi_18_ = true;
            } else {
                phi_18_ = (0.55f >= _e477);
            }
            let _e481 = phi_18_;
            let _e482 = select(_e477, 0.55f, _e481);
            let _e486 = ((_e476 - _e482) / (-(_e482) - _e482));
            let _e488 = select(_e486, 0f, (_e486 < 0f));
            let _e490 = select(_e488, 1f, (_e488 > 1f));
            let _e494 = ((_e490 * _e490) * (3f - (2f * _e490)));
            let _e495 = (_e476 != _e476);
            if _e495 {
                phi_19_ = true;
            } else {
                phi_19_ = (0f >= _e476);
            }
            let _e498 = phi_19_;
            let _e502 = (exp((select(_e476, 0f, _e498) * -0.3f)) * 0.16f);
            if (_e494 != _e494) {
                phi_20_ = true;
            } else {
                phi_20_ = (_e502 >= _e494);
            }
            let _e506 = phi_20_;
            let _e507 = select(_e494, _e502, _e506);
            if (_e507 <= 0.0009765625f) {
                discard;
            }
            let _e509 = (_e255 / _e243);
            let _e510 = (_e256 / _e254);
            if _e495 {
                phi_21_ = true;
            } else {
                phi_21_ = (0f <= _e476);
            }
            let _e515 = phi_21_;
            let _e518 = (1f + (select(_e476, 0f, _e515) * 0.008333334f));
            let _e520 = select(_e518, 0f, (_e518 < 0f));
            let _e522 = select(_e520, 0.6f, (_e520 > 0.6f));
            let _e532 = ((_e510 - (((_e510 - 0.5f) * _e522) * 0.08f)) - (_e461.y * 0.04f));
            let _e536 = pill_1.member[_e231].sun_height;
            let _e540 = pill_1.member[_e231].conditions;
            let _e544 = frame.member[0u].time;
            let _e552 = ((_e532 - 1f) * -1f);
            let _e554 = select(_e552, 0f, (_e552 < 0f));
            let _e556 = select(_e554, 1f, (_e554 > 1f));
            let _e560 = ((_e556 * _e556) * (3f - (2f * _e556)));
            let _e562 = ((_e536 - -0.04f) * 4.1666665f);
            let _e564 = select(_e562, 0f, (_e562 < 0f));
            let _e566 = select(_e564, 1f, (_e564 > 1f));
            let _e570 = ((_e566 * _e566) * (3f - (2f * _e566)));
            let _e572 = ((_e536 - -0.2f) * 4.5454545f);
            let _e574 = select(_e572, 0f, (_e572 < 0f));
            let _e576 = select(_e574, 1f, (_e574 > 1f));
            let _e581 = (1f - _e570);
            let _e582 = (((_e576 * _e576) * (3f - (2f * _e576))) * _e581);
            let _e583 = (1f - _e560);
            let _e595 = (0.65f * _e583);
            let _e619 = (1f - _e582);
            let _e633 = (((_e540.cloud * 0.34f) + (_e540.rain * 0.16f)) + (_e540.hail * 0.08f));
            let _e634 = (1f - _e633);
            let _e645 = (1f - (_e540.snow * 0.16f));
            let _e649 = (_e540.snow * 0.1312f);
            let _e654 = (1f - (_e540.fog * 0.62f));
            let _e667 = ((sin((_e544 * 2.7f)) - 0.92f) * 12.500003f);
            let _e669 = select(_e667, 0f, (_e667 < 0f));
            let _e671 = select(_e669, 1f, (_e669 > 1f));
            let _e676 = (((_e671 * _e671) * (3f - (2f * _e671))) * _e540.lightning);
            let _e678 = (1f - (_e676 * 0.45f));
            let _e689 = ((_e532 - 0.12f) * -8.333334f);
            let _e691 = select(_e689, 0f, (_e689 < 0f));
            let _e693 = select(_e691, 1f, (_e691 > 1f));
            let _e700 = ((_e476 - 5f) * -0.125f);
            let _e702 = select(_e700, 0f, (_e700 < 0f));
            let _e704 = select(_e702, 1f, (_e702 > 1f));
            let _e710 = ((((_e693 * _e693) * (3f - (2f * _e693))) * 0.12f) + (((_e704 * _e704) * (3f - (2f * _e704))) * 0.08f));
            let _e714 = (((_e509 - (((_e509 - 0.5f) * _e522) * 0.08f)) - (_e437.x * 0.04f)) * _e243);
            let _e715 = (_e532 * _e254);
            if (_e714 < 96f) {
                phi_29_ = 0u;
            } else {
                if (_e714 < 184f) {
                    phi_28_ = 1u;
                } else {
                    if _e238 {
                        phi_22_ = (_e237 <= 1f);
                    } else {
                        phi_22_ = false;
                    }
                    let _e720 = phi_22_;
                    if _e720 {
                        phi_23_ = select(true, false, (_e714 < 224f));
                    } else {
                        phi_23_ = true;
                    }
                    let _e724 = phi_23_;
                    if _e724 {
                        if _e238 {
                            phi_24_ = (_e237 <= 1f);
                        } else {
                            phi_24_ = false;
                        }
                        let _e727 = phi_24_;
                        if (_e714 < (select(0f, 40f, _e727) + 224f)) {
                            phi_26_ = 3u;
                        } else {
                            if _e238 {
                                phi_25_ = (_e237 <= 1f);
                            } else {
                                phi_25_ = false;
                            }
                            let _e733 = phi_25_;
                            phi_26_ = select(5u, 4u, (_e714 < (select(0f, 40f, _e733) + 256f)));
                        }
                        let _e739 = phi_26_;
                        phi_27_ = _e739;
                    } else {
                        phi_27_ = 2u;
                    }
                    let _e741 = phi_27_;
                    phi_28_ = _e741;
                }
                let _e743 = phi_28_;
                phi_29_ = _e743;
            }
            let _e745 = phi_29_;
            if _e238 {
                phi_30_ = (_e237 <= 1f);
            } else {
                phi_30_ = false;
            }
            let _e748 = phi_30_;
            let _e749 = select(0f, 40f, _e748);
            switch bitcast<i32>(_e745) {
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
                    phi_31_ = (188f + _e749);
                    break;
                }
                case 4: {
                    phi_31_ = (228f + _e749);
                    break;
                }
                case 5: {
                    phi_31_ = (260f + _e749);
                    break;
                }
                default: {
                    phi_31_ = f32();
                    break;
                }
            }
            let _e755 = phi_31_;
            switch bitcast<i32>(_e745) {
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
            let _e758 = phi_32_;
            let _e760 = phi_33_;
            let _e762 = phi_34_;
            let _e763 = select(_e760, false, _e758);
            let _e770 = (_e714 - (_e755 + (select(select(80f, 32f, _e763), 24f, select(select(_e762, false, _e758), false, _e763)) * 0.5f)));
            let _e771 = (_e715 - _e258);
            switch bitcast<i32>(_e745) {
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
            let _e774 = phi_35_;
            let _e776 = phi_36_;
            if _e776 {
                let _e777 = (_e714 - 52f);
                let _e782 = pill_1.member[_e231].cpu.temperature;
                if (_e782 <= 62f) {
                    phi_45_ = vec2<f32>(0f, 0f);
                } else {
                    let _e785 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e777, _e771), 13f, 13f);
                    phi_37_ = 0i;
                    phi_38_ = 0.5f;
                    phi_39_ = 0f;
                    phi_40_ = vec2<f32>(((_e777 + (_e544 * 1.8f)) * 0.035f), (((_e771 + -(_e544)) * 0.035f) + 6.1f));
                    loop {
                        let _e795 = phi_37_;
                        let _e797 = phi_38_;
                        let _e799 = phi_39_;
                        let _e801 = phi_40_;
                        local_32 = _e799;
                        let _e802 = (_e795 < 4i);
                        if _e802 {
                            let _e805 = cantus_render_shader_simplex_noise(_e801);
                            phi_41_ = (_e795 + 1i);
                            phi_42_ = (_e797 * 0.5f);
                            phi_43_ = (_e799 + (_e805 * _e797));
                            phi_44_ = vec2<f32>(((_e801.x * 1.6f) + (_e801.y * 1.2f)), ((_e801.y * 1.6f) - (_e801.x * 1.2f)));
                        } else {
                            phi_41_ = i32();
                            phi_42_ = f32();
                            phi_43_ = f32();
                            phi_44_ = vec2<f32>();
                        }
                        let _e818 = phi_41_;
                        let _e820 = phi_42_;
                        let _e822 = phi_43_;
                        let _e824 = phi_44_;
                        continue;
                        continuing {
                            phi_37_ = _e818;
                            phi_38_ = _e820;
                            phi_39_ = _e822;
                            phi_40_ = _e824;
                            break if !(_e802);
                        }
                    }
                    let _e827 = local_32;
                    let _e828 = (_e827 * 0.5f);
                    let _e831 = ((_e785 - -0.5f) * 0.5f);
                    let _e833 = select(_e831, 0f, (_e831 < 0f));
                    let _e835 = select(_e833, 1f, (_e833 > 1f));
                    let _e841 = ((_e785 - 14f) * -0.083333336f);
                    let _e843 = select(_e841, 0f, (_e841 < 0f));
                    let _e845 = select(_e843, 1f, (_e843 > 1f));
                    let _e850 = (((_e835 * _e835) * (3f - (2f * _e835))) * ((_e845 * _e845) * (3f - (2f * _e845))));
                    let _e855 = ((_e828 + 0.19999999f) * 3.125f);
                    let _e857 = select(_e855, 0f, (_e855 < 0f));
                    let _e859 = select(_e857, 1f, (_e857 > 1f));
                    let _e866 = ((_e782 - 62f) * 0.045454547f);
                    let _e868 = select(_e866, 0f, (_e866 < 0f));
                    let _e870 = select(_e868, 1f, (_e868 > 1f));
                    let _e874 = ((_e870 * _e870) * (3f - (2f * _e870)));
                    phi_45_ = vec2<f32>(((_e850 * (0.18f + ((0.5f + _e828) * 0.34f))) * _e874), ((_e850 * ((_e859 * _e859) * (3f - (2f * _e859)))) * _e874));
                }
                let _e879 = phi_45_;
                let _e882 = (_e714 - 140f);
                let _e887 = pill_1.member[_e231].gpu.temperature;
                if (_e887 <= 62f) {
                    phi_54_ = vec2<f32>(0f, 0f);
                } else {
                    let _e890 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e882, _e771), 13f, 13f);
                    phi_46_ = 0i;
                    phi_47_ = 0.5f;
                    phi_48_ = 0f;
                    phi_49_ = vec2<f32>(((_e882 + (_e544 * 1.8f)) * 0.035f), (((_e771 + -(_e544)) * 0.035f) + 6.1f));
                    loop {
                        let _e900 = phi_46_;
                        let _e902 = phi_47_;
                        let _e904 = phi_48_;
                        let _e906 = phi_49_;
                        local_33 = _e904;
                        let _e907 = (_e900 < 4i);
                        if _e907 {
                            let _e910 = cantus_render_shader_simplex_noise(_e906);
                            phi_50_ = (_e900 + 1i);
                            phi_51_ = (_e902 * 0.5f);
                            phi_52_ = (_e904 + (_e910 * _e902));
                            phi_53_ = vec2<f32>(((_e906.x * 1.6f) + (_e906.y * 1.2f)), ((_e906.y * 1.6f) - (_e906.x * 1.2f)));
                        } else {
                            phi_50_ = i32();
                            phi_51_ = f32();
                            phi_52_ = f32();
                            phi_53_ = vec2<f32>();
                        }
                        let _e923 = phi_50_;
                        let _e925 = phi_51_;
                        let _e927 = phi_52_;
                        let _e929 = phi_53_;
                        continue;
                        continuing {
                            phi_46_ = _e923;
                            phi_47_ = _e925;
                            phi_48_ = _e927;
                            phi_49_ = _e929;
                            break if !(_e907);
                        }
                    }
                    let _e932 = local_33;
                    let _e933 = (_e932 * 0.5f);
                    let _e936 = ((_e890 - -0.5f) * 0.5f);
                    let _e938 = select(_e936, 0f, (_e936 < 0f));
                    let _e940 = select(_e938, 1f, (_e938 > 1f));
                    let _e946 = ((_e890 - 14f) * -0.083333336f);
                    let _e948 = select(_e946, 0f, (_e946 < 0f));
                    let _e950 = select(_e948, 1f, (_e948 > 1f));
                    let _e955 = (((_e940 * _e940) * (3f - (2f * _e940))) * ((_e950 * _e950) * (3f - (2f * _e950))));
                    let _e960 = ((_e933 + 0.19999999f) * 3.125f);
                    let _e962 = select(_e960, 0f, (_e960 < 0f));
                    let _e964 = select(_e962, 1f, (_e962 > 1f));
                    let _e971 = ((_e887 - 62f) * 0.045454547f);
                    let _e973 = select(_e971, 0f, (_e971 < 0f));
                    let _e975 = select(_e973, 1f, (_e973 > 1f));
                    let _e979 = ((_e975 * _e975) * (3f - (2f * _e975)));
                    phi_54_ = vec2<f32>(((_e955 * (0.18f + ((0.5f + _e933) * 0.34f))) * _e979), ((_e955 * ((_e964 * _e964) * (3f - (2f * _e964)))) * _e979));
                }
                let _e984 = phi_54_;
                phi_55_ = vec2<f32>(select(_e984.x, _e879.x, (_e879.x > _e984.x)), select(_e984.y, _e879.y, (_e879.y > _e984.y)));
            } else {
                phi_55_ = _e774;
            }
            let _e993 = phi_55_;
            let _e998 = pill_1.member[_e231].cpu.temperature;
            let _e1003 = pill_1.member[_e231].gpu.temperature;
            if (_e998 != _e998) {
                phi_56_ = true;
            } else {
                phi_56_ = (_e1003 >= _e998);
            }
            let _e1007 = phi_56_;
            let _e1008 = select(_e998, _e1003, _e1007);
            let _e1010 = ((_e1008 - 60f) * 0.083333336f);
            let _e1012 = select(_e1010, 0f, (_e1010 < 0f));
            let _e1014 = select(_e1012, 1f, (_e1012 > 1f));
            let _e1018 = ((_e1014 * _e1014) * (3f - (2f * _e1014)));
            let _e1019 = (1f - _e1018);
            let _e1028 = ((_e1008 - 72f) * 0.0625f);
            let _e1030 = select(_e1028, 0f, (_e1028 < 0f));
            let _e1032 = select(_e1030, 1f, (_e1030 > 1f));
            let _e1036 = ((_e1032 * _e1032) * (3f - (2f * _e1032)));
            let _e1037 = (1f - _e1036);
            let _e1047 = (_e993.y * 0.12f);
            let _e1048 = (0.24f + _e1047);
            let _e1049 = (0.76f - _e1047);
            let _e1061 = (1f - (_e993.x * 0.46f));
            let _e1071 = (_e993.y * 0.64f);
            let _e1072 = (1f - _e1071);
            let _e1079 = (((((((((((((((((((0.008f * _e583) + (0.03f * _e560)) * _e581) + (((0.09f * _e583) + (0.34f * _e560)) * _e570)) * _e619) + ((_e595 + (0.3f * _e560)) * _e582)) * _e634) + (0.16f * _e633)) * _e645) + _e649) * _e654) + (_e540.fog * 0.3844f)) * _e678) + (_e676 * 0.2925f)) + _e710) * _e1061) + (_e993.x * 0.0009200001f)) * _e1072) + (((0.07f * _e1049) + (((((0.22f * _e1019) + _e1018) * _e1037) + _e1036) * _e1048)) * _e1071));
            let _e1080 = (((((((((((((((((((0.015f * _e583) + (0.06f * _e560)) * _e581) + (((0.37f * _e583) + (0.7f * _e560)) * _e570)) * _e619) + (((0.25f * _e583) + (0.2f * _e560)) * _e582)) * _e634) + (0.2f * _e633)) * _e645) + _e649) * _e654) + (_e540.fog * 0.4216f)) * _e678) + (_e676 * 0.333f)) + _e710) * _e1061) + (_e993.x * 0.00276f)) * _e1072) + (((0.12f * _e1049) + (((((0.62f * _e1019) + (0.38f * _e1018)) * _e1037) + (0.08f * _e1036)) * _e1048)) * _e1071));
            let _e1081 = (((((((((((((((((((0.04f * _e583) + (0.13f * _e560)) * _e581) + ((_e595 + (0.9f * _e560)) * _e570)) * _e619) + (((0.2f * _e583) + (0.4f * _e560)) * _e582)) * _e634) + (0.27f * _e633)) * _e645) + _e649) * _e654) + (_e540.fog * 0.44640002f)) * _e678) + (_e676 * 0.43199998f)) + _e710) * _e1061) + (_e993.x * 0.00552f)) * _e1072) + (((0.18f * _e1049) + ((((_e1019 + (0.08f * _e1018)) * _e1037) + (0.035f * _e1036)) * _e1048)) * _e1071));
            switch bitcast<i32>(_e745) {
                case 0: {
                    let _e1797 = pill_1.member[_e231].history_scroll;
                    switch bitcast<i32>(_e745) {
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
                    let _e1800 = phi_63_;
                    let _e1802 = phi_64_;
                    let _e1804 = phi_65_;
                    let _e1805 = select(_e1802, false, _e1800);
                    let _e1811 = ((select(select(80f, 32f, _e1805), 24f, select(select(_e1804, false, _e1800), false, _e1805)) * 0.5f) - 4f);
                    let _e1812 = (_e258 - 8f);
                    let _e1813 = (_e1811 - _e1812);
                    let _e1815 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e770, _e771), _e1813, _e1812);
                    let _e1816 = abs(_e770);
                    let _e1817 = abs(_e771);
                    let _e1820 = (round((_e1816 * 0.11111111f)) * 9f);
                    if (_e1820 != _e1820) {
                        phi_66_ = true;
                    } else {
                        phi_66_ = (_e1811 <= _e1820);
                    }
                    let _e1824 = phi_66_;
                    let _e1825 = select(_e1820, _e1811, _e1824);
                    let _e1826 = (_e1825 - _e1813);
                    if (_e1826 != _e1826) {
                        phi_67_ = true;
                    } else {
                        phi_67_ = (0f >= _e1826);
                    }
                    let _e1830 = phi_67_;
                    let _e1831 = select(_e1826, 0f, _e1830);
                    let _e1832 = (_e1812 * _e1812);
                    let _e1835 = sqrt((_e1832 - (_e1831 * _e1831)));
                    let _e1836 = (_e1831 / _e1812);
                    let _e1837 = (_e1835 / _e1812);
                    let _e1842 = ((_e1816 - _e1825) - (_e1836 * 0.9f));
                    let _e1843 = ((_e1817 - _e1835) - (_e1837 * 0.9f));
                    let _e1852 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1842 * -(_e1837)) + (_e1843 * _e1836)), ((_e1842 * _e1836) + (_e1843 * _e1837))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e1854 = round((_e1817 * 0.125f));
                    if (_e1854 != _e1854) {
                        phi_68_ = true;
                    } else {
                        phi_68_ = (1f <= _e1854);
                    }
                    let _e1858 = phi_68_;
                    let _e1860 = (select(_e1854, 1f, _e1858) * 8f);
                    let _e1863 = sqrt((_e1832 - (_e1860 * _e1860)));
                    let _e1865 = (_e1863 / _e1812);
                    let _e1866 = (_e1860 / _e1812);
                    let _e1871 = ((_e1816 - (_e1813 + _e1863)) - (_e1865 * 0.9f));
                    let _e1872 = ((_e1817 - _e1860) - (_e1866 * 0.9f));
                    let _e1881 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1871 * -(_e1866)) + (_e1872 * _e1865)), ((_e1871 * _e1865) + (_e1872 * _e1866))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e1852 != _e1852) {
                        phi_69_ = true;
                    } else {
                        phi_69_ = (_e1881 <= _e1852);
                    }
                    let _e1885 = phi_69_;
                    let _e1886 = select(_e1852, _e1881, _e1885);
                    let _e1889 = (0.5f + ((_e1886 - _e1815) * 0.3125f));
                    let _e1891 = select(_e1889, 0f, (_e1889 < 0f));
                    let _e1893 = select(_e1891, 1f, (_e1891 > 1f));
                    let _e1902 = ((_e1815 - 0.55f) * -0.9090909f);
                    let _e1904 = select(_e1902, 0f, (_e1902 < 0f));
                    let _e1906 = select(_e1904, 1f, (_e1904 > 1f));
                    let _e1910 = ((_e1906 * _e1906) * (3f - (2f * _e1906)));
                    let _e1911 = (_e1811 * 0.051282052f);
                    let _e1912 = (_e770 + _e1811);
                    let _e1914 = ((_e1912 / _e1911) + _e1797);
                    let _e1916 = select(_e1914, 0f, (_e1914 < 0f));
                    let _e1918 = select(_e1916, 39f, (_e1916 > 39f));
                    let _e1919 = floor(_e1918);
                    let _e1924 = select(select(u32(_e1919), 0u, (_e1919 < 0f)), 4294967295u, (_e1919 > 4294967000f));
                    let _e1925 = (_e258 - 10f);
                    let _e1929 = (((f32(_e1924) - _e1797) * _e1911) - _e1811);
                    let _e1931 = select(_e1924, 39u, (39u < _e1924));
                    let _e1932 = (_e1931 < 40u);
                    if _e1932 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1939 = pill_1.member[_e231].cpu.usage.samples[_e1931];
                    let _e1942 = (_e1925 * (1f - (_e1939 * 2f)));
                    let _e1943 = (_e1924 + 1u);
                    let _e1949 = select(_e1943, 39u, (39u < _e1943));
                    let _e1950 = (_e1949 < 40u);
                    if _e1950 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1957 = pill_1.member[_e231].cpu.usage.samples[_e1949];
                    let _e1961 = ((((f32(_e1943) - _e1797) * _e1911) - _e1811) - _e1929);
                    let _e1962 = ((_e1925 * (1f - (_e1957 * 2f))) - _e1942);
                    let _e1963 = (_e770 - _e1929);
                    let _e1964 = (_e771 - _e1942);
                    let _e1965 = (_e1963 * _e1961);
                    let _e1968 = (_e1961 * _e1961);
                    let _e1970 = (_e1968 + (_e1962 * _e1962));
                    if (_e1970 != _e1970) {
                        phi_70_ = true;
                    } else {
                        phi_70_ = (0.001f >= _e1970);
                    }
                    let _e1974 = phi_70_;
                    let _e1976 = ((_e1965 + (_e1964 * _e1962)) / select(_e1970, 0.001f, _e1974));
                    let _e1978 = select(_e1976, 0f, (_e1976 < 0f));
                    let _e1980 = select(_e1978, 1f, (_e1978 > 1f));
                    let _e1983 = (_e1963 - (_e1961 * _e1980));
                    let _e1984 = (_e1964 - (_e1962 * _e1980));
                    let _e1991 = ((abs(sqrt(((_e1983 * _e1983) + (_e1984 * _e1984)))) - 1.4000001f) * -0.9090908f);
                    let _e1993 = select(_e1991, 0f, (_e1991 < 0f));
                    let _e1995 = select(_e1993, 1f, (_e1993 > 1f));
                    let _e2001 = (_e1918 - trunc(_e1918));
                    let _e2003 = select(_e2001, 0f, (_e2001 < 0f));
                    let _e2005 = select(_e2003, 1f, (_e2003 > 1f));
                    let _e2009 = ((_e2005 * _e2005) * (3f - (2f * _e2005)));
                    let _e2016 = ((((_e1942 + (_e1962 * _e2009)) - _e771) - 0.55f) * -0.9090909f);
                    let _e2018 = select(_e2016, 0f, (_e2016 < 0f));
                    let _e2020 = select(_e2018, 1f, (_e2018 > 1f));
                    let _e2026 = ((((_e2020 * _e2020) * (3f - (2f * _e2020))) * 0.156f) + ((_e1995 * _e1995) * (3f - (2f * _e1995))));
                    if _e1932 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e2035 = pill_1.member[_e231].cpu.memory.samples[_e1931];
                    let _e2038 = (_e1925 * (1f - (_e2035 * 2f)));
                    if _e1950 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e2045 = pill_1.member[_e231].cpu.memory.samples[_e1949];
                    let _e2049 = ((_e1925 * (1f - (_e2045 * 2f))) - _e2038);
                    let _e2050 = (_e771 - _e2038);
                    let _e2054 = (_e1968 + (_e2049 * _e2049));
                    if (_e2054 != _e2054) {
                        phi_71_ = true;
                    } else {
                        phi_71_ = (0.001f >= _e2054);
                    }
                    let _e2058 = phi_71_;
                    let _e2060 = ((_e1965 + (_e2050 * _e2049)) / select(_e2054, 0.001f, _e2058));
                    let _e2062 = select(_e2060, 0f, (_e2060 < 0f));
                    let _e2064 = select(_e2062, 1f, (_e2062 > 1f));
                    let _e2067 = (_e1963 - (_e1961 * _e2064));
                    let _e2068 = (_e2050 - (_e2049 * _e2064));
                    let _e2075 = ((abs(sqrt(((_e2067 * _e2067) + (_e2068 * _e2068)))) - 1.4000001f) * -0.9090908f);
                    let _e2077 = select(_e2075, 0f, (_e2075 < 0f));
                    let _e2079 = select(_e2077, 1f, (_e2077 > 1f));
                    let _e2090 = ((((_e2038 + (_e2049 * _e2009)) - _e771) - 0.55f) * -0.9090909f);
                    let _e2092 = select(_e2090, 0f, (_e2090 < 0f));
                    let _e2094 = select(_e2092, 1f, (_e2092 > 1f));
                    let _e2100 = ((((_e2094 * _e2094) * (3f - (2f * _e2094))) * 0.084f) + ((_e2079 * _e2079) * (3f - (2f * _e2079))));
                    let _e2108 = (_e1912 * 0.14285715f);
                    let _e2109 = ((_e771 + _e1812) * 0.16393442f);
                    let _e2119 = ((abs(((_e2108 - trunc(_e2108)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2121 = select(_e2119, 0f, (_e2119 < 0f));
                    let _e2123 = select(_e2121, 1f, (_e2121 > 1f));
                    let _e2127 = ((_e2123 * _e2123) * (3f - (2f * _e2123)));
                    let _e2129 = ((abs(((_e2109 - trunc(_e2109)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2131 = select(_e2129, 0f, (_e2129 < 0f));
                    let _e2133 = select(_e2131, 1f, (_e2131 > 1f));
                    let _e2137 = ((_e2133 * _e2133) * (3f - (2f * _e2133)));
                    if (_e2127 != _e2127) {
                        phi_72_ = true;
                    } else {
                        phi_72_ = (_e2137 >= _e2127);
                    }
                    let _e2141 = phi_72_;
                    let _e2149 = pill_1.member[_e231].cpu.usage.samples[39u];
                    let _e2150 = (_e2149 * 0.24f);
                    let _e2151 = (0.18f + _e2150);
                    let _e2152 = (0.82f - _e2150);
                    let _e2161 = (_e998 - 60f);
                    let _e2162 = (_e2161 * 0.083333336f);
                    let _e2164 = select(_e2162, 0f, (_e2162 < 0f));
                    let _e2166 = select(_e2164, 1f, (_e2164 > 1f));
                    let _e2170 = ((_e2166 * _e2166) * (3f - (2f * _e2166)));
                    let _e2171 = (1f - _e2170);
                    let _e2180 = ((_e998 - 72f) * 0.0625f);
                    let _e2182 = select(_e2180, 0f, (_e2180 < 0f));
                    let _e2184 = select(_e2182, 1f, (_e2182 > 1f));
                    let _e2188 = ((_e2184 * _e2184) * (3f - (2f * _e2184)));
                    let _e2189 = (1f - _e2188);
                    let _e2198 = (_e2161 * 0.03846154f);
                    let _e2200 = select(_e2198, 0f, (_e2198 < 0f));
                    let _e2202 = select(_e2200, 1f, (_e2200 > 1f));
                    let _e2207 = (((_e2202 * _e2202) * (3f - (2f * _e2202))) * 0.9f);
                    let _e2208 = (1f - _e2207);
                    let _e2215 = ((((0.025f * _e2152) + (0.32f * _e2151)) * _e2208) + (((((0.22f * _e2171) + _e2170) * _e2189) + _e2188) * _e2207));
                    let _e2216 = ((((0.09f * _e2152) + (0.68f * _e2151)) * _e2208) + (((((0.62f * _e2171) + (0.38f * _e2170)) * _e2189) + (0.08f * _e2188)) * _e2207));
                    let _e2217 = ((((0.15f * _e2152) + _e2151) * _e2208) + ((((_e2171 + (0.08f * _e2170)) * _e2189) + (0.035f * _e2188)) * _e2207));
                    let _e2219 = ((((_e1886 + ((_e1815 - _e1886) * _e1893)) - ((1.6f * _e1893) * (1f - _e1893))) - 0.55f) * -0.9090909f);
                    let _e2221 = select(_e2219, 0f, (_e2219 < 0f));
                    let _e2223 = select(_e2221, 1f, (_e2221 > 1f));
                    let _e2227 = ((_e2223 * _e2223) * (3f - (2f * _e2223)));
                    let _e2229 = (1f - (_e2227 * 0.82f));
                    let _e2241 = ((abs(_e1815) - 2.1f) * -0.909091f);
                    let _e2243 = select(_e2241, 0f, (_e2241 < 0f));
                    let _e2245 = select(_e2243, 1f, (_e2243 > 1f));
                    let _e2250 = (((_e2245 * _e2245) * (3f - (2f * _e2245))) * 0.92f);
                    let _e2251 = (1f - _e2250);
                    let _e2262 = ((_e1886 - 0.55f) * -0.9090909f);
                    let _e2264 = select(_e2262, 0f, (_e2262 < 0f));
                    let _e2266 = select(_e2264, 1f, (_e2264 > 1f));
                    let _e2271 = (((_e2266 * _e2266) * (3f - (2f * _e2266))) * 0.78f);
                    let _e2272 = (1f - _e2271);
                    let _e2283 = ((_e1910 * select(_e2127, _e2137, _e2141)) * 0.045f);
                    phi_73_ = _e405;
                    phi_74_ = vec3<f32>(((((((((_e1079 * _e2229) + (_e2227 * 0.00328f)) * _e2251) + (_e2215 * _e2250)) * _e2272) + (_e2215 * _e2271)) + _e2283) + (((0.32f * _e1910) * _e2026) + ((0.78f * _e1910) * _e2100))), ((((((((_e1080 * _e2229) + (_e2227 * 0.00984f)) * _e2251) + (_e2216 * _e2250)) * _e2272) + (_e2216 * _e2271)) + _e2283) + (((0.68f * _e1910) * _e2026) + ((0.3f * _e1910) * _e2100))), ((((((((_e1081 * _e2229) + (_e2227 * 0.02132f)) * _e2251) + (_e2217 * _e2250)) * _e2272) + (_e2217 * _e2271)) + _e2283) + (_e1910 * (_e2026 + _e2100))));
                    phi_75_ = false;
                    break;
                }
                case 1: {
                    let _e1416 = pill_1.member[_e231].history_scroll;
                    switch bitcast<i32>(_e745) {
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
                    let _e1419 = phi_57_;
                    let _e1421 = phi_58_;
                    let _e1423 = phi_59_;
                    let _e1424 = select(_e1421, false, _e1419);
                    let _e1430 = ((select(select(80f, 32f, _e1424), 24f, select(select(_e1423, false, _e1419), false, _e1424)) * 0.5f) - 4f);
                    let _e1431 = (_e258 - 8f);
                    let _e1434 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e770, _e771), (_e1430 - _e1431), _e1431);
                    let _e1436 = ((_e1434 - 0.55f) * -0.9090909f);
                    let _e1438 = select(_e1436, 0f, (_e1436 < 0f));
                    let _e1440 = select(_e1438, 1f, (_e1438 > 1f));
                    let _e1444 = ((_e1440 * _e1440) * (3f - (2f * _e1440)));
                    let _e1445 = (_e1430 * 0.051282052f);
                    let _e1446 = (_e770 + _e1430);
                    let _e1448 = ((_e1446 / _e1445) + _e1416);
                    let _e1450 = select(_e1448, 0f, (_e1448 < 0f));
                    let _e1452 = select(_e1450, 39f, (_e1450 > 39f));
                    let _e1453 = floor(_e1452);
                    let _e1458 = select(select(u32(_e1453), 0u, (_e1453 < 0f)), 4294967295u, (_e1453 > 4294967000f));
                    let _e1459 = (_e258 - 10f);
                    let _e1463 = (((f32(_e1458) - _e1416) * _e1445) - _e1430);
                    let _e1465 = select(_e1458, 39u, (39u < _e1458));
                    let _e1466 = (_e1465 < 40u);
                    if _e1466 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1473 = pill_1.member[_e231].gpu.usage.samples[_e1465];
                    let _e1476 = (_e1459 * (1f - (_e1473 * 2f)));
                    let _e1477 = (_e1458 + 1u);
                    let _e1483 = select(_e1477, 39u, (39u < _e1477));
                    let _e1484 = (_e1483 < 40u);
                    if _e1484 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1491 = pill_1.member[_e231].gpu.usage.samples[_e1483];
                    let _e1495 = ((((f32(_e1477) - _e1416) * _e1445) - _e1430) - _e1463);
                    let _e1496 = ((_e1459 * (1f - (_e1491 * 2f))) - _e1476);
                    let _e1497 = (_e770 - _e1463);
                    let _e1498 = (_e771 - _e1476);
                    let _e1499 = (_e1497 * _e1495);
                    let _e1502 = (_e1495 * _e1495);
                    let _e1504 = (_e1502 + (_e1496 * _e1496));
                    if (_e1504 != _e1504) {
                        phi_60_ = true;
                    } else {
                        phi_60_ = (0.001f >= _e1504);
                    }
                    let _e1508 = phi_60_;
                    let _e1510 = ((_e1499 + (_e1498 * _e1496)) / select(_e1504, 0.001f, _e1508));
                    let _e1512 = select(_e1510, 0f, (_e1510 < 0f));
                    let _e1514 = select(_e1512, 1f, (_e1512 > 1f));
                    let _e1517 = (_e1497 - (_e1495 * _e1514));
                    let _e1518 = (_e1498 - (_e1496 * _e1514));
                    let _e1525 = ((abs(sqrt(((_e1517 * _e1517) + (_e1518 * _e1518)))) - 1.4000001f) * -0.9090908f);
                    let _e1527 = select(_e1525, 0f, (_e1525 < 0f));
                    let _e1529 = select(_e1527, 1f, (_e1527 > 1f));
                    let _e1535 = (_e1452 - trunc(_e1452));
                    let _e1537 = select(_e1535, 0f, (_e1535 < 0f));
                    let _e1539 = select(_e1537, 1f, (_e1537 > 1f));
                    let _e1543 = ((_e1539 * _e1539) * (3f - (2f * _e1539)));
                    let _e1550 = ((((_e1476 + (_e1496 * _e1543)) - _e771) - 0.55f) * -0.9090909f);
                    let _e1552 = select(_e1550, 0f, (_e1550 < 0f));
                    let _e1554 = select(_e1552, 1f, (_e1552 > 1f));
                    let _e1560 = ((((_e1554 * _e1554) * (3f - (2f * _e1554))) * 0.156f) + ((_e1529 * _e1529) * (3f - (2f * _e1529))));
                    if _e1466 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1569 = pill_1.member[_e231].gpu.memory.samples[_e1465];
                    let _e1572 = (_e1459 * (1f - (_e1569 * 2f)));
                    if _e1484 {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1579 = pill_1.member[_e231].gpu.memory.samples[_e1483];
                    let _e1583 = ((_e1459 * (1f - (_e1579 * 2f))) - _e1572);
                    let _e1584 = (_e771 - _e1572);
                    let _e1588 = (_e1502 + (_e1583 * _e1583));
                    if (_e1588 != _e1588) {
                        phi_61_ = true;
                    } else {
                        phi_61_ = (0.001f >= _e1588);
                    }
                    let _e1592 = phi_61_;
                    let _e1594 = ((_e1499 + (_e1584 * _e1583)) / select(_e1588, 0.001f, _e1592));
                    let _e1596 = select(_e1594, 0f, (_e1594 < 0f));
                    let _e1598 = select(_e1596, 1f, (_e1596 > 1f));
                    let _e1601 = (_e1497 - (_e1495 * _e1598));
                    let _e1602 = (_e1584 - (_e1583 * _e1598));
                    let _e1609 = ((abs(sqrt(((_e1601 * _e1601) + (_e1602 * _e1602)))) - 1.4000001f) * -0.9090908f);
                    let _e1611 = select(_e1609, 0f, (_e1609 < 0f));
                    let _e1613 = select(_e1611, 1f, (_e1611 > 1f));
                    let _e1624 = ((((_e1572 + (_e1583 * _e1543)) - _e771) - 0.55f) * -0.9090909f);
                    let _e1626 = select(_e1624, 0f, (_e1624 < 0f));
                    let _e1628 = select(_e1626, 1f, (_e1626 > 1f));
                    let _e1634 = ((((_e1628 * _e1628) * (3f - (2f * _e1628))) * 0.084f) + ((_e1613 * _e1613) * (3f - (2f * _e1613))));
                    let _e1642 = (_e1446 * 0.14285715f);
                    let _e1643 = ((_e771 + _e1431) * 0.16393442f);
                    let _e1653 = ((abs(((_e1642 - trunc(_e1642)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e1655 = select(_e1653, 0f, (_e1653 < 0f));
                    let _e1657 = select(_e1655, 1f, (_e1655 > 1f));
                    let _e1661 = ((_e1657 * _e1657) * (3f - (2f * _e1657)));
                    let _e1663 = ((abs(((_e1643 - trunc(_e1643)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e1665 = select(_e1663, 0f, (_e1663 < 0f));
                    let _e1667 = select(_e1665, 1f, (_e1665 > 1f));
                    let _e1671 = ((_e1667 * _e1667) * (3f - (2f * _e1667)));
                    if (_e1661 != _e1661) {
                        phi_62_ = true;
                    } else {
                        phi_62_ = (_e1671 >= _e1661);
                    }
                    let _e1675 = phi_62_;
                    let _e1683 = pill_1.member[_e231].gpu.usage.samples[39u];
                    let _e1684 = (_e1683 * 0.24f);
                    let _e1685 = (0.18f + _e1684);
                    let _e1686 = (0.82f - _e1684);
                    let _e1695 = (_e1003 - 60f);
                    let _e1696 = (_e1695 * 0.083333336f);
                    let _e1698 = select(_e1696, 0f, (_e1696 < 0f));
                    let _e1700 = select(_e1698, 1f, (_e1698 > 1f));
                    let _e1704 = ((_e1700 * _e1700) * (3f - (2f * _e1700)));
                    let _e1705 = (1f - _e1704);
                    let _e1714 = ((_e1003 - 72f) * 0.0625f);
                    let _e1716 = select(_e1714, 0f, (_e1714 < 0f));
                    let _e1718 = select(_e1716, 1f, (_e1716 > 1f));
                    let _e1722 = ((_e1718 * _e1718) * (3f - (2f * _e1718)));
                    let _e1723 = (1f - _e1722);
                    let _e1732 = (_e1695 * 0.03846154f);
                    let _e1734 = select(_e1732, 0f, (_e1732 < 0f));
                    let _e1736 = select(_e1734, 1f, (_e1734 > 1f));
                    let _e1741 = (((_e1736 * _e1736) * (3f - (2f * _e1736))) * 0.9f);
                    let _e1742 = (1f - _e1741);
                    let _e1753 = (1f - (_e1444 * 0.82f));
                    let _e1765 = ((abs(_e1434) - 2.1f) * -0.909091f);
                    let _e1767 = select(_e1765, 0f, (_e1765 < 0f));
                    let _e1769 = select(_e1767, 1f, (_e1767 > 1f));
                    let _e1774 = (((_e1769 * _e1769) * (3f - (2f * _e1769))) * 0.92f);
                    let _e1775 = (1f - _e1774);
                    let _e1786 = ((_e1444 * select(_e1661, _e1671, _e1675)) * 0.045f);
                    phi_73_ = _e405;
                    phi_74_ = vec3<f32>(((((((_e1079 * _e1753) + (_e1444 * 0.00328f)) * _e1775) + (((((0.025f * _e1686) + (0.32f * _e1685)) * _e1742) + (((((0.22f * _e1705) + _e1704) * _e1723) + _e1722) * _e1741)) * _e1774)) + _e1786) + (((0.32f * _e1444) * _e1560) + ((0.78f * _e1444) * _e1634))), ((((((_e1080 * _e1753) + (_e1444 * 0.00984f)) * _e1775) + (((((0.09f * _e1686) + (0.68f * _e1685)) * _e1742) + (((((0.62f * _e1705) + (0.38f * _e1704)) * _e1723) + (0.08f * _e1722)) * _e1741)) * _e1774)) + _e1786) + (((0.68f * _e1444) * _e1560) + ((0.3f * _e1444) * _e1634))), ((((((_e1081 * _e1753) + (_e1444 * 0.02132f)) * _e1775) + (((((0.15f * _e1686) + _e1685) * _e1742) + ((((_e1705 + (0.08f * _e1704)) * _e1723) + (0.035f * _e1722)) * _e1741)) * _e1774)) + _e1786) + (_e1444 * (_e1560 + _e1634))));
                    phi_75_ = false;
                    break;
                }
                case 2: {
                    let _e1208 = (_e770 * 1.25f);
                    let _e1209 = (_e771 * 1.25f);
                    let _e1211 = select(0f, 1f, (_e237 < 0f));
                    let _e1212 = abs(_e237);
                    let _e1213 = (_e1209 - 1f);
                    let _e1214 = vec2<f32>(_e1208, _e1213);
                    let _e1215 = cantus_render_shader_sd_rounded_box(_e1214, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e1218 = ((abs(_e1215) - 2.425f) * -0.909091f);
                    let _e1220 = select(_e1218, 0f, (_e1218 < 0f));
                    let _e1222 = select(_e1220, 1f, (_e1220 > 1f));
                    let _e1229 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1208, (_e1209 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e1231 = ((_e1229 - 0.55f) * -0.9090909f);
                    let _e1233 = select(_e1231, 0f, (_e1231 < 0f));
                    let _e1235 = select(_e1233, 1f, (_e1233 > 1f));
                    let _e1240 = cantus_render_shader_sd_rounded_box(_e1214, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e1242 = ((_e1240 - 0.55f) * -0.9090909f);
                    let _e1244 = select(_e1242, 0f, (_e1242 < 0f));
                    let _e1246 = select(_e1244, 1f, (_e1244 > 1f));
                    let _e1250 = ((_e1246 * _e1246) * (3f - (2f * _e1246)));
                    let _e1252 = select(_e1212, 0f, (_e1212 < 0f));
                    let _e1270 = ((12f - (select(_e1252, 1f, (_e1252 > 1f)) * 24f)) + ((sin(((_e770 * 0.775f) + (_e544 * (1.4f + (_e1211 * 1.2f))))) * 1.15f) + (sin(((_e770 * 0.3375f) - (_e544 * 0.8f))) * 0.45f)));
                    let _e1271 = (_e1270 - 0.7f);
                    let _e1275 = ((_e1213 - _e1271) / ((_e1270 + 0.7f) - _e1271));
                    let _e1277 = select(_e1275, 0f, (_e1275 < 0f));
                    let _e1279 = select(_e1277, 1f, (_e1277 > 1f));
                    let _e1284 = (_e1250 * ((_e1279 * _e1279) * (3f - (2f * _e1279))));
                    let _e1286 = ((_e1212 - 0.08f) * 5f);
                    let _e1288 = select(_e1286, 0f, (_e1286 < 0f));
                    let _e1290 = select(_e1288, 1f, (_e1288 > 1f));
                    let _e1294 = ((_e1290 * _e1290) * (3f - (2f * _e1290)));
                    let _e1295 = (1f - _e1294);
                    let _e1303 = ((_e1212 - 0.18f) * 1.8518518f);
                    let _e1305 = select(_e1303, 0f, (_e1303 < 0f));
                    let _e1307 = select(_e1305, 1f, (_e1305 > 1f));
                    let _e1311 = ((_e1307 * _e1307) * (3f - (2f * _e1307)));
                    let _e1312 = (1f - _e1311);
                    let _e1318 = (_e1312 + (0.22f * _e1311));
                    let _e1319 = ((((0.18f * _e1295) + (0.72f * _e1294)) * _e1312) + (0.95f * _e1311));
                    let _e1320 = ((((0.1f * _e1295) + (0.12f * _e1294)) * _e1312) + (0.55f * _e1311));
                    let _e1322 = floor((_e770 * 0.4166667f));
                    let _e1324 = cantus_render_shader_hash(vec2<f32>(_e1322, 0f));
                    let _e1327 = (_e1324.y * 0.5f);
                    let _e1331 = ((_e544 * (0.35f + _e1327)) + (_e1324.x * 7f));
                    let _e1333 = (_e1331 - trunc(_e1331));
                    let _e1340 = (_e1208 - (((_e1322 + 0.2f) + (_e1324.x * 0.6f)) * 3f));
                    let _e1341 = (_e1209 - (13f - (_e1333 * 24f)));
                    let _e1348 = (_e1333 * 4f);
                    let _e1350 = select(_e1348, 0f, (_e1348 < 0f));
                    let _e1352 = select(_e1350, 1f, (_e1350 > 1f));
                    let _e1358 = ((_e1333 - 1f) * -3.3333333f);
                    let _e1360 = select(_e1358, 0f, (_e1358 < 0f));
                    let _e1362 = select(_e1360, 1f, (_e1360 > 1f));
                    let _e1370 = ((abs((sqrt(((_e1340 * _e1340) + (_e1341 * _e1341))) - (0.4f + _e1327))) - 1f) * -0.9090909f);
                    let _e1372 = select(_e1370, 0f, (_e1370 < 0f));
                    let _e1374 = select(_e1372, 1f, (_e1372 > 1f));
                    let _e1381 = (((((_e1374 * _e1374) * (3f - (2f * _e1374))) * (((_e1352 * _e1352) * (3f - (2f * _e1352))) * ((_e1362 * _e1362) * (3f - (2f * _e1362))))) * _e1250) * _e1211);
                    let _e1384 = ((((_e1222 * _e1222) * (3f - (2f * _e1222))) * 0.43f) + (((_e1235 * _e1235) * (3f - (2f * _e1235))) * 0.38f));
                    phi_73_ = _e405;
                    phi_74_ = vec3<f32>((_e1079 + ((_e1384 + ((_e1318 * _e1284) * 0.78f)) + ((((_e1318 * 0.27999997f) + 0.72f) * _e1381) * 0.9f))), (_e1080 + ((_e1384 + ((_e1319 * _e1284) * 0.78f)) + ((((_e1319 * 0.27999997f) + 0.72f) * _e1381) * 0.9f))), (_e1081 + ((_e1384 + ((_e1320 * _e1284) * 0.78f)) + ((((_e1320 * 0.27999997f) + 0.72f) * _e1381) * 0.9f))));
                    phi_75_ = false;
                    break;
                }
                case 3: {
                    let _e1086 = pill_1.member[_e231].volume;
                    let _e1088 = select(0f, 1f, (_e1086 < 0f));
                    let _e1089 = abs(_e1086);
                    let _e1092 = round(((_e770 + 12f) * 0.25f));
                    let _e1094 = select(_e1092, 0f, (_e1092 < 0f));
                    let _e1096 = select(_e1094, 6f, (_e1094 > 6f));
                    let _e1101 = select(select(u32(_e1096), 0u, (_e1096 < 0f)), 4294967295u, (_e1096 > 4294967000f));
                    if (_e1101 < 7u) {
                    } else {
                        phi_73_ = true;
                        phi_74_ = vec3<f32>();
                        phi_75_ = bool();
                        break;
                    }
                    let _e1107 = pill_1.member[_e231].audio_spectrum[_e1101];
                    let _e1108 = (1f - _e1088);
                    let _e1109 = (_e1107 * _e1108);
                    let _e1118 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e770 - (-12f + (_e1096 * 4f))), (_e771 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1109))), 1.25f);
                    let _e1121 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e770, (_e771 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1123 = ((_e1121 - 0.55f) * -0.9090909f);
                    let _e1125 = select(_e1123, 0f, (_e1123 < 0f));
                    let _e1127 = select(_e1125, 1f, (_e1125 > 1f));
                    let _e1131 = ((_e1127 * _e1127) * (3f - (2f * _e1127)));
                    let _e1133 = select(_e1089, 0f, (_e1089 < 0f));
                    let _e1136 = (select(_e1133, 1f, (_e1133 > 1f)) * 28f);
                    let _e1137 = (_e1136 + -13.2f);
                    let _e1141 = ((_e770 - _e1137) / ((_e1136 + -14.8f) - _e1137));
                    let _e1143 = select(_e1141, 0f, (_e1141 < 0f));
                    let _e1145 = select(_e1143, 1f, (_e1143 > 1f));
                    let _e1150 = (_e1131 * ((_e1145 * _e1145) * (3f - (2f * _e1145))));
                    let _e1152 = (1f - (_e1089 * 0.65f));
                    let _e1157 = ((0.08f * _e1152) + (_e1089 * 0.42249995f));
                    let _e1158 = ((0.88f * _e1152) + (_e1089 * 0.221f));
                    let _e1160 = ((_e1118 - 0.7f) * -0.71428573f);
                    let _e1162 = select(_e1160, 0f, (_e1160 < 0f));
                    let _e1164 = select(_e1162, 1f, (_e1162 > 1f));
                    let _e1173 = ((_e1118 - 3.2f) * -0.3125f);
                    let _e1175 = select(_e1173, 0f, (_e1173 < 0f));
                    let _e1177 = select(_e1175, 1f, (_e1175 > 1f));
                    let _e1184 = ((((_e1164 * _e1164) * (3f - (2f * _e1164))) * (0.58f + (_e1109 * 0.35f))) + ((((_e1177 * _e1177) * (3f - (2f * _e1177))) * _e1109) * 0.12f));
                    let _e1197 = (_e1150 + ((_e1131 * (1f - _e1150)) * 0.22f));
                    phi_73_ = _e405;
                    phi_74_ = vec3<f32>((_e1079 + ((_e1157 * _e1184) + (((_e1157 * _e1108) + _e1088) * _e1197))), (_e1080 + ((_e1158 * _e1184) + (((_e1158 * _e1108) + (0.24f * _e1088)) * _e1197))), (_e1081 + (_e1184 + ((_e1108 + (0.3f * _e1088)) * _e1197))));
                    phi_75_ = false;
                    break;
                }
                case 4: {
                    phi_73_ = _e405;
                    phi_74_ = vec3<f32>();
                    phi_75_ = true;
                    break;
                }
                case 5: {
                    phi_73_ = _e405;
                    phi_74_ = vec3<f32>();
                    phi_75_ = true;
                    break;
                }
                default: {
                    phi_73_ = _e405;
                    phi_74_ = vec3<f32>();
                    phi_75_ = bool();
                    break;
                }
            }
            let _e2292 = phi_73_;
            let _e2294 = phi_74_;
            let _e2296 = phi_75_;
            if _e2292 {
                break;
            }
            if _e2296 {
                let _e2298 = select(1f, 0f, (_e745 == 5u));
                let _e2302 = pill_1.member[_e231].power_hover;
                let _e2307 = ((abs((f32(_e2302) - _e2298)) - 0.4f) * -2.857143f);
                let _e2309 = select(_e2307, 0f, (_e2307 < 0f));
                let _e2311 = select(_e2309, 1f, (_e2309 > 1f));
                let _e2315 = ((_e2311 * _e2311) * (3f - (2f * _e2311)));
                let _e2317 = (1f + (_e2315 * 0.07f));
                let _e2318 = (_e770 / _e2317);
                let _e2319 = (_e771 / _e2317);
                let _e2323 = pill_1.member[_e231].power_action;
                let _e2328 = ((abs((f32(_e2323) - _e2298)) - 0.4f) * -2.857143f);
                let _e2330 = select(_e2328, 0f, (_e2328 < 0f));
                let _e2332 = select(_e2330, 1f, (_e2330 > 1f));
                let _e2336 = ((_e2332 * _e2332) * (3f - (2f * _e2332)));
                let _e2340 = pill_1.member[_e231].power_progress;
                let _e2341 = (_e2340 * _e2336);
                if (_e2298 < 0.5f) {
                    let _e2465 = select(_e2341, 0f, (_e2341 < 0f));
                    let _e2467 = select(_e2465, 1f, (_e2465 > 1f));
                    let _e2471 = ((_e2467 * _e2467) * (3f - (2f * _e2467)));
                    let _e2477 = (1f - _e2341);
                    let _e2486 = (_e2471 * 0.7f);
                    let _e2487 = (_e2486 + 1.5999999f);
                    let _e2492 = ((abs((sqrt(((_e2318 * _e2318) + (_e2319 * _e2319))) - ((7.5f - (_e2341 * 4.6f)) + (((sin((_e544 * 8f)) * _e2341) * _e2477) * 0.16f)))) - _e2487) / ((_e2486 + 0.49999994f) - _e2487));
                    let _e2494 = select(_e2492, 0f, (_e2492 < 0f));
                    let _e2496 = select(_e2494, 1f, (_e2494 > 1f));
                    let _e2505 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2318, (_e2319 - -7f)), vec2<f32>((3f * _e2477), 3f), 0.5f);
                    let _e2507 = ((_e2505 - 0.55f) * -0.9090909f);
                    let _e2509 = select(_e2507, 0f, (_e2507 < 0f));
                    let _e2511 = select(_e2509, 1f, (_e2509 > 1f));
                    let _e2525 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2318, (_e2319 - (-5f + (_e2341 * 3.5f)))), vec2<f32>((1.05f + (_e2471 * 0.45f)), (4.6f - (_e2341 * 3f))), 0.7f);
                    let _e2527 = ((_e2525 - 0.55f) * -0.9090909f);
                    let _e2529 = select(_e2527, 0f, (_e2527 < 0f));
                    let _e2531 = select(_e2529, 1f, (_e2529 > 1f));
                    let _e2535 = ((_e2531 * _e2531) * (3f - (2f * _e2531)));
                    let _e2537 = (((_e2496 * _e2496) * (3f - (2f * _e2496))) * (1f - ((_e2511 * _e2511) * (3f - (2f * _e2511)))));
                    if (_e2537 != _e2537) {
                        phi_79_ = true;
                    } else {
                        phi_79_ = (_e2535 >= _e2537);
                    }
                    let _e2541 = phi_79_;
                    phi_80_ = select(_e2537, _e2535, _e2541);
                } else {
                    let _e2344 = ((1f - _e2336) + _e2341);
                    let _e2348 = (((atan2(_e2319, _e2318) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e2352 = ((_e2344 * 0.82f) - 0.045f);
                    if (_e2352 != _e2352) {
                        phi_76_ = true;
                    } else {
                        phi_76_ = (0f >= _e2352);
                    }
                    let _e2356 = phi_76_;
                    let _e2357 = select(_e2352, 0f, _e2356);
                    let _e2365 = ((abs((sqrt(((_e2318 * _e2318) + (_e2319 * _e2319))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e2367 = select(_e2365, 0f, (_e2365 < 0f));
                    let _e2369 = select(_e2367, 1f, (_e2367 > 1f));
                    let _e2374 = (_e2357 + 0.008f);
                    let _e2378 = (((_e2348 - trunc(_e2348)) - _e2374) / ((_e2357 - 0.008f) - _e2374));
                    let _e2380 = select(_e2378, 0f, (_e2378 < 0f));
                    let _e2382 = select(_e2380, 1f, (_e2380 > 1f));
                    let _e2388 = (_e2344 * 50f);
                    let _e2390 = select(_e2388, 0f, (_e2388 < 0f));
                    let _e2392 = select(_e2390, 1f, (_e2390 > 1f));
                    let _e2397 = ((((_e2369 * _e2369) * (3f - (2f * _e2369))) * ((_e2382 * _e2382) * (3f - (2f * _e2382)))) * ((_e2392 * _e2392) * (3f - (2f * _e2392))));
                    let _e2399 = (0.50265485f + (5.152212f * _e2344));
                    let _e2400 = cos(_e2399);
                    let _e2401 = sin(_e2399);
                    let _e2405 = (_e2318 - (_e2400 * 7.1f));
                    let _e2406 = (_e2319 - (_e2401 * 7.1f));
                    let _e2409 = ((_e2405 * -(_e2401)) + (_e2406 * _e2400));
                    let _e2412 = ((_e2405 * _e2400) + (_e2406 * _e2401));
                    let _e2413 = (_e2409 * -3.2f);
                    let _e2416 = ((_e2413 + (_e2412 * 2.1f)) * 0.06825939f);
                    let _e2418 = select(_e2416, 0f, (_e2416 < 0f));
                    let _e2420 = select(_e2418, 1f, (_e2418 > 1f));
                    let _e2423 = (_e2409 - (-3.2f * _e2420));
                    let _e2424 = (_e2412 - (2.1f * _e2420));
                    let _e2428 = sqrt(((_e2423 * _e2423) + (_e2424 * _e2424)));
                    let _e2431 = ((_e2413 + (_e2412 * -2.1f)) * 0.06825939f);
                    let _e2433 = select(_e2431, 0f, (_e2431 < 0f));
                    let _e2435 = select(_e2433, 1f, (_e2433 > 1f));
                    let _e2438 = (_e2409 - (-3.2f * _e2435));
                    let _e2439 = (_e2412 - (-2.1f * _e2435));
                    let _e2443 = sqrt(((_e2438 * _e2438) + (_e2439 * _e2439)));
                    if (_e2428 != _e2428) {
                        phi_77_ = true;
                    } else {
                        phi_77_ = (_e2443 <= _e2428);
                    }
                    let _e2447 = phi_77_;
                    let _e2450 = ((select(_e2428, _e2443, _e2447) - 1.7f) * -0.71428573f);
                    let _e2452 = select(_e2450, 0f, (_e2450 < 0f));
                    let _e2454 = select(_e2452, 1f, (_e2452 > 1f));
                    let _e2458 = ((_e2454 * _e2454) * (3f - (2f * _e2454)));
                    if (_e2397 != _e2397) {
                        phi_78_ = true;
                    } else {
                        phi_78_ = (_e2458 >= _e2397);
                    }
                    let _e2462 = phi_78_;
                    phi_80_ = select(_e2397, _e2458, _e2462);
                }
                let _e2544 = phi_80_;
                let _e2547 = (_e2336 * (0.5f + (_e2341 * 0.5f)));
                if (_e2315 != _e2315) {
                    phi_81_ = true;
                } else {
                    phi_81_ = (_e2547 >= _e2315);
                }
                let _e2551 = phi_81_;
                let _e2552 = select(_e2315, _e2547, _e2551);
                let _e2554 = (0.48f * (1f - _e2552));
                let _e2565 = (1f + (_e2341 * 0.45f));
                phi_82_ = vec3<f32>((_e1079 + (((_e2554 + (0.78f * _e2552)) * _e2544) * _e2565)), (_e1080 + (((_e2554 + (0.3f * _e2552)) * _e2544) * _e2565)), (_e1081 + (((_e2554 + (0.28f * _e2552)) * _e2544) * _e2565)));
            } else {
                phi_82_ = _e2294;
            }
            let _e2574 = phi_82_;
            let _e2576 = local_34;
            let _e2578 = (1f - (_e2576 * 0.35f));
            let _e2586 = local_35;
            let _e2587 = (_e2586 * 0.33249998f);
            switch bitcast<i32>(_e745) {
                case 0: {
                    let _e2601 = pill_1.member[_e231].labels[0u];
                    phi_83_ = _e2601;
                    break;
                }
                case 1: {
                    let _e2596 = pill_1.member[_e231].labels[1u];
                    phi_83_ = _e2596;
                    break;
                }
                default: {
                    phi_83_ = render_text_Line(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), 0f, 0f, 0u, 0u, 0u);
                    break;
                }
            }
            let _e2603 = phi_83_;
            switch bitcast<i32>(_e745) {
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
            let _e2606 = phi_84_;
            if _e2606 {
                if (_e714 < _e2603.min.x) {
                    phi_112_ = f32();
                    phi_113_ = true;
                } else {
                    if (_e714 > _e2603.max.x) {
                        phi_110_ = f32();
                        phi_111_ = true;
                    } else {
                        if (_e715 < _e2603.min.y) {
                            phi_108_ = f32();
                            phi_109_ = true;
                        } else {
                            let _e2618 = (_e715 > _e2603.max.y);
                            if _e2618 {
                                phi_107_ = f32();
                            } else {
                                let _e2620 = (1f / _e2603.size);
                                let _e2627 = ((_e714 - _e2603.origin.x) * _e2620);
                                phi_85_ = 0u;
                                phi_86_ = _e2603.count;
                                loop {
                                    let _e2632 = phi_85_;
                                    let _e2634 = phi_86_;
                                    local_36 = _e2632;
                                    let _e2635 = (_e2632 < _e2634);
                                    if _e2635 {
                                        let _e2638 = (_e2632 + ((_e2634 - _e2632) / 2u));
                                        let _e2643 = placed_glyphs_1.member[(_e2603.first + _e2638)].x;
                                        let _e2644 = (_e2643 <= _e2627);
                                        if _e2644 {
                                            phi_87_ = (_e2638 + 1u);
                                        } else {
                                            phi_87_ = _e2632;
                                        }
                                        let _e2647 = phi_87_;
                                        phi_88_ = _e2647;
                                        phi_89_ = select(_e2638, _e2634, _e2644);
                                    } else {
                                        phi_88_ = u32();
                                        phi_89_ = u32();
                                    }
                                    let _e2650 = phi_88_;
                                    let _e2652 = phi_89_;
                                    continue;
                                    continuing {
                                        phi_85_ = _e2650;
                                        phi_86_ = _e2652;
                                        break if !(_e2635);
                                    }
                                }
                                let _e2654 = (3.5f / _e2603.size);
                                let _e2656 = local_36;
                                let _e2657 = (_e2656 + 1u);
                                phi_90_ = select(_e2657, _e2603.count, (_e2603.count < _e2657));
                                phi_91_ = -1000000f;
                                loop {
                                    let _e2661 = phi_90_;
                                    let _e2663 = phi_91_;
                                    local_39 = _e2663;
                                    if (_e2661 > 0u) {
                                        let _e2665 = (_e2661 - 1u);
                                        let _e2666 = (_e2603.first + _e2665);
                                        let _e2670 = placed_glyphs_1.member[_e2666].x;
                                        let _e2674 = placed_glyphs_1.member[_e2666].glyph;
                                        let _e2679 = glyphs_1.member[_e2674].min[0u];
                                        let _e2684 = glyphs_1.member[_e2674].min[1u];
                                        let _e2689 = glyphs_1.member[_e2674].max[0u];
                                        let _e2694 = glyphs_1.member[_e2674].max[1u];
                                        let _e2698 = glyphs_1.member[_e2674].start;
                                        let _e2702 = glyphs_1.member[_e2674].count;
                                        let _e2703 = (_e2627 - _e2670);
                                        let _e2704 = -(((_e715 - _e2603.origin.y) * _e2620));
                                        let _e2705 = (_e2689 + _e2654);
                                        let _e2706 = (_e2703 > _e2705);
                                        if _e2706 {
                                            phi_103_ = f32();
                                        } else {
                                            if (_e2703 >= (_e2679 - _e2654)) {
                                                if (_e2704 >= (_e2684 - _e2654)) {
                                                    if (_e2703 <= _e2705) {
                                                        if (_e2704 <= (_e2694 + _e2654)) {
                                                            phi_92_ = 340282350000000000000000000000000000000f;
                                                            phi_93_ = 0u;
                                                            phi_94_ = 0i;
                                                            loop {
                                                                let _e2716 = phi_92_;
                                                                let _e2718 = phi_93_;
                                                                let _e2720 = phi_94_;
                                                                local_37 = _e2716;
                                                                local_38 = _e2720;
                                                                let _e2721 = (_e2718 < _e2702);
                                                                if _e2721 {
                                                                    let _e2725 = edges_1.member[(_e2698 + _e2718)];
                                                                    let _e2727 = cantus_render_text_edge_distance(_e2725, _e2603.weight, vec2<f32>(_e2703, _e2704), _e2716);
                                                                    phi_95_ = _e2727.member;
                                                                    phi_96_ = (_e2718 + 1u);
                                                                    phi_97_ = (_e2720 + _e2727.member_1);
                                                                } else {
                                                                    phi_95_ = f32();
                                                                    phi_96_ = u32();
                                                                    phi_97_ = i32();
                                                                }
                                                                let _e2733 = phi_95_;
                                                                let _e2735 = phi_96_;
                                                                let _e2737 = phi_97_;
                                                                continue;
                                                                continuing {
                                                                    phi_92_ = _e2733;
                                                                    phi_93_ = _e2735;
                                                                    phi_94_ = _e2737;
                                                                    break if !(_e2721);
                                                                }
                                                            }
                                                            let _e2740 = local_37;
                                                            let _e2744 = local_38;
                                                            let _e2747 = ((sqrt(_e2740) * _e2603.size) * select(1f, -1f, (_e2744 == 0i)));
                                                            if (_e2663 != _e2663) {
                                                                phi_98_ = true;
                                                            } else {
                                                                phi_98_ = (_e2747 >= _e2663);
                                                            }
                                                            let _e2751 = phi_98_;
                                                            phi_99_ = select(_e2663, _e2747, _e2751);
                                                        } else {
                                                            phi_99_ = _e2663;
                                                        }
                                                        let _e2754 = phi_99_;
                                                        phi_100_ = _e2754;
                                                    } else {
                                                        phi_100_ = _e2663;
                                                    }
                                                    let _e2756 = phi_100_;
                                                    phi_101_ = _e2756;
                                                } else {
                                                    phi_101_ = _e2663;
                                                }
                                                let _e2758 = phi_101_;
                                                phi_102_ = _e2758;
                                            } else {
                                                phi_102_ = _e2663;
                                            }
                                            let _e2760 = phi_102_;
                                            phi_103_ = _e2760;
                                        }
                                        let _e2762 = phi_103_;
                                        phi_104_ = _e2665;
                                        phi_105_ = _e2762;
                                        phi_106_ = select(true, false, _e2706);
                                    } else {
                                        phi_104_ = u32();
                                        phi_105_ = f32();
                                        phi_106_ = false;
                                    }
                                    let _e2765 = phi_104_;
                                    let _e2767 = phi_105_;
                                    let _e2769 = phi_106_;
                                    continue;
                                    continuing {
                                        phi_90_ = _e2765;
                                        phi_91_ = _e2767;
                                        break if !(_e2769);
                                    }
                                }
                                let _e2772 = local_39;
                                let _e2774 = ((_e2772 * 1.25f) + 0.5f);
                                let _e2776 = select(_e2774, 0f, (_e2774 < 0f));
                                let _e2778 = select(_e2776, 1f, (_e2776 > 1f));
                                phi_107_ = ((_e2778 * _e2778) * (3f - (2f * _e2778)));
                            }
                            let _e2784 = phi_107_;
                            phi_108_ = _e2784;
                            phi_109_ = _e2618;
                        }
                        let _e2786 = phi_108_;
                        let _e2788 = phi_109_;
                        phi_110_ = _e2786;
                        phi_111_ = _e2788;
                    }
                    let _e2790 = phi_110_;
                    let _e2792 = phi_111_;
                    phi_112_ = _e2790;
                    phi_113_ = _e2792;
                }
                let _e2794 = phi_112_;
                let _e2796 = phi_113_;
                phi_114_ = select(_e2794, 0f, _e2796);
            } else {
                phi_114_ = 0f;
            }
            let _e2799 = phi_114_;
            let _e2800 = (1f - _e2799);
            let _e2804 = (0.94f * _e2799);
            out_color = vec4<f32>((((((_e2574.x * _e2578) + _e2587) * _e2800) + _e2804) * _e494), (((((_e2574.y * _e2578) + _e2587) * _e2800) + _e2804) * _e494), (((((_e2574.z * _e2578) + _e2587) * _e2800) + _e2804) * _e494), _e507);
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
    var local_56: i32;
    var phi_109_: bool;
    var phi_110_: f32;
    var phi_111_: f32;
    var phi_112_: f32;
    var phi_113_: f32;
    var phi_114_: f32;
    var phi_115_: u32;
    var phi_116_: f32;
    var phi_117_: bool;
    var local_57: f32;
    var phi_118_: f32;
    var phi_119_: f32;
    var phi_120_: bool;
    var phi_121_: f32;
    var phi_122_: bool;
    var phi_123_: f32;
    var phi_124_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e217 = pixel_3;
            let _e218 = weather_1;
            let _e219 = _isthmus_instance_index_11;
            let _e230 = pill_2.member[_e219].x;
            let _e234 = frame.member[0u].panel_height;
            let _e235 = (_e217.x - _e230);
            let _e236 = (_e217.y - 6f);
            let _e237 = (_e234 * 0.5f);
            let _e241 = ((308f - _e234) * 0.5f);
            let _e243 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e235 - 154f), (_e236 - _e237)), _e241, _e237);
            let _e247 = frame.member[0u].mouse_pressure;
            let _e248 = (_e247 > 0f);
            if _e248 {
                let _e253 = frame.member[0u].mouse_pos[0u];
                let _e258 = frame.member[0u].mouse_pos[1u];
                let _e264 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e253 - _e230) - 154f), ((_e258 - 6f) - _e237)), _e241, _e237);
                phi_0_ = _e264;
            } else {
                phi_0_ = 1f;
            }
            let _e266 = phi_0_;
            phi_1_ = 0u;
            loop {
                let _e268 = phi_1_;
                let _e269 = (_e268 < 4u);
                if _e269 {
                    if _e269 {
                    } else {
                        phi_3_ = true;
                        break;
                    }
                    phi_2_ = (_e268 + 1u);
                } else {
                    phi_2_ = u32();
                }
                let _e272 = phi_2_;
                continue;
                continuing {
                    phi_1_ = _e272;
                    phi_3_ = false;
                    break if !(_e269);
                }
            }
            let _e275 = phi_3_;
            if _e275 {
                break;
            }
            let _e281 = (_e230 - (_e218.w * 158f));
            let _e282 = (6f + _e234);
            let _e283 = (8f * _e218.w);
            let _e284 = ((244f * _e218.w) - _e283);
            if (_e284 != _e284) {
                phi_4_ = true;
            } else {
                phi_4_ = (0f >= _e284);
            }
            let _e288 = phi_4_;
            let _e291 = (_e217.y - _e282);
            let _e292 = ((308f + (316f * _e218.w)) * 0.5f);
            let _e293 = (select(_e284, 0f, _e288) * 0.5f);
            let _e294 = (_e283 + _e293);
            let _e297 = (_e293 != _e293);
            if _e297 {
                phi_5_ = true;
            } else {
                phi_5_ = (18f <= _e293);
            }
            let _e300 = phi_5_;
            let _e303 = vec2<f32>(_e292, _e293);
            let _e304 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e217.x - _e281) - _e292), (_e291 - _e294)), _e303, select(_e293, 18f, _e300));
            let _e309 = frame.member[0u].mouse_pos[0u];
            let _e314 = frame.member[0u].mouse_pos[1u];
            if _e297 {
                phi_6_ = true;
            } else {
                phi_6_ = (18f <= _e293);
            }
            let _e321 = phi_6_;
            let _e324 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e309 - _e281) - _e292), ((_e314 - _e282) - _e294)), _e303, select(_e293, 18f, _e321));
            let _e327 = (0.5f + ((_e304 - _e243) * 0.008928572f));
            let _e329 = select(_e327, 0f, (_e327 < 0f));
            let _e331 = select(_e329, 1f, (_e329 > 1f));
            let _e344 = (0.5f + ((_e324 - _e266) * 0.008928572f));
            let _e346 = select(_e344, 0f, (_e344 < 0f));
            let _e348 = select(_e346, 1f, (_e346 > 1f));
            phi_7_ = vec2<f32>(0f, 0f);
            phi_8_ = 0f;
            phi_9_ = 0u;
            loop {
                let _e360 = phi_7_;
                let _e362 = phi_8_;
                let _e364 = phi_9_;
                local_41 = _e360;
                local_42 = _e360;
                local_43 = _e360;
                local_44 = _e360;
                local_50 = _e362;
                local_51 = _e362;
                local_52 = _e362;
                local_53 = _e362;
                let _e365 = (_e364 < 4u);
                if _e365 {
                    if _e365 {
                    } else {
                        phi_19_ = true;
                        break;
                    }
                    let _e372 = frame.member[0u].ripples[_e364].origin[0u];
                    let _e379 = frame.member[0u].ripples[_e364].origin[1u];
                    let _e385 = frame.member[0u].ripples[_e364].start_time;
                    let _e391 = frame.member[0u].ripples[_e364].strength;
                    let _e395 = frame.member[0u].time;
                    let _e397 = ((_e395 - _e385) * 1.2f);
                    let _e399 = select(_e397, 0f, (_e397 < 0f));
                    let _e401 = select(_e399, 1f, (_e399 > 1f));
                    if (_e391 > 0f) {
                        if (_e401 < 1f) {
                            let _e404 = (_e217.x - _e372);
                            let _e405 = (_e217.y - _e379);
                            let _e409 = sqrt(((_e404 * _e404) + (_e405 * _e405)));
                            if (_e409 > 0.001f) {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e404 / _e409), (_e405 / _e409)), _e409);
                            } else {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e409);
                            }
                            let _e417 = phi_10_;
                            let _e427 = ((abs((_e417.unnamed_1 - (_e401 * 600f))) - 80f) * -0.0125f);
                            let _e429 = select(_e427, 0f, (_e427 < 0f));
                            let _e431 = select(_e429, 1f, (_e429 > 1f));
                            let _e437 = (1f - _e401);
                            let _e438 = ((((_e431 * _e431) * (3f - (2f * _e431))) * _e391) * _e437);
                            let _e451 = (_e362 + (_e438 * 0.5f));
                            if (_e451 != _e451) {
                                phi_11_ = true;
                            } else {
                                phi_11_ = (1f <= _e451);
                            }
                            let _e455 = phi_11_;
                            phi_12_ = vec2<f32>((_e360.x + (((_e417.unnamed.x * _e438) * _e437) * 0.5f)), (_e360.y + (((_e417.unnamed.y * _e438) * _e437) * 0.5f)));
                            phi_13_ = select(_e451, 1f, _e455);
                        } else {
                            phi_12_ = _e360;
                            phi_13_ = _e362;
                        }
                        let _e458 = phi_12_;
                        let _e460 = phi_13_;
                        phi_14_ = _e458;
                        phi_15_ = _e460;
                    } else {
                        phi_14_ = _e360;
                        phi_15_ = _e362;
                    }
                    let _e462 = phi_14_;
                    let _e464 = phi_15_;
                    phi_16_ = _e462;
                    phi_17_ = _e464;
                    phi_18_ = (_e364 + 1u);
                } else {
                    phi_16_ = vec2<f32>();
                    phi_17_ = f32();
                    phi_18_ = u32();
                }
                let _e467 = phi_16_;
                let _e469 = phi_17_;
                let _e471 = phi_18_;
                continue;
                continuing {
                    phi_7_ = _e467;
                    phi_8_ = _e469;
                    phi_9_ = _e471;
                    phi_19_ = _e275;
                    break if !(_e365);
                }
            }
            let _e474 = phi_19_;
            if _e474 {
                break;
            }
            if _e248 {
                let _e475 = (_e217.x - _e309);
                let _e476 = (_e217.y - _e314);
                let _e482 = ((sqrt(((_e475 * _e475) + (_e476 * _e476))) - 150f) * -0.006666667f);
                let _e484 = select(_e482, 0f, (_e482 < 0f));
                let _e486 = select(_e484, 1f, (_e484 > 1f));
                phi_20_ = ((((_e486 * _e486) * (3f - (2f * _e486))) * _e247) * 8f);
            } else {
                phi_20_ = 0f;
            }
            let _e494 = phi_20_;
            let _e496 = local_41;
            let _e498 = global[0u];
            if (_e496.x == _e498) {
                let _e501 = local_42;
                let _e504 = global[1u];
                phi_21_ = (_e501.y == _e504);
            } else {
                phi_21_ = false;
            }
            let _e507 = phi_21_;
            if _e507 {
                phi_22_ = 0f;
            } else {
                let _e509 = local_43;
                phi_22_ = (sqrt(((_e496.x * _e496.x) + (_e509.y * _e509.y))) * 22f);
            }
            let _e517 = phi_22_;
            let _e519 = local_44;
            let _e522 = (((_e266 + ((((_e324 + ((_e266 - _e324) * _e348)) - ((56f * _e348) * (1f - _e348))) - _e266) * _e218.w)) - 0.5f) * -1f);
            let _e524 = select(_e522, 0f, (_e522 < 0f));
            let _e526 = select(_e524, 1f, (_e524 > 1f));
            let _e534 = ((_e243 + ((((_e304 + ((_e243 - _e304) * _e331)) - ((56f * _e331) * (1f - _e331))) - _e243) * _e218.w)) - (((_e494 * ((_e526 * _e526) * (3f - (2f * _e526)))) + _e517) * 0.5f));
            let _e536 = (_e234 + 60f);
            let _e537 = ((_e236 - _e234) > _e536);
            let _e542 = pill_2.member[_e219].calendar_expansion;
            let _e543 = (56f + _e237);
            let _e544 = (_e234 + 8f);
            let _e546 = (_e543 + (select(0f, 1f, _e537) * _e544));
            let _e547 = (_e546 * 0.0007377049f);
            let _e548 = (0.5f + _e547);
            let _e552 = ((_e542 - _e548) / ((_e547 + 0.74f) - _e548));
            let _e554 = select(_e552, 0f, (_e552 < 0f));
            let _e556 = select(_e554, 1f, (_e554 > 1f));
            let _e560 = ((_e556 * _e556) * (3f - (2f * _e556)));
            let _e562 = (292f * _e560);
            let _e563 = (_e234 * _e560);
            let _e571 = ((_e230 + 166f) + ((292f - _e562) * 0.5f));
            let _e572 = ((_e282 + (_e546 - _e237)) + ((_e234 - _e563) * 0.5f));
            let _e573 = (_e217.x - _e571);
            let _e574 = (_e217.y - _e572);
            let _e575 = select(6u, 5u, _e537);
            if (_e562 != _e562) {
                phi_23_ = true;
            } else {
                phi_23_ = (0.001f >= _e562);
            }
            let _e579 = phi_23_;
            let _e584 = (((_e573 / select(_e562, 0.001f, _e579)) * f32(_e575)) - 0.5f);
            let _e586 = f32((_e575 - 1u));
            if (0f <= _e586) {
            } else {
                break;
            }
            let _e589 = select(_e584, 0f, (_e584 < 0f));
            let _e591 = select(_e589, _e586, (_e589 > _e586));
            let _e592 = floor(_e591);
            let _e597 = select(select(u32(_e592), 0u, (_e592 < 0f)), 4294967295u, (_e592 > 4294967000f));
            let _e599 = (_e591 - trunc(_e591));
            let _e601 = select(_e599, 0f, (_e599 < 0f));
            let _e603 = select(_e601, 1f, (_e601 > 1f));
            let _e607 = ((_e603 * _e603) * (3f - (2f * _e603)));
            if _e537 {
                if (_e597 < 5u) {
                } else {
                    break;
                }
                let _e635 = pill_2.member[_e219].daily_conditions[_e597];
                let _e636 = (_e597 + 1u);
                let _e638 = select(_e636, 4u, (4u < _e636));
                if (_e638 < 5u) {
                } else {
                    break;
                }
                let _e644 = pill_2.member[_e219].daily_conditions[_e638];
                phi_24_ = 12f;
                phi_25_ = _e644;
                phi_26_ = _e635;
            } else {
                if (_e597 < 6u) {
                } else {
                    break;
                }
                let _e613 = pill_2.member[_e219].hourly_conditions[_e597];
                let _e614 = (_e597 + 1u);
                let _e616 = select(_e614, 5u, (5u < _e614));
                if (_e616 < 6u) {
                } else {
                    break;
                }
                let _e622 = pill_2.member[_e219].hourly_conditions[_e616];
                let _e626 = pill_2.member[_e219].hourly_start;
                phi_24_ = ((_e626 + (_e591 * 4f)) % 24f);
                phi_25_ = _e622;
                phi_26_ = _e613;
            }
            let _e646 = phi_24_;
            let _e648 = phi_25_;
            let _e650 = phi_26_;
            let _e651 = (_e560 <= 0.001f);
            if _e651 {
                phi_27_ = 340282350000000000000000000000000000000f;
            } else {
                let _e653 = (_e563 * 0.5f);
                let _e659 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e573 - (_e560 * 146f)), (_e574 - _e653)), ((_e562 - _e563) * 0.5f), _e653);
                phi_27_ = _e659;
            }
            let _e661 = phi_27_;
            if _e651 {
                phi_28_ = 340282350000000000000000000000000000000f;
            } else {
                let _e665 = (_e563 * 0.5f);
                let _e671 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e309 - _e571) - (_e560 * 146f)), ((_e314 - _e572) - _e665)), ((_e562 - _e563) * 0.5f), _e665);
                phi_28_ = _e671;
            }
            let _e673 = phi_28_;
            let _e707 = pill_2.member[_e219].sun_hours;
            let _e710 = (_e707[1] - _e707[0]);
            if (_e646 >= _e707[0]) {
                let _e712 = (_e646 <= _e707[1]);
                if _e712 {
                    let _e714 = ((_e646 - _e707[0]) / _e710);
                    phi_29_ = array<f32, 2>(_e714, sin((_e714 * 3.1415927f)));
                } else {
                    phi_29_ = array<f32, 2>();
                }
                let _e719 = phi_29_;
                phi_30_ = _e719;
                phi_31_ = select(true, false, _e712);
            } else {
                phi_30_ = array<f32, 2>();
                phi_31_ = true;
            }
            let _e722 = phi_30_;
            let _e724 = phi_31_;
            if _e724 {
                let _e725 = (24f - _e710);
                if (_e646 < _e707[0]) {
                    phi_32_ = (((_e646 + 24f) - _e707[1]) / _e725);
                } else {
                    phi_32_ = ((_e646 - _e707[1]) / _e725);
                }
                let _e733 = phi_32_;
                phi_33_ = array<f32, 2>(select(0f, 1f, (_e646 >= _e707[1])), -(sin((_e733 * 3.1415927f))));
            } else {
                phi_33_ = _e722;
            }
            let _e741 = phi_33_;
            let _e744 = ((_e673 - 0.5f) * -1f);
            let _e746 = select(_e744, 0f, (_e744 < 0f));
            let _e748 = select(_e746, 1f, (_e746 > 1f));
            let _e756 = (_e661 - (((_e494 * ((_e748 * _e748) * (3f - (2f * _e748)))) + _e517) * 0.5f));
            let _e757 = (_e534 != _e534);
            if _e757 {
                phi_34_ = true;
            } else {
                phi_34_ = (_e756 <= _e534);
            }
            let _e760 = phi_34_;
            let _e761 = select(_e534, _e756, _e760);
            let _e762 = fwidth(_e761);
            if (_e762 != _e762) {
                phi_35_ = true;
            } else {
                phi_35_ = (0.55f >= _e762);
            }
            let _e766 = phi_35_;
            let _e767 = select(_e762, 0.55f, _e766);
            let _e771 = ((_e761 - _e767) / (-(_e767) - _e767));
            let _e773 = select(_e771, 0f, (_e771 < 0f));
            let _e775 = select(_e773, 1f, (_e773 > 1f));
            let _e779 = ((_e775 * _e775) * (3f - (2f * _e775)));
            if (_e761 != _e761) {
                phi_36_ = true;
            } else {
                phi_36_ = (0f >= _e761);
            }
            let _e783 = phi_36_;
            let _e787 = (exp((select(_e761, 0f, _e783) * -0.3f)) * 0.16f);
            if (_e779 != _e779) {
                phi_37_ = true;
            } else {
                phi_37_ = (_e787 >= _e779);
            }
            let _e791 = phi_37_;
            let _e792 = select(_e779, _e787, _e791);
            if (_e792 <= 0.0009765625f) {
                discard;
            }
            let _e798 = pill_2.member[_e219].hourly_conditions[0u];
            let _e799 = (_e235 * 0.0032467532f);
            let _e801 = select(_e799, 0f, (_e799 < 0f));
            let _e810 = pill_2.member[_e219].hourly_conditions[1u];
            let _e812 = ((abs((select(_e801, 1f, (_e801 > 1f)) - 0.5f)) - 0.2f) * 20.000002f);
            let _e814 = select(_e812, 0f, (_e812 < 0f));
            let _e816 = select(_e814, 1f, (_e814 > 1f));
            let _e820 = ((_e816 * _e816) * (3f - (2f * _e816)));
            let _e825 = (_e798.fog + ((_e810.fog - _e798.fog) * _e820));
            let _e830 = (_e798.cloud + ((_e810.cloud - _e798.cloud) * _e820));
            let _e835 = (_e798.rain + ((_e810.rain - _e798.rain) * _e820));
            let _e840 = (_e798.snow + ((_e810.snow - _e798.snow) * _e820));
            let _e845 = (_e798.lightning + ((_e810.lightning - _e798.lightning) * _e820));
            let _e850 = (_e798.hail + ((_e810.hail - _e798.hail) * _e820));
            let _e853 = (_e825 + ((_e798.fog - _e825) * _e218.w));
            let _e856 = (_e830 + ((_e798.cloud - _e830) * _e218.w));
            let _e859 = (_e835 + ((_e798.rain - _e835) * _e218.w));
            let _e862 = (_e840 + ((_e798.snow - _e840) * _e218.w));
            let _e865 = (_e845 + ((_e798.lightning - _e845) * _e218.w));
            let _e868 = (_e850 + ((_e798.hail - _e850) * _e218.w));
            let _e869 = (_e236 / _e234);
            if _e757 {
                phi_38_ = true;
            } else {
                phi_38_ = (0f <= _e534);
            }
            let _e874 = phi_38_;
            let _e877 = (1f + (select(_e534, 0f, _e874) * 0.008333334f));
            let _e879 = select(_e877, 0f, (_e877 < 0f));
            let _e881 = select(_e879, 0.6f, (_e879 > 0.6f));
            let _e888 = (_e496.x * 0.04f);
            let _e889 = (_e519.y * 0.04f);
            let _e890 = ((_e799 - (((_e799 - 0.5f) * _e881) * 0.08f)) - _e888);
            let _e891 = ((_e869 - (((_e869 - 0.5f) * _e881) * 0.08f)) - _e889);
            if (_e560 > 0.001f) {
                let _e894 = (_e573 / _e562);
                let _e895 = (_e574 / _e563);
                if (_e756 != _e756) {
                    phi_39_ = true;
                } else {
                    phi_39_ = (0f <= _e756);
                }
                let _e901 = phi_39_;
                let _e904 = (1f + (select(_e756, 0f, _e901) * 0.008333334f));
                let _e906 = select(_e904, 0f, (_e904 < 0f));
                let _e908 = select(_e906, 0.6f, (_e906 > 0.6f));
                phi_40_ = vec2<f32>(((_e894 - (((_e894 - 0.5f) * _e908) * 0.08f)) - _e888), ((_e895 - (((_e895 - 0.5f) * _e908) * 0.08f)) - _e889));
            } else {
                phi_40_ = vec2<f32>(_e890, _e891);
            }
            let _e919 = phi_40_;
            let _e920 = fwidth(_e756);
            if (_e920 != _e920) {
                phi_41_ = true;
            } else {
                phi_41_ = (0.55f >= _e920);
            }
            let _e924 = phi_41_;
            let _e925 = select(_e920, 0.55f, _e924);
            let _e929 = ((_e756 - _e925) / (-(_e925) - _e925));
            let _e931 = select(_e929, 0f, (_e929 < 0f));
            let _e933 = select(_e931, 1f, (_e931 > 1f));
            let _e938 = (((_e933 * _e933) * (3f - (2f * _e933))) * _e560);
            let _e945 = (1f - _e938);
            let _e950 = (((_e890 * 308f) * _e945) + ((_e919.x * _e562) * _e938));
            let _e951 = (((_e891 * _e234) * _e945) + ((_e919.y * _e563) * _e938));
            if (_e756 != _e756) {
                phi_42_ = true;
            } else {
                phi_42_ = (1000f <= _e756);
            }
            let _e958 = phi_42_;
            let _e965 = (_e853 + (((_e650.fog + ((_e648.fog - _e650.fog) * _e607)) - _e853) * _e938));
            let _e968 = (_e856 + (((_e650.cloud + ((_e648.cloud - _e650.cloud) * _e607)) - _e856) * _e938));
            let _e971 = (_e859 + (((_e650.rain + ((_e648.rain - _e650.rain) * _e607)) - _e859) * _e938));
            let _e974 = (_e862 + (((_e650.snow + ((_e648.snow - _e650.snow) * _e607)) - _e862) * _e938));
            let _e980 = (_e868 + (((_e650.hail + ((_e648.hail - _e650.hail) * _e607)) - _e868) * _e938));
            let _e982 = ((_e218.y - -0.04f) * 4.1666665f);
            let _e984 = select(_e982, 0f, (_e982 < 0f));
            let _e986 = select(_e984, 1f, (_e984 > 1f));
            let _e990 = ((_e986 * _e986) * (3f - (2f * _e986)));
            let _e992 = ((_e218.y - -0.32f) * 4.166667f);
            let _e994 = select(_e992, 0f, (_e992 < 0f));
            let _e996 = select(_e994, 1f, (_e994 > 1f));
            let _e1004 = ((_e218.y - -0.18f) * 5.5555553f);
            let _e1006 = select(_e1004, 0f, (_e1004 < 0f));
            let _e1008 = select(_e1006, 1f, (_e1006 > 1f));
            let _e1014 = ((_e218.y - 0.2f) * -5.5555553f);
            let _e1016 = select(_e1014, 0f, (_e1014 < 0f));
            let _e1018 = select(_e1016, 1f, (_e1016 > 1f));
            let _e1025 = ((_e741[1] - -0.04f) * 4.1666665f);
            let _e1027 = select(_e1025, 0f, (_e1025 < 0f));
            let _e1029 = select(_e1027, 1f, (_e1027 > 1f));
            let _e1033 = ((_e1029 * _e1029) * (3f - (2f * _e1029)));
            let _e1035 = ((_e741[1] - -0.32f) * 4.166667f);
            let _e1037 = select(_e1035, 0f, (_e1035 < 0f));
            let _e1039 = select(_e1037, 1f, (_e1037 > 1f));
            let _e1047 = ((_e741[1] - -0.18f) * 5.5555553f);
            let _e1049 = select(_e1047, 0f, (_e1047 < 0f));
            let _e1051 = select(_e1049, 1f, (_e1049 > 1f));
            let _e1057 = ((_e741[1] - 0.2f) * -5.5555553f);
            let _e1059 = select(_e1057, 0f, (_e1057 < 0f));
            let _e1061 = select(_e1059, 1f, (_e1059 > 1f));
            let _e1073 = ((_e990 * _e945) + (_e1033 * _e938));
            let _e1075 = (((((_e1008 * _e1008) * (3f - (2f * _e1008))) * ((_e1018 * _e1018) * (3f - (2f * _e1018)))) * _e945) + ((((_e1051 * _e1051) * (3f - (2f * _e1051))) * ((_e1061 * _e1061) * (3f - (2f * _e1061)))) * _e938));
            let _e1079 = frame.member[0u].time;
            let _e1080 = (_e951 / _e234);
            let _e1082 = ((_e1080 - 1f) * -1f);
            let _e1084 = select(_e1082, 0f, (_e1082 < 0f));
            let _e1086 = select(_e1084, 1f, (_e1084 > 1f));
            let _e1090 = ((_e1086 * _e1086) * (3f - (2f * _e1086)));
            let _e1091 = (1f - _e1090);
            let _e1110 = (1f - _e1073);
            let _e1122 = (0.3f * _e1091);
            let _e1123 = (0.22f * _e1090);
            let _e1129 = ((((((_e996 * _e996) * (3f - (2f * _e996))) * (1f - _e990)) * _e945) + ((((_e1039 * _e1039) * (3f - (2f * _e1039))) * (1f - _e1033)) * _e938)) * 0.8f);
            let _e1130 = (1f - _e1129);
            let _e1147 = (_e1075 * 0.9f);
            let _e1148 = (1f - _e1147);
            let _e1160 = floor((_e950 * 0.055555556f));
            let _e1161 = floor((_e951 * 0.055555556f));
            let _e1165 = cantus_render_shader_hash(vec2<f32>(_e1160, _e1161));
            let _e1174 = (_e950 - (((_e1160 + 0.2f) + (_e1165.x * 0.6f)) * 18f));
            let _e1175 = (_e951 - (((_e1161 + 0.2f) + (_e1165.y * 0.6f)) * 18f));
            let _e1181 = ((sqrt(((_e1174 * _e1174) + (_e1175 * _e1175))) - 1f) * -1.6666666f);
            let _e1183 = select(_e1181, 0f, (_e1181 < 0f));
            let _e1185 = select(_e1183, 1f, (_e1183 > 1f));
            let _e1193 = cantus_render_shader_hash(vec2<f32>((_e1160 + 31.7f), (_e1161 + 31.7f)));
            let _e1196 = ((_e1193.x - 0.75f) * 4f);
            let _e1198 = select(_e1196, 0f, (_e1196 < 0f));
            let _e1200 = select(_e1198, 1f, (_e1198 > 1f));
            let _e1211 = ((((((_e1185 * _e1185) * (3f - (2f * _e1185))) * ((_e1200 * _e1200) * (3f - (2f * _e1200)))) * _e1110) * (1f - _e968)) * (0.3f + (_e1090 * 0.7f)));
            let _e1212 = (((((((((0.006f * _e1091) + (0.025f * _e1090)) * _e1110) + (((0.08f * _e1091) + (0.32f * _e1090)) * _e1073)) * _e1130) + (((0.1f * _e1091) + _e1123) * _e1129)) * _e1148) + (((0.78f * _e1091) + (0.38f * _e1090)) * _e1147)) + _e1211);
            let _e1213 = (((((((((0.012f * _e1091) + (0.04f * _e1090)) * _e1110) + (((0.34f * _e1091) + (0.67f * _e1090)) * _e1073)) * _e1130) + (((0.16f * _e1091) + (0.25f * _e1090)) * _e1129)) * _e1148) + ((_e1122 + _e1123) * _e1147)) + _e1211);
            let _e1214 = (((((((((0.035f * _e1091) + (0.095f * _e1090)) * _e1110) + (((0.62f * _e1091) + (0.87f * _e1090)) * _e1073)) * _e1130) + ((_e1122 + (0.45f * _e1090)) * _e1129)) * _e1148) + (((0.2f * _e1091) + (0.42f * _e1090)) * _e1147)) + _e1211);
            if (_e968 > 0.0009765625f) {
                let _e1217 = (_e950 / _e234);
                phi_43_ = 0i;
                phi_44_ = 0.5f;
                phi_45_ = 0f;
                phi_46_ = vec2<f32>(((_e1217 * 0.14f) + (_e1079 * 0.012f)), ((_e1080 * 0.14f) + 6.1f));
                loop {
                    let _e1225 = phi_43_;
                    let _e1227 = phi_44_;
                    let _e1229 = phi_45_;
                    let _e1231 = phi_46_;
                    local_45 = _e1229;
                    let _e1232 = (_e1225 < 4i);
                    if _e1232 {
                        let _e1235 = cantus_render_shader_simplex_noise(_e1231);
                        phi_47_ = (_e1225 + 1i);
                        phi_48_ = (_e1227 * 0.5f);
                        phi_49_ = (_e1229 + (_e1235 * _e1227));
                        phi_50_ = vec2<f32>(((_e1231.x * 1.6f) + (_e1231.y * 1.2f)), ((_e1231.y * 1.6f) - (_e1231.x * 1.2f)));
                    } else {
                        phi_47_ = i32();
                        phi_48_ = f32();
                        phi_49_ = f32();
                        phi_50_ = vec2<f32>();
                    }
                    let _e1248 = phi_47_;
                    let _e1250 = phi_48_;
                    let _e1252 = phi_49_;
                    let _e1254 = phi_50_;
                    continue;
                    continuing {
                        phi_43_ = _e1248;
                        phi_44_ = _e1250;
                        phi_45_ = _e1252;
                        phi_46_ = _e1254;
                        break if !(_e1232);
                    }
                }
                let _e1257 = local_45;
                let _e1258 = (_e1257 * 0.5f);
                phi_51_ = 0i;
                phi_52_ = 0.5f;
                phi_53_ = 0f;
                phi_54_ = vec2<f32>(((_e1217 * 0.287f) + (_e1079 * 0.018f)), ((_e1080 * 0.287f) + -3.7f));
                loop {
                    let _e1267 = phi_51_;
                    let _e1269 = phi_52_;
                    let _e1271 = phi_53_;
                    let _e1273 = phi_54_;
                    local_46 = _e1271;
                    local_47 = _e1271;
                    let _e1274 = (_e1267 < 4i);
                    if _e1274 {
                        let _e1277 = cantus_render_shader_simplex_noise(_e1273);
                        phi_55_ = (_e1267 + 1i);
                        phi_56_ = (_e1269 * 0.5f);
                        phi_57_ = (_e1271 + (_e1277 * _e1269));
                        phi_58_ = vec2<f32>(((_e1273.x * 1.6f) + (_e1273.y * 1.2f)), ((_e1273.y * 1.6f) - (_e1273.x * 1.2f)));
                    } else {
                        phi_55_ = i32();
                        phi_56_ = f32();
                        phi_57_ = f32();
                        phi_58_ = vec2<f32>();
                    }
                    let _e1290 = phi_55_;
                    let _e1292 = phi_56_;
                    let _e1294 = phi_57_;
                    let _e1296 = phi_58_;
                    continue;
                    continuing {
                        phi_51_ = _e1290;
                        phi_52_ = _e1292;
                        phi_53_ = _e1294;
                        phi_54_ = _e1296;
                        break if !(_e1274);
                    }
                }
                let _e1299 = local_46;
                let _e1302 = local_47;
                let _e1306 = ((((0.5f + _e1258) + (_e1302 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1308 = select(_e1306, 0f, (_e1306 < 0f));
                let _e1310 = select(_e1308, 1f, (_e1308 > 1f));
                let _e1316 = (((_e1299 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1318 = select(_e1316, 0f, (_e1316 < 0f));
                let _e1320 = select(_e1318, 1f, (_e1318 > 1f));
                let _e1327 = ((_e1258 + 0.02000001f) * 4.5454545f);
                let _e1329 = select(_e1327, 0f, (_e1327 < 0f));
                let _e1331 = select(_e1329, 1f, (_e1329 > 1f));
                let _e1337 = ((((_e1320 * _e1320) * (3f - (2f * _e1320))) * 0.55f) + (((_e1331 * _e1331) * (3f - (2f * _e1331))) * 0.45f));
                let _e1338 = (1f - _e1337);
                let _e1375 = (_e1075 * 0.45f);
                let _e1376 = (1f - _e1375);
                let _e1388 = (_e968 * (0.12f + (((_e1310 * _e1310) * (3f - (2f * _e1310))) * 0.7f)));
                let _e1389 = (1f - _e1388);
                phi_59_ = vec3<f32>(((_e1212 * _e1389) + (((((((0.16f * _e1338) + (0.32f * _e1337)) * _e1110) + (((0.62f * _e1338) + (0.92f * _e1337)) * _e1073)) * _e1376) + (((0.5f * _e1338) + (0.76f * _e1337)) * _e1375)) * _e1388)), ((_e1213 * _e1389) + (((((((0.2f * _e1338) + (0.36f * _e1337)) * _e1110) + (((0.7f * _e1338) + (0.94f * _e1337)) * _e1073)) * _e1376) + (((0.36f * _e1338) + (0.59f * _e1337)) * _e1375)) * _e1388)), ((_e1214 * _e1389) + (((((((0.28f * _e1338) + (0.43f * _e1337)) * _e1110) + (((0.78f * _e1338) + (0.96f * _e1337)) * _e1073)) * _e1376) + (((0.4f * _e1338) + (0.56f * _e1337)) * _e1375)) * _e1388)));
            } else {
                phi_59_ = vec3<f32>(_e1212, _e1213, _e1214);
            }
            let _e1401 = phi_59_;
            let _e1403 = (1f - (_e971 * 0.2f));
            let _e1413 = ((_e1401.x * _e1403) + (_e971 * 0.020000001f));
            let _e1414 = ((_e1401.y * _e1403) + (_e971 * 0.034f));
            let _e1415 = ((_e1401.z * _e1403) + (_e971 * 0.05f));
            if (_e971 > 0.0009765625f) {
                let _e1420 = (_e950 - (20f * _e1079));
                let _e1421 = (_e951 - (110f * _e1079));
                let _e1424 = floor((_e1420 * 0.06666667f));
                let _e1425 = floor((_e1421 * 0.04f));
                let _e1427 = cantus_render_shader_hash(vec2<f32>(_e1424, _e1425));
                let _e1438 = (_e1420 - (((_e1424 + 0.15f) + (_e1427.x * 0.7f)) * 15f));
                let _e1439 = (_e1421 - (((_e1425 + 0.15f) + (_e1427.y * 0.7f)) * 25f));
                let _e1443 = (((_e1438 * 1.8000001f) + (_e1439 * 9f)) * 0.011870845f);
                let _e1445 = select(_e1443, 0f, (_e1443 < 0f));
                let _e1447 = select(_e1445, 1f, (_e1445 > 1f));
                let _e1450 = (_e1438 - (1.8000001f * _e1447));
                let _e1451 = (_e1439 - (9f * _e1447));
                let _e1457 = ((sqrt(((_e1450 * _e1450) + (_e1451 * _e1451))) - 1.0999999f) * -1.666667f);
                let _e1459 = select(_e1457, 0f, (_e1457 < 0f));
                let _e1461 = select(_e1459, 1f, (_e1459 > 1f));
                let _e1469 = cantus_render_shader_hash(vec2<f32>((_e1424 + 19.3f), (_e1425 + 19.3f)));
                let _e1472 = ((_e1469.x - 0.22000003f) * 1.2820513f);
                let _e1474 = select(_e1472, 0f, (_e1472 < 0f));
                let _e1476 = select(_e1474, 1f, (_e1474 > 1f));
                let _e1483 = (((((_e1461 * _e1461) * (3f - (2f * _e1461))) * ((_e1476 * _e1476) * (3f - (2f * _e1476)))) * _e971) * 0.7f);
                let _e1485 = select(_e1483, 0f, (_e1483 < 0f));
                let _e1487 = select(_e1485, 1f, (_e1485 > 1f));
                let _e1488 = (1f - _e1487);
                phi_60_ = vec3<f32>(((_e1413 * _e1488) + (0.52f * _e1487)), ((_e1414 * _e1488) + (0.72f * _e1487)), ((_e1415 * _e1488) + (0.9f * _e1487)));
            } else {
                phi_60_ = vec3<f32>(_e1413, _e1414, _e1415);
            }
            let _e1500 = phi_60_;
            if (_e974 > 0.0009765625f) {
                let _e1504 = (_e950 - (5f * _e1079));
                let _e1505 = (_e951 - (14f * _e1079));
                let _e1508 = floor((_e1504 * 0.05f));
                let _e1509 = floor((_e1505 * 0.05f));
                let _e1513 = cantus_render_shader_hash(vec2<f32>((_e1508 + 31.7f), (_e1509 + 31.7f)));
                let _e1524 = (_e1504 - (((_e1508 + 0.15f) + (_e1513.x * 0.7f)) * 20f));
                let _e1525 = (_e1505 - (((_e1509 + 0.15f) + (_e1513.y * 0.7f)) * 20f));
                let _e1529 = (((_e1524 * 0.080000006f) + (_e1525 * 0.4f)) * 6.009615f);
                let _e1531 = select(_e1529, 0f, (_e1529 < 0f));
                let _e1533 = select(_e1531, 1f, (_e1531 > 1f));
                let _e1536 = (_e1524 - (0.080000006f * _e1533));
                let _e1537 = (_e1525 - (0.4f * _e1533));
                let _e1543 = ((sqrt(((_e1536 * _e1536) + (_e1537 * _e1537))) - 1.5999999f) * -1.666667f);
                let _e1545 = select(_e1543, 0f, (_e1543 < 0f));
                let _e1547 = select(_e1545, 1f, (_e1545 > 1f));
                let _e1555 = cantus_render_shader_hash(vec2<f32>((_e1508 + 19.3f), (_e1509 + 19.3f)));
                let _e1558 = ((_e1555.x - 0.3f) * 1.4285715f);
                let _e1560 = select(_e1558, 0f, (_e1558 < 0f));
                let _e1562 = select(_e1560, 1f, (_e1560 > 1f));
                let _e1569 = (((((_e1547 * _e1547) * (3f - (2f * _e1547))) * ((_e1562 * _e1562) * (3f - (2f * _e1562)))) * _e974) * 0.92f);
                let _e1571 = select(_e1569, 0f, (_e1569 < 0f));
                let _e1573 = select(_e1571, 1f, (_e1571 > 1f));
                let _e1574 = (1f - _e1573);
                let _e1581 = (0.96f * _e1573);
                phi_61_ = vec3<f32>(((_e1500.x * _e1574) + _e1581), ((_e1500.y * _e1574) + _e1581), ((_e1500.z * _e1574) + _e1581));
            } else {
                phi_61_ = _e1500;
            }
            let _e1587 = phi_61_;
            if (_e980 > 0.0009765625f) {
                let _e1591 = (_e950 - (18f * _e1079));
                let _e1592 = (_e951 - (85f * _e1079));
                let _e1595 = floor((_e1591 * 0.04347826f));
                let _e1596 = floor((_e1592 * 0.04347826f));
                let _e1600 = cantus_render_shader_hash(vec2<f32>((_e1595 + 63.4f), (_e1596 + 63.4f)));
                let _e1611 = (_e1591 - (((_e1595 + 0.15f) + (_e1600.x * 0.7f)) * 23f));
                let _e1612 = (_e1592 - (((_e1596 + 0.15f) + (_e1600.y * 0.7f)) * 23f));
                let _e1616 = (((_e1611 * 0.24000001f) + (_e1612 * 1.2f)) * 0.667735f);
                let _e1618 = select(_e1616, 0f, (_e1616 < 0f));
                let _e1620 = select(_e1618, 1f, (_e1618 > 1f));
                let _e1623 = (_e1611 - (0.24000001f * _e1620));
                let _e1624 = (_e1612 - (1.2f * _e1620));
                let _e1630 = ((sqrt(((_e1623 * _e1623) + (_e1624 * _e1624))) - 0.79999995f) * -1.6666667f);
                let _e1632 = select(_e1630, 0f, (_e1630 < 0f));
                let _e1634 = select(_e1632, 1f, (_e1632 > 1f));
                let _e1642 = cantus_render_shader_hash(vec2<f32>((_e1595 + 19.3f), (_e1596 + 19.3f)));
                let _e1645 = ((_e1642.x - 0.7f) * 3.3333333f);
                let _e1647 = select(_e1645, 0f, (_e1645 < 0f));
                let _e1649 = select(_e1647, 1f, (_e1647 > 1f));
                let _e1656 = (((((_e1634 * _e1634) * (3f - (2f * _e1634))) * ((_e1649 * _e1649) * (3f - (2f * _e1649)))) * _e980) * 0.7f);
                let _e1658 = select(_e1656, 0f, (_e1656 < 0f));
                let _e1660 = select(_e1658, 1f, (_e1658 > 1f));
                let _e1661 = (1f - _e1660);
                phi_62_ = vec3<f32>(((_e1587.x * _e1661) + (0.75f * _e1660)), ((_e1587.y * _e1661) + (0.86f * _e1660)), ((_e1587.z * _e1661) + (0.94f * _e1660)));
            } else {
                phi_62_ = _e1587;
            }
            let _e1676 = phi_62_;
            let _e1680 = ((sin((_e1079 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1682 = select(_e1680, 0f, (_e1680 < 0f));
            let _e1684 = select(_e1682, 1f, (_e1682 > 1f));
            let _e1689 = (((_e1684 * _e1684) * (3f - (2f * _e1684))) * (_e865 + (((_e650.lightning + ((_e648.lightning - _e650.lightning) * _e607)) - _e865) * _e938)));
            let _e1691 = (1f - (_e1689 * 0.55f));
            let _e1701 = ((_e1676.x * _e1691) + (_e1689 * 0.3575f));
            let _e1702 = ((_e1676.y * _e1691) + (_e1689 * 0.407f));
            let _e1703 = ((_e1676.z * _e1691) + (_e1689 * 0.528f));
            if (_e965 > 0.0009765625f) {
                phi_63_ = 0i;
                phi_64_ = 0.5f;
                phi_65_ = 0f;
                phi_66_ = vec2<f32>((((_e950 / (308f + ((_e562 - 308f) * _e938))) * 0.9f) + (_e1079 * 0.008f)), ((_e1080 * 0.32f) + 12f));
                loop {
                    let _e1714 = phi_63_;
                    let _e1716 = phi_64_;
                    let _e1718 = phi_65_;
                    let _e1720 = phi_66_;
                    local_48 = _e1718;
                    let _e1721 = (_e1714 < 4i);
                    if _e1721 {
                        let _e1724 = cantus_render_shader_simplex_noise(_e1720);
                        phi_67_ = (_e1714 + 1i);
                        phi_68_ = (_e1716 * 0.5f);
                        phi_69_ = (_e1718 + (_e1724 * _e1716));
                        phi_70_ = vec2<f32>(((_e1720.x * 1.6f) + (_e1720.y * 1.2f)), ((_e1720.y * 1.6f) - (_e1720.x * 1.2f)));
                    } else {
                        phi_67_ = i32();
                        phi_68_ = f32();
                        phi_69_ = f32();
                        phi_70_ = vec2<f32>();
                    }
                    let _e1737 = phi_67_;
                    let _e1739 = phi_68_;
                    let _e1741 = phi_69_;
                    let _e1743 = phi_70_;
                    continue;
                    continuing {
                        phi_63_ = _e1737;
                        phi_64_ = _e1739;
                        phi_65_ = _e1741;
                        phi_66_ = _e1743;
                        break if !(_e1721);
                    }
                }
                let _e1746 = local_48;
                let _e1749 = (((_e1746 * 0.5f) + 0.15f) * 2.857143f);
                let _e1751 = select(_e1749, 0f, (_e1749 < 0f));
                let _e1753 = select(_e1751, 1f, (_e1751 > 1f));
                let _e1760 = (_e965 * (0.58f + (((_e1753 * _e1753) * (3f - (2f * _e1753))) * 0.18f)));
                let _e1761 = (1f - _e1760);
                phi_71_ = vec3<f32>(((_e1701 * _e1761) + (0.63f * _e1760)), ((_e1702 * _e1761) + (0.69f * _e1760)), ((_e1703 * _e1761) + (0.73f * _e1760)));
            } else {
                phi_71_ = vec3<f32>(_e1701, _e1702, _e1703);
            }
            let _e1773 = phi_71_;
            let _e1775 = ((_e1080 - 0.12f) * -8.333334f);
            let _e1777 = select(_e1775, 0f, (_e1775 < 0f));
            let _e1779 = select(_e1777, 1f, (_e1777 > 1f));
            let _e1786 = (((_e534 + ((select(_e756, 1000f, _e958) - _e534) * _e938)) - 5f) * -0.125f);
            let _e1788 = select(_e1786, 0f, (_e1786 < 0f));
            let _e1790 = select(_e1788, 1f, (_e1788 > 1f));
            let _e1796 = ((((_e1779 * _e1779) * (3f - (2f * _e1779))) * 0.12f) + (((_e1790 * _e1790) * (3f - (2f * _e1790))) * 0.08f));
            let _e1798 = (_e1773.x + _e1796);
            let _e1800 = (_e1773.y + _e1796);
            let _e1802 = (_e1773.z + _e1796);
            if (_e243 < 1f) {
                let _e1807 = (16f + (_e218.x * 276f));
                let _e1809 = select(_e218.y, 0f, (_e218.y < 0f));
                let _e1813 = (0.72f - (select(_e1809, 1f, (_e1809 > 1f)) * 0.45f));
                let _e1816 = ((_e218.y - 0.55f) * -1.8867923f);
                let _e1818 = select(_e1816, 0f, (_e1816 < 0f));
                let _e1820 = select(_e1818, 1f, (_e1818 > 1f));
                let _e1824 = ((_e1820 * _e1820) * (3f - (2f * _e1820)));
                let _e1825 = (1f - _e1824);
                if (_e830 > 0.0009765625f) {
                    phi_72_ = 0i;
                    phi_73_ = 0.5f;
                    phi_74_ = 0f;
                    phi_75_ = vec2<f32>((((_e1807 / _e234) * 0.14f) + (_e1079 * 0.012f)), ((_e1813 * 0.14f) + 6.1f));
                    loop {
                        let _e1843 = phi_72_;
                        let _e1845 = phi_73_;
                        let _e1847 = phi_74_;
                        let _e1849 = phi_75_;
                        local_49 = _e1847;
                        let _e1850 = (_e1843 < 4i);
                        if _e1850 {
                            let _e1853 = cantus_render_shader_simplex_noise(_e1849);
                            phi_76_ = (_e1843 + 1i);
                            phi_77_ = (_e1845 * 0.5f);
                            phi_78_ = (_e1847 + (_e1853 * _e1845));
                            phi_79_ = vec2<f32>(((_e1849.x * 1.6f) + (_e1849.y * 1.2f)), ((_e1849.y * 1.6f) - (_e1849.x * 1.2f)));
                        } else {
                            phi_76_ = i32();
                            phi_77_ = f32();
                            phi_78_ = f32();
                            phi_79_ = vec2<f32>();
                        }
                        let _e1866 = phi_76_;
                        let _e1868 = phi_77_;
                        let _e1870 = phi_78_;
                        let _e1872 = phi_79_;
                        continue;
                        continuing {
                            phi_72_ = _e1866;
                            phi_73_ = _e1868;
                            phi_74_ = _e1870;
                            phi_75_ = _e1872;
                            break if !(_e1850);
                        }
                    }
                    let _e1875 = local_49;
                    let _e1878 = (((_e1875 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e1880 = select(_e1878, 0f, (_e1878 < 0f));
                    let _e1882 = select(_e1880, 1f, (_e1880 > 1f));
                    phi_80_ = ((((_e1882 * _e1882) * (3f - (2f * _e1882))) * _e830) * 0.82f);
                } else {
                    phi_80_ = 0f;
                }
                let _e1890 = phi_80_;
                let _e1892 = ((_e218.y - -0.02f) * 16.666668f);
                let _e1894 = select(_e1892, 0f, (_e1892 < 0f));
                let _e1896 = select(_e1894, 1f, (_e1894 > 1f));
                let _e1903 = (_e235 - _e1807);
                let _e1904 = (_e236 - (_e234 * _e1813));
                let _e1908 = sqrt(((_e1903 * _e1903) + (_e1904 * _e1904)));
                let _e1910 = ((_e1908 - 62f) * -0.01724138f);
                let _e1912 = select(_e1910, 0f, (_e1910 < 0f));
                let _e1914 = select(_e1912, 1f, (_e1912 > 1f));
                let _e1921 = ((_e1908 - 11f) * -0.1f);
                let _e1923 = select(_e1921, 0f, (_e1921 < 0f));
                let _e1925 = select(_e1923, 1f, (_e1923 > 1f));
                let _e1932 = (((((_e1914 * _e1914) * (3f - (2f * _e1914))) * 0.24f) + (((_e1925 * _e1925) * (3f - (2f * _e1925))) * 0.7f)) * (((_e1896 * _e1896) * (3f - (2f * _e1896))) * (1f - _e1890)));
                let _e1933 = (1f - _e1932);
                let _e1946 = ((_e243 - 1f) / ((_e234 * -0.25f) - 1f));
                let _e1948 = select(_e1946, 0f, (_e1946 < 0f));
                let _e1950 = select(_e1948, 1f, (_e1948 > 1f));
                let _e1954 = ((_e1950 * _e1950) * (3f - (2f * _e1950)));
                let _e1955 = (1f - _e1954);
                phi_81_ = vec3<f32>(((_e1798 * _e1955) + (((_e1798 * _e1933) + (((0.96f * _e1825) + (0.98f * _e1824)) * _e1932)) * _e1954)), ((_e1800 * _e1955) + (((_e1800 * _e1933) + (((0.98f * _e1825) + (0.74f * _e1824)) * _e1932)) * _e1954)), ((_e1802 * _e1955) + (((_e1802 * _e1933) + ((_e1825 + (0.66f * _e1824)) * _e1932)) * _e1954)));
            } else {
                phi_81_ = (_e1773 + vec3(_e1796));
            }
            let _e1967 = phi_81_;
            let _e1978 = local_50;
            let _e1979 = (1f - _e1978);
            let _e1984 = local_51;
            let _e1987 = local_52;
            let _e1990 = local_53;
            if (_e217.y < _e282) {
                phi_93_ = 0u;
                phi_94_ = u32();
                phi_95_ = true;
            } else {
                let _e1996 = (_e217.x - (_e230 - 158f));
                if (_e1996 >= 316f) {
                    if (_e291 < ((_e237 + 96f) * 0.5f)) {
                        phi_89_ = 4u;
                    } else {
                        let _e2041 = (_e543 + _e544);
                        let _e2042 = (_e2041 + _e237);
                        if (_e291 > _e2042) {
                            let _e2058 = cantus_render_tempestas_cell_index(_e291, _e2042, 28f, 2f);
                            phi_88_ = ((76u + (_e2058 * 2u)) + select(0u, 1u, (_e291 > (_e2042 + (4f * (3.5f + (f32(_e2058) * 7f)))))));
                        } else {
                            let _e2044 = (_e291 > _e536);
                            let _e2045 = select(6u, 5u, _e2044);
                            let _e2052 = cantus_render_tempestas_cell_index(_e1996, 316f, (308f / f32(_e2045)), f32((_e2045 - 1u)));
                            phi_88_ = ((select(5u, 17u, _e2044) + (_e2052 * 2u)) + select(0u, 1u, (_e291 > select(_e543, _e2041, _e2044))));
                        }
                        let _e2070 = phi_88_;
                        phi_89_ = _e2070;
                    }
                    let _e2072 = phi_89_;
                    phi_90_ = _e2072;
                    phi_91_ = u32();
                    phi_92_ = true;
                } else {
                    if (_e291 < 54f) {
                        let _e2011 = ((_e542 - 0.5295082f) * 4.1666665f);
                        let _e2013 = select(_e2011, 0f, (_e2011 < 0f));
                        let _e2015 = select(_e2013, 1f, (_e2013 > 1f));
                        let _e2020 = (126f * ((_e2015 * _e2015) * (3f - (2f * _e2015))));
                        if (abs((_e1996 - (154f - _e2020))) < 20f) {
                            phi_84_ = 2u;
                        } else {
                            phi_84_ = select(1u, 3u, (abs((_e1996 - (154f + _e2020))) < 20f));
                        }
                        let _e2031 = phi_84_;
                        phi_85_ = _e2031;
                        phi_86_ = u32();
                        phi_87_ = true;
                    } else {
                        let _e1999 = cantus_render_tempestas_cell_index(_e1996, 0f, 44f, 6f);
                        let _e2000 = (_e291 < 82f);
                        if _e2000 {
                            phi_82_ = (27u + _e1999);
                            phi_83_ = u32();
                        } else {
                            let _e2001 = cantus_render_tempestas_cell_index(_e291, 84f, 24f, 5f);
                            phi_82_ = u32();
                            phi_83_ = ((34u + (_e2001 * 7u)) + _e1999);
                        }
                        let _e2007 = phi_82_;
                        let _e2009 = phi_83_;
                        phi_85_ = _e2007;
                        phi_86_ = _e2009;
                        phi_87_ = _e2000;
                    }
                    let _e2033 = phi_85_;
                    let _e2035 = phi_86_;
                    let _e2037 = phi_87_;
                    phi_90_ = _e2033;
                    phi_91_ = _e2035;
                    phi_92_ = _e2037;
                }
                let _e2074 = phi_90_;
                let _e2076 = phi_91_;
                let _e2078 = phi_92_;
                phi_93_ = _e2074;
                phi_94_ = _e2076;
                phi_95_ = _e2078;
            }
            let _e2080 = phi_93_;
            let _e2082 = phi_94_;
            let _e2084 = phi_95_;
            let _e2085 = select(_e2082, _e2080, _e2084);
            if (_e2085 < arrayLength((&text_lines.member))) {
            } else {
                break;
            }
            let _e2089 = text_lines.member[_e2085];
            let _e2091 = unpack4x8unorm(_e2089.color);
            if (_e217.x < _e2089.min.x) {
                phi_123_ = f32();
                phi_124_ = true;
            } else {
                if (_e217.x > _e2089.max.x) {
                    phi_121_ = f32();
                    phi_122_ = true;
                } else {
                    if (_e217.y < _e2089.min.y) {
                        phi_119_ = f32();
                        phi_120_ = true;
                    } else {
                        let _e2103 = (_e217.y > _e2089.max.y);
                        if _e2103 {
                            phi_118_ = f32();
                        } else {
                            let _e2105 = (1f / _e2089.size);
                            let _e2112 = ((_e217.x - _e2089.origin.x) * _e2105);
                            phi_96_ = 0u;
                            phi_97_ = _e2089.count;
                            loop {
                                let _e2117 = phi_96_;
                                let _e2119 = phi_97_;
                                local_54 = _e2117;
                                let _e2120 = (_e2117 < _e2119);
                                if _e2120 {
                                    let _e2123 = (_e2117 + ((_e2119 - _e2117) / 2u));
                                    let _e2128 = placed_glyphs_2.member[(_e2089.first + _e2123)].x;
                                    let _e2129 = (_e2128 <= _e2112);
                                    if _e2129 {
                                        phi_98_ = (_e2123 + 1u);
                                    } else {
                                        phi_98_ = _e2117;
                                    }
                                    let _e2132 = phi_98_;
                                    phi_99_ = _e2132;
                                    phi_100_ = select(_e2123, _e2119, _e2129);
                                } else {
                                    phi_99_ = u32();
                                    phi_100_ = u32();
                                }
                                let _e2135 = phi_99_;
                                let _e2137 = phi_100_;
                                continue;
                                continuing {
                                    phi_96_ = _e2135;
                                    phi_97_ = _e2137;
                                    break if !(_e2120);
                                }
                            }
                            let _e2139 = (3.5f / _e2089.size);
                            let _e2141 = local_54;
                            let _e2142 = (_e2141 + 1u);
                            phi_101_ = select(_e2142, _e2089.count, (_e2089.count < _e2142));
                            phi_102_ = -1000000f;
                            loop {
                                let _e2146 = phi_101_;
                                let _e2148 = phi_102_;
                                local_57 = _e2148;
                                if (_e2146 > 0u) {
                                    let _e2150 = (_e2146 - 1u);
                                    let _e2151 = (_e2089.first + _e2150);
                                    let _e2155 = placed_glyphs_2.member[_e2151].x;
                                    let _e2159 = placed_glyphs_2.member[_e2151].glyph;
                                    let _e2164 = glyphs_2.member[_e2159].min[0u];
                                    let _e2169 = glyphs_2.member[_e2159].min[1u];
                                    let _e2174 = glyphs_2.member[_e2159].max[0u];
                                    let _e2179 = glyphs_2.member[_e2159].max[1u];
                                    let _e2183 = glyphs_2.member[_e2159].start;
                                    let _e2187 = glyphs_2.member[_e2159].count;
                                    let _e2188 = (_e2112 - _e2155);
                                    let _e2189 = -(((_e217.y - _e2089.origin.y) * _e2105));
                                    let _e2190 = (_e2174 + _e2139);
                                    let _e2191 = (_e2188 > _e2190);
                                    if _e2191 {
                                        phi_114_ = f32();
                                    } else {
                                        if (_e2188 >= (_e2164 - _e2139)) {
                                            if (_e2189 >= (_e2169 - _e2139)) {
                                                if (_e2188 <= _e2190) {
                                                    if (_e2189 <= (_e2179 + _e2139)) {
                                                        phi_103_ = 340282350000000000000000000000000000000f;
                                                        phi_104_ = 0u;
                                                        phi_105_ = 0i;
                                                        loop {
                                                            let _e2201 = phi_103_;
                                                            let _e2203 = phi_104_;
                                                            let _e2205 = phi_105_;
                                                            local_55 = _e2201;
                                                            local_56 = _e2205;
                                                            let _e2206 = (_e2203 < _e2187);
                                                            if _e2206 {
                                                                let _e2210 = edges_2.member[(_e2183 + _e2203)];
                                                                let _e2212 = cantus_render_text_edge_distance(_e2210, _e2089.weight, vec2<f32>(_e2188, _e2189), _e2201);
                                                                phi_106_ = _e2212.member;
                                                                phi_107_ = (_e2203 + 1u);
                                                                phi_108_ = (_e2205 + _e2212.member_1);
                                                            } else {
                                                                phi_106_ = f32();
                                                                phi_107_ = u32();
                                                                phi_108_ = i32();
                                                            }
                                                            let _e2218 = phi_106_;
                                                            let _e2220 = phi_107_;
                                                            let _e2222 = phi_108_;
                                                            continue;
                                                            continuing {
                                                                phi_103_ = _e2218;
                                                                phi_104_ = _e2220;
                                                                phi_105_ = _e2222;
                                                                break if !(_e2206);
                                                            }
                                                        }
                                                        let _e2225 = local_55;
                                                        let _e2229 = local_56;
                                                        let _e2232 = ((sqrt(_e2225) * _e2089.size) * select(1f, -1f, (_e2229 == 0i)));
                                                        if (_e2148 != _e2148) {
                                                            phi_109_ = true;
                                                        } else {
                                                            phi_109_ = (_e2232 >= _e2148);
                                                        }
                                                        let _e2236 = phi_109_;
                                                        phi_110_ = select(_e2148, _e2232, _e2236);
                                                    } else {
                                                        phi_110_ = _e2148;
                                                    }
                                                    let _e2239 = phi_110_;
                                                    phi_111_ = _e2239;
                                                } else {
                                                    phi_111_ = _e2148;
                                                }
                                                let _e2241 = phi_111_;
                                                phi_112_ = _e2241;
                                            } else {
                                                phi_112_ = _e2148;
                                            }
                                            let _e2243 = phi_112_;
                                            phi_113_ = _e2243;
                                        } else {
                                            phi_113_ = _e2148;
                                        }
                                        let _e2245 = phi_113_;
                                        phi_114_ = _e2245;
                                    }
                                    let _e2247 = phi_114_;
                                    phi_115_ = _e2150;
                                    phi_116_ = _e2247;
                                    phi_117_ = select(true, false, _e2191);
                                } else {
                                    phi_115_ = u32();
                                    phi_116_ = f32();
                                    phi_117_ = false;
                                }
                                let _e2250 = phi_115_;
                                let _e2252 = phi_116_;
                                let _e2254 = phi_117_;
                                continue;
                                continuing {
                                    phi_101_ = _e2250;
                                    phi_102_ = _e2252;
                                    break if !(_e2254);
                                }
                            }
                            let _e2257 = local_57;
                            let _e2259 = ((_e2257 * 1.25f) + 0.5f);
                            let _e2261 = select(_e2259, 0f, (_e2259 < 0f));
                            let _e2263 = select(_e2261, 1f, (_e2261 > 1f));
                            phi_118_ = ((_e2263 * _e2263) * (3f - (2f * _e2263)));
                        }
                        let _e2269 = phi_118_;
                        phi_119_ = _e2269;
                        phi_120_ = _e2103;
                    }
                    let _e2271 = phi_119_;
                    let _e2273 = phi_120_;
                    phi_121_ = _e2271;
                    phi_122_ = _e2273;
                }
                let _e2275 = phi_121_;
                let _e2277 = phi_122_;
                phi_123_ = _e2275;
                phi_124_ = _e2277;
            }
            let _e2279 = phi_123_;
            let _e2281 = phi_124_;
            let _e2284 = (select(_e2279, 0f, _e2281) * _e2091.w);
            let _e2285 = (1f - _e2284);
            out_color = vec4<f32>((((((_e1967.x * _e1979) + (((_e1967.x * 1.5f) + 0.1f) * _e1984)) * _e2285) + (_e2091.x * _e2284)) * _e779), (((((_e1967.y * _e1979) + (((_e1967.y * 1.5f) + 0.1f) * _e1987)) * _e2285) + (_e2091.y * _e2284)) * _e779), (((((_e1967.z * _e1979) + (((_e1967.z * 1.5f) + 0.1f) * _e1990)) * _e2285) + (_e2091.z * _e2284)) * _e779), _e792);
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
