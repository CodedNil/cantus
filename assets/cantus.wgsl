struct render_RipplePulse {
    origin: vec2<f32>,
    start_time: f32,
    strength: f32,
}

struct render_FrameData {
    screen_size: vec2<f32>,
    mouse_pos: vec2<f32>,
    panel_height: f32,
    mouse_pressure: f32,
    playhead_x: f32,
    px_per_ms: f32,
    status_width: f32,
    time: f32,
    weather_hour: f32,
    ripples: array<render_RipplePulse, 4>,
}

struct type_6 {
    member: array<render_FrameData>,
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
    end: vec2<f32>,
    start_delta: vec2<f32>,
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

struct render_launcher_LauncherRow {
    y: f32,
    icon: i32,
    caret: vec2<f32>,
    selection: vec2<f32>,
    badges: array<vec2<f32>, 2>,
    lines: array<render_text_Line, 4>,
}

struct type_32 {
    member: array<render_launcher_LauncherRow>,
}

struct render_playhead_PlayheadState {
    bar_split: f32,
    icon_presence: f32,
    icon_morph: f32,
}

struct type_34 {
    member: array<render_playhead_PlayheadState>,
}

struct render_particles_Particle {
    spawn_pos: vec2<f32>,
    spawn_vel: vec2<f32>,
    end_time: f32,
    duration: f32,
    rgb: u32,
}

struct type_36 {
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

struct type_42 {
    member: array<render_tempestas_WeatherSurface>,
}

struct type_44 {
    member: array<u32>,
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

var<private> vertex_7: u32;
var<private> instance_2: u32;
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
var<private> out_color: vec4<f32>;
@group(0) @binding(1)
var<storage> line: type_24;
var<private> _isthmus_instance_index_9: u32;
var<private> out_pixel: vec2<f32>;
var<private> out_isthmus_instance_index: u32;
var<private> pixel_4: vec2<f32>;
var<private> _isthmus_instance_index_10: u32;
@group(0) @binding(2)
var<storage> placed_glyphs_1: type_15;
@group(0) @binding(3)
var<storage> glyphs_1: type_17;
@group(0) @binding(4)
var<storage> edges_1: type_19;
@group(0) @binding(1)
var<storage> pill_1: type_28;
@group(0) @binding(1)
var<storage> row: type_32;
var<private> out_row_idx: u32;
var<private> row_idx_1: u32;
@group(0) @binding(2)
var icons: texture_2d_array<f32>;
var<private> out_world_pos: vec2<f32>;
var<private> world_pos_1: vec2<f32>;
@group(0) @binding(1)
var<storage> state: type_34;
@group(0) @binding(1)
var<storage> particle: type_36;
var<private> out_uv: vec2<f32>;
var<private> color_1: vec4<f32>;
var<private> uv_1: vec2<f32>;
@group(0) @binding(1)
var<storage> pill_2: type_42;
var<private> out_weather: vec4<f32>;
var<private> out_isthmus_instance_index_1: u32;
var<private> weather_1: vec4<f32>;
var<private> _isthmus_instance_index_11: u32;
@group(0) @binding(2)
var<storage> text_lines: type_24;
@group(0) @binding(3)
var<storage> text_cells: type_44;

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
            let _e33 = vertex_7;
            let _e34 = instance_2;
            let _e38 = frame.member[0u].mouse_pressure;
            phi_0_ = 0u;
            phi_1_ = (_e38 * 8f);
            loop {
                let _e41 = phi_0_;
                let _e43 = phi_1_;
                local = _e43;
                let _e44 = (_e41 < 4u);
                if _e44 {
                    if _e44 {
                    } else {
                        phi_4_ = true;
                        break;
                    }
                    let _e50 = frame.member[0u].ripples[_e41].start_time;
                    let _e56 = frame.member[0u].ripples[_e41].strength;
                    let _e60 = frame.member[0u].time;
                    let _e62 = ((_e60 - _e50) * 1.2f);
                    let _e64 = select(_e62, 0f, (_e62 < 0f));
                    let _e67 = (1f - select(_e64, 1f, (_e64 > 1f)));
                    phi_2_ = (_e41 + 1u);
                    phi_3_ = (_e43 + (((_e56 * _e67) * _e67) * 11f));
                } else {
                    phi_2_ = u32();
                    phi_3_ = f32();
                }
                let _e74 = phi_2_;
                let _e76 = phi_3_;
                continue;
                continuing {
                    phi_0_ = _e74;
                    phi_1_ = _e76;
                    phi_4_ = false;
                    break if !(_e44);
                }
            }
            let _e79 = phi_4_;
            if _e79 {
                break;
            }
            let _e81 = local;
            let _e82 = (_e81 * 0.5f);
            let _e83 = (18f + _e82);
            let _e87 = pill.member[_e34].width;
            let _e91 = frame.member[0u].panel_height;
            let _e95 = pill.member[_e34].x;
            let _e97 = (_e95 + (_e87 * 0.5f));
            let _e103 = pill.member[_e34].rating;
            let _e109 = pill.member[_e34].primary_playlist_count;
            let _e111 = (select(0f, 5f, (_e103 >= 0i)) + f32(_e109));
            let _e115 = pill.member[_e34].secondary_expansion;
            let _e121 = pill.member[_e34].secondary_playlist_count;
            let _e122 = f32(_e121);
            let _e126 = pill.member[_e34].primary_alpha;
            let _e127 = (_e111 - 1f);
            if (_e127 != _e127) {
                phi_5_ = true;
            } else {
                phi_5_ = (0f >= _e127);
            }
            let _e131 = phi_5_;
            let _e137 = select(0f, 1f, ((_e111 * _e126) > 0f));
            let _e138 = (((select(_e127, 0f, _e131) * 9f) + 32.4f) * _e137);
            let _e139 = (32.4f * _e137);
            let _e140 = (_e122 - 1f);
            if (_e140 != _e140) {
                phi_6_ = true;
            } else {
                phi_6_ = (0f >= _e140);
            }
            let _e144 = phi_6_;
            let _e152 = select(0f, 1f, ((_e122 * _e115) > 0f));
            let _e153 = (((((select(_e140, 0f, _e144) * 18f) * _e115) * 0.5f) + 32.4f) * _e152);
            let _e154 = (32.4f * _e152);
            let _e156 = select(_e153, _e138, (_e138 > _e153));
            let _e159 = (_e95 - _e83);
            let _e160 = (_e97 - _e156);
            if (_e159 != _e159) {
                phi_7_ = true;
            } else {
                phi_7_ = (_e160 <= _e159);
            }
            let _e164 = phi_7_;
            let _e165 = select(_e159, _e160, _e164);
            let _e166 = (-12f - _e82);
            let _e168 = ((_e95 + _e87) + _e83);
            let _e169 = (_e97 + _e156);
            if (_e168 != _e168) {
                phi_8_ = true;
            } else {
                phi_8_ = (_e169 >= _e168);
            }
            let _e173 = phi_8_;
            let _e176 = ((6f + _e91) + _e83);
            let _e178 = (((((_e91 * 0.975f) + 3f) + (18f * _e115)) + -5.4f) + select(_e154, _e139, (_e139 > _e154)));
            if (_e176 != _e176) {
                phi_9_ = true;
            } else {
                phi_9_ = (_e178 >= _e176);
            }
            let _e182 = phi_9_;
            let _e193 = (_e165 + (f32((_e33 & 1u)) * (select(_e168, _e169, _e173) - _e165)));
            let _e194 = (_e166 + (f32((_e33 >> bitcast<u32>(1i))) * (select(_e176, _e178, _e182) - _e166)));
            let _e199 = frame.member[0u].screen_size[0u];
            let _e204 = frame.member[0u].screen_size[1u];
            out_position = vec4<f32>((((_e193 / _e199) * 2f) - 1f), (1f - ((_e194 / _e204) * 2f)), 0f, 1f);
            out_pixel_pos[0u] = _e193;
            out_pixel_pos[1u] = _e194;
            out_pill_idx = _e34;
            break;
        }
    }
    return;
}

fn cantus_render_text_edge_distance(param: render_text_Edge, param_1: f32, param_2: vec2<f32>, param_3: f32) -> u0028_f32_u0020_i32_u0029_ {
    var phi_0_: bool;
    var phi_1_: bool;
    var phi_2_: bool;
    var phi_3_: i32;
    var phi_4_: i32;
    var phi_5_: bool;
    var phi_6_: bool;
    var phi_7_: bool;
    var phi_8_: u0028_f32_u0020_i32_u0029_;

    let _e24 = (param.start.x + (param.start_delta.x * param_1));
    let _e25 = (param.start.y + (param.start_delta.y * param_1));
    let _e36 = (param.end.x + (param.end_delta.x * param_1));
    let _e37 = (param.end.y + (param.end_delta.y * param_1));
    let _e38 = (_e36 - _e24);
    let _e39 = (_e37 - _e25);
    if (_e25 <= param_2.y) {
        phi_0_ = select(true, false, (param_2.y < _e37));
    } else {
        phi_0_ = true;
    }
    let _e44 = phi_0_;
    if _e44 {
        if (_e37 <= param_2.y) {
            phi_1_ = select(true, false, (param_2.y < _e25));
        } else {
            phi_1_ = true;
        }
        let _e49 = phi_1_;
        phi_2_ = select(true, false, _e49);
    } else {
        phi_2_ = true;
    }
    let _e52 = phi_2_;
    if _e52 {
        if ((_e24 + (((param_2.y - _e25) * _e38) / _e39)) > param_2.x) {
            phi_3_ = select(-1i, 1i, (_e39 > 0f));
        } else {
            phi_3_ = 0i;
        }
        let _e61 = phi_3_;
        phi_4_ = _e61;
    } else {
        phi_4_ = 0i;
    }
    let _e63 = phi_4_;
    let _e65 = select(_e36, _e24, (_e24 < _e36));
    let _e67 = select(_e37, _e25, (_e25 < _e37));
    let _e69 = select(_e36, _e24, (_e24 > _e36));
    let _e71 = select(_e37, _e25, (_e25 > _e37));
    let _e73 = select(_e65, param_2.x, (param_2.x > _e65));
    let _e75 = select(_e67, param_2.y, (param_2.y > _e67));
    let _e80 = (param_2.x - select(_e69, _e73, (_e73 < _e69)));
    let _e81 = (param_2.y - select(_e71, _e75, (_e75 < _e71)));
    if (((_e80 * _e80) + (_e81 * _e81)) >= param_3) {
        phi_8_ = u0028_f32_u0020_i32_u0029_(param_3, _e63);
    } else {
        let _e93 = ((_e38 * _e38) + (_e39 * _e39));
        if (_e93 != _e93) {
            phi_5_ = true;
        } else {
            phi_5_ = (0.00000001f >= _e93);
        }
        let _e97 = phi_5_;
        let _e99 = ((((param_2.x - _e24) * _e38) + ((param_2.y - _e25) * _e39)) / select(_e93, 0.00000001f, _e97));
        if (_e99 != _e99) {
            phi_6_ = true;
        } else {
            phi_6_ = (0f >= _e99);
        }
        let _e103 = phi_6_;
        let _e104 = select(_e99, 0f, _e103);
        if (_e104 != _e104) {
            phi_7_ = true;
        } else {
            phi_7_ = (1f <= _e104);
        }
        let _e108 = phi_7_;
        let _e109 = select(_e104, 1f, _e108);
        let _e114 = (param_2.x - (_e24 + (_e38 * _e109)));
        let _e115 = (param_2.y - (_e25 + (_e39 * _e109)));
        phi_8_ = u0028_f32_u0020_i32_u0029_(((_e114 * _e114) + (_e115 * _e115)), _e63);
    }
    let _e122 = phi_8_;
    return _e122;
}

fn cantus_render_shader_hash(param_4: vec2<f32>) -> vec2<f32> {
    let _e31 = ((bitcast<u32>(select(0i, select(select(i32(param_4.y), i32(-2147483648), (param_4.y < -2147483600f)), 2147483647i, (param_4.y > 2147483500f)), (param_4.y == param_4.y))) * 1664525u) + 1013904223u);
    let _e33 = (((bitcast<u32>(select(0i, select(select(i32(param_4.x), i32(-2147483648), (param_4.x < -2147483600f)), 2147483647i, (param_4.x > 2147483500f)), (param_4.x == param_4.x))) * 1664525u) + 1013904223u) + (_e31 * 1664525u));
    let _e35 = (_e31 + (_e33 * 1664525u));
    let _e41 = (_e35 ^ (_e35 >> bitcast<u32>(16i)));
    let _e43 = ((_e33 ^ (_e33 >> bitcast<u32>(16i))) + (_e41 * 1664525u));
    let _e45 = (_e41 + (_e43 * 1664525u));
    return vec2<f32>((f32((_e43 ^ (_e43 >> bitcast<u32>(16i)))) * 0.00000000023283064f), (f32((_e45 ^ (_e45 >> bitcast<u32>(16i)))) * 0.00000000023283064f));
}

fn cantus_render_shader_simplex_noise(param_5: vec2<f32>) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;
    var phi_2_: bool;

    let _e15 = ((param_5.x + param_5.y) * 0.36602542f);
    let _e18 = floor((param_5.x + _e15));
    let _e19 = floor((param_5.y + _e15));
    let _e23 = ((_e18 + _e19) * 0.21132487f);
    let _e24 = ((param_5.x - _e18) + _e23);
    let _e25 = ((param_5.y - _e19) + _e23);
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

fn cantus_render_track_plasma_field(param_6: vec2<f32>, param_7: vec4<f32>, param_8: f32, param_9: f32, param_10: f32) -> vec4<f32> {
    let _e17 = ((sin((((param_6.x * param_8) + (param_6.y * param_9)) + param_10)) * 0.5f) + 0.5f);
    let _e23 = ((0.12f + (_e17 * _e17)) * (0.25f + (param_7.w * 3f)));
    return vec4<f32>((param_7.x * _e23), (param_7.y * _e23), (param_7.z * _e23), _e23);
}

fn cantus_render_shader_sd_capsule_box(param_11: vec2<f32>, param_12: f32, param_13: f32) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;

    let _e8 = abs(param_11.y);
    let _e9 = (abs(param_11.x) - param_12);
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
    return ((sqrt(((_e11 * _e11) + (_e13 * _e13))) + select(_e22, 0f, _e26)) - param_13);
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
    var local_1: vec2<f32>;
    var local_2: vec2<f32>;
    var phi_15_: bool;
    var local_3: vec2<f32>;
    var phi_16_: f32;
    var local_4: vec2<f32>;
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
    var local_5: f32;
    var local_6: f32;
    var local_7: f32;
    var local_8: f32;
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
    var phi_49_: render_RipplePulse;
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
    var local_9: u32;
    var phi_68_: u32;
    var phi_69_: f32;
    var phi_70_: f32;
    var phi_71_: u32;
    var phi_72_: i32;
    var phi_73_: f32;
    var phi_74_: u32;
    var phi_75_: i32;
    var local_10: f32;
    var phi_76_: f32;
    var local_11: i32;
    var phi_77_: bool;
    var phi_78_: f32;
    var phi_79_: f32;
    var phi_80_: f32;
    var phi_81_: f32;
    var phi_82_: f32;
    var phi_83_: u32;
    var phi_84_: f32;
    var phi_85_: bool;
    var local_12: f32;
    var phi_86_: u32;
    var phi_87_: u32;
    var phi_88_: u32;
    var phi_89_: u32;
    var phi_90_: u32;
    var local_13: u32;
    var phi_91_: u32;
    var phi_92_: f32;
    var phi_93_: f32;
    var phi_94_: u32;
    var phi_95_: i32;
    var phi_96_: f32;
    var phi_97_: u32;
    var phi_98_: i32;
    var local_14: f32;
    var phi_99_: f32;
    var local_15: i32;
    var phi_100_: bool;
    var phi_101_: f32;
    var phi_102_: f32;
    var phi_103_: f32;
    var phi_104_: f32;
    var phi_105_: f32;
    var phi_106_: u32;
    var phi_107_: f32;
    var phi_108_: bool;
    var local_16: f32;
    var phi_109_: bool;
    var local_17: vec4<f32>;
    var local_18: vec4<f32>;
    var local_19: vec4<f32>;
    var local_20: vec4<f32>;
    var local_21: vec4<f32>;

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
                local_1 = _e215;
                local_2 = _e215;
                local_3 = _e215;
                local_4 = _e215;
                local_5 = _e217;
                local_6 = _e217;
                local_7 = _e217;
                local_8 = _e217;
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
            let _e361 = local_1;
            let _e364 = global[0u];
            if (_e361.x == _e364) {
                let _e367 = local_2;
                let _e370 = global[1u];
                phi_15_ = (_e367.y == _e370);
            } else {
                phi_15_ = false;
            }
            let _e373 = phi_15_;
            if _e373 {
                phi_16_ = 0f;
            } else {
                let _e375 = local_3;
                phi_16_ = (sqrt(((_e361.x * _e361.x) + (_e375.y * _e375.y))) * 22f);
            }
            let _e383 = phi_16_;
            let _e385 = local_4;
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
            let _e1219 = ((_e560 - 5f) * -0.125f);
            let _e1221 = select(_e1219, 0f, (_e1219 < 0f));
            let _e1223 = select(_e1221, 1f, (_e1221 > 1f));
            let _e1228 = (((_e1223 * _e1223) * (3f - (2f * _e1223))) * 0.14f);
            let _e1232 = (_e1208.x + (((_e1208.x * 0.68f) + 0.32f) * _e1228));
            let _e1233 = (_e1208.y + (((_e1208.y * 0.68f) + 0.32f) * _e1228));
            let _e1234 = (_e1208.z + (((_e1208.z * 0.68f) + 0.32f) * _e1228));
            let _e1242 = local_5;
            let _e1243 = (1f - _e1242);
            let _e1248 = local_6;
            let _e1251 = local_7;
            let _e1254 = local_8;
            let _e1262 = vec4<f32>((((_e1232 * _e1243) + (((_e1232 * 1.5f) + 0.1f) * _e1248)) * _e578), (((_e1233 * _e1243) + (((_e1233 * 1.5f) + 0.1f) * _e1251)) * _e578), (((_e1234 * _e1243) + (((_e1234 * 1.5f) + 0.1f) * _e1254)) * _e578), _e591);
            if _e398 {
                if (_e476 > 0f) {
                    phi_35_ = _e1262;
                    phi_36_ = 0u;
                    loop {
                        let _e1265 = phi_35_;
                        let _e1267 = phi_36_;
                        local_21 = _e1265;
                        let _e1268 = (_e1267 < 5u);
                        if _e1268 {
                            let _e1269 = f32(_e1267);
                            if _e434 {
                                phi_37_ = true;
                            } else {
                                phi_37_ = (0f >= _e433);
                            }
                            let _e1272 = phi_37_;
                            let _e1277 = (_e391 + ((_e1269 - (select(_e433, 0f, _e1272) * 0.5f)) * 18f));
                            let _e1278 = (_e392 + 5f);
                            let _e1279 = (_e165.x - _e1277);
                            let _e1280 = (_e165.y - _e1278);
                            let _e1281 = abs(_e1279);
                            let _e1282 = abs(_e1280);
                            if (select(_e1282, _e1281, (_e1281 > _e1282)) < 38.88f) {
                                let _e1289 = ((f32(_e397) - (_e1269 * 2f)) * 0.5f);
                                let _e1291 = select(_e1289, 0f, (_e1289 < 0f));
                                let _e1294 = (_e1277 - _e425);
                                let _e1295 = (_e1278 - _e430);
                                let _e1301 = ((sqrt(((_e1294 * _e1294) + (_e1295 * _e1295))) - 11.3f) * -1f);
                                let _e1303 = select(_e1301, 0f, (_e1301 < 0f));
                                let _e1305 = select(_e1303, 1f, (_e1303 > 1f));
                                let _e1311 = select(_e194, 0f, (_e194 < 0f));
                                let _e1314 = (((_e1305 * _e1305) * (3f - (2f * _e1305))) * select(_e1311, 1f, (_e1311 > 1f)));
                                let _e1316 = (1.05f + (0.63f * _e1314));
                                let _e1317 = (_e1294 * _e1314);
                                let _e1319 = (_e1279 - (_e1317 * 0.5f));
                                let _e1320 = (_e1317 * -0.005f);
                                let _e1321 = sin(_e1320);
                                let _e1322 = cos(_e1320);
                                let _e1325 = ((_e1322 * _e1319) - (_e1321 * _e1280));
                                let _e1328 = ((_e1321 * _e1319) + (_e1322 * _e1280));
                                let _e1332 = (_e1316 * 5.4f);
                                let _e1333 = abs(_e1325);
                                let _e1337 = ((0.809017f * _e1333) + (_e1328 * 0.58778524f));
                                if (_e1337 != _e1337) {
                                    phi_38_ = true;
                                } else {
                                    phi_38_ = (0f >= _e1337);
                                }
                                let _e1341 = phi_38_;
                                let _e1342 = select(_e1337, 0f, _e1341);
                                let _e1345 = (_e1333 - (_e1342 * 1.618034f));
                                let _e1346 = (-(_e1328) - (_e1342 * -1.1755705f));
                                let _e1349 = ((-0.809017f * _e1345) + (-0.58778524f * _e1346));
                                if (_e1349 != _e1349) {
                                    phi_39_ = true;
                                } else {
                                    phi_39_ = (0f >= _e1349);
                                }
                                let _e1353 = phi_39_;
                                let _e1354 = select(_e1349, 0f, _e1353);
                                let _e1359 = abs((_e1345 - (_e1354 * -1.618034f)));
                                let _e1360 = ((_e1346 - (_e1354 * -1.1755705f)) - _e1332);
                                let _e1361 = (_e1316 * 2.031386f);
                                let _e1363 = ((_e1316 * 2.7959628f) - _e1332);
                                let _e1370 = (((_e1359 * _e1361) + (_e1360 * _e1363)) / ((_e1361 * _e1361) + (_e1363 * _e1363)));
                                let _e1372 = select(_e1370, 0f, (_e1370 < 0f));
                                let _e1374 = select(_e1372, 1f, (_e1372 > 1f));
                                let _e1380 = (_e1359 - (_e1361 * _e1374));
                                let _e1381 = (_e1360 - (_e1363 * _e1374));
                                let _e1390 = ((sqrt(((_e1380 * _e1380) + (_e1381 * _e1381))) * select(1f, -1f, (((_e1360 * _e1361) - (_e1359 * _e1363)) < 0f))) - (_e1316 * 1.08f));
                                let _e1391 = (((_e1325 / (_e1316 * 21.6f)) + 0.5f) - select(_e1291, 1f, (_e1291 > 1f)));
                                let _e1392 = fwidth(_e1391);
                                let _e1394 = ((_e1391 / _e1392) + 0.5f);
                                let _e1396 = select(_e1394, 0f, (_e1394 < 0f));
                                let _e1398 = select(_e1396, 1f, (_e1396 > 1f));
                                let _e1399 = (1f - _e1398);
                                let _e1402 = (0.33f * _e1398);
                                let _e1406 = (0.5f - _e1390);
                                let _e1408 = select(_e1406, 0f, (_e1406 < 0f));
                                let _e1410 = select(_e1408, 1f, (_e1408 > 1f));
                                if (_e1390 != _e1390) {
                                    phi_40_ = true;
                                } else {
                                    phi_40_ = (0f >= _e1390);
                                }
                                let _e1414 = phi_40_;
                                let _e1417 = exp((select(_e1390, 0f, _e1414) * -0.5f));
                                let _e1418 = (_e1390 * -0.2f);
                                let _e1420 = select(_e1418, 0f, (_e1418 < 0f));
                                let _e1422 = select(_e1420, 1f, (_e1420 > 1f));
                                let _e1427 = (1f - ((_e1422 * _e1422) * (3f - (2f * _e1422))));
                                let _e1429 = ((_e1427 * _e1427) * 0.045f);
                                let _e1440 = ((_e1417 * _e1417) * 0.2f);
                                if (_e1410 != _e1410) {
                                    phi_41_ = true;
                                } else {
                                    phi_41_ = (_e1440 >= _e1410);
                                }
                                let _e1444 = phi_41_;
                                let _e1446 = (select(_e1410, _e1440, _e1444) * _e476);
                                let _e1447 = (1f - _e1446);
                                phi_42_ = vec4<f32>(((_e1265.x * _e1447) + ((((_e1399 + _e1402) + _e1429) * _e1410) * _e476)), ((_e1265.y * _e1447) + (((((0.85f * _e1399) + _e1402) + _e1429) * _e1410) * _e476)), ((_e1265.z * _e1447) + (((((0.2f * _e1399) + _e1402) + _e1429) * _e1410) * _e476)), ((_e1265.w * _e1447) + _e1446));
                            } else {
                                phi_42_ = _e1265;
                            }
                            let _e1462 = phi_42_;
                            phi_43_ = _e1462;
                            phi_44_ = (_e1267 + 1u);
                        } else {
                            phi_43_ = vec4<f32>();
                            phi_44_ = u32();
                        }
                        let _e1465 = phi_43_;
                        let _e1467 = phi_44_;
                        continue;
                        continuing {
                            phi_35_ = _e1465;
                            phi_36_ = _e1467;
                            break if !(_e1268);
                        }
                    }
                    if _e329 {
                        break;
                    }
                    let _e2116 = local_21;
                    phi_45_ = _e2116;
                } else {
                    phi_45_ = _e1262;
                }
                let _e1470 = phi_45_;
                phi_46_ = _e1470;
            } else {
                phi_46_ = _e1262;
            }
            let _e1472 = phi_46_;
            let _e1473 = (_e403 + _e417);
            phi_47_ = _e1472;
            phi_48_ = 0u;
            loop {
                let _e1477 = phi_47_;
                let _e1479 = phi_48_;
                local_17 = _e1477;
                local_18 = _e1477;
                local_19 = _e1477;
                local_20 = _e1477;
                let _e1480 = (_e1479 < select(_e1473, 8u, (8u < _e1473)));
                if _e1480 {
                    if (_e1479 < 8u) {
                    } else {
                        phi_62_ = true;
                        break;
                    }
                    let _e1486 = pill.member[_e166].playlist_images[_e1479];
                    if (_e1486 >= 0i) {
                        let _e1488 = (_e1479 < _e403);
                        if _e1488 {
                            phi_49_ = render_RipplePulse(vec2<f32>(_e391, _e393), _e405, 1f);
                            phi_50_ = (f32(_e1479) + _e399);
                        } else {
                            phi_49_ = render_RipplePulse(vec2<f32>(_e391, _e413), _e418, _e411);
                            phi_50_ = f32((_e1479 - _e403));
                        }
                        let _e1494 = phi_49_;
                        let _e1496 = phi_50_;
                        let _e1497 = select(_e411, _e476, _e1488);
                        let _e1499 = (_e1494.start_time - 1f);
                        if (_e1499 != _e1499) {
                            phi_51_ = true;
                        } else {
                            phi_51_ = (0f >= _e1499);
                        }
                        let _e1503 = phi_51_;
                        let _e1512 = (_e1494.origin.x + (((_e1496 - (select(_e1499, 0f, _e1503) * 0.5f)) * 18f) * _e1494.strength));
                        let _e1515 = (_e1494.origin.y + 2f);
                        if (_e1497 > 0f) {
                            let _e1517 = (_e165.x - _e1512);
                            let _e1518 = (_e165.y - _e1515);
                            let _e1519 = abs(_e1517);
                            let _e1520 = abs(_e1518);
                            if (select(_e1520, _e1519, (_e1519 > _e1520)) < 38.88f) {
                                let _e1524 = (_e1512 - _e425);
                                let _e1525 = (_e1515 - _e430);
                                let _e1529 = sqrt(((_e1524 * _e1524) + (_e1525 * _e1525)));
                                let _e1531 = ((_e1529 - 11.3f) * -1f);
                                let _e1533 = select(_e1531, 0f, (_e1531 < 0f));
                                let _e1535 = select(_e1533, 1f, (_e1533 > 1f));
                                let _e1541 = select(_e194, 0f, (_e194 < 0f));
                                let _e1544 = (((_e1535 * _e1535) * (3f - (2f * _e1535))) * select(_e1541, 1f, (_e1541 > 1f)));
                                let _e1546 = (1.05f + (0.63f * _e1544));
                                let _e1547 = (_e1524 * _e1544);
                                let _e1549 = (_e1517 - (_e1547 * 0.5f));
                                let _e1550 = (_e1547 * -0.005f);
                                let _e1551 = sin(_e1550);
                                let _e1552 = cos(_e1550);
                                let _e1555 = ((_e1552 * _e1549) - (_e1551 * _e1518));
                                let _e1558 = ((_e1551 * _e1549) + (_e1552 * _e1518));
                                let _e1559 = (_e1546 * 21.6f);
                                if _e1488 {
                                    phi_53_ = true;
                                } else {
                                    if _e195 {
                                        phi_52_ = select(true, false, (_e1529 <= 10.8f));
                                    } else {
                                        phi_52_ = true;
                                    }
                                    let _e1567 = phi_52_;
                                    phi_53_ = select(true, false, _e1567);
                                }
                                let _e1570 = phi_53_;
                                let _e1571 = select(0.2f, 0f, _e1570);
                                let _e1574 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1555, _e1558), 0f, (_e1546 * 6.4800005f));
                                if (_e1574 <= 7f) {
                                    let _e1577 = vec3<f32>(((_e1555 / _e1559) + 0.5f), ((_e1558 / _e1559) + 0.5f), f32(_e1486));
                                    let _e1583 = textureSample(images, sampler_, vec2<f32>(_e1577.x, _e1577.y), i32(_e1577.z));
                                    let _e1587 = (1f - _e1571);
                                    let _e1591 = (0.24f * _e1571);
                                    let _e1595 = (0.5f - _e1574);
                                    let _e1597 = select(_e1595, 0f, (_e1595 < 0f));
                                    let _e1599 = select(_e1597, 1f, (_e1597 > 1f));
                                    if (_e1574 != _e1574) {
                                        phi_54_ = true;
                                    } else {
                                        phi_54_ = (0f >= _e1574);
                                    }
                                    let _e1603 = phi_54_;
                                    let _e1606 = exp((select(_e1574, 0f, _e1603) * -0.5f));
                                    let _e1607 = (_e1574 * -0.2f);
                                    let _e1609 = select(_e1607, 0f, (_e1607 < 0f));
                                    let _e1611 = select(_e1609, 1f, (_e1609 > 1f));
                                    let _e1616 = (1f - ((_e1611 * _e1611) * (3f - (2f * _e1611))));
                                    let _e1618 = ((_e1616 * _e1616) * 0.045f);
                                    let _e1629 = ((_e1606 * _e1606) * 0.2f);
                                    if (_e1599 != _e1599) {
                                        phi_55_ = true;
                                    } else {
                                        phi_55_ = (_e1629 >= _e1599);
                                    }
                                    let _e1633 = phi_55_;
                                    let _e1635 = (select(_e1599, _e1629, _e1633) * _e1497);
                                    let _e1636 = (1f - _e1635);
                                    phi_56_ = vec4<f32>(((_e1477.x * _e1636) + (((((_e1583.x * _e1587) + _e1591) + _e1618) * _e1599) * _e1497)), ((_e1477.y * _e1636) + (((((_e1583.y * _e1587) + _e1591) + _e1618) * _e1599) * _e1497)), ((_e1477.z * _e1636) + (((((_e1583.z * _e1587) + _e1591) + _e1618) * _e1599) * _e1497)), ((_e1477.w * _e1636) + _e1635));
                                } else {
                                    phi_56_ = _e1477;
                                }
                                let _e1651 = phi_56_;
                                phi_57_ = _e1651;
                            } else {
                                phi_57_ = _e1477;
                            }
                            let _e1653 = phi_57_;
                            phi_58_ = _e1653;
                        } else {
                            phi_58_ = _e1477;
                        }
                        let _e1655 = phi_58_;
                        phi_59_ = _e1655;
                    } else {
                        phi_59_ = _e1477;
                    }
                    let _e1657 = phi_59_;
                    phi_60_ = _e1657;
                    phi_61_ = (_e1479 + 1u);
                } else {
                    phi_60_ = vec4<f32>();
                    phi_61_ = u32();
                }
                let _e1660 = phi_60_;
                let _e1662 = phi_61_;
                continue;
                continuing {
                    phi_47_ = _e1660;
                    phi_48_ = _e1662;
                    phi_62_ = _e329;
                    break if !(_e1480);
                }
            }
            let _e1665 = phi_62_;
            if _e1665 {
                break;
            }
            let _e1670 = pill.member[_e166].lines[0u];
            let _e1672 = (1f / _e1670.size);
            let _e1679 = ((_e618 - _e1670.origin.x) * _e1672);
            phi_63_ = 0u;
            phi_64_ = _e1670.count;
            loop {
                let _e1684 = phi_63_;
                let _e1686 = phi_64_;
                local_9 = _e1684;
                let _e1687 = (_e1684 < _e1686);
                if _e1687 {
                    let _e1690 = (_e1684 + ((_e1686 - _e1684) / 2u));
                    let _e1695 = placed_glyphs.member[(_e1670.first + _e1690)].x;
                    let _e1696 = (_e1695 <= _e1679);
                    if _e1696 {
                        phi_65_ = (_e1690 + 1u);
                    } else {
                        phi_65_ = _e1684;
                    }
                    let _e1699 = phi_65_;
                    phi_66_ = _e1699;
                    phi_67_ = select(_e1690, _e1686, _e1696);
                } else {
                    phi_66_ = u32();
                    phi_67_ = u32();
                }
                let _e1702 = phi_66_;
                let _e1704 = phi_67_;
                continue;
                continuing {
                    phi_63_ = _e1702;
                    phi_64_ = _e1704;
                    break if !(_e1687);
                }
            }
            let _e1706 = (3.5f / _e1670.size);
            let _e1708 = local_9;
            let _e1709 = (_e1708 + 1u);
            phi_68_ = select(_e1709, _e1670.count, (_e1670.count < _e1709));
            phi_69_ = -1000000f;
            loop {
                let _e1713 = phi_68_;
                let _e1715 = phi_69_;
                local_12 = _e1715;
                if (_e1713 > 0u) {
                    let _e1717 = (_e1713 - 1u);
                    let _e1718 = (_e1670.first + _e1717);
                    let _e1722 = placed_glyphs.member[_e1718].x;
                    let _e1726 = placed_glyphs.member[_e1718].glyph;
                    let _e1731 = glyphs.member[_e1726].min[0u];
                    let _e1736 = glyphs.member[_e1726].min[1u];
                    let _e1741 = glyphs.member[_e1726].max[0u];
                    let _e1746 = glyphs.member[_e1726].max[1u];
                    let _e1750 = glyphs.member[_e1726].start;
                    let _e1754 = glyphs.member[_e1726].count;
                    let _e1755 = (_e1679 - _e1722);
                    let _e1756 = -(((_e619 - _e1670.origin.y) * _e1672));
                    let _e1757 = (_e1741 + _e1706);
                    let _e1758 = (_e1755 > _e1757);
                    if _e1758 {
                        phi_82_ = f32();
                    } else {
                        if (_e1755 >= (_e1731 - _e1706)) {
                            if (_e1756 >= (_e1736 - _e1706)) {
                                if (_e1755 <= _e1757) {
                                    if (_e1756 <= (_e1746 + _e1706)) {
                                        phi_70_ = 340282350000000000000000000000000000000f;
                                        phi_71_ = 0u;
                                        phi_72_ = 0i;
                                        loop {
                                            let _e1768 = phi_70_;
                                            let _e1770 = phi_71_;
                                            let _e1772 = phi_72_;
                                            local_10 = _e1768;
                                            local_11 = _e1772;
                                            let _e1773 = (_e1770 < _e1754);
                                            if _e1773 {
                                                let _e1777 = edges.member[(_e1750 + _e1770)];
                                                let _e1779 = cantus_render_text_edge_distance(_e1777, _e1670.weight, vec2<f32>(_e1755, _e1756), _e1768);
                                                phi_73_ = _e1779.member;
                                                phi_74_ = (_e1770 + 1u);
                                                phi_75_ = (_e1772 + _e1779.member_1);
                                            } else {
                                                phi_73_ = f32();
                                                phi_74_ = u32();
                                                phi_75_ = i32();
                                            }
                                            let _e1785 = phi_73_;
                                            let _e1787 = phi_74_;
                                            let _e1789 = phi_75_;
                                            continue;
                                            continuing {
                                                phi_70_ = _e1785;
                                                phi_71_ = _e1787;
                                                phi_72_ = _e1789;
                                                break if !(_e1773);
                                            }
                                        }
                                        let _e1792 = local_10;
                                        let _e1794 = ((_e1792 * _e1670.size) * _e1670.size);
                                        if (_e1794 >= 12.25f) {
                                            phi_76_ = 3.5f;
                                        } else {
                                            phi_76_ = sqrt(_e1794);
                                        }
                                        let _e1798 = phi_76_;
                                        let _e1800 = local_11;
                                        let _e1803 = (_e1798 * select(1f, -1f, (_e1800 == 0i)));
                                        if (_e1715 != _e1715) {
                                            phi_77_ = true;
                                        } else {
                                            phi_77_ = (_e1803 >= _e1715);
                                        }
                                        let _e1807 = phi_77_;
                                        phi_78_ = select(_e1715, _e1803, _e1807);
                                    } else {
                                        phi_78_ = _e1715;
                                    }
                                    let _e1810 = phi_78_;
                                    phi_79_ = _e1810;
                                } else {
                                    phi_79_ = _e1715;
                                }
                                let _e1812 = phi_79_;
                                phi_80_ = _e1812;
                            } else {
                                phi_80_ = _e1715;
                            }
                            let _e1814 = phi_80_;
                            phi_81_ = _e1814;
                        } else {
                            phi_81_ = _e1715;
                        }
                        let _e1816 = phi_81_;
                        phi_82_ = _e1816;
                    }
                    let _e1818 = phi_82_;
                    phi_83_ = _e1717;
                    phi_84_ = _e1818;
                    phi_85_ = select(true, false, _e1758);
                } else {
                    phi_83_ = u32();
                    phi_84_ = f32();
                    phi_85_ = false;
                }
                let _e1821 = phi_83_;
                let _e1823 = phi_84_;
                let _e1825 = phi_85_;
                continue;
                continuing {
                    phi_68_ = _e1821;
                    phi_69_ = _e1823;
                    break if !(_e1825);
                }
            }
            let _e1828 = local_12;
            let _e1830 = ((_e1828 * 1.25f) + 0.5f);
            let _e1832 = select(_e1830, 0f, (_e1830 < 0f));
            let _e1834 = select(_e1832, 1f, (_e1832 > 1f));
            let _e1838 = ((_e1834 * _e1834) * (3f - (2f * _e1834)));
            let _e1843 = pill.member[_e166].lines[1u];
            let _e1845 = (1f / _e1843.size);
            let _e1852 = ((_e618 - _e1843.origin.x) * _e1845);
            phi_86_ = 0u;
            phi_87_ = _e1843.count;
            loop {
                let _e1857 = phi_86_;
                let _e1859 = phi_87_;
                local_13 = _e1857;
                let _e1860 = (_e1857 < _e1859);
                if _e1860 {
                    let _e1863 = (_e1857 + ((_e1859 - _e1857) / 2u));
                    let _e1868 = placed_glyphs.member[(_e1843.first + _e1863)].x;
                    let _e1869 = (_e1868 <= _e1852);
                    if _e1869 {
                        phi_88_ = (_e1863 + 1u);
                    } else {
                        phi_88_ = _e1857;
                    }
                    let _e1872 = phi_88_;
                    phi_89_ = _e1872;
                    phi_90_ = select(_e1863, _e1859, _e1869);
                } else {
                    phi_89_ = u32();
                    phi_90_ = u32();
                }
                let _e1875 = phi_89_;
                let _e1877 = phi_90_;
                continue;
                continuing {
                    phi_86_ = _e1875;
                    phi_87_ = _e1877;
                    break if !(_e1860);
                }
            }
            let _e1879 = (3.5f / _e1843.size);
            let _e1881 = local_13;
            let _e1882 = (_e1881 + 1u);
            phi_91_ = select(_e1882, _e1843.count, (_e1843.count < _e1882));
            phi_92_ = -1000000f;
            loop {
                let _e1886 = phi_91_;
                let _e1888 = phi_92_;
                local_16 = _e1888;
                if (_e1886 > 0u) {
                    let _e1890 = (_e1886 - 1u);
                    let _e1891 = (_e1843.first + _e1890);
                    let _e1895 = placed_glyphs.member[_e1891].x;
                    let _e1899 = placed_glyphs.member[_e1891].glyph;
                    let _e1904 = glyphs.member[_e1899].min[0u];
                    let _e1909 = glyphs.member[_e1899].min[1u];
                    let _e1914 = glyphs.member[_e1899].max[0u];
                    let _e1919 = glyphs.member[_e1899].max[1u];
                    let _e1923 = glyphs.member[_e1899].start;
                    let _e1927 = glyphs.member[_e1899].count;
                    let _e1928 = (_e1852 - _e1895);
                    let _e1929 = -(((_e619 - _e1843.origin.y) * _e1845));
                    let _e1930 = (_e1914 + _e1879);
                    let _e1931 = (_e1928 > _e1930);
                    if _e1931 {
                        phi_105_ = f32();
                    } else {
                        if (_e1928 >= (_e1904 - _e1879)) {
                            if (_e1929 >= (_e1909 - _e1879)) {
                                if (_e1928 <= _e1930) {
                                    if (_e1929 <= (_e1919 + _e1879)) {
                                        phi_93_ = 340282350000000000000000000000000000000f;
                                        phi_94_ = 0u;
                                        phi_95_ = 0i;
                                        loop {
                                            let _e1941 = phi_93_;
                                            let _e1943 = phi_94_;
                                            let _e1945 = phi_95_;
                                            local_14 = _e1941;
                                            local_15 = _e1945;
                                            let _e1946 = (_e1943 < _e1927);
                                            if _e1946 {
                                                let _e1950 = edges.member[(_e1923 + _e1943)];
                                                let _e1952 = cantus_render_text_edge_distance(_e1950, _e1843.weight, vec2<f32>(_e1928, _e1929), _e1941);
                                                phi_96_ = _e1952.member;
                                                phi_97_ = (_e1943 + 1u);
                                                phi_98_ = (_e1945 + _e1952.member_1);
                                            } else {
                                                phi_96_ = f32();
                                                phi_97_ = u32();
                                                phi_98_ = i32();
                                            }
                                            let _e1958 = phi_96_;
                                            let _e1960 = phi_97_;
                                            let _e1962 = phi_98_;
                                            continue;
                                            continuing {
                                                phi_93_ = _e1958;
                                                phi_94_ = _e1960;
                                                phi_95_ = _e1962;
                                                break if !(_e1946);
                                            }
                                        }
                                        let _e1965 = local_14;
                                        let _e1967 = ((_e1965 * _e1843.size) * _e1843.size);
                                        if (_e1967 >= 12.25f) {
                                            phi_99_ = 3.5f;
                                        } else {
                                            phi_99_ = sqrt(_e1967);
                                        }
                                        let _e1971 = phi_99_;
                                        let _e1973 = local_15;
                                        let _e1976 = (_e1971 * select(1f, -1f, (_e1973 == 0i)));
                                        if (_e1888 != _e1888) {
                                            phi_100_ = true;
                                        } else {
                                            phi_100_ = (_e1976 >= _e1888);
                                        }
                                        let _e1980 = phi_100_;
                                        phi_101_ = select(_e1888, _e1976, _e1980);
                                    } else {
                                        phi_101_ = _e1888;
                                    }
                                    let _e1983 = phi_101_;
                                    phi_102_ = _e1983;
                                } else {
                                    phi_102_ = _e1888;
                                }
                                let _e1985 = phi_102_;
                                phi_103_ = _e1985;
                            } else {
                                phi_103_ = _e1888;
                            }
                            let _e1987 = phi_103_;
                            phi_104_ = _e1987;
                        } else {
                            phi_104_ = _e1888;
                        }
                        let _e1989 = phi_104_;
                        phi_105_ = _e1989;
                    }
                    let _e1991 = phi_105_;
                    phi_106_ = _e1890;
                    phi_107_ = _e1991;
                    phi_108_ = select(true, false, _e1931);
                } else {
                    phi_106_ = u32();
                    phi_107_ = f32();
                    phi_108_ = false;
                }
                let _e1994 = phi_106_;
                let _e1996 = phi_107_;
                let _e1998 = phi_108_;
                continue;
                continuing {
                    phi_91_ = _e1994;
                    phi_92_ = _e1996;
                    break if !(_e1998);
                }
            }
            let _e2001 = local_16;
            let _e2003 = ((_e2001 * 1.25f) + 0.5f);
            let _e2005 = select(_e2003, 0f, (_e2003 < 0f));
            let _e2007 = select(_e2005, 1f, (_e2005 > 1f));
            let _e2011 = ((_e2007 * _e2007) * (3f - (2f * _e2007)));
            if (_e1838 != _e1838) {
                phi_109_ = true;
            } else {
                phi_109_ = (_e2011 >= _e1838);
            }
            let _e2015 = phi_109_;
            let _e2020 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e618 - _e1122), (_e619 - _e184)), 0f, _e184);
            let _e2022 = ((_e2020 - 2f) * 0.0625f);
            let _e2024 = select(_e2022, 0f, (_e2022 < 0f));
            let _e2026 = select(_e2024, 1f, (_e2024 > 1f));
            let _e2032 = ((select(_e1838, _e2011, _e2015) * ((_e2026 * _e2026) * (3f - (2f * _e2026)))) * _e578);
            let _e2033 = (1f - _e2032);
            let _e2035 = local_17;
            let _e2039 = local_18;
            let _e2043 = local_19;
            let _e2047 = local_20;
            let _e2050 = (0.94f * _e2032);
            let _e2058 = (((_e2047.w * _e2033) + _e2032) * _e595);
            if (_e2058 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e2035.x * _e2033) + _e2050) * _e595), (((_e2039.y * _e2033) + _e2050) * _e595), (((_e2043.z * _e2033) + _e2050) * _e595), _e2058);
            break;
        }
    }
    return;
}

fn render_lyrics_isthmus_lyricspass_vertex_impl() {
    let _e14 = vertex_7;
    let _e15 = _isthmus_instance_index_9;
    let _e18 = line.member[_e15];
    let _e21 = ((_e18.size * 0.20000005f) + 1f);
    let _e26 = (_e18.min.x - _e21);
    let _e27 = (_e18.min.y - _e21);
    let _e43 = (_e26 + (f32((_e14 & 1u)) * ((_e18.max.x + _e21) - _e26)));
    let _e44 = (_e27 + (f32((_e14 >> bitcast<u32>(1i))) * ((_e18.max.y + _e21) - _e27)));
    let _e49 = frame.member[0u].screen_size[0u];
    let _e54 = frame.member[0u].screen_size[1u];
    out_position = vec4<f32>((((_e43 / _e49) * 2f) - 1f), (1f - ((_e44 / _e54) * 2f)), 0f, 1f);
    out_pixel[0u] = _e43;
    out_pixel[1u] = _e44;
    out_isthmus_instance_index = _e15;
    return;
}

fn render_lyrics_isthmus_lyricspass_fragment_impl() {
    var phi_0_: bool;
    var phi_1_: u32;
    var phi_2_: u32;
    var phi_3_: u32;
    var phi_4_: u32;
    var phi_5_: u32;
    var local_22: u32;
    var phi_6_: u32;
    var phi_7_: f32;
    var phi_8_: f32;
    var phi_9_: u32;
    var phi_10_: i32;
    var phi_11_: f32;
    var phi_12_: u32;
    var phi_13_: i32;
    var local_23: f32;
    var phi_14_: f32;
    var local_24: i32;
    var phi_15_: bool;
    var phi_16_: f32;
    var phi_17_: f32;
    var phi_18_: f32;
    var phi_19_: f32;
    var phi_20_: f32;
    var phi_21_: u32;
    var phi_22_: f32;
    var phi_23_: bool;
    var local_25: f32;
    var local_26: f32;

    let _e38 = pixel_4;
    let _e39 = _isthmus_instance_index_10;
    let _e44 = (_e38.x * 0.03125f);
    let _e46 = select(_e44, 0f, (_e44 < 0f));
    let _e48 = select(_e46, 1f, (_e46 > 1f));
    let _e57 = frame.member[0u].screen_size[0u];
    let _e61 = ((_e38.x - _e57) / ((_e57 - 32f) - _e57));
    let _e63 = select(_e61, 0f, (_e61 < 0f));
    let _e65 = select(_e63, 1f, (_e63 > 1f));
    let _e70 = (((_e48 * _e48) * (3f - (2f * _e48))) * ((_e65 * _e65) * (3f - (2f * _e65))));
    let _e74 = frame.member[0u].playhead_x;
    let _e78 = ((abs((_e38.x - _e74)) - 110f) * -0.009090909f);
    let _e80 = select(_e78, 0f, (_e78 < 0f));
    let _e82 = select(_e80, 1f, (_e80 > 1f));
    let _e86 = ((_e82 * _e82) * (3f - (2f * _e82)));
    let _e87 = line.member[_e39];
    let _e90 = (_e87.weight + (_e86 * 0.15f));
    if (_e90 != _e90) {
        phi_0_ = true;
    } else {
        phi_0_ = (1f <= _e90);
    }
    let _e94 = phi_0_;
    let _e97 = (1f + (_e86 * 0.2f));
    let _e99 = (1f / _e87.size);
    let _e106 = ((_e38.x - _e87.origin.x) * _e99);
    phi_1_ = 0u;
    phi_2_ = _e87.count;
    loop {
        let _e111 = phi_1_;
        let _e113 = phi_2_;
        local_22 = _e111;
        let _e114 = (_e111 < _e113);
        if _e114 {
            let _e117 = (_e111 + ((_e113 - _e111) / 2u));
            let _e122 = placed_glyphs_1.member[(_e87.first + _e117)].x;
            let _e123 = (_e122 <= _e106);
            if _e123 {
                phi_3_ = (_e117 + 1u);
            } else {
                phi_3_ = _e111;
            }
            let _e126 = phi_3_;
            phi_4_ = _e126;
            phi_5_ = select(_e117, _e113, _e123);
        } else {
            phi_4_ = u32();
            phi_5_ = u32();
        }
        let _e129 = phi_4_;
        let _e131 = phi_5_;
        continue;
        continuing {
            phi_1_ = _e129;
            phi_2_ = _e131;
            break if !(_e114);
        }
    }
    let _e134 = ((3.5f / _e87.size) / _e97);
    let _e136 = local_22;
    let _e137 = (_e136 + 1u);
    phi_6_ = select(_e137, _e87.count, (_e87.count < _e137));
    phi_7_ = -1000000f;
    loop {
        let _e141 = phi_6_;
        let _e143 = phi_7_;
        local_25 = _e143;
        local_26 = _e143;
        if (_e141 > 0u) {
            let _e145 = (_e141 - 1u);
            let _e146 = (_e87.first + _e145);
            let _e150 = placed_glyphs_1.member[_e146].x;
            let _e154 = placed_glyphs_1.member[_e146].glyph;
            let _e159 = glyphs_1.member[_e154].min[0u];
            let _e164 = glyphs_1.member[_e154].min[1u];
            let _e169 = glyphs_1.member[_e154].max[0u];
            let _e174 = glyphs_1.member[_e154].max[1u];
            let _e178 = glyphs_1.member[_e154].start;
            let _e182 = glyphs_1.member[_e154].count;
            let _e185 = ((_e106 - _e150) / _e97);
            let _e186 = (-(((_e38.y - _e87.origin.y) * _e99)) / _e97);
            let _e187 = (_e169 + _e134);
            let _e188 = (_e185 > _e187);
            if _e188 {
                phi_20_ = f32();
            } else {
                if (_e185 >= (_e159 - _e134)) {
                    if (_e186 >= (_e164 - _e134)) {
                        if (_e185 <= _e187) {
                            if (_e186 <= (_e174 + _e134)) {
                                let _e196 = (_e87.size * _e97);
                                phi_8_ = 340282350000000000000000000000000000000f;
                                phi_9_ = 0u;
                                phi_10_ = 0i;
                                loop {
                                    let _e198 = phi_8_;
                                    let _e200 = phi_9_;
                                    let _e202 = phi_10_;
                                    local_23 = _e198;
                                    local_24 = _e202;
                                    let _e203 = (_e200 < _e182);
                                    if _e203 {
                                        let _e207 = edges_1.member[(_e178 + _e200)];
                                        let _e209 = cantus_render_text_edge_distance(_e207, select(_e90, 1f, _e94), vec2<f32>(_e185, _e186), _e198);
                                        phi_11_ = _e209.member;
                                        phi_12_ = (_e200 + 1u);
                                        phi_13_ = (_e202 + _e209.member_1);
                                    } else {
                                        phi_11_ = f32();
                                        phi_12_ = u32();
                                        phi_13_ = i32();
                                    }
                                    let _e215 = phi_11_;
                                    let _e217 = phi_12_;
                                    let _e219 = phi_13_;
                                    continue;
                                    continuing {
                                        phi_8_ = _e215;
                                        phi_9_ = _e217;
                                        phi_10_ = _e219;
                                        break if !(_e203);
                                    }
                                }
                                let _e222 = local_23;
                                let _e224 = ((_e222 * _e196) * _e196);
                                if (_e224 >= 12.25f) {
                                    phi_14_ = 3.5f;
                                } else {
                                    phi_14_ = sqrt(_e224);
                                }
                                let _e228 = phi_14_;
                                let _e230 = local_24;
                                let _e233 = (_e228 * select(1f, -1f, (_e230 == 0i)));
                                if (_e143 != _e143) {
                                    phi_15_ = true;
                                } else {
                                    phi_15_ = (_e233 >= _e143);
                                }
                                let _e237 = phi_15_;
                                phi_16_ = select(_e143, _e233, _e237);
                            } else {
                                phi_16_ = _e143;
                            }
                            let _e240 = phi_16_;
                            phi_17_ = _e240;
                        } else {
                            phi_17_ = _e143;
                        }
                        let _e242 = phi_17_;
                        phi_18_ = _e242;
                    } else {
                        phi_18_ = _e143;
                    }
                    let _e244 = phi_18_;
                    phi_19_ = _e244;
                } else {
                    phi_19_ = _e143;
                }
                let _e246 = phi_19_;
                phi_20_ = _e246;
            }
            let _e248 = phi_20_;
            phi_21_ = _e145;
            phi_22_ = _e248;
            phi_23_ = select(true, false, _e188);
        } else {
            phi_21_ = u32();
            phi_22_ = f32();
            phi_23_ = false;
        }
        let _e251 = phi_21_;
        let _e253 = phi_22_;
        let _e255 = phi_23_;
        continue;
        continuing {
            phi_6_ = _e251;
            phi_7_ = _e253;
            break if !(_e255);
        }
    }
    let _e258 = local_25;
    let _e260 = ((_e258 * 1.25f) + 0.5f);
    let _e262 = select(_e260, 0f, (_e260 < 0f));
    let _e264 = select(_e262, 1f, (_e262 > 1f));
    let _e268 = ((_e264 * _e264) * (3f - (2f * _e264)));
    let _e270 = local_26;
    let _e273 = (((_e270 + 0.9f) * 1.25f) + 0.5f);
    let _e275 = select(_e273, 0f, (_e273 < 0f));
    let _e277 = select(_e275, 1f, (_e275 > 1f));
    let _e286 = (_e74 + 4f);
    let _e290 = ((_e38.x - _e286) / ((_e74 - 4f) - _e286));
    let _e292 = select(_e290, 0f, (_e290 < 0f));
    let _e294 = select(_e292, 1f, (_e292 > 1f));
    let _e298 = ((_e294 * _e294) * (3f - (2f * _e294)));
    let _e302 = line.member[_e39].color;
    let _e303 = unpack4x8unorm(_e302);
    let _e310 = (1f - _e298);
    out_color = vec4<f32>(((((_e303.x * _e310) + ((_e303.x * 0.42f) * _e298)) * _e268) * _e70), ((((_e303.y * _e310) + ((_e303.y * 0.42f) * _e298)) * _e268) * _e70), ((((_e303.z * _e310) + ((_e303.z * 0.42f) * _e298)) * _e268) * _e70), ((_e268 + ((((_e277 * _e277) * (3f - (2f * _e277))) * 0.4f) * (1f - _e268))) * _e70));
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
            let _e28 = vertex_7;
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
            out_position = vec4<f32>((((_e113 / _e44) * 2f) - 1f), (1f - ((_e114 / _e119) * 2f)), 0f, 1f);
            out_pixel[0u] = _e113;
            out_pixel[1u] = _e114;
            out_isthmus_instance_index = _e29;
            break;
        }
    }
    return;
}

fn cantus_render_shader_sd_rounded_box(param_14: vec2<f32>, param_15: vec2<f32>, param_16: f32) -> f32 {
    var phi_0_: bool;
    var phi_1_: bool;

    let _e13 = ((abs(param_14.x) - param_15.x) + param_16);
    let _e14 = ((abs(param_14.y) - param_15.y) + param_16);
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
    return ((sqrt(((_e16 * _e16) + (_e18 * _e18))) + select(_e27, 0f, _e31)) - param_16);
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
    var phi_22_: i32;
    var phi_23_: f32;
    var phi_24_: f32;
    var phi_25_: vec2<f32>;
    var phi_26_: i32;
    var phi_27_: f32;
    var phi_28_: f32;
    var phi_29_: vec2<f32>;
    var local_32: f32;
    var phi_30_: i32;
    var phi_31_: f32;
    var phi_32_: f32;
    var phi_33_: vec2<f32>;
    var phi_34_: i32;
    var phi_35_: f32;
    var phi_36_: f32;
    var phi_37_: vec2<f32>;
    var local_33: f32;
    var local_34: f32;
    var phi_38_: vec3<f32>;
    var phi_39_: vec3<f32>;
    var phi_40_: vec3<f32>;
    var phi_41_: vec3<f32>;
    var phi_42_: i32;
    var phi_43_: f32;
    var phi_44_: f32;
    var phi_45_: vec2<f32>;
    var phi_46_: i32;
    var phi_47_: f32;
    var phi_48_: f32;
    var phi_49_: vec2<f32>;
    var local_35: f32;
    var phi_50_: vec3<f32>;
    var phi_51_: bool;
    var phi_52_: bool;
    var phi_53_: bool;
    var phi_54_: bool;
    var phi_55_: u32;
    var phi_56_: u32;
    var phi_57_: u32;
    var phi_58_: u32;
    var phi_59_: bool;
    var phi_60_: f32;
    var phi_61_: bool;
    var phi_62_: bool;
    var phi_63_: bool;
    var phi_64_: vec2<f32>;
    var phi_65_: bool;
    var phi_66_: i32;
    var phi_67_: f32;
    var phi_68_: f32;
    var phi_69_: vec2<f32>;
    var phi_70_: i32;
    var phi_71_: f32;
    var phi_72_: f32;
    var phi_73_: vec2<f32>;
    var local_36: f32;
    var phi_74_: vec2<f32>;
    var phi_75_: i32;
    var phi_76_: f32;
    var phi_77_: f32;
    var phi_78_: vec2<f32>;
    var phi_79_: i32;
    var phi_80_: f32;
    var phi_81_: f32;
    var phi_82_: vec2<f32>;
    var local_37: f32;
    var phi_83_: vec2<f32>;
    var phi_84_: vec2<f32>;
    var phi_85_: bool;
    var phi_86_: bool;
    var phi_87_: bool;
    var phi_88_: bool;
    var phi_89_: bool;
    var phi_90_: bool;
    var phi_91_: bool;
    var phi_92_: bool;
    var phi_93_: bool;
    var phi_94_: bool;
    var phi_95_: bool;
    var phi_96_: bool;
    var phi_97_: bool;
    var phi_98_: bool;
    var phi_99_: bool;
    var phi_100_: bool;
    var phi_101_: bool;
    var phi_102_: bool;
    var phi_103_: vec3<f32>;
    var phi_104_: bool;
    var phi_105_: bool;
    var phi_106_: bool;
    var phi_107_: bool;
    var phi_108_: bool;
    var phi_109_: f32;
    var phi_110_: bool;
    var phi_111_: vec3<f32>;
    var local_38: f32;
    var local_39: f32;
    var phi_112_: render_text_Line;
    var phi_113_: bool;
    var phi_114_: u32;
    var phi_115_: u32;
    var phi_116_: u32;
    var phi_117_: u32;
    var phi_118_: u32;
    var local_40: u32;
    var phi_119_: u32;
    var phi_120_: f32;
    var phi_121_: f32;
    var phi_122_: u32;
    var phi_123_: i32;
    var phi_124_: f32;
    var phi_125_: u32;
    var phi_126_: i32;
    var local_41: f32;
    var phi_127_: f32;
    var local_42: i32;
    var phi_128_: bool;
    var phi_129_: f32;
    var phi_130_: f32;
    var phi_131_: f32;
    var phi_132_: f32;
    var phi_133_: f32;
    var phi_134_: u32;
    var phi_135_: f32;
    var phi_136_: bool;
    var local_43: f32;
    var phi_137_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e280 = pixel_4;
            let _e281 = _isthmus_instance_index_10;
            let _e287 = pill_1.member[_e281].battery_level;
            let _e288 = (_e287 >= -1f);
            if _e288 {
                phi_0_ = (_e287 <= 1f);
            } else {
                phi_0_ = false;
            }
            let _e291 = phi_0_;
            let _e293 = (select(0f, 40f, _e291) + 296f);
            let _e298 = frame.member[0u].screen_size[0u];
            let _e300 = ((_e298 - _e293) - 8f);
            let _e304 = frame.member[0u].panel_height;
            let _e305 = (_e280.x - _e300);
            let _e306 = (_e280.y - 6f);
            let _e307 = (_e293 * 0.5f);
            let _e308 = (_e304 * 0.5f);
            let _e312 = ((_e293 - _e304) * 0.5f);
            let _e314 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e305 - _e307), (_e306 - _e308)), _e312, _e308);
            let _e318 = frame.member[0u].mouse_pressure;
            let _e319 = (_e318 > 0f);
            if _e319 {
                let _e324 = frame.member[0u].mouse_pos[0u];
                let _e329 = frame.member[0u].mouse_pos[1u];
                let _e335 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e324 - _e300) - _e307), ((_e329 - 6f) - _e308)), _e312, _e308);
                phi_1_ = _e335;
            } else {
                phi_1_ = 1f;
            }
            let _e337 = phi_1_;
            phi_2_ = vec2<f32>(0f, 0f);
            phi_3_ = 0f;
            phi_4_ = 0u;
            loop {
                let _e339 = phi_2_;
                let _e341 = phi_3_;
                let _e343 = phi_4_;
                local_28 = _e339;
                local_29 = _e339;
                local_30 = _e339;
                local_31 = _e339;
                local_38 = _e341;
                local_39 = _e341;
                let _e344 = (_e343 < 4u);
                if _e344 {
                    if _e344 {
                    } else {
                        phi_14_ = true;
                        break;
                    }
                    let _e351 = frame.member[0u].ripples[_e343].origin[0u];
                    let _e358 = frame.member[0u].ripples[_e343].origin[1u];
                    let _e364 = frame.member[0u].ripples[_e343].start_time;
                    let _e370 = frame.member[0u].ripples[_e343].strength;
                    let _e374 = frame.member[0u].time;
                    let _e376 = ((_e374 - _e364) * 1.2f);
                    let _e378 = select(_e376, 0f, (_e376 < 0f));
                    let _e380 = select(_e378, 1f, (_e378 > 1f));
                    if (_e370 > 0f) {
                        if (_e380 < 1f) {
                            let _e384 = (_e280 - vec2<f32>(_e351, _e358));
                            let _e390 = sqrt(((_e384.x * _e384.x) + (_e384.y * _e384.y)));
                            if (_e390 > 0.001f) {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e384.x / _e390), (_e384.y / _e390)), _e390);
                            } else {
                                phi_5_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e390);
                            }
                            let _e398 = phi_5_;
                            let _e408 = ((abs((_e398.unnamed_1 - (_e380 * 600f))) - 80f) * -0.0125f);
                            let _e410 = select(_e408, 0f, (_e408 < 0f));
                            let _e412 = select(_e410, 1f, (_e410 > 1f));
                            let _e418 = (1f - _e380);
                            let _e419 = ((((_e412 * _e412) * (3f - (2f * _e412))) * _e370) * _e418);
                            let _e432 = (_e341 + (_e419 * 0.5f));
                            if (_e432 != _e432) {
                                phi_6_ = true;
                            } else {
                                phi_6_ = (1f <= _e432);
                            }
                            let _e436 = phi_6_;
                            phi_7_ = vec2<f32>((_e339.x + (((_e398.unnamed.x * _e419) * _e418) * 0.5f)), (_e339.y + (((_e398.unnamed.y * _e419) * _e418) * 0.5f)));
                            phi_8_ = select(_e432, 1f, _e436);
                        } else {
                            phi_7_ = _e339;
                            phi_8_ = _e341;
                        }
                        let _e439 = phi_7_;
                        let _e441 = phi_8_;
                        phi_9_ = _e439;
                        phi_10_ = _e441;
                    } else {
                        phi_9_ = _e339;
                        phi_10_ = _e341;
                    }
                    let _e443 = phi_9_;
                    let _e445 = phi_10_;
                    phi_11_ = _e443;
                    phi_12_ = _e445;
                    phi_13_ = (_e343 + 1u);
                } else {
                    phi_11_ = vec2<f32>();
                    phi_12_ = f32();
                    phi_13_ = u32();
                }
                let _e448 = phi_11_;
                let _e450 = phi_12_;
                let _e452 = phi_13_;
                continue;
                continuing {
                    phi_2_ = _e448;
                    phi_3_ = _e450;
                    phi_4_ = _e452;
                    phi_14_ = false;
                    break if !(_e344);
                }
            }
            let _e455 = phi_14_;
            if _e455 {
                break;
            }
            if _e319 {
                let _e460 = frame.member[0u].mouse_pos[0u];
                let _e465 = frame.member[0u].mouse_pos[1u];
                let _e466 = (_e280.x - _e460);
                let _e467 = (_e280.y - _e465);
                let _e473 = ((sqrt(((_e466 * _e466) + (_e467 * _e467))) - 150f) * -0.006666667f);
                let _e475 = select(_e473, 0f, (_e473 < 0f));
                let _e477 = select(_e475, 1f, (_e475 > 1f));
                phi_15_ = ((((_e477 * _e477) * (3f - (2f * _e477))) * _e318) * 8f);
            } else {
                phi_15_ = 0f;
            }
            let _e485 = phi_15_;
            let _e487 = local_28;
            let _e490 = global[0u];
            if (_e487.x == _e490) {
                let _e493 = local_29;
                let _e496 = global[1u];
                phi_16_ = (_e493.y == _e496);
            } else {
                phi_16_ = false;
            }
            let _e499 = phi_16_;
            if _e499 {
                phi_17_ = 0f;
            } else {
                let _e501 = local_30;
                phi_17_ = (sqrt(((_e487.x * _e487.x) + (_e501.y * _e501.y))) * 22f);
            }
            let _e509 = phi_17_;
            let _e511 = local_31;
            let _e514 = ((_e337 - 0.5f) * -1f);
            let _e516 = select(_e514, 0f, (_e514 < 0f));
            let _e518 = select(_e516, 1f, (_e516 > 1f));
            let _e526 = (_e314 - (((_e485 * ((_e518 * _e518) * (3f - (2f * _e518)))) + _e509) * 0.5f));
            let _e527 = fwidth(_e526);
            if (_e527 != _e527) {
                phi_18_ = true;
            } else {
                phi_18_ = (0.55f >= _e527);
            }
            let _e531 = phi_18_;
            let _e532 = select(_e527, 0.55f, _e531);
            let _e536 = ((_e526 - _e532) / (-(_e532) - _e532));
            let _e538 = select(_e536, 0f, (_e536 < 0f));
            let _e540 = select(_e538, 1f, (_e538 > 1f));
            let _e544 = ((_e540 * _e540) * (3f - (2f * _e540)));
            let _e545 = (_e526 != _e526);
            if _e545 {
                phi_19_ = true;
            } else {
                phi_19_ = (0f >= _e526);
            }
            let _e548 = phi_19_;
            let _e552 = (exp((select(_e526, 0f, _e548) * -0.3f)) * 0.16f);
            if (_e544 != _e544) {
                phi_20_ = true;
            } else {
                phi_20_ = (_e552 >= _e544);
            }
            let _e556 = phi_20_;
            let _e557 = select(_e544, _e552, _e556);
            if (_e557 <= 0.0009765625f) {
                discard;
            }
            let _e559 = (_e305 / _e293);
            let _e560 = (_e306 / _e304);
            if _e545 {
                phi_21_ = true;
            } else {
                phi_21_ = (0f <= _e526);
            }
            let _e565 = phi_21_;
            let _e568 = (1f + (select(_e526, 0f, _e565) * 0.008333334f));
            let _e570 = select(_e568, 0f, (_e568 < 0f));
            let _e572 = select(_e570, 0.6f, (_e570 > 0.6f));
            let _e581 = ((_e559 - (((_e559 - 0.5f) * _e572) * 0.08f)) - (_e487.x * 0.04f));
            let _e582 = ((_e560 - (((_e560 - 0.5f) * _e572) * 0.08f)) - (_e511.y * 0.04f));
            let _e583 = (_e581 * _e293);
            let _e584 = (_e582 * _e304);
            let _e588 = pill_1.member[_e281].sun_height;
            let _e590 = ((_e588 - -0.04f) * 4.1666665f);
            let _e592 = select(_e590, 0f, (_e590 < 0f));
            let _e594 = select(_e592, 1f, (_e592 > 1f));
            let _e598 = ((_e594 * _e594) * (3f - (2f * _e594)));
            let _e600 = ((_e588 - -0.32f) * 4.166667f);
            let _e602 = select(_e600, 0f, (_e600 < 0f));
            let _e604 = select(_e602, 1f, (_e602 > 1f));
            let _e609 = (1f - _e598);
            let _e612 = ((_e588 - -0.18f) * 5.5555553f);
            let _e614 = select(_e612, 0f, (_e612 < 0f));
            let _e616 = select(_e614, 1f, (_e614 > 1f));
            let _e622 = ((_e588 - 0.2f) * -5.5555553f);
            let _e624 = select(_e622, 0f, (_e622 < 0f));
            let _e626 = select(_e624, 1f, (_e624 > 1f));
            let _e631 = (((_e616 * _e616) * (3f - (2f * _e616))) * ((_e626 * _e626) * (3f - (2f * _e626))));
            let _e635 = pill_1.member[_e281].conditions;
            let _e639 = frame.member[0u].time;
            let _e641 = ((_e582 - 1f) * -1f);
            let _e643 = select(_e641, 0f, (_e641 < 0f));
            let _e645 = select(_e643, 1f, (_e643 > 1f));
            let _e649 = ((_e645 * _e645) * (3f - (2f * _e645)));
            let _e650 = (1f - _e649);
            let _e680 = (0.3f * _e650);
            let _e681 = (0.22f * _e649);
            let _e687 = ((((_e604 * _e604) * (3f - (2f * _e604))) * _e609) * 0.8f);
            let _e688 = (1f - _e687);
            let _e705 = (_e631 * 0.9f);
            let _e706 = (1f - _e705);
            let _e718 = floor((_e583 * 0.055555556f));
            let _e719 = floor((_e584 * 0.055555556f));
            let _e723 = cantus_render_shader_hash(vec2<f32>(_e718, _e719));
            let _e732 = (_e583 - (((_e718 + 0.2f) + (_e723.x * 0.6f)) * 18f));
            let _e733 = (_e584 - (((_e719 + 0.2f) + (_e723.y * 0.6f)) * 18f));
            let _e739 = ((sqrt(((_e732 * _e732) + (_e733 * _e733))) - 1f) * -1.6666666f);
            let _e741 = select(_e739, 0f, (_e739 < 0f));
            let _e743 = select(_e741, 1f, (_e741 > 1f));
            let _e751 = cantus_render_shader_hash(vec2<f32>((_e718 + 31.7f), (_e719 + 31.7f)));
            let _e754 = ((_e751.x - 0.75f) * 4f);
            let _e756 = select(_e754, 0f, (_e754 < 0f));
            let _e758 = select(_e756, 1f, (_e756 > 1f));
            let _e770 = ((((((_e743 * _e743) * (3f - (2f * _e743))) * ((_e758 * _e758) * (3f - (2f * _e758)))) * _e609) * (1f - _e635.cloud)) * (0.3f + (_e649 * 0.7f)));
            let _e771 = (((((((((0.006f * _e650) + (0.025f * _e649)) * _e609) + (((0.08f * _e650) + (0.32f * _e649)) * _e598)) * _e688) + (((0.1f * _e650) + _e681) * _e687)) * _e706) + (((0.78f * _e650) + (0.38f * _e649)) * _e705)) + _e770);
            let _e772 = (((((((((0.012f * _e650) + (0.04f * _e649)) * _e609) + (((0.34f * _e650) + (0.67f * _e649)) * _e598)) * _e688) + (((0.16f * _e650) + (0.25f * _e649)) * _e687)) * _e706) + ((_e680 + _e681) * _e705)) + _e770);
            let _e773 = (((((((((0.035f * _e650) + (0.095f * _e649)) * _e609) + (((0.62f * _e650) + (0.87f * _e649)) * _e598)) * _e688) + ((_e680 + (0.45f * _e649)) * _e687)) * _e706) + (((0.2f * _e650) + (0.42f * _e649)) * _e705)) + _e770);
            if (_e635.cloud > 0.0009765625f) {
                let _e776 = (_e583 / _e304);
                phi_22_ = 0i;
                phi_23_ = 0.5f;
                phi_24_ = 0f;
                phi_25_ = vec2<f32>(((_e776 * 0.14f) + (_e639 * 0.012f)), ((_e582 * 0.14f) + 6.1f));
                loop {
                    let _e784 = phi_22_;
                    let _e786 = phi_23_;
                    let _e788 = phi_24_;
                    let _e790 = phi_25_;
                    local_32 = _e788;
                    let _e791 = (_e784 < 4i);
                    if _e791 {
                        let _e794 = cantus_render_shader_simplex_noise(_e790);
                        phi_26_ = (_e784 + 1i);
                        phi_27_ = (_e786 * 0.5f);
                        phi_28_ = (_e788 + (_e794 * _e786));
                        phi_29_ = vec2<f32>(((_e790.x * 1.6f) + (_e790.y * 1.2f)), ((_e790.y * 1.6f) - (_e790.x * 1.2f)));
                    } else {
                        phi_26_ = i32();
                        phi_27_ = f32();
                        phi_28_ = f32();
                        phi_29_ = vec2<f32>();
                    }
                    let _e807 = phi_26_;
                    let _e809 = phi_27_;
                    let _e811 = phi_28_;
                    let _e813 = phi_29_;
                    continue;
                    continuing {
                        phi_22_ = _e807;
                        phi_23_ = _e809;
                        phi_24_ = _e811;
                        phi_25_ = _e813;
                        break if !(_e791);
                    }
                }
                let _e816 = local_32;
                let _e817 = (_e816 * 0.5f);
                phi_30_ = 0i;
                phi_31_ = 0.5f;
                phi_32_ = 0f;
                phi_33_ = vec2<f32>(((_e776 * 0.287f) + (_e639 * 0.018f)), ((_e582 * 0.287f) + -3.7f));
                loop {
                    let _e826 = phi_30_;
                    let _e828 = phi_31_;
                    let _e830 = phi_32_;
                    let _e832 = phi_33_;
                    local_33 = _e830;
                    local_34 = _e830;
                    let _e833 = (_e826 < 4i);
                    if _e833 {
                        let _e836 = cantus_render_shader_simplex_noise(_e832);
                        phi_34_ = (_e826 + 1i);
                        phi_35_ = (_e828 * 0.5f);
                        phi_36_ = (_e830 + (_e836 * _e828));
                        phi_37_ = vec2<f32>(((_e832.x * 1.6f) + (_e832.y * 1.2f)), ((_e832.y * 1.6f) - (_e832.x * 1.2f)));
                    } else {
                        phi_34_ = i32();
                        phi_35_ = f32();
                        phi_36_ = f32();
                        phi_37_ = vec2<f32>();
                    }
                    let _e849 = phi_34_;
                    let _e851 = phi_35_;
                    let _e853 = phi_36_;
                    let _e855 = phi_37_;
                    continue;
                    continuing {
                        phi_30_ = _e849;
                        phi_31_ = _e851;
                        phi_32_ = _e853;
                        phi_33_ = _e855;
                        break if !(_e833);
                    }
                }
                let _e858 = local_33;
                let _e861 = local_34;
                let _e865 = ((((0.5f + _e817) + (_e861 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e867 = select(_e865, 0f, (_e865 < 0f));
                let _e869 = select(_e867, 1f, (_e867 > 1f));
                let _e875 = (((_e858 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e877 = select(_e875, 0f, (_e875 < 0f));
                let _e879 = select(_e877, 1f, (_e877 > 1f));
                let _e886 = ((_e817 + 0.02000001f) * 4.5454545f);
                let _e888 = select(_e886, 0f, (_e886 < 0f));
                let _e890 = select(_e888, 1f, (_e888 > 1f));
                let _e896 = ((((_e879 * _e879) * (3f - (2f * _e879))) * 0.55f) + (((_e890 * _e890) * (3f - (2f * _e890))) * 0.45f));
                let _e897 = (1f - _e896);
                let _e934 = (_e631 * 0.45f);
                let _e935 = (1f - _e934);
                let _e947 = (_e635.cloud * (0.12f + (((_e869 * _e869) * (3f - (2f * _e869))) * 0.7f)));
                let _e948 = (1f - _e947);
                phi_38_ = vec3<f32>(((_e771 * _e948) + (((((((0.16f * _e897) + (0.32f * _e896)) * _e609) + (((0.62f * _e897) + (0.92f * _e896)) * _e598)) * _e935) + (((0.5f * _e897) + (0.76f * _e896)) * _e934)) * _e947)), ((_e772 * _e948) + (((((((0.2f * _e897) + (0.36f * _e896)) * _e609) + (((0.7f * _e897) + (0.94f * _e896)) * _e598)) * _e935) + (((0.36f * _e897) + (0.59f * _e896)) * _e934)) * _e947)), ((_e773 * _e948) + (((((((0.28f * _e897) + (0.43f * _e896)) * _e609) + (((0.78f * _e897) + (0.96f * _e896)) * _e598)) * _e935) + (((0.4f * _e897) + (0.56f * _e896)) * _e934)) * _e947)));
            } else {
                phi_38_ = vec3<f32>(_e771, _e772, _e773);
            }
            let _e960 = phi_38_;
            let _e963 = (1f - (_e635.rain * 0.2f));
            let _e973 = ((_e960.x * _e963) + (_e635.rain * 0.020000001f));
            let _e974 = ((_e960.y * _e963) + (_e635.rain * 0.034f));
            let _e975 = ((_e960.z * _e963) + (_e635.rain * 0.05f));
            if (_e635.rain > 0.0009765625f) {
                let _e980 = (_e583 - (20f * _e639));
                let _e981 = (_e584 - (110f * _e639));
                let _e984 = floor((_e980 * 0.06666667f));
                let _e985 = floor((_e981 * 0.04f));
                let _e987 = cantus_render_shader_hash(vec2<f32>(_e984, _e985));
                let _e998 = (_e980 - (((_e984 + 0.15f) + (_e987.x * 0.7f)) * 15f));
                let _e999 = (_e981 - (((_e985 + 0.15f) + (_e987.y * 0.7f)) * 25f));
                let _e1003 = (((_e998 * 1.8000001f) + (_e999 * 9f)) * 0.011870845f);
                let _e1005 = select(_e1003, 0f, (_e1003 < 0f));
                let _e1007 = select(_e1005, 1f, (_e1005 > 1f));
                let _e1010 = (_e998 - (1.8000001f * _e1007));
                let _e1011 = (_e999 - (9f * _e1007));
                let _e1017 = ((sqrt(((_e1010 * _e1010) + (_e1011 * _e1011))) - 1.0999999f) * -1.666667f);
                let _e1019 = select(_e1017, 0f, (_e1017 < 0f));
                let _e1021 = select(_e1019, 1f, (_e1019 > 1f));
                let _e1029 = cantus_render_shader_hash(vec2<f32>((_e984 + 19.3f), (_e985 + 19.3f)));
                let _e1032 = ((_e1029.x - 0.22000003f) * 1.2820513f);
                let _e1034 = select(_e1032, 0f, (_e1032 < 0f));
                let _e1036 = select(_e1034, 1f, (_e1034 > 1f));
                let _e1043 = (((((_e1021 * _e1021) * (3f - (2f * _e1021))) * ((_e1036 * _e1036) * (3f - (2f * _e1036)))) * _e635.rain) * 0.7f);
                let _e1045 = select(_e1043, 0f, (_e1043 < 0f));
                let _e1047 = select(_e1045, 1f, (_e1045 > 1f));
                let _e1048 = (1f - _e1047);
                phi_39_ = vec3<f32>(((_e973 * _e1048) + (0.52f * _e1047)), ((_e974 * _e1048) + (0.72f * _e1047)), ((_e975 * _e1048) + (0.9f * _e1047)));
            } else {
                phi_39_ = vec3<f32>(_e973, _e974, _e975);
            }
            let _e1060 = phi_39_;
            if (_e635.snow > 0.0009765625f) {
                let _e1065 = (_e583 - (5f * _e639));
                let _e1066 = (_e584 - (14f * _e639));
                let _e1069 = floor((_e1065 * 0.05f));
                let _e1070 = floor((_e1066 * 0.05f));
                let _e1074 = cantus_render_shader_hash(vec2<f32>((_e1069 + 31.7f), (_e1070 + 31.7f)));
                let _e1085 = (_e1065 - (((_e1069 + 0.15f) + (_e1074.x * 0.7f)) * 20f));
                let _e1086 = (_e1066 - (((_e1070 + 0.15f) + (_e1074.y * 0.7f)) * 20f));
                let _e1090 = (((_e1085 * 0.080000006f) + (_e1086 * 0.4f)) * 6.009615f);
                let _e1092 = select(_e1090, 0f, (_e1090 < 0f));
                let _e1094 = select(_e1092, 1f, (_e1092 > 1f));
                let _e1097 = (_e1085 - (0.080000006f * _e1094));
                let _e1098 = (_e1086 - (0.4f * _e1094));
                let _e1104 = ((sqrt(((_e1097 * _e1097) + (_e1098 * _e1098))) - 1.5999999f) * -1.666667f);
                let _e1106 = select(_e1104, 0f, (_e1104 < 0f));
                let _e1108 = select(_e1106, 1f, (_e1106 > 1f));
                let _e1116 = cantus_render_shader_hash(vec2<f32>((_e1069 + 19.3f), (_e1070 + 19.3f)));
                let _e1119 = ((_e1116.x - 0.3f) * 1.4285715f);
                let _e1121 = select(_e1119, 0f, (_e1119 < 0f));
                let _e1123 = select(_e1121, 1f, (_e1121 > 1f));
                let _e1130 = (((((_e1108 * _e1108) * (3f - (2f * _e1108))) * ((_e1123 * _e1123) * (3f - (2f * _e1123)))) * _e635.snow) * 0.92f);
                let _e1132 = select(_e1130, 0f, (_e1130 < 0f));
                let _e1134 = select(_e1132, 1f, (_e1132 > 1f));
                let _e1135 = (1f - _e1134);
                let _e1142 = (0.96f * _e1134);
                phi_40_ = vec3<f32>(((_e1060.x * _e1135) + _e1142), ((_e1060.y * _e1135) + _e1142), ((_e1060.z * _e1135) + _e1142));
            } else {
                phi_40_ = _e1060;
            }
            let _e1148 = phi_40_;
            if (_e635.hail > 0.0009765625f) {
                let _e1153 = (_e583 - (18f * _e639));
                let _e1154 = (_e584 - (85f * _e639));
                let _e1157 = floor((_e1153 * 0.04347826f));
                let _e1158 = floor((_e1154 * 0.04347826f));
                let _e1162 = cantus_render_shader_hash(vec2<f32>((_e1157 + 63.4f), (_e1158 + 63.4f)));
                let _e1173 = (_e1153 - (((_e1157 + 0.15f) + (_e1162.x * 0.7f)) * 23f));
                let _e1174 = (_e1154 - (((_e1158 + 0.15f) + (_e1162.y * 0.7f)) * 23f));
                let _e1178 = (((_e1173 * 0.24000001f) + (_e1174 * 1.2f)) * 0.667735f);
                let _e1180 = select(_e1178, 0f, (_e1178 < 0f));
                let _e1182 = select(_e1180, 1f, (_e1180 > 1f));
                let _e1185 = (_e1173 - (0.24000001f * _e1182));
                let _e1186 = (_e1174 - (1.2f * _e1182));
                let _e1192 = ((sqrt(((_e1185 * _e1185) + (_e1186 * _e1186))) - 0.79999995f) * -1.6666667f);
                let _e1194 = select(_e1192, 0f, (_e1192 < 0f));
                let _e1196 = select(_e1194, 1f, (_e1194 > 1f));
                let _e1204 = cantus_render_shader_hash(vec2<f32>((_e1157 + 19.3f), (_e1158 + 19.3f)));
                let _e1207 = ((_e1204.x - 0.7f) * 3.3333333f);
                let _e1209 = select(_e1207, 0f, (_e1207 < 0f));
                let _e1211 = select(_e1209, 1f, (_e1209 > 1f));
                let _e1218 = (((((_e1196 * _e1196) * (3f - (2f * _e1196))) * ((_e1211 * _e1211) * (3f - (2f * _e1211)))) * _e635.hail) * 0.7f);
                let _e1220 = select(_e1218, 0f, (_e1218 < 0f));
                let _e1222 = select(_e1220, 1f, (_e1220 > 1f));
                let _e1223 = (1f - _e1222);
                phi_41_ = vec3<f32>(((_e1148.x * _e1223) + (0.75f * _e1222)), ((_e1148.y * _e1223) + (0.86f * _e1222)), ((_e1148.z * _e1223) + (0.94f * _e1222)));
            } else {
                phi_41_ = _e1148;
            }
            let _e1238 = phi_41_;
            let _e1242 = ((sin((_e639 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1244 = select(_e1242, 0f, (_e1242 < 0f));
            let _e1246 = select(_e1244, 1f, (_e1244 > 1f));
            let _e1252 = (((_e1246 * _e1246) * (3f - (2f * _e1246))) * _e635.lightning);
            let _e1254 = (1f - (_e1252 * 0.55f));
            let _e1264 = ((_e1238.x * _e1254) + (_e1252 * 0.3575f));
            let _e1265 = ((_e1238.y * _e1254) + (_e1252 * 0.407f));
            let _e1266 = ((_e1238.z * _e1254) + (_e1252 * 0.528f));
            if (_e635.fog > 0.0009765625f) {
                phi_42_ = 0i;
                phi_43_ = 0.5f;
                phi_44_ = 0f;
                phi_45_ = vec2<f32>(((_e581 * 0.9f) + (_e639 * 0.008f)), ((_e582 * 0.32f) + 12f));
                loop {
                    let _e1277 = phi_42_;
                    let _e1279 = phi_43_;
                    let _e1281 = phi_44_;
                    let _e1283 = phi_45_;
                    local_35 = _e1281;
                    let _e1284 = (_e1277 < 4i);
                    if _e1284 {
                        let _e1287 = cantus_render_shader_simplex_noise(_e1283);
                        phi_46_ = (_e1277 + 1i);
                        phi_47_ = (_e1279 * 0.5f);
                        phi_48_ = (_e1281 + (_e1287 * _e1279));
                        phi_49_ = vec2<f32>(((_e1283.x * 1.6f) + (_e1283.y * 1.2f)), ((_e1283.y * 1.6f) - (_e1283.x * 1.2f)));
                    } else {
                        phi_46_ = i32();
                        phi_47_ = f32();
                        phi_48_ = f32();
                        phi_49_ = vec2<f32>();
                    }
                    let _e1300 = phi_46_;
                    let _e1302 = phi_47_;
                    let _e1304 = phi_48_;
                    let _e1306 = phi_49_;
                    continue;
                    continuing {
                        phi_42_ = _e1300;
                        phi_43_ = _e1302;
                        phi_44_ = _e1304;
                        phi_45_ = _e1306;
                        break if !(_e1284);
                    }
                }
                let _e1309 = local_35;
                let _e1312 = (((_e1309 * 0.5f) + 0.15f) * 2.857143f);
                let _e1314 = select(_e1312, 0f, (_e1312 < 0f));
                let _e1316 = select(_e1314, 1f, (_e1314 > 1f));
                let _e1323 = (_e635.fog * (0.58f + (((_e1316 * _e1316) * (3f - (2f * _e1316))) * 0.18f)));
                let _e1324 = (1f - _e1323);
                phi_50_ = vec3<f32>(((_e1264 * _e1324) + (0.63f * _e1323)), ((_e1265 * _e1324) + (0.69f * _e1323)), ((_e1266 * _e1324) + (0.73f * _e1323)));
            } else {
                phi_50_ = vec3<f32>(_e1264, _e1265, _e1266);
            }
            let _e1336 = phi_50_;
            let _e1338 = ((_e526 - 5f) * -0.125f);
            let _e1340 = select(_e1338, 0f, (_e1338 < 0f));
            let _e1342 = select(_e1340, 1f, (_e1340 > 1f));
            let _e1347 = (((_e1342 * _e1342) * (3f - (2f * _e1342))) * 0.14f);
            if (_e583 < 96f) {
                phi_58_ = 0u;
            } else {
                if (_e583 < 184f) {
                    phi_57_ = 1u;
                } else {
                    if _e288 {
                        phi_51_ = (_e287 <= 1f);
                    } else {
                        phi_51_ = false;
                    }
                    let _e1358 = phi_51_;
                    if _e1358 {
                        phi_52_ = select(true, false, (_e583 < 224f));
                    } else {
                        phi_52_ = true;
                    }
                    let _e1362 = phi_52_;
                    if _e1362 {
                        if _e288 {
                            phi_53_ = (_e287 <= 1f);
                        } else {
                            phi_53_ = false;
                        }
                        let _e1365 = phi_53_;
                        if (_e583 < (select(0f, 40f, _e1365) + 224f)) {
                            phi_55_ = 3u;
                        } else {
                            if _e288 {
                                phi_54_ = (_e287 <= 1f);
                            } else {
                                phi_54_ = false;
                            }
                            let _e1371 = phi_54_;
                            phi_55_ = select(5u, 4u, (_e583 < (select(0f, 40f, _e1371) + 256f)));
                        }
                        let _e1377 = phi_55_;
                        phi_56_ = _e1377;
                    } else {
                        phi_56_ = 2u;
                    }
                    let _e1379 = phi_56_;
                    phi_57_ = _e1379;
                }
                let _e1381 = phi_57_;
                phi_58_ = _e1381;
            }
            let _e1383 = phi_58_;
            if _e288 {
                phi_59_ = (_e287 <= 1f);
            } else {
                phi_59_ = false;
            }
            let _e1386 = phi_59_;
            let _e1387 = select(0f, 40f, _e1386);
            switch bitcast<i32>(_e1383) {
                case 0: {
                    phi_60_ = 12f;
                    break;
                }
                case 1: {
                    phi_60_ = 100f;
                    break;
                }
                case 2: {
                    phi_60_ = 188f;
                    break;
                }
                case 3: {
                    phi_60_ = (188f + _e1387);
                    break;
                }
                case 4: {
                    phi_60_ = (228f + _e1387);
                    break;
                }
                case 5: {
                    phi_60_ = (260f + _e1387);
                    break;
                }
                default: {
                    phi_60_ = f32();
                    break;
                }
            }
            let _e1393 = phi_60_;
            switch bitcast<i32>(_e1383) {
                case 0: {
                    phi_61_ = true;
                    phi_62_ = false;
                    phi_63_ = false;
                    break;
                }
                case 1: {
                    phi_61_ = true;
                    phi_62_ = false;
                    phi_63_ = false;
                    break;
                }
                case 2: {
                    phi_61_ = false;
                    phi_62_ = true;
                    phi_63_ = false;
                    break;
                }
                case 3: {
                    phi_61_ = false;
                    phi_62_ = true;
                    phi_63_ = false;
                    break;
                }
                case 4: {
                    phi_61_ = false;
                    phi_62_ = false;
                    phi_63_ = true;
                    break;
                }
                case 5: {
                    phi_61_ = false;
                    phi_62_ = false;
                    phi_63_ = true;
                    break;
                }
                default: {
                    phi_61_ = bool();
                    phi_62_ = bool();
                    phi_63_ = bool();
                    break;
                }
            }
            let _e1396 = phi_61_;
            let _e1398 = phi_62_;
            let _e1400 = phi_63_;
            let _e1401 = select(_e1398, false, _e1396);
            let _e1408 = (_e583 - (_e1393 + (select(select(80f, 32f, _e1401), 24f, select(select(_e1400, false, _e1396), false, _e1401)) * 0.5f)));
            let _e1409 = (_e584 - _e308);
            switch bitcast<i32>(_e1383) {
                case 0: {
                    phi_64_ = vec2<f32>();
                    phi_65_ = true;
                    break;
                }
                case 1: {
                    phi_64_ = vec2<f32>();
                    phi_65_ = true;
                    break;
                }
                default: {
                    phi_64_ = vec2<f32>(0f, 0f);
                    phi_65_ = false;
                    break;
                }
            }
            let _e1412 = phi_64_;
            let _e1414 = phi_65_;
            if _e1414 {
                let _e1415 = (_e583 - 52f);
                let _e1420 = pill_1.member[_e281].cpu.temperature;
                if (_e1420 <= 62f) {
                    phi_74_ = vec2<f32>(0f, 0f);
                } else {
                    let _e1423 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1415, _e1409), 13f, 13f);
                    phi_66_ = 0i;
                    phi_67_ = 0.5f;
                    phi_68_ = 0f;
                    phi_69_ = vec2<f32>(((_e1415 + (_e639 * 1.8f)) * 0.035f), (((_e1409 + -(_e639)) * 0.035f) + 6.1f));
                    loop {
                        let _e1433 = phi_66_;
                        let _e1435 = phi_67_;
                        let _e1437 = phi_68_;
                        let _e1439 = phi_69_;
                        local_36 = _e1437;
                        let _e1440 = (_e1433 < 4i);
                        if _e1440 {
                            let _e1443 = cantus_render_shader_simplex_noise(_e1439);
                            phi_70_ = (_e1433 + 1i);
                            phi_71_ = (_e1435 * 0.5f);
                            phi_72_ = (_e1437 + (_e1443 * _e1435));
                            phi_73_ = vec2<f32>(((_e1439.x * 1.6f) + (_e1439.y * 1.2f)), ((_e1439.y * 1.6f) - (_e1439.x * 1.2f)));
                        } else {
                            phi_70_ = i32();
                            phi_71_ = f32();
                            phi_72_ = f32();
                            phi_73_ = vec2<f32>();
                        }
                        let _e1456 = phi_70_;
                        let _e1458 = phi_71_;
                        let _e1460 = phi_72_;
                        let _e1462 = phi_73_;
                        continue;
                        continuing {
                            phi_66_ = _e1456;
                            phi_67_ = _e1458;
                            phi_68_ = _e1460;
                            phi_69_ = _e1462;
                            break if !(_e1440);
                        }
                    }
                    let _e1465 = local_36;
                    let _e1466 = (_e1465 * 0.5f);
                    let _e1469 = ((_e1423 - -0.5f) * 0.5f);
                    let _e1471 = select(_e1469, 0f, (_e1469 < 0f));
                    let _e1473 = select(_e1471, 1f, (_e1471 > 1f));
                    let _e1479 = ((_e1423 - 14f) * -0.083333336f);
                    let _e1481 = select(_e1479, 0f, (_e1479 < 0f));
                    let _e1483 = select(_e1481, 1f, (_e1481 > 1f));
                    let _e1488 = (((_e1473 * _e1473) * (3f - (2f * _e1473))) * ((_e1483 * _e1483) * (3f - (2f * _e1483))));
                    let _e1493 = ((_e1466 + 0.19999999f) * 3.125f);
                    let _e1495 = select(_e1493, 0f, (_e1493 < 0f));
                    let _e1497 = select(_e1495, 1f, (_e1495 > 1f));
                    let _e1504 = ((_e1420 - 62f) * 0.045454547f);
                    let _e1506 = select(_e1504, 0f, (_e1504 < 0f));
                    let _e1508 = select(_e1506, 1f, (_e1506 > 1f));
                    let _e1512 = ((_e1508 * _e1508) * (3f - (2f * _e1508)));
                    phi_74_ = vec2<f32>(((_e1488 * (0.18f + ((0.5f + _e1466) * 0.34f))) * _e1512), ((_e1488 * ((_e1497 * _e1497) * (3f - (2f * _e1497)))) * _e1512));
                }
                let _e1517 = phi_74_;
                let _e1520 = (_e583 - 140f);
                let _e1525 = pill_1.member[_e281].gpu.temperature;
                if (_e1525 <= 62f) {
                    phi_83_ = vec2<f32>(0f, 0f);
                } else {
                    let _e1528 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1520, _e1409), 13f, 13f);
                    phi_75_ = 0i;
                    phi_76_ = 0.5f;
                    phi_77_ = 0f;
                    phi_78_ = vec2<f32>(((_e1520 + (_e639 * 1.8f)) * 0.035f), (((_e1409 + -(_e639)) * 0.035f) + 6.1f));
                    loop {
                        let _e1538 = phi_75_;
                        let _e1540 = phi_76_;
                        let _e1542 = phi_77_;
                        let _e1544 = phi_78_;
                        local_37 = _e1542;
                        let _e1545 = (_e1538 < 4i);
                        if _e1545 {
                            let _e1548 = cantus_render_shader_simplex_noise(_e1544);
                            phi_79_ = (_e1538 + 1i);
                            phi_80_ = (_e1540 * 0.5f);
                            phi_81_ = (_e1542 + (_e1548 * _e1540));
                            phi_82_ = vec2<f32>(((_e1544.x * 1.6f) + (_e1544.y * 1.2f)), ((_e1544.y * 1.6f) - (_e1544.x * 1.2f)));
                        } else {
                            phi_79_ = i32();
                            phi_80_ = f32();
                            phi_81_ = f32();
                            phi_82_ = vec2<f32>();
                        }
                        let _e1561 = phi_79_;
                        let _e1563 = phi_80_;
                        let _e1565 = phi_81_;
                        let _e1567 = phi_82_;
                        continue;
                        continuing {
                            phi_75_ = _e1561;
                            phi_76_ = _e1563;
                            phi_77_ = _e1565;
                            phi_78_ = _e1567;
                            break if !(_e1545);
                        }
                    }
                    let _e1570 = local_37;
                    let _e1571 = (_e1570 * 0.5f);
                    let _e1574 = ((_e1528 - -0.5f) * 0.5f);
                    let _e1576 = select(_e1574, 0f, (_e1574 < 0f));
                    let _e1578 = select(_e1576, 1f, (_e1576 > 1f));
                    let _e1584 = ((_e1528 - 14f) * -0.083333336f);
                    let _e1586 = select(_e1584, 0f, (_e1584 < 0f));
                    let _e1588 = select(_e1586, 1f, (_e1586 > 1f));
                    let _e1593 = (((_e1578 * _e1578) * (3f - (2f * _e1578))) * ((_e1588 * _e1588) * (3f - (2f * _e1588))));
                    let _e1598 = ((_e1571 + 0.19999999f) * 3.125f);
                    let _e1600 = select(_e1598, 0f, (_e1598 < 0f));
                    let _e1602 = select(_e1600, 1f, (_e1600 > 1f));
                    let _e1609 = ((_e1525 - 62f) * 0.045454547f);
                    let _e1611 = select(_e1609, 0f, (_e1609 < 0f));
                    let _e1613 = select(_e1611, 1f, (_e1611 > 1f));
                    let _e1617 = ((_e1613 * _e1613) * (3f - (2f * _e1613)));
                    phi_83_ = vec2<f32>(((_e1593 * (0.18f + ((0.5f + _e1571) * 0.34f))) * _e1617), ((_e1593 * ((_e1602 * _e1602) * (3f - (2f * _e1602)))) * _e1617));
                }
                let _e1622 = phi_83_;
                phi_84_ = vec2<f32>(select(_e1622.x, _e1517.x, (_e1517.x > _e1622.x)), select(_e1622.y, _e1517.y, (_e1517.y > _e1622.y)));
            } else {
                phi_84_ = _e1412;
            }
            let _e1631 = phi_84_;
            let _e1636 = pill_1.member[_e281].cpu.temperature;
            let _e1641 = pill_1.member[_e281].gpu.temperature;
            if (_e1636 != _e1636) {
                phi_85_ = true;
            } else {
                phi_85_ = (_e1641 >= _e1636);
            }
            let _e1645 = phi_85_;
            let _e1646 = select(_e1636, _e1641, _e1645);
            let _e1648 = ((_e1646 - 60f) * 0.083333336f);
            let _e1650 = select(_e1648, 0f, (_e1648 < 0f));
            let _e1652 = select(_e1650, 1f, (_e1650 > 1f));
            let _e1656 = ((_e1652 * _e1652) * (3f - (2f * _e1652)));
            let _e1657 = (1f - _e1656);
            let _e1666 = ((_e1646 - 72f) * 0.0625f);
            let _e1668 = select(_e1666, 0f, (_e1666 < 0f));
            let _e1670 = select(_e1668, 1f, (_e1668 > 1f));
            let _e1674 = ((_e1670 * _e1670) * (3f - (2f * _e1670)));
            let _e1675 = (1f - _e1674);
            let _e1685 = (_e1631.y * 0.12f);
            let _e1686 = (0.24f + _e1685);
            let _e1687 = (0.76f - _e1685);
            let _e1699 = (1f - (_e1631.x * 0.46f));
            let _e1709 = (_e1631.y * 0.64f);
            let _e1710 = (1f - _e1709);
            let _e1717 = (((((_e1336.x + _e1347) * _e1699) + (_e1631.x * 0.0009200001f)) * _e1710) + (((0.07f * _e1687) + (((((0.22f * _e1657) + _e1656) * _e1675) + _e1674) * _e1686)) * _e1709));
            let _e1718 = (((((_e1336.y + _e1347) * _e1699) + (_e1631.x * 0.00276f)) * _e1710) + (((0.12f * _e1687) + (((((0.62f * _e1657) + (0.38f * _e1656)) * _e1675) + (0.08f * _e1674)) * _e1686)) * _e1709));
            let _e1719 = (((((_e1336.z + _e1347) * _e1699) + (_e1631.x * 0.00552f)) * _e1710) + (((0.18f * _e1687) + ((((_e1657 + (0.08f * _e1656)) * _e1675) + (0.035f * _e1674)) * _e1686)) * _e1709));
            switch bitcast<i32>(_e1383) {
                case 0: {
                    let _e2435 = pill_1.member[_e281].history_scroll;
                    switch bitcast<i32>(_e1383) {
                        case 0: {
                            phi_92_ = true;
                            phi_93_ = false;
                            phi_94_ = false;
                            break;
                        }
                        case 1: {
                            phi_92_ = true;
                            phi_93_ = false;
                            phi_94_ = false;
                            break;
                        }
                        case 2: {
                            phi_92_ = false;
                            phi_93_ = true;
                            phi_94_ = false;
                            break;
                        }
                        case 3: {
                            phi_92_ = false;
                            phi_93_ = true;
                            phi_94_ = false;
                            break;
                        }
                        case 4: {
                            phi_92_ = false;
                            phi_93_ = false;
                            phi_94_ = true;
                            break;
                        }
                        case 5: {
                            phi_92_ = false;
                            phi_93_ = false;
                            phi_94_ = true;
                            break;
                        }
                        default: {
                            phi_92_ = bool();
                            phi_93_ = bool();
                            phi_94_ = bool();
                            break;
                        }
                    }
                    let _e2438 = phi_92_;
                    let _e2440 = phi_93_;
                    let _e2442 = phi_94_;
                    let _e2443 = select(_e2440, false, _e2438);
                    let _e2449 = ((select(select(80f, 32f, _e2443), 24f, select(select(_e2442, false, _e2438), false, _e2443)) * 0.5f) - 4f);
                    let _e2450 = (_e308 - 8f);
                    let _e2451 = (_e2449 - _e2450);
                    let _e2453 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1408, _e1409), _e2451, _e2450);
                    let _e2454 = abs(_e1408);
                    let _e2455 = abs(_e1409);
                    let _e2458 = (round((_e2454 * 0.11111111f)) * 9f);
                    if (_e2458 != _e2458) {
                        phi_95_ = true;
                    } else {
                        phi_95_ = (_e2449 <= _e2458);
                    }
                    let _e2462 = phi_95_;
                    let _e2463 = select(_e2458, _e2449, _e2462);
                    let _e2464 = (_e2463 - _e2451);
                    if (_e2464 != _e2464) {
                        phi_96_ = true;
                    } else {
                        phi_96_ = (0f >= _e2464);
                    }
                    let _e2468 = phi_96_;
                    let _e2469 = select(_e2464, 0f, _e2468);
                    let _e2470 = (_e2450 * _e2450);
                    let _e2473 = sqrt((_e2470 - (_e2469 * _e2469)));
                    let _e2474 = (_e2469 / _e2450);
                    let _e2475 = (_e2473 / _e2450);
                    let _e2480 = ((_e2454 - _e2463) - (_e2474 * 0.9f));
                    let _e2481 = ((_e2455 - _e2473) - (_e2475 * 0.9f));
                    let _e2490 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e2480 * -(_e2475)) + (_e2481 * _e2474)), ((_e2480 * _e2474) + (_e2481 * _e2475))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e2492 = round((_e2455 * 0.125f));
                    if (_e2492 != _e2492) {
                        phi_97_ = true;
                    } else {
                        phi_97_ = (1f <= _e2492);
                    }
                    let _e2496 = phi_97_;
                    let _e2498 = (select(_e2492, 1f, _e2496) * 8f);
                    let _e2501 = sqrt((_e2470 - (_e2498 * _e2498)));
                    let _e2503 = (_e2501 / _e2450);
                    let _e2504 = (_e2498 / _e2450);
                    let _e2509 = ((_e2454 - (_e2451 + _e2501)) - (_e2503 * 0.9f));
                    let _e2510 = ((_e2455 - _e2498) - (_e2504 * 0.9f));
                    let _e2519 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e2509 * -(_e2504)) + (_e2510 * _e2503)), ((_e2509 * _e2503) + (_e2510 * _e2504))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e2490 != _e2490) {
                        phi_98_ = true;
                    } else {
                        phi_98_ = (_e2519 <= _e2490);
                    }
                    let _e2523 = phi_98_;
                    let _e2524 = select(_e2490, _e2519, _e2523);
                    let _e2527 = (0.5f + ((_e2524 - _e2453) * 0.3125f));
                    let _e2529 = select(_e2527, 0f, (_e2527 < 0f));
                    let _e2531 = select(_e2529, 1f, (_e2529 > 1f));
                    let _e2540 = ((_e2453 - 0.55f) * -0.9090909f);
                    let _e2542 = select(_e2540, 0f, (_e2540 < 0f));
                    let _e2544 = select(_e2542, 1f, (_e2542 > 1f));
                    let _e2548 = ((_e2544 * _e2544) * (3f - (2f * _e2544)));
                    let _e2549 = (_e2449 * 0.051282052f);
                    let _e2550 = (_e1408 + _e2449);
                    let _e2552 = ((_e2550 / _e2549) + _e2435);
                    let _e2554 = select(_e2552, 0f, (_e2552 < 0f));
                    let _e2556 = select(_e2554, 39f, (_e2554 > 39f));
                    let _e2557 = floor(_e2556);
                    let _e2562 = select(select(u32(_e2557), 0u, (_e2557 < 0f)), 4294967295u, (_e2557 > 4294967000f));
                    let _e2563 = (_e308 - 10f);
                    let _e2567 = (((f32(_e2562) - _e2435) * _e2549) - _e2449);
                    let _e2569 = select(_e2562, 39u, (39u < _e2562));
                    let _e2570 = (_e2569 < 40u);
                    if _e2570 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2577 = pill_1.member[_e281].cpu.usage.samples[_e2569];
                    let _e2580 = (_e2563 * (1f - (_e2577 * 2f)));
                    let _e2581 = (_e2562 + 1u);
                    let _e2587 = select(_e2581, 39u, (39u < _e2581));
                    let _e2588 = (_e2587 < 40u);
                    if _e2588 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2595 = pill_1.member[_e281].cpu.usage.samples[_e2587];
                    let _e2599 = ((((f32(_e2581) - _e2435) * _e2549) - _e2449) - _e2567);
                    let _e2600 = ((_e2563 * (1f - (_e2595 * 2f))) - _e2580);
                    let _e2601 = (_e1408 - _e2567);
                    let _e2602 = (_e1409 - _e2580);
                    let _e2603 = (_e2601 * _e2599);
                    let _e2606 = (_e2599 * _e2599);
                    let _e2608 = (_e2606 + (_e2600 * _e2600));
                    if (_e2608 != _e2608) {
                        phi_99_ = true;
                    } else {
                        phi_99_ = (0.001f >= _e2608);
                    }
                    let _e2612 = phi_99_;
                    let _e2614 = ((_e2603 + (_e2602 * _e2600)) / select(_e2608, 0.001f, _e2612));
                    let _e2616 = select(_e2614, 0f, (_e2614 < 0f));
                    let _e2618 = select(_e2616, 1f, (_e2616 > 1f));
                    let _e2621 = (_e2601 - (_e2599 * _e2618));
                    let _e2622 = (_e2602 - (_e2600 * _e2618));
                    let _e2629 = ((abs(sqrt(((_e2621 * _e2621) + (_e2622 * _e2622)))) - 1.4000001f) * -0.9090908f);
                    let _e2631 = select(_e2629, 0f, (_e2629 < 0f));
                    let _e2633 = select(_e2631, 1f, (_e2631 > 1f));
                    let _e2639 = (_e2556 - trunc(_e2556));
                    let _e2641 = select(_e2639, 0f, (_e2639 < 0f));
                    let _e2643 = select(_e2641, 1f, (_e2641 > 1f));
                    let _e2647 = ((_e2643 * _e2643) * (3f - (2f * _e2643)));
                    let _e2654 = ((((_e2580 + (_e2600 * _e2647)) - _e1409) - 0.55f) * -0.9090909f);
                    let _e2656 = select(_e2654, 0f, (_e2654 < 0f));
                    let _e2658 = select(_e2656, 1f, (_e2656 > 1f));
                    let _e2664 = ((((_e2658 * _e2658) * (3f - (2f * _e2658))) * 0.156f) + ((_e2633 * _e2633) * (3f - (2f * _e2633))));
                    if _e2570 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2673 = pill_1.member[_e281].cpu.memory.samples[_e2569];
                    let _e2676 = (_e2563 * (1f - (_e2673 * 2f)));
                    if _e2588 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2683 = pill_1.member[_e281].cpu.memory.samples[_e2587];
                    let _e2687 = ((_e2563 * (1f - (_e2683 * 2f))) - _e2676);
                    let _e2688 = (_e1409 - _e2676);
                    let _e2692 = (_e2606 + (_e2687 * _e2687));
                    if (_e2692 != _e2692) {
                        phi_100_ = true;
                    } else {
                        phi_100_ = (0.001f >= _e2692);
                    }
                    let _e2696 = phi_100_;
                    let _e2698 = ((_e2603 + (_e2688 * _e2687)) / select(_e2692, 0.001f, _e2696));
                    let _e2700 = select(_e2698, 0f, (_e2698 < 0f));
                    let _e2702 = select(_e2700, 1f, (_e2700 > 1f));
                    let _e2705 = (_e2601 - (_e2599 * _e2702));
                    let _e2706 = (_e2688 - (_e2687 * _e2702));
                    let _e2713 = ((abs(sqrt(((_e2705 * _e2705) + (_e2706 * _e2706)))) - 1.4000001f) * -0.9090908f);
                    let _e2715 = select(_e2713, 0f, (_e2713 < 0f));
                    let _e2717 = select(_e2715, 1f, (_e2715 > 1f));
                    let _e2728 = ((((_e2676 + (_e2687 * _e2647)) - _e1409) - 0.55f) * -0.9090909f);
                    let _e2730 = select(_e2728, 0f, (_e2728 < 0f));
                    let _e2732 = select(_e2730, 1f, (_e2730 > 1f));
                    let _e2738 = ((((_e2732 * _e2732) * (3f - (2f * _e2732))) * 0.084f) + ((_e2717 * _e2717) * (3f - (2f * _e2717))));
                    let _e2746 = (_e2550 * 0.14285715f);
                    let _e2747 = ((_e1409 + _e2450) * 0.16393442f);
                    let _e2757 = ((abs(((_e2746 - trunc(_e2746)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2759 = select(_e2757, 0f, (_e2757 < 0f));
                    let _e2761 = select(_e2759, 1f, (_e2759 > 1f));
                    let _e2765 = ((_e2761 * _e2761) * (3f - (2f * _e2761)));
                    let _e2767 = ((abs(((_e2747 - trunc(_e2747)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2769 = select(_e2767, 0f, (_e2767 < 0f));
                    let _e2771 = select(_e2769, 1f, (_e2769 > 1f));
                    let _e2775 = ((_e2771 * _e2771) * (3f - (2f * _e2771)));
                    if (_e2765 != _e2765) {
                        phi_101_ = true;
                    } else {
                        phi_101_ = (_e2775 >= _e2765);
                    }
                    let _e2779 = phi_101_;
                    let _e2787 = pill_1.member[_e281].cpu.usage.samples[39u];
                    let _e2788 = (_e2787 * 0.24f);
                    let _e2789 = (0.18f + _e2788);
                    let _e2790 = (0.82f - _e2788);
                    let _e2799 = (_e1636 - 60f);
                    let _e2800 = (_e2799 * 0.083333336f);
                    let _e2802 = select(_e2800, 0f, (_e2800 < 0f));
                    let _e2804 = select(_e2802, 1f, (_e2802 > 1f));
                    let _e2808 = ((_e2804 * _e2804) * (3f - (2f * _e2804)));
                    let _e2809 = (1f - _e2808);
                    let _e2818 = ((_e1636 - 72f) * 0.0625f);
                    let _e2820 = select(_e2818, 0f, (_e2818 < 0f));
                    let _e2822 = select(_e2820, 1f, (_e2820 > 1f));
                    let _e2826 = ((_e2822 * _e2822) * (3f - (2f * _e2822)));
                    let _e2827 = (1f - _e2826);
                    let _e2836 = (_e2799 * 0.03846154f);
                    let _e2838 = select(_e2836, 0f, (_e2836 < 0f));
                    let _e2840 = select(_e2838, 1f, (_e2838 > 1f));
                    let _e2845 = (((_e2840 * _e2840) * (3f - (2f * _e2840))) * 0.9f);
                    let _e2846 = (1f - _e2845);
                    let _e2853 = ((((0.025f * _e2790) + (0.32f * _e2789)) * _e2846) + (((((0.22f * _e2809) + _e2808) * _e2827) + _e2826) * _e2845));
                    let _e2854 = ((((0.09f * _e2790) + (0.68f * _e2789)) * _e2846) + (((((0.62f * _e2809) + (0.38f * _e2808)) * _e2827) + (0.08f * _e2826)) * _e2845));
                    let _e2855 = ((((0.15f * _e2790) + _e2789) * _e2846) + ((((_e2809 + (0.08f * _e2808)) * _e2827) + (0.035f * _e2826)) * _e2845));
                    let _e2857 = ((((_e2524 + ((_e2453 - _e2524) * _e2531)) - ((1.6f * _e2531) * (1f - _e2531))) - 0.55f) * -0.9090909f);
                    let _e2859 = select(_e2857, 0f, (_e2857 < 0f));
                    let _e2861 = select(_e2859, 1f, (_e2859 > 1f));
                    let _e2865 = ((_e2861 * _e2861) * (3f - (2f * _e2861)));
                    let _e2867 = (1f - (_e2865 * 0.82f));
                    let _e2879 = ((abs(_e2453) - 2.1f) * -0.909091f);
                    let _e2881 = select(_e2879, 0f, (_e2879 < 0f));
                    let _e2883 = select(_e2881, 1f, (_e2881 > 1f));
                    let _e2888 = (((_e2883 * _e2883) * (3f - (2f * _e2883))) * 0.92f);
                    let _e2889 = (1f - _e2888);
                    let _e2900 = ((_e2524 - 0.55f) * -0.9090909f);
                    let _e2902 = select(_e2900, 0f, (_e2900 < 0f));
                    let _e2904 = select(_e2902, 1f, (_e2902 > 1f));
                    let _e2909 = (((_e2904 * _e2904) * (3f - (2f * _e2904))) * 0.78f);
                    let _e2910 = (1f - _e2909);
                    let _e2921 = ((_e2548 * select(_e2765, _e2775, _e2779)) * 0.045f);
                    phi_102_ = _e455;
                    phi_103_ = vec3<f32>(((((((((_e1717 * _e2867) + (_e2865 * 0.00328f)) * _e2889) + (_e2853 * _e2888)) * _e2910) + (_e2853 * _e2909)) + _e2921) + (((0.32f * _e2548) * _e2664) + ((0.78f * _e2548) * _e2738))), ((((((((_e1718 * _e2867) + (_e2865 * 0.00984f)) * _e2889) + (_e2854 * _e2888)) * _e2910) + (_e2854 * _e2909)) + _e2921) + (((0.68f * _e2548) * _e2664) + ((0.3f * _e2548) * _e2738))), ((((((((_e1719 * _e2867) + (_e2865 * 0.02132f)) * _e2889) + (_e2855 * _e2888)) * _e2910) + (_e2855 * _e2909)) + _e2921) + (_e2548 * (_e2664 + _e2738))));
                    phi_104_ = false;
                    break;
                }
                case 1: {
                    let _e2054 = pill_1.member[_e281].history_scroll;
                    switch bitcast<i32>(_e1383) {
                        case 0: {
                            phi_86_ = true;
                            phi_87_ = false;
                            phi_88_ = false;
                            break;
                        }
                        case 1: {
                            phi_86_ = true;
                            phi_87_ = false;
                            phi_88_ = false;
                            break;
                        }
                        case 2: {
                            phi_86_ = false;
                            phi_87_ = true;
                            phi_88_ = false;
                            break;
                        }
                        case 3: {
                            phi_86_ = false;
                            phi_87_ = true;
                            phi_88_ = false;
                            break;
                        }
                        case 4: {
                            phi_86_ = false;
                            phi_87_ = false;
                            phi_88_ = true;
                            break;
                        }
                        case 5: {
                            phi_86_ = false;
                            phi_87_ = false;
                            phi_88_ = true;
                            break;
                        }
                        default: {
                            phi_86_ = bool();
                            phi_87_ = bool();
                            phi_88_ = bool();
                            break;
                        }
                    }
                    let _e2057 = phi_86_;
                    let _e2059 = phi_87_;
                    let _e2061 = phi_88_;
                    let _e2062 = select(_e2059, false, _e2057);
                    let _e2068 = ((select(select(80f, 32f, _e2062), 24f, select(select(_e2061, false, _e2057), false, _e2062)) * 0.5f) - 4f);
                    let _e2069 = (_e308 - 8f);
                    let _e2072 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1408, _e1409), (_e2068 - _e2069), _e2069);
                    let _e2074 = ((_e2072 - 0.55f) * -0.9090909f);
                    let _e2076 = select(_e2074, 0f, (_e2074 < 0f));
                    let _e2078 = select(_e2076, 1f, (_e2076 > 1f));
                    let _e2082 = ((_e2078 * _e2078) * (3f - (2f * _e2078)));
                    let _e2083 = (_e2068 * 0.051282052f);
                    let _e2084 = (_e1408 + _e2068);
                    let _e2086 = ((_e2084 / _e2083) + _e2054);
                    let _e2088 = select(_e2086, 0f, (_e2086 < 0f));
                    let _e2090 = select(_e2088, 39f, (_e2088 > 39f));
                    let _e2091 = floor(_e2090);
                    let _e2096 = select(select(u32(_e2091), 0u, (_e2091 < 0f)), 4294967295u, (_e2091 > 4294967000f));
                    let _e2097 = (_e308 - 10f);
                    let _e2101 = (((f32(_e2096) - _e2054) * _e2083) - _e2068);
                    let _e2103 = select(_e2096, 39u, (39u < _e2096));
                    let _e2104 = (_e2103 < 40u);
                    if _e2104 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2111 = pill_1.member[_e281].gpu.usage.samples[_e2103];
                    let _e2114 = (_e2097 * (1f - (_e2111 * 2f)));
                    let _e2115 = (_e2096 + 1u);
                    let _e2121 = select(_e2115, 39u, (39u < _e2115));
                    let _e2122 = (_e2121 < 40u);
                    if _e2122 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2129 = pill_1.member[_e281].gpu.usage.samples[_e2121];
                    let _e2133 = ((((f32(_e2115) - _e2054) * _e2083) - _e2068) - _e2101);
                    let _e2134 = ((_e2097 * (1f - (_e2129 * 2f))) - _e2114);
                    let _e2135 = (_e1408 - _e2101);
                    let _e2136 = (_e1409 - _e2114);
                    let _e2137 = (_e2135 * _e2133);
                    let _e2140 = (_e2133 * _e2133);
                    let _e2142 = (_e2140 + (_e2134 * _e2134));
                    if (_e2142 != _e2142) {
                        phi_89_ = true;
                    } else {
                        phi_89_ = (0.001f >= _e2142);
                    }
                    let _e2146 = phi_89_;
                    let _e2148 = ((_e2137 + (_e2136 * _e2134)) / select(_e2142, 0.001f, _e2146));
                    let _e2150 = select(_e2148, 0f, (_e2148 < 0f));
                    let _e2152 = select(_e2150, 1f, (_e2150 > 1f));
                    let _e2155 = (_e2135 - (_e2133 * _e2152));
                    let _e2156 = (_e2136 - (_e2134 * _e2152));
                    let _e2163 = ((abs(sqrt(((_e2155 * _e2155) + (_e2156 * _e2156)))) - 1.4000001f) * -0.9090908f);
                    let _e2165 = select(_e2163, 0f, (_e2163 < 0f));
                    let _e2167 = select(_e2165, 1f, (_e2165 > 1f));
                    let _e2173 = (_e2090 - trunc(_e2090));
                    let _e2175 = select(_e2173, 0f, (_e2173 < 0f));
                    let _e2177 = select(_e2175, 1f, (_e2175 > 1f));
                    let _e2181 = ((_e2177 * _e2177) * (3f - (2f * _e2177)));
                    let _e2188 = ((((_e2114 + (_e2134 * _e2181)) - _e1409) - 0.55f) * -0.9090909f);
                    let _e2190 = select(_e2188, 0f, (_e2188 < 0f));
                    let _e2192 = select(_e2190, 1f, (_e2190 > 1f));
                    let _e2198 = ((((_e2192 * _e2192) * (3f - (2f * _e2192))) * 0.156f) + ((_e2167 * _e2167) * (3f - (2f * _e2167))));
                    if _e2104 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2207 = pill_1.member[_e281].gpu.memory.samples[_e2103];
                    let _e2210 = (_e2097 * (1f - (_e2207 * 2f)));
                    if _e2122 {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e2217 = pill_1.member[_e281].gpu.memory.samples[_e2121];
                    let _e2221 = ((_e2097 * (1f - (_e2217 * 2f))) - _e2210);
                    let _e2222 = (_e1409 - _e2210);
                    let _e2226 = (_e2140 + (_e2221 * _e2221));
                    if (_e2226 != _e2226) {
                        phi_90_ = true;
                    } else {
                        phi_90_ = (0.001f >= _e2226);
                    }
                    let _e2230 = phi_90_;
                    let _e2232 = ((_e2137 + (_e2222 * _e2221)) / select(_e2226, 0.001f, _e2230));
                    let _e2234 = select(_e2232, 0f, (_e2232 < 0f));
                    let _e2236 = select(_e2234, 1f, (_e2234 > 1f));
                    let _e2239 = (_e2135 - (_e2133 * _e2236));
                    let _e2240 = (_e2222 - (_e2221 * _e2236));
                    let _e2247 = ((abs(sqrt(((_e2239 * _e2239) + (_e2240 * _e2240)))) - 1.4000001f) * -0.9090908f);
                    let _e2249 = select(_e2247, 0f, (_e2247 < 0f));
                    let _e2251 = select(_e2249, 1f, (_e2249 > 1f));
                    let _e2262 = ((((_e2210 + (_e2221 * _e2181)) - _e1409) - 0.55f) * -0.9090909f);
                    let _e2264 = select(_e2262, 0f, (_e2262 < 0f));
                    let _e2266 = select(_e2264, 1f, (_e2264 > 1f));
                    let _e2272 = ((((_e2266 * _e2266) * (3f - (2f * _e2266))) * 0.084f) + ((_e2251 * _e2251) * (3f - (2f * _e2251))));
                    let _e2280 = (_e2084 * 0.14285715f);
                    let _e2281 = ((_e1409 + _e2069) * 0.16393442f);
                    let _e2291 = ((abs(((_e2280 - trunc(_e2280)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2293 = select(_e2291, 0f, (_e2291 < 0f));
                    let _e2295 = select(_e2293, 1f, (_e2293 > 1f));
                    let _e2299 = ((_e2295 * _e2295) * (3f - (2f * _e2295)));
                    let _e2301 = ((abs(((_e2281 - trunc(_e2281)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2303 = select(_e2301, 0f, (_e2301 < 0f));
                    let _e2305 = select(_e2303, 1f, (_e2303 > 1f));
                    let _e2309 = ((_e2305 * _e2305) * (3f - (2f * _e2305)));
                    if (_e2299 != _e2299) {
                        phi_91_ = true;
                    } else {
                        phi_91_ = (_e2309 >= _e2299);
                    }
                    let _e2313 = phi_91_;
                    let _e2321 = pill_1.member[_e281].gpu.usage.samples[39u];
                    let _e2322 = (_e2321 * 0.24f);
                    let _e2323 = (0.18f + _e2322);
                    let _e2324 = (0.82f - _e2322);
                    let _e2333 = (_e1641 - 60f);
                    let _e2334 = (_e2333 * 0.083333336f);
                    let _e2336 = select(_e2334, 0f, (_e2334 < 0f));
                    let _e2338 = select(_e2336, 1f, (_e2336 > 1f));
                    let _e2342 = ((_e2338 * _e2338) * (3f - (2f * _e2338)));
                    let _e2343 = (1f - _e2342);
                    let _e2352 = ((_e1641 - 72f) * 0.0625f);
                    let _e2354 = select(_e2352, 0f, (_e2352 < 0f));
                    let _e2356 = select(_e2354, 1f, (_e2354 > 1f));
                    let _e2360 = ((_e2356 * _e2356) * (3f - (2f * _e2356)));
                    let _e2361 = (1f - _e2360);
                    let _e2370 = (_e2333 * 0.03846154f);
                    let _e2372 = select(_e2370, 0f, (_e2370 < 0f));
                    let _e2374 = select(_e2372, 1f, (_e2372 > 1f));
                    let _e2379 = (((_e2374 * _e2374) * (3f - (2f * _e2374))) * 0.9f);
                    let _e2380 = (1f - _e2379);
                    let _e2391 = (1f - (_e2082 * 0.82f));
                    let _e2403 = ((abs(_e2072) - 2.1f) * -0.909091f);
                    let _e2405 = select(_e2403, 0f, (_e2403 < 0f));
                    let _e2407 = select(_e2405, 1f, (_e2405 > 1f));
                    let _e2412 = (((_e2407 * _e2407) * (3f - (2f * _e2407))) * 0.92f);
                    let _e2413 = (1f - _e2412);
                    let _e2424 = ((_e2082 * select(_e2299, _e2309, _e2313)) * 0.045f);
                    phi_102_ = _e455;
                    phi_103_ = vec3<f32>(((((((_e1717 * _e2391) + (_e2082 * 0.00328f)) * _e2413) + (((((0.025f * _e2324) + (0.32f * _e2323)) * _e2380) + (((((0.22f * _e2343) + _e2342) * _e2361) + _e2360) * _e2379)) * _e2412)) + _e2424) + (((0.32f * _e2082) * _e2198) + ((0.78f * _e2082) * _e2272))), ((((((_e1718 * _e2391) + (_e2082 * 0.00984f)) * _e2413) + (((((0.09f * _e2324) + (0.68f * _e2323)) * _e2380) + (((((0.62f * _e2343) + (0.38f * _e2342)) * _e2361) + (0.08f * _e2360)) * _e2379)) * _e2412)) + _e2424) + (((0.68f * _e2082) * _e2198) + ((0.3f * _e2082) * _e2272))), ((((((_e1719 * _e2391) + (_e2082 * 0.02132f)) * _e2413) + (((((0.15f * _e2324) + _e2323) * _e2380) + ((((_e2343 + (0.08f * _e2342)) * _e2361) + (0.035f * _e2360)) * _e2379)) * _e2412)) + _e2424) + (_e2082 * (_e2198 + _e2272))));
                    phi_104_ = false;
                    break;
                }
                case 2: {
                    let _e1846 = (_e1408 * 1.25f);
                    let _e1847 = (_e1409 * 1.25f);
                    let _e1849 = select(0f, 1f, (_e287 < 0f));
                    let _e1850 = abs(_e287);
                    let _e1851 = (_e1847 - 1f);
                    let _e1852 = vec2<f32>(_e1846, _e1851);
                    let _e1853 = cantus_render_shader_sd_rounded_box(_e1852, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e1856 = ((abs(_e1853) - 2.425f) * -0.909091f);
                    let _e1858 = select(_e1856, 0f, (_e1856 < 0f));
                    let _e1860 = select(_e1858, 1f, (_e1858 > 1f));
                    let _e1867 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1846, (_e1847 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e1869 = ((_e1867 - 0.55f) * -0.9090909f);
                    let _e1871 = select(_e1869, 0f, (_e1869 < 0f));
                    let _e1873 = select(_e1871, 1f, (_e1871 > 1f));
                    let _e1878 = cantus_render_shader_sd_rounded_box(_e1852, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e1880 = ((_e1878 - 0.55f) * -0.9090909f);
                    let _e1882 = select(_e1880, 0f, (_e1880 < 0f));
                    let _e1884 = select(_e1882, 1f, (_e1882 > 1f));
                    let _e1888 = ((_e1884 * _e1884) * (3f - (2f * _e1884)));
                    let _e1890 = select(_e1850, 0f, (_e1850 < 0f));
                    let _e1908 = ((12f - (select(_e1890, 1f, (_e1890 > 1f)) * 24f)) + ((sin(((_e1408 * 0.775f) + (_e639 * (1.4f + (_e1849 * 1.2f))))) * 1.15f) + (sin(((_e1408 * 0.3375f) - (_e639 * 0.8f))) * 0.45f)));
                    let _e1909 = (_e1908 - 0.7f);
                    let _e1913 = ((_e1851 - _e1909) / ((_e1908 + 0.7f) - _e1909));
                    let _e1915 = select(_e1913, 0f, (_e1913 < 0f));
                    let _e1917 = select(_e1915, 1f, (_e1915 > 1f));
                    let _e1922 = (_e1888 * ((_e1917 * _e1917) * (3f - (2f * _e1917))));
                    let _e1924 = ((_e1850 - 0.08f) * 5f);
                    let _e1926 = select(_e1924, 0f, (_e1924 < 0f));
                    let _e1928 = select(_e1926, 1f, (_e1926 > 1f));
                    let _e1932 = ((_e1928 * _e1928) * (3f - (2f * _e1928)));
                    let _e1933 = (1f - _e1932);
                    let _e1941 = ((_e1850 - 0.18f) * 1.8518518f);
                    let _e1943 = select(_e1941, 0f, (_e1941 < 0f));
                    let _e1945 = select(_e1943, 1f, (_e1943 > 1f));
                    let _e1949 = ((_e1945 * _e1945) * (3f - (2f * _e1945)));
                    let _e1950 = (1f - _e1949);
                    let _e1956 = (_e1950 + (0.22f * _e1949));
                    let _e1957 = ((((0.18f * _e1933) + (0.72f * _e1932)) * _e1950) + (0.95f * _e1949));
                    let _e1958 = ((((0.1f * _e1933) + (0.12f * _e1932)) * _e1950) + (0.55f * _e1949));
                    let _e1960 = floor((_e1408 * 0.4166667f));
                    let _e1962 = cantus_render_shader_hash(vec2<f32>(_e1960, 0f));
                    let _e1965 = (_e1962.y * 0.5f);
                    let _e1969 = ((_e639 * (0.35f + _e1965)) + (_e1962.x * 7f));
                    let _e1971 = (_e1969 - trunc(_e1969));
                    let _e1978 = (_e1846 - (((_e1960 + 0.2f) + (_e1962.x * 0.6f)) * 3f));
                    let _e1979 = (_e1847 - (13f - (_e1971 * 24f)));
                    let _e1986 = (_e1971 * 4f);
                    let _e1988 = select(_e1986, 0f, (_e1986 < 0f));
                    let _e1990 = select(_e1988, 1f, (_e1988 > 1f));
                    let _e1996 = ((_e1971 - 1f) * -3.3333333f);
                    let _e1998 = select(_e1996, 0f, (_e1996 < 0f));
                    let _e2000 = select(_e1998, 1f, (_e1998 > 1f));
                    let _e2008 = ((abs((sqrt(((_e1978 * _e1978) + (_e1979 * _e1979))) - (0.4f + _e1965))) - 1f) * -0.9090909f);
                    let _e2010 = select(_e2008, 0f, (_e2008 < 0f));
                    let _e2012 = select(_e2010, 1f, (_e2010 > 1f));
                    let _e2019 = (((((_e2012 * _e2012) * (3f - (2f * _e2012))) * (((_e1990 * _e1990) * (3f - (2f * _e1990))) * ((_e2000 * _e2000) * (3f - (2f * _e2000))))) * _e1888) * _e1849);
                    let _e2022 = ((((_e1860 * _e1860) * (3f - (2f * _e1860))) * 0.43f) + (((_e1873 * _e1873) * (3f - (2f * _e1873))) * 0.38f));
                    phi_102_ = _e455;
                    phi_103_ = vec3<f32>((_e1717 + ((_e2022 + ((_e1956 * _e1922) * 0.78f)) + ((((_e1956 * 0.27999997f) + 0.72f) * _e2019) * 0.9f))), (_e1718 + ((_e2022 + ((_e1957 * _e1922) * 0.78f)) + ((((_e1957 * 0.27999997f) + 0.72f) * _e2019) * 0.9f))), (_e1719 + ((_e2022 + ((_e1958 * _e1922) * 0.78f)) + ((((_e1958 * 0.27999997f) + 0.72f) * _e2019) * 0.9f))));
                    phi_104_ = false;
                    break;
                }
                case 3: {
                    let _e1724 = pill_1.member[_e281].volume;
                    let _e1726 = select(0f, 1f, (_e1724 < 0f));
                    let _e1727 = abs(_e1724);
                    let _e1730 = round(((_e1408 + 12f) * 0.25f));
                    let _e1732 = select(_e1730, 0f, (_e1730 < 0f));
                    let _e1734 = select(_e1732, 6f, (_e1732 > 6f));
                    let _e1739 = select(select(u32(_e1734), 0u, (_e1734 < 0f)), 4294967295u, (_e1734 > 4294967000f));
                    if (_e1739 < 7u) {
                    } else {
                        phi_102_ = true;
                        phi_103_ = vec3<f32>();
                        phi_104_ = bool();
                        break;
                    }
                    let _e1745 = pill_1.member[_e281].audio_spectrum[_e1739];
                    let _e1746 = (1f - _e1726);
                    let _e1747 = (_e1745 * _e1746);
                    let _e1756 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e1408 - (-12f + (_e1734 * 4f))), (_e1409 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1747))), 1.25f);
                    let _e1759 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1408, (_e1409 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1761 = ((_e1759 - 0.55f) * -0.9090909f);
                    let _e1763 = select(_e1761, 0f, (_e1761 < 0f));
                    let _e1765 = select(_e1763, 1f, (_e1763 > 1f));
                    let _e1769 = ((_e1765 * _e1765) * (3f - (2f * _e1765)));
                    let _e1771 = select(_e1727, 0f, (_e1727 < 0f));
                    let _e1774 = (select(_e1771, 1f, (_e1771 > 1f)) * 28f);
                    let _e1775 = (_e1774 + -13.2f);
                    let _e1779 = ((_e1408 - _e1775) / ((_e1774 + -14.8f) - _e1775));
                    let _e1781 = select(_e1779, 0f, (_e1779 < 0f));
                    let _e1783 = select(_e1781, 1f, (_e1781 > 1f));
                    let _e1788 = (_e1769 * ((_e1783 * _e1783) * (3f - (2f * _e1783))));
                    let _e1790 = (1f - (_e1727 * 0.65f));
                    let _e1795 = ((0.08f * _e1790) + (_e1727 * 0.42249995f));
                    let _e1796 = ((0.88f * _e1790) + (_e1727 * 0.221f));
                    let _e1798 = ((_e1756 - 0.7f) * -0.71428573f);
                    let _e1800 = select(_e1798, 0f, (_e1798 < 0f));
                    let _e1802 = select(_e1800, 1f, (_e1800 > 1f));
                    let _e1811 = ((_e1756 - 3.2f) * -0.3125f);
                    let _e1813 = select(_e1811, 0f, (_e1811 < 0f));
                    let _e1815 = select(_e1813, 1f, (_e1813 > 1f));
                    let _e1822 = ((((_e1802 * _e1802) * (3f - (2f * _e1802))) * (0.58f + (_e1747 * 0.35f))) + ((((_e1815 * _e1815) * (3f - (2f * _e1815))) * _e1747) * 0.12f));
                    let _e1835 = (_e1788 + ((_e1769 * (1f - _e1788)) * 0.22f));
                    phi_102_ = _e455;
                    phi_103_ = vec3<f32>((_e1717 + ((_e1795 * _e1822) + (((_e1795 * _e1746) + _e1726) * _e1835))), (_e1718 + ((_e1796 * _e1822) + (((_e1796 * _e1746) + (0.24f * _e1726)) * _e1835))), (_e1719 + (_e1822 + ((_e1746 + (0.3f * _e1726)) * _e1835))));
                    phi_104_ = false;
                    break;
                }
                case 4: {
                    phi_102_ = _e455;
                    phi_103_ = vec3<f32>();
                    phi_104_ = true;
                    break;
                }
                case 5: {
                    phi_102_ = _e455;
                    phi_103_ = vec3<f32>();
                    phi_104_ = true;
                    break;
                }
                default: {
                    phi_102_ = _e455;
                    phi_103_ = vec3<f32>();
                    phi_104_ = bool();
                    break;
                }
            }
            let _e2930 = phi_102_;
            let _e2932 = phi_103_;
            let _e2934 = phi_104_;
            if _e2930 {
                break;
            }
            if _e2934 {
                let _e2936 = select(1f, 0f, (_e1383 == 5u));
                let _e2940 = pill_1.member[_e281].power_hover;
                let _e2945 = ((abs((f32(_e2940) - _e2936)) - 0.4f) * -2.857143f);
                let _e2947 = select(_e2945, 0f, (_e2945 < 0f));
                let _e2949 = select(_e2947, 1f, (_e2947 > 1f));
                let _e2953 = ((_e2949 * _e2949) * (3f - (2f * _e2949)));
                let _e2955 = (1f + (_e2953 * 0.07f));
                let _e2956 = (_e1408 / _e2955);
                let _e2957 = (_e1409 / _e2955);
                let _e2961 = pill_1.member[_e281].power_action;
                let _e2966 = ((abs((f32(_e2961) - _e2936)) - 0.4f) * -2.857143f);
                let _e2968 = select(_e2966, 0f, (_e2966 < 0f));
                let _e2970 = select(_e2968, 1f, (_e2968 > 1f));
                let _e2974 = ((_e2970 * _e2970) * (3f - (2f * _e2970)));
                let _e2978 = pill_1.member[_e281].power_progress;
                let _e2979 = (_e2978 * _e2974);
                if (_e2936 < 0.5f) {
                    let _e3103 = select(_e2979, 0f, (_e2979 < 0f));
                    let _e3105 = select(_e3103, 1f, (_e3103 > 1f));
                    let _e3109 = ((_e3105 * _e3105) * (3f - (2f * _e3105)));
                    let _e3115 = (1f - _e2979);
                    let _e3124 = (_e3109 * 0.7f);
                    let _e3125 = (_e3124 + 1.5999999f);
                    let _e3130 = ((abs((sqrt(((_e2956 * _e2956) + (_e2957 * _e2957))) - ((7.5f - (_e2979 * 4.6f)) + (((sin((_e639 * 8f)) * _e2979) * _e3115) * 0.16f)))) - _e3125) / ((_e3124 + 0.49999994f) - _e3125));
                    let _e3132 = select(_e3130, 0f, (_e3130 < 0f));
                    let _e3134 = select(_e3132, 1f, (_e3132 > 1f));
                    let _e3143 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2956, (_e2957 - -7f)), vec2<f32>((3f * _e3115), 3f), 0.5f);
                    let _e3145 = ((_e3143 - 0.55f) * -0.9090909f);
                    let _e3147 = select(_e3145, 0f, (_e3145 < 0f));
                    let _e3149 = select(_e3147, 1f, (_e3147 > 1f));
                    let _e3163 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2956, (_e2957 - (-5f + (_e2979 * 3.5f)))), vec2<f32>((1.05f + (_e3109 * 0.45f)), (4.6f - (_e2979 * 3f))), 0.7f);
                    let _e3165 = ((_e3163 - 0.55f) * -0.9090909f);
                    let _e3167 = select(_e3165, 0f, (_e3165 < 0f));
                    let _e3169 = select(_e3167, 1f, (_e3167 > 1f));
                    let _e3173 = ((_e3169 * _e3169) * (3f - (2f * _e3169)));
                    let _e3175 = (((_e3134 * _e3134) * (3f - (2f * _e3134))) * (1f - ((_e3149 * _e3149) * (3f - (2f * _e3149)))));
                    if (_e3175 != _e3175) {
                        phi_108_ = true;
                    } else {
                        phi_108_ = (_e3173 >= _e3175);
                    }
                    let _e3179 = phi_108_;
                    phi_109_ = select(_e3175, _e3173, _e3179);
                } else {
                    let _e2982 = ((1f - _e2974) + _e2979);
                    let _e2986 = (((atan2(_e2957, _e2956) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e2990 = ((_e2982 * 0.82f) - 0.045f);
                    if (_e2990 != _e2990) {
                        phi_105_ = true;
                    } else {
                        phi_105_ = (0f >= _e2990);
                    }
                    let _e2994 = phi_105_;
                    let _e2995 = select(_e2990, 0f, _e2994);
                    let _e3003 = ((abs((sqrt(((_e2956 * _e2956) + (_e2957 * _e2957))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e3005 = select(_e3003, 0f, (_e3003 < 0f));
                    let _e3007 = select(_e3005, 1f, (_e3005 > 1f));
                    let _e3012 = (_e2995 + 0.008f);
                    let _e3016 = (((_e2986 - trunc(_e2986)) - _e3012) / ((_e2995 - 0.008f) - _e3012));
                    let _e3018 = select(_e3016, 0f, (_e3016 < 0f));
                    let _e3020 = select(_e3018, 1f, (_e3018 > 1f));
                    let _e3026 = (_e2982 * 50f);
                    let _e3028 = select(_e3026, 0f, (_e3026 < 0f));
                    let _e3030 = select(_e3028, 1f, (_e3028 > 1f));
                    let _e3035 = ((((_e3007 * _e3007) * (3f - (2f * _e3007))) * ((_e3020 * _e3020) * (3f - (2f * _e3020)))) * ((_e3030 * _e3030) * (3f - (2f * _e3030))));
                    let _e3037 = (0.50265485f + (5.152212f * _e2982));
                    let _e3038 = cos(_e3037);
                    let _e3039 = sin(_e3037);
                    let _e3043 = (_e2956 - (_e3038 * 7.1f));
                    let _e3044 = (_e2957 - (_e3039 * 7.1f));
                    let _e3047 = ((_e3043 * -(_e3039)) + (_e3044 * _e3038));
                    let _e3050 = ((_e3043 * _e3038) + (_e3044 * _e3039));
                    let _e3051 = (_e3047 * -3.2f);
                    let _e3054 = ((_e3051 + (_e3050 * 2.1f)) * 0.06825939f);
                    let _e3056 = select(_e3054, 0f, (_e3054 < 0f));
                    let _e3058 = select(_e3056, 1f, (_e3056 > 1f));
                    let _e3061 = (_e3047 - (-3.2f * _e3058));
                    let _e3062 = (_e3050 - (2.1f * _e3058));
                    let _e3066 = sqrt(((_e3061 * _e3061) + (_e3062 * _e3062)));
                    let _e3069 = ((_e3051 + (_e3050 * -2.1f)) * 0.06825939f);
                    let _e3071 = select(_e3069, 0f, (_e3069 < 0f));
                    let _e3073 = select(_e3071, 1f, (_e3071 > 1f));
                    let _e3076 = (_e3047 - (-3.2f * _e3073));
                    let _e3077 = (_e3050 - (-2.1f * _e3073));
                    let _e3081 = sqrt(((_e3076 * _e3076) + (_e3077 * _e3077)));
                    if (_e3066 != _e3066) {
                        phi_106_ = true;
                    } else {
                        phi_106_ = (_e3081 <= _e3066);
                    }
                    let _e3085 = phi_106_;
                    let _e3088 = ((select(_e3066, _e3081, _e3085) - 1.7f) * -0.71428573f);
                    let _e3090 = select(_e3088, 0f, (_e3088 < 0f));
                    let _e3092 = select(_e3090, 1f, (_e3090 > 1f));
                    let _e3096 = ((_e3092 * _e3092) * (3f - (2f * _e3092)));
                    if (_e3035 != _e3035) {
                        phi_107_ = true;
                    } else {
                        phi_107_ = (_e3096 >= _e3035);
                    }
                    let _e3100 = phi_107_;
                    phi_109_ = select(_e3035, _e3096, _e3100);
                }
                let _e3182 = phi_109_;
                let _e3185 = (_e2974 * (0.5f + (_e2979 * 0.5f)));
                if (_e2953 != _e2953) {
                    phi_110_ = true;
                } else {
                    phi_110_ = (_e3185 >= _e2953);
                }
                let _e3189 = phi_110_;
                let _e3190 = select(_e2953, _e3185, _e3189);
                let _e3192 = (0.48f * (1f - _e3190));
                let _e3203 = (1f + (_e2979 * 0.45f));
                phi_111_ = vec3<f32>((_e1717 + (((_e3192 + (0.78f * _e3190)) * _e3182) * _e3203)), (_e1718 + (((_e3192 + (0.3f * _e3190)) * _e3182) * _e3203)), (_e1719 + (((_e3192 + (0.28f * _e3190)) * _e3182) * _e3203)));
            } else {
                phi_111_ = _e2932;
            }
            let _e3212 = phi_111_;
            let _e3214 = local_38;
            let _e3216 = (1f - (_e3214 * 0.35f));
            let _e3224 = local_39;
            let _e3225 = (_e3224 * 0.33249998f);
            switch bitcast<i32>(_e1383) {
                case 0: {
                    let _e3239 = pill_1.member[_e281].labels[0u];
                    phi_112_ = _e3239;
                    break;
                }
                case 1: {
                    let _e3234 = pill_1.member[_e281].labels[1u];
                    phi_112_ = _e3234;
                    break;
                }
                default: {
                    phi_112_ = render_text_Line(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), 0f, 0f, 0u, 0u, 0u);
                    break;
                }
            }
            let _e3241 = phi_112_;
            switch bitcast<i32>(_e1383) {
                case 0: {
                    phi_113_ = true;
                    break;
                }
                case 1: {
                    phi_113_ = true;
                    break;
                }
                default: {
                    phi_113_ = false;
                    break;
                }
            }
            let _e3244 = phi_113_;
            if _e3244 {
                let _e3246 = (1f / _e3241.size);
                let _e3253 = ((_e583 - _e3241.origin.x) * _e3246);
                phi_114_ = 0u;
                phi_115_ = _e3241.count;
                loop {
                    let _e3258 = phi_114_;
                    let _e3260 = phi_115_;
                    local_40 = _e3258;
                    let _e3261 = (_e3258 < _e3260);
                    if _e3261 {
                        let _e3264 = (_e3258 + ((_e3260 - _e3258) / 2u));
                        let _e3269 = placed_glyphs_1.member[(_e3241.first + _e3264)].x;
                        let _e3270 = (_e3269 <= _e3253);
                        if _e3270 {
                            phi_116_ = (_e3264 + 1u);
                        } else {
                            phi_116_ = _e3258;
                        }
                        let _e3273 = phi_116_;
                        phi_117_ = _e3273;
                        phi_118_ = select(_e3264, _e3260, _e3270);
                    } else {
                        phi_117_ = u32();
                        phi_118_ = u32();
                    }
                    let _e3276 = phi_117_;
                    let _e3278 = phi_118_;
                    continue;
                    continuing {
                        phi_114_ = _e3276;
                        phi_115_ = _e3278;
                        break if !(_e3261);
                    }
                }
                let _e3280 = (3.5f / _e3241.size);
                let _e3282 = local_40;
                let _e3283 = (_e3282 + 1u);
                phi_119_ = select(_e3283, _e3241.count, (_e3241.count < _e3283));
                phi_120_ = -1000000f;
                loop {
                    let _e3287 = phi_119_;
                    let _e3289 = phi_120_;
                    local_43 = _e3289;
                    if (_e3287 > 0u) {
                        let _e3291 = (_e3287 - 1u);
                        let _e3292 = (_e3241.first + _e3291);
                        let _e3296 = placed_glyphs_1.member[_e3292].x;
                        let _e3300 = placed_glyphs_1.member[_e3292].glyph;
                        let _e3305 = glyphs_1.member[_e3300].min[0u];
                        let _e3310 = glyphs_1.member[_e3300].min[1u];
                        let _e3315 = glyphs_1.member[_e3300].max[0u];
                        let _e3320 = glyphs_1.member[_e3300].max[1u];
                        let _e3324 = glyphs_1.member[_e3300].start;
                        let _e3328 = glyphs_1.member[_e3300].count;
                        let _e3329 = (_e3253 - _e3296);
                        let _e3330 = -(((_e584 - _e3241.origin.y) * _e3246));
                        let _e3331 = (_e3315 + _e3280);
                        let _e3332 = (_e3329 > _e3331);
                        if _e3332 {
                            phi_133_ = f32();
                        } else {
                            if (_e3329 >= (_e3305 - _e3280)) {
                                if (_e3330 >= (_e3310 - _e3280)) {
                                    if (_e3329 <= _e3331) {
                                        if (_e3330 <= (_e3320 + _e3280)) {
                                            phi_121_ = 340282350000000000000000000000000000000f;
                                            phi_122_ = 0u;
                                            phi_123_ = 0i;
                                            loop {
                                                let _e3342 = phi_121_;
                                                let _e3344 = phi_122_;
                                                let _e3346 = phi_123_;
                                                local_41 = _e3342;
                                                local_42 = _e3346;
                                                let _e3347 = (_e3344 < _e3328);
                                                if _e3347 {
                                                    let _e3351 = edges_1.member[(_e3324 + _e3344)];
                                                    let _e3353 = cantus_render_text_edge_distance(_e3351, _e3241.weight, vec2<f32>(_e3329, _e3330), _e3342);
                                                    phi_124_ = _e3353.member;
                                                    phi_125_ = (_e3344 + 1u);
                                                    phi_126_ = (_e3346 + _e3353.member_1);
                                                } else {
                                                    phi_124_ = f32();
                                                    phi_125_ = u32();
                                                    phi_126_ = i32();
                                                }
                                                let _e3359 = phi_124_;
                                                let _e3361 = phi_125_;
                                                let _e3363 = phi_126_;
                                                continue;
                                                continuing {
                                                    phi_121_ = _e3359;
                                                    phi_122_ = _e3361;
                                                    phi_123_ = _e3363;
                                                    break if !(_e3347);
                                                }
                                            }
                                            let _e3366 = local_41;
                                            let _e3368 = ((_e3366 * _e3241.size) * _e3241.size);
                                            if (_e3368 >= 12.25f) {
                                                phi_127_ = 3.5f;
                                            } else {
                                                phi_127_ = sqrt(_e3368);
                                            }
                                            let _e3372 = phi_127_;
                                            let _e3374 = local_42;
                                            let _e3377 = (_e3372 * select(1f, -1f, (_e3374 == 0i)));
                                            if (_e3289 != _e3289) {
                                                phi_128_ = true;
                                            } else {
                                                phi_128_ = (_e3377 >= _e3289);
                                            }
                                            let _e3381 = phi_128_;
                                            phi_129_ = select(_e3289, _e3377, _e3381);
                                        } else {
                                            phi_129_ = _e3289;
                                        }
                                        let _e3384 = phi_129_;
                                        phi_130_ = _e3384;
                                    } else {
                                        phi_130_ = _e3289;
                                    }
                                    let _e3386 = phi_130_;
                                    phi_131_ = _e3386;
                                } else {
                                    phi_131_ = _e3289;
                                }
                                let _e3388 = phi_131_;
                                phi_132_ = _e3388;
                            } else {
                                phi_132_ = _e3289;
                            }
                            let _e3390 = phi_132_;
                            phi_133_ = _e3390;
                        }
                        let _e3392 = phi_133_;
                        phi_134_ = _e3291;
                        phi_135_ = _e3392;
                        phi_136_ = select(true, false, _e3332);
                    } else {
                        phi_134_ = u32();
                        phi_135_ = f32();
                        phi_136_ = false;
                    }
                    let _e3395 = phi_134_;
                    let _e3397 = phi_135_;
                    let _e3399 = phi_136_;
                    continue;
                    continuing {
                        phi_119_ = _e3395;
                        phi_120_ = _e3397;
                        break if !(_e3399);
                    }
                }
                let _e3402 = local_43;
                let _e3404 = ((_e3402 * 1.25f) + 0.5f);
                let _e3406 = select(_e3404, 0f, (_e3404 < 0f));
                let _e3408 = select(_e3406, 1f, (_e3406 > 1f));
                phi_137_ = ((_e3408 * _e3408) * (3f - (2f * _e3408)));
            } else {
                phi_137_ = 0f;
            }
            let _e3414 = phi_137_;
            let _e3415 = (1f - _e3414);
            let _e3419 = (0.94f * _e3414);
            out_color = vec4<f32>((((((_e3212.x * _e3216) + _e3225) * _e3415) + _e3419) * _e544), (((((_e3212.y * _e3216) + _e3225) * _e3415) + _e3419) * _e544), (((((_e3212.z * _e3216) + _e3225) * _e3415) + _e3419) * _e544), _e557);
            break;
        }
    }
    return;
}

fn render_launcher_isthmus_launcherpass_vertex_impl() {
    var phi_0_: u32;
    var phi_1_: f32;
    var phi_2_: u32;
    var phi_3_: f32;
    var phi_4_: bool;
    var local_44: f32;
    var phi_5_: isthmus_Vertex_render_text_Varyings;

    switch bitcast<i32>(0u) {
        default: {
            let _e30 = vertex_7;
            let _e31 = instance_2;
            let _e35 = row.member[_e31].icon;
            if (_e35 == -3i) {
                let _e135 = frame.member[0u].screen_size[0u];
                let _e140 = frame.member[0u].screen_size[1u];
                let _e144 = frame.member[0u].panel_height;
                let _e148 = (((_e144 + 36f) + (8f * _e144)) + 56f);
                let _e160 = (((_e135 - 520f) * 0.5f) + (f32((_e30 & 1u)) * 520f));
                let _e161 = (((_e140 - _e148) * 0.5f) + (f32((_e30 >> bitcast<u32>(1i))) * _e148));
                phi_5_ = isthmus_Vertex_render_text_Varyings(vec4<f32>((((_e160 / _e135) * 2f) - 1f), (1f - ((_e161 / _e140) * 2f)), 0f, 1f), vec2<f32>(_e160, _e161));
            } else {
                let _e41 = frame.member[0u].screen_size[0u];
                let _e46 = frame.member[0u].screen_size[1u];
                let _e50 = frame.member[0u].panel_height;
                let _e57 = row.member[_e31].y;
                let _e61 = frame.member[0u].mouse_pressure;
                phi_0_ = 0u;
                phi_1_ = (_e61 * 8f);
                loop {
                    let _e64 = phi_0_;
                    let _e66 = phi_1_;
                    local_44 = _e66;
                    let _e67 = (_e64 < 4u);
                    if _e67 {
                        if _e67 {
                        } else {
                            phi_4_ = true;
                            break;
                        }
                        let _e73 = frame.member[0u].ripples[_e64].start_time;
                        let _e79 = frame.member[0u].ripples[_e64].strength;
                        let _e83 = frame.member[0u].time;
                        let _e85 = ((_e83 - _e73) * 1.2f);
                        let _e87 = select(_e85, 0f, (_e85 < 0f));
                        let _e90 = (1f - select(_e87, 1f, (_e87 > 1f)));
                        phi_2_ = (_e64 + 1u);
                        phi_3_ = (_e66 + (((_e79 * _e90) * _e90) * 11f));
                    } else {
                        phi_2_ = u32();
                        phi_3_ = f32();
                    }
                    let _e97 = phi_2_;
                    let _e99 = phi_3_;
                    continue;
                    continuing {
                        phi_0_ = _e97;
                        phi_1_ = _e99;
                        phi_4_ = false;
                        break if !(_e67);
                    }
                }
                let _e102 = phi_4_;
                if _e102 {
                    break;
                }
                let _e104 = local_44;
                let _e106 = (18f + (_e104 * 0.5f));
                let _e120 = (((((_e41 - 520f) * 0.5f) + 12f) - _e106) + (f32((_e30 & 1u)) * (496f + (_e106 * 2f))));
                let _e121 = ((_e57 - _e106) + (f32((_e30 >> bitcast<u32>(1i))) * ((_e50 + _e106) * 2f)));
                phi_5_ = isthmus_Vertex_render_text_Varyings(vec4<f32>((((_e120 / _e41) * 2f) - 1f), (1f - ((_e121 / _e46) * 2f)), 0f, 1f), vec2<f32>(_e120, _e121));
            }
            let _e172 = phi_5_;
            out_position = _e172.position;
            out_pixel[0u] = _e172.varyings.x;
            out_pixel[1u] = _e172.varyings.y;
            out_row_idx = _e31;
            break;
        }
    }
    return;
}

fn render_launcher_isthmus_launcherpass_fragment_impl() {
    var phi_0_: f32;
    var phi_1_: vec2<f32>;
    var phi_2_: u32;
    var phi_3_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_4_: vec2<f32>;
    var phi_5_: vec2<f32>;
    var phi_6_: vec2<f32>;
    var phi_7_: u32;
    var phi_8_: bool;
    var phi_9_: f32;
    var local_45: vec2<f32>;
    var local_46: vec2<f32>;
    var phi_10_: bool;
    var local_47: vec2<f32>;
    var phi_11_: f32;
    var local_48: vec2<f32>;
    var phi_12_: bool;
    var phi_13_: bool;
    var phi_14_: bool;
    var phi_15_: bool;
    var phi_16_: bool;
    var phi_17_: vec3<f32>;
    var phi_18_: vec3<f32>;
    var phi_19_: bool;
    var phi_20_: bool;
    var phi_21_: vec3<f32>;
    var phi_22_: u32;
    var phi_23_: vec3<f32>;
    var phi_24_: bool;
    var phi_25_: bool;
    var phi_26_: bool;
    var phi_27_: bool;
    var phi_28_: bool;
    var phi_29_: bool;
    var phi_30_: bool;
    var phi_31_: bool;
    var phi_32_: bool;
    var phi_33_: f32;
    var phi_34_: bool;
    var phi_35_: bool;
    var phi_36_: vec4<f32>;
    var phi_37_: u32;
    var phi_38_: vec3<f32>;
    var phi_39_: bool;
    var phi_40_: u32;
    var phi_41_: vec3<f32>;
    var phi_42_: u32;
    var phi_43_: u32;
    var phi_44_: u32;
    var phi_45_: u32;
    var phi_46_: u32;
    var local_49: u32;
    var phi_47_: u32;
    var phi_48_: f32;
    var phi_49_: f32;
    var phi_50_: u32;
    var phi_51_: i32;
    var phi_52_: f32;
    var phi_53_: u32;
    var phi_54_: i32;
    var local_50: f32;
    var phi_55_: f32;
    var local_51: i32;
    var phi_56_: bool;
    var phi_57_: f32;
    var phi_58_: f32;
    var phi_59_: f32;
    var phi_60_: f32;
    var phi_61_: f32;
    var phi_62_: u32;
    var phi_63_: f32;
    var phi_64_: bool;
    var local_52: f32;
    var phi_65_: u32;
    var phi_66_: vec3<f32>;
    var phi_67_: bool;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var local_55: vec3<f32>;
    var phi_68_: f32;
    var phi_69_: u32;
    var phi_70_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_71_: bool;
    var phi_72_: f32;
    var phi_73_: f32;
    var phi_74_: f32;
    var phi_75_: u32;
    var phi_76_: bool;
    var local_56: f32;
    var local_57: f32;
    var phi_77_: bool;
    var phi_78_: u32;
    var phi_79_: u32;
    var phi_80_: u32;
    var phi_81_: u32;
    var phi_82_: u32;
    var local_58: u32;
    var phi_83_: u32;
    var phi_84_: f32;
    var phi_85_: f32;
    var phi_86_: u32;
    var phi_87_: i32;
    var phi_88_: f32;
    var phi_89_: u32;
    var phi_90_: i32;
    var local_59: f32;
    var phi_91_: f32;
    var local_60: i32;
    var phi_92_: bool;
    var phi_93_: f32;
    var phi_94_: f32;
    var phi_95_: f32;
    var phi_96_: f32;
    var phi_97_: f32;
    var phi_98_: u32;
    var phi_99_: f32;
    var phi_100_: bool;
    var local_61: f32;
    var phi_101_: vec4<f32>;
    var local_62: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e130 = pixel_4;
            let _e131 = row_idx_1;
            let _e137 = row.member[_e131].icon;
            if (_e137 == -3i) {
                let _e1098 = frame.member[0u].screen_size[0u];
                let _e1103 = frame.member[0u].screen_size[1u];
                let _e1107 = frame.member[0u].panel_height;
                let _e1111 = (((_e1107 + 36f) + (8f * _e1107)) + 56f);
                let _e1116 = (_e130.x - ((_e1098 - 520f) * 0.5f));
                let _e1117 = (_e130.y - ((_e1103 - _e1111) * 0.5f));
                let _e1118 = (_e1111 * 0.5f);
                let _e1119 = (_e1116 - 260f);
                let _e1123 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1119, (_e1117 - _e1118)), vec2<f32>(260f, _e1118), 16f);
                let _e1125 = ((_e1123 - 0.55f) * -0.9090909f);
                let _e1127 = select(_e1125, 0f, (_e1125 < 0f));
                let _e1129 = select(_e1127, 1f, (_e1127 > 1f));
                let _e1133 = ((_e1129 * _e1129) * (3f - (2f * _e1129)));
                if (_e1133 <= 0f) {
                    discard;
                }
                let _e1139 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1119, (_e1117 - (_e1107 + 11.5f))), vec2<f32>(260f, 0.5f), 0f);
                let _e1141 = ((_e1139 - 0.55f) * -0.9090909f);
                let _e1143 = select(_e1141, 0f, (_e1141 < 0f));
                let _e1145 = select(_e1143, 1f, (_e1143 > 1f));
                let _e1149 = ((_e1145 * _e1145) * (3f - (2f * _e1145)));
                let _e1153 = ((0.09f * (1f - _e1149)) + (0.17f * _e1149));
                phi_68_ = 0f;
                phi_69_ = 0u;
                loop {
                    let _e1157 = phi_68_;
                    let _e1159 = phi_69_;
                    local_56 = _e1157;
                    local_57 = _e1157;
                    let _e1160 = (_e1159 < 4u);
                    if _e1160 {
                        if _e1160 {
                        } else {
                            phi_76_ = true;
                            break;
                        }
                        let _e1167 = frame.member[0u].ripples[_e1159].origin[0u];
                        let _e1174 = frame.member[0u].ripples[_e1159].origin[1u];
                        let _e1180 = frame.member[0u].ripples[_e1159].start_time;
                        let _e1186 = frame.member[0u].ripples[_e1159].strength;
                        let _e1190 = frame.member[0u].time;
                        let _e1192 = ((_e1190 - _e1180) * 1.2f);
                        let _e1194 = select(_e1192, 0f, (_e1192 < 0f));
                        let _e1196 = select(_e1194, 1f, (_e1194 > 1f));
                        if (_e1186 > 0f) {
                            if (_e1196 < 1f) {
                                let _e1199 = (_e130.x - _e1167);
                                let _e1200 = (_e130.y - _e1174);
                                let _e1204 = sqrt(((_e1199 * _e1199) + (_e1200 * _e1200)));
                                if (_e1204 > 0.001f) {
                                    phi_70_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e1199 / _e1204), (_e1200 / _e1204)), _e1204);
                                } else {
                                    phi_70_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e1204);
                                }
                                let _e1212 = phi_70_;
                                let _e1218 = ((abs((_e1212.unnamed_1 - (_e1196 * 600f))) - 80f) * -0.0125f);
                                let _e1220 = select(_e1218, 0f, (_e1218 < 0f));
                                let _e1222 = select(_e1220, 1f, (_e1220 > 1f));
                                let _e1231 = (_e1157 + (((((_e1222 * _e1222) * (3f - (2f * _e1222))) * _e1186) * (1f - _e1196)) * 0.5f));
                                if (_e1231 != _e1231) {
                                    phi_71_ = true;
                                } else {
                                    phi_71_ = (1f <= _e1231);
                                }
                                let _e1235 = phi_71_;
                                phi_72_ = select(_e1231, 1f, _e1235);
                            } else {
                                phi_72_ = _e1157;
                            }
                            let _e1238 = phi_72_;
                            phi_73_ = _e1238;
                        } else {
                            phi_73_ = _e1157;
                        }
                        let _e1240 = phi_73_;
                        phi_74_ = _e1240;
                        phi_75_ = (_e1159 + 1u);
                    } else {
                        phi_74_ = f32();
                        phi_75_ = u32();
                    }
                    let _e1243 = phi_74_;
                    let _e1245 = phi_75_;
                    continue;
                    continuing {
                        phi_68_ = _e1243;
                        phi_69_ = _e1245;
                        phi_76_ = false;
                        break if !(_e1160);
                    }
                }
                let _e1248 = phi_76_;
                if _e1248 {
                    break;
                }
                let _e1250 = local_56;
                let _e1254 = local_57;
                let _e1258 = (_e1116 - 23f);
                let _e1259 = (_e1117 - ((_e1107 + 12f) * 0.5f));
                let _e1267 = ((abs((sqrt(((_e1258 * _e1258) + (_e1259 * _e1259))) - 6.2f)) - 1.5999999f) * -0.909091f);
                let _e1269 = select(_e1267, 0f, (_e1267 < 0f));
                let _e1271 = select(_e1269, 1f, (_e1269 > 1f));
                let _e1275 = ((_e1271 * _e1271) * (3f - (2f * _e1271)));
                let _e1276 = (_e1116 - 27.6f);
                let _e1277 = (_e1259 - 4.6f);
                let _e1279 = ((_e1276 + _e1277) * 0.119047605f);
                let _e1281 = select(_e1279, 0f, (_e1279 < 0f));
                let _e1284 = (4.2000003f * select(_e1281, 1f, (_e1281 > 1f)));
                let _e1285 = (_e1276 - _e1284);
                let _e1286 = (_e1277 - _e1284);
                let _e1293 = ((abs(sqrt(((_e1285 * _e1285) + (_e1286 * _e1286)))) - 1.5999999f) * -0.909091f);
                let _e1295 = select(_e1293, 0f, (_e1293 < 0f));
                let _e1297 = select(_e1295, 1f, (_e1295 > 1f));
                let _e1301 = ((_e1297 * _e1297) * (3f - (2f * _e1297)));
                if (_e1275 != _e1275) {
                    phi_77_ = true;
                } else {
                    phi_77_ = (_e1301 >= _e1275);
                }
                let _e1305 = phi_77_;
                let _e1306 = select(_e1275, _e1301, _e1305);
                let _e1315 = row.member[_e131].selection[1u];
                let _e1320 = row.member[_e131].selection[0u];
                let _e1321 = (_e1315 - _e1320);
                let _e1328 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e1116 - ((_e1320 + _e1315) * 0.5f)), _e1259), vec2<f32>((_e1321 * 0.5f), 13f), 3f);
                let _e1330 = ((_e1328 - 0.55f) * -0.9090909f);
                let _e1332 = select(_e1330, 0f, (_e1330 < 0f));
                let _e1334 = select(_e1332, 1f, (_e1332 > 1f));
                let _e1341 = (((_e1334 * _e1334) * (3f - (2f * _e1334))) * select(0f, 1f, (_e1321 > 0f)));
                let _e1343 = (((((_e1153 * (1f - _e1250)) + (((_e1153 * 1.5f) + 0.1f) * _e1254)) * (1f - _e1306)) + (0.58f * _e1306)) * (1f - _e1341));
                let _e1354 = row.member[_e131].caret[0u];
                let _e1357 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e1116 - _e1354), _e1259), vec2<f32>(0.9f, 12f), 0.9f);
                let _e1359 = ((_e1357 - 0.55f) * -0.9090909f);
                let _e1361 = select(_e1359, 0f, (_e1359 < 0f));
                let _e1363 = select(_e1361, 1f, (_e1361 > 1f));
                let _e1372 = row.member[_e131].caret[1u];
                let _e1373 = (((_e1363 * _e1363) * (3f - (2f * _e1363))) * _e1372);
                let _e1374 = (1f - _e1373);
                let _e1378 = (0.94f * _e1373);
                let _e1386 = row.member[_e131].lines[0u];
                let _e1388 = (1f / _e1386.size);
                let _e1395 = ((_e1116 - _e1386.origin.x) * _e1388);
                phi_78_ = 0u;
                phi_79_ = _e1386.count;
                loop {
                    let _e1400 = phi_78_;
                    let _e1402 = phi_79_;
                    local_58 = _e1400;
                    let _e1403 = (_e1400 < _e1402);
                    if _e1403 {
                        let _e1406 = (_e1400 + ((_e1402 - _e1400) / 2u));
                        let _e1411 = placed_glyphs.member[(_e1386.first + _e1406)].x;
                        let _e1412 = (_e1411 <= _e1395);
                        if _e1412 {
                            phi_80_ = (_e1406 + 1u);
                        } else {
                            phi_80_ = _e1400;
                        }
                        let _e1415 = phi_80_;
                        phi_81_ = _e1415;
                        phi_82_ = select(_e1406, _e1402, _e1412);
                    } else {
                        phi_81_ = u32();
                        phi_82_ = u32();
                    }
                    let _e1418 = phi_81_;
                    let _e1420 = phi_82_;
                    continue;
                    continuing {
                        phi_78_ = _e1418;
                        phi_79_ = _e1420;
                        break if !(_e1403);
                    }
                }
                let _e1422 = (3.5f / _e1386.size);
                let _e1424 = local_58;
                let _e1425 = (_e1424 + 1u);
                phi_83_ = select(_e1425, _e1386.count, (_e1386.count < _e1425));
                phi_84_ = -1000000f;
                loop {
                    let _e1429 = phi_83_;
                    let _e1431 = phi_84_;
                    local_61 = _e1431;
                    if (_e1429 > 0u) {
                        let _e1433 = (_e1429 - 1u);
                        let _e1434 = (_e1386.first + _e1433);
                        let _e1438 = placed_glyphs.member[_e1434].x;
                        let _e1442 = placed_glyphs.member[_e1434].glyph;
                        let _e1447 = glyphs.member[_e1442].min[0u];
                        let _e1452 = glyphs.member[_e1442].min[1u];
                        let _e1457 = glyphs.member[_e1442].max[0u];
                        let _e1462 = glyphs.member[_e1442].max[1u];
                        let _e1466 = glyphs.member[_e1442].start;
                        let _e1470 = glyphs.member[_e1442].count;
                        let _e1471 = (_e1395 - _e1438);
                        let _e1472 = -(((_e1117 - _e1386.origin.y) * _e1388));
                        let _e1473 = (_e1457 + _e1422);
                        let _e1474 = (_e1471 > _e1473);
                        if _e1474 {
                            phi_97_ = f32();
                        } else {
                            if (_e1471 >= (_e1447 - _e1422)) {
                                if (_e1472 >= (_e1452 - _e1422)) {
                                    if (_e1471 <= _e1473) {
                                        if (_e1472 <= (_e1462 + _e1422)) {
                                            phi_85_ = 340282350000000000000000000000000000000f;
                                            phi_86_ = 0u;
                                            phi_87_ = 0i;
                                            loop {
                                                let _e1484 = phi_85_;
                                                let _e1486 = phi_86_;
                                                let _e1488 = phi_87_;
                                                local_59 = _e1484;
                                                local_60 = _e1488;
                                                let _e1489 = (_e1486 < _e1470);
                                                if _e1489 {
                                                    let _e1493 = edges.member[(_e1466 + _e1486)];
                                                    let _e1495 = cantus_render_text_edge_distance(_e1493, _e1386.weight, vec2<f32>(_e1471, _e1472), _e1484);
                                                    phi_88_ = _e1495.member;
                                                    phi_89_ = (_e1486 + 1u);
                                                    phi_90_ = (_e1488 + _e1495.member_1);
                                                } else {
                                                    phi_88_ = f32();
                                                    phi_89_ = u32();
                                                    phi_90_ = i32();
                                                }
                                                let _e1501 = phi_88_;
                                                let _e1503 = phi_89_;
                                                let _e1505 = phi_90_;
                                                continue;
                                                continuing {
                                                    phi_85_ = _e1501;
                                                    phi_86_ = _e1503;
                                                    phi_87_ = _e1505;
                                                    break if !(_e1489);
                                                }
                                            }
                                            let _e1508 = local_59;
                                            let _e1510 = ((_e1508 * _e1386.size) * _e1386.size);
                                            if (_e1510 >= 12.25f) {
                                                phi_91_ = 3.5f;
                                            } else {
                                                phi_91_ = sqrt(_e1510);
                                            }
                                            let _e1514 = phi_91_;
                                            let _e1516 = local_60;
                                            let _e1519 = (_e1514 * select(1f, -1f, (_e1516 == 0i)));
                                            if (_e1431 != _e1431) {
                                                phi_92_ = true;
                                            } else {
                                                phi_92_ = (_e1519 >= _e1431);
                                            }
                                            let _e1523 = phi_92_;
                                            phi_93_ = select(_e1431, _e1519, _e1523);
                                        } else {
                                            phi_93_ = _e1431;
                                        }
                                        let _e1526 = phi_93_;
                                        phi_94_ = _e1526;
                                    } else {
                                        phi_94_ = _e1431;
                                    }
                                    let _e1528 = phi_94_;
                                    phi_95_ = _e1528;
                                } else {
                                    phi_95_ = _e1431;
                                }
                                let _e1530 = phi_95_;
                                phi_96_ = _e1530;
                            } else {
                                phi_96_ = _e1431;
                            }
                            let _e1532 = phi_96_;
                            phi_97_ = _e1532;
                        }
                        let _e1534 = phi_97_;
                        phi_98_ = _e1433;
                        phi_99_ = _e1534;
                        phi_100_ = select(true, false, _e1474);
                    } else {
                        phi_98_ = u32();
                        phi_99_ = f32();
                        phi_100_ = false;
                    }
                    let _e1537 = phi_98_;
                    let _e1539 = phi_99_;
                    let _e1541 = phi_100_;
                    continue;
                    continuing {
                        phi_83_ = _e1537;
                        phi_84_ = _e1539;
                        break if !(_e1541);
                    }
                }
                let _e1544 = local_61;
                let _e1546 = ((_e1544 * 1.25f) + 0.5f);
                let _e1548 = select(_e1546, 0f, (_e1546 < 0f));
                let _e1550 = select(_e1548, 1f, (_e1548 > 1f));
                let _e1554 = ((_e1550 * _e1550) * (3f - (2f * _e1550)));
                let _e1555 = (_e1133 * 0.82f);
                let _e1557 = unpack4x8unorm(_e1386.color);
                let _e1561 = (1f - _e1554);
                phi_101_ = vec4<f32>(((((((_e1343 + (0.24f * _e1341)) * _e1374) + _e1378) * _e1561) + (_e1557.x * _e1554)) * _e1555), ((((((_e1343 + (0.28f * _e1341)) * _e1374) + _e1378) * _e1561) + (_e1557.y * _e1554)) * _e1555), ((((((_e1343 + (0.52f * _e1341)) * _e1374) + _e1378) * _e1561) + (_e1557.z * _e1554)) * _e1555), _e1555);
            } else {
                let _e143 = frame.member[0u].screen_size[0u];
                let _e147 = frame.member[0u].panel_height;
                let _e150 = (((_e143 - 520f) * 0.5f) + 12f);
                let _e154 = row.member[_e131].y;
                let _e155 = (_e130.x - _e150);
                let _e156 = (_e130.y - _e154);
                let _e157 = (_e147 * 0.5f);
                let _e159 = (_e156 - _e157);
                let _e161 = ((496f - _e147) * 0.5f);
                let _e163 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e155 - 248f), _e159), _e161, _e157);
                let _e167 = frame.member[0u].mouse_pressure;
                let _e168 = (_e167 > 0f);
                if _e168 {
                    let _e173 = frame.member[0u].mouse_pos[0u];
                    let _e178 = frame.member[0u].mouse_pos[1u];
                    let _e184 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e173 - _e150) - 248f), ((_e178 - _e154) - _e157)), _e161, _e157);
                    phi_0_ = _e184;
                } else {
                    phi_0_ = 1f;
                }
                let _e186 = phi_0_;
                phi_1_ = vec2<f32>(0f, 0f);
                phi_2_ = 0u;
                loop {
                    let _e188 = phi_1_;
                    let _e190 = phi_2_;
                    local_45 = _e188;
                    local_46 = _e188;
                    local_47 = _e188;
                    local_48 = _e188;
                    let _e191 = (_e190 < 4u);
                    if _e191 {
                        if _e191 {
                        } else {
                            phi_8_ = true;
                            break;
                        }
                        let _e198 = frame.member[0u].ripples[_e190].origin[0u];
                        let _e205 = frame.member[0u].ripples[_e190].origin[1u];
                        let _e211 = frame.member[0u].ripples[_e190].start_time;
                        let _e217 = frame.member[0u].ripples[_e190].strength;
                        let _e221 = frame.member[0u].time;
                        let _e223 = ((_e221 - _e211) * 1.2f);
                        let _e225 = select(_e223, 0f, (_e223 < 0f));
                        let _e227 = select(_e225, 1f, (_e225 > 1f));
                        if (_e217 > 0f) {
                            if (_e227 < 1f) {
                                let _e230 = (_e130.x - _e198);
                                let _e231 = (_e130.y - _e205);
                                let _e235 = sqrt(((_e230 * _e230) + (_e231 * _e231)));
                                if (_e235 > 0.001f) {
                                    phi_3_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e230 / _e235), (_e231 / _e235)), _e235);
                                } else {
                                    phi_3_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e235);
                                }
                                let _e243 = phi_3_;
                                let _e253 = ((abs((_e243.unnamed_1 - (_e227 * 600f))) - 80f) * -0.0125f);
                                let _e255 = select(_e253, 0f, (_e253 < 0f));
                                let _e257 = select(_e255, 1f, (_e255 > 1f));
                                let _e263 = (1f - _e227);
                                let _e264 = ((((_e257 * _e257) * (3f - (2f * _e257))) * _e217) * _e263);
                                phi_4_ = vec2<f32>((_e188.x + (((_e243.unnamed.x * _e264) * _e263) * 0.5f)), (_e188.y + (((_e243.unnamed.y * _e264) * _e263) * 0.5f)));
                            } else {
                                phi_4_ = _e188;
                            }
                            let _e277 = phi_4_;
                            phi_5_ = _e277;
                        } else {
                            phi_5_ = _e188;
                        }
                        let _e279 = phi_5_;
                        phi_6_ = _e279;
                        phi_7_ = (_e190 + 1u);
                    } else {
                        phi_6_ = vec2<f32>();
                        phi_7_ = u32();
                    }
                    let _e282 = phi_6_;
                    let _e284 = phi_7_;
                    continue;
                    continuing {
                        phi_1_ = _e282;
                        phi_2_ = _e284;
                        phi_8_ = false;
                        break if !(_e191);
                    }
                }
                let _e287 = phi_8_;
                if _e287 {
                    break;
                }
                if _e168 {
                    let _e292 = frame.member[0u].mouse_pos[0u];
                    let _e297 = frame.member[0u].mouse_pos[1u];
                    let _e298 = (_e130.x - _e292);
                    let _e299 = (_e130.y - _e297);
                    let _e305 = ((sqrt(((_e298 * _e298) + (_e299 * _e299))) - 150f) * -0.006666667f);
                    let _e307 = select(_e305, 0f, (_e305 < 0f));
                    let _e309 = select(_e307, 1f, (_e307 > 1f));
                    phi_9_ = ((((_e309 * _e309) * (3f - (2f * _e309))) * _e167) * 8f);
                } else {
                    phi_9_ = 0f;
                }
                let _e317 = phi_9_;
                let _e319 = local_45;
                let _e322 = global[0u];
                if (_e319.x == _e322) {
                    let _e325 = local_46;
                    let _e328 = global[1u];
                    phi_10_ = (_e325.y == _e328);
                } else {
                    phi_10_ = false;
                }
                let _e331 = phi_10_;
                if _e331 {
                    phi_11_ = 0f;
                } else {
                    let _e333 = local_47;
                    phi_11_ = (sqrt(((_e319.x * _e319.x) + (_e333.y * _e333.y))) * 22f);
                }
                let _e341 = phi_11_;
                let _e343 = local_48;
                let _e346 = ((_e186 - 0.5f) * -1f);
                let _e348 = select(_e346, 0f, (_e346 < 0f));
                let _e350 = select(_e348, 1f, (_e348 > 1f));
                let _e356 = ((_e317 * ((_e350 * _e350) * (3f - (2f * _e350)))) + _e341);
                let _e358 = (_e163 - (_e356 * 0.5f));
                let _e359 = fwidth(_e358);
                if (_e359 != _e359) {
                    phi_12_ = true;
                } else {
                    phi_12_ = (0.55f >= _e359);
                }
                let _e363 = phi_12_;
                let _e364 = select(_e359, 0.55f, _e363);
                let _e368 = ((_e358 - _e364) / (-(_e364) - _e364));
                let _e370 = select(_e368, 0f, (_e368 < 0f));
                let _e372 = select(_e370, 1f, (_e370 > 1f));
                let _e376 = ((_e372 * _e372) * (3f - (2f * _e372)));
                let _e377 = (_e358 != _e358);
                if _e377 {
                    phi_13_ = true;
                } else {
                    phi_13_ = (0f >= _e358);
                }
                let _e380 = phi_13_;
                let _e384 = (exp((select(_e358, 0f, _e380) * -0.3f)) * 0.16f);
                if (_e376 != _e376) {
                    phi_14_ = true;
                } else {
                    phi_14_ = (_e384 >= _e376);
                }
                let _e388 = phi_14_;
                let _e389 = select(_e376, _e384, _e388);
                if (_e389 <= 0.0009765625f) {
                    discard;
                }
                let _e391 = (_e155 * 0.002016129f);
                let _e392 = (_e156 / _e147);
                if _e377 {
                    phi_15_ = true;
                } else {
                    phi_15_ = (0f <= _e358);
                }
                let _e397 = phi_15_;
                let _e400 = (1f + (select(_e358, 0f, _e397) * 0.008333334f));
                let _e402 = select(_e400, 0f, (_e400 < 0f));
                let _e404 = select(_e402, 0.6f, (_e402 > 0.6f));
                let _e415 = (((_e391 - (((_e391 - 0.5f) * _e404) * 0.08f)) - (_e319.x * 0.04f)) * 496f);
                let _e416 = (((_e392 - (((_e392 - 0.5f) * _e404) * 0.08f)) - (_e343.y * 0.04f)) * _e147);
                let _e422 = row.member[_e131].badges[0u][1u];
                let _e424 = select(0f, 1f, (_e422 > 0f));
                let _e429 = (_e356 * 0.125f);
                if (_e429 != _e429) {
                    phi_16_ = true;
                } else {
                    phi_16_ = (1f <= _e429);
                }
                let _e433 = phi_16_;
                let _e434 = select(_e429, 1f, _e433);
                let _e438 = ((((0.15f * (1f - _e424)) + (0.235f * _e424)) * (1f - _e434)) + (0.3f * _e434));
                let _e439 = vec3(_e438);
                let _e440 = (_e155 - _e157);
                if (_e137 == -2i) {
                    let _e478 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e440, _e159), vec2<f32>(13f, 13f), 9f);
                    let _e480 = ((_e478 - 0.55f) * -0.9090909f);
                    let _e482 = select(_e480, 0f, (_e480 < 0f));
                    let _e484 = select(_e482, 1f, (_e482 > 1f));
                    let _e488 = ((_e484 * _e484) * (3f - (2f * _e484)));
                    let _e491 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e440, (_e159 - -3.1f)), vec2<f32>(5.4f, 1.1f), 1.1f);
                    let _e493 = ((_e491 - 0.55f) * -0.9090909f);
                    let _e495 = select(_e493, 0f, (_e493 < 0f));
                    let _e497 = select(_e495, 1f, (_e495 > 1f));
                    let _e501 = ((_e497 * _e497) * (3f - (2f * _e497)));
                    let _e504 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e440, (_e159 - 3.1f)), vec2<f32>(5.4f, 1.1f), 1.1f);
                    let _e506 = ((_e504 - 0.55f) * -0.9090909f);
                    let _e508 = select(_e506, 0f, (_e506 < 0f));
                    let _e510 = select(_e508, 1f, (_e508 > 1f));
                    let _e514 = ((_e510 * _e510) * (3f - (2f * _e510)));
                    if (_e501 != _e501) {
                        phi_19_ = true;
                    } else {
                        phi_19_ = (_e514 >= _e501);
                    }
                    let _e518 = phi_19_;
                    let _e519 = select(_e501, _e514, _e518);
                    let _e520 = (1f - _e519);
                    let _e524 = (0.96f * _e519);
                    let _e528 = (_e519 * _e488);
                    if (_e488 != _e488) {
                        phi_20_ = true;
                    } else {
                        phi_20_ = (_e528 >= _e488);
                    }
                    let _e532 = phi_20_;
                    let _e533 = select(_e488, _e528, _e532);
                    let _e535 = (_e438 * (1f - _e533));
                    phi_21_ = vec3<f32>((_e535 + (((0.44f * _e520) + _e524) * _e533)), (_e535 + (((0.4f * _e520) + _e524) * _e533)), (_e535 + (((0.8f * _e520) + _e524) * _e533)));
                } else {
                    if (_e137 >= 0i) {
                        let _e443 = abs(_e440);
                        let _e444 = abs(_e159);
                        if (select(_e444, _e443, (_e443 > _e444)) < 16f) {
                            let _e453 = vec3<f32>(((_e440 * 0.03125f) + 0.5f), ((_e159 * 0.03125f) + 0.5f), f32(_e137));
                            let _e459 = textureSample(icons, sampler_, vec2<f32>(_e453.x, _e453.y), i32(_e453.z));
                            let _e465 = (_e438 * (1f - _e459.w));
                            phi_17_ = vec3<f32>((_e465 + (_e459.x * _e459.w)), (_e465 + (_e459.y * _e459.w)), (_e465 + (_e459.z * _e459.w)));
                        } else {
                            phi_17_ = _e439;
                        }
                        let _e474 = phi_17_;
                        phi_18_ = _e474;
                    } else {
                        phi_18_ = _e439;
                    }
                    let _e476 = phi_18_;
                    phi_21_ = _e476;
                }
                let _e544 = phi_21_;
                phi_22_ = 0u;
                phi_23_ = _e544;
                loop {
                    let _e546 = phi_22_;
                    let _e548 = phi_23_;
                    local_62 = _e548;
                    let _e549 = (_e546 < 2u);
                    if _e549 {
                        if _e549 {
                        } else {
                            phi_39_ = true;
                            break;
                        }
                        let _e555 = row.member[_e131].badges[_e546][0u];
                        let _e561 = row.member[_e131].badges[_e546][1u];
                        let _e562 = (_e415 - _e555);
                        let _e563 = (_e416 - _e157);
                        if (_e561 <= 0f) {
                            phi_36_ = vec4<f32>(0f, 0f, 0f, 0f);
                        } else {
                            let _e568 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e562, _e563), vec2<f32>(_e561, 10.5f), 6f);
                            let _e570 = ((_e568 - 0.55f) * -0.9090909f);
                            let _e572 = select(_e570, 0f, (_e570 < 0f));
                            let _e574 = select(_e572, 1f, (_e572 > 1f));
                            let _e578 = ((_e574 * _e574) * (3f - (2f * _e574)));
                            let _e581 = ((abs(_e568) - 1.2f) * -0.9090908f);
                            let _e583 = select(_e581, 0f, (_e581 < 0f));
                            let _e585 = select(_e583, 1f, (_e583 > 1f));
                            let _e589 = ((_e585 * _e585) * (3f - (2f * _e585)));
                            if (_e546 == 1u) {
                                let _e673 = (_e562 + 8.5f);
                                let _e674 = (_e563 - -4f);
                                let _e676 = (_e674 * 4.2f);
                                let _e678 = (((_e673 * -3.4f) + _e676) * 0.03424658f);
                                let _e680 = select(_e678, 0f, (_e678 < 0f));
                                let _e682 = select(_e680, 1f, (_e680 > 1f));
                                let _e685 = (_e673 - (-3.4f * _e682));
                                let _e686 = (_e674 - (4.2f * _e682));
                                let _e690 = sqrt(((_e685 * _e685) + (_e686 * _e686)));
                                let _e693 = (((_e673 * 3.4f) + _e676) * 0.03424658f);
                                let _e695 = select(_e693, 0f, (_e693 < 0f));
                                let _e697 = select(_e695, 1f, (_e695 > 1f));
                                let _e700 = (_e673 - (3.4f * _e697));
                                let _e701 = (_e674 - (4.2f * _e697));
                                let _e705 = sqrt(((_e700 * _e700) + (_e701 * _e701)));
                                if (_e690 != _e690) {
                                    phi_27_ = true;
                                } else {
                                    phi_27_ = (_e705 <= _e690);
                                }
                                let _e709 = phi_27_;
                                let _e710 = select(_e690, _e705, _e709);
                                let _e711 = (_e563 - -0.6f);
                                let _e712 = (_e711 * 0.21739131f);
                                let _e714 = select(_e712, 0f, (_e712 < 0f));
                                let _e718 = (_e711 - (4.6f * select(_e714, 1f, (_e714 > 1f))));
                                let _e722 = sqrt(((_e673 * _e673) + (_e718 * _e718)));
                                if (_e710 != _e710) {
                                    phi_28_ = true;
                                } else {
                                    phi_28_ = (_e722 <= _e710);
                                }
                                let _e726 = phi_28_;
                                let _e730 = ((abs(select(_e710, _e722, _e726)) - 1.35f) * -0.9090909f);
                                let _e732 = select(_e730, 0f, (_e730 < 0f));
                                let _e734 = select(_e732, 1f, (_e732 > 1f));
                                let _e738 = ((_e734 * _e734) * (3f - (2f * _e734)));
                                let _e739 = (_e562 - 10.9f);
                                let _e740 = (_e563 - -3.6f);
                                let _e741 = (_e740 * 0.18518521f);
                                let _e743 = select(_e741, 0f, (_e741 < 0f));
                                let _e747 = (_e740 - (5.3999996f * select(_e743, 1f, (_e743 > 1f))));
                                let _e751 = sqrt(((_e739 * _e739) + (_e747 * _e747)));
                                let _e752 = (_e563 - 1.8f);
                                let _e753 = (_e739 * -0.16666667f);
                                let _e755 = select(_e753, 0f, (_e753 < 0f));
                                let _e759 = (_e739 - (-6f * select(_e755, 1f, (_e755 > 1f))));
                                let _e763 = sqrt(((_e759 * _e759) + (_e752 * _e752)));
                                if (_e751 != _e751) {
                                    phi_29_ = true;
                                } else {
                                    phi_29_ = (_e763 <= _e751);
                                }
                                let _e767 = phi_29_;
                                let _e768 = select(_e751, _e763, _e767);
                                let _e769 = (_e562 - 4.9f);
                                let _e770 = (_e769 * 2.8f);
                                let _e773 = ((_e770 + (_e752 * -2.6f)) * 0.06849316f);
                                let _e775 = select(_e773, 0f, (_e773 < 0f));
                                let _e777 = select(_e775, 1f, (_e775 > 1f));
                                let _e780 = (_e769 - (2.8f * _e777));
                                let _e781 = (_e752 - (-2.6f * _e777));
                                let _e785 = sqrt(((_e780 * _e780) + (_e781 * _e781)));
                                if (_e768 != _e768) {
                                    phi_30_ = true;
                                } else {
                                    phi_30_ = (_e785 <= _e768);
                                }
                                let _e789 = phi_30_;
                                let _e790 = select(_e768, _e785, _e789);
                                let _e793 = ((_e770 + (_e752 * 2.6000001f)) * 0.06849315f);
                                let _e795 = select(_e793, 0f, (_e793 < 0f));
                                let _e797 = select(_e795, 1f, (_e795 > 1f));
                                let _e800 = (_e769 - (2.8f * _e797));
                                let _e801 = (_e752 - (2.6000001f * _e797));
                                let _e805 = sqrt(((_e800 * _e800) + (_e801 * _e801)));
                                if (_e790 != _e790) {
                                    phi_31_ = true;
                                } else {
                                    phi_31_ = (_e805 <= _e790);
                                }
                                let _e809 = phi_31_;
                                let _e813 = ((abs(select(_e790, _e805, _e809)) - 1.35f) * -0.9090909f);
                                let _e815 = select(_e813, 0f, (_e813 < 0f));
                                let _e817 = select(_e815, 1f, (_e815 > 1f));
                                let _e821 = ((_e817 * _e817) * (3f - (2f * _e817)));
                                if (_e738 != _e738) {
                                    phi_32_ = true;
                                } else {
                                    phi_32_ = (_e821 >= _e738);
                                }
                                let _e825 = phi_32_;
                                phi_33_ = select(_e738, _e821, _e825);
                            } else {
                                let _e590 = (_e562 - 3.4f);
                                let _e591 = (_e563 - -3.6f);
                                let _e592 = (_e591 * 0.18518521f);
                                let _e594 = select(_e592, 0f, (_e592 < 0f));
                                let _e598 = (_e591 - (5.3999996f * select(_e594, 1f, (_e594 > 1f))));
                                let _e602 = sqrt(((_e590 * _e590) + (_e598 * _e598)));
                                let _e603 = (_e563 - 1.8f);
                                let _e604 = (_e590 * -0.16666667f);
                                let _e606 = select(_e604, 0f, (_e604 < 0f));
                                let _e610 = (_e590 - (-6f * select(_e606, 1f, (_e606 > 1f))));
                                let _e614 = sqrt(((_e610 * _e610) + (_e603 * _e603)));
                                if (_e602 != _e602) {
                                    phi_24_ = true;
                                } else {
                                    phi_24_ = (_e614 <= _e602);
                                }
                                let _e618 = phi_24_;
                                let _e619 = select(_e602, _e614, _e618);
                                let _e620 = (_e562 - -2.6f);
                                let _e621 = (_e620 * 2.8f);
                                let _e624 = ((_e621 + (_e603 * -2.6f)) * 0.06849316f);
                                let _e626 = select(_e624, 0f, (_e624 < 0f));
                                let _e628 = select(_e626, 1f, (_e626 > 1f));
                                let _e631 = (_e620 - (2.8f * _e628));
                                let _e632 = (_e603 - (-2.6f * _e628));
                                let _e636 = sqrt(((_e631 * _e631) + (_e632 * _e632)));
                                if (_e619 != _e619) {
                                    phi_25_ = true;
                                } else {
                                    phi_25_ = (_e636 <= _e619);
                                }
                                let _e640 = phi_25_;
                                let _e641 = select(_e619, _e636, _e640);
                                let _e644 = ((_e621 + (_e603 * 2.6000001f)) * 0.06849315f);
                                let _e646 = select(_e644, 0f, (_e644 < 0f));
                                let _e648 = select(_e646, 1f, (_e646 > 1f));
                                let _e651 = (_e620 - (2.8f * _e648));
                                let _e652 = (_e603 - (2.6000001f * _e648));
                                let _e656 = sqrt(((_e651 * _e651) + (_e652 * _e652)));
                                if (_e641 != _e641) {
                                    phi_26_ = true;
                                } else {
                                    phi_26_ = (_e656 <= _e641);
                                }
                                let _e660 = phi_26_;
                                let _e664 = ((abs(select(_e641, _e656, _e660)) - 1.35f) * -0.9090909f);
                                let _e666 = select(_e664, 0f, (_e664 < 0f));
                                let _e668 = select(_e666, 1f, (_e666 > 1f));
                                phi_33_ = ((_e668 * _e668) * (3f - (2f * _e668)));
                            }
                            let _e828 = phi_33_;
                            let _e836 = ((((0.27f * (1f - _e589)) + (0.58f * _e589)) * (1f - _e828)) + (0.94f * _e828));
                            if (_e578 != _e578) {
                                phi_34_ = true;
                            } else {
                                phi_34_ = (_e589 >= _e578);
                            }
                            let _e840 = phi_34_;
                            let _e841 = select(_e578, _e589, _e840);
                            if (_e841 != _e841) {
                                phi_35_ = true;
                            } else {
                                phi_35_ = (_e828 >= _e841);
                            }
                            let _e845 = phi_35_;
                            phi_36_ = vec4<f32>(_e836, _e836, _e836, select(_e841, _e828, _e845));
                        }
                        let _e849 = phi_36_;
                        let _e854 = (1f - _e849.w);
                        phi_37_ = (_e546 + 1u);
                        phi_38_ = vec3<f32>(((_e548.x * _e854) + (_e849.x * _e849.w)), ((_e548.y * _e854) + (_e849.y * _e849.w)), ((_e548.z * _e854) + (_e849.z * _e849.w)));
                    } else {
                        phi_37_ = u32();
                        phi_38_ = vec3<f32>();
                    }
                    let _e870 = phi_37_;
                    let _e872 = phi_38_;
                    continue;
                    continuing {
                        phi_22_ = _e870;
                        phi_23_ = _e872;
                        phi_39_ = _e287;
                        break if !(_e549);
                    }
                }
                let _e875 = phi_39_;
                if _e875 {
                    break;
                }
                phi_40_ = 0u;
                let _e1624 = local_62;
                phi_41_ = _e1624;
                loop {
                    let _e877 = phi_40_;
                    let _e879 = phi_41_;
                    local_53 = _e879;
                    local_54 = _e879;
                    local_55 = _e879;
                    let _e880 = (_e877 < 4u);
                    if _e880 {
                        if _e880 {
                        } else {
                            phi_67_ = true;
                            break;
                        }
                        let _e885 = row.member[_e131].lines[_e877];
                        let _e887 = (1f / _e885.size);
                        let _e894 = ((_e415 - _e885.origin.x) * _e887);
                        phi_42_ = 0u;
                        phi_43_ = _e885.count;
                        loop {
                            let _e899 = phi_42_;
                            let _e901 = phi_43_;
                            local_49 = _e899;
                            let _e902 = (_e899 < _e901);
                            if _e902 {
                                let _e905 = (_e899 + ((_e901 - _e899) / 2u));
                                let _e910 = placed_glyphs.member[(_e885.first + _e905)].x;
                                let _e911 = (_e910 <= _e894);
                                if _e911 {
                                    phi_44_ = (_e905 + 1u);
                                } else {
                                    phi_44_ = _e899;
                                }
                                let _e914 = phi_44_;
                                phi_45_ = _e914;
                                phi_46_ = select(_e905, _e901, _e911);
                            } else {
                                phi_45_ = u32();
                                phi_46_ = u32();
                            }
                            let _e917 = phi_45_;
                            let _e919 = phi_46_;
                            continue;
                            continuing {
                                phi_42_ = _e917;
                                phi_43_ = _e919;
                                break if !(_e902);
                            }
                        }
                        let _e921 = (3.5f / _e885.size);
                        let _e923 = local_49;
                        let _e924 = (_e923 + 1u);
                        phi_47_ = select(_e924, _e885.count, (_e885.count < _e924));
                        phi_48_ = -1000000f;
                        loop {
                            let _e928 = phi_47_;
                            let _e930 = phi_48_;
                            local_52 = _e930;
                            if (_e928 > 0u) {
                                let _e932 = (_e928 - 1u);
                                let _e933 = (_e885.first + _e932);
                                let _e937 = placed_glyphs.member[_e933].x;
                                let _e941 = placed_glyphs.member[_e933].glyph;
                                let _e946 = glyphs.member[_e941].min[0u];
                                let _e951 = glyphs.member[_e941].min[1u];
                                let _e956 = glyphs.member[_e941].max[0u];
                                let _e961 = glyphs.member[_e941].max[1u];
                                let _e965 = glyphs.member[_e941].start;
                                let _e969 = glyphs.member[_e941].count;
                                let _e970 = (_e894 - _e937);
                                let _e971 = -(((_e416 - _e885.origin.y) * _e887));
                                let _e972 = (_e956 + _e921);
                                let _e973 = (_e970 > _e972);
                                if _e973 {
                                    phi_61_ = f32();
                                } else {
                                    if (_e970 >= (_e946 - _e921)) {
                                        if (_e971 >= (_e951 - _e921)) {
                                            if (_e970 <= _e972) {
                                                if (_e971 <= (_e961 + _e921)) {
                                                    phi_49_ = 340282350000000000000000000000000000000f;
                                                    phi_50_ = 0u;
                                                    phi_51_ = 0i;
                                                    loop {
                                                        let _e983 = phi_49_;
                                                        let _e985 = phi_50_;
                                                        let _e987 = phi_51_;
                                                        local_50 = _e983;
                                                        local_51 = _e987;
                                                        let _e988 = (_e985 < _e969);
                                                        if _e988 {
                                                            let _e992 = edges.member[(_e965 + _e985)];
                                                            let _e994 = cantus_render_text_edge_distance(_e992, _e885.weight, vec2<f32>(_e970, _e971), _e983);
                                                            phi_52_ = _e994.member;
                                                            phi_53_ = (_e985 + 1u);
                                                            phi_54_ = (_e987 + _e994.member_1);
                                                        } else {
                                                            phi_52_ = f32();
                                                            phi_53_ = u32();
                                                            phi_54_ = i32();
                                                        }
                                                        let _e1000 = phi_52_;
                                                        let _e1002 = phi_53_;
                                                        let _e1004 = phi_54_;
                                                        continue;
                                                        continuing {
                                                            phi_49_ = _e1000;
                                                            phi_50_ = _e1002;
                                                            phi_51_ = _e1004;
                                                            break if !(_e988);
                                                        }
                                                    }
                                                    let _e1007 = local_50;
                                                    let _e1009 = ((_e1007 * _e885.size) * _e885.size);
                                                    if (_e1009 >= 12.25f) {
                                                        phi_55_ = 3.5f;
                                                    } else {
                                                        phi_55_ = sqrt(_e1009);
                                                    }
                                                    let _e1013 = phi_55_;
                                                    let _e1015 = local_51;
                                                    let _e1018 = (_e1013 * select(1f, -1f, (_e1015 == 0i)));
                                                    if (_e930 != _e930) {
                                                        phi_56_ = true;
                                                    } else {
                                                        phi_56_ = (_e1018 >= _e930);
                                                    }
                                                    let _e1022 = phi_56_;
                                                    phi_57_ = select(_e930, _e1018, _e1022);
                                                } else {
                                                    phi_57_ = _e930;
                                                }
                                                let _e1025 = phi_57_;
                                                phi_58_ = _e1025;
                                            } else {
                                                phi_58_ = _e930;
                                            }
                                            let _e1027 = phi_58_;
                                            phi_59_ = _e1027;
                                        } else {
                                            phi_59_ = _e930;
                                        }
                                        let _e1029 = phi_59_;
                                        phi_60_ = _e1029;
                                    } else {
                                        phi_60_ = _e930;
                                    }
                                    let _e1031 = phi_60_;
                                    phi_61_ = _e1031;
                                }
                                let _e1033 = phi_61_;
                                phi_62_ = _e932;
                                phi_63_ = _e1033;
                                phi_64_ = select(true, false, _e973);
                            } else {
                                phi_62_ = u32();
                                phi_63_ = f32();
                                phi_64_ = false;
                            }
                            let _e1036 = phi_62_;
                            let _e1038 = phi_63_;
                            let _e1040 = phi_64_;
                            continue;
                            continuing {
                                phi_47_ = _e1036;
                                phi_48_ = _e1038;
                                break if !(_e1040);
                            }
                        }
                        let _e1043 = local_52;
                        let _e1045 = ((_e1043 * 1.25f) + 0.5f);
                        let _e1047 = select(_e1045, 0f, (_e1045 < 0f));
                        let _e1049 = select(_e1047, 1f, (_e1047 > 1f));
                        let _e1053 = ((_e1049 * _e1049) * (3f - (2f * _e1049)));
                        let _e1055 = unpack4x8unorm(_e885.color);
                        let _e1059 = (1f - _e1053);
                        phi_65_ = (_e877 + 1u);
                        phi_66_ = vec3<f32>(((_e879.x * _e1059) + (_e1055.x * _e1053)), ((_e879.y * _e1059) + (_e1055.y * _e1053)), ((_e879.z * _e1059) + (_e1055.z * _e1053)));
                    } else {
                        phi_65_ = u32();
                        phi_66_ = vec3<f32>();
                    }
                    let _e1075 = phi_65_;
                    let _e1077 = phi_66_;
                    continue;
                    continuing {
                        phi_40_ = _e1075;
                        phi_41_ = _e1077;
                        phi_67_ = _e875;
                        break if !(_e880);
                    }
                }
                let _e1080 = phi_67_;
                if _e1080 {
                    break;
                }
                let _e1082 = local_53;
                let _e1086 = local_54;
                let _e1090 = local_55;
                phi_101_ = vec4<f32>((_e1082.x * _e376), (_e1086.y * _e376), (_e1090.z * _e376), _e389);
            }
            let _e1576 = phi_101_;
            out_color = _e1576;
            break;
        }
    }
    return;
}

fn render_playhead_isthmus_playheadpass_vertex_impl() {
    let _e14 = vertex_7;
    let _e15 = _isthmus_instance_index_9;
    let _e24 = frame.member[0u].playhead_x;
    let _e30 = frame.member[0u].panel_height;
    let _e33 = (_e24 + ((((f32((_e14 & 1u)) * 2f) - 1f) * _e30) * 0.4f));
    let _e36 = (1f + (f32((_e14 >> bitcast<u32>(1i))) * (_e30 + 10f)));
    let _e41 = frame.member[0u].screen_size[0u];
    let _e46 = frame.member[0u].screen_size[1u];
    out_position = vec4<f32>((((_e33 / _e41) * 2f) - 1f), (1f - ((_e36 / _e46) * 2f)), 0f, 1f);
    out_world_pos[0u] = _e33;
    out_world_pos[1u] = _e36;
    out_isthmus_instance_index = _e15;
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

    let _e30 = vertex_7;
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
            let _e159 = (_e45 * 6.6666665f);
            let _e161 = select(_e159, 0f, (_e159 < 0f));
            let _e163 = select(_e161, 1f, (_e161 > 1f));
            phi_1_ = isthmus_Vertex_render_particles_Varyings(isthmus_Vertex_render_text_Varyings(vec4<f32>(((((_e125 + (_e116.x * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.y * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.z * 2f)) * 0.8f) + 0.2f) * 2f), (((1f - _e48) * ((_e163 * _e163) * (3f - (2f * _e163)))) * 0.3f)), vec2<f32>(_e82, _e83)), vec4<f32>(((((((_e93 + (_e53 * _e45)) + (_e70.unnamed.x * _e87)) + (-(_e70.unnamed.y) * _e88)) / _e145) * 2f) - 1f), (1f - (((((_e98 + (_e58 * _e45)) + (_e70.unnamed.y * _e87)) + (_e70.unnamed.x * _e88)) / _e150) * 2f)), 0f, 1f));
        }
        let _e175 = phi_1_;
        phi_2_ = _e175;
        phi_3_ = _e47;
    }
    let _e177 = phi_2_;
    let _e179 = phi_3_;
    if _e179 {
        phi_4_ = isthmus_Vertex_render_particles_Varyings(isthmus_Vertex_render_text_Varyings(vec4<f32>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f)), vec4<f32>(0f, 0f, 0f, 0f));
    } else {
        phi_4_ = _e177;
    }
    let _e181 = phi_4_;
    out_position = _e181.position;
    out_color = _e181.varyings.position;
    out_uv[0u] = _e181.varyings.varyings.x;
    out_uv[1u] = _e181.varyings.varyings.y;
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
    var local_63: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e36 = vertex_7;
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
                local_63 = _e143;
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
            let _e181 = local_63;
            let _e182 = (_e181 * 0.5f);
            let _e183 = (18f + _e182);
            let _e194 = frame.member[0u].panel_height;
            let _e202 = (((_e129 - (_e49 * 158f)) - _e183) + (f32((_e36 & 1u)) * ((308f + (316f * _e49)) + (_e183 * 2f))));
            let _e203 = ((-12f - _e182) + (f32((_e36 >> bitcast<u32>(1i))) * ((244f * _e49) + ((_e194 + _e183) * 2f))));
            let _e208 = frame.member[0u].screen_size[0u];
            let _e213 = frame.member[0u].screen_size[1u];
            out_position = vec4<f32>((((_e202 / _e208) * 2f) - 1f), (1f - ((_e203 / _e213) * 2f)), 0f, 1f);
            out_pixel[0u] = _e202;
            out_pixel[1u] = _e203;
            out_weather = vec4<f32>(_e91[0], _e91[1], _e123[1], _e49);
            out_isthmus_instance_index_1 = _e37;
            break;
        }
    }
    return;
}

fn render_tempestas_isthmus_tempestaspass_fragment_impl() {
    var phi_0_: f32;
    var phi_1_: u32;
    var phi_2_: u32;
    var phi_3_: bool;
    var phi_4_: vec2<f32>;
    var phi_5_: f32;
    var phi_6_: u32;
    var phi_7_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_8_: bool;
    var phi_9_: vec2<f32>;
    var phi_10_: f32;
    var phi_11_: vec2<f32>;
    var phi_12_: f32;
    var phi_13_: vec2<f32>;
    var phi_14_: f32;
    var phi_15_: u32;
    var phi_16_: bool;
    var phi_17_: f32;
    var local_64: vec2<f32>;
    var local_65: vec2<f32>;
    var phi_18_: bool;
    var local_66: vec2<f32>;
    var phi_19_: f32;
    var local_67: vec2<f32>;
    var phi_20_: bool;
    var phi_21_: bool;
    var phi_22_: bool;
    var phi_23_: bool;
    var phi_24_: bool;
    var phi_25_: bool;
    var phi_26_: f32;
    var phi_27_: render_tempestas_WeatherCondition;
    var phi_28_: render_tempestas_WeatherCondition;
    var phi_29_: array<f32, 2>;
    var phi_30_: array<f32, 2>;
    var phi_31_: bool;
    var phi_32_: f32;
    var phi_33_: array<f32, 2>;
    var phi_34_: bool;
    var phi_35_: bool;
    var phi_36_: bool;
    var phi_37_: vec3<f32>;
    var phi_38_: vec2<f32>;
    var phi_39_: render_tempestas_WeatherCondition;
    var phi_40_: i32;
    var phi_41_: f32;
    var phi_42_: f32;
    var phi_43_: vec2<f32>;
    var phi_44_: i32;
    var phi_45_: f32;
    var phi_46_: f32;
    var phi_47_: vec2<f32>;
    var local_68: f32;
    var phi_48_: i32;
    var phi_49_: f32;
    var phi_50_: f32;
    var phi_51_: vec2<f32>;
    var phi_52_: i32;
    var phi_53_: f32;
    var phi_54_: f32;
    var phi_55_: vec2<f32>;
    var local_69: f32;
    var local_70: f32;
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
    var local_71: f32;
    var phi_68_: vec3<f32>;
    var phi_69_: i32;
    var phi_70_: f32;
    var phi_71_: f32;
    var phi_72_: vec2<f32>;
    var phi_73_: i32;
    var phi_74_: f32;
    var phi_75_: f32;
    var phi_76_: vec2<f32>;
    var local_72: f32;
    var phi_77_: i32;
    var phi_78_: f32;
    var phi_79_: f32;
    var phi_80_: vec2<f32>;
    var phi_81_: i32;
    var phi_82_: f32;
    var phi_83_: f32;
    var phi_84_: vec2<f32>;
    var local_73: f32;
    var local_74: f32;
    var phi_85_: vec3<f32>;
    var phi_86_: vec3<f32>;
    var phi_87_: vec3<f32>;
    var phi_88_: vec3<f32>;
    var phi_89_: i32;
    var phi_90_: f32;
    var phi_91_: f32;
    var phi_92_: vec2<f32>;
    var phi_93_: i32;
    var phi_94_: f32;
    var phi_95_: f32;
    var phi_96_: vec2<f32>;
    var local_75: f32;
    var phi_97_: vec3<f32>;
    var phi_98_: vec3<f32>;
    var phi_99_: vec3<f32>;
    var phi_100_: i32;
    var phi_101_: f32;
    var phi_102_: f32;
    var phi_103_: vec2<f32>;
    var phi_104_: i32;
    var phi_105_: f32;
    var phi_106_: f32;
    var phi_107_: vec2<f32>;
    var local_76: f32;
    var phi_108_: f32;
    var phi_109_: vec3<f32>;
    var local_77: f32;
    var local_78: f32;
    var local_79: f32;
    var local_80: f32;
    var phi_110_: vec3<f32>;
    var phi_111_: i32;
    var phi_112_: u32;
    var phi_113_: u32;
    var phi_114_: u32;
    var phi_115_: u32;
    var phi_116_: u32;
    var local_81: u32;
    var phi_117_: u32;
    var phi_118_: f32;
    var phi_119_: f32;
    var phi_120_: u32;
    var phi_121_: i32;
    var phi_122_: f32;
    var phi_123_: u32;
    var phi_124_: i32;
    var local_82: f32;
    var phi_125_: f32;
    var local_83: i32;
    var phi_126_: bool;
    var phi_127_: f32;
    var phi_128_: f32;
    var phi_129_: f32;
    var phi_130_: f32;
    var phi_131_: f32;
    var phi_132_: u32;
    var phi_133_: f32;
    var phi_134_: bool;
    var local_84: f32;
    var phi_135_: vec3<f32>;
    var phi_136_: i32;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e211 = pixel_4;
            let _e212 = weather_1;
            let _e213 = _isthmus_instance_index_11;
            let _e222 = pill_2.member[_e213].x;
            let _e226 = frame.member[0u].panel_height;
            let _e227 = (_e211.x - _e222);
            let _e228 = (_e211.y - 6f);
            let _e229 = (_e226 * 0.5f);
            let _e233 = ((308f - _e226) * 0.5f);
            let _e235 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e227 - 154f), (_e228 - _e229)), _e233, _e229);
            let _e239 = frame.member[0u].mouse_pressure;
            let _e240 = (_e239 > 0f);
            if _e240 {
                let _e245 = frame.member[0u].mouse_pos[0u];
                let _e250 = frame.member[0u].mouse_pos[1u];
                let _e256 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e245 - _e222) - 154f), ((_e250 - 6f) - _e229)), _e233, _e229);
                phi_0_ = _e256;
            } else {
                phi_0_ = 1f;
            }
            let _e258 = phi_0_;
            phi_1_ = 0u;
            loop {
                let _e260 = phi_1_;
                let _e261 = (_e260 < 4u);
                if _e261 {
                    if _e261 {
                    } else {
                        phi_3_ = true;
                        break;
                    }
                    phi_2_ = (_e260 + 1u);
                } else {
                    phi_2_ = u32();
                }
                let _e264 = phi_2_;
                continue;
                continuing {
                    phi_1_ = _e264;
                    phi_3_ = false;
                    break if !(_e261);
                }
            }
            let _e267 = phi_3_;
            if _e267 {
                break;
            }
            phi_4_ = vec2<f32>(0f, 0f);
            phi_5_ = 0f;
            phi_6_ = 0u;
            loop {
                let _e270 = phi_4_;
                let _e272 = phi_5_;
                let _e274 = phi_6_;
                local_64 = _e270;
                local_65 = _e270;
                local_66 = _e270;
                local_67 = _e270;
                local_77 = _e272;
                local_78 = _e272;
                local_79 = _e272;
                local_80 = _e272;
                let _e275 = (_e274 < 4u);
                if _e275 {
                    if _e275 {
                    } else {
                        phi_16_ = true;
                        break;
                    }
                    let _e282 = frame.member[0u].ripples[_e274].origin[0u];
                    let _e289 = frame.member[0u].ripples[_e274].origin[1u];
                    let _e295 = frame.member[0u].ripples[_e274].start_time;
                    let _e301 = frame.member[0u].ripples[_e274].strength;
                    let _e305 = frame.member[0u].time;
                    let _e307 = ((_e305 - _e295) * 1.2f);
                    let _e309 = select(_e307, 0f, (_e307 < 0f));
                    let _e311 = select(_e309, 1f, (_e309 > 1f));
                    if (_e301 > 0f) {
                        if (_e311 < 1f) {
                            let _e314 = (_e211.x - _e282);
                            let _e315 = (_e211.y - _e289);
                            let _e319 = sqrt(((_e314 * _e314) + (_e315 * _e315)));
                            if (_e319 > 0.001f) {
                                phi_7_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e314 / _e319), (_e315 / _e319)), _e319);
                            } else {
                                phi_7_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e319);
                            }
                            let _e327 = phi_7_;
                            let _e337 = ((abs((_e327.unnamed_1 - (_e311 * 600f))) - 80f) * -0.0125f);
                            let _e339 = select(_e337, 0f, (_e337 < 0f));
                            let _e341 = select(_e339, 1f, (_e339 > 1f));
                            let _e347 = (1f - _e311);
                            let _e348 = ((((_e341 * _e341) * (3f - (2f * _e341))) * _e301) * _e347);
                            let _e361 = (_e272 + (_e348 * 0.5f));
                            if (_e361 != _e361) {
                                phi_8_ = true;
                            } else {
                                phi_8_ = (1f <= _e361);
                            }
                            let _e365 = phi_8_;
                            phi_9_ = vec2<f32>((_e270.x + (((_e327.unnamed.x * _e348) * _e347) * 0.5f)), (_e270.y + (((_e327.unnamed.y * _e348) * _e347) * 0.5f)));
                            phi_10_ = select(_e361, 1f, _e365);
                        } else {
                            phi_9_ = _e270;
                            phi_10_ = _e272;
                        }
                        let _e368 = phi_9_;
                        let _e370 = phi_10_;
                        phi_11_ = _e368;
                        phi_12_ = _e370;
                    } else {
                        phi_11_ = _e270;
                        phi_12_ = _e272;
                    }
                    let _e372 = phi_11_;
                    let _e374 = phi_12_;
                    phi_13_ = _e372;
                    phi_14_ = _e374;
                    phi_15_ = (_e274 + 1u);
                } else {
                    phi_13_ = vec2<f32>();
                    phi_14_ = f32();
                    phi_15_ = u32();
                }
                let _e377 = phi_13_;
                let _e379 = phi_14_;
                let _e381 = phi_15_;
                continue;
                continuing {
                    phi_4_ = _e377;
                    phi_5_ = _e379;
                    phi_6_ = _e381;
                    phi_16_ = _e267;
                    break if !(_e275);
                }
            }
            let _e384 = phi_16_;
            if _e384 {
                break;
            }
            if _e240 {
                let _e389 = frame.member[0u].mouse_pos[0u];
                let _e394 = frame.member[0u].mouse_pos[1u];
                let _e395 = (_e211.x - _e389);
                let _e396 = (_e211.y - _e394);
                let _e402 = ((sqrt(((_e395 * _e395) + (_e396 * _e396))) - 150f) * -0.006666667f);
                let _e404 = select(_e402, 0f, (_e402 < 0f));
                let _e406 = select(_e404, 1f, (_e404 > 1f));
                phi_17_ = ((((_e406 * _e406) * (3f - (2f * _e406))) * _e239) * 8f);
            } else {
                phi_17_ = 0f;
            }
            let _e414 = phi_17_;
            let _e416 = local_64;
            let _e418 = global[0u];
            if (_e416.x == _e418) {
                let _e421 = local_65;
                let _e424 = global[1u];
                phi_18_ = (_e421.y == _e424);
            } else {
                phi_18_ = false;
            }
            let _e427 = phi_18_;
            if _e427 {
                phi_19_ = 0f;
            } else {
                let _e429 = local_66;
                phi_19_ = (sqrt(((_e416.x * _e416.x) + (_e429.y * _e429.y))) * 22f);
            }
            let _e437 = phi_19_;
            let _e439 = local_67;
            let _e445 = (_e222 - (_e212.w * 158f));
            let _e446 = (6f + _e226);
            let _e447 = (8f * _e212.w);
            let _e448 = ((244f * _e212.w) - _e447);
            if (_e448 != _e448) {
                phi_20_ = true;
            } else {
                phi_20_ = (0f >= _e448);
            }
            let _e452 = phi_20_;
            let _e458 = frame.member[0u].mouse_pos[0u];
            let _e463 = frame.member[0u].mouse_pos[1u];
            let _e466 = ((308f + (316f * _e212.w)) * 0.5f);
            let _e467 = (select(_e448, 0f, _e452) * 0.5f);
            let _e468 = (_e447 + _e467);
            let _e471 = (_e467 != _e467);
            if _e471 {
                phi_21_ = true;
            } else {
                phi_21_ = (18f <= _e467);
            }
            let _e474 = phi_21_;
            let _e477 = vec2<f32>(_e466, _e467);
            let _e478 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e211.x - _e445) - _e466), ((_e211.y - _e446) - _e468)), _e477, select(_e467, 18f, _e474));
            if _e471 {
                phi_22_ = true;
            } else {
                phi_22_ = (18f <= _e467);
            }
            let _e485 = phi_22_;
            let _e488 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e458 - _e445) - _e466), ((_e463 - _e446) - _e468)), _e477, select(_e467, 18f, _e485));
            let _e491 = (0.5f + ((_e478 - _e235) * 0.008928572f));
            let _e493 = select(_e491, 0f, (_e491 < 0f));
            let _e495 = select(_e493, 1f, (_e493 > 1f));
            let _e508 = (0.5f + ((_e488 - _e258) * 0.008928572f));
            let _e510 = select(_e508, 0f, (_e508 < 0f));
            let _e512 = select(_e510, 1f, (_e510 > 1f));
            let _e524 = (((_e258 + ((((_e488 + ((_e258 - _e488) * _e512)) - ((56f * _e512) * (1f - _e512))) - _e258) * _e212.w)) - 0.5f) * -1f);
            let _e526 = select(_e524, 0f, (_e524 < 0f));
            let _e528 = select(_e526, 1f, (_e526 > 1f));
            let _e536 = ((_e235 + ((((_e478 + ((_e235 - _e478) * _e495)) - ((56f * _e495) * (1f - _e495))) - _e235) * _e212.w)) - (((_e414 * ((_e528 * _e528) * (3f - (2f * _e528)))) + _e437) * 0.5f));
            let _e537 = fwidth(_e536);
            if (_e537 != _e537) {
                phi_23_ = true;
            } else {
                phi_23_ = (0.55f >= _e537);
            }
            let _e541 = phi_23_;
            let _e542 = select(_e537, 0.55f, _e541);
            let _e546 = ((_e536 - _e542) / (-(_e542) - _e542));
            let _e548 = select(_e546, 0f, (_e546 < 0f));
            let _e550 = select(_e548, 1f, (_e548 > 1f));
            let _e554 = ((_e550 * _e550) * (3f - (2f * _e550)));
            let _e555 = (_e536 != _e536);
            if _e555 {
                phi_24_ = true;
            } else {
                phi_24_ = (0f >= _e536);
            }
            let _e558 = phi_24_;
            let _e562 = (exp((select(_e536, 0f, _e558) * -0.3f)) * 0.16f);
            if (_e554 != _e554) {
                phi_25_ = true;
            } else {
                phi_25_ = (_e562 >= _e554);
            }
            let _e566 = phi_25_;
            let _e567 = select(_e554, _e562, _e566);
            if (_e567 <= 0.0009765625f) {
                discard;
            }
            let _e571 = ((_e228 - _e226) > (_e226 + 60f));
            let _e579 = (_e222 + 166f);
            let _e580 = (_e446 + (((56f + _e229) + (select(0f, 1f, _e571) * (_e226 + 8f))) - _e229));
            let _e581 = (_e211.x - _e579);
            let _e582 = (_e211.y - _e580);
            let _e583 = select(6u, 5u, _e571);
            let _e584 = (_e581 * 0.0034246575f);
            let _e587 = ((_e584 * f32(_e583)) - 0.5f);
            let _e589 = f32((_e583 - 1u));
            if (0f <= _e589) {
            } else {
                break;
            }
            let _e592 = select(_e587, 0f, (_e587 < 0f));
            let _e594 = select(_e592, _e589, (_e592 > _e589));
            let _e595 = floor(_e594);
            let _e600 = select(select(u32(_e595), 0u, (_e595 < 0f)), 4294967295u, (_e595 > 4294967000f));
            let _e602 = (_e594 - trunc(_e594));
            let _e604 = select(_e602, 0f, (_e602 < 0f));
            let _e606 = select(_e604, 1f, (_e604 > 1f));
            let _e610 = ((_e606 * _e606) * (3f - (2f * _e606)));
            if _e571 {
                if (_e600 < 5u) {
                } else {
                    break;
                }
                let _e638 = pill_2.member[_e213].daily_conditions[_e600];
                let _e639 = (_e600 + 1u);
                let _e641 = select(_e639, 4u, (4u < _e639));
                if (_e641 < 5u) {
                } else {
                    break;
                }
                let _e647 = pill_2.member[_e213].daily_conditions[_e641];
                phi_26_ = 12f;
                phi_27_ = _e647;
                phi_28_ = _e638;
            } else {
                if (_e600 < 6u) {
                } else {
                    break;
                }
                let _e616 = pill_2.member[_e213].hourly_conditions[_e600];
                let _e617 = (_e600 + 1u);
                let _e619 = select(_e617, 5u, (5u < _e617));
                if (_e619 < 6u) {
                } else {
                    break;
                }
                let _e625 = pill_2.member[_e213].hourly_conditions[_e619];
                let _e629 = pill_2.member[_e213].hourly_start;
                phi_26_ = ((_e629 + (_e594 * 4f)) % 24f);
                phi_27_ = _e625;
                phi_28_ = _e616;
            }
            let _e649 = phi_26_;
            let _e651 = phi_27_;
            let _e653 = phi_28_;
            let _e657 = ((292f - _e226) * 0.5f);
            let _e659 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e581 - 146f), (_e582 - _e229)), _e657, _e229);
            let _e665 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e458 - _e579) - 146f), ((_e463 - _e580) - _e229)), _e657, _e229);
            let _e700 = pill_2.member[_e213].sun_hours;
            let _e703 = (_e700[1] - _e700[0]);
            if (_e649 >= _e700[0]) {
                let _e705 = (_e649 <= _e700[1]);
                if _e705 {
                    let _e707 = ((_e649 - _e700[0]) / _e703);
                    phi_29_ = array<f32, 2>(_e707, sin((_e707 * 3.1415927f)));
                } else {
                    phi_29_ = array<f32, 2>();
                }
                let _e712 = phi_29_;
                phi_30_ = _e712;
                phi_31_ = select(true, false, _e705);
            } else {
                phi_30_ = array<f32, 2>();
                phi_31_ = true;
            }
            let _e715 = phi_30_;
            let _e717 = phi_31_;
            if _e717 {
                let _e718 = (24f - _e703);
                if (_e649 < _e700[0]) {
                    phi_32_ = (((_e649 + 24f) - _e700[1]) / _e718);
                } else {
                    phi_32_ = ((_e649 - _e700[1]) / _e718);
                }
                let _e726 = phi_32_;
                phi_33_ = array<f32, 2>(select(0f, 1f, (_e649 >= _e700[1])), -(sin((_e726 * 3.1415927f))));
            } else {
                phi_33_ = _e715;
            }
            let _e734 = phi_33_;
            let _e737 = ((_e665 - 0.5f) * -1f);
            let _e739 = select(_e737, 0f, (_e737 < 0f));
            let _e741 = select(_e739, 1f, (_e739 > 1f));
            let _e749 = (_e659 - (((_e414 * ((_e741 * _e741) * (3f - (2f * _e741)))) + _e437) * 0.5f));
            let _e754 = pill_2.member[_e213].hourly_conditions[0u];
            let _e755 = (_e227 * 0.0032467532f);
            let _e757 = select(_e755, 0f, (_e755 < 0f));
            let _e766 = pill_2.member[_e213].hourly_conditions[1u];
            let _e768 = ((abs((select(_e757, 1f, (_e757 > 1f)) - 0.5f)) - 0.05f) * 5f);
            let _e770 = select(_e768, 0f, (_e768 < 0f));
            let _e772 = select(_e770, 1f, (_e770 > 1f));
            let _e776 = ((_e772 * _e772) * (3f - (2f * _e772)));
            let _e781 = (_e754.fog + ((_e766.fog - _e754.fog) * _e776));
            let _e786 = (_e754.cloud + ((_e766.cloud - _e754.cloud) * _e776));
            let _e791 = (_e754.rain + ((_e766.rain - _e754.rain) * _e776));
            let _e796 = (_e754.snow + ((_e766.snow - _e754.snow) * _e776));
            let _e801 = (_e754.lightning + ((_e766.lightning - _e754.lightning) * _e776));
            let _e806 = (_e754.hail + ((_e766.hail - _e754.hail) * _e776));
            let _e809 = (_e781 + ((_e754.fog - _e781) * _e212.w));
            let _e812 = (_e786 + ((_e754.cloud - _e786) * _e212.w));
            let _e815 = (_e791 + ((_e754.rain - _e791) * _e212.w));
            let _e818 = (_e796 + ((_e754.snow - _e796) * _e212.w));
            let _e821 = (_e801 + ((_e754.lightning - _e801) * _e212.w));
            let _e824 = (_e806 + ((_e754.hail - _e806) * _e212.w));
            let _e826 = (_e228 / _e226);
            if _e555 {
                phi_34_ = true;
            } else {
                phi_34_ = (0f <= _e536);
            }
            let _e831 = phi_34_;
            let _e834 = (1f + (select(_e536, 0f, _e831) * 0.008333334f));
            let _e836 = select(_e834, 0f, (_e834 < 0f));
            let _e838 = select(_e836, 0.6f, (_e836 > 0.6f));
            let _e845 = (_e416.x * 0.04f);
            let _e846 = (_e439.y * 0.04f);
            let _e847 = ((_e755 - (((_e755 - 0.5f) * _e838) * 0.08f)) - _e845);
            let _e848 = ((_e826 - (((_e826 - 0.5f) * _e838) * 0.08f)) - _e846);
            let _e849 = (_e582 / _e226);
            if (_e749 != _e749) {
                phi_35_ = true;
            } else {
                phi_35_ = (0f <= _e749);
            }
            let _e855 = phi_35_;
            let _e858 = (1f + (select(_e749, 0f, _e855) * 0.008333334f));
            let _e860 = select(_e858, 0f, (_e858 < 0f));
            let _e862 = select(_e860, 0.6f, (_e860 > 0.6f));
            let _e871 = fwidth(_e749);
            if (_e871 != _e871) {
                phi_36_ = true;
            } else {
                phi_36_ = (0.55f >= _e871);
            }
            let _e875 = phi_36_;
            let _e876 = select(_e871, 0.55f, _e875);
            let _e880 = ((_e749 - _e876) / (-(_e876) - _e876));
            let _e882 = select(_e880, 0f, (_e880 < 0f));
            let _e884 = select(_e882, 1f, (_e882 > 1f));
            let _e888 = ((_e884 * _e884) * (3f - (2f * _e884)));
            let _e889 = (_e888 > 0.001f);
            if _e889 {
                let _e941 = ((_e734[1] - -0.04f) * 4.1666665f);
                let _e943 = select(_e941, 0f, (_e941 < 0f));
                let _e945 = select(_e943, 1f, (_e943 > 1f));
                let _e949 = ((_e945 * _e945) * (3f - (2f * _e945)));
                let _e951 = ((_e734[1] - -0.32f) * 4.166667f);
                let _e953 = select(_e951, 0f, (_e951 < 0f));
                let _e955 = select(_e953, 1f, (_e953 > 1f));
                let _e963 = ((_e734[1] - -0.18f) * 5.5555553f);
                let _e965 = select(_e963, 0f, (_e963 < 0f));
                let _e967 = select(_e965, 1f, (_e965 > 1f));
                let _e973 = ((_e734[1] - 0.2f) * -5.5555553f);
                let _e975 = select(_e973, 0f, (_e973 < 0f));
                let _e977 = select(_e975, 1f, (_e975 > 1f));
                phi_37_ = vec3<f32>(_e949, (((_e955 * _e955) * (3f - (2f * _e955))) * (1f - _e949)), (((_e967 * _e967) * (3f - (2f * _e967))) * ((_e977 * _e977) * (3f - (2f * _e977)))));
                phi_38_ = vec2<f32>((((_e584 - (((_e584 - 0.5f) * _e862) * 0.08f)) - _e845) * 292f), (((_e849 - (((_e849 - 0.5f) * _e862) * 0.08f)) - _e846) * _e226));
                phi_39_ = render_tempestas_WeatherCondition((_e653.fog + ((_e651.fog - _e653.fog) * _e610)), (_e653.cloud + ((_e651.cloud - _e653.cloud) * _e610)), (_e653.rain + ((_e651.rain - _e653.rain) * _e610)), (_e653.snow + ((_e651.snow - _e653.snow) * _e610)), (_e653.lightning + ((_e651.lightning - _e653.lightning) * _e610)), (_e653.hail + ((_e651.hail - _e653.hail) * _e610)));
            } else {
                let _e894 = ((_e212.y - -0.04f) * 4.1666665f);
                let _e896 = select(_e894, 0f, (_e894 < 0f));
                let _e898 = select(_e896, 1f, (_e896 > 1f));
                let _e902 = ((_e898 * _e898) * (3f - (2f * _e898)));
                let _e904 = ((_e212.y - -0.32f) * 4.166667f);
                let _e906 = select(_e904, 0f, (_e904 < 0f));
                let _e908 = select(_e906, 1f, (_e906 > 1f));
                let _e916 = ((_e212.y - -0.18f) * 5.5555553f);
                let _e918 = select(_e916, 0f, (_e916 < 0f));
                let _e920 = select(_e918, 1f, (_e918 > 1f));
                let _e926 = ((_e212.y - 0.2f) * -5.5555553f);
                let _e928 = select(_e926, 0f, (_e926 < 0f));
                let _e930 = select(_e928, 1f, (_e928 > 1f));
                phi_37_ = vec3<f32>(_e902, (((_e908 * _e908) * (3f - (2f * _e908))) * (1f - _e902)), (((_e920 * _e920) * (3f - (2f * _e920))) * ((_e930 * _e930) * (3f - (2f * _e930)))));
                phi_38_ = vec2<f32>((_e847 * 308f), (_e848 * _e226));
                phi_39_ = render_tempestas_WeatherCondition(_e809, _e812, _e815, _e818, _e821, _e824);
            }
            let _e985 = phi_37_;
            let _e987 = phi_38_;
            let _e989 = phi_39_;
            let _e997 = frame.member[0u].time;
            let _e998 = (_e987.y / _e226);
            let _e1000 = ((_e998 - 1f) * -1f);
            let _e1002 = select(_e1000, 0f, (_e1000 < 0f));
            let _e1004 = select(_e1002, 1f, (_e1002 > 1f));
            let _e1008 = ((_e1004 * _e1004) * (3f - (2f * _e1004)));
            let _e1009 = (1f - _e1008);
            let _e1029 = (1f - _e985.x);
            let _e1041 = (0.3f * _e1009);
            let _e1042 = (0.22f * _e1008);
            let _e1049 = (_e985.y * 0.8f);
            let _e1050 = (1f - _e1049);
            let _e1068 = (_e985.z * 0.9f);
            let _e1069 = (1f - _e1068);
            let _e1081 = floor((_e987.x * 0.055555556f));
            let _e1082 = floor((_e987.y * 0.055555556f));
            let _e1086 = cantus_render_shader_hash(vec2<f32>(_e1081, _e1082));
            let _e1095 = (_e987.x - (((_e1081 + 0.2f) + (_e1086.x * 0.6f)) * 18f));
            let _e1096 = (_e987.y - (((_e1082 + 0.2f) + (_e1086.y * 0.6f)) * 18f));
            let _e1102 = ((sqrt(((_e1095 * _e1095) + (_e1096 * _e1096))) - 1f) * -1.6666666f);
            let _e1104 = select(_e1102, 0f, (_e1102 < 0f));
            let _e1106 = select(_e1104, 1f, (_e1104 > 1f));
            let _e1114 = cantus_render_shader_hash(vec2<f32>((_e1081 + 31.7f), (_e1082 + 31.7f)));
            let _e1117 = ((_e1114.x - 0.75f) * 4f);
            let _e1119 = select(_e1117, 0f, (_e1117 < 0f));
            let _e1121 = select(_e1119, 1f, (_e1119 > 1f));
            let _e1133 = ((((((_e1106 * _e1106) * (3f - (2f * _e1106))) * ((_e1121 * _e1121) * (3f - (2f * _e1121)))) * _e1029) * (1f - _e989.cloud)) * (0.3f + (_e1008 * 0.7f)));
            let _e1134 = (((((((((0.006f * _e1009) + (0.025f * _e1008)) * _e1029) + (((0.08f * _e1009) + (0.32f * _e1008)) * _e985.x)) * _e1050) + (((0.1f * _e1009) + _e1042) * _e1049)) * _e1069) + (((0.78f * _e1009) + (0.38f * _e1008)) * _e1068)) + _e1133);
            let _e1135 = (((((((((0.012f * _e1009) + (0.04f * _e1008)) * _e1029) + (((0.34f * _e1009) + (0.67f * _e1008)) * _e985.x)) * _e1050) + (((0.16f * _e1009) + (0.25f * _e1008)) * _e1049)) * _e1069) + ((_e1041 + _e1042) * _e1068)) + _e1133);
            let _e1136 = (((((((((0.035f * _e1009) + (0.095f * _e1008)) * _e1029) + (((0.62f * _e1009) + (0.87f * _e1008)) * _e985.x)) * _e1050) + ((_e1041 + (0.45f * _e1008)) * _e1049)) * _e1069) + (((0.2f * _e1009) + (0.42f * _e1008)) * _e1068)) + _e1133);
            if (_e989.cloud > 0.0009765625f) {
                let _e1139 = (_e987.x / _e226);
                phi_40_ = 0i;
                phi_41_ = 0.5f;
                phi_42_ = 0f;
                phi_43_ = vec2<f32>(((_e1139 * 0.14f) + (_e997 * 0.012f)), ((_e998 * 0.14f) + 6.1f));
                loop {
                    let _e1147 = phi_40_;
                    let _e1149 = phi_41_;
                    let _e1151 = phi_42_;
                    let _e1153 = phi_43_;
                    local_68 = _e1151;
                    let _e1154 = (_e1147 < 4i);
                    if _e1154 {
                        let _e1157 = cantus_render_shader_simplex_noise(_e1153);
                        phi_44_ = (_e1147 + 1i);
                        phi_45_ = (_e1149 * 0.5f);
                        phi_46_ = (_e1151 + (_e1157 * _e1149));
                        phi_47_ = vec2<f32>(((_e1153.x * 1.6f) + (_e1153.y * 1.2f)), ((_e1153.y * 1.6f) - (_e1153.x * 1.2f)));
                    } else {
                        phi_44_ = i32();
                        phi_45_ = f32();
                        phi_46_ = f32();
                        phi_47_ = vec2<f32>();
                    }
                    let _e1170 = phi_44_;
                    let _e1172 = phi_45_;
                    let _e1174 = phi_46_;
                    let _e1176 = phi_47_;
                    continue;
                    continuing {
                        phi_40_ = _e1170;
                        phi_41_ = _e1172;
                        phi_42_ = _e1174;
                        phi_43_ = _e1176;
                        break if !(_e1154);
                    }
                }
                let _e1179 = local_68;
                let _e1180 = (_e1179 * 0.5f);
                phi_48_ = 0i;
                phi_49_ = 0.5f;
                phi_50_ = 0f;
                phi_51_ = vec2<f32>(((_e1139 * 0.287f) + (_e997 * 0.018f)), ((_e998 * 0.287f) + -3.7f));
                loop {
                    let _e1189 = phi_48_;
                    let _e1191 = phi_49_;
                    let _e1193 = phi_50_;
                    let _e1195 = phi_51_;
                    local_69 = _e1193;
                    local_70 = _e1193;
                    let _e1196 = (_e1189 < 4i);
                    if _e1196 {
                        let _e1199 = cantus_render_shader_simplex_noise(_e1195);
                        phi_52_ = (_e1189 + 1i);
                        phi_53_ = (_e1191 * 0.5f);
                        phi_54_ = (_e1193 + (_e1199 * _e1191));
                        phi_55_ = vec2<f32>(((_e1195.x * 1.6f) + (_e1195.y * 1.2f)), ((_e1195.y * 1.6f) - (_e1195.x * 1.2f)));
                    } else {
                        phi_52_ = i32();
                        phi_53_ = f32();
                        phi_54_ = f32();
                        phi_55_ = vec2<f32>();
                    }
                    let _e1212 = phi_52_;
                    let _e1214 = phi_53_;
                    let _e1216 = phi_54_;
                    let _e1218 = phi_55_;
                    continue;
                    continuing {
                        phi_48_ = _e1212;
                        phi_49_ = _e1214;
                        phi_50_ = _e1216;
                        phi_51_ = _e1218;
                        break if !(_e1196);
                    }
                }
                let _e1221 = local_69;
                let _e1224 = local_70;
                let _e1228 = ((((0.5f + _e1180) + (_e1224 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1230 = select(_e1228, 0f, (_e1228 < 0f));
                let _e1232 = select(_e1230, 1f, (_e1230 > 1f));
                let _e1238 = (((_e1221 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1240 = select(_e1238, 0f, (_e1238 < 0f));
                let _e1242 = select(_e1240, 1f, (_e1240 > 1f));
                let _e1249 = ((_e1180 + 0.02000001f) * 4.5454545f);
                let _e1251 = select(_e1249, 0f, (_e1249 < 0f));
                let _e1253 = select(_e1251, 1f, (_e1251 > 1f));
                let _e1259 = ((((_e1242 * _e1242) * (3f - (2f * _e1242))) * 0.55f) + (((_e1253 * _e1253) * (3f - (2f * _e1253))) * 0.45f));
                let _e1260 = (1f - _e1259);
                let _e1297 = (_e985.z * 0.45f);
                let _e1298 = (1f - _e1297);
                let _e1310 = (_e989.cloud * (0.12f + (((_e1232 * _e1232) * (3f - (2f * _e1232))) * 0.7f)));
                let _e1311 = (1f - _e1310);
                phi_56_ = vec3<f32>(((_e1134 * _e1311) + (((((((0.16f * _e1260) + (0.32f * _e1259)) * _e1029) + (((0.62f * _e1260) + (0.92f * _e1259)) * _e985.x)) * _e1298) + (((0.5f * _e1260) + (0.76f * _e1259)) * _e1297)) * _e1310)), ((_e1135 * _e1311) + (((((((0.2f * _e1260) + (0.36f * _e1259)) * _e1029) + (((0.7f * _e1260) + (0.94f * _e1259)) * _e985.x)) * _e1298) + (((0.36f * _e1260) + (0.59f * _e1259)) * _e1297)) * _e1310)), ((_e1136 * _e1311) + (((((((0.28f * _e1260) + (0.43f * _e1259)) * _e1029) + (((0.78f * _e1260) + (0.96f * _e1259)) * _e985.x)) * _e1298) + (((0.4f * _e1260) + (0.56f * _e1259)) * _e1297)) * _e1310)));
            } else {
                phi_56_ = vec3<f32>(_e1134, _e1135, _e1136);
            }
            let _e1323 = phi_56_;
            let _e1326 = (1f - (_e989.rain * 0.2f));
            let _e1336 = ((_e1323.x * _e1326) + (_e989.rain * 0.020000001f));
            let _e1337 = ((_e1323.y * _e1326) + (_e989.rain * 0.034f));
            let _e1338 = ((_e1323.z * _e1326) + (_e989.rain * 0.05f));
            if (_e989.rain > 0.0009765625f) {
                let _e1343 = (_e987.x - (20f * _e997));
                let _e1344 = (_e987.y - (110f * _e997));
                let _e1347 = floor((_e1343 * 0.06666667f));
                let _e1348 = floor((_e1344 * 0.04f));
                let _e1350 = cantus_render_shader_hash(vec2<f32>(_e1347, _e1348));
                let _e1361 = (_e1343 - (((_e1347 + 0.15f) + (_e1350.x * 0.7f)) * 15f));
                let _e1362 = (_e1344 - (((_e1348 + 0.15f) + (_e1350.y * 0.7f)) * 25f));
                let _e1366 = (((_e1361 * 1.8000001f) + (_e1362 * 9f)) * 0.011870845f);
                let _e1368 = select(_e1366, 0f, (_e1366 < 0f));
                let _e1370 = select(_e1368, 1f, (_e1368 > 1f));
                let _e1373 = (_e1361 - (1.8000001f * _e1370));
                let _e1374 = (_e1362 - (9f * _e1370));
                let _e1380 = ((sqrt(((_e1373 * _e1373) + (_e1374 * _e1374))) - 1.0999999f) * -1.666667f);
                let _e1382 = select(_e1380, 0f, (_e1380 < 0f));
                let _e1384 = select(_e1382, 1f, (_e1382 > 1f));
                let _e1392 = cantus_render_shader_hash(vec2<f32>((_e1347 + 19.3f), (_e1348 + 19.3f)));
                let _e1395 = ((_e1392.x - 0.22000003f) * 1.2820513f);
                let _e1397 = select(_e1395, 0f, (_e1395 < 0f));
                let _e1399 = select(_e1397, 1f, (_e1397 > 1f));
                let _e1406 = (((((_e1384 * _e1384) * (3f - (2f * _e1384))) * ((_e1399 * _e1399) * (3f - (2f * _e1399)))) * _e989.rain) * 0.7f);
                let _e1408 = select(_e1406, 0f, (_e1406 < 0f));
                let _e1410 = select(_e1408, 1f, (_e1408 > 1f));
                let _e1411 = (1f - _e1410);
                phi_57_ = vec3<f32>(((_e1336 * _e1411) + (0.52f * _e1410)), ((_e1337 * _e1411) + (0.72f * _e1410)), ((_e1338 * _e1411) + (0.9f * _e1410)));
            } else {
                phi_57_ = vec3<f32>(_e1336, _e1337, _e1338);
            }
            let _e1423 = phi_57_;
            if (_e989.snow > 0.0009765625f) {
                let _e1428 = (_e987.x - (5f * _e997));
                let _e1429 = (_e987.y - (14f * _e997));
                let _e1432 = floor((_e1428 * 0.05f));
                let _e1433 = floor((_e1429 * 0.05f));
                let _e1437 = cantus_render_shader_hash(vec2<f32>((_e1432 + 31.7f), (_e1433 + 31.7f)));
                let _e1448 = (_e1428 - (((_e1432 + 0.15f) + (_e1437.x * 0.7f)) * 20f));
                let _e1449 = (_e1429 - (((_e1433 + 0.15f) + (_e1437.y * 0.7f)) * 20f));
                let _e1453 = (((_e1448 * 0.080000006f) + (_e1449 * 0.4f)) * 6.009615f);
                let _e1455 = select(_e1453, 0f, (_e1453 < 0f));
                let _e1457 = select(_e1455, 1f, (_e1455 > 1f));
                let _e1460 = (_e1448 - (0.080000006f * _e1457));
                let _e1461 = (_e1449 - (0.4f * _e1457));
                let _e1467 = ((sqrt(((_e1460 * _e1460) + (_e1461 * _e1461))) - 1.5999999f) * -1.666667f);
                let _e1469 = select(_e1467, 0f, (_e1467 < 0f));
                let _e1471 = select(_e1469, 1f, (_e1469 > 1f));
                let _e1479 = cantus_render_shader_hash(vec2<f32>((_e1432 + 19.3f), (_e1433 + 19.3f)));
                let _e1482 = ((_e1479.x - 0.3f) * 1.4285715f);
                let _e1484 = select(_e1482, 0f, (_e1482 < 0f));
                let _e1486 = select(_e1484, 1f, (_e1484 > 1f));
                let _e1493 = (((((_e1471 * _e1471) * (3f - (2f * _e1471))) * ((_e1486 * _e1486) * (3f - (2f * _e1486)))) * _e989.snow) * 0.92f);
                let _e1495 = select(_e1493, 0f, (_e1493 < 0f));
                let _e1497 = select(_e1495, 1f, (_e1495 > 1f));
                let _e1498 = (1f - _e1497);
                let _e1505 = (0.96f * _e1497);
                phi_58_ = vec3<f32>(((_e1423.x * _e1498) + _e1505), ((_e1423.y * _e1498) + _e1505), ((_e1423.z * _e1498) + _e1505));
            } else {
                phi_58_ = _e1423;
            }
            let _e1511 = phi_58_;
            if (_e989.hail > 0.0009765625f) {
                let _e1516 = (_e987.x - (18f * _e997));
                let _e1517 = (_e987.y - (85f * _e997));
                let _e1520 = floor((_e1516 * 0.04347826f));
                let _e1521 = floor((_e1517 * 0.04347826f));
                let _e1525 = cantus_render_shader_hash(vec2<f32>((_e1520 + 63.4f), (_e1521 + 63.4f)));
                let _e1536 = (_e1516 - (((_e1520 + 0.15f) + (_e1525.x * 0.7f)) * 23f));
                let _e1537 = (_e1517 - (((_e1521 + 0.15f) + (_e1525.y * 0.7f)) * 23f));
                let _e1541 = (((_e1536 * 0.24000001f) + (_e1537 * 1.2f)) * 0.667735f);
                let _e1543 = select(_e1541, 0f, (_e1541 < 0f));
                let _e1545 = select(_e1543, 1f, (_e1543 > 1f));
                let _e1548 = (_e1536 - (0.24000001f * _e1545));
                let _e1549 = (_e1537 - (1.2f * _e1545));
                let _e1555 = ((sqrt(((_e1548 * _e1548) + (_e1549 * _e1549))) - 0.79999995f) * -1.6666667f);
                let _e1557 = select(_e1555, 0f, (_e1555 < 0f));
                let _e1559 = select(_e1557, 1f, (_e1557 > 1f));
                let _e1567 = cantus_render_shader_hash(vec2<f32>((_e1520 + 19.3f), (_e1521 + 19.3f)));
                let _e1570 = ((_e1567.x - 0.7f) * 3.3333333f);
                let _e1572 = select(_e1570, 0f, (_e1570 < 0f));
                let _e1574 = select(_e1572, 1f, (_e1572 > 1f));
                let _e1581 = (((((_e1559 * _e1559) * (3f - (2f * _e1559))) * ((_e1574 * _e1574) * (3f - (2f * _e1574)))) * _e989.hail) * 0.7f);
                let _e1583 = select(_e1581, 0f, (_e1581 < 0f));
                let _e1585 = select(_e1583, 1f, (_e1583 > 1f));
                let _e1586 = (1f - _e1585);
                phi_59_ = vec3<f32>(((_e1511.x * _e1586) + (0.75f * _e1585)), ((_e1511.y * _e1586) + (0.86f * _e1585)), ((_e1511.z * _e1586) + (0.94f * _e1585)));
            } else {
                phi_59_ = _e1511;
            }
            let _e1601 = phi_59_;
            let _e1605 = ((sin((_e997 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1607 = select(_e1605, 0f, (_e1605 < 0f));
            let _e1609 = select(_e1607, 1f, (_e1607 > 1f));
            let _e1613 = ((_e1609 * _e1609) * (3f - (2f * _e1609)));
            let _e1615 = (_e1613 * _e989.lightning);
            let _e1617 = (1f - (_e1615 * 0.55f));
            let _e1627 = ((_e1601.x * _e1617) + (_e1615 * 0.3575f));
            let _e1628 = ((_e1601.y * _e1617) + (_e1615 * 0.407f));
            let _e1629 = ((_e1601.z * _e1617) + (_e1615 * 0.528f));
            if (_e989.fog > 0.0009765625f) {
                phi_60_ = 0i;
                phi_61_ = 0.5f;
                phi_62_ = 0f;
                phi_63_ = vec2<f32>((((_e987.x / select(308f, 292f, _e889)) * 0.9f) + (_e997 * 0.008f)), ((_e998 * 0.32f) + 12f));
                loop {
                    let _e1641 = phi_60_;
                    let _e1643 = phi_61_;
                    let _e1645 = phi_62_;
                    let _e1647 = phi_63_;
                    local_71 = _e1645;
                    let _e1648 = (_e1641 < 4i);
                    if _e1648 {
                        let _e1651 = cantus_render_shader_simplex_noise(_e1647);
                        phi_64_ = (_e1641 + 1i);
                        phi_65_ = (_e1643 * 0.5f);
                        phi_66_ = (_e1645 + (_e1651 * _e1643));
                        phi_67_ = vec2<f32>(((_e1647.x * 1.6f) + (_e1647.y * 1.2f)), ((_e1647.y * 1.6f) - (_e1647.x * 1.2f)));
                    } else {
                        phi_64_ = i32();
                        phi_65_ = f32();
                        phi_66_ = f32();
                        phi_67_ = vec2<f32>();
                    }
                    let _e1664 = phi_64_;
                    let _e1666 = phi_65_;
                    let _e1668 = phi_66_;
                    let _e1670 = phi_67_;
                    continue;
                    continuing {
                        phi_60_ = _e1664;
                        phi_61_ = _e1666;
                        phi_62_ = _e1668;
                        phi_63_ = _e1670;
                        break if !(_e1648);
                    }
                }
                let _e1673 = local_71;
                let _e1676 = (((_e1673 * 0.5f) + 0.15f) * 2.857143f);
                let _e1678 = select(_e1676, 0f, (_e1676 < 0f));
                let _e1680 = select(_e1678, 1f, (_e1678 > 1f));
                let _e1687 = (_e989.fog * (0.58f + (((_e1680 * _e1680) * (3f - (2f * _e1680))) * 0.18f)));
                let _e1688 = (1f - _e1687);
                phi_68_ = vec3<f32>(((_e1627 * _e1688) + (0.63f * _e1687)), ((_e1628 * _e1688) + (0.69f * _e1687)), ((_e1629 * _e1688) + (0.73f * _e1687)));
            } else {
                phi_68_ = vec3<f32>(_e1627, _e1628, _e1629);
            }
            let _e1700 = phi_68_;
            let _e1702 = ((select(_e536, _e749, _e889) - 5f) * -0.125f);
            let _e1704 = select(_e1702, 0f, (_e1702 < 0f));
            let _e1706 = select(_e1704, 1f, (_e1704 > 1f));
            let _e1711 = (((_e1706 * _e1706) * (3f - (2f * _e1706))) * 0.14f);
            let _e1719 = (_e1700 + vec3(_e1711));
            if _e889 {
                if (_e888 < 0.999f) {
                    let _e1721 = (_e847 * 308f);
                    let _e1722 = (_e848 * _e226);
                    let _e1724 = ((_e212.y - -0.04f) * 4.1666665f);
                    let _e1726 = select(_e1724, 0f, (_e1724 < 0f));
                    let _e1728 = select(_e1726, 1f, (_e1726 > 1f));
                    let _e1732 = ((_e1728 * _e1728) * (3f - (2f * _e1728)));
                    let _e1734 = ((_e212.y - -0.32f) * 4.166667f);
                    let _e1736 = select(_e1734, 0f, (_e1734 < 0f));
                    let _e1738 = select(_e1736, 1f, (_e1736 > 1f));
                    let _e1743 = (1f - _e1732);
                    let _e1746 = ((_e212.y - -0.18f) * 5.5555553f);
                    let _e1748 = select(_e1746, 0f, (_e1746 < 0f));
                    let _e1750 = select(_e1748, 1f, (_e1748 > 1f));
                    let _e1756 = ((_e212.y - 0.2f) * -5.5555553f);
                    let _e1758 = select(_e1756, 0f, (_e1756 < 0f));
                    let _e1760 = select(_e1758, 1f, (_e1758 > 1f));
                    let _e1765 = (((_e1750 * _e1750) * (3f - (2f * _e1750))) * ((_e1760 * _e1760) * (3f - (2f * _e1760))));
                    let _e1767 = ((_e848 - 1f) * -1f);
                    let _e1769 = select(_e1767, 0f, (_e1767 < 0f));
                    let _e1771 = select(_e1769, 1f, (_e1769 > 1f));
                    let _e1775 = ((_e1771 * _e1771) * (3f - (2f * _e1771)));
                    let _e1776 = (1f - _e1775);
                    let _e1806 = (0.3f * _e1776);
                    let _e1807 = (0.22f * _e1775);
                    let _e1813 = ((((_e1738 * _e1738) * (3f - (2f * _e1738))) * _e1743) * 0.8f);
                    let _e1814 = (1f - _e1813);
                    let _e1831 = (_e1765 * 0.9f);
                    let _e1832 = (1f - _e1831);
                    let _e1844 = floor((_e847 * 17.11111f));
                    let _e1845 = floor((_e1722 * 0.055555556f));
                    let _e1849 = cantus_render_shader_hash(vec2<f32>(_e1844, _e1845));
                    let _e1858 = (_e1721 - (((_e1844 + 0.2f) + (_e1849.x * 0.6f)) * 18f));
                    let _e1859 = (_e1722 - (((_e1845 + 0.2f) + (_e1849.y * 0.6f)) * 18f));
                    let _e1865 = ((sqrt(((_e1858 * _e1858) + (_e1859 * _e1859))) - 1f) * -1.6666666f);
                    let _e1867 = select(_e1865, 0f, (_e1865 < 0f));
                    let _e1869 = select(_e1867, 1f, (_e1867 > 1f));
                    let _e1877 = cantus_render_shader_hash(vec2<f32>((_e1844 + 31.7f), (_e1845 + 31.7f)));
                    let _e1880 = ((_e1877.x - 0.75f) * 4f);
                    let _e1882 = select(_e1880, 0f, (_e1880 < 0f));
                    let _e1884 = select(_e1882, 1f, (_e1882 > 1f));
                    let _e1895 = ((((((_e1869 * _e1869) * (3f - (2f * _e1869))) * ((_e1884 * _e1884) * (3f - (2f * _e1884)))) * _e1743) * (1f - _e812)) * (0.3f + (_e1775 * 0.7f)));
                    let _e1896 = (((((((((0.006f * _e1776) + (0.025f * _e1775)) * _e1743) + (((0.08f * _e1776) + (0.32f * _e1775)) * _e1732)) * _e1814) + (((0.1f * _e1776) + _e1807) * _e1813)) * _e1832) + (((0.78f * _e1776) + (0.38f * _e1775)) * _e1831)) + _e1895);
                    let _e1897 = (((((((((0.012f * _e1776) + (0.04f * _e1775)) * _e1743) + (((0.34f * _e1776) + (0.67f * _e1775)) * _e1732)) * _e1814) + (((0.16f * _e1776) + (0.25f * _e1775)) * _e1813)) * _e1832) + ((_e1806 + _e1807) * _e1831)) + _e1895);
                    let _e1898 = (((((((((0.035f * _e1776) + (0.095f * _e1775)) * _e1743) + (((0.62f * _e1776) + (0.87f * _e1775)) * _e1732)) * _e1814) + ((_e1806 + (0.45f * _e1775)) * _e1813)) * _e1832) + (((0.2f * _e1776) + (0.42f * _e1775)) * _e1831)) + _e1895);
                    if (_e812 > 0.0009765625f) {
                        let _e1901 = (_e1721 / _e226);
                        phi_69_ = 0i;
                        phi_70_ = 0.5f;
                        phi_71_ = 0f;
                        phi_72_ = vec2<f32>(((_e1901 * 0.14f) + (_e997 * 0.012f)), ((_e848 * 0.14f) + 6.1f));
                        loop {
                            let _e1909 = phi_69_;
                            let _e1911 = phi_70_;
                            let _e1913 = phi_71_;
                            let _e1915 = phi_72_;
                            local_72 = _e1913;
                            let _e1916 = (_e1909 < 4i);
                            if _e1916 {
                                let _e1919 = cantus_render_shader_simplex_noise(_e1915);
                                phi_73_ = (_e1909 + 1i);
                                phi_74_ = (_e1911 * 0.5f);
                                phi_75_ = (_e1913 + (_e1919 * _e1911));
                                phi_76_ = vec2<f32>(((_e1915.x * 1.6f) + (_e1915.y * 1.2f)), ((_e1915.y * 1.6f) - (_e1915.x * 1.2f)));
                            } else {
                                phi_73_ = i32();
                                phi_74_ = f32();
                                phi_75_ = f32();
                                phi_76_ = vec2<f32>();
                            }
                            let _e1932 = phi_73_;
                            let _e1934 = phi_74_;
                            let _e1936 = phi_75_;
                            let _e1938 = phi_76_;
                            continue;
                            continuing {
                                phi_69_ = _e1932;
                                phi_70_ = _e1934;
                                phi_71_ = _e1936;
                                phi_72_ = _e1938;
                                break if !(_e1916);
                            }
                        }
                        let _e1941 = local_72;
                        let _e1942 = (_e1941 * 0.5f);
                        phi_77_ = 0i;
                        phi_78_ = 0.5f;
                        phi_79_ = 0f;
                        phi_80_ = vec2<f32>(((_e1901 * 0.287f) + (_e997 * 0.018f)), ((_e848 * 0.287f) + -3.7f));
                        loop {
                            let _e1951 = phi_77_;
                            let _e1953 = phi_78_;
                            let _e1955 = phi_79_;
                            let _e1957 = phi_80_;
                            local_73 = _e1955;
                            local_74 = _e1955;
                            let _e1958 = (_e1951 < 4i);
                            if _e1958 {
                                let _e1961 = cantus_render_shader_simplex_noise(_e1957);
                                phi_81_ = (_e1951 + 1i);
                                phi_82_ = (_e1953 * 0.5f);
                                phi_83_ = (_e1955 + (_e1961 * _e1953));
                                phi_84_ = vec2<f32>(((_e1957.x * 1.6f) + (_e1957.y * 1.2f)), ((_e1957.y * 1.6f) - (_e1957.x * 1.2f)));
                            } else {
                                phi_81_ = i32();
                                phi_82_ = f32();
                                phi_83_ = f32();
                                phi_84_ = vec2<f32>();
                            }
                            let _e1974 = phi_81_;
                            let _e1976 = phi_82_;
                            let _e1978 = phi_83_;
                            let _e1980 = phi_84_;
                            continue;
                            continuing {
                                phi_77_ = _e1974;
                                phi_78_ = _e1976;
                                phi_79_ = _e1978;
                                phi_80_ = _e1980;
                                break if !(_e1958);
                            }
                        }
                        let _e1983 = local_73;
                        let _e1986 = local_74;
                        let _e1990 = ((((0.5f + _e1942) + (_e1986 * 0.12f)) - 0.35f) * 3.9999995f);
                        let _e1992 = select(_e1990, 0f, (_e1990 < 0f));
                        let _e1994 = select(_e1992, 1f, (_e1992 > 1f));
                        let _e2000 = (((_e1983 * 0.5f) + 0.08000001f) * 3.3333328f);
                        let _e2002 = select(_e2000, 0f, (_e2000 < 0f));
                        let _e2004 = select(_e2002, 1f, (_e2002 > 1f));
                        let _e2011 = ((_e1942 + 0.02000001f) * 4.5454545f);
                        let _e2013 = select(_e2011, 0f, (_e2011 < 0f));
                        let _e2015 = select(_e2013, 1f, (_e2013 > 1f));
                        let _e2021 = ((((_e2004 * _e2004) * (3f - (2f * _e2004))) * 0.55f) + (((_e2015 * _e2015) * (3f - (2f * _e2015))) * 0.45f));
                        let _e2022 = (1f - _e2021);
                        let _e2059 = (_e1765 * 0.45f);
                        let _e2060 = (1f - _e2059);
                        let _e2072 = (_e812 * (0.12f + (((_e1994 * _e1994) * (3f - (2f * _e1994))) * 0.7f)));
                        let _e2073 = (1f - _e2072);
                        phi_85_ = vec3<f32>(((_e1896 * _e2073) + (((((((0.16f * _e2022) + (0.32f * _e2021)) * _e1743) + (((0.62f * _e2022) + (0.92f * _e2021)) * _e1732)) * _e2060) + (((0.5f * _e2022) + (0.76f * _e2021)) * _e2059)) * _e2072)), ((_e1897 * _e2073) + (((((((0.2f * _e2022) + (0.36f * _e2021)) * _e1743) + (((0.7f * _e2022) + (0.94f * _e2021)) * _e1732)) * _e2060) + (((0.36f * _e2022) + (0.59f * _e2021)) * _e2059)) * _e2072)), ((_e1898 * _e2073) + (((((((0.28f * _e2022) + (0.43f * _e2021)) * _e1743) + (((0.78f * _e2022) + (0.96f * _e2021)) * _e1732)) * _e2060) + (((0.4f * _e2022) + (0.56f * _e2021)) * _e2059)) * _e2072)));
                    } else {
                        phi_85_ = vec3<f32>(_e1896, _e1897, _e1898);
                    }
                    let _e2085 = phi_85_;
                    let _e2087 = (1f - (_e815 * 0.2f));
                    let _e2097 = ((_e2085.x * _e2087) + (_e815 * 0.020000001f));
                    let _e2098 = ((_e2085.y * _e2087) + (_e815 * 0.034f));
                    let _e2099 = ((_e2085.z * _e2087) + (_e815 * 0.05f));
                    if (_e815 > 0.0009765625f) {
                        let _e2104 = (_e1721 - (20f * _e997));
                        let _e2105 = (_e1722 - (110f * _e997));
                        let _e2108 = floor((_e2104 * 0.06666667f));
                        let _e2109 = floor((_e2105 * 0.04f));
                        let _e2111 = cantus_render_shader_hash(vec2<f32>(_e2108, _e2109));
                        let _e2122 = (_e2104 - (((_e2108 + 0.15f) + (_e2111.x * 0.7f)) * 15f));
                        let _e2123 = (_e2105 - (((_e2109 + 0.15f) + (_e2111.y * 0.7f)) * 25f));
                        let _e2127 = (((_e2122 * 1.8000001f) + (_e2123 * 9f)) * 0.011870845f);
                        let _e2129 = select(_e2127, 0f, (_e2127 < 0f));
                        let _e2131 = select(_e2129, 1f, (_e2129 > 1f));
                        let _e2134 = (_e2122 - (1.8000001f * _e2131));
                        let _e2135 = (_e2123 - (9f * _e2131));
                        let _e2141 = ((sqrt(((_e2134 * _e2134) + (_e2135 * _e2135))) - 1.0999999f) * -1.666667f);
                        let _e2143 = select(_e2141, 0f, (_e2141 < 0f));
                        let _e2145 = select(_e2143, 1f, (_e2143 > 1f));
                        let _e2153 = cantus_render_shader_hash(vec2<f32>((_e2108 + 19.3f), (_e2109 + 19.3f)));
                        let _e2156 = ((_e2153.x - 0.22000003f) * 1.2820513f);
                        let _e2158 = select(_e2156, 0f, (_e2156 < 0f));
                        let _e2160 = select(_e2158, 1f, (_e2158 > 1f));
                        let _e2167 = (((((_e2145 * _e2145) * (3f - (2f * _e2145))) * ((_e2160 * _e2160) * (3f - (2f * _e2160)))) * _e815) * 0.7f);
                        let _e2169 = select(_e2167, 0f, (_e2167 < 0f));
                        let _e2171 = select(_e2169, 1f, (_e2169 > 1f));
                        let _e2172 = (1f - _e2171);
                        phi_86_ = vec3<f32>(((_e2097 * _e2172) + (0.52f * _e2171)), ((_e2098 * _e2172) + (0.72f * _e2171)), ((_e2099 * _e2172) + (0.9f * _e2171)));
                    } else {
                        phi_86_ = vec3<f32>(_e2097, _e2098, _e2099);
                    }
                    let _e2184 = phi_86_;
                    if (_e818 > 0.0009765625f) {
                        let _e2188 = (_e1721 - (5f * _e997));
                        let _e2189 = (_e1722 - (14f * _e997));
                        let _e2192 = floor((_e2188 * 0.05f));
                        let _e2193 = floor((_e2189 * 0.05f));
                        let _e2197 = cantus_render_shader_hash(vec2<f32>((_e2192 + 31.7f), (_e2193 + 31.7f)));
                        let _e2208 = (_e2188 - (((_e2192 + 0.15f) + (_e2197.x * 0.7f)) * 20f));
                        let _e2209 = (_e2189 - (((_e2193 + 0.15f) + (_e2197.y * 0.7f)) * 20f));
                        let _e2213 = (((_e2208 * 0.080000006f) + (_e2209 * 0.4f)) * 6.009615f);
                        let _e2215 = select(_e2213, 0f, (_e2213 < 0f));
                        let _e2217 = select(_e2215, 1f, (_e2215 > 1f));
                        let _e2220 = (_e2208 - (0.080000006f * _e2217));
                        let _e2221 = (_e2209 - (0.4f * _e2217));
                        let _e2227 = ((sqrt(((_e2220 * _e2220) + (_e2221 * _e2221))) - 1.5999999f) * -1.666667f);
                        let _e2229 = select(_e2227, 0f, (_e2227 < 0f));
                        let _e2231 = select(_e2229, 1f, (_e2229 > 1f));
                        let _e2239 = cantus_render_shader_hash(vec2<f32>((_e2192 + 19.3f), (_e2193 + 19.3f)));
                        let _e2242 = ((_e2239.x - 0.3f) * 1.4285715f);
                        let _e2244 = select(_e2242, 0f, (_e2242 < 0f));
                        let _e2246 = select(_e2244, 1f, (_e2244 > 1f));
                        let _e2253 = (((((_e2231 * _e2231) * (3f - (2f * _e2231))) * ((_e2246 * _e2246) * (3f - (2f * _e2246)))) * _e818) * 0.92f);
                        let _e2255 = select(_e2253, 0f, (_e2253 < 0f));
                        let _e2257 = select(_e2255, 1f, (_e2255 > 1f));
                        let _e2258 = (1f - _e2257);
                        let _e2265 = (0.96f * _e2257);
                        phi_87_ = vec3<f32>(((_e2184.x * _e2258) + _e2265), ((_e2184.y * _e2258) + _e2265), ((_e2184.z * _e2258) + _e2265));
                    } else {
                        phi_87_ = _e2184;
                    }
                    let _e2271 = phi_87_;
                    if (_e824 > 0.0009765625f) {
                        let _e2275 = (_e1721 - (18f * _e997));
                        let _e2276 = (_e1722 - (85f * _e997));
                        let _e2279 = floor((_e2275 * 0.04347826f));
                        let _e2280 = floor((_e2276 * 0.04347826f));
                        let _e2284 = cantus_render_shader_hash(vec2<f32>((_e2279 + 63.4f), (_e2280 + 63.4f)));
                        let _e2295 = (_e2275 - (((_e2279 + 0.15f) + (_e2284.x * 0.7f)) * 23f));
                        let _e2296 = (_e2276 - (((_e2280 + 0.15f) + (_e2284.y * 0.7f)) * 23f));
                        let _e2300 = (((_e2295 * 0.24000001f) + (_e2296 * 1.2f)) * 0.667735f);
                        let _e2302 = select(_e2300, 0f, (_e2300 < 0f));
                        let _e2304 = select(_e2302, 1f, (_e2302 > 1f));
                        let _e2307 = (_e2295 - (0.24000001f * _e2304));
                        let _e2308 = (_e2296 - (1.2f * _e2304));
                        let _e2314 = ((sqrt(((_e2307 * _e2307) + (_e2308 * _e2308))) - 0.79999995f) * -1.6666667f);
                        let _e2316 = select(_e2314, 0f, (_e2314 < 0f));
                        let _e2318 = select(_e2316, 1f, (_e2316 > 1f));
                        let _e2326 = cantus_render_shader_hash(vec2<f32>((_e2279 + 19.3f), (_e2280 + 19.3f)));
                        let _e2329 = ((_e2326.x - 0.7f) * 3.3333333f);
                        let _e2331 = select(_e2329, 0f, (_e2329 < 0f));
                        let _e2333 = select(_e2331, 1f, (_e2331 > 1f));
                        let _e2340 = (((((_e2318 * _e2318) * (3f - (2f * _e2318))) * ((_e2333 * _e2333) * (3f - (2f * _e2333)))) * _e824) * 0.7f);
                        let _e2342 = select(_e2340, 0f, (_e2340 < 0f));
                        let _e2344 = select(_e2342, 1f, (_e2342 > 1f));
                        let _e2345 = (1f - _e2344);
                        phi_88_ = vec3<f32>(((_e2271.x * _e2345) + (0.75f * _e2344)), ((_e2271.y * _e2345) + (0.86f * _e2344)), ((_e2271.z * _e2345) + (0.94f * _e2344)));
                    } else {
                        phi_88_ = _e2271;
                    }
                    let _e2360 = phi_88_;
                    let _e2361 = (_e1613 * _e821);
                    let _e2363 = (1f - (_e2361 * 0.55f));
                    let _e2373 = ((_e2360.x * _e2363) + (_e2361 * 0.3575f));
                    let _e2374 = ((_e2360.y * _e2363) + (_e2361 * 0.407f));
                    let _e2375 = ((_e2360.z * _e2363) + (_e2361 * 0.528f));
                    if (_e809 > 0.0009765625f) {
                        phi_89_ = 0i;
                        phi_90_ = 0.5f;
                        phi_91_ = 0f;
                        phi_92_ = vec2<f32>(((_e847 * 0.9f) + (_e997 * 0.008f)), ((_e848 * 0.32f) + 12f));
                        loop {
                            let _e2385 = phi_89_;
                            let _e2387 = phi_90_;
                            let _e2389 = phi_91_;
                            let _e2391 = phi_92_;
                            local_75 = _e2389;
                            let _e2392 = (_e2385 < 4i);
                            if _e2392 {
                                let _e2395 = cantus_render_shader_simplex_noise(_e2391);
                                phi_93_ = (_e2385 + 1i);
                                phi_94_ = (_e2387 * 0.5f);
                                phi_95_ = (_e2389 + (_e2395 * _e2387));
                                phi_96_ = vec2<f32>(((_e2391.x * 1.6f) + (_e2391.y * 1.2f)), ((_e2391.y * 1.6f) - (_e2391.x * 1.2f)));
                            } else {
                                phi_93_ = i32();
                                phi_94_ = f32();
                                phi_95_ = f32();
                                phi_96_ = vec2<f32>();
                            }
                            let _e2408 = phi_93_;
                            let _e2410 = phi_94_;
                            let _e2412 = phi_95_;
                            let _e2414 = phi_96_;
                            continue;
                            continuing {
                                phi_89_ = _e2408;
                                phi_90_ = _e2410;
                                phi_91_ = _e2412;
                                phi_92_ = _e2414;
                                break if !(_e2392);
                            }
                        }
                        let _e2417 = local_75;
                        let _e2420 = (((_e2417 * 0.5f) + 0.15f) * 2.857143f);
                        let _e2422 = select(_e2420, 0f, (_e2420 < 0f));
                        let _e2424 = select(_e2422, 1f, (_e2422 > 1f));
                        let _e2431 = (_e809 * (0.58f + (((_e2424 * _e2424) * (3f - (2f * _e2424))) * 0.18f)));
                        let _e2432 = (1f - _e2431);
                        phi_97_ = vec3<f32>(((_e2373 * _e2432) + (0.63f * _e2431)), ((_e2374 * _e2432) + (0.69f * _e2431)), ((_e2375 * _e2432) + (0.73f * _e2431)));
                    } else {
                        phi_97_ = vec3<f32>(_e2373, _e2374, _e2375);
                    }
                    let _e2444 = phi_97_;
                    let _e2446 = ((_e536 - 5f) * -0.125f);
                    let _e2448 = select(_e2446, 0f, (_e2446 < 0f));
                    let _e2450 = select(_e2448, 1f, (_e2448 > 1f));
                    let _e2455 = (((_e2450 * _e2450) * (3f - (2f * _e2450))) * 0.14f);
                    let _e2462 = (1f - _e888);
                    phi_98_ = vec3<f32>((((_e2444.x + _e2455) * _e2462) + ((_e1700.x + _e1711) * _e888)), (((_e2444.y + _e2455) * _e2462) + ((_e1700.y + _e1711) * _e888)), (((_e2444.z + _e2455) * _e2462) + ((_e1700.z + _e1711) * _e888)));
                } else {
                    phi_98_ = _e1719;
                }
                let _e2474 = phi_98_;
                phi_99_ = _e2474;
            } else {
                phi_99_ = _e1719;
            }
            let _e2476 = phi_99_;
            if (_e235 < 1f) {
                let _e2479 = (16f + (_e212.x * 276f));
                let _e2481 = select(_e212.y, 0f, (_e212.y < 0f));
                let _e2485 = (0.72f - (select(_e2481, 1f, (_e2481 > 1f)) * 0.45f));
                let _e2488 = ((_e212.y - 0.55f) * -1.8867923f);
                let _e2490 = select(_e2488, 0f, (_e2488 < 0f));
                let _e2492 = select(_e2490, 1f, (_e2490 > 1f));
                let _e2496 = ((_e2492 * _e2492) * (3f - (2f * _e2492)));
                let _e2497 = (1f - _e2496);
                if (_e786 > 0.0009765625f) {
                    phi_100_ = 0i;
                    phi_101_ = 0.5f;
                    phi_102_ = 0f;
                    phi_103_ = vec2<f32>((((_e2479 / _e226) * 0.14f) + (_e997 * 0.012f)), ((_e2485 * 0.14f) + 6.1f));
                    loop {
                        let _e2515 = phi_100_;
                        let _e2517 = phi_101_;
                        let _e2519 = phi_102_;
                        let _e2521 = phi_103_;
                        local_76 = _e2519;
                        let _e2522 = (_e2515 < 4i);
                        if _e2522 {
                            let _e2525 = cantus_render_shader_simplex_noise(_e2521);
                            phi_104_ = (_e2515 + 1i);
                            phi_105_ = (_e2517 * 0.5f);
                            phi_106_ = (_e2519 + (_e2525 * _e2517));
                            phi_107_ = vec2<f32>(((_e2521.x * 1.6f) + (_e2521.y * 1.2f)), ((_e2521.y * 1.6f) - (_e2521.x * 1.2f)));
                        } else {
                            phi_104_ = i32();
                            phi_105_ = f32();
                            phi_106_ = f32();
                            phi_107_ = vec2<f32>();
                        }
                        let _e2538 = phi_104_;
                        let _e2540 = phi_105_;
                        let _e2542 = phi_106_;
                        let _e2544 = phi_107_;
                        continue;
                        continuing {
                            phi_100_ = _e2538;
                            phi_101_ = _e2540;
                            phi_102_ = _e2542;
                            phi_103_ = _e2544;
                            break if !(_e2522);
                        }
                    }
                    let _e2547 = local_76;
                    let _e2550 = (((_e2547 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e2552 = select(_e2550, 0f, (_e2550 < 0f));
                    let _e2554 = select(_e2552, 1f, (_e2552 > 1f));
                    phi_108_ = ((((_e2554 * _e2554) * (3f - (2f * _e2554))) * _e786) * 0.82f);
                } else {
                    phi_108_ = 0f;
                }
                let _e2562 = phi_108_;
                let _e2564 = ((_e212.y - -0.02f) * 16.666668f);
                let _e2566 = select(_e2564, 0f, (_e2564 < 0f));
                let _e2568 = select(_e2566, 1f, (_e2566 > 1f));
                let _e2575 = (_e227 - _e2479);
                let _e2576 = (_e228 - (_e226 * _e2485));
                let _e2580 = sqrt(((_e2575 * _e2575) + (_e2576 * _e2576)));
                let _e2582 = ((_e2580 - 62f) * -0.01724138f);
                let _e2584 = select(_e2582, 0f, (_e2582 < 0f));
                let _e2586 = select(_e2584, 1f, (_e2584 > 1f));
                let _e2593 = ((_e2580 - 11f) * -0.1f);
                let _e2595 = select(_e2593, 0f, (_e2593 < 0f));
                let _e2597 = select(_e2595, 1f, (_e2595 > 1f));
                let _e2604 = (((((_e2586 * _e2586) * (3f - (2f * _e2586))) * 0.24f) + (((_e2597 * _e2597) * (3f - (2f * _e2597))) * 0.7f)) * (((_e2568 * _e2568) * (3f - (2f * _e2568))) * (1f - _e2562)));
                let _e2605 = (1f - _e2604);
                let _e2621 = ((_e235 - 1f) / ((_e226 * -0.25f) - 1f));
                let _e2623 = select(_e2621, 0f, (_e2621 < 0f));
                let _e2625 = select(_e2623, 1f, (_e2623 > 1f));
                let _e2629 = ((_e2625 * _e2625) * (3f - (2f * _e2625)));
                let _e2630 = (1f - _e2629);
                phi_109_ = vec3<f32>(((_e2476.x * _e2630) + (((_e2476.x * _e2605) + (((0.96f * _e2497) + (0.98f * _e2496)) * _e2604)) * _e2629)), ((_e2476.y * _e2630) + (((_e2476.y * _e2605) + (((0.98f * _e2497) + (0.74f * _e2496)) * _e2604)) * _e2629)), ((_e2476.z * _e2630) + (((_e2476.z * _e2605) + ((_e2497 + (0.66f * _e2496)) * _e2604)) * _e2629)));
            } else {
                phi_109_ = _e2476;
            }
            let _e2642 = phi_109_;
            let _e2653 = local_77;
            let _e2654 = (1f - _e2653);
            let _e2659 = local_78;
            let _e2662 = local_79;
            let _e2665 = local_80;
            let _e2676 = floor(((_e211.x - (_e222 - 158f)) * 0.03846154f));
            let _e2677 = floor((_e228 / ((_e226 + 244f) * 0.027777778f)));
            let _e2679 = select(0f, _e2676, (_e2676 > 0f));
            let _e2681 = select(0f, _e2677, (_e2677 > 0f));
            let _e2687 = ((select(35f, _e2681, (_e2681 < 35f)) * 24f) + select(23f, _e2679, (_e2679 < 23f)));
            let _e2695 = text_cells.member[select(select(u32(_e2687), 0u, (_e2687 < 0f)), 4294967295u, (_e2687 > 4294967000f))];
            phi_110_ = vec3<f32>(((_e2642.x * _e2654) + (((_e2642.x * 1.5f) + 0.1f) * _e2659)), ((_e2642.y * _e2654) + (((_e2642.y * 1.5f) + 0.1f) * _e2662)), ((_e2642.z * _e2654) + (((_e2642.z * 1.5f) + 0.1f) * _e2665)));
            phi_111_ = 0i;
            loop {
                let _e2697 = phi_110_;
                let _e2699 = phi_111_;
                local_85 = _e2697;
                local_86 = _e2697;
                local_87 = _e2697;
                let _e2700 = (_e2699 < 2i);
                if _e2700 {
                    let _e2708 = text_lines.member[((_e2695 >> bitcast<u32>(((_e2699 * 16i) & 31i))) & 65535u)];
                    let _e2710 = unpack4x8unorm(_e2708.color);
                    let _e2712 = (1f / _e2708.size);
                    let _e2719 = ((_e211.x - _e2708.origin.x) * _e2712);
                    phi_112_ = 0u;
                    phi_113_ = _e2708.count;
                    loop {
                        let _e2724 = phi_112_;
                        let _e2726 = phi_113_;
                        local_81 = _e2724;
                        let _e2727 = (_e2724 < _e2726);
                        if _e2727 {
                            let _e2730 = (_e2724 + ((_e2726 - _e2724) / 2u));
                            let _e2735 = placed_glyphs.member[(_e2708.first + _e2730)].x;
                            let _e2736 = (_e2735 <= _e2719);
                            if _e2736 {
                                phi_114_ = (_e2730 + 1u);
                            } else {
                                phi_114_ = _e2724;
                            }
                            let _e2739 = phi_114_;
                            phi_115_ = _e2739;
                            phi_116_ = select(_e2730, _e2726, _e2736);
                        } else {
                            phi_115_ = u32();
                            phi_116_ = u32();
                        }
                        let _e2742 = phi_115_;
                        let _e2744 = phi_116_;
                        continue;
                        continuing {
                            phi_112_ = _e2742;
                            phi_113_ = _e2744;
                            break if !(_e2727);
                        }
                    }
                    let _e2746 = (3.5f / _e2708.size);
                    let _e2748 = local_81;
                    let _e2749 = (_e2748 + 1u);
                    phi_117_ = select(_e2749, _e2708.count, (_e2708.count < _e2749));
                    phi_118_ = -1000000f;
                    loop {
                        let _e2753 = phi_117_;
                        let _e2755 = phi_118_;
                        local_84 = _e2755;
                        if (_e2753 > 0u) {
                            let _e2757 = (_e2753 - 1u);
                            let _e2758 = (_e2708.first + _e2757);
                            let _e2762 = placed_glyphs.member[_e2758].x;
                            let _e2766 = placed_glyphs.member[_e2758].glyph;
                            let _e2771 = glyphs.member[_e2766].min[0u];
                            let _e2776 = glyphs.member[_e2766].min[1u];
                            let _e2781 = glyphs.member[_e2766].max[0u];
                            let _e2786 = glyphs.member[_e2766].max[1u];
                            let _e2790 = glyphs.member[_e2766].start;
                            let _e2794 = glyphs.member[_e2766].count;
                            let _e2795 = (_e2719 - _e2762);
                            let _e2796 = -(((_e211.y - _e2708.origin.y) * _e2712));
                            let _e2797 = (_e2781 + _e2746);
                            let _e2798 = (_e2795 > _e2797);
                            if _e2798 {
                                phi_131_ = f32();
                            } else {
                                if (_e2795 >= (_e2771 - _e2746)) {
                                    if (_e2796 >= (_e2776 - _e2746)) {
                                        if (_e2795 <= _e2797) {
                                            if (_e2796 <= (_e2786 + _e2746)) {
                                                phi_119_ = 340282350000000000000000000000000000000f;
                                                phi_120_ = 0u;
                                                phi_121_ = 0i;
                                                loop {
                                                    let _e2808 = phi_119_;
                                                    let _e2810 = phi_120_;
                                                    let _e2812 = phi_121_;
                                                    local_82 = _e2808;
                                                    local_83 = _e2812;
                                                    let _e2813 = (_e2810 < _e2794);
                                                    if _e2813 {
                                                        let _e2817 = edges.member[(_e2790 + _e2810)];
                                                        let _e2819 = cantus_render_text_edge_distance(_e2817, _e2708.weight, vec2<f32>(_e2795, _e2796), _e2808);
                                                        phi_122_ = _e2819.member;
                                                        phi_123_ = (_e2810 + 1u);
                                                        phi_124_ = (_e2812 + _e2819.member_1);
                                                    } else {
                                                        phi_122_ = f32();
                                                        phi_123_ = u32();
                                                        phi_124_ = i32();
                                                    }
                                                    let _e2825 = phi_122_;
                                                    let _e2827 = phi_123_;
                                                    let _e2829 = phi_124_;
                                                    continue;
                                                    continuing {
                                                        phi_119_ = _e2825;
                                                        phi_120_ = _e2827;
                                                        phi_121_ = _e2829;
                                                        break if !(_e2813);
                                                    }
                                                }
                                                let _e2832 = local_82;
                                                let _e2834 = ((_e2832 * _e2708.size) * _e2708.size);
                                                if (_e2834 >= 12.25f) {
                                                    phi_125_ = 3.5f;
                                                } else {
                                                    phi_125_ = sqrt(_e2834);
                                                }
                                                let _e2838 = phi_125_;
                                                let _e2840 = local_83;
                                                let _e2843 = (_e2838 * select(1f, -1f, (_e2840 == 0i)));
                                                if (_e2755 != _e2755) {
                                                    phi_126_ = true;
                                                } else {
                                                    phi_126_ = (_e2843 >= _e2755);
                                                }
                                                let _e2847 = phi_126_;
                                                phi_127_ = select(_e2755, _e2843, _e2847);
                                            } else {
                                                phi_127_ = _e2755;
                                            }
                                            let _e2850 = phi_127_;
                                            phi_128_ = _e2850;
                                        } else {
                                            phi_128_ = _e2755;
                                        }
                                        let _e2852 = phi_128_;
                                        phi_129_ = _e2852;
                                    } else {
                                        phi_129_ = _e2755;
                                    }
                                    let _e2854 = phi_129_;
                                    phi_130_ = _e2854;
                                } else {
                                    phi_130_ = _e2755;
                                }
                                let _e2856 = phi_130_;
                                phi_131_ = _e2856;
                            }
                            let _e2858 = phi_131_;
                            phi_132_ = _e2757;
                            phi_133_ = _e2858;
                            phi_134_ = select(true, false, _e2798);
                        } else {
                            phi_132_ = u32();
                            phi_133_ = f32();
                            phi_134_ = false;
                        }
                        let _e2861 = phi_132_;
                        let _e2863 = phi_133_;
                        let _e2865 = phi_134_;
                        continue;
                        continuing {
                            phi_117_ = _e2861;
                            phi_118_ = _e2863;
                            break if !(_e2865);
                        }
                    }
                    let _e2868 = local_84;
                    let _e2870 = ((_e2868 * 1.25f) + 0.5f);
                    let _e2872 = select(_e2870, 0f, (_e2870 < 0f));
                    let _e2874 = select(_e2872, 1f, (_e2872 > 1f));
                    let _e2880 = (((_e2874 * _e2874) * (3f - (2f * _e2874))) * _e2710.w);
                    let _e2881 = (1f - _e2880);
                    phi_135_ = vec3<f32>(((_e2697.x * _e2881) + (_e2710.x * _e2880)), ((_e2697.y * _e2881) + (_e2710.y * _e2880)), ((_e2697.z * _e2881) + (_e2710.z * _e2880)));
                    phi_136_ = (_e2699 + 1i);
                } else {
                    phi_135_ = vec3<f32>();
                    phi_136_ = i32();
                }
                let _e2900 = phi_135_;
                let _e2902 = phi_136_;
                continue;
                continuing {
                    phi_110_ = _e2900;
                    phi_111_ = _e2902;
                    break if !(_e2700);
                }
            }
            if _e384 {
                break;
            }
            let _e2905 = local_85;
            let _e2909 = local_86;
            let _e2913 = local_87;
            out_color = vec4<f32>((_e2905.x * _e554), (_e2909.y * _e554), (_e2913.z * _e554), _e567);
            break;
        }
    }
    return;
}

@vertex
fn render_track_isthmus_trackpass_vertex(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    vertex_7 = vertex;
    instance_2 = instance;
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
    vertex_7 = vertex_1;
    _isthmus_instance_index_9 = _isthmus_instance_index;
    render_lyrics_isthmus_lyricspass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_lyrics_isthmus_lyricspass_fragment(@location(0) pixel: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_1: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel;
    _isthmus_instance_index_10 = _isthmus_instance_index_1;
    render_lyrics_isthmus_lyricspass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_status_isthmus_statuspass_vertex(@builtin(vertex_index) vertex_2: u32, @builtin(instance_index) _isthmus_instance_index_2: u32) -> VertexOutput {
    vertex_7 = vertex_2;
    _isthmus_instance_index_9 = _isthmus_instance_index_2;
    render_status_isthmus_statuspass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_isthmus_instance_index;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_status_isthmus_statuspass_fragment(@location(0) pixel_1: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_3: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel_1;
    _isthmus_instance_index_10 = _isthmus_instance_index_3;
    render_status_isthmus_statuspass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_launcher_isthmus_launcherpass_vertex(@builtin(vertex_index) vertex_3: u32, @builtin(instance_index) instance_1: u32) -> VertexOutput {
    vertex_7 = vertex_3;
    instance_2 = instance_1;
    render_launcher_isthmus_launcherpass_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_row_idx;
    return VertexOutput(_e7, _e8, _e9);
}

@fragment
fn render_launcher_isthmus_launcherpass_fragment(@location(0) pixel_2: vec2<f32>, @location(1) @interpolate(flat) row_idx: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel_2;
    row_idx_1 = row_idx;
    render_launcher_isthmus_launcherpass_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_playhead_isthmus_playheadpass_vertex(@builtin(vertex_index) vertex_4: u32, @builtin(instance_index) _isthmus_instance_index_4: u32) -> VertexOutput {
    vertex_7 = vertex_4;
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
fn render_particles_isthmus_particlepass_vertex(@builtin(vertex_index) vertex_5: u32, @builtin(instance_index) _isthmus_instance_index_6: u32) -> VertexOutput_1 {
    vertex_7 = vertex_5;
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
fn render_tempestas_isthmus_tempestaspass_vertex(@builtin(vertex_index) vertex_6: u32, @builtin(instance_index) _isthmus_instance_index_7: u32) -> VertexOutput_2 {
    vertex_7 = vertex_6;
    _isthmus_instance_index_9 = _isthmus_instance_index_7;
    render_tempestas_isthmus_tempestaspass_vertex_impl();
    let _e8 = out_position;
    let _e9 = out_pixel;
    let _e10 = out_weather;
    let _e11 = out_isthmus_instance_index_1;
    return VertexOutput_2(_e8, _e9, _e10, _e11);
}

@fragment
fn render_tempestas_isthmus_tempestaspass_fragment(@location(0) pixel_3: vec2<f32>, @location(1) @interpolate(flat) weather: vec4<f32>, @location(2) @interpolate(flat) _isthmus_instance_index_8: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel_3;
    weather_1 = weather;
    _isthmus_instance_index_11 = _isthmus_instance_index_8;
    render_tempestas_isthmus_tempestaspass_fragment_impl();
    let _e7 = out_color;
    return _e7;
}
