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
    low_start: vec2<f32>,
    low_control: vec2<f32>,
    low_end: vec2<f32>,
    high_start: vec2<f32>,
    high_control: vec2<f32>,
    high_end: vec2<f32>,
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

struct render_tempestas_WeatherLine {
    line: render_text_Line,
    color: u32,
}

struct type_38 {
    member: array<render_tempestas_WeatherLine>,
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

fn cantus_render_text_quadratic(param_2: vec2<f32>, param_3: vec2<f32>, param_4: vec2<f32>, param_5: f32) -> vec2<f32> {
    let _e12 = (1f - param_5);
    let _e18 = ((2f * _e12) * param_5);
    return vec2<f32>(((((param_2.x * _e12) * _e12) + (param_3.x * _e18)) + ((param_4.x * param_5) * param_5)), ((((param_2.y * _e12) * _e12) + (param_3.y * _e18)) + ((param_4.y * param_5) * param_5)));
}

fn cantus_render_text_ray_crossing(param_6: vec2<f32>, param_7: vec2<f32>, param_8: vec2<f32>, param_9: vec2<f32>, param_10: f32) -> i32 {
    var phi_0_: i32;
    var phi_1_: i32;
    var phi_2_: i32;
    var phi_3_: i32;
    var phi_4_: bool;

    if (param_10 < 0f) {
        phi_3_ = i32();
        phi_4_ = true;
    } else {
        let _e18 = (param_10 >= 1f);
        if _e18 {
            phi_2_ = i32();
        } else {
            let _e19 = cantus_render_text_quadratic(param_6, param_7, param_8, param_10);
            if (_e19.x <= param_9.x) {
                phi_1_ = 0i;
            } else {
                let _e28 = ((((param_7.y - param_6.y) * (1f - param_10)) + ((param_8.y - param_7.y) * param_10)) * 2f);
                if (_e28 > 0f) {
                    phi_0_ = 1i;
                } else {
                    phi_0_ = select(0i, -1i, (_e28 < 0f));
                }
                let _e33 = phi_0_;
                phi_1_ = _e33;
            }
            let _e35 = phi_1_;
            phi_2_ = _e35;
        }
        let _e37 = phi_2_;
        phi_3_ = _e37;
        phi_4_ = _e18;
    }
    let _e39 = phi_3_;
    let _e41 = phi_4_;
    return select(_e39, 0i, _e41);
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

    let _e29 = (1f - param_12);
    let _e34 = ((param_11.low_start.x * _e29) + (param_11.high_start.x * param_12));
    let _e35 = ((param_11.low_start.y * _e29) + (param_11.high_start.y * param_12));
    let _e48 = ((param_11.low_control.x * _e29) + (param_11.high_control.x * param_12));
    let _e49 = ((param_11.low_control.y * _e29) + (param_11.high_control.y * param_12));
    let _e62 = ((param_11.low_end.x * _e29) + (param_11.high_end.x * param_12));
    let _e63 = ((param_11.low_end.y * _e29) + (param_11.high_end.y * param_12));
    let _e65 = select(_e48, _e34, (_e34 < _e48));
    let _e67 = select(_e49, _e35, (_e35 < _e49));
    let _e69 = select(_e62, _e65, (_e65 < _e62));
    let _e71 = select(_e63, _e67, (_e67 < _e63));
    let _e73 = select(_e48, _e34, (_e34 > _e48));
    let _e75 = select(_e49, _e35, (_e35 > _e49));
    let _e77 = select(_e62, _e73, (_e73 > _e62));
    let _e79 = select(_e63, _e75, (_e75 > _e63));
    if (param_13.x >= _e77) {
        phi_8_ = i32();
        phi_9_ = true;
    } else {
        if (param_13.y < _e71) {
            phi_6_ = i32();
            phi_7_ = true;
        } else {
            let _e82 = (param_13.y >= _e79);
            if _e82 {
                phi_5_ = i32();
            } else {
                let _e85 = ((_e35 - (_e49 * 2f)) + _e63);
                let _e86 = (_e49 - _e35);
                let _e87 = (_e86 * 2f);
                let _e88 = (_e35 - param_13.y);
                if (abs(_e85) < 0.0000001f) {
                    if (abs(_e87) < 0.0000001f) {
                        phi_1_ = 0i;
                    } else {
                        let _e118 = cantus_render_text_ray_crossing(vec2<f32>(_e34, _e35), vec2<f32>(_e48, _e49), vec2<f32>(_e62, _e63), param_13, (-(_e88) / _e87));
                        phi_1_ = _e118;
                    }
                    let _e120 = phi_1_;
                    phi_2_ = _e120;
                    phi_3_ = i32();
                    phi_4_ = true;
                } else {
                    let _e94 = ((_e87 * _e87) - ((4f * _e85) * _e88));
                    let _e95 = (_e94 <= 0f);
                    if _e95 {
                        phi_0_ = i32();
                    } else {
                        let _e96 = sqrt(_e94);
                        let _e97 = (_e85 * 2f);
                        let _e98 = (_e86 * -2f);
                        let _e101 = vec2<f32>(_e34, _e35);
                        let _e102 = vec2<f32>(_e48, _e49);
                        let _e103 = vec2<f32>(_e62, _e63);
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
            phi_7_ = _e82;
        }
        let _e131 = phi_6_;
        let _e133 = phi_7_;
        phi_8_ = _e131;
        phi_9_ = _e133;
    }
    let _e135 = phi_8_;
    let _e137 = phi_9_;
    let _e138 = select(_e135, 0i, _e137);
    let _e140 = select(_e69, param_13.x, (param_13.x > _e69));
    let _e142 = select(_e71, param_13.y, (param_13.y > _e71));
    let _e147 = (param_13.x - select(_e77, _e140, (_e140 < _e77)));
    let _e148 = (param_13.y - select(_e79, _e142, (_e142 < _e79)));
    if (((_e147 * _e147) + (_e148 * _e148)) >= param_14) {
        phi_22_ = u0028_f32_u0020_i32_u0029_(param_14, _e138);
    } else {
        let _e153 = (_e62 - _e34);
        let _e154 = (_e63 - _e35);
        let _e155 = (param_13.x - _e34);
        let _e156 = (param_13.y - _e35);
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
        phi_13_ = select(_e173, 1f, _e177);
        phi_14_ = 0i;
        loop {
            let _e188 = phi_13_;
            let _e190 = phi_14_;
            local_1 = _e188;
            let _e191 = (_e190 < 2i);
            if _e191 {
                let _e195 = cantus_render_text_quadratic(vec2<f32>(_e34, _e35), vec2<f32>(_e48, _e49), vec2<f32>(_e62, _e63), _e188);
                let _e200 = (1f - _e188);
                let _e209 = ((((_e48 - _e34) * _e200) + ((_e62 - _e48) * _e188)) * 2f);
                let _e210 = ((((_e49 - _e35) * _e200) + ((_e63 - _e49) * _e188)) * 2f);
                let _e211 = (_e195.x - param_13.x);
                let _e212 = (_e195.y - param_13.y);
                let _e219 = (((_e209 * _e209) + (_e210 * _e210)) + ((_e211 * (((_e34 - (_e48 * 2f)) + _e62) * 2f)) + (_e212 * (((_e35 - (_e49 * 2f)) + _e63) * 2f))));
                let _e220 = abs(_e219);
                if (_e220 != _e220) {
                    phi_15_ = true;
                } else {
                    phi_15_ = (0.00000001f >= _e220);
                }
                let _e224 = phi_15_;
                let _e236 = (_e188 - (((_e211 * _e209) + (_e212 * _e210)) / bitcast<f32>(((bitcast<u32>(select(_e220, 0.00000001f, _e224)) & 2147483647u) | (bitcast<u32>(_e219) & 2147483648u)))));
                if (_e236 != _e236) {
                    phi_16_ = true;
                } else {
                    phi_16_ = (0f >= _e236);
                }
                let _e240 = phi_16_;
                let _e241 = select(_e236, 0f, _e240);
                if (_e241 != _e241) {
                    phi_17_ = true;
                } else {
                    phi_17_ = (1f <= _e241);
                }
                let _e245 = phi_17_;
                phi_18_ = select(_e241, 1f, _e245);
                phi_19_ = (_e190 + 1i);
            } else {
                phi_18_ = f32();
                phi_19_ = i32();
            }
            let _e249 = phi_18_;
            let _e251 = phi_19_;
            continue;
            continuing {
                phi_13_ = _e249;
                phi_14_ = _e251;
                break if !(_e191);
            }
        }
        let _e255 = ((_e155 * _e155) + (_e156 * _e156));
        let _e256 = (param_13.x - _e62);
        let _e257 = (param_13.y - _e63);
        let _e260 = ((_e256 * _e256) + (_e257 * _e257));
        if (_e255 != _e255) {
            phi_20_ = true;
        } else {
            phi_20_ = (_e260 <= _e255);
        }
        let _e264 = phi_20_;
        let _e265 = select(_e255, _e260, _e264);
        let _e270 = local_1;
        let _e271 = cantus_render_text_quadratic(vec2<f32>(_e34, _e35), vec2<f32>(_e48, _e49), vec2<f32>(_e62, _e63), _e270);
        let _e274 = (param_13.x - _e271.x);
        let _e275 = (param_13.y - _e271.y);
        let _e278 = ((_e274 * _e274) + (_e275 * _e275));
        if (_e265 != _e265) {
            phi_21_ = true;
        } else {
            phi_21_ = (_e278 <= _e265);
        }
        let _e282 = phi_21_;
        phi_22_ = u0028_f32_u0020_i32_u0029_(select(_e265, _e278, _e282), _e138);
    }
    let _e287 = phi_22_;
    return _e287;
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
                    let _e2246 = local_22;
                    phi_45_ = _e2246;
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
                            phi_63_ = _e1688.count;
                            phi_64_ = 0u;
                            loop {
                                let _e1703 = phi_63_;
                                let _e1705 = phi_64_;
                                local_10 = _e1705;
                                let _e1706 = (_e1705 < _e1703);
                                if _e1706 {
                                    let _e1709 = (_e1705 + ((_e1703 - _e1705) / 2u));
                                    let _e1711 = (_e1688.first + _e1709);
                                    if (_e1711 < _e168) {
                                    } else {
                                        phi_68_ = true;
                                        break;
                                    }
                                    let _e1716 = placed_glyphs.member[_e1711].x;
                                    let _e1722 = (_e1716 <= ((_e624 - _e1688.origin.x) / _e1688.size));
                                    if _e1722 {
                                        phi_65_ = (_e1709 + 1u);
                                    } else {
                                        phi_65_ = _e1705;
                                    }
                                    let _e1725 = phi_65_;
                                    phi_66_ = select(_e1709, _e1703, _e1722);
                                    phi_67_ = _e1725;
                                } else {
                                    phi_66_ = u32();
                                    phi_67_ = u32();
                                }
                                let _e1728 = phi_66_;
                                let _e1730 = phi_67_;
                                continue;
                                continuing {
                                    phi_63_ = _e1728;
                                    phi_64_ = _e1730;
                                    phi_68_ = _e1683;
                                    break if !(_e1706);
                                }
                            }
                            let _e1733 = phi_68_;
                            if _e1733 {
                                break;
                            }
                            let _e1735 = local_10;
                            let _e1736 = (_e1735 + 1u);
                            phi_69_ = _e1733;
                            phi_70_ = select(_e1736, _e1688.count, (_e1688.count < _e1736));
                            phi_71_ = -1000000f;
                            loop {
                                let _e1740 = phi_69_;
                                let _e1742 = phi_70_;
                                let _e1744 = phi_71_;
                                local_13 = _e1744;
                                if (_e1742 > 0u) {
                                    let _e1746 = (_e1742 - 1u);
                                    let _e1748 = (_e1688.first + _e1746);
                                    if (_e1748 < _e168) {
                                    } else {
                                        phi_94_ = true;
                                        break;
                                    }
                                    let _e1753 = placed_glyphs.member[_e1748].x;
                                    let _e1757 = placed_glyphs.member[_e1748].glyph;
                                    if (_e1757 < _e170) {
                                    } else {
                                        phi_94_ = true;
                                        break;
                                    }
                                    let _e1763 = glyphs.member[_e1757].min[0u];
                                    let _e1768 = glyphs.member[_e1757].min[1u];
                                    let _e1773 = glyphs.member[_e1757].max[0u];
                                    let _e1778 = glyphs.member[_e1757].max[1u];
                                    let _e1782 = glyphs.member[_e1757].start;
                                    let _e1786 = glyphs.member[_e1757].count;
                                    let _e1792 = (((_e624 - _e1688.origin.x) / _e1688.size) - _e1753);
                                    let _e1797 = (-((_e625 - _e1688.origin.y)) / _e1688.size);
                                    let _e1798 = (3.5f / _e1688.size);
                                    let _e1799 = (_e1773 + _e1798);
                                    let _e1800 = (_e1792 > _e1799);
                                    if _e1800 {
                                        phi_88_ = _e1740;
                                        phi_89_ = f32();
                                    } else {
                                        if (_e1792 >= (_e1763 - _e1798)) {
                                            if (_e1797 >= (_e1768 - _e1798)) {
                                                if (_e1792 <= _e1799) {
                                                    if (_e1797 <= (_e1778 + _e1798)) {
                                                        phi_72_ = 340282350000000000000000000000000000000f;
                                                        phi_73_ = 0u;
                                                        phi_74_ = 0i;
                                                        loop {
                                                            let _e1810 = phi_72_;
                                                            let _e1812 = phi_73_;
                                                            let _e1814 = phi_74_;
                                                            local_11 = _e1810;
                                                            local_12 = _e1814;
                                                            let _e1815 = (_e1812 < _e1786);
                                                            if _e1815 {
                                                                let _e1816 = (_e1782 + _e1812);
                                                                if (_e1816 < _e172) {
                                                                } else {
                                                                    phi_78_ = true;
                                                                    break;
                                                                }
                                                                let _e1820 = edges.member[_e1816];
                                                                let _e1822 = cantus_render_text_edge_distance(_e1820, _e1688.weight, vec2<f32>(_e1792, _e1797), _e1810);
                                                                phi_75_ = _e1822.member;
                                                                phi_76_ = (_e1812 + 1u);
                                                                phi_77_ = (_e1814 + _e1822.member_1);
                                                            } else {
                                                                phi_75_ = f32();
                                                                phi_76_ = u32();
                                                                phi_77_ = i32();
                                                            }
                                                            let _e1828 = phi_75_;
                                                            let _e1830 = phi_76_;
                                                            let _e1832 = phi_77_;
                                                            continue;
                                                            continuing {
                                                                phi_72_ = _e1828;
                                                                phi_73_ = _e1830;
                                                                phi_74_ = _e1832;
                                                                phi_78_ = _e1740;
                                                                break if !(_e1815);
                                                            }
                                                        }
                                                        let _e1835 = phi_78_;
                                                        phi_94_ = _e1835;
                                                        if _e1835 {
                                                            break;
                                                        }
                                                        let _e1837 = local_11;
                                                        let _e1841 = local_12;
                                                        let _e1844 = ((sqrt(_e1837) * _e1688.size) * select(1f, -1f, (_e1841 == 0i)));
                                                        if (_e1744 != _e1744) {
                                                            phi_79_ = true;
                                                        } else {
                                                            phi_79_ = (_e1844 >= _e1744);
                                                        }
                                                        let _e1848 = phi_79_;
                                                        phi_80_ = _e1835;
                                                        phi_81_ = select(_e1744, _e1844, _e1848);
                                                    } else {
                                                        phi_80_ = _e1740;
                                                        phi_81_ = _e1744;
                                                    }
                                                    let _e1851 = phi_80_;
                                                    let _e1853 = phi_81_;
                                                    phi_82_ = _e1851;
                                                    phi_83_ = _e1853;
                                                } else {
                                                    phi_82_ = _e1740;
                                                    phi_83_ = _e1744;
                                                }
                                                let _e1855 = phi_82_;
                                                let _e1857 = phi_83_;
                                                phi_84_ = _e1855;
                                                phi_85_ = _e1857;
                                            } else {
                                                phi_84_ = _e1740;
                                                phi_85_ = _e1744;
                                            }
                                            let _e1859 = phi_84_;
                                            let _e1861 = phi_85_;
                                            phi_86_ = _e1859;
                                            phi_87_ = _e1861;
                                        } else {
                                            phi_86_ = _e1740;
                                            phi_87_ = _e1744;
                                        }
                                        let _e1863 = phi_86_;
                                        let _e1865 = phi_87_;
                                        phi_88_ = _e1863;
                                        phi_89_ = _e1865;
                                    }
                                    let _e1867 = phi_88_;
                                    let _e1869 = phi_89_;
                                    phi_90_ = _e1867;
                                    phi_91_ = _e1746;
                                    phi_92_ = _e1869;
                                    phi_93_ = select(true, false, _e1800);
                                } else {
                                    phi_90_ = _e1740;
                                    phi_91_ = u32();
                                    phi_92_ = f32();
                                    phi_93_ = false;
                                }
                                let _e1872 = phi_90_;
                                let _e1874 = phi_91_;
                                let _e1876 = phi_92_;
                                let _e1878 = phi_93_;
                                continue;
                                continuing {
                                    phi_69_ = _e1872;
                                    phi_70_ = _e1874;
                                    phi_71_ = _e1876;
                                    phi_94_ = _e1872;
                                    break if !(_e1878);
                                }
                            }
                            let _e1881 = phi_94_;
                            if _e1881 {
                                break;
                            }
                            let _e1883 = local_13;
                            let _e1885 = ((_e1883 * 1.25f) + 0.5f);
                            let _e1887 = select(_e1885, 0f, (_e1885 < 0f));
                            let _e1889 = select(_e1887, 1f, (_e1887 > 1f));
                            phi_95_ = _e1881;
                            phi_96_ = ((_e1889 * _e1889) * (3f - (2f * _e1889)));
                        }
                        let _e1895 = phi_95_;
                        let _e1897 = phi_96_;
                        phi_97_ = _e1895;
                        phi_98_ = _e1897;
                        phi_99_ = _e1700;
                    }
                    let _e1899 = phi_97_;
                    let _e1901 = phi_98_;
                    let _e1903 = phi_99_;
                    phi_100_ = _e1899;
                    phi_101_ = _e1901;
                    phi_102_ = _e1903;
                }
                let _e1905 = phi_100_;
                let _e1907 = phi_101_;
                let _e1909 = phi_102_;
                phi_103_ = _e1905;
                phi_104_ = _e1907;
                phi_105_ = _e1909;
            }
            let _e1911 = phi_103_;
            let _e1913 = phi_104_;
            let _e1915 = phi_105_;
            let _e1916 = select(_e1913, 0f, _e1915);
            let _e1921 = pill.member[_e166].lines[1u];
            if (_e624 < _e1921.min.x) {
                phi_143_ = f32();
                phi_144_ = true;
            } else {
                if (_e624 > _e1921.max.x) {
                    phi_141_ = f32();
                    phi_142_ = true;
                } else {
                    if (_e625 < _e1921.min.y) {
                        phi_139_ = f32();
                        phi_140_ = true;
                    } else {
                        let _e1933 = (_e625 > _e1921.max.y);
                        if _e1933 {
                            phi_138_ = f32();
                        } else {
                            phi_106_ = _e1921.count;
                            phi_107_ = 0u;
                            loop {
                                let _e1936 = phi_106_;
                                let _e1938 = phi_107_;
                                local_14 = _e1938;
                                let _e1939 = (_e1938 < _e1936);
                                if _e1939 {
                                    let _e1942 = (_e1938 + ((_e1936 - _e1938) / 2u));
                                    let _e1944 = (_e1921.first + _e1942);
                                    if (_e1944 < _e168) {
                                    } else {
                                        phi_111_ = true;
                                        break;
                                    }
                                    let _e1949 = placed_glyphs.member[_e1944].x;
                                    let _e1955 = (_e1949 <= ((_e624 - _e1921.origin.x) / _e1921.size));
                                    if _e1955 {
                                        phi_108_ = (_e1942 + 1u);
                                    } else {
                                        phi_108_ = _e1938;
                                    }
                                    let _e1958 = phi_108_;
                                    phi_109_ = select(_e1942, _e1936, _e1955);
                                    phi_110_ = _e1958;
                                } else {
                                    phi_109_ = u32();
                                    phi_110_ = u32();
                                }
                                let _e1961 = phi_109_;
                                let _e1963 = phi_110_;
                                continue;
                                continuing {
                                    phi_106_ = _e1961;
                                    phi_107_ = _e1963;
                                    phi_111_ = _e1911;
                                    break if !(_e1939);
                                }
                            }
                            let _e1966 = phi_111_;
                            if _e1966 {
                                break;
                            }
                            let _e1968 = local_14;
                            let _e1969 = (_e1968 + 1u);
                            phi_112_ = _e1966;
                            phi_113_ = select(_e1969, _e1921.count, (_e1921.count < _e1969));
                            phi_114_ = -1000000f;
                            loop {
                                let _e1973 = phi_112_;
                                let _e1975 = phi_113_;
                                let _e1977 = phi_114_;
                                local_17 = _e1977;
                                if (_e1975 > 0u) {
                                    let _e1979 = (_e1975 - 1u);
                                    let _e1981 = (_e1921.first + _e1979);
                                    if (_e1981 < _e168) {
                                    } else {
                                        phi_137_ = true;
                                        break;
                                    }
                                    let _e1986 = placed_glyphs.member[_e1981].x;
                                    let _e1990 = placed_glyphs.member[_e1981].glyph;
                                    if (_e1990 < _e170) {
                                    } else {
                                        phi_137_ = true;
                                        break;
                                    }
                                    let _e1996 = glyphs.member[_e1990].min[0u];
                                    let _e2001 = glyphs.member[_e1990].min[1u];
                                    let _e2006 = glyphs.member[_e1990].max[0u];
                                    let _e2011 = glyphs.member[_e1990].max[1u];
                                    let _e2015 = glyphs.member[_e1990].start;
                                    let _e2019 = glyphs.member[_e1990].count;
                                    let _e2025 = (((_e624 - _e1921.origin.x) / _e1921.size) - _e1986);
                                    let _e2030 = (-((_e625 - _e1921.origin.y)) / _e1921.size);
                                    let _e2031 = (3.5f / _e1921.size);
                                    let _e2032 = (_e2006 + _e2031);
                                    let _e2033 = (_e2025 > _e2032);
                                    if _e2033 {
                                        phi_131_ = _e1973;
                                        phi_132_ = f32();
                                    } else {
                                        if (_e2025 >= (_e1996 - _e2031)) {
                                            if (_e2030 >= (_e2001 - _e2031)) {
                                                if (_e2025 <= _e2032) {
                                                    if (_e2030 <= (_e2011 + _e2031)) {
                                                        phi_115_ = 340282350000000000000000000000000000000f;
                                                        phi_116_ = 0u;
                                                        phi_117_ = 0i;
                                                        loop {
                                                            let _e2043 = phi_115_;
                                                            let _e2045 = phi_116_;
                                                            let _e2047 = phi_117_;
                                                            local_15 = _e2043;
                                                            local_16 = _e2047;
                                                            let _e2048 = (_e2045 < _e2019);
                                                            if _e2048 {
                                                                let _e2049 = (_e2015 + _e2045);
                                                                if (_e2049 < _e172) {
                                                                } else {
                                                                    phi_121_ = true;
                                                                    break;
                                                                }
                                                                let _e2053 = edges.member[_e2049];
                                                                let _e2055 = cantus_render_text_edge_distance(_e2053, _e1921.weight, vec2<f32>(_e2025, _e2030), _e2043);
                                                                phi_118_ = _e2055.member;
                                                                phi_119_ = (_e2045 + 1u);
                                                                phi_120_ = (_e2047 + _e2055.member_1);
                                                            } else {
                                                                phi_118_ = f32();
                                                                phi_119_ = u32();
                                                                phi_120_ = i32();
                                                            }
                                                            let _e2061 = phi_118_;
                                                            let _e2063 = phi_119_;
                                                            let _e2065 = phi_120_;
                                                            continue;
                                                            continuing {
                                                                phi_115_ = _e2061;
                                                                phi_116_ = _e2063;
                                                                phi_117_ = _e2065;
                                                                phi_121_ = _e1973;
                                                                break if !(_e2048);
                                                            }
                                                        }
                                                        let _e2068 = phi_121_;
                                                        phi_137_ = _e2068;
                                                        if _e2068 {
                                                            break;
                                                        }
                                                        let _e2070 = local_15;
                                                        let _e2074 = local_16;
                                                        let _e2077 = ((sqrt(_e2070) * _e1921.size) * select(1f, -1f, (_e2074 == 0i)));
                                                        if (_e1977 != _e1977) {
                                                            phi_122_ = true;
                                                        } else {
                                                            phi_122_ = (_e2077 >= _e1977);
                                                        }
                                                        let _e2081 = phi_122_;
                                                        phi_123_ = _e2068;
                                                        phi_124_ = select(_e1977, _e2077, _e2081);
                                                    } else {
                                                        phi_123_ = _e1973;
                                                        phi_124_ = _e1977;
                                                    }
                                                    let _e2084 = phi_123_;
                                                    let _e2086 = phi_124_;
                                                    phi_125_ = _e2084;
                                                    phi_126_ = _e2086;
                                                } else {
                                                    phi_125_ = _e1973;
                                                    phi_126_ = _e1977;
                                                }
                                                let _e2088 = phi_125_;
                                                let _e2090 = phi_126_;
                                                phi_127_ = _e2088;
                                                phi_128_ = _e2090;
                                            } else {
                                                phi_127_ = _e1973;
                                                phi_128_ = _e1977;
                                            }
                                            let _e2092 = phi_127_;
                                            let _e2094 = phi_128_;
                                            phi_129_ = _e2092;
                                            phi_130_ = _e2094;
                                        } else {
                                            phi_129_ = _e1973;
                                            phi_130_ = _e1977;
                                        }
                                        let _e2096 = phi_129_;
                                        let _e2098 = phi_130_;
                                        phi_131_ = _e2096;
                                        phi_132_ = _e2098;
                                    }
                                    let _e2100 = phi_131_;
                                    let _e2102 = phi_132_;
                                    phi_133_ = _e2100;
                                    phi_134_ = _e1979;
                                    phi_135_ = _e2102;
                                    phi_136_ = select(true, false, _e2033);
                                } else {
                                    phi_133_ = _e1973;
                                    phi_134_ = u32();
                                    phi_135_ = f32();
                                    phi_136_ = false;
                                }
                                let _e2105 = phi_133_;
                                let _e2107 = phi_134_;
                                let _e2109 = phi_135_;
                                let _e2111 = phi_136_;
                                continue;
                                continuing {
                                    phi_112_ = _e2105;
                                    phi_113_ = _e2107;
                                    phi_114_ = _e2109;
                                    phi_137_ = _e2105;
                                    break if !(_e2111);
                                }
                            }
                            let _e2114 = phi_137_;
                            if _e2114 {
                                break;
                            }
                            let _e2116 = local_17;
                            let _e2118 = ((_e2116 * 1.25f) + 0.5f);
                            let _e2120 = select(_e2118, 0f, (_e2118 < 0f));
                            let _e2122 = select(_e2120, 1f, (_e2120 > 1f));
                            phi_138_ = ((_e2122 * _e2122) * (3f - (2f * _e2122)));
                        }
                        let _e2128 = phi_138_;
                        phi_139_ = _e2128;
                        phi_140_ = _e1933;
                    }
                    let _e2130 = phi_139_;
                    let _e2132 = phi_140_;
                    phi_141_ = _e2130;
                    phi_142_ = _e2132;
                }
                let _e2134 = phi_141_;
                let _e2136 = phi_142_;
                phi_143_ = _e2134;
                phi_144_ = _e2136;
            }
            let _e2138 = phi_143_;
            let _e2140 = phi_144_;
            let _e2141 = select(_e2138, 0f, _e2140);
            if (_e1916 != _e1916) {
                phi_145_ = true;
            } else {
                phi_145_ = (_e2141 >= _e1916);
            }
            let _e2145 = phi_145_;
            let _e2150 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e624 - _e1128), (_e625 - _e190)), 0f, _e190);
            let _e2152 = ((_e2150 - 2f) * 0.0625f);
            let _e2154 = select(_e2152, 0f, (_e2152 < 0f));
            let _e2156 = select(_e2154, 1f, (_e2154 > 1f));
            let _e2162 = ((select(_e1916, _e2141, _e2145) * ((_e2156 * _e2156) * (3f - (2f * _e2156)))) * _e584);
            let _e2163 = (1f - _e2162);
            let _e2165 = local_18;
            let _e2169 = local_19;
            let _e2173 = local_20;
            let _e2177 = local_21;
            let _e2180 = (0.94f * _e2162);
            let _e2188 = (((_e2177.w * _e2163) + _e2162) * _e601);
            if (_e2188 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e2165.x * _e2163) + _e2180) * _e601), (((_e2169.y * _e2163) + _e2180) * _e601), (((_e2173.z * _e2163) + _e2180) * _e601), _e2188);
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
                    phi_83_ = render_text_Line(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), 0f, 0f, 0u, 0u);
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
                                phi_85_ = _e2609.count;
                                phi_86_ = 0u;
                                loop {
                                    let _e2627 = phi_85_;
                                    let _e2629 = phi_86_;
                                    local_32 = _e2629;
                                    let _e2630 = (_e2629 < _e2627);
                                    if _e2630 {
                                        let _e2633 = (_e2629 + ((_e2627 - _e2629) / 2u));
                                        let _e2635 = (_e2609.first + _e2633);
                                        if (_e2635 < _e233) {
                                        } else {
                                            phi_90_ = true;
                                            break;
                                        }
                                        let _e2640 = placed_glyphs_1.member[_e2635].x;
                                        let _e2646 = (_e2640 <= ((_e720 - _e2609.origin.x) / _e2609.size));
                                        if _e2646 {
                                            phi_87_ = (_e2633 + 1u);
                                        } else {
                                            phi_87_ = _e2629;
                                        }
                                        let _e2649 = phi_87_;
                                        phi_88_ = select(_e2633, _e2627, _e2646);
                                        phi_89_ = _e2649;
                                    } else {
                                        phi_88_ = u32();
                                        phi_89_ = u32();
                                    }
                                    let _e2652 = phi_88_;
                                    let _e2654 = phi_89_;
                                    continue;
                                    continuing {
                                        phi_85_ = _e2652;
                                        phi_86_ = _e2654;
                                        phi_90_ = _e2298;
                                        break if !(_e2630);
                                    }
                                }
                                let _e2657 = phi_90_;
                                if _e2657 {
                                    break;
                                }
                                let _e2659 = local_32;
                                let _e2660 = (_e2659 + 1u);
                                phi_91_ = _e2657;
                                phi_92_ = select(_e2660, _e2609.count, (_e2609.count < _e2660));
                                phi_93_ = -1000000f;
                                loop {
                                    let _e2664 = phi_91_;
                                    let _e2666 = phi_92_;
                                    let _e2668 = phi_93_;
                                    local_35 = _e2668;
                                    if (_e2666 > 0u) {
                                        let _e2670 = (_e2666 - 1u);
                                        let _e2672 = (_e2609.first + _e2670);
                                        if (_e2672 < _e233) {
                                        } else {
                                            phi_116_ = true;
                                            break;
                                        }
                                        let _e2677 = placed_glyphs_1.member[_e2672].x;
                                        let _e2681 = placed_glyphs_1.member[_e2672].glyph;
                                        if (_e2681 < arrayLength((&glyphs_1.member))) {
                                        } else {
                                            phi_116_ = true;
                                            break;
                                        }
                                        let _e2687 = glyphs_1.member[_e2681].min[0u];
                                        let _e2692 = glyphs_1.member[_e2681].min[1u];
                                        let _e2697 = glyphs_1.member[_e2681].max[0u];
                                        let _e2702 = glyphs_1.member[_e2681].max[1u];
                                        let _e2706 = glyphs_1.member[_e2681].start;
                                        let _e2710 = glyphs_1.member[_e2681].count;
                                        let _e2716 = (((_e720 - _e2609.origin.x) / _e2609.size) - _e2677);
                                        let _e2721 = (-((_e721 - _e2609.origin.y)) / _e2609.size);
                                        let _e2722 = (3.5f / _e2609.size);
                                        let _e2723 = (_e2697 + _e2722);
                                        let _e2724 = (_e2716 > _e2723);
                                        if _e2724 {
                                            phi_110_ = _e2664;
                                            phi_111_ = f32();
                                        } else {
                                            if (_e2716 >= (_e2687 - _e2722)) {
                                                if (_e2721 >= (_e2692 - _e2722)) {
                                                    if (_e2716 <= _e2723) {
                                                        if (_e2721 <= (_e2702 + _e2722)) {
                                                            phi_94_ = 340282350000000000000000000000000000000f;
                                                            phi_95_ = 0u;
                                                            phi_96_ = 0i;
                                                            loop {
                                                                let _e2734 = phi_94_;
                                                                let _e2736 = phi_95_;
                                                                let _e2738 = phi_96_;
                                                                local_33 = _e2734;
                                                                local_34 = _e2738;
                                                                let _e2739 = (_e2736 < _e2710);
                                                                if _e2739 {
                                                                    let _e2740 = (_e2706 + _e2736);
                                                                    if (_e2740 < arrayLength((&edges_1.member))) {
                                                                    } else {
                                                                        phi_100_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2744 = edges_1.member[_e2740];
                                                                    let _e2746 = cantus_render_text_edge_distance(_e2744, _e2609.weight, vec2<f32>(_e2716, _e2721), _e2734);
                                                                    phi_97_ = _e2746.member;
                                                                    phi_98_ = (_e2736 + 1u);
                                                                    phi_99_ = (_e2738 + _e2746.member_1);
                                                                } else {
                                                                    phi_97_ = f32();
                                                                    phi_98_ = u32();
                                                                    phi_99_ = i32();
                                                                }
                                                                let _e2752 = phi_97_;
                                                                let _e2754 = phi_98_;
                                                                let _e2756 = phi_99_;
                                                                continue;
                                                                continuing {
                                                                    phi_94_ = _e2752;
                                                                    phi_95_ = _e2754;
                                                                    phi_96_ = _e2756;
                                                                    phi_100_ = _e2664;
                                                                    break if !(_e2739);
                                                                }
                                                            }
                                                            let _e2759 = phi_100_;
                                                            phi_116_ = _e2759;
                                                            if _e2759 {
                                                                break;
                                                            }
                                                            let _e2761 = local_33;
                                                            let _e2765 = local_34;
                                                            let _e2768 = ((sqrt(_e2761) * _e2609.size) * select(1f, -1f, (_e2765 == 0i)));
                                                            if (_e2668 != _e2668) {
                                                                phi_101_ = true;
                                                            } else {
                                                                phi_101_ = (_e2768 >= _e2668);
                                                            }
                                                            let _e2772 = phi_101_;
                                                            phi_102_ = _e2759;
                                                            phi_103_ = select(_e2668, _e2768, _e2772);
                                                        } else {
                                                            phi_102_ = _e2664;
                                                            phi_103_ = _e2668;
                                                        }
                                                        let _e2775 = phi_102_;
                                                        let _e2777 = phi_103_;
                                                        phi_104_ = _e2775;
                                                        phi_105_ = _e2777;
                                                    } else {
                                                        phi_104_ = _e2664;
                                                        phi_105_ = _e2668;
                                                    }
                                                    let _e2779 = phi_104_;
                                                    let _e2781 = phi_105_;
                                                    phi_106_ = _e2779;
                                                    phi_107_ = _e2781;
                                                } else {
                                                    phi_106_ = _e2664;
                                                    phi_107_ = _e2668;
                                                }
                                                let _e2783 = phi_106_;
                                                let _e2785 = phi_107_;
                                                phi_108_ = _e2783;
                                                phi_109_ = _e2785;
                                            } else {
                                                phi_108_ = _e2664;
                                                phi_109_ = _e2668;
                                            }
                                            let _e2787 = phi_108_;
                                            let _e2789 = phi_109_;
                                            phi_110_ = _e2787;
                                            phi_111_ = _e2789;
                                        }
                                        let _e2791 = phi_110_;
                                        let _e2793 = phi_111_;
                                        phi_112_ = _e2791;
                                        phi_113_ = _e2670;
                                        phi_114_ = _e2793;
                                        phi_115_ = select(true, false, _e2724);
                                    } else {
                                        phi_112_ = _e2664;
                                        phi_113_ = u32();
                                        phi_114_ = f32();
                                        phi_115_ = false;
                                    }
                                    let _e2796 = phi_112_;
                                    let _e2798 = phi_113_;
                                    let _e2800 = phi_114_;
                                    let _e2802 = phi_115_;
                                    continue;
                                    continuing {
                                        phi_91_ = _e2796;
                                        phi_92_ = _e2798;
                                        phi_93_ = _e2800;
                                        phi_116_ = _e2796;
                                        break if !(_e2802);
                                    }
                                }
                                let _e2805 = phi_116_;
                                if _e2805 {
                                    break;
                                }
                                let _e2807 = local_35;
                                let _e2809 = ((_e2807 * 1.25f) + 0.5f);
                                let _e2811 = select(_e2809, 0f, (_e2809 < 0f));
                                let _e2813 = select(_e2811, 1f, (_e2811 > 1f));
                                phi_117_ = ((_e2813 * _e2813) * (3f - (2f * _e2813)));
                            }
                            let _e2819 = phi_117_;
                            phi_118_ = _e2819;
                            phi_119_ = _e2624;
                        }
                        let _e2821 = phi_118_;
                        let _e2823 = phi_119_;
                        phi_120_ = _e2821;
                        phi_121_ = _e2823;
                    }
                    let _e2825 = phi_120_;
                    let _e2827 = phi_121_;
                    phi_122_ = _e2825;
                    phi_123_ = _e2827;
                }
                let _e2829 = phi_122_;
                let _e2831 = phi_123_;
                phi_124_ = select(_e2829, 0f, _e2831);
            } else {
                phi_124_ = 0f;
            }
            let _e2834 = phi_124_;
            let _e2835 = (1f - _e2834);
            let _e2839 = (0.94f * _e2834);
            out_color = vec4<f32>((((((_e2580.x * _e2584) + _e2593) * _e2835) + _e2839) * _e500), (((((_e2580.y * _e2584) + _e2593) * _e2835) + _e2839) * _e500), (((((_e2580.z * _e2584) + _e2593) * _e2835) + _e2839) * _e500), _e513);
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
    var phi_23_: f32;
    var phi_24_: f32;
    var phi_25_: bool;
    var phi_26_: f32;
    var phi_27_: f32;
    var phi_28_: render_tempestas_WeatherCondition;
    var phi_29_: render_tempestas_WeatherCondition;
    var phi_30_: array<f32, 2>;
    var phi_31_: array<f32, 2>;
    var phi_32_: bool;
    var phi_33_: f32;
    var phi_34_: array<f32, 2>;
    var phi_35_: bool;
    var phi_36_: bool;
    var phi_37_: bool;
    var phi_38_: bool;
    var phi_39_: bool;
    var phi_40_: bool;
    var phi_41_: vec2<f32>;
    var phi_42_: bool;
    var phi_43_: bool;
    var phi_44_: i32;
    var phi_45_: f32;
    var phi_46_: f32;
    var phi_47_: vec2<f32>;
    var phi_48_: i32;
    var phi_49_: f32;
    var phi_50_: f32;
    var phi_51_: vec2<f32>;
    var local_41: f32;
    var phi_52_: i32;
    var phi_53_: f32;
    var phi_54_: f32;
    var phi_55_: vec2<f32>;
    var phi_56_: i32;
    var phi_57_: f32;
    var phi_58_: f32;
    var phi_59_: vec2<f32>;
    var local_42: f32;
    var local_43: f32;
    var phi_60_: vec3<f32>;
    var phi_61_: vec3<f32>;
    var phi_62_: vec3<f32>;
    var phi_63_: vec3<f32>;
    var phi_64_: i32;
    var phi_65_: f32;
    var phi_66_: f32;
    var phi_67_: vec2<f32>;
    var phi_68_: i32;
    var phi_69_: f32;
    var phi_70_: f32;
    var phi_71_: vec2<f32>;
    var local_44: f32;
    var phi_72_: vec3<f32>;
    var phi_73_: i32;
    var phi_74_: f32;
    var phi_75_: f32;
    var phi_76_: vec2<f32>;
    var phi_77_: i32;
    var phi_78_: f32;
    var phi_79_: f32;
    var phi_80_: vec2<f32>;
    var local_45: f32;
    var phi_81_: f32;
    var phi_82_: vec3<f32>;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var local_49: f32;
    var phi_83_: u32;
    var phi_84_: u32;
    var phi_85_: u32;
    var phi_86_: u32;
    var phi_87_: u32;
    var phi_88_: bool;
    var phi_89_: u32;
    var phi_90_: u32;
    var phi_91_: u32;
    var phi_92_: u32;
    var phi_93_: bool;
    var phi_94_: u32;
    var phi_95_: u32;
    var phi_96_: bool;
    var phi_97_: u32;
    var phi_98_: u32;
    var phi_99_: u32;
    var phi_100_: u32;
    var phi_101_: u32;
    var phi_102_: bool;
    var local_50: u32;
    var phi_103_: bool;
    var phi_104_: u32;
    var phi_105_: f32;
    var phi_106_: f32;
    var phi_107_: u32;
    var phi_108_: i32;
    var phi_109_: f32;
    var phi_110_: u32;
    var phi_111_: i32;
    var phi_112_: bool;
    var local_51: f32;
    var local_52: i32;
    var phi_113_: bool;
    var phi_114_: bool;
    var phi_115_: f32;
    var phi_116_: bool;
    var phi_117_: f32;
    var phi_118_: bool;
    var phi_119_: f32;
    var phi_120_: bool;
    var phi_121_: f32;
    var phi_122_: bool;
    var phi_123_: f32;
    var phi_124_: bool;
    var phi_125_: u32;
    var phi_126_: f32;
    var phi_127_: bool;
    var phi_128_: bool;
    var local_53: f32;
    var phi_129_: f32;
    var phi_130_: f32;
    var phi_131_: bool;
    var phi_132_: f32;
    var phi_133_: bool;
    var phi_134_: f32;
    var phi_135_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e216 = pixel_2;
            let _e217 = weather_1;
            let _e218 = _isthmus_instance_index_9;
            let _e222 = arrayLength((&placed_glyphs_2.member));
            let _e235 = pill_2.member[_e218].x;
            let _e239 = frame.member[0u].panel_height;
            let _e240 = (_e216.x - _e235);
            let _e241 = (_e216.y - 6f);
            let _e242 = (_e239 * 0.5f);
            let _e246 = ((308f - _e239) * 0.5f);
            let _e248 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e240 - 154f), (_e241 - _e242)), _e246, _e242);
            let _e252 = frame.member[0u].mouse_pressure;
            let _e253 = (_e252 > 0f);
            if _e253 {
                let _e258 = frame.member[0u].mouse_pos[0u];
                let _e263 = frame.member[0u].mouse_pos[1u];
                let _e269 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e258 - _e235) - 154f), ((_e263 - 6f) - _e242)), _e246, _e242);
                phi_0_ = _e269;
            } else {
                phi_0_ = 1f;
            }
            let _e271 = phi_0_;
            phi_1_ = 0u;
            loop {
                let _e273 = phi_1_;
                let _e274 = (_e273 < 4u);
                if _e274 {
                    if _e274 {
                    } else {
                        phi_3_ = true;
                        break;
                    }
                    phi_2_ = (_e273 + 1u);
                } else {
                    phi_2_ = u32();
                }
                let _e277 = phi_2_;
                continue;
                continuing {
                    phi_1_ = _e277;
                    phi_3_ = false;
                    break if !(_e274);
                }
            }
            let _e280 = phi_3_;
            if _e280 {
                break;
            }
            let _e286 = (_e235 - (_e217.w * 158f));
            let _e287 = (6f + _e239);
            let _e288 = (8f * _e217.w);
            let _e289 = ((244f * _e217.w) - _e288);
            if (_e289 != _e289) {
                phi_4_ = true;
            } else {
                phi_4_ = (0f >= _e289);
            }
            let _e293 = phi_4_;
            let _e296 = (_e216.y - _e287);
            let _e297 = ((308f + (316f * _e217.w)) * 0.5f);
            let _e298 = (select(_e289, 0f, _e293) * 0.5f);
            let _e299 = (_e288 + _e298);
            let _e302 = (_e298 != _e298);
            if _e302 {
                phi_5_ = true;
            } else {
                phi_5_ = (18f <= _e298);
            }
            let _e305 = phi_5_;
            let _e308 = vec2<f32>(_e297, _e298);
            let _e309 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e216.x - _e286) - _e297), (_e296 - _e299)), _e308, select(_e298, 18f, _e305));
            let _e314 = frame.member[0u].mouse_pos[0u];
            let _e319 = frame.member[0u].mouse_pos[1u];
            if _e302 {
                phi_6_ = true;
            } else {
                phi_6_ = (18f <= _e298);
            }
            let _e326 = phi_6_;
            let _e329 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e314 - _e286) - _e297), ((_e319 - _e287) - _e299)), _e308, select(_e298, 18f, _e326));
            let _e332 = (0.5f + ((_e309 - _e248) * 0.008928572f));
            let _e334 = select(_e332, 0f, (_e332 < 0f));
            let _e336 = select(_e334, 1f, (_e334 > 1f));
            let _e349 = (0.5f + ((_e329 - _e271) * 0.008928572f));
            let _e351 = select(_e349, 0f, (_e349 < 0f));
            let _e353 = select(_e351, 1f, (_e351 > 1f));
            phi_7_ = vec2<f32>(0f, 0f);
            phi_8_ = 0f;
            phi_9_ = 0u;
            loop {
                let _e365 = phi_7_;
                let _e367 = phi_8_;
                let _e369 = phi_9_;
                local_37 = _e365;
                local_38 = _e365;
                local_39 = _e365;
                local_40 = _e365;
                local_46 = _e367;
                local_47 = _e367;
                local_48 = _e367;
                local_49 = _e367;
                let _e370 = (_e369 < 4u);
                if _e370 {
                    if _e370 {
                    } else {
                        phi_19_ = true;
                        break;
                    }
                    let _e377 = frame.member[0u].ripples[_e369].origin[0u];
                    let _e384 = frame.member[0u].ripples[_e369].origin[1u];
                    let _e390 = frame.member[0u].ripples[_e369].start_time;
                    let _e396 = frame.member[0u].ripples[_e369].strength;
                    let _e400 = frame.member[0u].time;
                    let _e402 = ((_e400 - _e390) * 1.2f);
                    let _e404 = select(_e402, 0f, (_e402 < 0f));
                    let _e406 = select(_e404, 1f, (_e404 > 1f));
                    if (_e396 > 0f) {
                        if (_e406 < 1f) {
                            let _e409 = (_e216.x - _e377);
                            let _e410 = (_e216.y - _e384);
                            let _e414 = sqrt(((_e409 * _e409) + (_e410 * _e410)));
                            if (_e414 > 0.001f) {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e409 / _e414), (_e410 / _e414)), _e414);
                            } else {
                                phi_10_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e414);
                            }
                            let _e422 = phi_10_;
                            let _e432 = ((abs((_e422.unnamed_1 - (_e406 * 600f))) - 80f) * -0.0125f);
                            let _e434 = select(_e432, 0f, (_e432 < 0f));
                            let _e436 = select(_e434, 1f, (_e434 > 1f));
                            let _e442 = (1f - _e406);
                            let _e443 = ((((_e436 * _e436) * (3f - (2f * _e436))) * _e396) * _e442);
                            let _e456 = (_e367 + (_e443 * 0.5f));
                            if (_e456 != _e456) {
                                phi_11_ = true;
                            } else {
                                phi_11_ = (1f <= _e456);
                            }
                            let _e460 = phi_11_;
                            phi_12_ = vec2<f32>((_e365.x + (((_e422.unnamed.x * _e443) * _e442) * 0.5f)), (_e365.y + (((_e422.unnamed.y * _e443) * _e442) * 0.5f)));
                            phi_13_ = select(_e456, 1f, _e460);
                        } else {
                            phi_12_ = _e365;
                            phi_13_ = _e367;
                        }
                        let _e463 = phi_12_;
                        let _e465 = phi_13_;
                        phi_14_ = _e463;
                        phi_15_ = _e465;
                    } else {
                        phi_14_ = _e365;
                        phi_15_ = _e367;
                    }
                    let _e467 = phi_14_;
                    let _e469 = phi_15_;
                    phi_16_ = _e467;
                    phi_17_ = _e469;
                    phi_18_ = (_e369 + 1u);
                } else {
                    phi_16_ = vec2<f32>();
                    phi_17_ = f32();
                    phi_18_ = u32();
                }
                let _e472 = phi_16_;
                let _e474 = phi_17_;
                let _e476 = phi_18_;
                continue;
                continuing {
                    phi_7_ = _e472;
                    phi_8_ = _e474;
                    phi_9_ = _e476;
                    phi_19_ = _e280;
                    break if !(_e370);
                }
            }
            let _e479 = phi_19_;
            if _e479 {
                break;
            }
            if _e253 {
                let _e480 = (_e216.x - _e314);
                let _e481 = (_e216.y - _e319);
                let _e487 = ((sqrt(((_e480 * _e480) + (_e481 * _e481))) - 150f) * -0.006666667f);
                let _e489 = select(_e487, 0f, (_e487 < 0f));
                let _e491 = select(_e489, 1f, (_e489 > 1f));
                phi_20_ = ((((_e491 * _e491) * (3f - (2f * _e491))) * _e252) * 8f);
            } else {
                phi_20_ = 0f;
            }
            let _e499 = phi_20_;
            let _e501 = local_37;
            let _e503 = global[0u];
            if (_e501.x == _e503) {
                let _e506 = local_38;
                let _e509 = global[1u];
                phi_21_ = (_e506.y == _e509);
            } else {
                phi_21_ = false;
            }
            let _e512 = phi_21_;
            if _e512 {
                phi_22_ = 0f;
            } else {
                let _e514 = local_39;
                phi_22_ = (sqrt(((_e501.x * _e501.x) + (_e514.y * _e514.y))) * 22f);
            }
            let _e522 = phi_22_;
            let _e524 = local_40;
            let _e527 = (((_e271 + ((((_e329 + ((_e271 - _e329) * _e353)) - ((56f * _e353) * (1f - _e353))) - _e271) * _e217.w)) - 0.5f) * -1f);
            let _e529 = select(_e527, 0f, (_e527 < 0f));
            let _e531 = select(_e529, 1f, (_e529 > 1f));
            let _e539 = ((_e248 + ((((_e309 + ((_e248 - _e309) * _e336)) - ((56f * _e336) * (1f - _e336))) - _e248) * _e217.w)) - (((_e499 * ((_e531 * _e531) * (3f - (2f * _e531)))) + _e522) * 0.5f));
            let _e541 = (56f + _e242);
            let _e542 = (_e239 + 8f);
            let _e543 = (_e541 + _e542);
            let _e545 = ((_e541 + _e543) * 0.5f);
            let _e546 = ((_e241 - _e239) > _e545);
            let _e551 = pill_2.member[_e218].calendar_expansion;
            let _e553 = (_e541 + (select(0f, 1f, _e546) * _e542));
            let _e554 = (_e553 * 0.0007377049f);
            let _e555 = (0.5f + _e554);
            let _e559 = ((_e551 - _e555) / ((_e554 + 0.74f) - _e555));
            let _e561 = select(_e559, 0f, (_e559 < 0f));
            let _e563 = select(_e561, 1f, (_e561 > 1f));
            let _e567 = ((_e563 * _e563) * (3f - (2f * _e563)));
            let _e569 = (292f * _e567);
            let _e570 = (_e239 * _e567);
            let _e578 = ((_e235 + 166f) + ((292f - _e569) * 0.5f));
            let _e579 = ((_e287 + (_e553 - _e242)) + ((_e239 - _e570) * 0.5f));
            let _e580 = (_e216.x - _e578);
            let _e581 = (_e216.y - _e579);
            let _e582 = (_e567 <= 0.001f);
            if _e582 {
                phi_23_ = 340282350000000000000000000000000000000f;
            } else {
                let _e584 = (_e570 * 0.5f);
                let _e590 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e580 - (_e567 * 146f)), (_e581 - _e584)), ((_e569 - _e570) * 0.5f), _e584);
                phi_23_ = _e590;
            }
            let _e592 = phi_23_;
            if _e582 {
                phi_24_ = 340282350000000000000000000000000000000f;
            } else {
                let _e596 = (_e570 * 0.5f);
                let _e602 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e314 - _e578) - (_e567 * 146f)), ((_e319 - _e579) - _e596)), ((_e569 - _e570) * 0.5f), _e596);
                phi_24_ = _e602;
            }
            let _e604 = phi_24_;
            if (_e569 != _e569) {
                phi_25_ = true;
            } else {
                phi_25_ = (0.001f >= _e569);
            }
            let _e608 = phi_25_;
            let _e610 = (_e580 / select(_e569, 0.001f, _e608));
            if _e546 {
                let _e618 = ((_e610 * 5f) - 0.5f);
                let _e620 = select(_e618, 0f, (_e618 < 0f));
                phi_26_ = select(_e620, 4f, (_e620 > 4f));
            } else {
                let _e612 = ((_e610 * 6f) - 0.5f);
                let _e614 = select(_e612, 0f, (_e612 < 0f));
                phi_26_ = select(_e614, 5f, (_e614 > 5f));
            }
            let _e624 = phi_26_;
            let _e625 = floor(_e624);
            let _e630 = select(select(u32(_e625), 0u, (_e625 < 0f)), 4294967295u, (_e625 > 4294967000f));
            let _e632 = (_e624 - trunc(_e624));
            let _e634 = select(_e632, 0f, (_e632 < 0f));
            let _e636 = select(_e634, 1f, (_e634 > 1f));
            let _e640 = ((_e636 * _e636) * (3f - (2f * _e636)));
            if _e546 {
                if (_e630 < 5u) {
                } else {
                    break;
                }
                let _e668 = pill_2.member[_e218].daily_conditions[_e630];
                let _e669 = (_e630 + 1u);
                let _e671 = select(_e669, 4u, (4u < _e669));
                if (_e671 < 5u) {
                } else {
                    break;
                }
                let _e677 = pill_2.member[_e218].daily_conditions[_e671];
                phi_27_ = 12f;
                phi_28_ = _e677;
                phi_29_ = _e668;
            } else {
                if (_e630 < 6u) {
                } else {
                    break;
                }
                let _e646 = pill_2.member[_e218].hourly_conditions[_e630];
                let _e647 = (_e630 + 1u);
                let _e649 = select(_e647, 5u, (5u < _e647));
                if (_e649 < 6u) {
                } else {
                    break;
                }
                let _e655 = pill_2.member[_e218].hourly_conditions[_e649];
                let _e659 = pill_2.member[_e218].hourly_start;
                phi_27_ = ((_e659 + (_e624 * 4f)) % 24f);
                phi_28_ = _e655;
                phi_29_ = _e646;
            }
            let _e679 = phi_27_;
            let _e681 = phi_28_;
            let _e683 = phi_29_;
            let _e717 = pill_2.member[_e218].sun_hours;
            let _e720 = (_e717[1] - _e717[0]);
            if (_e679 >= _e717[0]) {
                let _e722 = (_e679 <= _e717[1]);
                if _e722 {
                    let _e724 = ((_e679 - _e717[0]) / _e720);
                    phi_30_ = array<f32, 2>(_e724, sin((_e724 * 3.1415927f)));
                } else {
                    phi_30_ = array<f32, 2>();
                }
                let _e729 = phi_30_;
                phi_31_ = _e729;
                phi_32_ = select(true, false, _e722);
            } else {
                phi_31_ = array<f32, 2>();
                phi_32_ = true;
            }
            let _e732 = phi_31_;
            let _e734 = phi_32_;
            if _e734 {
                let _e735 = (24f - _e720);
                if (_e679 < _e717[0]) {
                    phi_33_ = (((_e679 + 24f) - _e717[1]) / _e735);
                } else {
                    phi_33_ = ((_e679 - _e717[1]) / _e735);
                }
                let _e743 = phi_33_;
                phi_34_ = array<f32, 2>(select(0f, 1f, (_e679 >= _e717[1])), -(sin((_e743 * 3.1415927f))));
            } else {
                phi_34_ = _e732;
            }
            let _e751 = phi_34_;
            let _e754 = ((_e604 - 0.5f) * -1f);
            let _e756 = select(_e754, 0f, (_e754 < 0f));
            let _e758 = select(_e756, 1f, (_e756 > 1f));
            let _e766 = (_e592 - (((_e499 * ((_e758 * _e758) * (3f - (2f * _e758)))) + _e522) * 0.5f));
            let _e767 = (_e539 != _e539);
            if _e767 {
                phi_35_ = true;
            } else {
                phi_35_ = (_e766 <= _e539);
            }
            let _e770 = phi_35_;
            let _e771 = select(_e539, _e766, _e770);
            let _e772 = fwidth(_e771);
            if (_e772 != _e772) {
                phi_36_ = true;
            } else {
                phi_36_ = (0.55f >= _e772);
            }
            let _e776 = phi_36_;
            let _e777 = select(_e772, 0.55f, _e776);
            let _e781 = ((_e771 - _e777) / (-(_e777) - _e777));
            let _e783 = select(_e781, 0f, (_e781 < 0f));
            let _e785 = select(_e783, 1f, (_e783 > 1f));
            let _e789 = ((_e785 * _e785) * (3f - (2f * _e785)));
            if (_e771 != _e771) {
                phi_37_ = true;
            } else {
                phi_37_ = (0f >= _e771);
            }
            let _e793 = phi_37_;
            let _e797 = (exp((select(_e771, 0f, _e793) * -0.3f)) * 0.16f);
            if (_e789 != _e789) {
                phi_38_ = true;
            } else {
                phi_38_ = (_e797 >= _e789);
            }
            let _e801 = phi_38_;
            let _e802 = select(_e789, _e797, _e801);
            if (_e802 <= 0.0009765625f) {
                discard;
            }
            let _e808 = pill_2.member[_e218].hourly_conditions[0u];
            let _e809 = (_e240 * 0.0032467532f);
            let _e811 = select(_e809, 0f, (_e809 < 0f));
            let _e820 = pill_2.member[_e218].hourly_conditions[1u];
            let _e822 = ((abs((select(_e811, 1f, (_e811 > 1f)) - 0.5f)) - 0.2f) * 20.000002f);
            let _e824 = select(_e822, 0f, (_e822 < 0f));
            let _e826 = select(_e824, 1f, (_e824 > 1f));
            let _e830 = ((_e826 * _e826) * (3f - (2f * _e826)));
            let _e835 = (_e808.fog + ((_e820.fog - _e808.fog) * _e830));
            let _e840 = (_e808.cloud + ((_e820.cloud - _e808.cloud) * _e830));
            let _e845 = (_e808.rain + ((_e820.rain - _e808.rain) * _e830));
            let _e850 = (_e808.snow + ((_e820.snow - _e808.snow) * _e830));
            let _e855 = (_e808.lightning + ((_e820.lightning - _e808.lightning) * _e830));
            let _e860 = (_e808.hail + ((_e820.hail - _e808.hail) * _e830));
            let _e863 = (_e835 + ((_e808.fog - _e835) * _e217.w));
            let _e866 = (_e840 + ((_e808.cloud - _e840) * _e217.w));
            let _e869 = (_e845 + ((_e808.rain - _e845) * _e217.w));
            let _e872 = (_e850 + ((_e808.snow - _e850) * _e217.w));
            let _e875 = (_e855 + ((_e808.lightning - _e855) * _e217.w));
            let _e878 = (_e860 + ((_e808.hail - _e860) * _e217.w));
            let _e879 = (_e241 / _e239);
            if _e767 {
                phi_39_ = true;
            } else {
                phi_39_ = (0f <= _e539);
            }
            let _e884 = phi_39_;
            let _e887 = (1f + (select(_e539, 0f, _e884) * 0.008333334f));
            let _e889 = select(_e887, 0f, (_e887 < 0f));
            let _e891 = select(_e889, 0.6f, (_e889 > 0.6f));
            let _e898 = (_e501.x * 0.04f);
            let _e899 = (_e524.y * 0.04f);
            let _e900 = ((_e809 - (((_e809 - 0.5f) * _e891) * 0.08f)) - _e898);
            let _e901 = ((_e879 - (((_e879 - 0.5f) * _e891) * 0.08f)) - _e899);
            if (_e567 > 0.001f) {
                let _e904 = (_e580 / _e569);
                let _e905 = (_e581 / _e570);
                if (_e766 != _e766) {
                    phi_40_ = true;
                } else {
                    phi_40_ = (0f <= _e766);
                }
                let _e911 = phi_40_;
                let _e914 = (1f + (select(_e766, 0f, _e911) * 0.008333334f));
                let _e916 = select(_e914, 0f, (_e914 < 0f));
                let _e918 = select(_e916, 0.6f, (_e916 > 0.6f));
                phi_41_ = vec2<f32>(((_e904 - (((_e904 - 0.5f) * _e918) * 0.08f)) - _e898), ((_e905 - (((_e905 - 0.5f) * _e918) * 0.08f)) - _e899));
            } else {
                phi_41_ = vec2<f32>(_e900, _e901);
            }
            let _e929 = phi_41_;
            let _e930 = fwidth(_e766);
            if (_e930 != _e930) {
                phi_42_ = true;
            } else {
                phi_42_ = (0.55f >= _e930);
            }
            let _e934 = phi_42_;
            let _e935 = select(_e930, 0.55f, _e934);
            let _e939 = ((_e766 - _e935) / (-(_e935) - _e935));
            let _e941 = select(_e939, 0f, (_e939 < 0f));
            let _e943 = select(_e941, 1f, (_e941 > 1f));
            let _e948 = (((_e943 * _e943) * (3f - (2f * _e943))) * _e567);
            let _e955 = (1f - _e948);
            let _e960 = (((_e900 * 308f) * _e955) + ((_e929.x * _e569) * _e948));
            let _e961 = (((_e901 * _e239) * _e955) + ((_e929.y * _e570) * _e948));
            if (_e766 != _e766) {
                phi_43_ = true;
            } else {
                phi_43_ = (1000f <= _e766);
            }
            let _e968 = phi_43_;
            let _e975 = (_e863 + (((_e683.fog + ((_e681.fog - _e683.fog) * _e640)) - _e863) * _e948));
            let _e978 = (_e866 + (((_e683.cloud + ((_e681.cloud - _e683.cloud) * _e640)) - _e866) * _e948));
            let _e981 = (_e869 + (((_e683.rain + ((_e681.rain - _e683.rain) * _e640)) - _e869) * _e948));
            let _e984 = (_e872 + (((_e683.snow + ((_e681.snow - _e683.snow) * _e640)) - _e872) * _e948));
            let _e990 = (_e878 + (((_e683.hail + ((_e681.hail - _e683.hail) * _e640)) - _e878) * _e948));
            let _e992 = ((_e217.y - -0.04f) * 4.1666665f);
            let _e994 = select(_e992, 0f, (_e992 < 0f));
            let _e996 = select(_e994, 1f, (_e994 > 1f));
            let _e1000 = ((_e996 * _e996) * (3f - (2f * _e996)));
            let _e1002 = ((_e217.y - -0.32f) * 4.166667f);
            let _e1004 = select(_e1002, 0f, (_e1002 < 0f));
            let _e1006 = select(_e1004, 1f, (_e1004 > 1f));
            let _e1012 = (((_e1006 * _e1006) * (3f - (2f * _e1006))) * (1f - _e1000));
            let _e1014 = ((_e217.y - -0.18f) * 5.5555553f);
            let _e1016 = select(_e1014, 0f, (_e1014 < 0f));
            let _e1018 = select(_e1016, 1f, (_e1016 > 1f));
            let _e1024 = ((_e217.y - 0.2f) * -5.5555553f);
            let _e1026 = select(_e1024, 0f, (_e1024 < 0f));
            let _e1028 = select(_e1026, 1f, (_e1026 > 1f));
            let _e1033 = (((_e1018 * _e1018) * (3f - (2f * _e1018))) * ((_e1028 * _e1028) * (3f - (2f * _e1028))));
            let _e1035 = ((_e751[1] - -0.04f) * 4.1666665f);
            let _e1037 = select(_e1035, 0f, (_e1035 < 0f));
            let _e1039 = select(_e1037, 1f, (_e1037 > 1f));
            let _e1043 = ((_e1039 * _e1039) * (3f - (2f * _e1039)));
            let _e1045 = ((_e751[1] - -0.32f) * 4.166667f);
            let _e1047 = select(_e1045, 0f, (_e1045 < 0f));
            let _e1049 = select(_e1047, 1f, (_e1047 > 1f));
            let _e1057 = ((_e751[1] - -0.18f) * 5.5555553f);
            let _e1059 = select(_e1057, 0f, (_e1057 < 0f));
            let _e1061 = select(_e1059, 1f, (_e1059 > 1f));
            let _e1067 = ((_e751[1] - 0.2f) * -5.5555553f);
            let _e1069 = select(_e1067, 0f, (_e1067 < 0f));
            let _e1071 = select(_e1069, 1f, (_e1069 > 1f));
            let _e1079 = (_e1000 + ((_e1043 - _e1000) * _e948));
            let _e1085 = (_e1033 + (((((_e1061 * _e1061) * (3f - (2f * _e1061))) * ((_e1071 * _e1071) * (3f - (2f * _e1071)))) - _e1033) * _e948));
            let _e1089 = frame.member[0u].time;
            let _e1090 = (_e961 / _e239);
            let _e1092 = ((_e1090 - 1f) * -1f);
            let _e1094 = select(_e1092, 0f, (_e1092 < 0f));
            let _e1096 = select(_e1094, 1f, (_e1094 > 1f));
            let _e1100 = ((_e1096 * _e1096) * (3f - (2f * _e1096)));
            let _e1101 = (1f - _e1100);
            let _e1120 = (1f - _e1079);
            let _e1132 = (0.3f * _e1101);
            let _e1133 = (0.22f * _e1100);
            let _e1139 = ((_e1012 + (((((_e1049 * _e1049) * (3f - (2f * _e1049))) * (1f - _e1043)) - _e1012) * _e948)) * 0.8f);
            let _e1140 = (1f - _e1139);
            let _e1157 = (_e1085 * 0.9f);
            let _e1158 = (1f - _e1157);
            let _e1170 = floor((_e960 * 0.055555556f));
            let _e1171 = floor((_e961 * 0.055555556f));
            let _e1175 = cantus_render_shader_hash(vec2<f32>(_e1170, _e1171));
            let _e1184 = (_e960 - (((_e1170 + 0.2f) + (_e1175.x * 0.6f)) * 18f));
            let _e1185 = (_e961 - (((_e1171 + 0.2f) + (_e1175.y * 0.6f)) * 18f));
            let _e1191 = ((sqrt(((_e1184 * _e1184) + (_e1185 * _e1185))) - 1f) * -1.6666666f);
            let _e1193 = select(_e1191, 0f, (_e1191 < 0f));
            let _e1195 = select(_e1193, 1f, (_e1193 > 1f));
            let _e1203 = cantus_render_shader_hash(vec2<f32>((_e1170 + 31.7f), (_e1171 + 31.7f)));
            let _e1206 = ((_e1203.x - 0.75f) * 4f);
            let _e1208 = select(_e1206, 0f, (_e1206 < 0f));
            let _e1210 = select(_e1208, 1f, (_e1208 > 1f));
            let _e1221 = ((((((_e1195 * _e1195) * (3f - (2f * _e1195))) * ((_e1210 * _e1210) * (3f - (2f * _e1210)))) * _e1120) * (1f - _e978)) * (0.3f + (_e1100 * 0.7f)));
            let _e1222 = (((((((((0.006f * _e1101) + (0.025f * _e1100)) * _e1120) + (((0.08f * _e1101) + (0.32f * _e1100)) * _e1079)) * _e1140) + (((0.1f * _e1101) + _e1133) * _e1139)) * _e1158) + (((0.78f * _e1101) + (0.38f * _e1100)) * _e1157)) + _e1221);
            let _e1223 = (((((((((0.012f * _e1101) + (0.04f * _e1100)) * _e1120) + (((0.34f * _e1101) + (0.67f * _e1100)) * _e1079)) * _e1140) + (((0.16f * _e1101) + (0.25f * _e1100)) * _e1139)) * _e1158) + ((_e1132 + _e1133) * _e1157)) + _e1221);
            let _e1224 = (((((((((0.035f * _e1101) + (0.095f * _e1100)) * _e1120) + (((0.62f * _e1101) + (0.87f * _e1100)) * _e1079)) * _e1140) + ((_e1132 + (0.45f * _e1100)) * _e1139)) * _e1158) + (((0.2f * _e1101) + (0.42f * _e1100)) * _e1157)) + _e1221);
            if (_e978 > 0.0009765625f) {
                let _e1227 = (_e960 / _e239);
                phi_44_ = 0i;
                phi_45_ = 0.5f;
                phi_46_ = 0f;
                phi_47_ = vec2<f32>(((_e1227 * 0.14f) + (_e1089 * 0.012f)), ((_e1090 * 0.14f) + 6.1f));
                loop {
                    let _e1235 = phi_44_;
                    let _e1237 = phi_45_;
                    let _e1239 = phi_46_;
                    let _e1241 = phi_47_;
                    local_41 = _e1239;
                    let _e1242 = (_e1235 < 4i);
                    if _e1242 {
                        let _e1245 = cantus_render_shader_simplex_noise(_e1241);
                        phi_48_ = (_e1235 + 1i);
                        phi_49_ = (_e1237 * 0.5f);
                        phi_50_ = (_e1239 + (_e1245 * _e1237));
                        phi_51_ = vec2<f32>(((_e1241.x * 1.6f) + (_e1241.y * 1.2f)), ((_e1241.y * 1.6f) - (_e1241.x * 1.2f)));
                    } else {
                        phi_48_ = i32();
                        phi_49_ = f32();
                        phi_50_ = f32();
                        phi_51_ = vec2<f32>();
                    }
                    let _e1258 = phi_48_;
                    let _e1260 = phi_49_;
                    let _e1262 = phi_50_;
                    let _e1264 = phi_51_;
                    continue;
                    continuing {
                        phi_44_ = _e1258;
                        phi_45_ = _e1260;
                        phi_46_ = _e1262;
                        phi_47_ = _e1264;
                        break if !(_e1242);
                    }
                }
                let _e1267 = local_41;
                let _e1268 = (_e1267 * 0.5f);
                phi_52_ = 0i;
                phi_53_ = 0.5f;
                phi_54_ = 0f;
                phi_55_ = vec2<f32>(((_e1227 * 0.287f) + (_e1089 * 0.018f)), ((_e1090 * 0.287f) + -3.7f));
                loop {
                    let _e1277 = phi_52_;
                    let _e1279 = phi_53_;
                    let _e1281 = phi_54_;
                    let _e1283 = phi_55_;
                    local_42 = _e1281;
                    local_43 = _e1281;
                    let _e1284 = (_e1277 < 4i);
                    if _e1284 {
                        let _e1287 = cantus_render_shader_simplex_noise(_e1283);
                        phi_56_ = (_e1277 + 1i);
                        phi_57_ = (_e1279 * 0.5f);
                        phi_58_ = (_e1281 + (_e1287 * _e1279));
                        phi_59_ = vec2<f32>(((_e1283.x * 1.6f) + (_e1283.y * 1.2f)), ((_e1283.y * 1.6f) - (_e1283.x * 1.2f)));
                    } else {
                        phi_56_ = i32();
                        phi_57_ = f32();
                        phi_58_ = f32();
                        phi_59_ = vec2<f32>();
                    }
                    let _e1300 = phi_56_;
                    let _e1302 = phi_57_;
                    let _e1304 = phi_58_;
                    let _e1306 = phi_59_;
                    continue;
                    continuing {
                        phi_52_ = _e1300;
                        phi_53_ = _e1302;
                        phi_54_ = _e1304;
                        phi_55_ = _e1306;
                        break if !(_e1284);
                    }
                }
                let _e1309 = local_42;
                let _e1312 = local_43;
                let _e1316 = ((((0.5f + _e1268) + (_e1312 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1318 = select(_e1316, 0f, (_e1316 < 0f));
                let _e1320 = select(_e1318, 1f, (_e1318 > 1f));
                let _e1326 = (((_e1309 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1328 = select(_e1326, 0f, (_e1326 < 0f));
                let _e1330 = select(_e1328, 1f, (_e1328 > 1f));
                let _e1337 = ((_e1268 + 0.02000001f) * 4.5454545f);
                let _e1339 = select(_e1337, 0f, (_e1337 < 0f));
                let _e1341 = select(_e1339, 1f, (_e1339 > 1f));
                let _e1347 = ((((_e1330 * _e1330) * (3f - (2f * _e1330))) * 0.55f) + (((_e1341 * _e1341) * (3f - (2f * _e1341))) * 0.45f));
                let _e1348 = (1f - _e1347);
                let _e1385 = (_e1085 * 0.45f);
                let _e1386 = (1f - _e1385);
                let _e1398 = (_e978 * (0.12f + (((_e1320 * _e1320) * (3f - (2f * _e1320))) * 0.7f)));
                let _e1399 = (1f - _e1398);
                phi_60_ = vec3<f32>(((_e1222 * _e1399) + (((((((0.16f * _e1348) + (0.32f * _e1347)) * _e1120) + (((0.62f * _e1348) + (0.92f * _e1347)) * _e1079)) * _e1386) + (((0.5f * _e1348) + (0.76f * _e1347)) * _e1385)) * _e1398)), ((_e1223 * _e1399) + (((((((0.2f * _e1348) + (0.36f * _e1347)) * _e1120) + (((0.7f * _e1348) + (0.94f * _e1347)) * _e1079)) * _e1386) + (((0.36f * _e1348) + (0.59f * _e1347)) * _e1385)) * _e1398)), ((_e1224 * _e1399) + (((((((0.28f * _e1348) + (0.43f * _e1347)) * _e1120) + (((0.78f * _e1348) + (0.96f * _e1347)) * _e1079)) * _e1386) + (((0.4f * _e1348) + (0.56f * _e1347)) * _e1385)) * _e1398)));
            } else {
                phi_60_ = vec3<f32>(_e1222, _e1223, _e1224);
            }
            let _e1411 = phi_60_;
            let _e1413 = (1f - (_e981 * 0.2f));
            let _e1423 = ((_e1411.x * _e1413) + (_e981 * 0.020000001f));
            let _e1424 = ((_e1411.y * _e1413) + (_e981 * 0.034f));
            let _e1425 = ((_e1411.z * _e1413) + (_e981 * 0.05f));
            if (_e981 > 0.0009765625f) {
                let _e1430 = (_e960 - (20f * _e1089));
                let _e1431 = (_e961 - (110f * _e1089));
                let _e1434 = floor((_e1430 * 0.06666667f));
                let _e1435 = floor((_e1431 * 0.04f));
                let _e1437 = cantus_render_shader_hash(vec2<f32>(_e1434, _e1435));
                let _e1448 = (_e1430 - (((_e1434 + 0.15f) + (_e1437.x * 0.7f)) * 15f));
                let _e1449 = (_e1431 - (((_e1435 + 0.15f) + (_e1437.y * 0.7f)) * 25f));
                let _e1453 = (((_e1448 * 1.8000001f) + (_e1449 * 9f)) * 0.011870845f);
                let _e1455 = select(_e1453, 0f, (_e1453 < 0f));
                let _e1457 = select(_e1455, 1f, (_e1455 > 1f));
                let _e1460 = (_e1448 - (1.8000001f * _e1457));
                let _e1461 = (_e1449 - (9f * _e1457));
                let _e1467 = ((sqrt(((_e1460 * _e1460) + (_e1461 * _e1461))) - 1.0999999f) * -1.666667f);
                let _e1469 = select(_e1467, 0f, (_e1467 < 0f));
                let _e1471 = select(_e1469, 1f, (_e1469 > 1f));
                let _e1479 = cantus_render_shader_hash(vec2<f32>((_e1434 + 19.3f), (_e1435 + 19.3f)));
                let _e1482 = ((_e1479.x - 0.22000003f) * 1.2820513f);
                let _e1484 = select(_e1482, 0f, (_e1482 < 0f));
                let _e1486 = select(_e1484, 1f, (_e1484 > 1f));
                let _e1493 = (((((_e1471 * _e1471) * (3f - (2f * _e1471))) * ((_e1486 * _e1486) * (3f - (2f * _e1486)))) * _e981) * 0.7f);
                let _e1495 = select(_e1493, 0f, (_e1493 < 0f));
                let _e1497 = select(_e1495, 1f, (_e1495 > 1f));
                let _e1498 = (1f - _e1497);
                phi_61_ = vec3<f32>(((_e1423 * _e1498) + (0.52f * _e1497)), ((_e1424 * _e1498) + (0.72f * _e1497)), ((_e1425 * _e1498) + (0.9f * _e1497)));
            } else {
                phi_61_ = vec3<f32>(_e1423, _e1424, _e1425);
            }
            let _e1510 = phi_61_;
            if (_e984 > 0.0009765625f) {
                let _e1514 = (_e960 - (5f * _e1089));
                let _e1515 = (_e961 - (14f * _e1089));
                let _e1518 = floor((_e1514 * 0.05f));
                let _e1519 = floor((_e1515 * 0.05f));
                let _e1523 = cantus_render_shader_hash(vec2<f32>((_e1518 + 31.7f), (_e1519 + 31.7f)));
                let _e1534 = (_e1514 - (((_e1518 + 0.15f) + (_e1523.x * 0.7f)) * 20f));
                let _e1535 = (_e1515 - (((_e1519 + 0.15f) + (_e1523.y * 0.7f)) * 20f));
                let _e1539 = (((_e1534 * 0.080000006f) + (_e1535 * 0.4f)) * 6.009615f);
                let _e1541 = select(_e1539, 0f, (_e1539 < 0f));
                let _e1543 = select(_e1541, 1f, (_e1541 > 1f));
                let _e1546 = (_e1534 - (0.080000006f * _e1543));
                let _e1547 = (_e1535 - (0.4f * _e1543));
                let _e1553 = ((sqrt(((_e1546 * _e1546) + (_e1547 * _e1547))) - 1.5999999f) * -1.666667f);
                let _e1555 = select(_e1553, 0f, (_e1553 < 0f));
                let _e1557 = select(_e1555, 1f, (_e1555 > 1f));
                let _e1565 = cantus_render_shader_hash(vec2<f32>((_e1518 + 19.3f), (_e1519 + 19.3f)));
                let _e1568 = ((_e1565.x - 0.3f) * 1.4285715f);
                let _e1570 = select(_e1568, 0f, (_e1568 < 0f));
                let _e1572 = select(_e1570, 1f, (_e1570 > 1f));
                let _e1579 = (((((_e1557 * _e1557) * (3f - (2f * _e1557))) * ((_e1572 * _e1572) * (3f - (2f * _e1572)))) * _e984) * 0.92f);
                let _e1581 = select(_e1579, 0f, (_e1579 < 0f));
                let _e1583 = select(_e1581, 1f, (_e1581 > 1f));
                let _e1584 = (1f - _e1583);
                let _e1591 = (0.96f * _e1583);
                phi_62_ = vec3<f32>(((_e1510.x * _e1584) + _e1591), ((_e1510.y * _e1584) + _e1591), ((_e1510.z * _e1584) + _e1591));
            } else {
                phi_62_ = _e1510;
            }
            let _e1597 = phi_62_;
            if (_e990 > 0.0009765625f) {
                let _e1601 = (_e960 - (18f * _e1089));
                let _e1602 = (_e961 - (85f * _e1089));
                let _e1605 = floor((_e1601 * 0.04347826f));
                let _e1606 = floor((_e1602 * 0.04347826f));
                let _e1610 = cantus_render_shader_hash(vec2<f32>((_e1605 + 63.4f), (_e1606 + 63.4f)));
                let _e1621 = (_e1601 - (((_e1605 + 0.15f) + (_e1610.x * 0.7f)) * 23f));
                let _e1622 = (_e1602 - (((_e1606 + 0.15f) + (_e1610.y * 0.7f)) * 23f));
                let _e1626 = (((_e1621 * 0.24000001f) + (_e1622 * 1.2f)) * 0.667735f);
                let _e1628 = select(_e1626, 0f, (_e1626 < 0f));
                let _e1630 = select(_e1628, 1f, (_e1628 > 1f));
                let _e1633 = (_e1621 - (0.24000001f * _e1630));
                let _e1634 = (_e1622 - (1.2f * _e1630));
                let _e1640 = ((sqrt(((_e1633 * _e1633) + (_e1634 * _e1634))) - 0.79999995f) * -1.6666667f);
                let _e1642 = select(_e1640, 0f, (_e1640 < 0f));
                let _e1644 = select(_e1642, 1f, (_e1642 > 1f));
                let _e1652 = cantus_render_shader_hash(vec2<f32>((_e1605 + 19.3f), (_e1606 + 19.3f)));
                let _e1655 = ((_e1652.x - 0.7f) * 3.3333333f);
                let _e1657 = select(_e1655, 0f, (_e1655 < 0f));
                let _e1659 = select(_e1657, 1f, (_e1657 > 1f));
                let _e1666 = (((((_e1644 * _e1644) * (3f - (2f * _e1644))) * ((_e1659 * _e1659) * (3f - (2f * _e1659)))) * _e990) * 0.7f);
                let _e1668 = select(_e1666, 0f, (_e1666 < 0f));
                let _e1670 = select(_e1668, 1f, (_e1668 > 1f));
                let _e1671 = (1f - _e1670);
                phi_63_ = vec3<f32>(((_e1597.x * _e1671) + (0.75f * _e1670)), ((_e1597.y * _e1671) + (0.86f * _e1670)), ((_e1597.z * _e1671) + (0.94f * _e1670)));
            } else {
                phi_63_ = _e1597;
            }
            let _e1686 = phi_63_;
            let _e1690 = ((sin((_e1089 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1692 = select(_e1690, 0f, (_e1690 < 0f));
            let _e1694 = select(_e1692, 1f, (_e1692 > 1f));
            let _e1699 = (((_e1694 * _e1694) * (3f - (2f * _e1694))) * (_e875 + (((_e683.lightning + ((_e681.lightning - _e683.lightning) * _e640)) - _e875) * _e948)));
            let _e1701 = (1f - (_e1699 * 0.55f));
            let _e1711 = ((_e1686.x * _e1701) + (_e1699 * 0.3575f));
            let _e1712 = ((_e1686.y * _e1701) + (_e1699 * 0.407f));
            let _e1713 = ((_e1686.z * _e1701) + (_e1699 * 0.528f));
            if (_e975 > 0.0009765625f) {
                phi_64_ = 0i;
                phi_65_ = 0.5f;
                phi_66_ = 0f;
                phi_67_ = vec2<f32>((((_e960 / (308f + ((_e569 - 308f) * _e948))) * 0.9f) + (_e1089 * 0.008f)), ((_e1090 * 0.32f) + 12f));
                loop {
                    let _e1724 = phi_64_;
                    let _e1726 = phi_65_;
                    let _e1728 = phi_66_;
                    let _e1730 = phi_67_;
                    local_44 = _e1728;
                    let _e1731 = (_e1724 < 4i);
                    if _e1731 {
                        let _e1734 = cantus_render_shader_simplex_noise(_e1730);
                        phi_68_ = (_e1724 + 1i);
                        phi_69_ = (_e1726 * 0.5f);
                        phi_70_ = (_e1728 + (_e1734 * _e1726));
                        phi_71_ = vec2<f32>(((_e1730.x * 1.6f) + (_e1730.y * 1.2f)), ((_e1730.y * 1.6f) - (_e1730.x * 1.2f)));
                    } else {
                        phi_68_ = i32();
                        phi_69_ = f32();
                        phi_70_ = f32();
                        phi_71_ = vec2<f32>();
                    }
                    let _e1747 = phi_68_;
                    let _e1749 = phi_69_;
                    let _e1751 = phi_70_;
                    let _e1753 = phi_71_;
                    continue;
                    continuing {
                        phi_64_ = _e1747;
                        phi_65_ = _e1749;
                        phi_66_ = _e1751;
                        phi_67_ = _e1753;
                        break if !(_e1731);
                    }
                }
                let _e1756 = local_44;
                let _e1759 = (((_e1756 * 0.5f) + 0.15f) * 2.857143f);
                let _e1761 = select(_e1759, 0f, (_e1759 < 0f));
                let _e1763 = select(_e1761, 1f, (_e1761 > 1f));
                let _e1770 = (_e975 * (0.58f + (((_e1763 * _e1763) * (3f - (2f * _e1763))) * 0.18f)));
                let _e1771 = (1f - _e1770);
                phi_72_ = vec3<f32>(((_e1711 * _e1771) + (0.63f * _e1770)), ((_e1712 * _e1771) + (0.69f * _e1770)), ((_e1713 * _e1771) + (0.73f * _e1770)));
            } else {
                phi_72_ = vec3<f32>(_e1711, _e1712, _e1713);
            }
            let _e1783 = phi_72_;
            let _e1785 = ((_e1090 - 0.12f) * -8.333334f);
            let _e1787 = select(_e1785, 0f, (_e1785 < 0f));
            let _e1789 = select(_e1787, 1f, (_e1787 > 1f));
            let _e1796 = (((_e539 + ((select(_e766, 1000f, _e968) - _e539) * _e948)) - 5f) * -0.125f);
            let _e1798 = select(_e1796, 0f, (_e1796 < 0f));
            let _e1800 = select(_e1798, 1f, (_e1798 > 1f));
            let _e1806 = ((((_e1789 * _e1789) * (3f - (2f * _e1789))) * 0.12f) + (((_e1800 * _e1800) * (3f - (2f * _e1800))) * 0.08f));
            let _e1808 = (_e1783.x + _e1806);
            let _e1810 = (_e1783.y + _e1806);
            let _e1812 = (_e1783.z + _e1806);
            if (_e248 < 1f) {
                let _e1817 = (16f + (_e217.x * 276f));
                let _e1819 = select(_e217.y, 0f, (_e217.y < 0f));
                let _e1823 = (0.72f - (select(_e1819, 1f, (_e1819 > 1f)) * 0.45f));
                let _e1826 = ((_e217.y - 0.55f) * -1.8867923f);
                let _e1828 = select(_e1826, 0f, (_e1826 < 0f));
                let _e1830 = select(_e1828, 1f, (_e1828 > 1f));
                let _e1834 = ((_e1830 * _e1830) * (3f - (2f * _e1830)));
                let _e1835 = (1f - _e1834);
                if (_e840 > 0.0009765625f) {
                    phi_73_ = 0i;
                    phi_74_ = 0.5f;
                    phi_75_ = 0f;
                    phi_76_ = vec2<f32>((((_e1817 / _e239) * 0.14f) + (_e1089 * 0.012f)), ((_e1823 * 0.14f) + 6.1f));
                    loop {
                        let _e1853 = phi_73_;
                        let _e1855 = phi_74_;
                        let _e1857 = phi_75_;
                        let _e1859 = phi_76_;
                        local_45 = _e1857;
                        let _e1860 = (_e1853 < 4i);
                        if _e1860 {
                            let _e1863 = cantus_render_shader_simplex_noise(_e1859);
                            phi_77_ = (_e1853 + 1i);
                            phi_78_ = (_e1855 * 0.5f);
                            phi_79_ = (_e1857 + (_e1863 * _e1855));
                            phi_80_ = vec2<f32>(((_e1859.x * 1.6f) + (_e1859.y * 1.2f)), ((_e1859.y * 1.6f) - (_e1859.x * 1.2f)));
                        } else {
                            phi_77_ = i32();
                            phi_78_ = f32();
                            phi_79_ = f32();
                            phi_80_ = vec2<f32>();
                        }
                        let _e1876 = phi_77_;
                        let _e1878 = phi_78_;
                        let _e1880 = phi_79_;
                        let _e1882 = phi_80_;
                        continue;
                        continuing {
                            phi_73_ = _e1876;
                            phi_74_ = _e1878;
                            phi_75_ = _e1880;
                            phi_76_ = _e1882;
                            break if !(_e1860);
                        }
                    }
                    let _e1885 = local_45;
                    let _e1888 = (((_e1885 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e1890 = select(_e1888, 0f, (_e1888 < 0f));
                    let _e1892 = select(_e1890, 1f, (_e1890 > 1f));
                    phi_81_ = ((((_e1892 * _e1892) * (3f - (2f * _e1892))) * _e840) * 0.82f);
                } else {
                    phi_81_ = 0f;
                }
                let _e1900 = phi_81_;
                let _e1902 = ((_e217.y - -0.02f) * 16.666668f);
                let _e1904 = select(_e1902, 0f, (_e1902 < 0f));
                let _e1906 = select(_e1904, 1f, (_e1904 > 1f));
                let _e1913 = (_e240 - _e1817);
                let _e1914 = (_e241 - (_e239 * _e1823));
                let _e1918 = sqrt(((_e1913 * _e1913) + (_e1914 * _e1914)));
                let _e1920 = ((_e1918 - 62f) * -0.01724138f);
                let _e1922 = select(_e1920, 0f, (_e1920 < 0f));
                let _e1924 = select(_e1922, 1f, (_e1922 > 1f));
                let _e1931 = ((_e1918 - 11f) * -0.1f);
                let _e1933 = select(_e1931, 0f, (_e1931 < 0f));
                let _e1935 = select(_e1933, 1f, (_e1933 > 1f));
                let _e1942 = (((((_e1924 * _e1924) * (3f - (2f * _e1924))) * 0.24f) + (((_e1935 * _e1935) * (3f - (2f * _e1935))) * 0.7f)) * (((_e1906 * _e1906) * (3f - (2f * _e1906))) * (1f - _e1900)));
                let _e1943 = (1f - _e1942);
                let _e1956 = ((_e248 - 1f) / ((_e239 * -0.25f) - 1f));
                let _e1958 = select(_e1956, 0f, (_e1956 < 0f));
                let _e1960 = select(_e1958, 1f, (_e1958 > 1f));
                let _e1964 = ((_e1960 * _e1960) * (3f - (2f * _e1960)));
                let _e1965 = (1f - _e1964);
                phi_82_ = vec3<f32>(((_e1808 * _e1965) + (((_e1808 * _e1943) + (((0.96f * _e1835) + (0.98f * _e1834)) * _e1942)) * _e1964)), ((_e1810 * _e1965) + (((_e1810 * _e1943) + (((0.98f * _e1835) + (0.74f * _e1834)) * _e1942)) * _e1964)), ((_e1812 * _e1965) + (((_e1812 * _e1943) + ((_e1835 + (0.66f * _e1834)) * _e1942)) * _e1964)));
            } else {
                phi_82_ = (_e1783 + vec3(_e1806));
            }
            let _e1977 = phi_82_;
            let _e1988 = local_46;
            let _e1989 = (1f - _e1988);
            let _e1994 = local_47;
            let _e1997 = local_48;
            let _e2000 = local_49;
            if (_e216.y < _e287) {
                phi_94_ = 0u;
                phi_95_ = u32();
                phi_96_ = true;
            } else {
                let _e2006 = (_e216.x - (_e235 - 158f));
                if (_e2006 >= 316f) {
                    if (_e296 < ((_e242 + 96f) * 0.5f)) {
                        phi_90_ = 4u;
                    } else {
                        let _e2051 = (_e543 + _e242);
                        if (_e296 > _e2051) {
                            let _e2067 = cantus_render_tempestas_cell_index(_e296, _e2051, 28f, 2f);
                            phi_89_ = ((76u + (_e2067 * 2u)) + select(0u, 1u, (_e296 > (_e2051 + (4f * (3.5f + (f32(_e2067) * 7f)))))));
                        } else {
                            let _e2053 = (_e296 > _e545);
                            let _e2054 = select(6u, 5u, _e2053);
                            let _e2061 = cantus_render_tempestas_cell_index(_e2006, 316f, (308f / f32(_e2054)), f32((_e2054 - 1u)));
                            phi_89_ = ((select(5u, 17u, _e2053) + (_e2061 * 2u)) + select(0u, 1u, (_e296 > select(_e541, _e543, _e2053))));
                        }
                        let _e2079 = phi_89_;
                        phi_90_ = _e2079;
                    }
                    let _e2081 = phi_90_;
                    phi_91_ = _e2081;
                    phi_92_ = u32();
                    phi_93_ = true;
                } else {
                    if (_e296 < 54f) {
                        let _e2021 = ((_e551 - 0.5295082f) * 4.1666665f);
                        let _e2023 = select(_e2021, 0f, (_e2021 < 0f));
                        let _e2025 = select(_e2023, 1f, (_e2023 > 1f));
                        let _e2030 = (126f * ((_e2025 * _e2025) * (3f - (2f * _e2025))));
                        if (abs((_e2006 - (154f - _e2030))) < 20f) {
                            phi_85_ = 2u;
                        } else {
                            phi_85_ = select(1u, 3u, (abs((_e2006 - (154f + _e2030))) < 20f));
                        }
                        let _e2041 = phi_85_;
                        phi_86_ = _e2041;
                        phi_87_ = u32();
                        phi_88_ = true;
                    } else {
                        let _e2009 = cantus_render_tempestas_cell_index(_e2006, 0f, 44f, 6f);
                        let _e2010 = (_e296 < 82f);
                        if _e2010 {
                            phi_83_ = (27u + _e2009);
                            phi_84_ = u32();
                        } else {
                            let _e2011 = cantus_render_tempestas_cell_index(_e296, 84f, 24f, 5f);
                            phi_83_ = u32();
                            phi_84_ = ((34u + (_e2011 * 7u)) + _e2009);
                        }
                        let _e2017 = phi_83_;
                        let _e2019 = phi_84_;
                        phi_86_ = _e2017;
                        phi_87_ = _e2019;
                        phi_88_ = _e2010;
                    }
                    let _e2043 = phi_86_;
                    let _e2045 = phi_87_;
                    let _e2047 = phi_88_;
                    phi_91_ = _e2043;
                    phi_92_ = _e2045;
                    phi_93_ = _e2047;
                }
                let _e2083 = phi_91_;
                let _e2085 = phi_92_;
                let _e2087 = phi_93_;
                phi_94_ = _e2083;
                phi_95_ = _e2085;
                phi_96_ = _e2087;
            }
            let _e2089 = phi_94_;
            let _e2091 = phi_95_;
            let _e2093 = phi_96_;
            let _e2094 = select(_e2091, _e2089, _e2093);
            if (_e2094 < arrayLength((&text_lines.member))) {
            } else {
                break;
            }
            let _e2099 = text_lines.member[_e2094].line;
            let _e2103 = text_lines.member[_e2094].color;
            let _e2104 = unpack4x8unorm(_e2103);
            if (_e216.x < _e2099.min.x) {
                phi_134_ = f32();
                phi_135_ = true;
            } else {
                if (_e216.x > _e2099.max.x) {
                    phi_132_ = f32();
                    phi_133_ = true;
                } else {
                    if (_e216.y < _e2099.min.y) {
                        phi_130_ = f32();
                        phi_131_ = true;
                    } else {
                        let _e2116 = (_e216.y > _e2099.max.y);
                        if _e2116 {
                            phi_129_ = f32();
                        } else {
                            phi_97_ = _e2099.count;
                            phi_98_ = 0u;
                            loop {
                                let _e2119 = phi_97_;
                                let _e2121 = phi_98_;
                                local_50 = _e2121;
                                let _e2122 = (_e2121 < _e2119);
                                if _e2122 {
                                    let _e2125 = (_e2121 + ((_e2119 - _e2121) / 2u));
                                    let _e2127 = (_e2099.first + _e2125);
                                    if (_e2127 < _e222) {
                                    } else {
                                        phi_102_ = true;
                                        break;
                                    }
                                    let _e2132 = placed_glyphs_2.member[_e2127].x;
                                    let _e2138 = (_e2132 <= ((_e216.x - _e2099.origin.x) / _e2099.size));
                                    if _e2138 {
                                        phi_99_ = (_e2125 + 1u);
                                    } else {
                                        phi_99_ = _e2121;
                                    }
                                    let _e2141 = phi_99_;
                                    phi_100_ = select(_e2125, _e2119, _e2138);
                                    phi_101_ = _e2141;
                                } else {
                                    phi_100_ = u32();
                                    phi_101_ = u32();
                                }
                                let _e2144 = phi_100_;
                                let _e2146 = phi_101_;
                                continue;
                                continuing {
                                    phi_97_ = _e2144;
                                    phi_98_ = _e2146;
                                    phi_102_ = _e479;
                                    break if !(_e2122);
                                }
                            }
                            let _e2149 = phi_102_;
                            if _e2149 {
                                break;
                            }
                            let _e2151 = local_50;
                            let _e2152 = (_e2151 + 1u);
                            phi_103_ = _e2149;
                            phi_104_ = select(_e2152, _e2099.count, (_e2099.count < _e2152));
                            phi_105_ = -1000000f;
                            loop {
                                let _e2156 = phi_103_;
                                let _e2158 = phi_104_;
                                let _e2160 = phi_105_;
                                local_53 = _e2160;
                                if (_e2158 > 0u) {
                                    let _e2162 = (_e2158 - 1u);
                                    let _e2164 = (_e2099.first + _e2162);
                                    if (_e2164 < _e222) {
                                    } else {
                                        phi_128_ = true;
                                        break;
                                    }
                                    let _e2169 = placed_glyphs_2.member[_e2164].x;
                                    let _e2173 = placed_glyphs_2.member[_e2164].glyph;
                                    if (_e2173 < arrayLength((&glyphs_2.member))) {
                                    } else {
                                        phi_128_ = true;
                                        break;
                                    }
                                    let _e2179 = glyphs_2.member[_e2173].min[0u];
                                    let _e2184 = glyphs_2.member[_e2173].min[1u];
                                    let _e2189 = glyphs_2.member[_e2173].max[0u];
                                    let _e2194 = glyphs_2.member[_e2173].max[1u];
                                    let _e2198 = glyphs_2.member[_e2173].start;
                                    let _e2202 = glyphs_2.member[_e2173].count;
                                    let _e2208 = (((_e216.x - _e2099.origin.x) / _e2099.size) - _e2169);
                                    let _e2213 = (-((_e216.y - _e2099.origin.y)) / _e2099.size);
                                    let _e2214 = (3.5f / _e2099.size);
                                    let _e2215 = (_e2189 + _e2214);
                                    let _e2216 = (_e2208 > _e2215);
                                    if _e2216 {
                                        phi_122_ = _e2156;
                                        phi_123_ = f32();
                                    } else {
                                        if (_e2208 >= (_e2179 - _e2214)) {
                                            if (_e2213 >= (_e2184 - _e2214)) {
                                                if (_e2208 <= _e2215) {
                                                    if (_e2213 <= (_e2194 + _e2214)) {
                                                        phi_106_ = 340282350000000000000000000000000000000f;
                                                        phi_107_ = 0u;
                                                        phi_108_ = 0i;
                                                        loop {
                                                            let _e2226 = phi_106_;
                                                            let _e2228 = phi_107_;
                                                            let _e2230 = phi_108_;
                                                            local_51 = _e2226;
                                                            local_52 = _e2230;
                                                            let _e2231 = (_e2228 < _e2202);
                                                            if _e2231 {
                                                                let _e2232 = (_e2198 + _e2228);
                                                                if (_e2232 < arrayLength((&edges_2.member))) {
                                                                } else {
                                                                    phi_112_ = true;
                                                                    break;
                                                                }
                                                                let _e2236 = edges_2.member[_e2232];
                                                                let _e2238 = cantus_render_text_edge_distance(_e2236, _e2099.weight, vec2<f32>(_e2208, _e2213), _e2226);
                                                                phi_109_ = _e2238.member;
                                                                phi_110_ = (_e2228 + 1u);
                                                                phi_111_ = (_e2230 + _e2238.member_1);
                                                            } else {
                                                                phi_109_ = f32();
                                                                phi_110_ = u32();
                                                                phi_111_ = i32();
                                                            }
                                                            let _e2244 = phi_109_;
                                                            let _e2246 = phi_110_;
                                                            let _e2248 = phi_111_;
                                                            continue;
                                                            continuing {
                                                                phi_106_ = _e2244;
                                                                phi_107_ = _e2246;
                                                                phi_108_ = _e2248;
                                                                phi_112_ = _e2156;
                                                                break if !(_e2231);
                                                            }
                                                        }
                                                        let _e2251 = phi_112_;
                                                        phi_128_ = _e2251;
                                                        if _e2251 {
                                                            break;
                                                        }
                                                        let _e2253 = local_51;
                                                        let _e2257 = local_52;
                                                        let _e2260 = ((sqrt(_e2253) * _e2099.size) * select(1f, -1f, (_e2257 == 0i)));
                                                        if (_e2160 != _e2160) {
                                                            phi_113_ = true;
                                                        } else {
                                                            phi_113_ = (_e2260 >= _e2160);
                                                        }
                                                        let _e2264 = phi_113_;
                                                        phi_114_ = _e2251;
                                                        phi_115_ = select(_e2160, _e2260, _e2264);
                                                    } else {
                                                        phi_114_ = _e2156;
                                                        phi_115_ = _e2160;
                                                    }
                                                    let _e2267 = phi_114_;
                                                    let _e2269 = phi_115_;
                                                    phi_116_ = _e2267;
                                                    phi_117_ = _e2269;
                                                } else {
                                                    phi_116_ = _e2156;
                                                    phi_117_ = _e2160;
                                                }
                                                let _e2271 = phi_116_;
                                                let _e2273 = phi_117_;
                                                phi_118_ = _e2271;
                                                phi_119_ = _e2273;
                                            } else {
                                                phi_118_ = _e2156;
                                                phi_119_ = _e2160;
                                            }
                                            let _e2275 = phi_118_;
                                            let _e2277 = phi_119_;
                                            phi_120_ = _e2275;
                                            phi_121_ = _e2277;
                                        } else {
                                            phi_120_ = _e2156;
                                            phi_121_ = _e2160;
                                        }
                                        let _e2279 = phi_120_;
                                        let _e2281 = phi_121_;
                                        phi_122_ = _e2279;
                                        phi_123_ = _e2281;
                                    }
                                    let _e2283 = phi_122_;
                                    let _e2285 = phi_123_;
                                    phi_124_ = _e2283;
                                    phi_125_ = _e2162;
                                    phi_126_ = _e2285;
                                    phi_127_ = select(true, false, _e2216);
                                } else {
                                    phi_124_ = _e2156;
                                    phi_125_ = u32();
                                    phi_126_ = f32();
                                    phi_127_ = false;
                                }
                                let _e2288 = phi_124_;
                                let _e2290 = phi_125_;
                                let _e2292 = phi_126_;
                                let _e2294 = phi_127_;
                                continue;
                                continuing {
                                    phi_103_ = _e2288;
                                    phi_104_ = _e2290;
                                    phi_105_ = _e2292;
                                    phi_128_ = _e2288;
                                    break if !(_e2294);
                                }
                            }
                            let _e2297 = phi_128_;
                            if _e2297 {
                                break;
                            }
                            let _e2299 = local_53;
                            let _e2301 = ((_e2299 * 1.25f) + 0.5f);
                            let _e2303 = select(_e2301, 0f, (_e2301 < 0f));
                            let _e2305 = select(_e2303, 1f, (_e2303 > 1f));
                            phi_129_ = ((_e2305 * _e2305) * (3f - (2f * _e2305)));
                        }
                        let _e2311 = phi_129_;
                        phi_130_ = _e2311;
                        phi_131_ = _e2116;
                    }
                    let _e2313 = phi_130_;
                    let _e2315 = phi_131_;
                    phi_132_ = _e2313;
                    phi_133_ = _e2315;
                }
                let _e2317 = phi_132_;
                let _e2319 = phi_133_;
                phi_134_ = _e2317;
                phi_135_ = _e2319;
            }
            let _e2321 = phi_134_;
            let _e2323 = phi_135_;
            let _e2326 = (select(_e2321, 0f, _e2323) * _e2104.w);
            let _e2327 = (1f - _e2326);
            out_color = vec4<f32>((((((_e1977.x * _e1989) + (((_e1977.x * 1.5f) + 0.1f) * _e1994)) * _e2327) + (_e2104.x * _e2326)) * _e789), (((((_e1977.y * _e1989) + (((_e1977.y * 1.5f) + 0.1f) * _e1997)) * _e2327) + (_e2104.y * _e2326)) * _e789), (((((_e1977.z * _e1989) + (((_e1977.z * 1.5f) + 0.1f) * _e2000)) * _e2327) + (_e2104.z * _e2326)) * _e789), _e802);
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
