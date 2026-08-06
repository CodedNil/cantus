struct render_shared_RipplePulse {
    origin: vec2<f32>,
    animation: vec2<f32>,
}

struct render_shared_FrameData {
    screen_size: vec2<f32>,
    panel_top: f32,
    panel_height: f32,
    mouse_pos: vec2<f32>,
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

struct render_tempo_WeatherCondition {
    fog: f32,
    cloud: f32,
    rain: f32,
    snow: f32,
    lightning: f32,
    hail: f32,
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

struct render_text_Text_76_u0020_512_ {
    lines: array<render_text_Line, 76>,
    glyphs: array<render_text_PlacedGlyph, 512>,
    line_count: u32,
}

struct render_tempo_WeatherSurface {
    x: f32,
    calendar_expansion: f32,
    sun_hours: array<f32, 2>,
    hourly_start: f32,
    today_index: i32,
    month_range: array<u32, 2>,
    text_hover: array<f32, 3>,
    hourly_conditions: array<render_tempo_WeatherCondition, 6>,
    daily_conditions: array<render_tempo_WeatherCondition, 5>,
    text: render_text_Text_76_u0020_512_,
}

struct type_16 {
    member: array<render_tempo_WeatherSurface>,
}

struct render_tempo_Varyings {
    weather: vec4<f32>,
    pixel: vec2<f32>,
}

struct isthmus_Vertex_render_tempo_Varyings {
    varyings: render_tempo_Varyings,
    position: vec4<f32>,
}

struct render_text_Glyph {
    min: vec2<f32>,
    max: vec2<f32>,
    start: u32,
    count: u32,
}

struct type_19 {
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

struct type_21 {
    member: array<render_text_Edge>,
}

struct u0028_isthmus_glam_Vec2_u0020_f32_u0029_ {
    unnamed: vec2<f32>,
    unnamed_1: f32,
}

struct render_track_PillIconRow {
    center: vec2<f32>,
    count: f32,
    expansion: f32,
}

struct u0028_render_tempo_WeatherCondition_u0020_f32_u0029_ {
    unnamed: render_tempo_WeatherCondition,
    unnamed_1: f32,
}

struct u0028_f32_u0020_i32_u0029_ {
    member: f32,
    member_1: i32,
}

struct render_track_PaletteColor {
    rgb: u32,
    weight: f32,
}

struct render_track_TrackEffects {
    acousticness: f32,
    valence: f32,
    instrumentalness: f32,
    turbulence: f32,
    seed: f32,
    beat: f32,
    flow_time: f32,
}

struct render_text_Text_2_u0020_128_ {
    lines: array<render_text_Line, 2>,
    glyphs: array<render_text_PlacedGlyph, 128>,
    line_count: u32,
}

struct render_track_TrackPill {
    x: f32,
    width: f32,
    colors: array<render_track_PaletteColor, 4>,
    image_index: i32,
    rating: i32,
    primary_playlist_count: u32,
    secondary_playlist_count: u32,
    visibility: f32,
    primary_alpha: f32,
    secondary_expansion: f32,
    effects: render_track_TrackEffects,
    playlist_images: array<i32, 8>,
    text: render_text_Text_2_u0020_128_,
}

struct type_28 {
    member: array<render_track_TrackPill>,
}

struct render_status_UsageHistory {
    samples: array<f32, 40>,
}

struct render_status_ProcessorStatus {
    temperature: f32,
    usage: render_status_UsageHistory,
    memory: render_status_UsageHistory,
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
    power_state: f32,
    power_hover: u32,
    sun_height: f32,
    conditions: render_tempo_WeatherCondition,
    text: render_text_Text_2_u0020_32_,
}

struct type_36 {
    member: array<render_status_StatusPill>,
}

struct render_playhead_PlayheadState {
    bar_split: f32,
    icon_presence: f32,
    icon_morph: f32,
}

struct type_38 {
    member: array<render_playhead_PlayheadState>,
}

struct render_particles_Particle {
    spawn_pos: vec2<f32>,
    spawn_vel: vec2<f32>,
    end_time: f32,
    duration: f32,
    rgb: u32,
}

struct type_40 {
    member: array<render_particles_Particle>,
}

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec2<f32>,
    @location(1) @interpolate(flat) member_2: vec4<f32>,
    @location(2) @interpolate(flat) member_3: u32,
}

struct VertexOutput_1 {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec2<f32>,
    @location(1) @interpolate(flat) member_2: u32,
}

struct VertexOutput_2 {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec4<f32>,
    @location(1) member_2: vec2<f32>,
}

var<private> vertex_5: u32;
@group(0) @binding(0)
var<storage> frame: type_6;
@group(0) @binding(1)
var<storage> pill: type_16;
var<private> _isthmus_instance_index_7: u32;
var<private> out_position: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> out_pixel: vec2<f32>;
var<private> out_weather: vec4<f32>;
var<private> out_isthmus_instance_index: u32;
var<private> pixel_2: vec2<f32>;
var<private> weather_1: vec4<f32>;
var<private> _isthmus_instance_index_8: u32;
@group(0) @binding(2)
var<storage> glyphs: type_19;
@group(0) @binding(3)
var<storage> edges: type_21;
var<private> out_color: vec4<f32>;
var<private> instance_1: u32;
@group(0) @binding(1)
var<storage> pill_1: type_28;
var<private> out_pixel_pos: vec2<f32>;
var<private> out_pill_idx: u32;
var<private> pixel_pos_1: vec2<f32>;
var<private> pill_idx_1: u32;
@group(0) @binding(5)
var sampler_: sampler;
@group(0) @binding(4)
var images: texture_2d_array<f32>;
@group(0) @binding(1)
var<storage> pill_2: type_36;
var<private> out_isthmus_instance_index_1: u32;
var<private> _isthmus_instance_index_9: u32;
var<private> out_world_pos: vec2<f32>;
var<private> world_pos_1: vec2<f32>;
@group(0) @binding(1)
var<storage> state: type_38;
@group(0) @binding(1)
var<storage> particle: type_40;
var<private> out_uv: vec2<f32>;
var<private> color_1: vec4<f32>;
var<private> uv_1: vec2<f32>;

fn cantus_render_shader_pixel_to_ndc(param: vec2<f32>, param_1: vec2<f32>) -> vec4<f32> {
    return vec4<f32>((((param.x / param_1.x) * 2f) - 1f), (1f - ((param.y / param_1.y) * 2f)), 0f, 1f);
}

fn render_tempo_vertex_impl() {
    var phi_11779_: array<f32, 2>;
    var phi_11782_: array<f32, 2>;
    var phi_11783_: bool;
    var phi_11796_: f32;
    var phi_11806_: array<f32, 2>;
    var phi_11831_: array<f32, 2>;
    var phi_11834_: array<f32, 2>;
    var phi_11835_: bool;
    var phi_11848_: f32;
    var phi_11858_: array<f32, 2>;

    let _e28 = vertex_5;
    let _e29 = _isthmus_instance_index_7;
    let _e33 = pill.member[_e29].x;
    let _e37 = pill.member[_e29].calendar_expansion;
    let _e39 = select(_e37, 0f, (_e37 < 0f));
    let _e41 = select(_e39, 1f, (_e39 > 1f));
    let _e45 = ((_e41 * _e41) * (3f - (2f * _e41)));
    let _e49 = frame.member[0u].weather_hour;
    let _e53 = pill.member[_e29].sun_hours;
    let _e56 = (_e53[1] - _e53[0]);
    if (_e49 >= _e53[0]) {
        let _e58 = (_e49 <= _e53[1]);
        if _e58 {
            let _e60 = ((_e49 - _e53[0]) / _e56);
            phi_11779_ = array<f32, 2>(_e60, sin((_e60 * 3.1415927f)));
        } else {
            phi_11779_ = array<f32, 2>();
        }
        let _e65 = phi_11779_;
        phi_11782_ = _e65;
        phi_11783_ = select(true, false, _e58);
    } else {
        phi_11782_ = array<f32, 2>();
        phi_11783_ = true;
    }
    let _e68 = phi_11782_;
    let _e70 = phi_11783_;
    if _e70 {
        let _e71 = (24f - _e56);
        if (_e49 < _e53[0]) {
            phi_11796_ = (((_e49 + 24f) - _e53[1]) / _e71);
        } else {
            phi_11796_ = ((_e49 - _e53[1]) / _e71);
        }
        let _e79 = phi_11796_;
        phi_11806_ = array<f32, 2>(select(0f, 1f, (_e49 >= _e53[1])), -(sin((_e79 * 3.1415927f))));
    } else {
        phi_11806_ = _e68;
    }
    let _e87 = phi_11806_;
    if (12f >= _e53[0]) {
        let _e91 = (12f <= _e53[1]);
        if _e91 {
            let _e93 = ((12f - _e53[0]) / _e56);
            phi_11831_ = array<f32, 2>(_e93, sin((_e93 * 3.1415927f)));
        } else {
            phi_11831_ = array<f32, 2>();
        }
        let _e98 = phi_11831_;
        phi_11834_ = _e98;
        phi_11835_ = select(true, false, _e91);
    } else {
        phi_11834_ = array<f32, 2>();
        phi_11835_ = true;
    }
    let _e101 = phi_11834_;
    let _e103 = phi_11835_;
    if _e103 {
        let _e104 = (24f - _e56);
        if (12f < _e53[0]) {
            phi_11848_ = ((36f - _e53[1]) / _e104);
        } else {
            phi_11848_ = ((12f - _e53[1]) / _e104);
        }
        let _e111 = phi_11848_;
        phi_11858_ = array<f32, 2>(select(0f, 1f, (12f >= _e53[1])), -(sin((_e111 * 3.1415927f))));
    } else {
        phi_11858_ = _e101;
    }
    let _e119 = phi_11858_;
    let _e130 = frame.member[0u].panel_top;
    let _e140 = frame.member[0u].panel_height;
    let _e146 = (((_e33 - (_e45 * 158f)) - 48f) + (f32((_e28 & 1u)) * ((316f * _e45) + 404f)));
    let _e147 = ((_e130 - 48f) + (f32((_e28 >> bitcast<u32>(1i))) * ((244f * _e45) + (_e140 + 96f))));
    let _e152 = frame.member[0u].screen_size[0u];
    let _e157 = frame.member[0u].screen_size[1u];
    let _e160 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e146, _e147), vec2<f32>(_e152, _e157));
    out_position = _e160;
    out_pixel[0u] = _e146;
    out_pixel[1u] = _e147;
    out_weather = vec4<f32>(_e87[0], _e87[1], _e119[1], _e45);
    out_isthmus_instance_index = _e29;
    return;
}

fn cantus_render_text_edge_distance(param_2: render_text_Edge, param_3: f32, param_4: vec2<f32>) -> u0028_f32_u0020_i32_u0029_ {
    var phi_14781_: bool;
    var phi_14796_: bool;
    var phi_14811_: bool;
    var phi_4395_: f32;
    var phi_4398_: i32;
    var phi_4444_: f32;
    var phi_14852_: bool;
    var phi_14867_: bool;
    var phi_4396_: f32;
    var phi_4399_: i32;
    var phi_14882_: bool;
    var local: f32;
    var local_1: f32;
    var local_2: f32;
    var local_3: f32;
    var local_4: f32;
    var local_5: f32;
    var phi_14923_: bool;
    var phi_4478_: i32;
    var phi_4481_: vec2<f32>;
    var phi_4483_: i32;
    var phi_4535_: i32;
    var phi_4536_: i32;
    var phi_4524_: i32;
    var phi_4525_: i32;
    var phi_4537_: i32;
    var phi_4479_: i32;
    var phi_4482_: vec2<f32>;
    var phi_4484_: i32;
    var local_6: i32;

    let _e30 = (param_2.low_start.x + ((param_2.high_start.x - param_2.low_start.x) * param_3));
    let _e31 = (param_2.low_start.y + ((param_2.high_start.y - param_2.low_start.y) * param_3));
    let _e44 = (param_2.low_control.x + ((param_2.high_control.x - param_2.low_control.x) * param_3));
    let _e45 = (param_2.low_control.y + ((param_2.high_control.y - param_2.low_control.y) * param_3));
    let _e58 = (param_2.low_end.x + ((param_2.high_end.x - param_2.low_end.x) * param_3));
    let _e59 = (param_2.low_end.y + ((param_2.high_end.y - param_2.low_end.y) * param_3));
    let _e60 = (_e58 - _e30);
    let _e61 = (_e59 - _e31);
    let _e62 = (param_4.x - _e30);
    let _e63 = (param_4.y - _e31);
    let _e69 = ((_e60 * _e60) + (_e61 * _e61));
    if (_e69 != _e69) {
        phi_14781_ = true;
    } else {
        phi_14781_ = (0.00000001f >= _e69);
    }
    let _e73 = phi_14781_;
    let _e75 = (((_e62 * _e60) + (_e63 * _e61)) / select(_e69, 0.00000001f, _e73));
    if (_e75 != _e75) {
        phi_14796_ = true;
    } else {
        phi_14796_ = (0f >= _e75);
    }
    let _e79 = phi_14796_;
    let _e80 = select(_e75, 0f, _e79);
    if (_e80 != _e80) {
        phi_14811_ = true;
    } else {
        phi_14811_ = (1f <= _e80);
    }
    let _e84 = phi_14811_;
    phi_4395_ = select(_e80, 1f, _e84);
    phi_4398_ = 0i;
    loop {
        let _e95 = phi_4395_;
        let _e97 = phi_4398_;
        local = _e95;
        local_1 = _e95;
        local_2 = _e95;
        local_3 = _e95;
        local_4 = _e95;
        local_5 = _e95;
        let _e98 = (_e97 < 3i);
        if _e98 {
            let _e99 = (1f - _e95);
            let _e105 = ((2f * _e99) * _e95);
            let _e126 = ((((_e44 - _e30) * _e99) + ((_e58 - _e44) * _e95)) * 2f);
            let _e127 = ((((_e45 - _e31) * _e99) + ((_e59 - _e45) * _e95)) * 2f);
            let _e128 = (((((_e30 * _e99) * _e99) + (_e44 * _e105)) + ((_e58 * _e95) * _e95)) - param_4.x);
            let _e129 = (((((_e31 * _e99) * _e99) + (_e45 * _e105)) + ((_e59 * _e95) * _e95)) - param_4.y);
            let _e136 = (((_e126 * _e126) + (_e127 * _e127)) + ((_e128 * (((_e30 - (_e44 * 2f)) + _e58) * 2f)) + (_e129 * (((_e31 - (_e45 * 2f)) + _e59) * 2f))));
            if (abs(_e136) < 0.00000001f) {
                phi_4444_ = select(0.00000001f, -0.00000001f, (_e136 < 0f));
            } else {
                phi_4444_ = _e136;
            }
            let _e142 = phi_4444_;
            let _e147 = (_e95 - (((_e128 * _e126) + (_e129 * _e127)) / _e142));
            if (_e147 != _e147) {
                phi_14852_ = true;
            } else {
                phi_14852_ = (0f >= _e147);
            }
            let _e151 = phi_14852_;
            let _e152 = select(_e147, 0f, _e151);
            if (_e152 != _e152) {
                phi_14867_ = true;
            } else {
                phi_14867_ = (1f <= _e152);
            }
            let _e156 = phi_14867_;
            phi_4396_ = select(_e152, 1f, _e156);
            phi_4399_ = (_e97 + 1i);
        } else {
            phi_4396_ = f32();
            phi_4399_ = i32();
        }
        let _e160 = phi_4396_;
        let _e162 = phi_4399_;
        continue;
        continuing {
            phi_4395_ = _e160;
            phi_4398_ = _e162;
            break if !(_e98);
        }
    }
    let _e166 = ((_e62 * _e62) + (_e63 * _e63));
    let _e167 = (param_4.x - _e58);
    let _e168 = (param_4.y - _e59);
    let _e171 = ((_e167 * _e167) + (_e168 * _e168));
    if (_e166 != _e166) {
        phi_14882_ = true;
    } else {
        phi_14882_ = (_e171 <= _e166);
    }
    let _e175 = phi_14882_;
    let _e176 = select(_e166, _e171, _e175);
    let _e179 = local;
    let _e180 = (1f - _e179);
    let _e187 = local_1;
    let _e188 = ((2f * _e180) * _e187);
    let _e194 = local_2;
    let _e197 = local_3;
    let _e200 = local_4;
    let _e203 = local_5;
    let _e207 = (param_4.x - ((((_e30 * _e180) * _e180) + (_e44 * _e188)) + ((_e58 * _e194) * _e200)));
    let _e208 = (param_4.y - ((((_e31 * _e180) * _e180) + (_e45 * _e188)) + ((_e59 * _e197) * _e203)));
    let _e211 = ((_e207 * _e207) + (_e208 * _e208));
    if (_e176 != _e176) {
        phi_14923_ = true;
    } else {
        phi_14923_ = (_e211 <= _e176);
    }
    let _e215 = phi_14923_;
    phi_4478_ = 1i;
    phi_4481_ = vec2<f32>(_e30, _e31);
    phi_4483_ = 0i;
    loop {
        let _e218 = phi_4478_;
        let _e220 = phi_4481_;
        let _e222 = phi_4483_;
        local_6 = _e222;
        let _e223 = (_e218 <= 3i);
        if _e223 {
            let _e225 = (f32(_e218) * 0.33333334f);
            let _e226 = (1f - _e225);
            let _e232 = ((2f * _e226) * _e225);
            let _e241 = ((((_e30 * _e226) * _e226) + (_e44 * _e232)) + ((_e58 * _e225) * _e225));
            let _e242 = ((((_e31 * _e226) * _e226) + (_e45 * _e232)) + ((_e59 * _e225) * _e225));
            let _e251 = (((_e241 - _e220.x) * (param_4.y - _e220.y)) - ((_e242 - _e220.y) * (param_4.x - _e220.x)));
            if (_e220.y <= param_4.y) {
                if (_e242 > param_4.y) {
                    if (_e251 > 0f) {
                        phi_4524_ = (_e222 + 1i);
                    } else {
                        phi_4524_ = _e222;
                    }
                    let _e264 = phi_4524_;
                    phi_4525_ = _e264;
                } else {
                    phi_4525_ = _e222;
                }
                let _e266 = phi_4525_;
                phi_4537_ = _e266;
            } else {
                if (_e242 <= param_4.y) {
                    if (_e251 < 0f) {
                        phi_4535_ = (_e222 - 1i);
                    } else {
                        phi_4535_ = _e222;
                    }
                    let _e257 = phi_4535_;
                    phi_4536_ = _e257;
                } else {
                    phi_4536_ = _e222;
                }
                let _e259 = phi_4536_;
                phi_4537_ = _e259;
            }
            let _e268 = phi_4537_;
            phi_4479_ = (_e218 + 1i);
            phi_4482_ = vec2<f32>(_e241, _e242);
            phi_4484_ = _e268;
        } else {
            phi_4479_ = i32();
            phi_4482_ = vec2<f32>();
            phi_4484_ = i32();
        }
        let _e272 = phi_4479_;
        let _e274 = phi_4482_;
        let _e276 = phi_4484_;
        continue;
        continuing {
            phi_4478_ = _e272;
            phi_4481_ = _e274;
            phi_4483_ = _e276;
            break if !(_e223);
        }
    }
    let _e279 = local_6;
    return u0028_f32_u0020_i32_u0029_(select(_e176, _e211, _e215), _e279);
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
    var phi_14736_: bool;
    var phi_14751_: bool;
    var phi_14766_: bool;

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
        phi_14736_ = true;
    } else {
        phi_14736_ = (0f >= _e40);
    }
    let _e44 = phi_14736_;
    let _e45 = select(_e40, 0f, _e44);
    let _e50 = cantus_render_shader_hash(vec2<f32>(_e18, _e19));
    let _e64 = (0.5f - ((_e33 * _e33) + (_e34 * _e34)));
    if (_e64 != _e64) {
        phi_14751_ = true;
    } else {
        phi_14751_ = (0f >= _e64);
    }
    let _e68 = phi_14751_;
    let _e69 = select(_e64, 0f, _e68);
    let _e76 = cantus_render_shader_hash(vec2<f32>((_e18 + _e28.x), (_e19 + _e28.y)));
    let _e91 = (0.5f - ((_e35 * _e35) + (_e36 * _e36)));
    if (_e91 != _e91) {
        phi_14766_ = true;
    } else {
        phi_14766_ = (0f >= _e91);
    }
    let _e95 = phi_14766_;
    let _e96 = select(_e91, 0f, _e95);
    let _e103 = cantus_render_shader_hash(vec2<f32>((_e18 + 1f), (_e19 + 1f)));
    return (70f * ((((((_e45 * _e45) * _e45) * _e45) * ((_e24 * ((_e50.x * 2f) - 1f)) + (_e25 * ((_e50.y * 2f) - 1f)))) + ((((_e69 * _e69) * _e69) * _e69) * ((_e33 * ((_e76.x * 2f) - 1f)) + (_e34 * ((_e76.y * 2f) - 1f))))) + ((((_e96 * _e96) * _e96) * _e96) * ((_e35 * ((_e103.x * 2f) - 1f)) + (_e36 * ((_e103.y * 2f) - 1f))))));
}

fn cantus_render_shader_sd_rounded_box(param_7: vec2<f32>, param_8: vec2<f32>, param_9: f32) -> f32 {
    var phi_14677_: bool;
    var phi_14692_: bool;

    let _e13 = ((abs(param_7.x) - param_8.x) + param_9);
    let _e14 = ((abs(param_7.y) - param_8.y) + param_9);
    let _e16 = select(0f, _e13, (_e13 > 0f));
    let _e18 = select(0f, _e14, (_e14 > 0f));
    if (_e13 != _e13) {
        phi_14677_ = true;
    } else {
        phi_14677_ = (_e14 >= _e13);
    }
    let _e26 = phi_14677_;
    let _e27 = select(_e13, _e14, _e26);
    if (_e27 != _e27) {
        phi_14692_ = true;
    } else {
        phi_14692_ = (0f <= _e27);
    }
    let _e31 = phi_14692_;
    return ((sqrt(((_e16 * _e16) + (_e18 * _e18))) + select(_e27, 0f, _e31)) - param_9);
}

fn cantus_render_shader_sd_capsule_box(param_10: vec2<f32>, param_11: f32, param_12: f32) -> f32 {
    var phi_14647_: bool;
    var phi_14662_: bool;

    let _e8 = abs(param_10.y);
    let _e9 = (abs(param_10.x) - param_11);
    let _e11 = select(0f, _e9, (_e9 > 0f));
    let _e13 = select(0f, _e8, (_e8 > 0f));
    if (_e9 != _e9) {
        phi_14647_ = true;
    } else {
        phi_14647_ = (_e8 >= _e9);
    }
    let _e21 = phi_14647_;
    let _e22 = select(_e9, _e8, _e21);
    if (_e22 != _e22) {
        phi_14662_ = true;
    } else {
        phi_14662_ = (0f <= _e22);
    }
    let _e26 = phi_14662_;
    return ((sqrt(((_e11 * _e11) + (_e13 * _e13))) + select(_e22, 0f, _e26)) - param_12);
}

fn render_tempo_fragment_impl() {
    var phi_836_: u32;
    var phi_837_: u32;
    var phi_14967_: bool;
    var phi_11953_: bool;
    var phi_11968_: bool;
    var phi_11983_: bool;
    var phi_1111_: vec2<f32>;
    var phi_1114_: f32;
    var phi_1116_: u32;
    var phi_12011_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_12022_: bool;
    var phi_1112_: vec2<f32>;
    var phi_1115_: f32;
    var phi_1117_: u32;
    var phi_14977_: bool;
    var phi_1258_: f32;
    var local_7: vec2<f32>;
    var local_8: vec2<f32>;
    var phi_12086_: bool;
    var phi_1414_: f32;
    var phi_1428_: f32;
    var phi_1451_: f32;
    var phi_12104_: bool;
    var phi_12119_: bool;
    var phi_12134_: bool;
    var phi_12149_: bool;
    var phi_12233_: bool;
    var phi_12248_: bool;
    var phi_12263_: bool;
    var phi_1719_: vec2<f32>;
    var phi_12278_: bool;
    var phi_12332_: array<f32, 2>;
    var phi_12335_: array<f32, 2>;
    var phi_12336_: bool;
    var phi_12349_: f32;
    var phi_12359_: array<f32, 2>;
    var phi_12417_: array<f32, 2>;
    var phi_12420_: array<f32, 2>;
    var phi_12421_: bool;
    var phi_12434_: f32;
    var phi_12444_: array<f32, 2>;
    var phi_1864_: u0028_render_tempo_WeatherCondition_u0020_f32_u0029_;
    var phi_12463_: bool;
    var phi_12520_: i32;
    var phi_12521_: f32;
    var phi_12522_: f32;
    var phi_12523_: vec2<f32>;
    var phi_12548_: i32;
    var phi_12549_: f32;
    var phi_12550_: f32;
    var phi_12551_: vec2<f32>;
    var local_9: f32;
    var phi_12562_: i32;
    var phi_12563_: f32;
    var phi_12564_: f32;
    var phi_12565_: vec2<f32>;
    var phi_12590_: i32;
    var phi_12591_: f32;
    var phi_12592_: f32;
    var phi_12593_: vec2<f32>;
    var local_10: f32;
    var local_11: f32;
    var phi_2315_: vec3<f32>;
    var phi_2525_: vec3<f32>;
    var phi_2723_: vec3<f32>;
    var phi_2921_: vec3<f32>;
    var phi_12604_: i32;
    var phi_12605_: f32;
    var phi_12606_: f32;
    var phi_12607_: vec2<f32>;
    var phi_12632_: i32;
    var phi_12633_: f32;
    var phi_12634_: f32;
    var phi_12635_: vec2<f32>;
    var local_12: f32;
    var phi_3009_: vec3<f32>;
    var phi_12659_: i32;
    var phi_12660_: f32;
    var phi_12661_: f32;
    var phi_12662_: vec2<f32>;
    var phi_12687_: i32;
    var phi_12688_: f32;
    var phi_12689_: f32;
    var phi_12690_: vec2<f32>;
    var local_13: f32;
    var phi_3151_: f32;
    var phi_3269_: vec3<f32>;
    var phi_3519_: f32;
    var phi_3520_: u32;
    var phi_3521_: f32;
    var phi_3522_: u32;
    var phi_3421_: bool;
    var phi_3426_: f32;
    var phi_3427_: u32;
    var phi_3428_: f32;
    var phi_3429_: vec3<f32>;
    var phi_3305_: u32;
    var phi_3430_: f32;
    var phi_3431_: u32;
    var phi_3432_: f32;
    var phi_3433_: vec3<f32>;
    var phi_3523_: f32;
    var phi_3524_: u32;
    var phi_3525_: f32;
    var phi_3526_: vec3<f32>;
    var phi_3555_: u32;
    var phi_3556_: f32;
    var phi_3557_: vec3<f32>;
    var phi_3558_: vec2<f32>;
    var phi_3560_: u32;
    var phi_3561_: f32;
    var phi_3562_: vec3<f32>;
    var phi_3563_: vec2<f32>;
    var phi_3564_: bool;
    var phi_3640_: u32;
    var phi_3643_: u32;
    var phi_3677_: u32;
    var phi_3641_: u32;
    var phi_3644_: u32;
    var phi_15000_: bool;
    var local_14: u32;
    var phi_15203_: bool;
    var phi_3685_: u32;
    var phi_3688_: f32;
    var phi_3775_: u32;
    var phi_3778_: i32;
    var phi_3780_: f32;
    var phi_12747_: bool;
    var phi_3776_: u32;
    var phi_3779_: i32;
    var phi_3781_: f32;
    var phi_15200_: bool;
    var local_15: f32;
    var local_16: i32;
    var phi_12762_: bool;
    var phi_15213_: bool;
    var phi_3812_: f32;
    var phi_15212_: bool;
    var phi_3813_: f32;
    var phi_15211_: bool;
    var phi_3814_: f32;
    var phi_15210_: bool;
    var phi_3815_: f32;
    var phi_15209_: bool;
    var phi_3816_: f32;
    var phi_15208_: bool;
    var phi_3686_: u32;
    var phi_3689_: f32;
    var phi_3818_: bool;
    var phi_15207_: bool;
    var phi_3823_: f32;
    var phi_3824_: f32;
    var phi_3825_: bool;
    var phi_3826_: f32;
    var phi_3827_: bool;
    var phi_3828_: f32;
    var phi_3829_: bool;
    var local_17: f32;
    var local_18: f32;
    var local_19: f32;
    var local_20: f32;
    var local_21: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e220 = pixel_2;
            let _e221 = weather_1;
            let _e222 = _isthmus_instance_index_8;
            let _e235 = pill.member[_e222].x;
            let _e239 = frame.member[0u].panel_height;
            let _e243 = frame.member[0u].panel_top;
            let _e244 = (_e220.x - _e235);
            let _e245 = (_e220.y - _e243);
            let _e246 = (_e239 * 0.5f);
            let _e250 = ((308f - _e239) * 0.5f);
            let _e252 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e244 - 154f), (_e245 - _e246)), _e250, _e246);
            let _e257 = frame.member[0u].mouse_pos[0u];
            let _e262 = frame.member[0u].mouse_pos[1u];
            let _e268 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e257 - _e235) - 154f), ((_e262 - _e243) - _e246)), _e250, _e246);
            phi_836_ = 0u;
            loop {
                let _e270 = phi_836_;
                let _e271 = (_e270 < 4u);
                if _e271 {
                    if _e271 {
                    } else {
                        phi_14967_ = true;
                        break;
                    }
                    phi_837_ = (_e270 + 1u);
                } else {
                    phi_837_ = u32();
                }
                let _e274 = phi_837_;
                continue;
                continuing {
                    phi_836_ = _e274;
                    phi_14967_ = false;
                    break if !(_e271);
                }
            }
            let _e277 = phi_14967_;
            if _e277 {
                break;
            }
            let _e281 = frame.member[0u].mouse_pressure;
            let _e284 = (_e243 + _e239);
            let _e287 = (244f * _e221.w);
            let _e289 = (_e235 - (_e221.w * 158f));
            let _e291 = (_e220.y - _e284);
            let _e292 = (_e235 - 158f);
            let _e293 = (_e220.x - _e292);
            let _e294 = (8f * _e221.w);
            let _e295 = (_e287 - _e294);
            if (_e295 != _e295) {
                phi_11953_ = true;
            } else {
                phi_11953_ = (0f >= _e295);
            }
            let _e299 = phi_11953_;
            let _e301 = ((308f + (316f * _e221.w)) * 0.5f);
            let _e302 = (select(_e295, 0f, _e299) * 0.5f);
            let _e303 = (_e294 + _e302);
            let _e306 = (_e302 != _e302);
            if _e306 {
                phi_11968_ = true;
            } else {
                phi_11968_ = (18f <= _e302);
            }
            let _e309 = phi_11968_;
            let _e312 = vec2<f32>(_e301, _e302);
            let _e313 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e220.x - _e289) - _e301), (_e291 - _e303)), _e312, select(_e302, 18f, _e309));
            let _e315 = (_e262 - _e284);
            if _e306 {
                phi_11983_ = true;
            } else {
                phi_11983_ = (18f <= _e302);
            }
            let _e320 = phi_11983_;
            let _e323 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e257 - _e289) - _e301), (_e315 - _e303)), _e312, select(_e302, 18f, _e320));
            let _e326 = (0.5f + ((_e313 - _e252) * 0.015625f));
            let _e328 = select(_e326, 0f, (_e326 < 0f));
            let _e330 = select(_e328, 1f, (_e328 > 1f));
            let _e343 = (0.5f + ((_e323 - _e268) * 0.015625f));
            let _e345 = select(_e343, 0f, (_e343 < 0f));
            let _e347 = select(_e345, 1f, (_e345 > 1f));
            phi_1111_ = vec2<f32>(0f, 0f);
            phi_1114_ = 0f;
            phi_1116_ = 0u;
            loop {
                let _e359 = phi_1111_;
                let _e361 = phi_1114_;
                let _e363 = phi_1116_;
                local_7 = _e359;
                local_8 = _e359;
                local_17 = _e361;
                local_18 = _e361;
                local_19 = _e361;
                local_20 = _e361;
                let _e364 = (_e363 < 4u);
                if _e364 {
                    if _e364 {
                    } else {
                        phi_14977_ = true;
                        break;
                    }
                    let _e371 = frame.member[0u].ripples[_e363].origin[0u];
                    let _e378 = frame.member[0u].ripples[_e363].origin[1u];
                    let _e385 = frame.member[0u].ripples[_e363].animation[0u];
                    let _e392 = frame.member[0u].ripples[_e363].animation[1u];
                    let _e396 = frame.member[0u].time;
                    let _e398 = ((_e396 - _e385) * 1.2f);
                    let _e400 = select(_e398, 0f, (_e398 < 0f));
                    let _e402 = select(_e400, 1f, (_e400 > 1f));
                    let _e403 = (_e220.x - _e371);
                    let _e404 = (_e220.y - _e378);
                    let _e408 = sqrt(((_e403 * _e403) + (_e404 * _e404)));
                    if (_e408 > 0.001f) {
                        phi_12011_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e403 / _e408), (_e404 / _e408)), _e408);
                    } else {
                        phi_12011_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e408);
                    }
                    let _e416 = phi_12011_;
                    let _e426 = ((abs((_e416.unnamed_1 - (_e402 * 600f))) - 80f) * -0.0125f);
                    let _e428 = select(_e426, 0f, (_e426 < 0f));
                    let _e430 = select(_e428, 1f, (_e428 > 1f));
                    let _e436 = (1f - _e402);
                    let _e437 = ((((_e430 * _e430) * (3f - (2f * _e430))) * _e392) * _e436);
                    let _e450 = (_e361 + (_e437 * 0.5f));
                    if (_e450 != _e450) {
                        phi_12022_ = true;
                    } else {
                        phi_12022_ = (1f <= _e450);
                    }
                    let _e454 = phi_12022_;
                    phi_1112_ = vec2<f32>((_e359.x + (((_e416.unnamed.x * _e437) * _e436) * 0.5f)), (_e359.y + (((_e416.unnamed.y * _e437) * _e436) * 0.5f)));
                    phi_1115_ = select(_e450, 1f, _e454);
                    phi_1117_ = (_e363 + 1u);
                } else {
                    phi_1112_ = vec2<f32>();
                    phi_1115_ = f32();
                    phi_1117_ = u32();
                }
                let _e458 = phi_1112_;
                let _e460 = phi_1115_;
                let _e462 = phi_1117_;
                continue;
                continuing {
                    phi_1111_ = _e458;
                    phi_1114_ = _e460;
                    phi_1116_ = _e462;
                    phi_14977_ = _e277;
                    break if !(_e364);
                }
            }
            let _e465 = phi_14977_;
            if _e465 {
                break;
            }
            if (_e281 > 0f) {
                let _e466 = (_e220.x - _e257);
                let _e467 = (_e220.y - _e262);
                let _e473 = ((sqrt(((_e466 * _e466) + (_e467 * _e467))) - 150f) * -0.006666667f);
                let _e475 = select(_e473, 0f, (_e473 < 0f));
                let _e477 = select(_e475, 1f, (_e475 > 1f));
                phi_1258_ = ((((_e477 * _e477) * (3f - (2f * _e477))) * _e281) * 8f);
            } else {
                phi_1258_ = 0f;
            }
            let _e485 = phi_1258_;
            let _e487 = local_7;
            let _e490 = local_8;
            let _e493 = (((_e268 + ((((_e323 + ((_e268 - _e323) * _e347)) - ((32f * _e347) * (1f - _e347))) - _e268) * _e221.w)) - 0.5f) * -1f);
            let _e495 = select(_e493, 0f, (_e493 < 0f));
            let _e497 = select(_e495, 1f, (_e495 > 1f));
            let _e507 = (sqrt(((_e487.x * _e487.x) + (_e490.y * _e490.y))) * 22f);
            let _e510 = ((_e252 + ((((_e313 + ((_e252 - _e313) * _e330)) - ((32f * _e330) * (1f - _e330))) - _e252) * _e221.w)) - (((_e485 * ((_e497 * _e497) * (3f - (2f * _e497)))) + _e507) * 0.5f));
            let _e511 = (56f + _e246);
            let _e512 = (_e239 + 8f);
            let _e516 = (_e291 > ((_e511 + (_e511 + _e512)) * 0.5f));
            let _e521 = pill.member[_e222].calendar_expansion;
            let _e523 = (_e511 + (select(0f, 1f, _e516) * _e512));
            let _e524 = (_e523 * 0.0007377049f);
            let _e525 = (0.5f + _e524);
            let _e529 = ((_e521 - _e525) / ((_e524 + 0.74f) - _e525));
            let _e531 = select(_e529, 0f, (_e529 < 0f));
            let _e533 = select(_e531, 1f, (_e531 > 1f));
            let _e537 = ((_e533 * _e533) * (3f - (2f * _e533)));
            let _e539 = (292f * _e537);
            let _e540 = (_e239 * _e537);
            let _e545 = (324f + ((292f - _e539) * 0.5f));
            let _e546 = ((_e523 - _e246) + ((_e239 - _e540) * 0.5f));
            let _e547 = (_e293 - _e545);
            let _e548 = (_e291 - _e546);
            if (_e539 != _e539) {
                phi_12086_ = true;
            } else {
                phi_12086_ = (0.001f >= _e539);
            }
            let _e552 = phi_12086_;
            let _e554 = (_e547 / select(_e539, 0.001f, _e552));
            if _e516 {
                let _e562 = ((_e554 * 5f) - 0.5f);
                let _e564 = select(_e562, 0f, (_e562 < 0f));
                phi_1414_ = select(_e564, 4f, (_e564 > 4f));
            } else {
                let _e556 = ((_e554 * 6f) - 0.5f);
                let _e558 = select(_e556, 0f, (_e556 < 0f));
                phi_1414_ = select(_e558, 5f, (_e558 > 5f));
            }
            let _e568 = phi_1414_;
            let _e569 = (_e537 <= 0.001f);
            if _e569 {
                phi_1428_ = 340282350000000000000000000000000000000f;
            } else {
                let _e571 = (_e540 * 0.5f);
                let _e577 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e547 - (_e537 * 146f)), (_e548 - _e571)), ((_e539 - _e540) * 0.5f), _e571);
                phi_1428_ = _e577;
            }
            let _e579 = phi_1428_;
            if _e569 {
                phi_1451_ = 340282350000000000000000000000000000000f;
            } else {
                let _e584 = (_e540 * 0.5f);
                let _e590 = cantus_render_shader_sd_capsule_box(vec2<f32>((((_e257 - _e292) - _e545) - (_e537 * 146f)), ((_e315 - _e546) - _e584)), ((_e539 - _e540) * 0.5f), _e584);
                phi_1451_ = _e590;
            }
            let _e592 = phi_1451_;
            let _e594 = ((_e592 - 0.5f) * -1f);
            let _e596 = select(_e594, 0f, (_e594 < 0f));
            let _e598 = select(_e596, 1f, (_e596 > 1f));
            let _e606 = (_e579 - (((_e485 * ((_e598 * _e598) * (3f - (2f * _e598)))) + _e507) * 0.5f));
            let _e607 = (_e510 != _e510);
            if _e607 {
                phi_12104_ = true;
            } else {
                phi_12104_ = (_e606 <= _e510);
            }
            let _e610 = phi_12104_;
            let _e611 = select(_e510, _e606, _e610);
            let _e612 = fwidth(_e611);
            if (_e612 != _e612) {
                phi_12119_ = true;
            } else {
                phi_12119_ = (0.55f >= _e612);
            }
            let _e616 = phi_12119_;
            let _e617 = select(_e612, 0.55f, _e616);
            let _e621 = ((_e611 - _e617) / (-(_e617) - _e617));
            let _e623 = select(_e621, 0f, (_e621 < 0f));
            let _e625 = select(_e623, 1f, (_e623 > 1f));
            let _e629 = ((_e625 * _e625) * (3f - (2f * _e625)));
            if (_e611 != _e611) {
                phi_12134_ = true;
            } else {
                phi_12134_ = (0f >= _e611);
            }
            let _e633 = phi_12134_;
            let _e637 = (exp((select(_e611, 0f, _e633) * -0.3f)) * 0.16f);
            if (_e629 != _e629) {
                phi_12149_ = true;
            } else {
                phi_12149_ = (_e637 >= _e629);
            }
            let _e641 = phi_12149_;
            let _e642 = select(_e629, _e637, _e641);
            if (_e642 <= 0.0009765625f) {
                discard;
            }
            let _e648 = pill.member[_e222].hourly_conditions[0u];
            let _e649 = (_e244 * 0.0032467532f);
            let _e651 = select(_e649, 0f, (_e649 < 0f));
            let _e660 = pill.member[_e222].hourly_conditions[1u];
            let _e662 = ((abs((select(_e651, 1f, (_e651 > 1f)) - 0.5f)) - 0.2f) * 9.999999f);
            let _e664 = select(_e662, 0f, (_e662 < 0f));
            let _e666 = select(_e664, 1f, (_e664 > 1f));
            let _e670 = ((_e666 * _e666) * (3f - (2f * _e666)));
            let _e675 = (_e648.fog + ((_e660.fog - _e648.fog) * _e670));
            let _e680 = (_e648.cloud + ((_e660.cloud - _e648.cloud) * _e670));
            let _e685 = (_e648.rain + ((_e660.rain - _e648.rain) * _e670));
            let _e690 = (_e648.snow + ((_e660.snow - _e648.snow) * _e670));
            let _e695 = (_e648.lightning + ((_e660.lightning - _e648.lightning) * _e670));
            let _e700 = (_e648.hail + ((_e660.hail - _e648.hail) * _e670));
            let _e702 = ((_e313 - 8f) * -0.0625f);
            let _e704 = select(_e702, 0f, (_e702 < 0f));
            let _e706 = select(_e704, 1f, (_e704 > 1f));
            let _e710 = ((_e706 * _e706) * (3f - (2f * _e706)));
            let _e713 = (_e675 + ((_e648.fog - _e675) * _e710));
            let _e716 = (_e680 + ((_e648.cloud - _e680) * _e710));
            let _e719 = (_e685 + ((_e648.rain - _e685) * _e710));
            let _e722 = (_e690 + ((_e648.snow - _e690) * _e710));
            let _e725 = (_e695 + ((_e648.lightning - _e695) * _e710));
            let _e728 = (_e700 + ((_e648.hail - _e700) * _e710));
            let _e729 = fwidth(_e313);
            if (_e729 != _e729) {
                phi_12233_ = true;
            } else {
                phi_12233_ = (0.55f >= _e729);
            }
            let _e733 = phi_12233_;
            let _e734 = select(_e729, 0.55f, _e733);
            let _e738 = ((_e313 - _e734) / (-(_e734) - _e734));
            let _e740 = select(_e738, 0f, (_e738 < 0f));
            let _e742 = select(_e740, 1f, (_e740 > 1f));
            let _e747 = (((_e742 * _e742) * (3f - (2f * _e742))) * _e221.w);
            let _e748 = (1f - _e747);
            let _e758 = ((_e239 * _e748) + ((_e239 + _e287) * _e747));
            let _e759 = (((_e244 * _e748) + (_e293 * _e747)) * 0.0032467532f);
            let _e760 = (((_e245 * _e748) + (_e291 * _e747)) / _e758);
            if _e607 {
                phi_12248_ = true;
            } else {
                phi_12248_ = (0f <= _e510);
            }
            let _e765 = phi_12248_;
            let _e768 = (1f + (select(_e510, 0f, _e765) * 0.008333334f));
            let _e770 = select(_e768, 0f, (_e768 < 0f));
            let _e772 = select(_e770, 0.6f, (_e770 > 0.6f));
            let _e779 = (_e487.x * 0.04f);
            let _e780 = (_e490.y * 0.04f);
            let _e781 = ((_e759 - (((_e759 - 0.5f) * _e772) * 0.08f)) - _e779);
            let _e782 = ((_e760 - (((_e760 - 0.5f) * _e772) * 0.08f)) - _e780);
            if (_e537 > 0.001f) {
                let _e785 = (_e547 / _e539);
                let _e786 = (_e548 / _e540);
                if (_e606 != _e606) {
                    phi_12263_ = true;
                } else {
                    phi_12263_ = (0f <= _e606);
                }
                let _e792 = phi_12263_;
                let _e795 = (1f + (select(_e606, 0f, _e792) * 0.008333334f));
                let _e797 = select(_e795, 0f, (_e795 < 0f));
                let _e799 = select(_e797, 0.6f, (_e797 > 0.6f));
                phi_1719_ = vec2<f32>(((_e785 - (((_e785 - 0.5f) * _e799) * 0.08f)) - _e779), ((_e786 - (((_e786 - 0.5f) * _e799) * 0.08f)) - _e780));
            } else {
                phi_1719_ = vec2<f32>(_e781, _e782);
            }
            let _e810 = phi_1719_;
            let _e811 = fwidth(_e606);
            if (_e811 != _e811) {
                phi_12278_ = true;
            } else {
                phi_12278_ = (0.55f >= _e811);
            }
            let _e815 = phi_12278_;
            let _e816 = select(_e811, 0.55f, _e815);
            let _e820 = ((_e606 - _e816) / (-(_e816) - _e816));
            let _e822 = select(_e820, 0f, (_e820 < 0f));
            let _e824 = select(_e822, 1f, (_e822 > 1f));
            let _e829 = (((_e824 * _e824) * (3f - (2f * _e824))) * _e537);
            let _e830 = floor(_e568);
            let _e835 = select(select(u32(_e830), 0u, (_e830 < 0f)), 4294967295u, (_e830 > 4294967000f));
            if _e516 {
                if (_e835 < 5u) {
                } else {
                    break;
                }
                let _e944 = pill.member[_e222].daily_conditions[_e835];
                let _e945 = (_e835 + 1u);
                let _e947 = select(_e945, 4u, (4u < _e945));
                if (_e947 < 5u) {
                } else {
                    break;
                }
                let _e953 = pill.member[_e222].daily_conditions[_e947];
                let _e955 = (_e568 - trunc(_e568));
                let _e957 = select(_e955, 0f, (_e955 < 0f));
                let _e959 = select(_e957, 1f, (_e957 > 1f));
                let _e963 = ((_e959 * _e959) * (3f - (2f * _e959)));
                let _e998 = pill.member[_e222].sun_hours;
                let _e1001 = (_e998[1] - _e998[0]);
                if (12f >= _e998[0]) {
                    let _e1003 = (12f <= _e998[1]);
                    if _e1003 {
                        let _e1005 = ((12f - _e998[0]) / _e1001);
                        phi_12417_ = array<f32, 2>(_e1005, sin((_e1005 * 3.1415927f)));
                    } else {
                        phi_12417_ = array<f32, 2>();
                    }
                    let _e1010 = phi_12417_;
                    phi_12420_ = _e1010;
                    phi_12421_ = select(true, false, _e1003);
                } else {
                    phi_12420_ = array<f32, 2>();
                    phi_12421_ = true;
                }
                let _e1013 = phi_12420_;
                let _e1015 = phi_12421_;
                if _e1015 {
                    let _e1016 = (24f - _e1001);
                    if (12f < _e998[0]) {
                        phi_12434_ = ((36f - _e998[1]) / _e1016);
                    } else {
                        phi_12434_ = ((12f - _e998[1]) / _e1016);
                    }
                    let _e1023 = phi_12434_;
                    phi_12444_ = array<f32, 2>(select(0f, 1f, (12f >= _e998[1])), -(sin((_e1023 * 3.1415927f))));
                } else {
                    phi_12444_ = _e1013;
                }
                let _e1031 = phi_12444_;
                phi_1864_ = u0028_render_tempo_WeatherCondition_u0020_f32_u0029_(render_tempo_WeatherCondition((_e944.fog + ((_e953.fog - _e944.fog) * _e963)), (_e944.cloud + ((_e953.cloud - _e944.cloud) * _e963)), (_e944.rain + ((_e953.rain - _e944.rain) * _e963)), (_e944.snow + ((_e953.snow - _e944.snow) * _e963)), (_e944.lightning + ((_e953.lightning - _e944.lightning) * _e963)), (_e944.hail + ((_e953.hail - _e944.hail) * _e963))), _e1031[1]);
            } else {
                if (_e835 < 6u) {
                } else {
                    break;
                }
                let _e841 = pill.member[_e222].hourly_conditions[_e835];
                let _e842 = (_e835 + 1u);
                let _e844 = select(_e842, 5u, (5u < _e842));
                if (_e844 < 6u) {
                } else {
                    break;
                }
                let _e850 = pill.member[_e222].hourly_conditions[_e844];
                let _e852 = (_e568 - trunc(_e568));
                let _e854 = select(_e852, 0f, (_e852 < 0f));
                let _e856 = select(_e854, 1f, (_e854 > 1f));
                let _e860 = ((_e856 * _e856) * (3f - (2f * _e856)));
                let _e895 = pill.member[_e222].hourly_start;
                let _e898 = ((_e895 + (_e568 * 4f)) % 24f);
                let _e902 = pill.member[_e222].sun_hours;
                let _e905 = (_e902[1] - _e902[0]);
                if (_e898 >= _e902[0]) {
                    let _e907 = (_e898 <= _e902[1]);
                    if _e907 {
                        let _e909 = ((_e898 - _e902[0]) / _e905);
                        phi_12332_ = array<f32, 2>(_e909, sin((_e909 * 3.1415927f)));
                    } else {
                        phi_12332_ = array<f32, 2>();
                    }
                    let _e914 = phi_12332_;
                    phi_12335_ = _e914;
                    phi_12336_ = select(true, false, _e907);
                } else {
                    phi_12335_ = array<f32, 2>();
                    phi_12336_ = true;
                }
                let _e917 = phi_12335_;
                let _e919 = phi_12336_;
                if _e919 {
                    let _e920 = (24f - _e905);
                    if (_e898 < _e902[0]) {
                        phi_12349_ = (((_e898 + 24f) - _e902[1]) / _e920);
                    } else {
                        phi_12349_ = ((_e898 - _e902[1]) / _e920);
                    }
                    let _e928 = phi_12349_;
                    phi_12359_ = array<f32, 2>(select(0f, 1f, (_e898 >= _e902[1])), -(sin((_e928 * 3.1415927f))));
                } else {
                    phi_12359_ = _e917;
                }
                let _e936 = phi_12359_;
                phi_1864_ = u0028_render_tempo_WeatherCondition_u0020_f32_u0029_(render_tempo_WeatherCondition((_e841.fog + ((_e850.fog - _e841.fog) * _e860)), (_e841.cloud + ((_e850.cloud - _e841.cloud) * _e860)), (_e841.rain + ((_e850.rain - _e841.rain) * _e860)), (_e841.snow + ((_e850.snow - _e841.snow) * _e860)), (_e841.lightning + ((_e850.lightning - _e841.lightning) * _e860)), (_e841.hail + ((_e850.hail - _e841.hail) * _e860))), _e936[1]);
            }
            let _e1035 = phi_1864_;
            let _e1040 = (1f - _e829);
            let _e1045 = ((_e781 * _e1040) + (_e810.x * _e829));
            if (_e606 != _e606) {
                phi_12463_ = true;
            } else {
                phi_12463_ = (1000f <= _e606);
            }
            let _e1056 = phi_12463_;
            let _e1064 = (_e713 + ((_e1035.unnamed.fog - _e713) * _e829));
            let _e1068 = (_e716 + ((_e1035.unnamed.cloud - _e716) * _e829));
            let _e1072 = (_e719 + ((_e1035.unnamed.rain - _e719) * _e829));
            let _e1076 = (_e722 + ((_e1035.unnamed.snow - _e722) * _e829));
            let _e1084 = (_e728 + ((_e1035.unnamed.hail - _e728) * _e829));
            let _e1087 = (_e221.y + ((_e1035.unnamed_1 - _e221.y) * _e829));
            let _e1088 = (_e1045 * ((308f * _e1040) + (_e539 * _e829)));
            let _e1089 = (((_e782 * _e1040) + (_e810.y * _e829)) * ((_e758 * _e1040) + (_e540 * _e829)));
            let _e1093 = frame.member[0u].time;
            let _e1094 = (_e1089 / _e239);
            let _e1096 = ((_e1087 - -0.04f) * 4.1666665f);
            let _e1098 = select(_e1096, 0f, (_e1096 < 0f));
            let _e1100 = select(_e1098, 1f, (_e1098 > 1f));
            let _e1104 = ((_e1100 * _e1100) * (3f - (2f * _e1100)));
            let _e1106 = ((_e1087 - -0.32f) * 4.166667f);
            let _e1108 = select(_e1106, 0f, (_e1106 < 0f));
            let _e1110 = select(_e1108, 1f, (_e1108 > 1f));
            let _e1115 = (1f - _e1104);
            let _e1118 = ((_e1087 - -0.18f) * 5.5555553f);
            let _e1120 = select(_e1118, 0f, (_e1118 < 0f));
            let _e1122 = select(_e1120, 1f, (_e1120 > 1f));
            let _e1128 = ((_e1087 - 0.2f) * -5.5555553f);
            let _e1130 = select(_e1128, 0f, (_e1128 < 0f));
            let _e1132 = select(_e1130, 1f, (_e1130 > 1f));
            let _e1137 = (((_e1122 * _e1122) * (3f - (2f * _e1122))) * ((_e1132 * _e1132) * (3f - (2f * _e1132))));
            let _e1139 = ((_e1094 - 1f) * -1f);
            let _e1141 = select(_e1139, 0f, (_e1139 < 0f));
            let _e1143 = select(_e1141, 1f, (_e1141 > 1f));
            let _e1147 = ((_e1143 * _e1143) * (3f - (2f * _e1143)));
            let _e1148 = (1f - _e1147);
            let _e1178 = (0.3f * _e1148);
            let _e1179 = (0.22f * _e1147);
            let _e1185 = ((((_e1110 * _e1110) * (3f - (2f * _e1110))) * _e1115) * 0.8f);
            let _e1186 = (1f - _e1185);
            let _e1203 = (_e1137 * 0.9f);
            let _e1204 = (1f - _e1203);
            let _e1216 = floor((_e1088 * 0.055555556f));
            let _e1217 = floor((_e1089 * 0.055555556f));
            let _e1221 = cantus_render_shader_hash(vec2<f32>(_e1216, _e1217));
            let _e1230 = (_e1088 - (((_e1216 + 0.2f) + (_e1221.x * 0.6f)) * 18f));
            let _e1231 = (_e1089 - (((_e1217 + 0.2f) + (_e1221.y * 0.6f)) * 18f));
            let _e1237 = ((sqrt(((_e1230 * _e1230) + (_e1231 * _e1231))) - 1f) * -1.6666666f);
            let _e1239 = select(_e1237, 0f, (_e1237 < 0f));
            let _e1241 = select(_e1239, 1f, (_e1239 > 1f));
            let _e1249 = cantus_render_shader_hash(vec2<f32>((_e1216 + 31.7f), (_e1217 + 31.7f)));
            let _e1252 = ((_e1249.x - 0.75f) * 4f);
            let _e1254 = select(_e1252, 0f, (_e1252 < 0f));
            let _e1256 = select(_e1254, 1f, (_e1254 > 1f));
            let _e1267 = ((((((_e1241 * _e1241) * (3f - (2f * _e1241))) * ((_e1256 * _e1256) * (3f - (2f * _e1256)))) * _e1115) * (1f - _e1068)) * (0.3f + (_e1147 * 0.7f)));
            let _e1268 = (((((((((0.006f * _e1148) + (0.025f * _e1147)) * _e1115) + (((0.08f * _e1148) + (0.32f * _e1147)) * _e1104)) * _e1186) + (((0.1f * _e1148) + _e1179) * _e1185)) * _e1204) + (((0.78f * _e1148) + (0.38f * _e1147)) * _e1203)) + _e1267);
            let _e1269 = (((((((((0.012f * _e1148) + (0.04f * _e1147)) * _e1115) + (((0.34f * _e1148) + (0.67f * _e1147)) * _e1104)) * _e1186) + (((0.16f * _e1148) + (0.25f * _e1147)) * _e1185)) * _e1204) + ((_e1178 + _e1179) * _e1203)) + _e1267);
            let _e1270 = (((((((((0.035f * _e1148) + (0.095f * _e1147)) * _e1115) + (((0.62f * _e1148) + (0.87f * _e1147)) * _e1104)) * _e1186) + ((_e1178 + (0.45f * _e1147)) * _e1185)) * _e1204) + (((0.2f * _e1148) + (0.42f * _e1147)) * _e1203)) + _e1267);
            if (_e1068 > 0.0009765625f) {
                let _e1273 = (_e1088 / _e239);
                phi_12520_ = 0i;
                phi_12521_ = 0.5f;
                phi_12522_ = 0f;
                phi_12523_ = vec2<f32>(((_e1273 * 0.14f) + (_e1093 * 0.012f)), ((_e1094 * 0.14f) + 6.1f));
                loop {
                    let _e1281 = phi_12520_;
                    let _e1283 = phi_12521_;
                    let _e1285 = phi_12522_;
                    let _e1287 = phi_12523_;
                    local_9 = _e1285;
                    let _e1288 = (_e1281 < 4i);
                    if _e1288 {
                        let _e1291 = cantus_render_shader_simplex_noise(_e1287);
                        phi_12548_ = (_e1281 + 1i);
                        phi_12549_ = (_e1283 * 0.5f);
                        phi_12550_ = (_e1285 + (_e1291 * _e1283));
                        phi_12551_ = vec2<f32>(((_e1287.x * 1.6f) + (_e1287.y * 1.2f)), ((_e1287.y * 1.6f) - (_e1287.x * 1.2f)));
                    } else {
                        phi_12548_ = i32();
                        phi_12549_ = f32();
                        phi_12550_ = f32();
                        phi_12551_ = vec2<f32>();
                    }
                    let _e1304 = phi_12548_;
                    let _e1306 = phi_12549_;
                    let _e1308 = phi_12550_;
                    let _e1310 = phi_12551_;
                    continue;
                    continuing {
                        phi_12520_ = _e1304;
                        phi_12521_ = _e1306;
                        phi_12522_ = _e1308;
                        phi_12523_ = _e1310;
                        break if !(_e1288);
                    }
                }
                let _e1313 = local_9;
                let _e1314 = (_e1313 * 0.5f);
                phi_12562_ = 0i;
                phi_12563_ = 0.5f;
                phi_12564_ = 0f;
                phi_12565_ = vec2<f32>(((_e1273 * 0.287f) + (_e1093 * 0.018f)), ((_e1094 * 0.287f) + -3.7f));
                loop {
                    let _e1323 = phi_12562_;
                    let _e1325 = phi_12563_;
                    let _e1327 = phi_12564_;
                    let _e1329 = phi_12565_;
                    local_10 = _e1327;
                    local_11 = _e1327;
                    let _e1330 = (_e1323 < 4i);
                    if _e1330 {
                        let _e1333 = cantus_render_shader_simplex_noise(_e1329);
                        phi_12590_ = (_e1323 + 1i);
                        phi_12591_ = (_e1325 * 0.5f);
                        phi_12592_ = (_e1327 + (_e1333 * _e1325));
                        phi_12593_ = vec2<f32>(((_e1329.x * 1.6f) + (_e1329.y * 1.2f)), ((_e1329.y * 1.6f) - (_e1329.x * 1.2f)));
                    } else {
                        phi_12590_ = i32();
                        phi_12591_ = f32();
                        phi_12592_ = f32();
                        phi_12593_ = vec2<f32>();
                    }
                    let _e1346 = phi_12590_;
                    let _e1348 = phi_12591_;
                    let _e1350 = phi_12592_;
                    let _e1352 = phi_12593_;
                    continue;
                    continuing {
                        phi_12562_ = _e1346;
                        phi_12563_ = _e1348;
                        phi_12564_ = _e1350;
                        phi_12565_ = _e1352;
                        break if !(_e1330);
                    }
                }
                let _e1355 = local_10;
                let _e1358 = local_11;
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
                let _e1431 = (_e1137 * 0.45f);
                let _e1432 = (1f - _e1431);
                let _e1444 = (_e1068 * (0.12f + (((_e1366 * _e1366) * (3f - (2f * _e1366))) * 0.7f)));
                let _e1445 = (1f - _e1444);
                phi_2315_ = vec3<f32>(((_e1268 * _e1445) + (((((((0.16f * _e1394) + (0.32f * _e1393)) * _e1115) + (((0.62f * _e1394) + (0.92f * _e1393)) * _e1104)) * _e1432) + (((0.5f * _e1394) + (0.76f * _e1393)) * _e1431)) * _e1444)), ((_e1269 * _e1445) + (((((((0.2f * _e1394) + (0.36f * _e1393)) * _e1115) + (((0.7f * _e1394) + (0.94f * _e1393)) * _e1104)) * _e1432) + (((0.36f * _e1394) + (0.59f * _e1393)) * _e1431)) * _e1444)), ((_e1270 * _e1445) + (((((((0.28f * _e1394) + (0.43f * _e1393)) * _e1115) + (((0.78f * _e1394) + (0.96f * _e1393)) * _e1104)) * _e1432) + (((0.4f * _e1394) + (0.56f * _e1393)) * _e1431)) * _e1444)));
            } else {
                phi_2315_ = vec3<f32>(_e1268, _e1269, _e1270);
            }
            let _e1457 = phi_2315_;
            let _e1459 = (1f - (_e1072 * 0.2f));
            let _e1469 = ((_e1457.x * _e1459) + (_e1072 * 0.020000001f));
            let _e1470 = ((_e1457.y * _e1459) + (_e1072 * 0.034f));
            let _e1471 = ((_e1457.z * _e1459) + (_e1072 * 0.05f));
            if (_e1072 > 0.0009765625f) {
                let _e1476 = (_e1088 - (20f * _e1093));
                let _e1477 = (_e1089 - (110f * _e1093));
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
                let _e1539 = (((((_e1517 * _e1517) * (3f - (2f * _e1517))) * ((_e1532 * _e1532) * (3f - (2f * _e1532)))) * _e1072) * 0.7f);
                let _e1541 = select(_e1539, 0f, (_e1539 < 0f));
                let _e1543 = select(_e1541, 1f, (_e1541 > 1f));
                let _e1544 = (1f - _e1543);
                phi_2525_ = vec3<f32>(((_e1469 * _e1544) + (0.52f * _e1543)), ((_e1470 * _e1544) + (0.72f * _e1543)), ((_e1471 * _e1544) + (0.9f * _e1543)));
            } else {
                phi_2525_ = vec3<f32>(_e1469, _e1470, _e1471);
            }
            let _e1556 = phi_2525_;
            if (_e1076 > 0.0009765625f) {
                let _e1560 = (_e1088 - (5f * _e1093));
                let _e1561 = (_e1089 - (14f * _e1093));
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
                let _e1625 = (((((_e1603 * _e1603) * (3f - (2f * _e1603))) * ((_e1618 * _e1618) * (3f - (2f * _e1618)))) * _e1076) * 0.92f);
                let _e1627 = select(_e1625, 0f, (_e1625 < 0f));
                let _e1629 = select(_e1627, 1f, (_e1627 > 1f));
                let _e1630 = (1f - _e1629);
                let _e1637 = (0.96f * _e1629);
                phi_2723_ = vec3<f32>(((_e1556.x * _e1630) + _e1637), ((_e1556.y * _e1630) + _e1637), ((_e1556.z * _e1630) + _e1637));
            } else {
                phi_2723_ = _e1556;
            }
            let _e1643 = phi_2723_;
            if (_e1084 > 0.0009765625f) {
                let _e1647 = (_e1088 - (18f * _e1093));
                let _e1648 = (_e1089 - (85f * _e1093));
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
                let _e1712 = (((((_e1690 * _e1690) * (3f - (2f * _e1690))) * ((_e1705 * _e1705) * (3f - (2f * _e1705)))) * _e1084) * 0.7f);
                let _e1714 = select(_e1712, 0f, (_e1712 < 0f));
                let _e1716 = select(_e1714, 1f, (_e1714 > 1f));
                let _e1717 = (1f - _e1716);
                phi_2921_ = vec3<f32>(((_e1643.x * _e1717) + (0.75f * _e1716)), ((_e1643.y * _e1717) + (0.86f * _e1716)), ((_e1643.z * _e1717) + (0.94f * _e1716)));
            } else {
                phi_2921_ = _e1643;
            }
            let _e1732 = phi_2921_;
            let _e1736 = ((sin((_e1093 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1738 = select(_e1736, 0f, (_e1736 < 0f));
            let _e1740 = select(_e1738, 1f, (_e1738 > 1f));
            let _e1745 = (((_e1740 * _e1740) * (3f - (2f * _e1740))) * (_e725 + ((_e1035.unnamed.lightning - _e725) * _e829)));
            let _e1747 = (1f - (_e1745 * 0.55f));
            let _e1757 = ((_e1732.x * _e1747) + (_e1745 * 0.3575f));
            let _e1758 = ((_e1732.y * _e1747) + (_e1745 * 0.407f));
            let _e1759 = ((_e1732.z * _e1747) + (_e1745 * 0.528f));
            if (_e1064 > 0.0009765625f) {
                phi_12604_ = 0i;
                phi_12605_ = 0.5f;
                phi_12606_ = 0f;
                phi_12607_ = vec2<f32>(((_e1045 * 0.9f) + (_e1093 * 0.008f)), ((_e1094 * 0.32f) + 12f));
                loop {
                    let _e1769 = phi_12604_;
                    let _e1771 = phi_12605_;
                    let _e1773 = phi_12606_;
                    let _e1775 = phi_12607_;
                    local_12 = _e1773;
                    let _e1776 = (_e1769 < 4i);
                    if _e1776 {
                        let _e1779 = cantus_render_shader_simplex_noise(_e1775);
                        phi_12632_ = (_e1769 + 1i);
                        phi_12633_ = (_e1771 * 0.5f);
                        phi_12634_ = (_e1773 + (_e1779 * _e1771));
                        phi_12635_ = vec2<f32>(((_e1775.x * 1.6f) + (_e1775.y * 1.2f)), ((_e1775.y * 1.6f) - (_e1775.x * 1.2f)));
                    } else {
                        phi_12632_ = i32();
                        phi_12633_ = f32();
                        phi_12634_ = f32();
                        phi_12635_ = vec2<f32>();
                    }
                    let _e1792 = phi_12632_;
                    let _e1794 = phi_12633_;
                    let _e1796 = phi_12634_;
                    let _e1798 = phi_12635_;
                    continue;
                    continuing {
                        phi_12604_ = _e1792;
                        phi_12605_ = _e1794;
                        phi_12606_ = _e1796;
                        phi_12607_ = _e1798;
                        break if !(_e1776);
                    }
                }
                let _e1801 = local_12;
                let _e1804 = (((_e1801 * 0.5f) + 0.15f) * 2.857143f);
                let _e1806 = select(_e1804, 0f, (_e1804 < 0f));
                let _e1808 = select(_e1806, 1f, (_e1806 > 1f));
                let _e1815 = (_e1064 * (0.58f + (((_e1808 * _e1808) * (3f - (2f * _e1808))) * 0.18f)));
                let _e1816 = (1f - _e1815);
                phi_3009_ = vec3<f32>(((_e1757 * _e1816) + (0.63f * _e1815)), ((_e1758 * _e1816) + (0.69f * _e1815)), ((_e1759 * _e1816) + (0.73f * _e1815)));
            } else {
                phi_3009_ = vec3<f32>(_e1757, _e1758, _e1759);
            }
            let _e1828 = phi_3009_;
            let _e1830 = ((_e1094 - 0.12f) * -8.333334f);
            let _e1832 = select(_e1830, 0f, (_e1830 < 0f));
            let _e1834 = select(_e1832, 1f, (_e1832 > 1f));
            let _e1841 = (((_e510 + ((select(_e606, 1000f, _e1056) - _e510) * _e829)) - 5f) * -0.125f);
            let _e1843 = select(_e1841, 0f, (_e1841 < 0f));
            let _e1845 = select(_e1843, 1f, (_e1843 > 1f));
            let _e1851 = ((((_e1834 * _e1834) * (3f - (2f * _e1834))) * 0.12f) + (((_e1845 * _e1845) * (3f - (2f * _e1845))) * 0.08f));
            let _e1853 = (_e1828.x + _e1851);
            let _e1855 = (_e1828.y + _e1851);
            let _e1857 = (_e1828.z + _e1851);
            if (_e252 < 1f) {
                let _e1862 = (16f + (_e221.x * 276f));
                let _e1864 = select(_e221.y, 0f, (_e221.y < 0f));
                let _e1868 = (0.72f - (select(_e1864, 1f, (_e1864 > 1f)) * 0.45f));
                let _e1871 = ((_e221.y - 0.55f) * -1.8867923f);
                let _e1873 = select(_e1871, 0f, (_e1871 < 0f));
                let _e1875 = select(_e1873, 1f, (_e1873 > 1f));
                let _e1879 = ((_e1875 * _e1875) * (3f - (2f * _e1875)));
                let _e1880 = (1f - _e1879);
                if (_e680 > 0.0009765625f) {
                    phi_12659_ = 0i;
                    phi_12660_ = 0.5f;
                    phi_12661_ = 0f;
                    phi_12662_ = vec2<f32>((((_e1862 / _e239) * 0.14f) + (_e1093 * 0.012f)), ((_e1868 * 0.14f) + 6.1f));
                    loop {
                        let _e1898 = phi_12659_;
                        let _e1900 = phi_12660_;
                        let _e1902 = phi_12661_;
                        let _e1904 = phi_12662_;
                        local_13 = _e1902;
                        let _e1905 = (_e1898 < 4i);
                        if _e1905 {
                            let _e1908 = cantus_render_shader_simplex_noise(_e1904);
                            phi_12687_ = (_e1898 + 1i);
                            phi_12688_ = (_e1900 * 0.5f);
                            phi_12689_ = (_e1902 + (_e1908 * _e1900));
                            phi_12690_ = vec2<f32>(((_e1904.x * 1.6f) + (_e1904.y * 1.2f)), ((_e1904.y * 1.6f) - (_e1904.x * 1.2f)));
                        } else {
                            phi_12687_ = i32();
                            phi_12688_ = f32();
                            phi_12689_ = f32();
                            phi_12690_ = vec2<f32>();
                        }
                        let _e1921 = phi_12687_;
                        let _e1923 = phi_12688_;
                        let _e1925 = phi_12689_;
                        let _e1927 = phi_12690_;
                        continue;
                        continuing {
                            phi_12659_ = _e1921;
                            phi_12660_ = _e1923;
                            phi_12661_ = _e1925;
                            phi_12662_ = _e1927;
                            break if !(_e1905);
                        }
                    }
                    let _e1930 = local_13;
                    let _e1933 = (((_e1930 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e1935 = select(_e1933, 0f, (_e1933 < 0f));
                    let _e1937 = select(_e1935, 1f, (_e1935 > 1f));
                    phi_3151_ = ((((_e1937 * _e1937) * (3f - (2f * _e1937))) * _e680) * 0.82f);
                } else {
                    phi_3151_ = 0f;
                }
                let _e1945 = phi_3151_;
                let _e1947 = ((_e221.y - -0.02f) * 16.666668f);
                let _e1949 = select(_e1947, 0f, (_e1947 < 0f));
                let _e1951 = select(_e1949, 1f, (_e1949 > 1f));
                let _e1958 = (_e244 - _e1862);
                let _e1959 = (_e245 - (_e239 * _e1868));
                let _e1963 = sqrt(((_e1958 * _e1958) + (_e1959 * _e1959)));
                let _e1965 = ((_e1963 - 62f) * -0.01724138f);
                let _e1967 = select(_e1965, 0f, (_e1965 < 0f));
                let _e1969 = select(_e1967, 1f, (_e1967 > 1f));
                let _e1976 = ((_e1963 - 11f) * -0.1f);
                let _e1978 = select(_e1976, 0f, (_e1976 < 0f));
                let _e1980 = select(_e1978, 1f, (_e1978 > 1f));
                let _e1987 = (((((_e1969 * _e1969) * (3f - (2f * _e1969))) * 0.24f) + (((_e1980 * _e1980) * (3f - (2f * _e1980))) * 0.7f)) * (((_e1951 * _e1951) * (3f - (2f * _e1951))) * (1f - _e1945)));
                let _e1988 = (1f - _e1987);
                let _e1999 = ((_e252 - 1f) * -0.5f);
                let _e2001 = select(_e1999, 0f, (_e1999 < 0f));
                let _e2003 = select(_e2001, 1f, (_e2001 > 1f));
                let _e2007 = ((_e2003 * _e2003) * (3f - (2f * _e2003)));
                let _e2008 = (1f - _e2007);
                phi_3269_ = vec3<f32>(((_e1853 * _e2008) + (((_e1853 * _e1988) + (((0.96f * _e1880) + (0.98f * _e1879)) * _e1987)) * _e2007)), ((_e1855 * _e2008) + (((_e1855 * _e1988) + (((0.98f * _e1880) + (0.74f * _e1879)) * _e1987)) * _e2007)), ((_e1857 * _e2008) + (((_e1857 * _e1988) + ((_e1880 + (0.66f * _e1879)) * _e1987)) * _e2007)));
            } else {
                phi_3269_ = (_e1828 + vec3(_e1851));
            }
            let _e2020 = phi_3269_;
            if (_e521 > 0f) {
                let _e2022 = (_e291 >= 0f);
                if _e2022 {
                    if (_e293 < 308f) {
                        if (_e291 < 54f) {
                            let _e2140 = (_e293 - 154f);
                            if (_e2140 < -60f) {
                                phi_3305_ = 2u;
                            } else {
                                phi_3305_ = select(1u, 3u, (_e2140 > 60f));
                            }
                            let _e2145 = phi_3305_;
                            phi_3430_ = 40f;
                            phi_3431_ = _e2145;
                            phi_3432_ = 1f;
                            phi_3433_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                        } else {
                            if (_e291 < 82f) {
                                let _e2121 = floor((_e293 * 0.022727273f));
                                let _e2123 = select(_e2121, 0f, (_e2121 < 0f));
                                let _e2125 = select(_e2123, 6f, (_e2123 > 6f));
                                phi_3426_ = 68f;
                                phi_3427_ = (27u + select(select(u32(_e2125), 0u, (_e2125 < 0f)), 4294967295u, (_e2125 > 4294967000f)));
                                phi_3428_ = 0.75f;
                                phi_3429_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                            } else {
                                let _e2069 = floor((((_e291 - 96f) * 0.041666668f) + 0.5f));
                                let _e2071 = select(_e2069, 0f, (_e2069 < 0f));
                                let _e2073 = select(_e2071, 5f, (_e2071 > 5f));
                                let _e2081 = floor((_e293 * 0.022727273f));
                                let _e2083 = select(_e2081, 0f, (_e2081 < 0f));
                                let _e2085 = select(_e2083, 6f, (_e2083 > 6f));
                                let _e2091 = ((select(select(u32(_e2073), 0u, (_e2073 < 0f)), 4294967295u, (_e2073 > 4294967000f)) * 7u) + select(select(u32(_e2085), 0u, (_e2085 < 0f)), 4294967295u, (_e2085 > 4294967000f)));
                                let _e2101 = pill.member[_e222].today_index;
                                let _e2109 = pill.member[_e222].month_range[0u];
                                if (_e2091 < _e2109) {
                                    phi_3421_ = true;
                                } else {
                                    let _e2115 = pill.member[_e222].month_range[1u];
                                    phi_3421_ = (_e2091 >= _e2115);
                                }
                                let _e2118 = phi_3421_;
                                phi_3426_ = (96f + (f32((_e2091 / 7u)) * 24f));
                                phi_3427_ = (34u + _e2091);
                                phi_3428_ = select(1f, 0.32f, _e2118);
                                phi_3429_ = select(vec3<f32>(0.94f, 0.94f, 0.94f), vec3<f32>(1f, 0.68f, 0.68f), vec3((bitcast<i32>(_e2091) == _e2101)));
                            }
                            let _e2133 = phi_3426_;
                            let _e2135 = phi_3427_;
                            let _e2137 = phi_3428_;
                            let _e2139 = phi_3429_;
                            phi_3430_ = _e2133;
                            phi_3431_ = _e2135;
                            phi_3432_ = _e2137;
                            phi_3433_ = _e2139;
                        }
                        let _e2147 = phi_3430_;
                        let _e2149 = phi_3431_;
                        let _e2151 = phi_3432_;
                        let _e2153 = phi_3433_;
                        phi_3523_ = _e2147;
                        phi_3524_ = _e2149;
                        phi_3525_ = _e2151;
                        phi_3526_ = _e2153;
                    } else {
                        if (_e293 >= 316f) {
                            if (_e291 >= 56f) {
                                let _e2027 = select(6u, 5u, _e516);
                                let _e2032 = ((((_e293 - 316f) * 0.0032467532f) * f32(_e2027)) - 0.5f);
                                let _e2034 = f32((_e2027 - 1u));
                                let _e2035 = (0f <= _e2034);
                                if _e2035 {
                                } else {
                                    break;
                                }
                                let _e2037 = select(_e2032, 0f, (_e2032 < 0f));
                                let _e2040 = round(select(_e2037, _e2034, (_e2037 > _e2034)));
                                if _e2035 {
                                } else {
                                    break;
                                }
                                let _e2042 = select(_e2040, 0f, (_e2040 < 0f));
                                let _e2044 = select(_e2042, _e2034, (_e2042 > _e2034));
                                phi_3519_ = _e523;
                                phi_3520_ = (select(5u, 17u, _e516) + ((select(select(u32(_e2044), 0u, (_e2044 < 0f)), 4294967295u, (_e2044 > 4294967000f)) * 2u) + select(0u, 1u, (_e291 >= _e523))));
                            } else {
                                phi_3519_ = 40f;
                                phi_3520_ = 4u;
                            }
                            let _e2057 = phi_3519_;
                            let _e2059 = phi_3520_;
                            phi_3521_ = _e2057;
                            phi_3522_ = _e2059;
                        } else {
                            phi_3521_ = 40f;
                            phi_3522_ = 4u;
                        }
                        let _e2061 = phi_3521_;
                        let _e2063 = phi_3522_;
                        phi_3523_ = _e2061;
                        phi_3524_ = _e2063;
                        phi_3525_ = 1f;
                        phi_3526_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                    }
                    let _e2155 = phi_3523_;
                    let _e2157 = phi_3524_;
                    let _e2159 = phi_3525_;
                    let _e2161 = phi_3526_;
                    let _e2162 = (_e2155 * 0.0007377049f);
                    let _e2163 = (0.5f + _e2162);
                    let _e2167 = ((_e521 - _e2163) / ((_e2162 + 0.74f) - _e2163));
                    let _e2169 = select(_e2167, 0f, (_e2167 < 0f));
                    let _e2171 = select(_e2169, 1f, (_e2169 > 1f));
                    phi_3555_ = _e2157;
                    phi_3556_ = (((_e2171 * _e2171) * (3f - (2f * _e2171))) * _e2159);
                    phi_3557_ = _e2161;
                    phi_3558_ = vec2<f32>(_e293, _e291);
                } else {
                    phi_3555_ = u32();
                    phi_3556_ = f32();
                    phi_3557_ = vec3<f32>();
                    phi_3558_ = vec2<f32>();
                }
                let _e2178 = phi_3555_;
                let _e2180 = phi_3556_;
                let _e2182 = phi_3557_;
                let _e2184 = phi_3558_;
                phi_3560_ = _e2178;
                phi_3561_ = _e2180;
                phi_3562_ = _e2182;
                phi_3563_ = _e2184;
                phi_3564_ = select(true, false, _e2022);
            } else {
                phi_3560_ = u32();
                phi_3561_ = f32();
                phi_3562_ = vec3<f32>();
                phi_3563_ = vec2<f32>();
                phi_3564_ = true;
            }
            let _e2187 = phi_3560_;
            let _e2189 = phi_3561_;
            let _e2191 = phi_3562_;
            let _e2193 = phi_3563_;
            let _e2195 = phi_3564_;
            let _e2196 = select(_e2187, 0u, _e2195);
            let _e2199 = select(_e2191, vec3<f32>(0.94f, 0.94f, 0.94f), vec3(_e2195));
            let _e2201 = select(_e2193, vec2<f32>(_e244, _e245), vec2(_e2195));
            if (_e2196 < 76u) {
            } else {
                break;
            }
            let _e2212 = pill.member[_e222].text.lines[_e2196].min[0u];
            let _e2220 = pill.member[_e222].text.lines[_e2196].min[1u];
            let _e2228 = pill.member[_e222].text.lines[_e2196].max[0u];
            let _e2236 = pill.member[_e222].text.lines[_e2196].max[1u];
            let _e2244 = pill.member[_e222].text.lines[_e2196].origin[0u];
            let _e2252 = pill.member[_e222].text.lines[_e2196].origin[1u];
            let _e2259 = pill.member[_e222].text.lines[_e2196].size;
            let _e2266 = pill.member[_e222].text.lines[_e2196].weight;
            let _e2273 = pill.member[_e222].text.lines[_e2196].count;
            let _e2280 = pill.member[_e222].text.lines[_e2196].first;
            if (_e2201.x < _e2212) {
                phi_3828_ = f32();
                phi_3829_ = true;
            } else {
                if (_e2201.x > _e2228) {
                    phi_3826_ = f32();
                    phi_3827_ = true;
                } else {
                    if (_e2201.y < _e2220) {
                        phi_3824_ = f32();
                        phi_3825_ = true;
                    } else {
                        let _e2284 = (_e2201.y > _e2236);
                        if _e2284 {
                            phi_3823_ = f32();
                        } else {
                            phi_3640_ = _e2273;
                            phi_3643_ = 0u;
                            loop {
                                let _e2286 = phi_3640_;
                                let _e2288 = phi_3643_;
                                local_14 = _e2288;
                                let _e2289 = (_e2288 < _e2286);
                                if _e2289 {
                                    let _e2292 = (_e2288 + ((_e2286 - _e2288) / 2u));
                                    let _e2293 = (_e2280 + _e2292);
                                    if (_e2293 < 512u) {
                                    } else {
                                        phi_15000_ = true;
                                        break;
                                    }
                                    let _e2301 = pill.member[_e222].text.glyphs[_e2293].x;
                                    let _e2304 = (_e2301 <= ((_e2201.x - _e2244) / _e2259));
                                    if _e2304 {
                                        phi_3677_ = (_e2292 + 1u);
                                    } else {
                                        phi_3677_ = _e2288;
                                    }
                                    let _e2307 = phi_3677_;
                                    phi_3641_ = select(_e2292, _e2286, _e2304);
                                    phi_3644_ = _e2307;
                                } else {
                                    phi_3641_ = u32();
                                    phi_3644_ = u32();
                                }
                                let _e2310 = phi_3641_;
                                let _e2312 = phi_3644_;
                                continue;
                                continuing {
                                    phi_3640_ = _e2310;
                                    phi_3643_ = _e2312;
                                    phi_15000_ = _e465;
                                    break if !(_e2289);
                                }
                            }
                            let _e2315 = phi_15000_;
                            if _e2315 {
                                break;
                            }
                            let _e2317 = local_14;
                            let _e2318 = (_e2317 + 1u);
                            phi_15203_ = _e2315;
                            phi_3685_ = select(_e2318, _e2273, (_e2273 < _e2318));
                            phi_3688_ = -1000000f;
                            loop {
                                let _e2322 = phi_15203_;
                                let _e2324 = phi_3685_;
                                let _e2326 = phi_3688_;
                                local_21 = _e2326;
                                if (_e2324 > 0u) {
                                    let _e2328 = (_e2324 - 1u);
                                    let _e2329 = (_e2280 + _e2328);
                                    if (_e2329 < 512u) {
                                    } else {
                                        phi_15207_ = true;
                                        break;
                                    }
                                    let _e2337 = pill.member[_e222].text.glyphs[_e2329].x;
                                    let _e2344 = pill.member[_e222].text.glyphs[_e2329].glyph;
                                    if (_e2344 < arrayLength((&glyphs.member))) {
                                    } else {
                                        phi_15207_ = true;
                                        break;
                                    }
                                    let _e2350 = glyphs.member[_e2344].min[0u];
                                    let _e2355 = glyphs.member[_e2344].min[1u];
                                    let _e2360 = glyphs.member[_e2344].max[0u];
                                    let _e2365 = glyphs.member[_e2344].max[1u];
                                    let _e2369 = glyphs.member[_e2344].start;
                                    let _e2373 = glyphs.member[_e2344].count;
                                    let _e2376 = (((_e2201.x - _e2244) / _e2259) - _e2337);
                                    let _e2379 = (-((_e2201.y - _e2252)) / _e2259);
                                    let _e2380 = (3.5f / _e2259);
                                    let _e2381 = (_e2360 + _e2380);
                                    let _e2382 = (_e2376 > _e2381);
                                    if _e2382 {
                                        phi_15209_ = _e2322;
                                        phi_3816_ = f32();
                                    } else {
                                        if (_e2376 >= (_e2350 - _e2380)) {
                                            if (_e2379 >= (_e2355 - _e2380)) {
                                                if (_e2376 <= _e2381) {
                                                    if (_e2379 <= (_e2365 + _e2380)) {
                                                        phi_3775_ = 0u;
                                                        phi_3778_ = 0i;
                                                        phi_3780_ = 340282350000000000000000000000000000000f;
                                                        loop {
                                                            let _e2391 = phi_3775_;
                                                            let _e2393 = phi_3778_;
                                                            let _e2395 = phi_3780_;
                                                            local_15 = _e2395;
                                                            local_16 = _e2393;
                                                            let _e2396 = (_e2391 < _e2373);
                                                            if _e2396 {
                                                                let _e2397 = (_e2369 + _e2391);
                                                                if (_e2397 < arrayLength((&edges.member))) {
                                                                } else {
                                                                    phi_15200_ = true;
                                                                    break;
                                                                }
                                                                let _e2401 = edges.member[_e2397];
                                                                let _e2403 = cantus_render_text_edge_distance(_e2401, _e2266, vec2<f32>(_e2376, _e2379));
                                                                if (_e2395 != _e2395) {
                                                                    phi_12747_ = true;
                                                                } else {
                                                                    phi_12747_ = (_e2403.member <= _e2395);
                                                                }
                                                                let _e2409 = phi_12747_;
                                                                phi_3776_ = (_e2391 + 1u);
                                                                phi_3779_ = (_e2393 + _e2403.member_1);
                                                                phi_3781_ = select(_e2395, _e2403.member, _e2409);
                                                            } else {
                                                                phi_3776_ = u32();
                                                                phi_3779_ = i32();
                                                                phi_3781_ = f32();
                                                            }
                                                            let _e2414 = phi_3776_;
                                                            let _e2416 = phi_3779_;
                                                            let _e2418 = phi_3781_;
                                                            continue;
                                                            continuing {
                                                                phi_3775_ = _e2414;
                                                                phi_3778_ = _e2416;
                                                                phi_3780_ = _e2418;
                                                                phi_15200_ = _e2322;
                                                                break if !(_e2396);
                                                            }
                                                        }
                                                        let _e2421 = phi_15200_;
                                                        phi_15207_ = _e2421;
                                                        if _e2421 {
                                                            break;
                                                        }
                                                        let _e2423 = local_15;
                                                        let _e2427 = local_16;
                                                        let _e2430 = ((sqrt(_e2423) * _e2259) * select(1f, -1f, (_e2427 == 0i)));
                                                        if (_e2326 != _e2326) {
                                                            phi_12762_ = true;
                                                        } else {
                                                            phi_12762_ = (_e2430 >= _e2326);
                                                        }
                                                        let _e2434 = phi_12762_;
                                                        phi_15213_ = _e2421;
                                                        phi_3812_ = select(_e2326, _e2430, _e2434);
                                                    } else {
                                                        phi_15213_ = _e2322;
                                                        phi_3812_ = _e2326;
                                                    }
                                                    let _e2437 = phi_15213_;
                                                    let _e2439 = phi_3812_;
                                                    phi_15212_ = _e2437;
                                                    phi_3813_ = _e2439;
                                                } else {
                                                    phi_15212_ = _e2322;
                                                    phi_3813_ = _e2326;
                                                }
                                                let _e2441 = phi_15212_;
                                                let _e2443 = phi_3813_;
                                                phi_15211_ = _e2441;
                                                phi_3814_ = _e2443;
                                            } else {
                                                phi_15211_ = _e2322;
                                                phi_3814_ = _e2326;
                                            }
                                            let _e2445 = phi_15211_;
                                            let _e2447 = phi_3814_;
                                            phi_15210_ = _e2445;
                                            phi_3815_ = _e2447;
                                        } else {
                                            phi_15210_ = _e2322;
                                            phi_3815_ = _e2326;
                                        }
                                        let _e2449 = phi_15210_;
                                        let _e2451 = phi_3815_;
                                        phi_15209_ = _e2449;
                                        phi_3816_ = _e2451;
                                    }
                                    let _e2453 = phi_15209_;
                                    let _e2455 = phi_3816_;
                                    phi_15208_ = _e2453;
                                    phi_3686_ = _e2328;
                                    phi_3689_ = _e2455;
                                    phi_3818_ = select(true, false, _e2382);
                                } else {
                                    phi_15208_ = _e2322;
                                    phi_3686_ = u32();
                                    phi_3689_ = f32();
                                    phi_3818_ = false;
                                }
                                let _e2458 = phi_15208_;
                                let _e2460 = phi_3686_;
                                let _e2462 = phi_3689_;
                                let _e2464 = phi_3818_;
                                continue;
                                continuing {
                                    phi_15203_ = _e2458;
                                    phi_3685_ = _e2460;
                                    phi_3688_ = _e2462;
                                    phi_15207_ = _e2458;
                                    break if !(_e2464);
                                }
                            }
                            let _e2467 = phi_15207_;
                            if _e2467 {
                                break;
                            }
                            let _e2690 = local_21;
                            phi_3823_ = _e2690;
                        }
                        let _e2469 = phi_3823_;
                        phi_3824_ = _e2469;
                        phi_3825_ = _e2284;
                    }
                    let _e2471 = phi_3824_;
                    let _e2473 = phi_3825_;
                    phi_3826_ = _e2471;
                    phi_3827_ = _e2473;
                }
                let _e2475 = phi_3826_;
                let _e2477 = phi_3827_;
                phi_3828_ = _e2475;
                phi_3829_ = _e2477;
            }
            let _e2479 = phi_3828_;
            let _e2481 = phi_3829_;
            let _e2484 = ((select(_e2479, -1000000f, _e2481) * 1.25f) + 0.5f);
            let _e2486 = select(_e2484, 0f, (_e2484 < 0f));
            let _e2488 = select(_e2486, 1f, (_e2486 > 1f));
            let _e2493 = (((_e2488 * _e2488) * (3f - (2f * _e2488))) * select(_e2189, 1f, _e2195));
            let _e2494 = (1f - _e2493);
            let _e2507 = ((_e2020.x * _e2494) + (_e2199.x * _e2493));
            let _e2508 = ((_e2020.y * _e2494) + (_e2199.y * _e2493));
            let _e2509 = ((_e2020.z * _e2494) + (_e2199.z * _e2493));
            let _e2517 = local_17;
            let _e2518 = (1f - _e2517);
            let _e2523 = local_18;
            let _e2526 = local_19;
            let _e2529 = local_20;
            out_color = vec4<f32>((((_e2507 * _e2518) + (((_e2507 * 1.5f) + 0.1f) * _e2523)) * _e629), (((_e2508 * _e2518) + (((_e2508 * 1.5f) + 0.1f) * _e2526)) * _e629), (((_e2509 * _e2518) + (((_e2509 * 1.5f) + 0.1f) * _e2529)) * _e629), _e642);
            break;
        }
    }
    return;
}

fn render_track_vertex_impl() {
    var phi_12791_: bool;
    var phi_12827_: bool;
    var phi_12849_: bool;
    var phi_12864_: bool;
    var phi_12888_: bool;

    let _e23 = vertex_5;
    let _e24 = instance_1;
    let _e28 = pill_1.member[_e24].width;
    let _e32 = frame.member[0u].panel_height;
    let _e36 = pill_1.member[_e24].x;
    let _e40 = frame.member[0u].panel_top;
    let _e42 = (_e36 + (_e28 * 0.5f));
    let _e49 = pill_1.member[_e24].secondary_expansion;
    let _e53 = pill_1.member[_e24].rating;
    let _e59 = pill_1.member[_e24].primary_playlist_count;
    let _e61 = (select(0f, 5f, (_e53 >= 0i)) + f32(_e59));
    let _e67 = pill_1.member[_e24].secondary_playlist_count;
    let _e68 = f32(_e67);
    let _e72 = pill_1.member[_e24].primary_alpha;
    let _e73 = (_e61 - 1f);
    if (_e73 != _e73) {
        phi_12791_ = true;
    } else {
        phi_12791_ = (0f >= _e73);
    }
    let _e77 = phi_12791_;
    let _e83 = select(0f, 1f, ((_e61 * _e72) > 0f));
    let _e84 = (((select(_e73, 0f, _e77) * 9f) + 32.4f) * _e83);
    let _e85 = (32.4f * _e83);
    let _e86 = (_e68 - 1f);
    if (_e86 != _e86) {
        phi_12827_ = true;
    } else {
        phi_12827_ = (0f >= _e86);
    }
    let _e90 = phi_12827_;
    let _e98 = select(0f, 1f, ((_e68 * _e49) > 0f));
    let _e99 = (((((select(_e86, 0f, _e90) * 18f) * _e49) * 0.5f) + 32.4f) * _e98);
    let _e100 = (32.4f * _e98);
    let _e102 = select(_e99, _e84, (_e84 > _e99));
    let _e105 = (_e36 - 48f);
    let _e106 = (_e42 - _e102);
    if (_e105 != _e105) {
        phi_12849_ = true;
    } else {
        phi_12849_ = (_e106 <= _e105);
    }
    let _e110 = phi_12849_;
    let _e111 = select(_e105, _e106, _e110);
    let _e112 = (_e40 - 48f);
    let _e114 = ((_e36 + _e28) + 48f);
    let _e115 = (_e42 + _e102);
    if (_e114 != _e114) {
        phi_12864_ = true;
    } else {
        phi_12864_ = (_e115 >= _e114);
    }
    let _e119 = phi_12864_;
    let _e122 = ((_e40 + _e32) + 48f);
    let _e124 = (((((_e40 + (_e32 * 0.975f)) - 3f) + (18f * _e49)) + -5.4f) + select(_e100, _e85, (_e85 > _e100)));
    if (_e122 != _e122) {
        phi_12888_ = true;
    } else {
        phi_12888_ = (_e124 >= _e122);
    }
    let _e128 = phi_12888_;
    let _e139 = (_e111 + (f32((_e23 & 1u)) * (select(_e114, _e115, _e119) - _e111)));
    let _e140 = (_e112 + (f32((_e23 >> bitcast<u32>(1i))) * (select(_e122, _e124, _e128) - _e112)));
    let _e145 = frame.member[0u].screen_size[0u];
    let _e150 = frame.member[0u].screen_size[1u];
    let _e153 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e139, _e140), vec2<f32>(_e145, _e150));
    out_position = _e153;
    out_pixel_pos[0u] = _e139;
    out_pixel_pos[1u] = _e140;
    out_pill_idx = _e24;
    return;
}

fn cantus_render_track_plasma_field(param_13: vec2<f32>, param_14: render_track_PaletteColor, param_15: f32, param_16: f32, param_17: f32) -> vec4<f32> {
    let _e17 = ((sin((((param_13.x * param_15) + (param_13.y * param_16)) + param_17)) * 0.5f) + 0.5f);
    let _e23 = ((0.12f + (_e17 * _e17)) * (0.25f + (param_14.weight * 3f)));
    let _e25 = unpack4x8unorm(param_14.rgb);
    return vec4<f32>((_e25.x * _e23), (_e25.y * _e23), (_e25.z * _e23), _e23);
}

fn render_track_fragment_impl() {
    var local_22: array<u32, 2>;
    var phi_4819_: vec2<f32>;
    var phi_4822_: f32;
    var phi_4824_: u32;
    var phi_12920_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_12931_: bool;
    var phi_4820_: vec2<f32>;
    var phi_4823_: f32;
    var phi_4825_: u32;
    var phi_15217_: bool;
    var phi_4966_: f32;
    var local_23: vec2<f32>;
    var local_24: vec2<f32>;
    var phi_12967_: bool;
    var phi_12991_: bool;
    var phi_13027_: bool;
    var phi_13051_: bool;
    var phi_13076_: bool;
    var phi_13091_: bool;
    var phi_13106_: bool;
    var phi_13123_: bool;
    var phi_13138_: bool;
    var phi_13153_: bool;
    var phi_13168_: bool;
    var phi_13183_: bool;
    var phi_5838_: vec3<f32>;
    var phi_5839_: vec3<f32>;
    var local_25: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_5945_: vec4<f32>;
    var phi_5948_: i32;
    var phi_13355_: bool;
    var phi_13390_: bool;
    var phi_13405_: bool;
    var phi_13420_: bool;
    var phi_13435_: bool;
    var phi_6243_: vec4<f32>;
    var phi_5946_: vec4<f32>;
    var phi_5949_: i32;
    var phi_6245_: vec4<f32>;
    var phi_6246_: vec4<f32>;
    var phi_6258_: vec4<f32>;
    var phi_6261_: u32;
    var phi_6297_: render_track_PillIconRow;
    var phi_6298_: f32;
    var phi_13465_: bool;
    var phi_6417_: bool;
    var phi_6422_: bool;
    var phi_13502_: bool;
    var phi_13517_: bool;
    var phi_6524_: vec4<f32>;
    var phi_6525_: vec4<f32>;
    var phi_6526_: vec4<f32>;
    var phi_6527_: vec4<f32>;
    var phi_6259_: vec4<f32>;
    var phi_6262_: u32;
    var phi_15332_: bool;
    var phi_15367_: bool;
    var phi_6535_: u32;
    var phi_6538_: f32;
    var phi_6618_: u32;
    var phi_6621_: u32;
    var phi_6655_: u32;
    var phi_6619_: u32;
    var phi_6622_: u32;
    var phi_15364_: bool;
    var local_29: u32;
    var phi_15372_: bool;
    var phi_6663_: u32;
    var phi_6666_: f32;
    var phi_6753_: u32;
    var phi_6756_: i32;
    var phi_6758_: f32;
    var phi_13532_: bool;
    var phi_6754_: u32;
    var phi_6757_: i32;
    var phi_6759_: f32;
    var phi_15369_: bool;
    var local_30: f32;
    var local_31: i32;
    var phi_13547_: bool;
    var phi_15382_: bool;
    var phi_6790_: f32;
    var phi_15381_: bool;
    var phi_6791_: f32;
    var phi_15380_: bool;
    var phi_6792_: f32;
    var phi_15379_: bool;
    var phi_6793_: f32;
    var phi_15378_: bool;
    var phi_6794_: f32;
    var phi_15377_: bool;
    var phi_6664_: u32;
    var phi_6667_: f32;
    var phi_6796_: bool;
    var phi_15376_: bool;
    var phi_15394_: bool;
    var phi_6801_: f32;
    var phi_15393_: bool;
    var phi_6802_: f32;
    var phi_6803_: bool;
    var phi_15392_: bool;
    var phi_6804_: f32;
    var phi_6805_: bool;
    var phi_15391_: bool;
    var phi_6806_: f32;
    var phi_6807_: bool;
    var phi_13562_: bool;
    var phi_15387_: bool;
    var phi_6536_: u32;
    var phi_6539_: f32;
    var phi_15386_: bool;
    var local_32: f32;
    var local_33: vec4<f32>;
    var local_34: vec4<f32>;
    var local_35: vec4<f32>;
    var local_36: vec4<f32>;
    var local_37: vec4<f32>;
    var local_38: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e162 = pixel_pos_1;
            let _e163 = pill_idx_1;
            let _e173 = pill_1.member[_e163].x;
            let _e177 = pill_1.member[_e163].width;
            let _e181 = frame.member[0u].panel_height;
            let _e185 = frame.member[0u].panel_top;
            let _e186 = (_e162.x - _e173);
            let _e187 = (_e162.y - _e185);
            let _e188 = (_e177 * 0.5f);
            let _e189 = (_e181 * 0.5f);
            let _e192 = (_e177 - _e181);
            let _e193 = (_e192 * 0.5f);
            let _e195 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e186 - _e188), (_e187 - _e189)), _e193, _e189);
            let _e200 = frame.member[0u].mouse_pos[0u];
            let _e205 = frame.member[0u].mouse_pos[1u];
            let _e211 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e200 - _e173) - _e188), ((_e205 - _e185) - _e189)), _e193, _e189);
            phi_4819_ = vec2<f32>(0f, 0f);
            phi_4822_ = 0f;
            phi_4824_ = 0u;
            loop {
                let _e213 = phi_4819_;
                let _e215 = phi_4822_;
                let _e217 = phi_4824_;
                local_23 = _e213;
                local_24 = _e213;
                local_25 = _e215;
                local_26 = _e215;
                local_27 = _e215;
                local_28 = _e215;
                let _e218 = (_e217 < 4u);
                if _e218 {
                    if _e218 {
                    } else {
                        phi_15217_ = true;
                        break;
                    }
                    let _e225 = frame.member[0u].ripples[_e217].origin[0u];
                    let _e232 = frame.member[0u].ripples[_e217].origin[1u];
                    let _e239 = frame.member[0u].ripples[_e217].animation[0u];
                    let _e246 = frame.member[0u].ripples[_e217].animation[1u];
                    let _e250 = frame.member[0u].time;
                    let _e252 = ((_e250 - _e239) * 1.2f);
                    let _e254 = select(_e252, 0f, (_e252 < 0f));
                    let _e256 = select(_e254, 1f, (_e254 > 1f));
                    let _e257 = (_e162.x - _e225);
                    let _e258 = (_e162.y - _e232);
                    let _e262 = sqrt(((_e257 * _e257) + (_e258 * _e258)));
                    if (_e262 > 0.001f) {
                        phi_12920_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e257 / _e262), (_e258 / _e262)), _e262);
                    } else {
                        phi_12920_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e262);
                    }
                    let _e270 = phi_12920_;
                    let _e280 = ((abs((_e270.unnamed_1 - (_e256 * 600f))) - 80f) * -0.0125f);
                    let _e282 = select(_e280, 0f, (_e280 < 0f));
                    let _e284 = select(_e282, 1f, (_e282 > 1f));
                    let _e290 = (1f - _e256);
                    let _e291 = ((((_e284 * _e284) * (3f - (2f * _e284))) * _e246) * _e290);
                    let _e304 = (_e215 + (_e291 * 0.5f));
                    if (_e304 != _e304) {
                        phi_12931_ = true;
                    } else {
                        phi_12931_ = (1f <= _e304);
                    }
                    let _e308 = phi_12931_;
                    phi_4820_ = vec2<f32>((_e213.x + (((_e270.unnamed.x * _e291) * _e290) * 0.5f)), (_e213.y + (((_e270.unnamed.y * _e291) * _e290) * 0.5f)));
                    phi_4823_ = select(_e304, 1f, _e308);
                    phi_4825_ = (_e217 + 1u);
                } else {
                    phi_4820_ = vec2<f32>();
                    phi_4823_ = f32();
                    phi_4825_ = u32();
                }
                let _e312 = phi_4820_;
                let _e314 = phi_4823_;
                let _e316 = phi_4825_;
                continue;
                continuing {
                    phi_4819_ = _e312;
                    phi_4822_ = _e314;
                    phi_4824_ = _e316;
                    phi_15217_ = false;
                    break if !(_e218);
                }
            }
            let _e319 = phi_15217_;
            if _e319 {
                break;
            }
            let _e323 = frame.member[0u].mouse_pressure;
            let _e324 = (_e323 > 0f);
            if _e324 {
                let _e325 = (_e162.x - _e200);
                let _e326 = (_e162.y - _e205);
                let _e332 = ((sqrt(((_e325 * _e325) + (_e326 * _e326))) - 150f) * -0.006666667f);
                let _e334 = select(_e332, 0f, (_e332 < 0f));
                let _e336 = select(_e334, 1f, (_e334 > 1f));
                phi_4966_ = ((((_e336 * _e336) * (3f - (2f * _e336))) * _e323) * 8f);
            } else {
                phi_4966_ = 0f;
            }
            let _e344 = phi_4966_;
            let _e346 = local_23;
            let _e349 = local_24;
            let _e351 = (_e186 / _e177);
            let _e352 = (_e187 / _e181);
            let _e353 = (_e351 - 0.5f);
            let _e354 = (_e352 - 0.5f);
            let _e355 = (_e173 + _e188);
            let _e357 = (_e185 + (_e181 * 0.975f));
            let _e358 = (_e357 - 3f);
            let _e362 = pill_1.member[_e163].secondary_expansion;
            let _e366 = pill_1.member[_e163].rating;
            let _e367 = (_e366 >= 0i);
            let _e368 = select(0f, 5f, _e367);
            let _e372 = pill_1.member[_e163].primary_playlist_count;
            let _e374 = (_e368 + f32(_e372));
            let _e378 = (_e358 + (18f * _e362));
            let _e382 = pill_1.member[_e163].secondary_playlist_count;
            let _e383 = f32(_e382);
            let _e389 = pill_1.member[_e163].primary_alpha;
            let _e390 = (_e357 + -10.4f);
            let _e393 = (_e374 - 1f);
            let _e394 = (_e393 != _e393);
            if _e394 {
                phi_12967_ = true;
            } else {
                phi_12967_ = (0f >= _e393);
            }
            let _e397 = phi_12967_;
            let _e401 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e162.x - _e355), (_e162.y - _e390)), (select(_e393, 0f, _e397) * 9f), 9f);
            if _e394 {
                phi_12991_ = true;
            } else {
                phi_12991_ = (0f >= _e393);
            }
            let _e406 = phi_12991_;
            let _e410 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e200 - _e355), (_e205 - _e390)), (select(_e393, 0f, _e406) * 9f), 9f);
            let _e412 = vec2<f32>(_e355, (_e378 + -5.4f));
            let _e413 = (_e383 - 1f);
            let _e414 = (_e413 != _e413);
            if _e414 {
                phi_13027_ = true;
            } else {
                phi_13027_ = (0f >= _e413);
            }
            let _e417 = phi_13027_;
            let _e422 = (10.5f * _e362);
            let _e424 = cantus_render_shader_sd_capsule_box((_e162 - _e412), (((select(_e413, 0f, _e417) * 18f) * _e362) * 0.5f), _e422);
            if _e414 {
                phi_13051_ = true;
            } else {
                phi_13051_ = (0f >= _e413);
            }
            let _e427 = phi_13051_;
            let _e434 = cantus_render_shader_sd_capsule_box((vec2<f32>(_e200, _e205) - _e412), (((select(_e413, 0f, _e427) * 18f) * _e362) * 0.5f), _e422);
            let _e437 = (0.5f + ((_e401 - _e195) * 0.05f));
            let _e439 = select(_e437, 0f, (_e437 < 0f));
            let _e441 = select(_e439, 1f, (_e439 > 1f));
            let _e451 = (_e195 + ((((_e401 + ((_e195 - _e401) * _e441)) - ((10f * _e441) * (1f - _e441))) - _e195) * _e389));
            let _e454 = (0.5f + ((_e410 - _e211) * 0.05f));
            let _e456 = select(_e454, 0f, (_e454 < 0f));
            let _e458 = select(_e456, 1f, (_e456 > 1f));
            let _e468 = (_e211 + ((((_e410 + ((_e211 - _e410) * _e458)) - ((10f * _e458) * (1f - _e458))) - _e211) * _e389));
            let _e470 = select(0f, 1f, (_e362 > 0f));
            let _e473 = (0.5f + ((_e424 - _e451) * 0.046296295f));
            let _e475 = select(_e473, 0f, (_e473 < 0f));
            let _e477 = select(_e475, 1f, (_e475 > 1f));
            let _e490 = (0.5f + ((_e434 - _e468) * 0.046296295f));
            let _e492 = select(_e490, 0f, (_e490 < 0f));
            let _e494 = select(_e492, 1f, (_e492 > 1f));
            let _e506 = (((_e468 + ((((_e434 + ((_e468 - _e434) * _e494)) - ((10.8f * _e494) * (1f - _e494))) - _e468) * _e470)) - 0.5f) * -1f);
            let _e508 = select(_e506, 0f, (_e506 < 0f));
            let _e510 = select(_e508, 1f, (_e508 > 1f));
            let _e520 = (sqrt(((_e346.x * _e346.x) + (_e349.y * _e349.y))) * 22f);
            let _e523 = ((_e451 + ((((_e424 + ((_e451 - _e424) * _e477)) - ((10.8f * _e477) * (1f - _e477))) - _e451) * _e470)) - (((_e344 * ((_e510 * _e510) * (3f - (2f * _e510)))) + _e520) * 0.5f));
            let _e524 = fwidth(_e523);
            if (_e524 != _e524) {
                phi_13076_ = true;
            } else {
                phi_13076_ = (0.55f >= _e524);
            }
            let _e528 = phi_13076_;
            let _e529 = select(_e524, 0.55f, _e528);
            let _e533 = ((_e523 - _e529) / (-(_e529) - _e529));
            let _e535 = select(_e533, 0f, (_e533 < 0f));
            let _e537 = select(_e535, 1f, (_e535 > 1f));
            let _e541 = ((_e537 * _e537) * (3f - (2f * _e537)));
            let _e542 = (_e523 != _e523);
            if _e542 {
                phi_13091_ = true;
            } else {
                phi_13091_ = (0f >= _e523);
            }
            let _e545 = phi_13091_;
            let _e549 = (exp((select(_e523, 0f, _e545) * -0.3f)) * 0.16f);
            if (_e541 != _e541) {
                phi_13106_ = true;
            } else {
                phi_13106_ = (_e549 >= _e541);
            }
            let _e553 = phi_13106_;
            let _e554 = select(_e541, _e549, _e553);
            let _e558 = pill_1.member[_e163].visibility;
            if ((_e554 * _e558) <= 0.0009765625f) {
                discard;
            }
            if _e542 {
                phi_13123_ = true;
            } else {
                phi_13123_ = (0f <= _e523);
            }
            let _e563 = phi_13123_;
            let _e566 = (1f + (select(_e523, 0f, _e563) * 0.008333334f));
            let _e568 = select(_e566, 0f, (_e566 < 0f));
            let _e570 = select(_e568, 0.6f, (_e568 > 0.6f));
            let _e580 = ((_e352 - ((_e354 * _e570) * 0.08f)) - (_e349.y * 0.04f));
            let _e581 = (((_e351 - ((_e353 * _e570) * 0.08f)) - (_e346.x * 0.04f)) * _e177);
            let _e582 = (_e580 * _e181);
            let _e586 = pill_1.member[_e163].effects;
            if _e542 {
                phi_13138_ = true;
            } else {
                phi_13138_ = (0f <= _e523);
            }
            let _e592 = phi_13138_;
            let _e595 = (1f + (select(_e523, 0f, _e592) * 0.008333334f));
            let _e597 = select(_e595, 0f, (_e595 < 0f));
            let _e599 = select(_e597, 1f, (_e597 > 1f));
            let _e612 = (_e586.seed - trunc(_e586.seed));
            let _e617 = ((_e177 / _e181) * ((0.5f + (_e612 * 0.12f)) + (_e586.turbulence * 0.18f)));
            if (_e617 != _e617) {
                phi_13153_ = true;
            } else {
                phi_13153_ = (1.7f >= _e617);
            }
            let _e621 = phi_13153_;
            let _e624 = select(0f, _e351, (_e351 > 0f));
            let _e626 = select(0f, _e352, (_e352 > 0f));
            let _e634 = (select(1f, _e626, (_e626 < 1f)) - (((((_e354 * _e599) * _e599) * 0.6f) + _e349.y) * 0.08f));
            let _e635 = ((select(1f, _e624, (_e624 < 1f)) - (((((_e353 * _e599) * _e599) * 0.6f) + _e346.x) * 0.08f)) * select(_e617, 1.7f, _e621));
            let _e646 = (_e586.flow_time * 0.8f);
            let _e656 = ((0.14f + (_e586.turbulence * 0.2f)) + _e586.beat);
            let _e661 = (_e586.seed + 1.5707964f);
            let _e666 = pill_1.member[_e163].colors[0u];
            let _e667 = vec2<f32>((_e635 + ((sin(((_e634 * 4.32f) + _e586.flow_time)) + cos(((_e635 * 1.3f) - (_e586.flow_time * 0.7f)))) * _e656)), ((_e634 * 1.6f) + ((cos(((_e635 * 2.3f) - _e646)) + sin(((_e634 * 2.72f) + (_e586.flow_time * 0.6f)))) * _e656)));
            let _e668 = cantus_render_track_plasma_field(_e667, _e666, 2.1f, 0.7f, _e586.flow_time);
            let _e673 = pill_1.member[_e163].colors[1u];
            let _e675 = cantus_render_track_plasma_field(_e667, _e673, 0.6f, -2.4f, (_e661 - _e646));
            let _e692 = pill_1.member[_e163].colors[2u];
            let _e695 = cantus_render_track_plasma_field(_e667, _e692, -1.5f, 1.9f, ((_e586.flow_time * 0.65f) + 2f));
            let _e708 = pill_1.member[_e163].colors[3u];
            let _e711 = cantus_render_track_plasma_field(_e667, _e708, 2.4f, 1.6f, (_e661 - (_e586.flow_time * 0.55f)));
            let _e719 = (((_e668.w + _e675.w) + _e695.w) + _e711.w);
            let _e720 = ((((_e668.x + _e675.x) + _e695.x) + _e711.x) / _e719);
            let _e721 = ((((_e668.y + _e675.y) + _e695.y) + _e711.y) / _e719);
            let _e722 = ((((_e668.z + _e675.z) + _e695.z) + _e711.z) / _e719);
            let _e727 = (((_e720 * 0.2126f) + (_e721 * 0.7152f)) + (_e722 * 0.0722f));
            let _e731 = frame.member[0u].playhead_x;
            let _e732 = (_e731 + 3f);
            let _e736 = ((_e162.x - _e732) / ((_e731 - 3f) - _e732));
            let _e738 = select(_e736, 0f, (_e736 < 0f));
            let _e740 = select(_e738, 1f, (_e738 > 1f));
            let _e745 = (_e586.valence * 0.4f);
            let _e746 = (1.55f + _e745);
            let _e748 = (_e727 * (-0.54999995f - _e745));
            let _e752 = (_e748 + (_e720 * _e746));
            let _e753 = (_e748 + (_e721 * _e746));
            let _e754 = (_e748 + (_e722 * _e746));
            let _e756 = select(0.035f, _e752, (_e752 > 0.035f));
            let _e758 = select(0.035f, _e753, (_e753 > 0.035f));
            let _e760 = select(0.035f, _e754, (_e754 > 0.035f));
            if (_e727 != _e727) {
                phi_13168_ = true;
            } else {
                phi_13168_ = (0.001f >= _e727);
            }
            let _e770 = phi_13168_;
            let _e772 = (0.52f / select(_e727, 0.001f, _e770));
            if (_e772 != _e772) {
                phi_13183_ = true;
            } else {
                phi_13183_ = (1f <= _e772);
            }
            let _e776 = phi_13183_;
            let _e777 = select(_e772, 1f, _e776);
            let _e784 = ((0.96f + (_e586.valence * 0.06f)) + (_e586.beat * 0.5f));
            let _e789 = ((_e580 - 0.45f) * 1.8181818f);
            let _e791 = select(_e789, 0f, (_e789 < 0f));
            let _e793 = select(_e791, 1f, (_e791 > 1f));
            let _e799 = (0.84f + (((_e793 * _e793) * (3f - (2f * _e793))) * 0.1f));
            let _e804 = (1f - (0.4f * ((_e740 * _e740) * (3f - (2f * _e740)))));
            let _e813 = pill_1.member[_e163].colors[3u].rgb;
            let _e814 = unpack4x8unorm(_e813);
            let _e827 = frame.member[0u].time;
            let _e832 = ((_e586.acousticness * 0.7f) + (_e586.instrumentalness * 0.3f));
            let _e839 = (8f - _e832);
            let _e844 = (_e827 * (0.35f + (_e586.instrumentalness * 0.55f)));
            let _e847 = ((_e186 / _e839) + (_e844 * (0.16f + (_e612 * 0.08f))));
            let _e848 = ((_e187 / _e839) + (_e844 * (0.055f + (sin((_e586.seed * 0.7f)) * 0.025f))));
            let _e849 = floor(_e847);
            let _e850 = floor(_e848);
            let _e859 = bitcast<u32>(select(0i, select(select(i32(_e850), i32(-2147483648), (_e850 < -2147483600f)), 2147483647i, (_e850 > 2147483500f)), (_e850 == _e850)));
            let _e867 = bitcast<u32>(select(0i, select(select(i32(_e849), i32(-2147483648), (_e849 < -2147483600f)), 2147483647i, (_e849 > 2147483500f)), (_e849 == _e849)));
            let _e869 = (bitcast<u32>((_e586.seed + 2.71f)) * 2654435761u);
            let _e875 = (((_e867 ^ _e869) * 1664525u) + 1013904223u);
            let _e877 = ((((_e859 ^ _e869) * 1664525u) + 1013904223u) + (_e875 * 1664525u));
            let _e879 = (_e875 + (_e877 * 1664525u));
            let _e887 = ((_e877 ^ (_e877 >> bitcast<u32>(16i))) + ((_e879 ^ (_e879 >> bitcast<u32>(16i))) * 1664525u));
            let _e891 = f32((_e887 ^ (_e887 >> bitcast<u32>(16i))));
            let _e892 = (_e891 * 0.0000000016600825f);
            let _e906 = (_e832 * 0.09f);
            let _e909 = (bitcast<u32>(_e586.seed) * 2654435761u);
            let _e915 = (((_e859 ^ _e909) * 1664525u) + 1013904223u);
            let _e917 = ((((_e867 ^ _e909) * 1664525u) + 1013904223u) + (_e915 * 1664525u));
            let _e919 = (_e915 + (_e917 * 1664525u));
            let _e927 = ((_e917 ^ (_e917 >> bitcast<u32>(16i))) + ((_e919 ^ (_e919 >> bitcast<u32>(16i))) * 1664525u));
            let _e935 = (((f32((_e927 ^ (_e927 >> bitcast<u32>(16i)))) * 0.00000000023283064f) - (0.985f - _e906)) / (_e906 + 0.014999986f));
            let _e937 = select(_e935, 0f, (_e935 < 0f));
            let _e939 = select(_e937, 1f, (_e937 > 1f));
            let _e948 = (((_e847 - _e849) - 0.5f) - ((_e891 * 0.00000000013038516f) - 0.28f));
            let _e949 = (((_e848 - _e850) - 0.5f) - (((_e892 - trunc(_e892)) * 0.56f) - 0.28f));
            let _e955 = ((sqrt(((_e948 * _e948) + (_e949 * _e949))) - 0.06f) * 4.5454545f);
            let _e957 = select(_e955, 0f, (_e955 < 0f));
            let _e959 = select(_e957, 1f, (_e957 > 1f));
            let _e972 = (((((_e939 * _e939) * (3f - (2f * _e939))) * (1f - ((_e959 * _e959) * (3f - (2f * _e959))))) * ((sin(((_e827 * ((0.7f + (_e891 * 0.00000000020954757f)) + (_e586.instrumentalness * 0.8f))) + (_e891 * 0.0000000014629181f))) * 0.5f) + 0.5f)) * (0.12f + (_e832 * 0.48f)));
            let _e976 = (((((select(0.92f, _e756, (_e756 < 0.92f)) * _e777) * _e784) * _e799) * _e804) + (((_e814.x * 0.75f) + 0.25f) * _e972));
            let _e977 = (((((select(0.92f, _e758, (_e758 < 0.92f)) * _e777) * _e784) * _e799) * _e804) + (((_e814.y * 0.75f) + 0.25f) * _e972));
            let _e978 = (((((select(0.92f, _e760, (_e760 < 0.92f)) * _e777) * _e784) * _e799) * _e804) + (((_e814.z * 0.75f) + 0.25f) * _e972));
            let _e979 = vec3<f32>(_e976, _e977, _e978);
            let _e983 = pill_1.member[_e163].image_index;
            if (_e983 >= 0i) {
                if (_e186 >= _e192) {
                    let _e987 = ((_e581 - _e192) / _e181);
                    let _e989 = vec3<f32>(_e987, _e580, f32(_e983));
                    let _e995 = textureSample(images, sampler_, vec2<f32>(_e989.x, _e989.y), i32(_e989.z));
                    let _e1001 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e987 - 0.5f) * _e181), ((_e580 - 0.5f) * _e181)), 0f, _e189);
                    let _e1003 = ((_e1001 - -4f) * 0.25f);
                    let _e1005 = select(_e1003, 0f, (_e1003 < 0f));
                    let _e1007 = select(_e1005, 1f, (_e1005 > 1f));
                    let _e1014 = ((_e211 - 0.5f) * -1f);
                    let _e1016 = select(_e1014, 0f, (_e1014 < 0f));
                    let _e1018 = select(_e1016, 1f, (_e1016 > 1f));
                    let _e1027 = ((_e195 - (((_e344 * ((_e1018 * _e1018) * (3f - (2f * _e1018)))) + _e520) * 0.5f)) - -0.5f);
                    let _e1029 = select(_e1027, 0f, (_e1027 < 0f));
                    let _e1031 = select(_e1029, 1f, (_e1029 > 1f));
                    let _e1042 = (((1f - ((_e1007 * _e1007) * (3f - (2f * _e1007)))) * (1f - ((_e1031 * _e1031) * (3f - (2f * _e1031))))) * _e995.w);
                    let _e1043 = (1f - _e1042);
                    phi_5838_ = vec3<f32>(((_e976 * _e1043) + (_e995.x * _e1042)), ((_e977 * _e1043) + (_e995.y * _e1042)), ((_e978 * _e1043) + (_e995.z * _e1042)));
                } else {
                    phi_5838_ = _e979;
                }
                let _e1055 = phi_5838_;
                phi_5839_ = _e1055;
            } else {
                phi_5839_ = _e979;
            }
            let _e1057 = phi_5839_;
            let _e1068 = ((_e580 - 0.12f) * -8.333334f);
            let _e1070 = select(_e1068, 0f, (_e1068 < 0f));
            let _e1072 = select(_e1070, 1f, (_e1070 > 1f));
            let _e1079 = ((_e523 - 5f) * -0.125f);
            let _e1081 = select(_e1079, 0f, (_e1079 < 0f));
            let _e1083 = select(_e1081, 1f, (_e1081 > 1f));
            let _e1089 = ((((_e1072 * _e1072) * (3f - (2f * _e1072))) * 0.12f) + (((_e1083 * _e1083) * (3f - (2f * _e1083))) * 0.08f));
            let _e1093 = (_e1057.x + (((_e1057.x * 0.68f) + 0.32f) * _e1089));
            let _e1094 = (_e1057.y + (((_e1057.y * 0.68f) + 0.32f) * _e1089));
            let _e1095 = (_e1057.z + (((_e1057.z * 0.68f) + 0.32f) * _e1089));
            let _e1103 = local_25;
            let _e1104 = (1f - _e1103);
            let _e1109 = local_26;
            let _e1112 = local_27;
            let _e1115 = local_28;
            let _e1123 = vec4<f32>((((_e1093 * _e1104) + (((_e1093 * 1.5f) + 0.1f) * _e1109)) * _e541), (((_e1094 * _e1104) + (((_e1094 * 1.5f) + 0.1f) * _e1112)) * _e541), (((_e1095 * _e1104) + (((_e1095 * 1.5f) + 0.1f) * _e1115)) * _e541), _e554);
            if _e367 {
                if (_e389 > 0f) {
                    phi_5945_ = _e1123;
                    phi_5948_ = 0i;
                    loop {
                        let _e1126 = phi_5945_;
                        let _e1128 = phi_5948_;
                        local_37 = _e1126;
                        let _e1129 = (_e1128 < 5i);
                        if _e1129 {
                            let _e1130 = f32(_e1128);
                            if _e394 {
                                phi_13355_ = true;
                            } else {
                                phi_13355_ = (0f >= _e393);
                            }
                            let _e1133 = phi_13355_;
                            let _e1138 = (_e355 + ((_e1130 - (select(_e393, 0f, _e1133) * 0.5f)) * 18f));
                            let _e1139 = (_e357 + -1f);
                            let _e1140 = (_e162.x - _e1138);
                            let _e1141 = (_e162.y - _e1139);
                            let _e1142 = abs(_e1140);
                            let _e1143 = abs(_e1141);
                            if (select(_e1143, _e1142, (_e1142 > _e1143)) < 38.88f) {
                                let _e1150 = ((f32(_e366) - (_e1130 * 2f)) * 0.5f);
                                let _e1152 = select(_e1150, 0f, (_e1150 < 0f));
                                let _e1155 = (_e1138 - _e200);
                                let _e1156 = (_e1139 - _e205);
                                let _e1162 = ((sqrt(((_e1155 * _e1155) + (_e1156 * _e1156))) - 11.3f) * -1f);
                                let _e1164 = select(_e1162, 0f, (_e1162 < 0f));
                                let _e1166 = select(_e1164, 1f, (_e1164 > 1f));
                                let _e1172 = select(_e323, 0f, (_e323 < 0f));
                                let _e1175 = (((_e1166 * _e1166) * (3f - (2f * _e1166))) * select(_e1172, 1f, (_e1172 > 1f)));
                                let _e1177 = (1.05f + (0.63f * _e1175));
                                let _e1178 = (_e1155 * _e1175);
                                let _e1180 = (_e1140 - (_e1178 * 0.5f));
                                let _e1181 = (_e1178 * -0.005f);
                                let _e1182 = sin(_e1181);
                                let _e1183 = cos(_e1181);
                                let _e1186 = ((_e1183 * _e1180) - (_e1182 * _e1141));
                                let _e1189 = ((_e1182 * _e1180) + (_e1183 * _e1141));
                                let _e1193 = (_e1177 * 5.4f);
                                let _e1194 = abs(_e1186);
                                let _e1198 = ((0.809017f * _e1194) + (_e1189 * 0.58778524f));
                                if (_e1198 != _e1198) {
                                    phi_13390_ = true;
                                } else {
                                    phi_13390_ = (0f >= _e1198);
                                }
                                let _e1202 = phi_13390_;
                                let _e1203 = select(_e1198, 0f, _e1202);
                                let _e1206 = (_e1194 - (_e1203 * 1.618034f));
                                let _e1207 = (-(_e1189) - (_e1203 * -1.1755705f));
                                let _e1210 = ((-0.809017f * _e1206) + (-0.58778524f * _e1207));
                                if (_e1210 != _e1210) {
                                    phi_13405_ = true;
                                } else {
                                    phi_13405_ = (0f >= _e1210);
                                }
                                let _e1214 = phi_13405_;
                                let _e1215 = select(_e1210, 0f, _e1214);
                                let _e1220 = abs((_e1206 - (_e1215 * -1.618034f)));
                                let _e1221 = ((_e1207 - (_e1215 * -1.1755705f)) - _e1193);
                                let _e1222 = (_e1177 * 2.031386f);
                                let _e1224 = ((_e1177 * 2.7959628f) - _e1193);
                                let _e1231 = (((_e1220 * _e1222) + (_e1221 * _e1224)) / ((_e1222 * _e1222) + (_e1224 * _e1224)));
                                let _e1233 = select(_e1231, 0f, (_e1231 < 0f));
                                let _e1235 = select(_e1233, 1f, (_e1233 > 1f));
                                let _e1241 = (_e1220 - (_e1222 * _e1235));
                                let _e1242 = (_e1221 - (_e1224 * _e1235));
                                let _e1251 = ((sqrt(((_e1241 * _e1241) + (_e1242 * _e1242))) * select(1f, -1f, (((_e1221 * _e1222) - (_e1220 * _e1224)) < 0f))) - (_e1177 * 1.08f));
                                let _e1252 = (((_e1186 / (_e1177 * 21.6f)) + 0.5f) - select(_e1152, 1f, (_e1152 > 1f)));
                                let _e1253 = fwidth(_e1252);
                                let _e1255 = ((_e1252 / _e1253) + 0.5f);
                                let _e1257 = select(_e1255, 0f, (_e1255 < 0f));
                                let _e1259 = select(_e1257, 1f, (_e1257 > 1f));
                                let _e1260 = (1f - _e1259);
                                let _e1263 = (0.33f * _e1259);
                                let _e1267 = (0.5f - _e1251);
                                let _e1269 = select(_e1267, 0f, (_e1267 < 0f));
                                let _e1271 = select(_e1269, 1f, (_e1269 > 1f));
                                if (_e1251 != _e1251) {
                                    phi_13420_ = true;
                                } else {
                                    phi_13420_ = (0f >= _e1251);
                                }
                                let _e1275 = phi_13420_;
                                let _e1278 = exp((select(_e1251, 0f, _e1275) * -0.5f));
                                let _e1279 = (_e1251 * -0.2f);
                                let _e1281 = select(_e1279, 0f, (_e1279 < 0f));
                                let _e1283 = select(_e1281, 1f, (_e1281 > 1f));
                                let _e1288 = (1f - ((_e1283 * _e1283) * (3f - (2f * _e1283))));
                                let _e1290 = ((_e1288 * _e1288) * 0.045f);
                                let _e1301 = ((_e1278 * _e1278) * 0.2f);
                                if (_e1271 != _e1271) {
                                    phi_13435_ = true;
                                } else {
                                    phi_13435_ = (_e1301 >= _e1271);
                                }
                                let _e1305 = phi_13435_;
                                let _e1307 = (select(_e1271, _e1301, _e1305) * _e389);
                                let _e1308 = (1f - _e1307);
                                phi_6243_ = vec4<f32>(((_e1126.x * _e1308) + ((((_e1260 + _e1263) + _e1290) * _e1271) * _e389)), ((_e1126.y * _e1308) + (((((0.85f * _e1260) + _e1263) + _e1290) * _e1271) * _e389)), ((_e1126.z * _e1308) + (((((0.2f * _e1260) + _e1263) + _e1290) * _e1271) * _e389)), ((_e1126.w * _e1308) + _e1307));
                            } else {
                                phi_6243_ = _e1126;
                            }
                            let _e1323 = phi_6243_;
                            phi_5946_ = _e1323;
                            phi_5949_ = (_e1128 + 1i);
                        } else {
                            phi_5946_ = vec4<f32>();
                            phi_5949_ = i32();
                        }
                        let _e1326 = phi_5946_;
                        let _e1328 = phi_5949_;
                        continue;
                        continuing {
                            phi_5945_ = _e1326;
                            phi_5948_ = _e1328;
                            break if !(_e1129);
                        }
                    }
                    if _e319 {
                        break;
                    }
                    let _e1936 = local_37;
                    phi_6245_ = _e1936;
                } else {
                    phi_6245_ = _e1123;
                }
                let _e1331 = phi_6245_;
                phi_6246_ = _e1331;
            } else {
                phi_6246_ = _e1123;
            }
            let _e1333 = phi_6246_;
            let _e1334 = (_e372 + _e382);
            phi_6258_ = _e1333;
            phi_6261_ = 0u;
            loop {
                let _e1338 = phi_6258_;
                let _e1340 = phi_6261_;
                local_33 = _e1338;
                local_34 = _e1338;
                local_35 = _e1338;
                local_36 = _e1338;
                let _e1341 = (_e1340 < select(_e1334, 8u, (8u < _e1334)));
                if _e1341 {
                    if (_e1340 < 8u) {
                    } else {
                        phi_15332_ = true;
                        break;
                    }
                    let _e1347 = pill_1.member[_e163].playlist_images[_e1340];
                    if (_e1347 >= 0i) {
                        let _e1349 = (_e1340 < _e372);
                        if _e1349 {
                            phi_6297_ = render_track_PillIconRow(vec2<f32>(_e355, _e358), _e374, 1f);
                            phi_6298_ = (f32(_e1340) + _e368);
                        } else {
                            phi_6297_ = render_track_PillIconRow(vec2<f32>(_e355, _e378), _e383, _e362);
                            phi_6298_ = f32((_e1340 - _e372));
                        }
                        let _e1355 = phi_6297_;
                        let _e1357 = phi_6298_;
                        let _e1358 = select(_e362, _e389, _e1349);
                        let _e1360 = (_e1355.count - 1f);
                        if (_e1360 != _e1360) {
                            phi_13465_ = true;
                        } else {
                            phi_13465_ = (0f >= _e1360);
                        }
                        let _e1364 = phi_13465_;
                        let _e1373 = (_e1355.center.x + (((_e1357 - (select(_e1360, 0f, _e1364) * 0.5f)) * 18f) * _e1355.expansion));
                        let _e1376 = (_e1355.center.y + 2f);
                        if (_e1358 > 0f) {
                            let _e1378 = (_e162.x - _e1373);
                            let _e1379 = (_e162.y - _e1376);
                            let _e1380 = abs(_e1378);
                            let _e1381 = abs(_e1379);
                            if (select(_e1381, _e1380, (_e1380 > _e1381)) < 38.88f) {
                                let _e1385 = (_e1373 - _e200);
                                let _e1386 = (_e1376 - _e205);
                                let _e1390 = sqrt(((_e1385 * _e1385) + (_e1386 * _e1386)));
                                let _e1392 = ((_e1390 - 11.3f) * -1f);
                                let _e1394 = select(_e1392, 0f, (_e1392 < 0f));
                                let _e1396 = select(_e1394, 1f, (_e1394 > 1f));
                                let _e1402 = select(_e323, 0f, (_e323 < 0f));
                                let _e1405 = (((_e1396 * _e1396) * (3f - (2f * _e1396))) * select(_e1402, 1f, (_e1402 > 1f)));
                                let _e1407 = (1.05f + (0.63f * _e1405));
                                let _e1408 = (_e1385 * _e1405);
                                let _e1410 = (_e1378 - (_e1408 * 0.5f));
                                let _e1411 = (_e1408 * -0.005f);
                                let _e1412 = sin(_e1411);
                                let _e1413 = cos(_e1411);
                                let _e1416 = ((_e1413 * _e1410) - (_e1412 * _e1379));
                                let _e1419 = ((_e1412 * _e1410) + (_e1413 * _e1379));
                                let _e1420 = (_e1407 * 21.6f);
                                if _e1349 {
                                    phi_6422_ = true;
                                } else {
                                    if _e324 {
                                        phi_6417_ = select(true, false, (_e1390 <= 10.8f));
                                    } else {
                                        phi_6417_ = true;
                                    }
                                    let _e1428 = phi_6417_;
                                    phi_6422_ = select(true, false, _e1428);
                                }
                                let _e1431 = phi_6422_;
                                let _e1432 = select(0.2f, 0f, _e1431);
                                let _e1435 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1416, _e1419), 0f, (_e1407 * 6.4800005f));
                                if (_e1435 <= 7f) {
                                    let _e1438 = vec3<f32>(((_e1416 / _e1420) + 0.5f), ((_e1419 / _e1420) + 0.5f), f32(_e1347));
                                    let _e1444 = textureSample(images, sampler_, vec2<f32>(_e1438.x, _e1438.y), i32(_e1438.z));
                                    let _e1448 = (1f - _e1432);
                                    let _e1452 = (0.24f * _e1432);
                                    let _e1456 = (0.5f - _e1435);
                                    let _e1458 = select(_e1456, 0f, (_e1456 < 0f));
                                    let _e1460 = select(_e1458, 1f, (_e1458 > 1f));
                                    if (_e1435 != _e1435) {
                                        phi_13502_ = true;
                                    } else {
                                        phi_13502_ = (0f >= _e1435);
                                    }
                                    let _e1464 = phi_13502_;
                                    let _e1467 = exp((select(_e1435, 0f, _e1464) * -0.5f));
                                    let _e1468 = (_e1435 * -0.2f);
                                    let _e1470 = select(_e1468, 0f, (_e1468 < 0f));
                                    let _e1472 = select(_e1470, 1f, (_e1470 > 1f));
                                    let _e1477 = (1f - ((_e1472 * _e1472) * (3f - (2f * _e1472))));
                                    let _e1479 = ((_e1477 * _e1477) * 0.045f);
                                    let _e1490 = ((_e1467 * _e1467) * 0.2f);
                                    if (_e1460 != _e1460) {
                                        phi_13517_ = true;
                                    } else {
                                        phi_13517_ = (_e1490 >= _e1460);
                                    }
                                    let _e1494 = phi_13517_;
                                    let _e1496 = (select(_e1460, _e1490, _e1494) * _e1358);
                                    let _e1497 = (1f - _e1496);
                                    phi_6524_ = vec4<f32>(((_e1338.x * _e1497) + (((((_e1444.x * _e1448) + _e1452) + _e1479) * _e1460) * _e1358)), ((_e1338.y * _e1497) + (((((_e1444.y * _e1448) + _e1452) + _e1479) * _e1460) * _e1358)), ((_e1338.z * _e1497) + (((((_e1444.z * _e1448) + _e1452) + _e1479) * _e1460) * _e1358)), ((_e1338.w * _e1497) + _e1496));
                                } else {
                                    phi_6524_ = _e1338;
                                }
                                let _e1512 = phi_6524_;
                                phi_6525_ = _e1512;
                            } else {
                                phi_6525_ = _e1338;
                            }
                            let _e1514 = phi_6525_;
                            phi_6526_ = _e1514;
                        } else {
                            phi_6526_ = _e1338;
                        }
                        let _e1516 = phi_6526_;
                        phi_6527_ = _e1516;
                    } else {
                        phi_6527_ = _e1338;
                    }
                    let _e1518 = phi_6527_;
                    phi_6259_ = _e1518;
                    phi_6262_ = (_e1340 + 1u);
                } else {
                    phi_6259_ = vec4<f32>();
                    phi_6262_ = u32();
                }
                let _e1521 = phi_6259_;
                let _e1523 = phi_6262_;
                continue;
                continuing {
                    phi_6258_ = _e1521;
                    phi_6261_ = _e1523;
                    phi_15332_ = _e319;
                    break if !(_e1341);
                }
            }
            let _e1526 = phi_15332_;
            if _e1526 {
                break;
            }
            local_22 = array<u32, 2>(0u, 1u);
            phi_15367_ = _e1526;
            phi_6535_ = 0u;
            phi_6538_ = -1000000f;
            loop {
                let _e1528 = phi_15367_;
                let _e1530 = phi_6535_;
                let _e1532 = phi_6538_;
                local_32 = _e1532;
                let _e1533 = (_e1530 < 2u);
                if _e1533 {
                    if _e1533 {
                    } else {
                        phi_15386_ = true;
                        break;
                    }
                    let _e1535 = local_22[_e1530];
                    if (_e1535 < 2u) {
                    } else {
                        phi_15386_ = true;
                        break;
                    }
                    let _e1544 = pill_1.member[_e163].text.lines[_e1535].min[0u];
                    let _e1552 = pill_1.member[_e163].text.lines[_e1535].min[1u];
                    let _e1560 = pill_1.member[_e163].text.lines[_e1535].max[0u];
                    let _e1568 = pill_1.member[_e163].text.lines[_e1535].max[1u];
                    let _e1576 = pill_1.member[_e163].text.lines[_e1535].origin[0u];
                    let _e1584 = pill_1.member[_e163].text.lines[_e1535].origin[1u];
                    let _e1591 = pill_1.member[_e163].text.lines[_e1535].size;
                    let _e1598 = pill_1.member[_e163].text.lines[_e1535].weight;
                    let _e1605 = pill_1.member[_e163].text.lines[_e1535].count;
                    let _e1612 = pill_1.member[_e163].text.lines[_e1535].first;
                    if (_e581 < _e1544) {
                        phi_15391_ = _e1528;
                        phi_6806_ = f32();
                        phi_6807_ = true;
                    } else {
                        if (_e581 > _e1560) {
                            phi_15392_ = _e1528;
                            phi_6804_ = f32();
                            phi_6805_ = true;
                        } else {
                            if (_e582 < _e1552) {
                                phi_15393_ = _e1528;
                                phi_6802_ = f32();
                                phi_6803_ = true;
                            } else {
                                let _e1616 = (_e582 > _e1568);
                                if _e1616 {
                                    phi_15394_ = _e1528;
                                    phi_6801_ = f32();
                                } else {
                                    phi_6618_ = _e1605;
                                    phi_6621_ = 0u;
                                    loop {
                                        let _e1618 = phi_6618_;
                                        let _e1620 = phi_6621_;
                                        local_29 = _e1620;
                                        let _e1621 = (_e1620 < _e1618);
                                        if _e1621 {
                                            let _e1624 = (_e1620 + ((_e1618 - _e1620) / 2u));
                                            let _e1625 = (_e1612 + _e1624);
                                            if (_e1625 < 128u) {
                                            } else {
                                                phi_15364_ = true;
                                                break;
                                            }
                                            let _e1633 = pill_1.member[_e163].text.glyphs[_e1625].x;
                                            let _e1636 = (_e1633 <= ((_e581 - _e1576) / _e1591));
                                            if _e1636 {
                                                phi_6655_ = (_e1624 + 1u);
                                            } else {
                                                phi_6655_ = _e1620;
                                            }
                                            let _e1639 = phi_6655_;
                                            phi_6619_ = select(_e1624, _e1618, _e1636);
                                            phi_6622_ = _e1639;
                                        } else {
                                            phi_6619_ = u32();
                                            phi_6622_ = u32();
                                        }
                                        let _e1642 = phi_6619_;
                                        let _e1644 = phi_6622_;
                                        continue;
                                        continuing {
                                            phi_6618_ = _e1642;
                                            phi_6621_ = _e1644;
                                            phi_15364_ = _e1528;
                                            break if !(_e1621);
                                        }
                                    }
                                    let _e1647 = phi_15364_;
                                    phi_15386_ = _e1647;
                                    if _e1647 {
                                        break;
                                    }
                                    let _e1649 = local_29;
                                    let _e1650 = (_e1649 + 1u);
                                    phi_15372_ = _e1647;
                                    phi_6663_ = select(_e1650, _e1605, (_e1605 < _e1650));
                                    phi_6666_ = -1000000f;
                                    loop {
                                        let _e1654 = phi_15372_;
                                        let _e1656 = phi_6663_;
                                        let _e1658 = phi_6666_;
                                        local_38 = _e1658;
                                        if (_e1656 > 0u) {
                                            let _e1660 = (_e1656 - 1u);
                                            let _e1661 = (_e1612 + _e1660);
                                            if (_e1661 < 128u) {
                                            } else {
                                                phi_15376_ = true;
                                                break;
                                            }
                                            let _e1669 = pill_1.member[_e163].text.glyphs[_e1661].x;
                                            let _e1676 = pill_1.member[_e163].text.glyphs[_e1661].glyph;
                                            if (_e1676 < arrayLength((&glyphs.member))) {
                                            } else {
                                                phi_15376_ = true;
                                                break;
                                            }
                                            let _e1682 = glyphs.member[_e1676].min[0u];
                                            let _e1687 = glyphs.member[_e1676].min[1u];
                                            let _e1692 = glyphs.member[_e1676].max[0u];
                                            let _e1697 = glyphs.member[_e1676].max[1u];
                                            let _e1701 = glyphs.member[_e1676].start;
                                            let _e1705 = glyphs.member[_e1676].count;
                                            let _e1708 = (((_e581 - _e1576) / _e1591) - _e1669);
                                            let _e1711 = (-((_e582 - _e1584)) / _e1591);
                                            let _e1712 = (3.5f / _e1591);
                                            let _e1713 = (_e1692 + _e1712);
                                            let _e1714 = (_e1708 > _e1713);
                                            if _e1714 {
                                                phi_15378_ = _e1654;
                                                phi_6794_ = f32();
                                            } else {
                                                if (_e1708 >= (_e1682 - _e1712)) {
                                                    if (_e1711 >= (_e1687 - _e1712)) {
                                                        if (_e1708 <= _e1713) {
                                                            if (_e1711 <= (_e1697 + _e1712)) {
                                                                phi_6753_ = 0u;
                                                                phi_6756_ = 0i;
                                                                phi_6758_ = 340282350000000000000000000000000000000f;
                                                                loop {
                                                                    let _e1723 = phi_6753_;
                                                                    let _e1725 = phi_6756_;
                                                                    let _e1727 = phi_6758_;
                                                                    local_30 = _e1727;
                                                                    local_31 = _e1725;
                                                                    let _e1728 = (_e1723 < _e1705);
                                                                    if _e1728 {
                                                                        let _e1729 = (_e1701 + _e1723);
                                                                        if (_e1729 < arrayLength((&edges.member))) {
                                                                        } else {
                                                                            phi_15369_ = true;
                                                                            break;
                                                                        }
                                                                        let _e1733 = edges.member[_e1729];
                                                                        let _e1735 = cantus_render_text_edge_distance(_e1733, _e1598, vec2<f32>(_e1708, _e1711));
                                                                        if (_e1727 != _e1727) {
                                                                            phi_13532_ = true;
                                                                        } else {
                                                                            phi_13532_ = (_e1735.member <= _e1727);
                                                                        }
                                                                        let _e1741 = phi_13532_;
                                                                        phi_6754_ = (_e1723 + 1u);
                                                                        phi_6757_ = (_e1725 + _e1735.member_1);
                                                                        phi_6759_ = select(_e1727, _e1735.member, _e1741);
                                                                    } else {
                                                                        phi_6754_ = u32();
                                                                        phi_6757_ = i32();
                                                                        phi_6759_ = f32();
                                                                    }
                                                                    let _e1746 = phi_6754_;
                                                                    let _e1748 = phi_6757_;
                                                                    let _e1750 = phi_6759_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_6753_ = _e1746;
                                                                        phi_6756_ = _e1748;
                                                                        phi_6758_ = _e1750;
                                                                        phi_15369_ = _e1654;
                                                                        break if !(_e1728);
                                                                    }
                                                                }
                                                                let _e1753 = phi_15369_;
                                                                phi_15376_ = _e1753;
                                                                if _e1753 {
                                                                    break;
                                                                }
                                                                let _e1755 = local_30;
                                                                let _e1759 = local_31;
                                                                let _e1762 = ((sqrt(_e1755) * _e1591) * select(1f, -1f, (_e1759 == 0i)));
                                                                if (_e1658 != _e1658) {
                                                                    phi_13547_ = true;
                                                                } else {
                                                                    phi_13547_ = (_e1762 >= _e1658);
                                                                }
                                                                let _e1766 = phi_13547_;
                                                                phi_15382_ = _e1753;
                                                                phi_6790_ = select(_e1658, _e1762, _e1766);
                                                            } else {
                                                                phi_15382_ = _e1654;
                                                                phi_6790_ = _e1658;
                                                            }
                                                            let _e1769 = phi_15382_;
                                                            let _e1771 = phi_6790_;
                                                            phi_15381_ = _e1769;
                                                            phi_6791_ = _e1771;
                                                        } else {
                                                            phi_15381_ = _e1654;
                                                            phi_6791_ = _e1658;
                                                        }
                                                        let _e1773 = phi_15381_;
                                                        let _e1775 = phi_6791_;
                                                        phi_15380_ = _e1773;
                                                        phi_6792_ = _e1775;
                                                    } else {
                                                        phi_15380_ = _e1654;
                                                        phi_6792_ = _e1658;
                                                    }
                                                    let _e1777 = phi_15380_;
                                                    let _e1779 = phi_6792_;
                                                    phi_15379_ = _e1777;
                                                    phi_6793_ = _e1779;
                                                } else {
                                                    phi_15379_ = _e1654;
                                                    phi_6793_ = _e1658;
                                                }
                                                let _e1781 = phi_15379_;
                                                let _e1783 = phi_6793_;
                                                phi_15378_ = _e1781;
                                                phi_6794_ = _e1783;
                                            }
                                            let _e1785 = phi_15378_;
                                            let _e1787 = phi_6794_;
                                            phi_15377_ = _e1785;
                                            phi_6664_ = _e1660;
                                            phi_6667_ = _e1787;
                                            phi_6796_ = select(true, false, _e1714);
                                        } else {
                                            phi_15377_ = _e1654;
                                            phi_6664_ = u32();
                                            phi_6667_ = f32();
                                            phi_6796_ = false;
                                        }
                                        let _e1790 = phi_15377_;
                                        let _e1792 = phi_6664_;
                                        let _e1794 = phi_6667_;
                                        let _e1796 = phi_6796_;
                                        continue;
                                        continuing {
                                            phi_15372_ = _e1790;
                                            phi_6663_ = _e1792;
                                            phi_6666_ = _e1794;
                                            phi_15376_ = _e1790;
                                            break if !(_e1796);
                                        }
                                    }
                                    let _e1799 = phi_15376_;
                                    phi_15386_ = _e1799;
                                    if _e1799 {
                                        break;
                                    }
                                    phi_15394_ = _e1799;
                                    let _e1996 = local_38;
                                    phi_6801_ = _e1996;
                                }
                                let _e1801 = phi_15394_;
                                let _e1803 = phi_6801_;
                                phi_15393_ = _e1801;
                                phi_6802_ = _e1803;
                                phi_6803_ = _e1616;
                            }
                            let _e1805 = phi_15393_;
                            let _e1807 = phi_6802_;
                            let _e1809 = phi_6803_;
                            phi_15392_ = _e1805;
                            phi_6804_ = _e1807;
                            phi_6805_ = _e1809;
                        }
                        let _e1811 = phi_15392_;
                        let _e1813 = phi_6804_;
                        let _e1815 = phi_6805_;
                        phi_15391_ = _e1811;
                        phi_6806_ = _e1813;
                        phi_6807_ = _e1815;
                    }
                    let _e1817 = phi_15391_;
                    let _e1819 = phi_6806_;
                    let _e1821 = phi_6807_;
                    let _e1822 = select(_e1819, -1000000f, _e1821);
                    if (_e1532 != _e1532) {
                        phi_13562_ = true;
                    } else {
                        phi_13562_ = (_e1822 >= _e1532);
                    }
                    let _e1826 = phi_13562_;
                    phi_15387_ = _e1817;
                    phi_6536_ = (_e1530 + 1u);
                    phi_6539_ = select(_e1532, _e1822, _e1826);
                } else {
                    phi_15387_ = _e1528;
                    phi_6536_ = u32();
                    phi_6539_ = f32();
                }
                let _e1830 = phi_15387_;
                let _e1832 = phi_6536_;
                let _e1834 = phi_6539_;
                continue;
                continuing {
                    phi_15367_ = _e1830;
                    phi_6535_ = _e1832;
                    phi_6538_ = _e1834;
                    phi_15386_ = _e1830;
                    break if !(_e1533);
                }
            }
            let _e1837 = phi_15386_;
            if _e1837 {
                break;
            }
            let _e1842 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e581 - (_e192 + _e189)), (_e582 - _e189)), 0f, _e189);
            let _e1843 = (_e1842 * 0.125f);
            let _e1845 = select(_e1843, 0f, (_e1843 < 0f));
            let _e1847 = select(_e1845, 1f, (_e1845 > 1f));
            let _e1853 = local_32;
            let _e1855 = ((_e1853 * 1.25f) + 0.5f);
            let _e1857 = select(_e1855, 0f, (_e1855 < 0f));
            let _e1859 = select(_e1857, 1f, (_e1857 > 1f));
            let _e1865 = ((((_e1859 * _e1859) * (3f - (2f * _e1859))) * ((_e1847 * _e1847) * (3f - (2f * _e1847)))) * _e541);
            let _e1866 = (1f - _e1865);
            let _e1868 = local_33;
            let _e1872 = local_34;
            let _e1876 = local_35;
            let _e1880 = local_36;
            let _e1883 = (0.94f * _e1865);
            let _e1891 = (((_e1880.w * _e1866) + _e1865) * _e558);
            if (_e1891 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e1868.x * _e1866) + _e1883) * _e558), (((_e1872.y * _e1866) + _e1883) * _e558), (((_e1876.z * _e1866) + _e1883) * _e558), _e1891);
            break;
        }
    }
    return;
}

fn render_status_vertex_impl() {
    var local_39: array<u32, 6>;
    var phi_7040_: u32;
    var phi_7043_: f32;
    var phi_7067_: bool;
    var phi_7076_: bool;
    var phi_13579_: bool;
    var phi_13580_: bool;
    var phi_13581_: bool;
    var phi_7083_: f32;
    var phi_7041_: u32;
    var phi_7044_: f32;
    var phi_15395_: bool;
    var local_40: f32;
    var local_41: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e33 = vertex_5;
            let _e34 = _isthmus_instance_index_7;
            phi_7040_ = 0u;
            phi_7043_ = 12f;
            loop {
                let _e36 = phi_7040_;
                let _e38 = phi_7043_;
                local_40 = _e38;
                local_41 = _e38;
                let _e39 = (_e36 < 5u);
                if _e39 {
                    local_39 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                    if (_e36 < 6u) {
                    } else {
                        phi_15395_ = true;
                        break;
                    }
                    let _e42 = local_39[_e36];
                    let _e46 = pill_2.member[_e34].battery_level;
                    if (_e46 >= -1f) {
                        phi_7067_ = (_e46 <= 1f);
                    } else {
                        phi_7067_ = false;
                    }
                    let _e50 = phi_7067_;
                    if _e50 {
                        phi_7076_ = true;
                    } else {
                        phi_7076_ = select(true, false, (_e42 == 2u));
                    }
                    let _e54 = phi_7076_;
                    if _e54 {
                        switch bitcast<i32>(_e42) {
                            case 0: {
                                phi_13579_ = true;
                                phi_13580_ = false;
                                phi_13581_ = false;
                                break;
                            }
                            case 1: {
                                phi_13579_ = true;
                                phi_13580_ = false;
                                phi_13581_ = false;
                                break;
                            }
                            case 2: {
                                phi_13579_ = false;
                                phi_13580_ = true;
                                phi_13581_ = false;
                                break;
                            }
                            case 3: {
                                phi_13579_ = false;
                                phi_13580_ = true;
                                phi_13581_ = false;
                                break;
                            }
                            case 4: {
                                phi_13579_ = false;
                                phi_13580_ = false;
                                phi_13581_ = true;
                                break;
                            }
                            case 5: {
                                phi_13579_ = false;
                                phi_13580_ = false;
                                phi_13581_ = true;
                                break;
                            }
                            default: {
                                phi_13579_ = bool();
                                phi_13580_ = bool();
                                phi_13581_ = bool();
                                break;
                            }
                        }
                        let _e57 = phi_13579_;
                        let _e59 = phi_13580_;
                        let _e61 = phi_13581_;
                        let _e62 = select(_e59, false, _e57);
                        phi_7083_ = (_e38 + (select(select(80f, 32f, _e62), 24f, select(select(_e61, false, _e57), false, _e62)) + 8f));
                    } else {
                        phi_7083_ = _e38;
                    }
                    let _e70 = phi_7083_;
                    phi_7041_ = (_e36 + 1u);
                    phi_7044_ = _e70;
                } else {
                    phi_7041_ = u32();
                    phi_7044_ = f32();
                }
                let _e73 = phi_7041_;
                let _e75 = phi_7044_;
                continue;
                continuing {
                    phi_7040_ = _e73;
                    phi_7043_ = _e75;
                    phi_15395_ = false;
                    break if !(_e39);
                }
            }
            let _e78 = phi_15395_;
            if _e78 {
                break;
            }
            let _e80 = local_40;
            let _e86 = frame.member[0u].screen_size[0u];
            let _e92 = frame.member[0u].panel_top;
            let _e102 = frame.member[0u].panel_height;
            let _e105 = local_41;
            let _e109 = (((_e86 - (_e80 + 36f)) - 56f) + (f32((_e33 & 1u)) * (_e105 + 132f)));
            let _e110 = ((_e92 - 48f) + (f32((_e33 >> bitcast<u32>(1i))) * (_e102 + 96f)));
            let _e115 = frame.member[0u].screen_size[1u];
            let _e118 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e109, _e110), vec2<f32>(_e86, _e115));
            out_position = _e118;
            out_pixel[0u] = _e109;
            out_pixel[1u] = _e110;
            out_isthmus_instance_index_1 = _e34;
            break;
        }
    }
    return;
}

fn render_status_fragment_impl() {
    var local_42: array<u32, 6>;
    var local_43: array<u32, 6>;
    var local_44: array<u32, 6>;
    var phi_7202_: u32;
    var phi_7205_: f32;
    var phi_7229_: bool;
    var phi_7238_: bool;
    var phi_13671_: bool;
    var phi_13672_: bool;
    var phi_13673_: bool;
    var phi_7245_: f32;
    var phi_7203_: u32;
    var phi_7206_: f32;
    var phi_15406_: bool;
    var local_45: f32;
    var phi_7292_: vec2<f32>;
    var phi_7295_: f32;
    var phi_7297_: u32;
    var phi_13766_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_13777_: bool;
    var phi_7293_: vec2<f32>;
    var phi_7296_: f32;
    var phi_7298_: u32;
    var phi_15417_: bool;
    var phi_7438_: f32;
    var local_46: vec2<f32>;
    var local_47: vec2<f32>;
    var phi_13795_: bool;
    var phi_13810_: bool;
    var phi_13825_: bool;
    var phi_13842_: bool;
    var phi_7814_: u32;
    var phi_7817_: f32;
    var phi_7819_: f32;
    var phi_7847_: bool;
    var phi_7851_: bool;
    var phi_13857_: bool;
    var phi_13858_: bool;
    var phi_13859_: bool;
    var phi_7868_: f32;
    var phi_7869_: f32;
    var phi_7870_: f32;
    var phi_7871_: bool;
    var phi_7876_: u32;
    var phi_7815_: u32;
    var phi_7818_: f32;
    var phi_7820_: f32;
    var phi_7877_: u32;
    var phi_7878_: f32;
    var phi_7879_: bool;
    var phi_15435_: bool;
    var phi_11751_: bool;
    var phi_11750_: f32;
    var phi_11748_: u32;
    var local_48: f32;
    var phi_7885_: render_track_PaletteColor;
    var phi_7891_: render_track_PaletteColor;
    var phi_7902_: vec2<f32>;
    var phi_7903_: bool;
    var phi_13947_: i32;
    var phi_13948_: f32;
    var phi_13949_: f32;
    var phi_13950_: vec2<f32>;
    var phi_13975_: i32;
    var phi_13976_: f32;
    var phi_13977_: f32;
    var phi_13978_: vec2<f32>;
    var local_49: f32;
    var phi_8069_: vec2<f32>;
    var phi_8073_: u32;
    var phi_8076_: f32;
    var phi_8100_: bool;
    var phi_8109_: bool;
    var phi_13993_: bool;
    var phi_13994_: bool;
    var phi_13995_: bool;
    var phi_8116_: f32;
    var phi_8074_: u32;
    var phi_8077_: f32;
    var phi_15490_: bool;
    var local_50: f32;
    var phi_14056_: i32;
    var phi_14057_: f32;
    var phi_14058_: f32;
    var phi_14059_: vec2<f32>;
    var phi_14084_: i32;
    var phi_14085_: f32;
    var phi_14086_: f32;
    var phi_14087_: vec2<f32>;
    var local_51: f32;
    var phi_8234_: vec2<f32>;
    var phi_15523_: bool;
    var phi_8248_: vec2<f32>;
    var phi_14102_: bool;
    var phi_14117_: bool;
    var phi_14118_: bool;
    var phi_14119_: bool;
    var phi_14144_: bool;
    var phi_14159_: bool;
    var phi_14174_: bool;
    var phi_14189_: bool;
    var phi_14190_: bool;
    var phi_14191_: bool;
    var phi_14295_: bool;
    var phi_14310_: bool;
    var phi_14325_: bool;
    var phi_14340_: bool;
    var phi_14355_: bool;
    var phi_14370_: bool;
    var phi_14385_: bool;
    var phi_15516_: bool;
    var phi_10345_: vec3<f32>;
    var phi_10346_: bool;
    var phi_14400_: bool;
    var phi_14445_: bool;
    var phi_14460_: bool;
    var phi_14475_: bool;
    var phi_10746_: f32;
    var phi_14490_: bool;
    var phi_10773_: vec3<f32>;
    var phi_10783_: bool;
    var phi_10853_: u32;
    var phi_10856_: u32;
    var phi_10890_: u32;
    var phi_10854_: u32;
    var phi_10857_: u32;
    var phi_15687_: bool;
    var local_52: u32;
    var phi_15737_: bool;
    var phi_10898_: u32;
    var phi_10901_: f32;
    var phi_10988_: u32;
    var phi_10991_: i32;
    var phi_10993_: f32;
    var phi_14505_: bool;
    var phi_10989_: u32;
    var phi_10992_: i32;
    var phi_10994_: f32;
    var phi_15734_: bool;
    var local_53: f32;
    var local_54: i32;
    var phi_14520_: bool;
    var phi_15747_: bool;
    var phi_11025_: f32;
    var phi_15746_: bool;
    var phi_11026_: f32;
    var phi_15745_: bool;
    var phi_11027_: f32;
    var phi_15744_: bool;
    var phi_11028_: f32;
    var phi_15743_: bool;
    var phi_11029_: f32;
    var phi_15742_: bool;
    var phi_10899_: u32;
    var phi_10902_: f32;
    var phi_11031_: bool;
    var phi_15741_: bool;
    var phi_11036_: f32;
    var phi_11037_: f32;
    var phi_11038_: bool;
    var phi_11039_: f32;
    var phi_11040_: bool;
    var phi_11041_: f32;
    var phi_11042_: bool;
    var phi_11047_: f32;
    var local_55: f32;
    var local_56: f32;
    var local_57: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e228 = pixel_2;
            let _e229 = _isthmus_instance_index_9;
            phi_7202_ = 0u;
            phi_7205_ = 12f;
            loop {
                let _e237 = phi_7202_;
                let _e239 = phi_7205_;
                local_45 = _e239;
                let _e240 = (_e237 < 5u);
                if _e240 {
                    local_44 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                    if (_e237 < 6u) {
                    } else {
                        phi_15406_ = true;
                        break;
                    }
                    let _e243 = local_44[_e237];
                    let _e247 = pill_2.member[_e229].battery_level;
                    if (_e247 >= -1f) {
                        phi_7229_ = (_e247 <= 1f);
                    } else {
                        phi_7229_ = false;
                    }
                    let _e251 = phi_7229_;
                    if _e251 {
                        phi_7238_ = true;
                    } else {
                        phi_7238_ = select(true, false, (_e243 == 2u));
                    }
                    let _e255 = phi_7238_;
                    if _e255 {
                        switch bitcast<i32>(_e243) {
                            case 0: {
                                phi_13671_ = true;
                                phi_13672_ = false;
                                phi_13673_ = false;
                                break;
                            }
                            case 1: {
                                phi_13671_ = true;
                                phi_13672_ = false;
                                phi_13673_ = false;
                                break;
                            }
                            case 2: {
                                phi_13671_ = false;
                                phi_13672_ = true;
                                phi_13673_ = false;
                                break;
                            }
                            case 3: {
                                phi_13671_ = false;
                                phi_13672_ = true;
                                phi_13673_ = false;
                                break;
                            }
                            case 4: {
                                phi_13671_ = false;
                                phi_13672_ = false;
                                phi_13673_ = true;
                                break;
                            }
                            case 5: {
                                phi_13671_ = false;
                                phi_13672_ = false;
                                phi_13673_ = true;
                                break;
                            }
                            default: {
                                phi_13671_ = bool();
                                phi_13672_ = bool();
                                phi_13673_ = bool();
                                break;
                            }
                        }
                        let _e258 = phi_13671_;
                        let _e260 = phi_13672_;
                        let _e262 = phi_13673_;
                        let _e263 = select(_e260, false, _e258);
                        phi_7245_ = (_e239 + (select(select(80f, 32f, _e263), 24f, select(select(_e262, false, _e258), false, _e263)) + 8f));
                    } else {
                        phi_7245_ = _e239;
                    }
                    let _e271 = phi_7245_;
                    phi_7203_ = (_e237 + 1u);
                    phi_7206_ = _e271;
                } else {
                    phi_7203_ = u32();
                    phi_7206_ = f32();
                }
                let _e274 = phi_7203_;
                let _e276 = phi_7206_;
                continue;
                continuing {
                    phi_7202_ = _e274;
                    phi_7205_ = _e276;
                    phi_15406_ = false;
                    break if !(_e240);
                }
            }
            let _e279 = phi_15406_;
            if _e279 {
                break;
            }
            let _e281 = local_45;
            let _e282 = (_e281 + 36f);
            let _e287 = frame.member[0u].screen_size[0u];
            let _e289 = ((_e287 - _e282) - 8f);
            let _e293 = frame.member[0u].panel_height;
            let _e297 = frame.member[0u].panel_top;
            let _e298 = (_e228.x - _e289);
            let _e299 = (_e228.y - _e297);
            let _e300 = (_e282 * 0.5f);
            let _e301 = (_e293 * 0.5f);
            let _e305 = ((_e282 - _e293) * 0.5f);
            let _e307 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e298 - _e300), (_e299 - _e301)), _e305, _e301);
            let _e312 = frame.member[0u].mouse_pos[0u];
            let _e317 = frame.member[0u].mouse_pos[1u];
            let _e323 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e312 - _e289) - _e300), ((_e317 - _e297) - _e301)), _e305, _e301);
            phi_7292_ = vec2<f32>(0f, 0f);
            phi_7295_ = 0f;
            phi_7297_ = 0u;
            loop {
                let _e325 = phi_7292_;
                let _e327 = phi_7295_;
                let _e329 = phi_7297_;
                local_46 = _e325;
                local_47 = _e325;
                local_55 = _e327;
                local_56 = _e327;
                let _e330 = (_e329 < 4u);
                if _e330 {
                    if _e330 {
                    } else {
                        phi_15417_ = true;
                        break;
                    }
                    let _e337 = frame.member[0u].ripples[_e329].origin[0u];
                    let _e344 = frame.member[0u].ripples[_e329].origin[1u];
                    let _e351 = frame.member[0u].ripples[_e329].animation[0u];
                    let _e358 = frame.member[0u].ripples[_e329].animation[1u];
                    let _e362 = frame.member[0u].time;
                    let _e364 = ((_e362 - _e351) * 1.2f);
                    let _e366 = select(_e364, 0f, (_e364 < 0f));
                    let _e368 = select(_e366, 1f, (_e366 > 1f));
                    let _e370 = (_e228 - vec2<f32>(_e337, _e344));
                    let _e376 = sqrt(((_e370.x * _e370.x) + (_e370.y * _e370.y)));
                    if (_e376 > 0.001f) {
                        phi_13766_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e370.x / _e376), (_e370.y / _e376)), _e376);
                    } else {
                        phi_13766_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e376);
                    }
                    let _e384 = phi_13766_;
                    let _e394 = ((abs((_e384.unnamed_1 - (_e368 * 600f))) - 80f) * -0.0125f);
                    let _e396 = select(_e394, 0f, (_e394 < 0f));
                    let _e398 = select(_e396, 1f, (_e396 > 1f));
                    let _e404 = (1f - _e368);
                    let _e405 = ((((_e398 * _e398) * (3f - (2f * _e398))) * _e358) * _e404);
                    let _e418 = (_e327 + (_e405 * 0.5f));
                    if (_e418 != _e418) {
                        phi_13777_ = true;
                    } else {
                        phi_13777_ = (1f <= _e418);
                    }
                    let _e422 = phi_13777_;
                    phi_7293_ = vec2<f32>((_e325.x + (((_e384.unnamed.x * _e405) * _e404) * 0.5f)), (_e325.y + (((_e384.unnamed.y * _e405) * _e404) * 0.5f)));
                    phi_7296_ = select(_e418, 1f, _e422);
                    phi_7298_ = (_e329 + 1u);
                } else {
                    phi_7293_ = vec2<f32>();
                    phi_7296_ = f32();
                    phi_7298_ = u32();
                }
                let _e426 = phi_7293_;
                let _e428 = phi_7296_;
                let _e430 = phi_7298_;
                continue;
                continuing {
                    phi_7292_ = _e426;
                    phi_7295_ = _e428;
                    phi_7297_ = _e430;
                    phi_15417_ = _e279;
                    break if !(_e330);
                }
            }
            let _e433 = phi_15417_;
            if _e433 {
                break;
            }
            let _e437 = frame.member[0u].mouse_pressure;
            if (_e437 > 0f) {
                let _e439 = (_e228.x - _e312);
                let _e440 = (_e228.y - _e317);
                let _e446 = ((sqrt(((_e439 * _e439) + (_e440 * _e440))) - 150f) * -0.006666667f);
                let _e448 = select(_e446, 0f, (_e446 < 0f));
                let _e450 = select(_e448, 1f, (_e448 > 1f));
                phi_7438_ = ((((_e450 * _e450) * (3f - (2f * _e450))) * _e437) * 8f);
            } else {
                phi_7438_ = 0f;
            }
            let _e458 = phi_7438_;
            let _e460 = local_46;
            let _e463 = local_47;
            let _e466 = ((_e323 - 0.5f) * -1f);
            let _e468 = select(_e466, 0f, (_e466 < 0f));
            let _e470 = select(_e468, 1f, (_e468 > 1f));
            let _e483 = (_e307 - (((_e458 * ((_e470 * _e470) * (3f - (2f * _e470)))) + (sqrt(((_e460.x * _e460.x) + (_e463.y * _e463.y))) * 22f)) * 0.5f));
            let _e484 = fwidth(_e483);
            if (_e484 != _e484) {
                phi_13795_ = true;
            } else {
                phi_13795_ = (0.55f >= _e484);
            }
            let _e488 = phi_13795_;
            let _e489 = select(_e484, 0.55f, _e488);
            let _e493 = ((_e483 - _e489) / (-(_e489) - _e489));
            let _e495 = select(_e493, 0f, (_e493 < 0f));
            let _e497 = select(_e495, 1f, (_e495 > 1f));
            let _e501 = ((_e497 * _e497) * (3f - (2f * _e497)));
            let _e502 = (_e483 != _e483);
            if _e502 {
                phi_13810_ = true;
            } else {
                phi_13810_ = (0f >= _e483);
            }
            let _e505 = phi_13810_;
            let _e509 = (exp((select(_e483, 0f, _e505) * -0.3f)) * 0.16f);
            if (_e501 != _e501) {
                phi_13825_ = true;
            } else {
                phi_13825_ = (_e509 >= _e501);
            }
            let _e513 = phi_13825_;
            let _e514 = select(_e501, _e509, _e513);
            if (_e514 <= 0.0009765625f) {
                discard;
            }
            let _e516 = (_e298 / _e282);
            let _e517 = (_e299 / _e293);
            if _e502 {
                phi_13842_ = true;
            } else {
                phi_13842_ = (0f <= _e483);
            }
            let _e522 = phi_13842_;
            let _e525 = (1f + (select(_e483, 0f, _e522) * 0.008333334f));
            let _e527 = select(_e525, 0f, (_e525 < 0f));
            let _e529 = select(_e527, 0.6f, (_e527 > 0.6f));
            let _e539 = ((_e517 - (((_e517 - 0.5f) * _e529) * 0.08f)) - (_e463.y * 0.04f));
            let _e543 = pill_2.member[_e229].sun_height;
            let _e547 = pill_2.member[_e229].conditions;
            let _e551 = frame.member[0u].time;
            let _e559 = ((_e539 - 1f) * -1f);
            let _e561 = select(_e559, 0f, (_e559 < 0f));
            let _e563 = select(_e561, 1f, (_e561 > 1f));
            let _e567 = ((_e563 * _e563) * (3f - (2f * _e563)));
            let _e569 = ((_e543 - -0.04f) * 4.1666665f);
            let _e571 = select(_e569, 0f, (_e569 < 0f));
            let _e573 = select(_e571, 1f, (_e571 > 1f));
            let _e577 = ((_e573 * _e573) * (3f - (2f * _e573)));
            let _e579 = ((_e543 - -0.2f) * 4.5454545f);
            let _e581 = select(_e579, 0f, (_e579 < 0f));
            let _e583 = select(_e581, 1f, (_e581 > 1f));
            let _e588 = (1f - _e577);
            let _e589 = (((_e583 * _e583) * (3f - (2f * _e583))) * _e588);
            let _e590 = (1f - _e567);
            let _e602 = (0.65f * _e590);
            let _e626 = (1f - _e589);
            let _e640 = (((_e547.cloud * 0.34f) + (_e547.rain * 0.16f)) + (_e547.hail * 0.08f));
            let _e641 = (1f - _e640);
            let _e652 = (1f - (_e547.snow * 0.16f));
            let _e656 = (_e547.snow * 0.1312f);
            let _e661 = (1f - (_e547.fog * 0.62f));
            let _e674 = ((sin((_e551 * 2.7f)) - 0.92f) * 12.500003f);
            let _e676 = select(_e674, 0f, (_e674 < 0f));
            let _e678 = select(_e676, 1f, (_e676 > 1f));
            let _e683 = (((_e678 * _e678) * (3f - (2f * _e678))) * _e547.lightning);
            let _e685 = (1f - (_e683 * 0.45f));
            let _e696 = ((_e539 - 0.12f) * -8.333334f);
            let _e698 = select(_e696, 0f, (_e696 < 0f));
            let _e700 = select(_e698, 1f, (_e698 > 1f));
            let _e707 = ((_e483 - 5f) * -0.125f);
            let _e709 = select(_e707, 0f, (_e707 < 0f));
            let _e711 = select(_e709, 1f, (_e709 > 1f));
            let _e717 = ((((_e700 * _e700) * (3f - (2f * _e700))) * 0.12f) + (((_e711 * _e711) * (3f - (2f * _e711))) * 0.08f));
            let _e721 = (((_e516 - (((_e516 - 0.5f) * _e529) * 0.08f)) - (_e460.x * 0.04f)) * _e282);
            phi_7814_ = 0u;
            phi_7817_ = 0f;
            phi_7819_ = 12f;
            loop {
                let _e724 = phi_7814_;
                let _e726 = phi_7817_;
                let _e728 = phi_7819_;
                local_48 = _e726;
                let _e729 = (_e724 < 6u);
                if _e729 {
                    local_43 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                    if _e729 {
                    } else {
                        phi_15435_ = true;
                        phi_11751_ = bool();
                        phi_11750_ = f32();
                        phi_11748_ = u32();
                        break;
                    }
                    let _e731 = local_43[_e724];
                    if (_e731 == 2u) {
                        let _e736 = pill_2.member[_e229].battery_level;
                        if (_e736 >= -1f) {
                            phi_7847_ = (_e736 <= 1f);
                        } else {
                            phi_7847_ = false;
                        }
                        let _e740 = phi_7847_;
                        phi_7851_ = _e740;
                    } else {
                        phi_7851_ = true;
                    }
                    let _e742 = phi_7851_;
                    if _e742 {
                        switch bitcast<i32>(_e731) {
                            case 0: {
                                phi_13857_ = true;
                                phi_13858_ = false;
                                phi_13859_ = false;
                                break;
                            }
                            case 1: {
                                phi_13857_ = true;
                                phi_13858_ = false;
                                phi_13859_ = false;
                                break;
                            }
                            case 2: {
                                phi_13857_ = false;
                                phi_13858_ = true;
                                phi_13859_ = false;
                                break;
                            }
                            case 3: {
                                phi_13857_ = false;
                                phi_13858_ = true;
                                phi_13859_ = false;
                                break;
                            }
                            case 4: {
                                phi_13857_ = false;
                                phi_13858_ = false;
                                phi_13859_ = true;
                                break;
                            }
                            case 5: {
                                phi_13857_ = false;
                                phi_13858_ = false;
                                phi_13859_ = true;
                                break;
                            }
                            default: {
                                phi_13857_ = bool();
                                phi_13858_ = bool();
                                phi_13859_ = bool();
                                break;
                            }
                        }
                        let _e745 = phi_13857_;
                        let _e747 = phi_13858_;
                        let _e749 = phi_13859_;
                        let _e750 = select(_e747, false, _e745);
                        let _e754 = select(select(80f, 32f, _e750), 24f, select(select(_e749, false, _e745), false, _e750));
                        let _e756 = (_e728 + (_e754 + 8f));
                        let _e759 = ((_e756 - 8f) - (_e754 * 0.5f));
                        phi_7868_ = _e759;
                        phi_7869_ = _e759;
                        phi_7870_ = _e756;
                        phi_7871_ = select(true, false, (_e721 < (_e756 - 4f)));
                    } else {
                        phi_7868_ = f32();
                        phi_7869_ = _e726;
                        phi_7870_ = _e728;
                        phi_7871_ = true;
                    }
                    let _e764 = phi_7868_;
                    let _e766 = phi_7869_;
                    let _e768 = phi_7870_;
                    let _e770 = phi_7871_;
                    if _e770 {
                        phi_7876_ = (_e724 + 1u);
                    } else {
                        phi_7876_ = u32();
                    }
                    let _e773 = phi_7876_;
                    phi_7815_ = _e773;
                    phi_7818_ = _e766;
                    phi_7820_ = _e768;
                    phi_7877_ = _e731;
                    phi_7878_ = _e764;
                    phi_7879_ = _e770;
                } else {
                    phi_7815_ = u32();
                    phi_7818_ = f32();
                    phi_7820_ = f32();
                    phi_7877_ = u32();
                    phi_7878_ = f32();
                    phi_7879_ = false;
                }
                let _e775 = phi_7815_;
                let _e777 = phi_7818_;
                let _e779 = phi_7820_;
                let _e781 = phi_7877_;
                let _e783 = phi_7878_;
                let _e785 = phi_7879_;
                continue;
                continuing {
                    phi_7814_ = _e775;
                    phi_7817_ = _e777;
                    phi_7819_ = _e779;
                    phi_15435_ = _e433;
                    phi_11751_ = select(true, false, _e729);
                    phi_11750_ = _e783;
                    phi_11748_ = _e781;
                    break if !(_e785);
                }
            }
            let _e789 = phi_15435_;
            let _e791 = phi_11751_;
            let _e793 = phi_11750_;
            let _e795 = phi_11748_;
            if _e789 {
                break;
            }
            if _e791 {
                let _e797 = local_48;
                phi_7885_ = render_track_PaletteColor(5u, _e797);
            } else {
                phi_7885_ = render_track_PaletteColor();
            }
            let _e800 = phi_7885_;
            if select(true, false, _e791) {
                phi_7891_ = render_track_PaletteColor(_e795, _e793);
            } else {
                phi_7891_ = _e800;
            }
            let _e804 = phi_7891_;
            let _e807 = (_e721 - _e804.weight);
            let _e808 = ((_e539 * _e293) - _e301);
            switch bitcast<i32>(_e804.rgb) {
                case 0: {
                    phi_7902_ = vec2<f32>();
                    phi_7903_ = true;
                    break;
                }
                case 1: {
                    phi_7902_ = vec2<f32>();
                    phi_7903_ = true;
                    break;
                }
                default: {
                    phi_7902_ = vec2<f32>(0f, 0f);
                    phi_7903_ = false;
                    break;
                }
            }
            let _e811 = phi_7902_;
            let _e813 = phi_7903_;
            if _e813 {
                if _e789 {
                    break;
                }
                let _e814 = (_e721 - 52f);
                let _e819 = pill_2.member[_e229].cpu.temperature;
                if (_e819 <= 62f) {
                    phi_8069_ = vec2<f32>(0f, 0f);
                } else {
                    let _e822 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e814, _e808), 13f, 13f);
                    phi_13947_ = 0i;
                    phi_13948_ = 0.5f;
                    phi_13949_ = 0f;
                    phi_13950_ = vec2<f32>(((_e814 + (_e551 * 1.8f)) * 0.035f), (((_e808 + -(_e551)) * 0.035f) + 6.1f));
                    loop {
                        let _e832 = phi_13947_;
                        let _e834 = phi_13948_;
                        let _e836 = phi_13949_;
                        let _e838 = phi_13950_;
                        local_49 = _e836;
                        let _e839 = (_e832 < 4i);
                        if _e839 {
                            let _e842 = cantus_render_shader_simplex_noise(_e838);
                            phi_13975_ = (_e832 + 1i);
                            phi_13976_ = (_e834 * 0.5f);
                            phi_13977_ = (_e836 + (_e842 * _e834));
                            phi_13978_ = vec2<f32>(((_e838.x * 1.6f) + (_e838.y * 1.2f)), ((_e838.y * 1.6f) - (_e838.x * 1.2f)));
                        } else {
                            phi_13975_ = i32();
                            phi_13976_ = f32();
                            phi_13977_ = f32();
                            phi_13978_ = vec2<f32>();
                        }
                        let _e855 = phi_13975_;
                        let _e857 = phi_13976_;
                        let _e859 = phi_13977_;
                        let _e861 = phi_13978_;
                        continue;
                        continuing {
                            phi_13947_ = _e855;
                            phi_13948_ = _e857;
                            phi_13949_ = _e859;
                            phi_13950_ = _e861;
                            break if !(_e839);
                        }
                    }
                    let _e864 = local_49;
                    let _e865 = (_e864 * 0.5f);
                    let _e868 = ((_e822 - -0.5f) * 0.5f);
                    let _e870 = select(_e868, 0f, (_e868 < 0f));
                    let _e872 = select(_e870, 1f, (_e870 > 1f));
                    let _e878 = ((_e822 - 14f) * -0.083333336f);
                    let _e880 = select(_e878, 0f, (_e878 < 0f));
                    let _e882 = select(_e880, 1f, (_e880 > 1f));
                    let _e887 = (((_e872 * _e872) * (3f - (2f * _e872))) * ((_e882 * _e882) * (3f - (2f * _e882))));
                    let _e892 = ((_e865 + 0.19999999f) * 3.125f);
                    let _e894 = select(_e892, 0f, (_e892 < 0f));
                    let _e896 = select(_e894, 1f, (_e894 > 1f));
                    let _e903 = ((_e819 - 62f) * 0.045454547f);
                    let _e905 = select(_e903, 0f, (_e903 < 0f));
                    let _e907 = select(_e905, 1f, (_e905 > 1f));
                    let _e911 = ((_e907 * _e907) * (3f - (2f * _e907)));
                    phi_8069_ = vec2<f32>(((_e887 * (0.18f + ((0.5f + _e865) * 0.34f))) * _e911), ((_e887 * ((_e896 * _e896) * (3f - (2f * _e896)))) * _e911));
                }
                let _e916 = phi_8069_;
                phi_8073_ = 0u;
                phi_8076_ = 12f;
                loop {
                    let _e920 = phi_8073_;
                    let _e922 = phi_8076_;
                    local_50 = _e922;
                    let _e923 = (_e920 < 1u);
                    if _e923 {
                        local_42 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                        if (_e920 < 6u) {
                        } else {
                            phi_15490_ = true;
                            break;
                        }
                        let _e926 = local_42[_e920];
                        let _e930 = pill_2.member[_e229].battery_level;
                        if (_e930 >= -1f) {
                            phi_8100_ = (_e930 <= 1f);
                        } else {
                            phi_8100_ = false;
                        }
                        let _e934 = phi_8100_;
                        if _e934 {
                            phi_8109_ = true;
                        } else {
                            phi_8109_ = select(true, false, (_e926 == 2u));
                        }
                        let _e938 = phi_8109_;
                        if _e938 {
                            switch bitcast<i32>(_e926) {
                                case 0: {
                                    phi_13993_ = true;
                                    phi_13994_ = false;
                                    phi_13995_ = false;
                                    break;
                                }
                                case 1: {
                                    phi_13993_ = true;
                                    phi_13994_ = false;
                                    phi_13995_ = false;
                                    break;
                                }
                                case 2: {
                                    phi_13993_ = false;
                                    phi_13994_ = true;
                                    phi_13995_ = false;
                                    break;
                                }
                                case 3: {
                                    phi_13993_ = false;
                                    phi_13994_ = true;
                                    phi_13995_ = false;
                                    break;
                                }
                                case 4: {
                                    phi_13993_ = false;
                                    phi_13994_ = false;
                                    phi_13995_ = true;
                                    break;
                                }
                                case 5: {
                                    phi_13993_ = false;
                                    phi_13994_ = false;
                                    phi_13995_ = true;
                                    break;
                                }
                                default: {
                                    phi_13993_ = bool();
                                    phi_13994_ = bool();
                                    phi_13995_ = bool();
                                    break;
                                }
                            }
                            let _e941 = phi_13993_;
                            let _e943 = phi_13994_;
                            let _e945 = phi_13995_;
                            let _e946 = select(_e943, false, _e941);
                            phi_8116_ = (_e922 + (select(select(80f, 32f, _e946), 24f, select(select(_e945, false, _e941), false, _e946)) + 8f));
                        } else {
                            phi_8116_ = _e922;
                        }
                        let _e954 = phi_8116_;
                        phi_8074_ = (_e920 + 1u);
                        phi_8077_ = _e954;
                    } else {
                        phi_8074_ = u32();
                        phi_8077_ = f32();
                    }
                    let _e957 = phi_8074_;
                    let _e959 = phi_8077_;
                    continue;
                    continuing {
                        phi_8073_ = _e957;
                        phi_8076_ = _e959;
                        phi_15490_ = _e789;
                        break if !(_e923);
                    }
                }
                let _e962 = phi_15490_;
                if _e962 {
                    break;
                }
                let _e964 = local_50;
                let _e966 = (_e721 - (_e964 + 40f));
                let _e971 = pill_2.member[_e229].gpu.temperature;
                if (_e971 <= 62f) {
                    phi_8234_ = vec2<f32>(0f, 0f);
                } else {
                    let _e974 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e966, _e808), 13f, 13f);
                    phi_14056_ = 0i;
                    phi_14057_ = 0.5f;
                    phi_14058_ = 0f;
                    phi_14059_ = vec2<f32>(((_e966 + (_e551 * 1.8f)) * 0.035f), (((_e808 + -(_e551)) * 0.035f) + 6.1f));
                    loop {
                        let _e984 = phi_14056_;
                        let _e986 = phi_14057_;
                        let _e988 = phi_14058_;
                        let _e990 = phi_14059_;
                        local_51 = _e988;
                        let _e991 = (_e984 < 4i);
                        if _e991 {
                            let _e994 = cantus_render_shader_simplex_noise(_e990);
                            phi_14084_ = (_e984 + 1i);
                            phi_14085_ = (_e986 * 0.5f);
                            phi_14086_ = (_e988 + (_e994 * _e986));
                            phi_14087_ = vec2<f32>(((_e990.x * 1.6f) + (_e990.y * 1.2f)), ((_e990.y * 1.6f) - (_e990.x * 1.2f)));
                        } else {
                            phi_14084_ = i32();
                            phi_14085_ = f32();
                            phi_14086_ = f32();
                            phi_14087_ = vec2<f32>();
                        }
                        let _e1007 = phi_14084_;
                        let _e1009 = phi_14085_;
                        let _e1011 = phi_14086_;
                        let _e1013 = phi_14087_;
                        continue;
                        continuing {
                            phi_14056_ = _e1007;
                            phi_14057_ = _e1009;
                            phi_14058_ = _e1011;
                            phi_14059_ = _e1013;
                            break if !(_e991);
                        }
                    }
                    let _e1016 = local_51;
                    let _e1017 = (_e1016 * 0.5f);
                    let _e1020 = ((_e974 - -0.5f) * 0.5f);
                    let _e1022 = select(_e1020, 0f, (_e1020 < 0f));
                    let _e1024 = select(_e1022, 1f, (_e1022 > 1f));
                    let _e1030 = ((_e974 - 14f) * -0.083333336f);
                    let _e1032 = select(_e1030, 0f, (_e1030 < 0f));
                    let _e1034 = select(_e1032, 1f, (_e1032 > 1f));
                    let _e1039 = (((_e1024 * _e1024) * (3f - (2f * _e1024))) * ((_e1034 * _e1034) * (3f - (2f * _e1034))));
                    let _e1044 = ((_e1017 + 0.19999999f) * 3.125f);
                    let _e1046 = select(_e1044, 0f, (_e1044 < 0f));
                    let _e1048 = select(_e1046, 1f, (_e1046 > 1f));
                    let _e1055 = ((_e971 - 62f) * 0.045454547f);
                    let _e1057 = select(_e1055, 0f, (_e1055 < 0f));
                    let _e1059 = select(_e1057, 1f, (_e1057 > 1f));
                    let _e1063 = ((_e1059 * _e1059) * (3f - (2f * _e1059)));
                    phi_8234_ = vec2<f32>(((_e1039 * (0.18f + ((0.5f + _e1017) * 0.34f))) * _e1063), ((_e1039 * ((_e1048 * _e1048) * (3f - (2f * _e1048)))) * _e1063));
                }
                let _e1068 = phi_8234_;
                phi_15523_ = _e962;
                phi_8248_ = vec2<f32>(select(_e1068.x, _e916.x, (_e916.x > _e1068.x)), select(_e1068.y, _e916.y, (_e916.y > _e1068.y)));
            } else {
                phi_15523_ = _e789;
                phi_8248_ = _e811;
            }
            let _e1077 = phi_15523_;
            let _e1079 = phi_8248_;
            let _e1084 = pill_2.member[_e229].cpu.temperature;
            let _e1089 = pill_2.member[_e229].gpu.temperature;
            if (_e1084 != _e1084) {
                phi_14102_ = true;
            } else {
                phi_14102_ = (_e1089 >= _e1084);
            }
            let _e1093 = phi_14102_;
            let _e1094 = select(_e1084, _e1089, _e1093);
            let _e1096 = ((_e1094 - 60f) * 0.083333336f);
            let _e1098 = select(_e1096, 0f, (_e1096 < 0f));
            let _e1100 = select(_e1098, 1f, (_e1098 > 1f));
            let _e1104 = ((_e1100 * _e1100) * (3f - (2f * _e1100)));
            let _e1105 = (1f - _e1104);
            let _e1114 = ((_e1094 - 72f) * 0.0625f);
            let _e1116 = select(_e1114, 0f, (_e1114 < 0f));
            let _e1118 = select(_e1116, 1f, (_e1116 > 1f));
            let _e1122 = ((_e1118 * _e1118) * (3f - (2f * _e1118)));
            let _e1123 = (1f - _e1122);
            let _e1133 = (_e1079.y * 0.12f);
            let _e1134 = (0.24f + _e1133);
            let _e1135 = (0.76f - _e1133);
            let _e1147 = (1f - (_e1079.x * 0.46f));
            let _e1157 = (_e1079.y * 0.64f);
            let _e1158 = (1f - _e1157);
            let _e1165 = (((((((((((((((((((0.008f * _e590) + (0.03f * _e567)) * _e588) + (((0.09f * _e590) + (0.34f * _e567)) * _e577)) * _e626) + ((_e602 + (0.3f * _e567)) * _e589)) * _e641) + (0.16f * _e640)) * _e652) + _e656) * _e661) + (_e547.fog * 0.3844f)) * _e685) + (_e683 * 0.2925f)) + _e717) * _e1147) + (_e1079.x * 0.0009200001f)) * _e1158) + (((0.07f * _e1135) + (((((0.22f * _e1105) + _e1104) * _e1123) + _e1122) * _e1134)) * _e1157));
            let _e1166 = (((((((((((((((((((0.015f * _e590) + (0.06f * _e567)) * _e588) + (((0.37f * _e590) + (0.7f * _e567)) * _e577)) * _e626) + (((0.25f * _e590) + (0.2f * _e567)) * _e589)) * _e641) + (0.2f * _e640)) * _e652) + _e656) * _e661) + (_e547.fog * 0.4216f)) * _e685) + (_e683 * 0.333f)) + _e717) * _e1147) + (_e1079.x * 0.00276f)) * _e1158) + (((0.12f * _e1135) + (((((0.62f * _e1105) + (0.38f * _e1104)) * _e1123) + (0.08f * _e1122)) * _e1134)) * _e1157));
            let _e1167 = (((((((((((((((((((0.04f * _e590) + (0.13f * _e567)) * _e588) + ((_e602 + (0.9f * _e567)) * _e577)) * _e626) + (((0.2f * _e590) + (0.4f * _e567)) * _e589)) * _e641) + (0.27f * _e640)) * _e652) + _e656) * _e661) + (_e547.fog * 0.44640002f)) * _e685) + (_e683 * 0.43199998f)) + _e717) * _e1147) + (_e1079.x * 0.00552f)) * _e1158) + (((0.18f * _e1135) + ((((_e1105 + (0.08f * _e1104)) * _e1123) + (0.035f * _e1122)) * _e1134)) * _e1157));
            switch bitcast<i32>(_e804.rgb) {
                case 0: {
                    let _e1893 = pill_2.member[_e229].history_scroll;
                    switch bitcast<i32>(_e804.rgb) {
                        case 0: {
                            phi_14189_ = true;
                            phi_14190_ = false;
                            phi_14191_ = false;
                            break;
                        }
                        case 1: {
                            phi_14189_ = true;
                            phi_14190_ = false;
                            phi_14191_ = false;
                            break;
                        }
                        case 2: {
                            phi_14189_ = false;
                            phi_14190_ = true;
                            phi_14191_ = false;
                            break;
                        }
                        case 3: {
                            phi_14189_ = false;
                            phi_14190_ = true;
                            phi_14191_ = false;
                            break;
                        }
                        case 4: {
                            phi_14189_ = false;
                            phi_14190_ = false;
                            phi_14191_ = true;
                            break;
                        }
                        case 5: {
                            phi_14189_ = false;
                            phi_14190_ = false;
                            phi_14191_ = true;
                            break;
                        }
                        default: {
                            phi_14189_ = bool();
                            phi_14190_ = bool();
                            phi_14191_ = bool();
                            break;
                        }
                    }
                    let _e1896 = phi_14189_;
                    let _e1898 = phi_14190_;
                    let _e1900 = phi_14191_;
                    let _e1901 = select(_e1898, false, _e1896);
                    let _e1907 = ((select(select(80f, 32f, _e1901), 24f, select(select(_e1900, false, _e1896), false, _e1901)) * 0.5f) - 4f);
                    let _e1908 = (_e301 - 8f);
                    let _e1909 = (_e1907 - _e1908);
                    let _e1911 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e807, _e808), _e1909, _e1908);
                    let _e1912 = abs(_e807);
                    let _e1913 = abs(_e808);
                    let _e1916 = (round((_e1912 * 0.11111111f)) * 9f);
                    if (_e1916 != _e1916) {
                        phi_14295_ = true;
                    } else {
                        phi_14295_ = (_e1907 <= _e1916);
                    }
                    let _e1920 = phi_14295_;
                    let _e1921 = select(_e1916, _e1907, _e1920);
                    let _e1922 = (_e1921 - _e1909);
                    if (_e1922 != _e1922) {
                        phi_14310_ = true;
                    } else {
                        phi_14310_ = (0f >= _e1922);
                    }
                    let _e1926 = phi_14310_;
                    let _e1927 = select(_e1922, 0f, _e1926);
                    let _e1928 = (_e1908 * _e1908);
                    let _e1931 = sqrt((_e1928 - (_e1927 * _e1927)));
                    let _e1932 = (_e1927 / _e1908);
                    let _e1933 = (_e1931 / _e1908);
                    let _e1938 = ((_e1912 - _e1921) - (_e1932 * 0.9f));
                    let _e1939 = ((_e1913 - _e1931) - (_e1933 * 0.9f));
                    let _e1948 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1938 * -(_e1933)) + (_e1939 * _e1932)), ((_e1938 * _e1932) + (_e1939 * _e1933))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e1950 = round((_e1913 * 0.125f));
                    if (_e1950 != _e1950) {
                        phi_14325_ = true;
                    } else {
                        phi_14325_ = (1f <= _e1950);
                    }
                    let _e1954 = phi_14325_;
                    let _e1956 = (select(_e1950, 1f, _e1954) * 8f);
                    let _e1959 = sqrt((_e1928 - (_e1956 * _e1956)));
                    let _e1961 = (_e1959 / _e1908);
                    let _e1962 = (_e1956 / _e1908);
                    let _e1967 = ((_e1912 - (_e1909 + _e1959)) - (_e1961 * 0.9f));
                    let _e1968 = ((_e1913 - _e1956) - (_e1962 * 0.9f));
                    let _e1977 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1967 * -(_e1962)) + (_e1968 * _e1961)), ((_e1967 * _e1961) + (_e1968 * _e1962))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e1948 != _e1948) {
                        phi_14340_ = true;
                    } else {
                        phi_14340_ = (_e1977 <= _e1948);
                    }
                    let _e1981 = phi_14340_;
                    let _e1982 = select(_e1948, _e1977, _e1981);
                    let _e1985 = (0.5f + ((_e1982 - _e1911) * 0.3125f));
                    let _e1987 = select(_e1985, 0f, (_e1985 < 0f));
                    let _e1989 = select(_e1987, 1f, (_e1987 > 1f));
                    let _e1998 = ((_e1911 - 0.55f) * -0.9090909f);
                    let _e2000 = select(_e1998, 0f, (_e1998 < 0f));
                    let _e2002 = select(_e2000, 1f, (_e2000 > 1f));
                    let _e2006 = ((_e2002 * _e2002) * (3f - (2f * _e2002)));
                    let _e2007 = (_e1907 * 0.051282052f);
                    let _e2008 = (_e807 + _e1907);
                    let _e2010 = ((_e2008 / _e2007) + _e1893);
                    let _e2012 = select(_e2010, 0f, (_e2010 < 0f));
                    let _e2014 = select(_e2012, 39f, (_e2012 > 39f));
                    let _e2015 = floor(_e2014);
                    let _e2020 = select(select(u32(_e2015), 0u, (_e2015 < 0f)), 4294967295u, (_e2015 > 4294967000f));
                    let _e2021 = (_e301 - 10f);
                    let _e2025 = (((f32(_e2020) - _e1893) * _e2007) - _e1907);
                    let _e2027 = select(_e2020, 39u, (39u < _e2020));
                    let _e2028 = (_e2027 < 40u);
                    if _e2028 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e2035 = pill_2.member[_e229].cpu.usage.samples[_e2027];
                    let _e2038 = (_e2021 * (1f - (_e2035 * 2f)));
                    let _e2039 = (_e2020 + 1u);
                    let _e2045 = select(_e2039, 39u, (39u < _e2039));
                    let _e2046 = (_e2045 < 40u);
                    if _e2046 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e2053 = pill_2.member[_e229].cpu.usage.samples[_e2045];
                    let _e2057 = ((((f32(_e2039) - _e1893) * _e2007) - _e1907) - _e2025);
                    let _e2058 = ((_e2021 * (1f - (_e2053 * 2f))) - _e2038);
                    let _e2059 = (_e807 - _e2025);
                    let _e2060 = (_e808 - _e2038);
                    let _e2061 = (_e2059 * _e2057);
                    let _e2064 = (_e2057 * _e2057);
                    let _e2066 = (_e2064 + (_e2058 * _e2058));
                    if (_e2066 != _e2066) {
                        phi_14355_ = true;
                    } else {
                        phi_14355_ = (0.001f >= _e2066);
                    }
                    let _e2070 = phi_14355_;
                    let _e2072 = ((_e2061 + (_e2060 * _e2058)) / select(_e2066, 0.001f, _e2070));
                    let _e2074 = select(_e2072, 0f, (_e2072 < 0f));
                    let _e2076 = select(_e2074, 1f, (_e2074 > 1f));
                    let _e2079 = (_e2059 - (_e2057 * _e2076));
                    let _e2080 = (_e2060 - (_e2058 * _e2076));
                    let _e2087 = ((abs(sqrt(((_e2079 * _e2079) + (_e2080 * _e2080)))) - 1.4000001f) * -0.9090908f);
                    let _e2089 = select(_e2087, 0f, (_e2087 < 0f));
                    let _e2091 = select(_e2089, 1f, (_e2089 > 1f));
                    let _e2097 = (_e2014 - trunc(_e2014));
                    let _e2099 = select(_e2097, 0f, (_e2097 < 0f));
                    let _e2101 = select(_e2099, 1f, (_e2099 > 1f));
                    let _e2105 = ((_e2101 * _e2101) * (3f - (2f * _e2101)));
                    let _e2112 = ((((_e2038 + (_e2058 * _e2105)) - _e808) - 0.55f) * -0.9090909f);
                    let _e2114 = select(_e2112, 0f, (_e2112 < 0f));
                    let _e2116 = select(_e2114, 1f, (_e2114 > 1f));
                    let _e2122 = ((((_e2116 * _e2116) * (3f - (2f * _e2116))) * 0.156f) + ((_e2091 * _e2091) * (3f - (2f * _e2091))));
                    if _e2028 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e2131 = pill_2.member[_e229].cpu.memory.samples[_e2027];
                    let _e2134 = (_e2021 * (1f - (_e2131 * 2f)));
                    if _e2046 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e2141 = pill_2.member[_e229].cpu.memory.samples[_e2045];
                    let _e2145 = ((_e2021 * (1f - (_e2141 * 2f))) - _e2134);
                    let _e2146 = (_e808 - _e2134);
                    let _e2150 = (_e2064 + (_e2145 * _e2145));
                    if (_e2150 != _e2150) {
                        phi_14370_ = true;
                    } else {
                        phi_14370_ = (0.001f >= _e2150);
                    }
                    let _e2154 = phi_14370_;
                    let _e2156 = ((_e2061 + (_e2146 * _e2145)) / select(_e2150, 0.001f, _e2154));
                    let _e2158 = select(_e2156, 0f, (_e2156 < 0f));
                    let _e2160 = select(_e2158, 1f, (_e2158 > 1f));
                    let _e2163 = (_e2059 - (_e2057 * _e2160));
                    let _e2164 = (_e2146 - (_e2145 * _e2160));
                    let _e2171 = ((abs(sqrt(((_e2163 * _e2163) + (_e2164 * _e2164)))) - 1.4000001f) * -0.9090908f);
                    let _e2173 = select(_e2171, 0f, (_e2171 < 0f));
                    let _e2175 = select(_e2173, 1f, (_e2173 > 1f));
                    let _e2186 = ((((_e2134 + (_e2145 * _e2105)) - _e808) - 0.55f) * -0.9090909f);
                    let _e2188 = select(_e2186, 0f, (_e2186 < 0f));
                    let _e2190 = select(_e2188, 1f, (_e2188 > 1f));
                    let _e2196 = ((((_e2190 * _e2190) * (3f - (2f * _e2190))) * 0.084f) + ((_e2175 * _e2175) * (3f - (2f * _e2175))));
                    let _e2204 = (_e2008 * 0.14285715f);
                    let _e2205 = ((_e808 + _e1908) * 0.16393442f);
                    let _e2215 = ((abs(((_e2204 - trunc(_e2204)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2217 = select(_e2215, 0f, (_e2215 < 0f));
                    let _e2219 = select(_e2217, 1f, (_e2217 > 1f));
                    let _e2223 = ((_e2219 * _e2219) * (3f - (2f * _e2219)));
                    let _e2225 = ((abs(((_e2205 - trunc(_e2205)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2227 = select(_e2225, 0f, (_e2225 < 0f));
                    let _e2229 = select(_e2227, 1f, (_e2227 > 1f));
                    let _e2233 = ((_e2229 * _e2229) * (3f - (2f * _e2229)));
                    if (_e2223 != _e2223) {
                        phi_14385_ = true;
                    } else {
                        phi_14385_ = (_e2233 >= _e2223);
                    }
                    let _e2237 = phi_14385_;
                    let _e2245 = pill_2.member[_e229].cpu.usage.samples[39u];
                    let _e2246 = (_e2245 * 0.24f);
                    let _e2247 = (0.18f + _e2246);
                    let _e2248 = (0.82f - _e2246);
                    let _e2257 = (_e1084 - 60f);
                    let _e2258 = (_e2257 * 0.083333336f);
                    let _e2260 = select(_e2258, 0f, (_e2258 < 0f));
                    let _e2262 = select(_e2260, 1f, (_e2260 > 1f));
                    let _e2266 = ((_e2262 * _e2262) * (3f - (2f * _e2262)));
                    let _e2267 = (1f - _e2266);
                    let _e2276 = ((_e1084 - 72f) * 0.0625f);
                    let _e2278 = select(_e2276, 0f, (_e2276 < 0f));
                    let _e2280 = select(_e2278, 1f, (_e2278 > 1f));
                    let _e2284 = ((_e2280 * _e2280) * (3f - (2f * _e2280)));
                    let _e2285 = (1f - _e2284);
                    let _e2294 = (_e2257 * 0.03846154f);
                    let _e2296 = select(_e2294, 0f, (_e2294 < 0f));
                    let _e2298 = select(_e2296, 1f, (_e2296 > 1f));
                    let _e2303 = (((_e2298 * _e2298) * (3f - (2f * _e2298))) * 0.9f);
                    let _e2304 = (1f - _e2303);
                    let _e2311 = ((((0.025f * _e2248) + (0.32f * _e2247)) * _e2304) + (((((0.22f * _e2267) + _e2266) * _e2285) + _e2284) * _e2303));
                    let _e2312 = ((((0.09f * _e2248) + (0.68f * _e2247)) * _e2304) + (((((0.62f * _e2267) + (0.38f * _e2266)) * _e2285) + (0.08f * _e2284)) * _e2303));
                    let _e2313 = ((((0.15f * _e2248) + _e2247) * _e2304) + ((((_e2267 + (0.08f * _e2266)) * _e2285) + (0.035f * _e2284)) * _e2303));
                    let _e2315 = ((((_e1982 + ((_e1911 - _e1982) * _e1989)) - ((1.6f * _e1989) * (1f - _e1989))) - 0.55f) * -0.9090909f);
                    let _e2317 = select(_e2315, 0f, (_e2315 < 0f));
                    let _e2319 = select(_e2317, 1f, (_e2317 > 1f));
                    let _e2323 = ((_e2319 * _e2319) * (3f - (2f * _e2319)));
                    let _e2325 = (1f - (_e2323 * 0.82f));
                    let _e2337 = ((abs(_e1911) - 2.1f) * -0.909091f);
                    let _e2339 = select(_e2337, 0f, (_e2337 < 0f));
                    let _e2341 = select(_e2339, 1f, (_e2339 > 1f));
                    let _e2346 = (((_e2341 * _e2341) * (3f - (2f * _e2341))) * 0.92f);
                    let _e2347 = (1f - _e2346);
                    let _e2358 = ((_e1982 - 0.55f) * -0.9090909f);
                    let _e2360 = select(_e2358, 0f, (_e2358 < 0f));
                    let _e2362 = select(_e2360, 1f, (_e2360 > 1f));
                    let _e2367 = (((_e2362 * _e2362) * (3f - (2f * _e2362))) * 0.78f);
                    let _e2368 = (1f - _e2367);
                    let _e2379 = ((_e2006 * select(_e2223, _e2233, _e2237)) * 0.045f);
                    phi_15516_ = _e1077;
                    phi_10345_ = vec3<f32>(((((((((_e1165 * _e2325) + (_e2323 * 0.00328f)) * _e2347) + (_e2311 * _e2346)) * _e2368) + (_e2311 * _e2367)) + _e2379) + (((0.32f * _e2006) * _e2122) + ((0.78f * _e2006) * _e2196))), ((((((((_e1166 * _e2325) + (_e2323 * 0.00984f)) * _e2347) + (_e2312 * _e2346)) * _e2368) + (_e2312 * _e2367)) + _e2379) + (((0.68f * _e2006) * _e2122) + ((0.3f * _e2006) * _e2196))), ((((((((_e1167 * _e2325) + (_e2323 * 0.02132f)) * _e2347) + (_e2313 * _e2346)) * _e2368) + (_e2313 * _e2367)) + _e2379) + (_e2006 * (_e2122 + _e2196))));
                    phi_10346_ = false;
                    break;
                }
                case 1: {
                    let _e1512 = pill_2.member[_e229].history_scroll;
                    switch bitcast<i32>(_e804.rgb) {
                        case 0: {
                            phi_14117_ = true;
                            phi_14118_ = false;
                            phi_14119_ = false;
                            break;
                        }
                        case 1: {
                            phi_14117_ = true;
                            phi_14118_ = false;
                            phi_14119_ = false;
                            break;
                        }
                        case 2: {
                            phi_14117_ = false;
                            phi_14118_ = true;
                            phi_14119_ = false;
                            break;
                        }
                        case 3: {
                            phi_14117_ = false;
                            phi_14118_ = true;
                            phi_14119_ = false;
                            break;
                        }
                        case 4: {
                            phi_14117_ = false;
                            phi_14118_ = false;
                            phi_14119_ = true;
                            break;
                        }
                        case 5: {
                            phi_14117_ = false;
                            phi_14118_ = false;
                            phi_14119_ = true;
                            break;
                        }
                        default: {
                            phi_14117_ = bool();
                            phi_14118_ = bool();
                            phi_14119_ = bool();
                            break;
                        }
                    }
                    let _e1515 = phi_14117_;
                    let _e1517 = phi_14118_;
                    let _e1519 = phi_14119_;
                    let _e1520 = select(_e1517, false, _e1515);
                    let _e1526 = ((select(select(80f, 32f, _e1520), 24f, select(select(_e1519, false, _e1515), false, _e1520)) * 0.5f) - 4f);
                    let _e1527 = (_e301 - 8f);
                    let _e1530 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e807, _e808), (_e1526 - _e1527), _e1527);
                    let _e1532 = ((_e1530 - 0.55f) * -0.9090909f);
                    let _e1534 = select(_e1532, 0f, (_e1532 < 0f));
                    let _e1536 = select(_e1534, 1f, (_e1534 > 1f));
                    let _e1540 = ((_e1536 * _e1536) * (3f - (2f * _e1536)));
                    let _e1541 = (_e1526 * 0.051282052f);
                    let _e1542 = (_e807 + _e1526);
                    let _e1544 = ((_e1542 / _e1541) + _e1512);
                    let _e1546 = select(_e1544, 0f, (_e1544 < 0f));
                    let _e1548 = select(_e1546, 39f, (_e1546 > 39f));
                    let _e1549 = floor(_e1548);
                    let _e1554 = select(select(u32(_e1549), 0u, (_e1549 < 0f)), 4294967295u, (_e1549 > 4294967000f));
                    let _e1555 = (_e301 - 10f);
                    let _e1559 = (((f32(_e1554) - _e1512) * _e1541) - _e1526);
                    let _e1561 = select(_e1554, 39u, (39u < _e1554));
                    let _e1562 = (_e1561 < 40u);
                    if _e1562 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e1569 = pill_2.member[_e229].gpu.usage.samples[_e1561];
                    let _e1572 = (_e1555 * (1f - (_e1569 * 2f)));
                    let _e1573 = (_e1554 + 1u);
                    let _e1579 = select(_e1573, 39u, (39u < _e1573));
                    let _e1580 = (_e1579 < 40u);
                    if _e1580 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e1587 = pill_2.member[_e229].gpu.usage.samples[_e1579];
                    let _e1591 = ((((f32(_e1573) - _e1512) * _e1541) - _e1526) - _e1559);
                    let _e1592 = ((_e1555 * (1f - (_e1587 * 2f))) - _e1572);
                    let _e1593 = (_e807 - _e1559);
                    let _e1594 = (_e808 - _e1572);
                    let _e1595 = (_e1593 * _e1591);
                    let _e1598 = (_e1591 * _e1591);
                    let _e1600 = (_e1598 + (_e1592 * _e1592));
                    if (_e1600 != _e1600) {
                        phi_14144_ = true;
                    } else {
                        phi_14144_ = (0.001f >= _e1600);
                    }
                    let _e1604 = phi_14144_;
                    let _e1606 = ((_e1595 + (_e1594 * _e1592)) / select(_e1600, 0.001f, _e1604));
                    let _e1608 = select(_e1606, 0f, (_e1606 < 0f));
                    let _e1610 = select(_e1608, 1f, (_e1608 > 1f));
                    let _e1613 = (_e1593 - (_e1591 * _e1610));
                    let _e1614 = (_e1594 - (_e1592 * _e1610));
                    let _e1621 = ((abs(sqrt(((_e1613 * _e1613) + (_e1614 * _e1614)))) - 1.4000001f) * -0.9090908f);
                    let _e1623 = select(_e1621, 0f, (_e1621 < 0f));
                    let _e1625 = select(_e1623, 1f, (_e1623 > 1f));
                    let _e1631 = (_e1548 - trunc(_e1548));
                    let _e1633 = select(_e1631, 0f, (_e1631 < 0f));
                    let _e1635 = select(_e1633, 1f, (_e1633 > 1f));
                    let _e1639 = ((_e1635 * _e1635) * (3f - (2f * _e1635)));
                    let _e1646 = ((((_e1572 + (_e1592 * _e1639)) - _e808) - 0.55f) * -0.9090909f);
                    let _e1648 = select(_e1646, 0f, (_e1646 < 0f));
                    let _e1650 = select(_e1648, 1f, (_e1648 > 1f));
                    let _e1656 = ((((_e1650 * _e1650) * (3f - (2f * _e1650))) * 0.156f) + ((_e1625 * _e1625) * (3f - (2f * _e1625))));
                    if _e1562 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e1665 = pill_2.member[_e229].gpu.memory.samples[_e1561];
                    let _e1668 = (_e1555 * (1f - (_e1665 * 2f)));
                    if _e1580 {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e1675 = pill_2.member[_e229].gpu.memory.samples[_e1579];
                    let _e1679 = ((_e1555 * (1f - (_e1675 * 2f))) - _e1668);
                    let _e1680 = (_e808 - _e1668);
                    let _e1684 = (_e1598 + (_e1679 * _e1679));
                    if (_e1684 != _e1684) {
                        phi_14159_ = true;
                    } else {
                        phi_14159_ = (0.001f >= _e1684);
                    }
                    let _e1688 = phi_14159_;
                    let _e1690 = ((_e1595 + (_e1680 * _e1679)) / select(_e1684, 0.001f, _e1688));
                    let _e1692 = select(_e1690, 0f, (_e1690 < 0f));
                    let _e1694 = select(_e1692, 1f, (_e1692 > 1f));
                    let _e1697 = (_e1593 - (_e1591 * _e1694));
                    let _e1698 = (_e1680 - (_e1679 * _e1694));
                    let _e1705 = ((abs(sqrt(((_e1697 * _e1697) + (_e1698 * _e1698)))) - 1.4000001f) * -0.9090908f);
                    let _e1707 = select(_e1705, 0f, (_e1705 < 0f));
                    let _e1709 = select(_e1707, 1f, (_e1707 > 1f));
                    let _e1720 = ((((_e1668 + (_e1679 * _e1639)) - _e808) - 0.55f) * -0.9090909f);
                    let _e1722 = select(_e1720, 0f, (_e1720 < 0f));
                    let _e1724 = select(_e1722, 1f, (_e1722 > 1f));
                    let _e1730 = ((((_e1724 * _e1724) * (3f - (2f * _e1724))) * 0.084f) + ((_e1709 * _e1709) * (3f - (2f * _e1709))));
                    let _e1738 = (_e1542 * 0.14285715f);
                    let _e1739 = ((_e808 + _e1527) * 0.16393442f);
                    let _e1749 = ((abs(((_e1738 - trunc(_e1738)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e1751 = select(_e1749, 0f, (_e1749 < 0f));
                    let _e1753 = select(_e1751, 1f, (_e1751 > 1f));
                    let _e1757 = ((_e1753 * _e1753) * (3f - (2f * _e1753)));
                    let _e1759 = ((abs(((_e1739 - trunc(_e1739)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e1761 = select(_e1759, 0f, (_e1759 < 0f));
                    let _e1763 = select(_e1761, 1f, (_e1761 > 1f));
                    let _e1767 = ((_e1763 * _e1763) * (3f - (2f * _e1763)));
                    if (_e1757 != _e1757) {
                        phi_14174_ = true;
                    } else {
                        phi_14174_ = (_e1767 >= _e1757);
                    }
                    let _e1771 = phi_14174_;
                    let _e1779 = pill_2.member[_e229].gpu.usage.samples[39u];
                    let _e1780 = (_e1779 * 0.24f);
                    let _e1781 = (0.18f + _e1780);
                    let _e1782 = (0.82f - _e1780);
                    let _e1791 = (_e1089 - 60f);
                    let _e1792 = (_e1791 * 0.083333336f);
                    let _e1794 = select(_e1792, 0f, (_e1792 < 0f));
                    let _e1796 = select(_e1794, 1f, (_e1794 > 1f));
                    let _e1800 = ((_e1796 * _e1796) * (3f - (2f * _e1796)));
                    let _e1801 = (1f - _e1800);
                    let _e1810 = ((_e1089 - 72f) * 0.0625f);
                    let _e1812 = select(_e1810, 0f, (_e1810 < 0f));
                    let _e1814 = select(_e1812, 1f, (_e1812 > 1f));
                    let _e1818 = ((_e1814 * _e1814) * (3f - (2f * _e1814)));
                    let _e1819 = (1f - _e1818);
                    let _e1828 = (_e1791 * 0.03846154f);
                    let _e1830 = select(_e1828, 0f, (_e1828 < 0f));
                    let _e1832 = select(_e1830, 1f, (_e1830 > 1f));
                    let _e1837 = (((_e1832 * _e1832) * (3f - (2f * _e1832))) * 0.9f);
                    let _e1838 = (1f - _e1837);
                    let _e1849 = (1f - (_e1540 * 0.82f));
                    let _e1861 = ((abs(_e1530) - 2.1f) * -0.909091f);
                    let _e1863 = select(_e1861, 0f, (_e1861 < 0f));
                    let _e1865 = select(_e1863, 1f, (_e1863 > 1f));
                    let _e1870 = (((_e1865 * _e1865) * (3f - (2f * _e1865))) * 0.92f);
                    let _e1871 = (1f - _e1870);
                    let _e1882 = ((_e1540 * select(_e1757, _e1767, _e1771)) * 0.045f);
                    phi_15516_ = _e1077;
                    phi_10345_ = vec3<f32>(((((((_e1165 * _e1849) + (_e1540 * 0.00328f)) * _e1871) + (((((0.025f * _e1782) + (0.32f * _e1781)) * _e1838) + (((((0.22f * _e1801) + _e1800) * _e1819) + _e1818) * _e1837)) * _e1870)) + _e1882) + (((0.32f * _e1540) * _e1656) + ((0.78f * _e1540) * _e1730))), ((((((_e1166 * _e1849) + (_e1540 * 0.00984f)) * _e1871) + (((((0.09f * _e1782) + (0.68f * _e1781)) * _e1838) + (((((0.62f * _e1801) + (0.38f * _e1800)) * _e1819) + (0.08f * _e1818)) * _e1837)) * _e1870)) + _e1882) + (((0.68f * _e1540) * _e1656) + ((0.3f * _e1540) * _e1730))), ((((((_e1167 * _e1849) + (_e1540 * 0.02132f)) * _e1871) + (((((0.15f * _e1782) + _e1781) * _e1838) + ((((_e1801 + (0.08f * _e1800)) * _e1819) + (0.035f * _e1818)) * _e1837)) * _e1870)) + _e1882) + (_e1540 * (_e1656 + _e1730))));
                    phi_10346_ = false;
                    break;
                }
                case 2: {
                    let _e1294 = (_e807 * 1.25f);
                    let _e1295 = (_e808 * 1.25f);
                    let _e1299 = pill_2.member[_e229].battery_level;
                    let _e1301 = select(0f, 1f, (_e1299 < 0f));
                    let _e1302 = abs(_e1299);
                    let _e1303 = (_e1295 - 1f);
                    let _e1304 = vec2<f32>(_e1294, _e1303);
                    let _e1305 = cantus_render_shader_sd_rounded_box(_e1304, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e1308 = ((abs(_e1305) - 2.425f) * -0.909091f);
                    let _e1310 = select(_e1308, 0f, (_e1308 < 0f));
                    let _e1312 = select(_e1310, 1f, (_e1310 > 1f));
                    let _e1319 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1294, (_e1295 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e1321 = ((_e1319 - 0.55f) * -0.9090909f);
                    let _e1323 = select(_e1321, 0f, (_e1321 < 0f));
                    let _e1325 = select(_e1323, 1f, (_e1323 > 1f));
                    let _e1330 = cantus_render_shader_sd_rounded_box(_e1304, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e1332 = ((_e1330 - 0.55f) * -0.9090909f);
                    let _e1334 = select(_e1332, 0f, (_e1332 < 0f));
                    let _e1336 = select(_e1334, 1f, (_e1334 > 1f));
                    let _e1342 = select(_e1302, 0f, (_e1302 < 0f));
                    let _e1360 = ((12f - (select(_e1342, 1f, (_e1342 > 1f)) * 24f)) + ((sin(((_e807 * 0.775f) + (_e551 * (1.4f + (_e1301 * 1.2f))))) * 1.15f) + (sin(((_e807 * 0.3375f) - (_e551 * 0.8f))) * 0.45f)));
                    let _e1361 = (_e1360 - 0.7f);
                    let _e1365 = ((_e1303 - _e1361) / ((_e1360 + 0.7f) - _e1361));
                    let _e1367 = select(_e1365, 0f, (_e1365 < 0f));
                    let _e1369 = select(_e1367, 1f, (_e1367 > 1f));
                    let _e1374 = (((_e1336 * _e1336) * (3f - (2f * _e1336))) * ((_e1369 * _e1369) * (3f - (2f * _e1369))));
                    let _e1376 = ((_e1302 - 0.08f) * 5f);
                    let _e1378 = select(_e1376, 0f, (_e1376 < 0f));
                    let _e1380 = select(_e1378, 1f, (_e1378 > 1f));
                    let _e1384 = ((_e1380 * _e1380) * (3f - (2f * _e1380)));
                    let _e1385 = (1f - _e1384);
                    let _e1393 = ((_e1302 - 0.18f) * 1.8518518f);
                    let _e1395 = select(_e1393, 0f, (_e1393 < 0f));
                    let _e1397 = select(_e1395, 1f, (_e1395 > 1f));
                    let _e1401 = ((_e1397 * _e1397) * (3f - (2f * _e1397)));
                    let _e1402 = (1f - _e1401);
                    let _e1408 = (_e1402 + (0.22f * _e1401));
                    let _e1409 = ((((0.18f * _e1385) + (0.72f * _e1384)) * _e1402) + (0.95f * _e1401));
                    let _e1410 = ((((0.1f * _e1385) + (0.12f * _e1384)) * _e1402) + (0.55f * _e1401));
                    let _e1413 = floor((_e807 * 0.4166667f));
                    let _e1414 = floor((_e808 * 0.36764705f));
                    let _e1416 = cantus_render_shader_hash(vec2<f32>(_e1413, _e1414));
                    let _e1430 = ((_e551 * (0.5f + _e1416.y)) + (_e1416.x * 11f));
                    let _e1432 = (_e1430 - trunc(_e1430));
                    let _e1433 = (_e1294 - (((_e1413 + 0.2f) + (_e1416.x * 0.6f)) * 3f));
                    let _e1436 = ((_e1295 - (((_e1414 + 0.2f) + (_e1416.y * 0.6f)) * 3.4f)) + (_e1432 * 5f));
                    let _e1444 = (_e1432 * 4f);
                    let _e1446 = select(_e1444, 0f, (_e1444 < 0f));
                    let _e1448 = select(_e1446, 1f, (_e1446 > 1f));
                    let _e1454 = ((_e1432 - 1f) * -3.3333333f);
                    let _e1456 = select(_e1454, 0f, (_e1454 < 0f));
                    let _e1458 = select(_e1456, 1f, (_e1456 > 1f));
                    let _e1466 = ((abs((sqrt(((_e1433 * _e1433) + (_e1436 * _e1436))) - (0.4f + (_e1416.y * 0.5f)))) - 1f) * -0.9090909f);
                    let _e1468 = select(_e1466, 0f, (_e1466 < 0f));
                    let _e1470 = select(_e1468, 1f, (_e1468 > 1f));
                    let _e1477 = (((((_e1470 * _e1470) * (3f - (2f * _e1470))) * (((_e1448 * _e1448) * (3f - (2f * _e1448))) * ((_e1458 * _e1458) * (3f - (2f * _e1458))))) * _e1374) * _e1301);
                    let _e1480 = ((((_e1312 * _e1312) * (3f - (2f * _e1312))) * 0.43f) + (((_e1325 * _e1325) * (3f - (2f * _e1325))) * 0.38f));
                    phi_15516_ = _e1077;
                    phi_10345_ = vec3<f32>((_e1165 + ((_e1480 + ((_e1408 * _e1374) * 0.78f)) + ((((_e1408 * 0.27999997f) + 0.72f) * _e1477) * 0.9f))), (_e1166 + ((_e1480 + ((_e1409 * _e1374) * 0.78f)) + ((((_e1409 * 0.27999997f) + 0.72f) * _e1477) * 0.9f))), (_e1167 + ((_e1480 + ((_e1410 * _e1374) * 0.78f)) + ((((_e1410 * 0.27999997f) + 0.72f) * _e1477) * 0.9f))));
                    phi_10346_ = false;
                    break;
                }
                case 3: {
                    let _e1172 = pill_2.member[_e229].volume;
                    let _e1174 = select(0f, 1f, (_e1172 < 0f));
                    let _e1175 = abs(_e1172);
                    let _e1178 = round(((_e807 + 12f) * 0.25f));
                    let _e1180 = select(_e1178, 0f, (_e1178 < 0f));
                    let _e1182 = select(_e1180, 6f, (_e1180 > 6f));
                    let _e1187 = select(select(u32(_e1182), 0u, (_e1182 < 0f)), 4294967295u, (_e1182 > 4294967000f));
                    if (_e1187 < 7u) {
                    } else {
                        phi_15516_ = true;
                        phi_10345_ = vec3<f32>();
                        phi_10346_ = bool();
                        break;
                    }
                    let _e1193 = pill_2.member[_e229].audio_spectrum[_e1187];
                    let _e1194 = (1f - _e1174);
                    let _e1195 = (_e1193 * _e1194);
                    let _e1204 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e807 - (-12f + (_e1182 * 4f))), (_e808 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1195))), 1.25f);
                    let _e1207 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e807, (_e808 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1209 = ((_e1207 - 0.55f) * -0.9090909f);
                    let _e1211 = select(_e1209, 0f, (_e1209 < 0f));
                    let _e1213 = select(_e1211, 1f, (_e1211 > 1f));
                    let _e1217 = ((_e1213 * _e1213) * (3f - (2f * _e1213)));
                    let _e1219 = select(_e1175, 0f, (_e1175 < 0f));
                    let _e1222 = (select(_e1219, 1f, (_e1219 > 1f)) * 28f);
                    let _e1223 = (_e1222 + -13.2f);
                    let _e1227 = ((_e807 - _e1223) / ((_e1222 + -14.8f) - _e1223));
                    let _e1229 = select(_e1227, 0f, (_e1227 < 0f));
                    let _e1231 = select(_e1229, 1f, (_e1229 > 1f));
                    let _e1236 = (_e1217 * ((_e1231 * _e1231) * (3f - (2f * _e1231))));
                    let _e1238 = (1f - (_e1175 * 0.65f));
                    let _e1243 = ((0.08f * _e1238) + (_e1175 * 0.42249995f));
                    let _e1244 = ((0.88f * _e1238) + (_e1175 * 0.221f));
                    let _e1246 = ((_e1204 - 0.7f) * -0.71428573f);
                    let _e1248 = select(_e1246, 0f, (_e1246 < 0f));
                    let _e1250 = select(_e1248, 1f, (_e1248 > 1f));
                    let _e1259 = ((_e1204 - 3.2f) * -0.3125f);
                    let _e1261 = select(_e1259, 0f, (_e1259 < 0f));
                    let _e1263 = select(_e1261, 1f, (_e1261 > 1f));
                    let _e1270 = ((((_e1250 * _e1250) * (3f - (2f * _e1250))) * (0.58f + (_e1195 * 0.35f))) + ((((_e1263 * _e1263) * (3f - (2f * _e1263))) * _e1195) * 0.12f));
                    let _e1283 = (_e1236 + ((_e1217 * (1f - _e1236)) * 0.22f));
                    phi_15516_ = _e1077;
                    phi_10345_ = vec3<f32>((_e1165 + ((_e1243 * _e1270) + (((_e1243 * _e1194) + _e1174) * _e1283))), (_e1166 + ((_e1244 * _e1270) + (((_e1244 * _e1194) + (0.24f * _e1174)) * _e1283))), (_e1167 + (_e1270 + ((_e1194 + (0.3f * _e1174)) * _e1283))));
                    phi_10346_ = false;
                    break;
                }
                case 4: {
                    phi_15516_ = _e1077;
                    phi_10345_ = vec3<f32>();
                    phi_10346_ = true;
                    break;
                }
                case 5: {
                    phi_15516_ = _e1077;
                    phi_10345_ = vec3<f32>();
                    phi_10346_ = true;
                    break;
                }
                default: {
                    phi_15516_ = _e1077;
                    phi_10345_ = vec3<f32>();
                    phi_10346_ = bool();
                    break;
                }
            }
            let _e2388 = phi_15516_;
            let _e2390 = phi_10345_;
            let _e2392 = phi_10346_;
            if _e2388 {
                break;
            }
            if _e2392 {
                let _e2394 = select(1f, 0f, (_e804.rgb == 5u));
                let _e2398 = pill_2.member[_e229].power_hover;
                let _e2404 = ((abs(((f32(_e2398) - _e2394) - 1f)) - 0.4f) * -2.857143f);
                let _e2406 = select(_e2404, 0f, (_e2404 < 0f));
                let _e2408 = select(_e2406, 1f, (_e2406 > 1f));
                let _e2412 = ((_e2408 * _e2408) * (3f - (2f * _e2408)));
                let _e2414 = (1f + (_e2412 * 0.07f));
                let _e2415 = (_e807 / _e2414);
                let _e2416 = (_e808 / _e2414);
                let _e2420 = pill_2.member[_e229].power_state;
                let _e2426 = ((abs(((floor(_e2420) - _e2394) - 1f)) - 0.4f) * -2.857143f);
                let _e2428 = select(_e2426, 0f, (_e2426 < 0f));
                let _e2430 = select(_e2428, 1f, (_e2428 > 1f));
                let _e2434 = ((_e2430 * _e2430) * (3f - (2f * _e2430)));
                let _e2437 = ((_e2420 - trunc(_e2420)) * _e2434);
                if (_e2394 < 0.5f) {
                    let _e2561 = select(_e2437, 0f, (_e2437 < 0f));
                    let _e2563 = select(_e2561, 1f, (_e2561 > 1f));
                    let _e2567 = ((_e2563 * _e2563) * (3f - (2f * _e2563)));
                    let _e2573 = (1f - _e2437);
                    let _e2582 = (_e2567 * 0.7f);
                    let _e2583 = (_e2582 + 1.5999999f);
                    let _e2588 = ((abs((sqrt(((_e2415 * _e2415) + (_e2416 * _e2416))) - ((7.5f - (_e2437 * 4.6f)) + (((sin((_e551 * 8f)) * _e2437) * _e2573) * 0.16f)))) - _e2583) / ((_e2582 + 0.49999994f) - _e2583));
                    let _e2590 = select(_e2588, 0f, (_e2588 < 0f));
                    let _e2592 = select(_e2590, 1f, (_e2590 > 1f));
                    let _e2601 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2415, (_e2416 - -7f)), vec2<f32>((3f * _e2573), 3f), 0.5f);
                    let _e2603 = ((_e2601 - 0.55f) * -0.9090909f);
                    let _e2605 = select(_e2603, 0f, (_e2603 < 0f));
                    let _e2607 = select(_e2605, 1f, (_e2605 > 1f));
                    let _e2621 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2415, (_e2416 - (-5f + (_e2437 * 3.5f)))), vec2<f32>((1.05f + (_e2567 * 0.45f)), (4.6f - (_e2437 * 3f))), 0.7f);
                    let _e2623 = ((_e2621 - 0.55f) * -0.9090909f);
                    let _e2625 = select(_e2623, 0f, (_e2623 < 0f));
                    let _e2627 = select(_e2625, 1f, (_e2625 > 1f));
                    let _e2631 = ((_e2627 * _e2627) * (3f - (2f * _e2627)));
                    let _e2633 = (((_e2592 * _e2592) * (3f - (2f * _e2592))) * (1f - ((_e2607 * _e2607) * (3f - (2f * _e2607)))));
                    if (_e2633 != _e2633) {
                        phi_14475_ = true;
                    } else {
                        phi_14475_ = (_e2631 >= _e2633);
                    }
                    let _e2637 = phi_14475_;
                    phi_10746_ = select(_e2633, _e2631, _e2637);
                } else {
                    let _e2440 = ((1f - _e2434) + _e2437);
                    let _e2444 = (((atan2(_e2416, _e2415) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e2448 = ((_e2440 * 0.82f) - 0.045f);
                    if (_e2448 != _e2448) {
                        phi_14400_ = true;
                    } else {
                        phi_14400_ = (0f >= _e2448);
                    }
                    let _e2452 = phi_14400_;
                    let _e2453 = select(_e2448, 0f, _e2452);
                    let _e2461 = ((abs((sqrt(((_e2415 * _e2415) + (_e2416 * _e2416))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e2463 = select(_e2461, 0f, (_e2461 < 0f));
                    let _e2465 = select(_e2463, 1f, (_e2463 > 1f));
                    let _e2470 = (_e2453 + 0.008f);
                    let _e2474 = (((_e2444 - trunc(_e2444)) - _e2470) / ((_e2453 - 0.008f) - _e2470));
                    let _e2476 = select(_e2474, 0f, (_e2474 < 0f));
                    let _e2478 = select(_e2476, 1f, (_e2476 > 1f));
                    let _e2484 = (_e2440 * 50f);
                    let _e2486 = select(_e2484, 0f, (_e2484 < 0f));
                    let _e2488 = select(_e2486, 1f, (_e2486 > 1f));
                    let _e2493 = ((((_e2465 * _e2465) * (3f - (2f * _e2465))) * ((_e2478 * _e2478) * (3f - (2f * _e2478)))) * ((_e2488 * _e2488) * (3f - (2f * _e2488))));
                    let _e2495 = (0.50265485f + (5.152212f * _e2440));
                    let _e2496 = cos(_e2495);
                    let _e2497 = sin(_e2495);
                    let _e2501 = (_e2415 - (_e2496 * 7.1f));
                    let _e2502 = (_e2416 - (_e2497 * 7.1f));
                    let _e2505 = ((_e2501 * -(_e2497)) + (_e2502 * _e2496));
                    let _e2508 = ((_e2501 * _e2496) + (_e2502 * _e2497));
                    let _e2509 = (_e2505 * -3.2f);
                    let _e2512 = ((_e2509 + (_e2508 * 2.1f)) * 0.06825939f);
                    let _e2514 = select(_e2512, 0f, (_e2512 < 0f));
                    let _e2516 = select(_e2514, 1f, (_e2514 > 1f));
                    let _e2519 = (_e2505 - (-3.2f * _e2516));
                    let _e2520 = (_e2508 - (2.1f * _e2516));
                    let _e2524 = sqrt(((_e2519 * _e2519) + (_e2520 * _e2520)));
                    let _e2527 = ((_e2509 + (_e2508 * -2.1f)) * 0.06825939f);
                    let _e2529 = select(_e2527, 0f, (_e2527 < 0f));
                    let _e2531 = select(_e2529, 1f, (_e2529 > 1f));
                    let _e2534 = (_e2505 - (-3.2f * _e2531));
                    let _e2535 = (_e2508 - (-2.1f * _e2531));
                    let _e2539 = sqrt(((_e2534 * _e2534) + (_e2535 * _e2535)));
                    if (_e2524 != _e2524) {
                        phi_14445_ = true;
                    } else {
                        phi_14445_ = (_e2539 <= _e2524);
                    }
                    let _e2543 = phi_14445_;
                    let _e2546 = ((select(_e2524, _e2539, _e2543) - 1.7f) * -0.71428573f);
                    let _e2548 = select(_e2546, 0f, (_e2546 < 0f));
                    let _e2550 = select(_e2548, 1f, (_e2548 > 1f));
                    let _e2554 = ((_e2550 * _e2550) * (3f - (2f * _e2550)));
                    if (_e2493 != _e2493) {
                        phi_14460_ = true;
                    } else {
                        phi_14460_ = (_e2554 >= _e2493);
                    }
                    let _e2558 = phi_14460_;
                    phi_10746_ = select(_e2493, _e2554, _e2558);
                }
                let _e2640 = phi_10746_;
                let _e2643 = (_e2434 * (0.5f + (_e2437 * 0.5f)));
                if (_e2412 != _e2412) {
                    phi_14490_ = true;
                } else {
                    phi_14490_ = (_e2643 >= _e2412);
                }
                let _e2647 = phi_14490_;
                let _e2648 = select(_e2412, _e2643, _e2647);
                let _e2650 = (0.48f * (1f - _e2648));
                let _e2661 = (1f + (_e2437 * 0.45f));
                phi_10773_ = vec3<f32>((_e1165 + (((_e2650 + (0.78f * _e2648)) * _e2640) * _e2661)), (_e1166 + (((_e2650 + (0.3f * _e2648)) * _e2640) * _e2661)), (_e1167 + (((_e2650 + (0.28f * _e2648)) * _e2640) * _e2661)));
            } else {
                phi_10773_ = _e2390;
            }
            let _e2670 = phi_10773_;
            let _e2672 = select(1u, 0u, (_e804.rgb == 0u));
            switch bitcast<i32>(_e804.rgb) {
                case 0: {
                    phi_10783_ = true;
                    break;
                }
                case 1: {
                    phi_10783_ = true;
                    break;
                }
                default: {
                    phi_10783_ = false;
                    break;
                }
            }
            let _e2675 = phi_10783_;
            if _e2675 {
                if (_e2672 < 2u) {
                } else {
                    break;
                }
                let _e2684 = pill_2.member[_e229].text.lines[_e2672].min[0u];
                let _e2692 = pill_2.member[_e229].text.lines[_e2672].min[1u];
                let _e2700 = pill_2.member[_e229].text.lines[_e2672].max[0u];
                let _e2708 = pill_2.member[_e229].text.lines[_e2672].max[1u];
                let _e2716 = pill_2.member[_e229].text.lines[_e2672].origin[0u];
                let _e2724 = pill_2.member[_e229].text.lines[_e2672].origin[1u];
                let _e2731 = pill_2.member[_e229].text.lines[_e2672].size;
                let _e2738 = pill_2.member[_e229].text.lines[_e2672].weight;
                let _e2745 = pill_2.member[_e229].text.lines[_e2672].count;
                let _e2752 = pill_2.member[_e229].text.lines[_e2672].first;
                if (_e298 < _e2684) {
                    phi_11041_ = f32();
                    phi_11042_ = true;
                } else {
                    if (_e298 > _e2700) {
                        phi_11039_ = f32();
                        phi_11040_ = true;
                    } else {
                        if (_e299 < _e2692) {
                            phi_11037_ = f32();
                            phi_11038_ = true;
                        } else {
                            let _e2756 = (_e299 > _e2708);
                            if _e2756 {
                                phi_11036_ = f32();
                            } else {
                                phi_10853_ = _e2745;
                                phi_10856_ = 0u;
                                loop {
                                    let _e2758 = phi_10853_;
                                    let _e2760 = phi_10856_;
                                    local_52 = _e2760;
                                    let _e2761 = (_e2760 < _e2758);
                                    if _e2761 {
                                        let _e2764 = (_e2760 + ((_e2758 - _e2760) / 2u));
                                        let _e2765 = (_e2752 + _e2764);
                                        if (_e2765 < 32u) {
                                        } else {
                                            phi_15687_ = true;
                                            break;
                                        }
                                        let _e2773 = pill_2.member[_e229].text.glyphs[_e2765].x;
                                        let _e2776 = (_e2773 <= ((_e298 - _e2716) / _e2731));
                                        if _e2776 {
                                            phi_10890_ = (_e2764 + 1u);
                                        } else {
                                            phi_10890_ = _e2760;
                                        }
                                        let _e2779 = phi_10890_;
                                        phi_10854_ = select(_e2764, _e2758, _e2776);
                                        phi_10857_ = _e2779;
                                    } else {
                                        phi_10854_ = u32();
                                        phi_10857_ = u32();
                                    }
                                    let _e2782 = phi_10854_;
                                    let _e2784 = phi_10857_;
                                    continue;
                                    continuing {
                                        phi_10853_ = _e2782;
                                        phi_10856_ = _e2784;
                                        phi_15687_ = _e2388;
                                        break if !(_e2761);
                                    }
                                }
                                let _e2787 = phi_15687_;
                                if _e2787 {
                                    break;
                                }
                                let _e2789 = local_52;
                                let _e2790 = (_e2789 + 1u);
                                phi_15737_ = _e2787;
                                phi_10898_ = select(_e2790, _e2745, (_e2745 < _e2790));
                                phi_10901_ = -1000000f;
                                loop {
                                    let _e2794 = phi_15737_;
                                    let _e2796 = phi_10898_;
                                    let _e2798 = phi_10901_;
                                    local_57 = _e2798;
                                    if (_e2796 > 0u) {
                                        let _e2800 = (_e2796 - 1u);
                                        let _e2801 = (_e2752 + _e2800);
                                        if (_e2801 < 32u) {
                                        } else {
                                            phi_15741_ = true;
                                            break;
                                        }
                                        let _e2809 = pill_2.member[_e229].text.glyphs[_e2801].x;
                                        let _e2816 = pill_2.member[_e229].text.glyphs[_e2801].glyph;
                                        if (_e2816 < arrayLength((&glyphs.member))) {
                                        } else {
                                            phi_15741_ = true;
                                            break;
                                        }
                                        let _e2822 = glyphs.member[_e2816].min[0u];
                                        let _e2827 = glyphs.member[_e2816].min[1u];
                                        let _e2832 = glyphs.member[_e2816].max[0u];
                                        let _e2837 = glyphs.member[_e2816].max[1u];
                                        let _e2841 = glyphs.member[_e2816].start;
                                        let _e2845 = glyphs.member[_e2816].count;
                                        let _e2848 = (((_e298 - _e2716) / _e2731) - _e2809);
                                        let _e2851 = (-((_e299 - _e2724)) / _e2731);
                                        let _e2852 = (3.5f / _e2731);
                                        let _e2853 = (_e2832 + _e2852);
                                        let _e2854 = (_e2848 > _e2853);
                                        if _e2854 {
                                            phi_15743_ = _e2794;
                                            phi_11029_ = f32();
                                        } else {
                                            if (_e2848 >= (_e2822 - _e2852)) {
                                                if (_e2851 >= (_e2827 - _e2852)) {
                                                    if (_e2848 <= _e2853) {
                                                        if (_e2851 <= (_e2837 + _e2852)) {
                                                            phi_10988_ = 0u;
                                                            phi_10991_ = 0i;
                                                            phi_10993_ = 340282350000000000000000000000000000000f;
                                                            loop {
                                                                let _e2863 = phi_10988_;
                                                                let _e2865 = phi_10991_;
                                                                let _e2867 = phi_10993_;
                                                                local_53 = _e2867;
                                                                local_54 = _e2865;
                                                                let _e2868 = (_e2863 < _e2845);
                                                                if _e2868 {
                                                                    let _e2869 = (_e2841 + _e2863);
                                                                    if (_e2869 < arrayLength((&edges.member))) {
                                                                    } else {
                                                                        phi_15734_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2873 = edges.member[_e2869];
                                                                    let _e2875 = cantus_render_text_edge_distance(_e2873, _e2738, vec2<f32>(_e2848, _e2851));
                                                                    if (_e2867 != _e2867) {
                                                                        phi_14505_ = true;
                                                                    } else {
                                                                        phi_14505_ = (_e2875.member <= _e2867);
                                                                    }
                                                                    let _e2881 = phi_14505_;
                                                                    phi_10989_ = (_e2863 + 1u);
                                                                    phi_10992_ = (_e2865 + _e2875.member_1);
                                                                    phi_10994_ = select(_e2867, _e2875.member, _e2881);
                                                                } else {
                                                                    phi_10989_ = u32();
                                                                    phi_10992_ = i32();
                                                                    phi_10994_ = f32();
                                                                }
                                                                let _e2886 = phi_10989_;
                                                                let _e2888 = phi_10992_;
                                                                let _e2890 = phi_10994_;
                                                                continue;
                                                                continuing {
                                                                    phi_10988_ = _e2886;
                                                                    phi_10991_ = _e2888;
                                                                    phi_10993_ = _e2890;
                                                                    phi_15734_ = _e2794;
                                                                    break if !(_e2868);
                                                                }
                                                            }
                                                            let _e2893 = phi_15734_;
                                                            phi_15741_ = _e2893;
                                                            if _e2893 {
                                                                break;
                                                            }
                                                            let _e2895 = local_53;
                                                            let _e2899 = local_54;
                                                            let _e2902 = ((sqrt(_e2895) * _e2731) * select(1f, -1f, (_e2899 == 0i)));
                                                            if (_e2798 != _e2798) {
                                                                phi_14520_ = true;
                                                            } else {
                                                                phi_14520_ = (_e2902 >= _e2798);
                                                            }
                                                            let _e2906 = phi_14520_;
                                                            phi_15747_ = _e2893;
                                                            phi_11025_ = select(_e2798, _e2902, _e2906);
                                                        } else {
                                                            phi_15747_ = _e2794;
                                                            phi_11025_ = _e2798;
                                                        }
                                                        let _e2909 = phi_15747_;
                                                        let _e2911 = phi_11025_;
                                                        phi_15746_ = _e2909;
                                                        phi_11026_ = _e2911;
                                                    } else {
                                                        phi_15746_ = _e2794;
                                                        phi_11026_ = _e2798;
                                                    }
                                                    let _e2913 = phi_15746_;
                                                    let _e2915 = phi_11026_;
                                                    phi_15745_ = _e2913;
                                                    phi_11027_ = _e2915;
                                                } else {
                                                    phi_15745_ = _e2794;
                                                    phi_11027_ = _e2798;
                                                }
                                                let _e2917 = phi_15745_;
                                                let _e2919 = phi_11027_;
                                                phi_15744_ = _e2917;
                                                phi_11028_ = _e2919;
                                            } else {
                                                phi_15744_ = _e2794;
                                                phi_11028_ = _e2798;
                                            }
                                            let _e2921 = phi_15744_;
                                            let _e2923 = phi_11028_;
                                            phi_15743_ = _e2921;
                                            phi_11029_ = _e2923;
                                        }
                                        let _e2925 = phi_15743_;
                                        let _e2927 = phi_11029_;
                                        phi_15742_ = _e2925;
                                        phi_10899_ = _e2800;
                                        phi_10902_ = _e2927;
                                        phi_11031_ = select(true, false, _e2854);
                                    } else {
                                        phi_15742_ = _e2794;
                                        phi_10899_ = u32();
                                        phi_10902_ = f32();
                                        phi_11031_ = false;
                                    }
                                    let _e2930 = phi_15742_;
                                    let _e2932 = phi_10899_;
                                    let _e2934 = phi_10902_;
                                    let _e2936 = phi_11031_;
                                    continue;
                                    continuing {
                                        phi_15737_ = _e2930;
                                        phi_10898_ = _e2932;
                                        phi_10901_ = _e2934;
                                        phi_15741_ = _e2930;
                                        break if !(_e2936);
                                    }
                                }
                                let _e2939 = phi_15741_;
                                if _e2939 {
                                    break;
                                }
                                let _e3151 = local_57;
                                phi_11036_ = _e3151;
                            }
                            let _e2941 = phi_11036_;
                            phi_11037_ = _e2941;
                            phi_11038_ = _e2756;
                        }
                        let _e2943 = phi_11037_;
                        let _e2945 = phi_11038_;
                        phi_11039_ = _e2943;
                        phi_11040_ = _e2945;
                    }
                    let _e2947 = phi_11039_;
                    let _e2949 = phi_11040_;
                    phi_11041_ = _e2947;
                    phi_11042_ = _e2949;
                }
                let _e2951 = phi_11041_;
                let _e2953 = phi_11042_;
                phi_11047_ = select(_e2951, -1000000f, _e2953);
            } else {
                phi_11047_ = -1000000f;
            }
            let _e2956 = phi_11047_;
            let _e2958 = ((_e2956 * 1.25f) + 0.5f);
            let _e2960 = select(_e2958, 0f, (_e2958 < 0f));
            let _e2962 = select(_e2960, 1f, (_e2960 > 1f));
            let _e2966 = ((_e2962 * _e2962) * (3f - (2f * _e2962)));
            let _e2967 = (1f - _e2966);
            let _e2974 = (0.94f * _e2966);
            let _e2979 = local_55;
            let _e2981 = (1f - (_e2979 * 0.35f));
            let _e2986 = local_56;
            let _e2987 = (_e2986 * 0.33249998f);
            out_color = vec4<f32>((((((_e2670.x * _e2967) + _e2974) * _e2981) + _e2987) * _e501), (((((_e2670.y * _e2967) + _e2974) * _e2981) + _e2987) * _e501), (((((_e2670.z * _e2967) + _e2974) * _e2981) + _e2987) * _e501), _e514);
            break;
        }
    }
    return;
}

fn render_playhead_vertex_impl() {
    let _e14 = vertex_5;
    let _e15 = _isthmus_instance_index_7;
    let _e24 = frame.member[0u].playhead_x;
    let _e30 = frame.member[0u].panel_height;
    let _e33 = (_e24 + ((((f32((_e14 & 1u)) * 2f) - 1f) * _e30) * 0.4f));
    let _e37 = frame.member[0u].panel_top;
    let _e41 = ((_e37 - 5f) + (f32((_e14 >> bitcast<u32>(1i))) * (_e30 + 10f)));
    let _e46 = frame.member[0u].screen_size[0u];
    let _e51 = frame.member[0u].screen_size[1u];
    let _e54 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e33, _e41), vec2<f32>(_e46, _e51));
    out_position = _e54;
    out_world_pos[0u] = _e33;
    out_world_pos[1u] = _e41;
    out_isthmus_instance_index_1 = _e15;
    return;
}

fn render_playhead_fragment_impl() {
    var phi_14542_: bool;
    var phi_14557_: bool;
    var phi_14572_: bool;
    var phi_14589_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e30 = world_pos_1;
            let _e31 = _isthmus_instance_index_9;
            let _e37 = frame.member[0u].playhead_x;
            let _e41 = frame.member[0u].panel_top;
            let _e45 = frame.member[0u].panel_height;
            let _e48 = (_e30.x - _e37);
            let _e49 = (_e30.y - (_e41 + (_e45 * 0.5f)));
            let _e50 = abs(_e48);
            let _e51 = abs(_e49);
            let _e55 = state.member[_e31].bar_split;
            let _e58 = (_e45 * (0.5f - (0.375f * _e55)));
            let _e64 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e51 - ((_e45 - _e58) * 0.5f)), _e50), (_e58 * 0.5f), 4.5f);
            let _e67 = abs((_e50 - (4f * _e55)));
            let _e69 = (_e51 - (_e45 * 0.1f));
            if (_e69 != _e69) {
                phi_14542_ = true;
            } else {
                phi_14542_ = (0f >= _e69);
            }
            let _e73 = phi_14542_;
            let _e74 = select(_e69, 0f, _e73);
            let _e79 = (sqrt(((_e67 * _e67) + (_e74 * _e74))) - 3.5f);
            let _e84 = state.member[_e31].icon_morph;
            let _e88 = state.member[_e31].icon_presence;
            let _e92 = ((_e45 * 0.18f) * (1f + (_e84 * (1f - _e88))));
            let _e94 = (_e92 * 0.5f);
            let _e95 = abs(-(_e49));
            let _e97 = (_e95 + (1.7320508f * _e48));
            if (_e97 != _e97) {
                phi_14557_ = true;
            } else {
                phi_14557_ = (0f >= _e97);
            }
            let _e101 = phi_14557_;
            let _e102 = select(_e97, 0f, _e101);
            let _e105 = (_e95 - (0.5f * _e102));
            let _e107 = (_e92 - _e94);
            let _e109 = (_e107 * -0.8660254f);
            let _e110 = (_e107 * 0.8660254f);
            if (_e109 <= _e110) {
            } else {
                break;
            }
            let _e113 = select(_e105, _e109, (_e105 < _e109));
            let _e116 = (_e105 - select(_e113, _e110, (_e113 > _e110)));
            let _e117 = ((_e48 - (_e102 * 0.8660254f)) - (-0.5f * _e107));
            let _e128 = (_e79 + ((((sqrt(((_e116 * _e116) + (_e117 * _e117))) * select(1f, -1f, (_e117 > 0f))) - _e94) - _e79) * _e84));
            let _e129 = (_e64 - -0.8f);
            let _e131 = select(_e129, 0f, (_e129 < 0f));
            let _e133 = select(_e131, 1f, (_e131 > 1f));
            let _e138 = (1f - ((_e133 * _e133) * (3f - (2f * _e133))));
            let _e139 = (_e128 - -0.8f);
            let _e141 = select(_e139, 0f, (_e139 < 0f));
            let _e143 = select(_e141, 1f, (_e141 > 1f));
            let _e149 = ((1f - ((_e143 * _e143) * (3f - (2f * _e143)))) * _e88);
            if (_e149 != _e149) {
                phi_14572_ = true;
            } else {
                phi_14572_ = (_e138 >= _e149);
            }
            let _e153 = phi_14572_;
            let _e154 = select(_e149, _e138, _e153);
            if (_e154 <= 0f) {
                discard;
            }
            if (_e64 != _e64) {
                phi_14589_ = true;
            } else {
                phi_14589_ = (_e128 <= _e64);
            }
            let _e159 = phi_14589_;
            let _e162 = ((select(_e64, _e128, _e159) - -2.5f) * 0.6666667f);
            let _e164 = select(_e162, 0f, (_e162 < 0f));
            let _e166 = select(_e164, 1f, (_e164 > 1f));
            let _e170 = ((_e166 * _e166) * (3f - (2f * _e166)));
            let _e171 = (1f - _e170);
            let _e174 = (0.15f * _e170);
            out_color = vec4<f32>((_e171 + _e174), ((0.878f * _e171) + _e174), ((0.824f * _e171) + _e174), _e154);
            break;
        }
    }
    return;
}

fn render_particles_vertex_impl() {
    var phi_14614_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_11614_: isthmus_Vertex_render_tempo_Varyings;
    var phi_11615_: isthmus_Vertex_render_tempo_Varyings;
    var phi_11616_: bool;
    var phi_11624_: isthmus_Vertex_render_tempo_Varyings;

    let _e30 = vertex_5;
    let _e31 = _isthmus_instance_index_7;
    let _e35 = frame.member[0u].time;
    let _e39 = particle.member[_e31].end_time;
    let _e43 = particle.member[_e31].duration;
    let _e45 = (_e35 - (_e39 - _e43));
    if (_e45 < 0f) {
        phi_11615_ = isthmus_Vertex_render_tempo_Varyings();
        phi_11616_ = true;
    } else {
        let _e47 = (_e45 > _e43);
        if _e47 {
            phi_11614_ = isthmus_Vertex_render_tempo_Varyings();
        } else {
            let _e48 = (_e45 / _e43);
            let _e53 = particle.member[_e31].spawn_vel[0u];
            let _e58 = particle.member[_e31].spawn_vel[1u];
            let _e62 = sqrt(((_e53 * _e53) + (_e58 * _e58)));
            if (_e62 > 0.001f) {
                phi_14614_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e53 / _e62), (_e58 / _e62)), _e62);
            } else {
                phi_14614_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e62);
            }
            let _e70 = phi_14614_;
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
            phi_11614_ = isthmus_Vertex_render_tempo_Varyings(render_tempo_Varyings(vec4<f32>(((((_e125 + (_e116.x * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.y * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.z * 2f)) * 0.8f) + 0.2f) * 2f), (((1f - _e48) * ((_e159 * _e159) * (3f - (2f * _e159)))) * 0.3f)), vec2<f32>(_e82, _e83)), _e153);
        }
        let _e171 = phi_11614_;
        phi_11615_ = _e171;
        phi_11616_ = _e47;
    }
    let _e173 = phi_11615_;
    let _e175 = phi_11616_;
    if _e175 {
        phi_11624_ = isthmus_Vertex_render_tempo_Varyings(render_tempo_Varyings(vec4<f32>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f)), vec4<f32>(0f, 0f, 0f, 0f));
    } else {
        phi_11624_ = _e173;
    }
    let _e177 = phi_11624_;
    out_position = _e177.position;
    out_color = _e177.varyings.weather;
    out_uv[0u] = _e177.varyings.pixel.x;
    out_uv[1u] = _e177.varyings.pixel.y;
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

@vertex
fn render_tempo_vertex(@builtin(vertex_index) vertex: u32, @builtin(instance_index) _isthmus_instance_index: u32) -> VertexOutput {
    vertex_5 = vertex;
    _isthmus_instance_index_7 = _isthmus_instance_index;
    render_tempo_vertex_impl();
    let _e8 = out_position;
    let _e9 = out_pixel;
    let _e10 = out_weather;
    let _e11 = out_isthmus_instance_index;
    return VertexOutput(_e8, _e9, _e10, _e11);
}

@fragment
fn render_tempo_fragment(@location(0) pixel: vec2<f32>, @location(1) @interpolate(flat) weather: vec4<f32>, @location(2) @interpolate(flat) _isthmus_instance_index_1: u32) -> @location(0) vec4<f32> {
    pixel_2 = pixel;
    weather_1 = weather;
    _isthmus_instance_index_8 = _isthmus_instance_index_1;
    render_tempo_fragment_impl();
    let _e7 = out_color;
    return _e7;
}

@vertex
fn render_track_vertex(@builtin(vertex_index) vertex_1: u32, @builtin(instance_index) instance: u32) -> VertexOutput_1 {
    vertex_5 = vertex_1;
    instance_1 = instance;
    render_track_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel_pos;
    let _e9 = out_pill_idx;
    return VertexOutput_1(_e7, _e8, _e9);
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
fn render_status_vertex(@builtin(vertex_index) vertex_2: u32, @builtin(instance_index) _isthmus_instance_index_2: u32) -> VertexOutput_1 {
    vertex_5 = vertex_2;
    _isthmus_instance_index_7 = _isthmus_instance_index_2;
    render_status_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_pixel;
    let _e9 = out_isthmus_instance_index_1;
    return VertexOutput_1(_e7, _e8, _e9);
}

@fragment
fn render_status_fragment(@location(0) pixel_1: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_3: u32) -> @location(0) vec4<f32> {
    pixel_2 = pixel_1;
    _isthmus_instance_index_9 = _isthmus_instance_index_3;
    render_status_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_playhead_vertex(@builtin(vertex_index) vertex_3: u32, @builtin(instance_index) _isthmus_instance_index_4: u32) -> VertexOutput_1 {
    vertex_5 = vertex_3;
    _isthmus_instance_index_7 = _isthmus_instance_index_4;
    render_playhead_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_world_pos;
    let _e9 = out_isthmus_instance_index_1;
    return VertexOutput_1(_e7, _e8, _e9);
}

@fragment
fn render_playhead_fragment(@location(0) world_pos: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_5: u32) -> @location(0) vec4<f32> {
    world_pos_1 = world_pos;
    _isthmus_instance_index_9 = _isthmus_instance_index_5;
    render_playhead_fragment_impl();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_particles_vertex(@builtin(vertex_index) vertex_4: u32, @builtin(instance_index) _isthmus_instance_index_6: u32) -> VertexOutput_2 {
    vertex_5 = vertex_4;
    _isthmus_instance_index_7 = _isthmus_instance_index_6;
    render_particles_vertex_impl();
    let _e7 = out_position;
    let _e8 = out_color;
    let _e9 = out_uv;
    return VertexOutput_2(_e7, _e8, _e9);
}

@fragment
fn render_particles_fragment(@location(0) color: vec4<f32>, @location(1) uv: vec2<f32>) -> @location(0) vec4<f32> {
    color_1 = color;
    uv_1 = uv;
    render_particles_fragment_impl();
    let _e5 = out_color;
    return _e5;
}
