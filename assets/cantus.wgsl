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
    launcher_open: f32,
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

struct type_13 {
    member: array<render_track_TrackPill>,
}

struct render_text_PlacedGlyph {
    x: f32,
    glyph: u32,
}

struct type_26 {
    member: array<render_text_PlacedGlyph>,
}

struct render_text_Glyph {
    min: vec2<f32>,
    max: vec2<f32>,
    start: u32,
    count: u32,
}

struct type_29 {
    member: array<render_text_Glyph>,
}

struct render_text_Edge {
    start: vec2<f32>,
    end: vec2<f32>,
    start_delta: vec2<f32>,
    end_delta: vec2<f32>,
}

struct type_32 {
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

struct type_39 {
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

struct type_44 {
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

struct type_50 {
    member: array<render_launcher_LauncherRow>,
}

struct render_playhead_PlayheadState {
    bar_split: f32,
    icon_presence: f32,
    icon_morph: f32,
}

struct type_53 {
    member: array<render_playhead_PlayheadState>,
}

struct render_particles_Particle {
    spawn_pos: vec2<f32>,
    spawn_vel: vec2<f32>,
    end_time: f32,
    duration: f32,
    rgb: u32,
}

struct type_56 {
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

struct type_63 {
    member: array<render_tempestas_WeatherSurface>,
}

struct type_67 {
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
var<storage> pill: type_13;
var<private> out_position: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> out_pixel_pos: vec2<f32>;
var<private> out_pill_idx: u32;
var<private> pixel_pos_1: vec2<f32>;
var<private> pill_idx_1: u32;
@group(0) @binding(4)
var<storage> placed_glyphs: type_26;
@group(0) @binding(5)
var<storage> glyphs: type_29;
@group(0) @binding(6)
var<storage> edges: type_32;
var<private> global: vec2<f32> = vec2<f32>(0f, 0f);
@group(0) @binding(3)
var sampler_: sampler;
@group(0) @binding(2)
var images: texture_2d_array<f32>;
var<private> out_color: vec4<f32>;
@group(0) @binding(1)
var<storage> line: type_39;
var<private> _isthmus_instance_index_9: u32;
var<private> out_pixel: vec2<f32>;
var<private> out_isthmus_instance_index: u32;
var<private> pixel_4: vec2<f32>;
var<private> _isthmus_instance_index_10: u32;
@group(0) @binding(2)
var<storage> placed_glyphs_1: type_26;
@group(0) @binding(3)
var<storage> glyphs_1: type_29;
@group(0) @binding(4)
var<storage> edges_1: type_32;
@group(0) @binding(1)
var<storage> pill_1: type_44;
@group(0) @binding(1)
var<storage> row: type_50;
var<private> out_row_idx: u32;
var<private> row_idx_1: u32;
@group(0) @binding(2)
var icons: texture_2d_array<f32>;
var<private> out_world_pos: vec2<f32>;
var<private> world_pos_1: vec2<f32>;
@group(0) @binding(1)
var<storage> state: type_53;
@group(0) @binding(1)
var<storage> particle: type_56;
var<private> out_uv: vec2<f32>;
var<private> color_1: vec4<f32>;
var<private> uv_1: vec2<f32>;
@group(0) @binding(1)
var<storage> pill_2: type_63;
var<private> out_weather: vec4<f32>;
var<private> out_isthmus_instance_index_1: u32;
var<private> weather_1: vec4<f32>;
var<private> _isthmus_instance_index_11: u32;
@group(0) @binding(2)
var<storage> text_lines: type_39;
@group(0) @binding(3)
var<storage> text_cells: type_67;

fn function_() {
    var phi_643_: u32;
    var phi_646_: f32;
    var phi_644_: u32;
    var phi_647_: f32;
    var phi_21614_: bool;
    var local: f32;
    var phi_17433_: bool;
    var phi_17469_: bool;
    var phi_17491_: bool;
    var phi_17506_: bool;
    var phi_17530_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = vertex_7;
            let _e497 = instance_2;
            let _e501 = frame.member[0u].mouse_pressure;
            phi_643_ = 0u;
            phi_646_ = (_e501 * 8f);
            loop {
                let _e504 = phi_643_;
                let _e506 = phi_646_;
                local = _e506;
                let _e507 = (_e504 < 4u);
                if _e507 {
                    if _e507 {
                    } else {
                        phi_21614_ = true;
                        break;
                    }
                    let _e513 = frame.member[0u].ripples[_e504].start_time;
                    let _e519 = frame.member[0u].ripples[_e504].strength;
                    let _e523 = frame.member[0u].time;
                    let _e525 = ((_e523 - _e513) * 1.2f);
                    let _e527 = select(_e525, 0f, (_e525 < 0f));
                    let _e530 = (1f - select(_e527, 1f, (_e527 > 1f)));
                    phi_644_ = (_e504 + 1u);
                    phi_647_ = (_e506 + (((_e519 * _e530) * _e530) * 11f));
                } else {
                    phi_644_ = u32();
                    phi_647_ = f32();
                }
                let _e537 = phi_644_;
                let _e539 = phi_647_;
                continue;
                continuing {
                    phi_643_ = _e537;
                    phi_646_ = _e539;
                    phi_21614_ = false;
                    break if !(_e507);
                }
            }
            let _e542 = phi_21614_;
            if _e542 {
                break;
            }
            let _e544 = local;
            let _e545 = (_e544 * 0.5f);
            let _e546 = (18f + _e545);
            let _e550 = pill.member[_e497].width;
            let _e554 = frame.member[0u].panel_height;
            let _e558 = pill.member[_e497].x;
            let _e560 = (_e558 + (_e550 * 0.5f));
            let _e566 = pill.member[_e497].rating;
            let _e572 = pill.member[_e497].primary_playlist_count;
            let _e574 = (select(0f, 5f, (_e566 >= 0i)) + f32(_e572));
            let _e578 = pill.member[_e497].secondary_expansion;
            let _e584 = pill.member[_e497].secondary_playlist_count;
            let _e585 = f32(_e584);
            let _e589 = pill.member[_e497].primary_alpha;
            let _e590 = (_e574 - 1f);
            if (_e590 != _e590) {
                phi_17433_ = true;
            } else {
                phi_17433_ = (0f >= _e590);
            }
            let _e594 = phi_17433_;
            let _e600 = select(0f, 1f, ((_e574 * _e589) > 0f));
            let _e601 = (((select(_e590, 0f, _e594) * 9f) + 32.4f) * _e600);
            let _e602 = (32.4f * _e600);
            let _e603 = (_e585 - 1f);
            if (_e603 != _e603) {
                phi_17469_ = true;
            } else {
                phi_17469_ = (0f >= _e603);
            }
            let _e607 = phi_17469_;
            let _e615 = select(0f, 1f, ((_e585 * _e578) > 0f));
            let _e616 = (((((select(_e603, 0f, _e607) * 18f) * _e578) * 0.5f) + 32.4f) * _e615);
            let _e617 = (32.4f * _e615);
            let _e619 = select(_e616, _e601, (_e601 > _e616));
            let _e622 = (_e558 - _e546);
            let _e623 = (_e560 - _e619);
            if (_e622 != _e622) {
                phi_17491_ = true;
            } else {
                phi_17491_ = (_e623 <= _e622);
            }
            let _e627 = phi_17491_;
            let _e628 = select(_e622, _e623, _e627);
            let _e629 = (-12f - _e545);
            let _e631 = ((_e558 + _e550) + _e546);
            let _e632 = (_e560 + _e619);
            if (_e631 != _e631) {
                phi_17506_ = true;
            } else {
                phi_17506_ = (_e632 >= _e631);
            }
            let _e636 = phi_17506_;
            let _e639 = ((6f + _e554) + _e546);
            let _e641 = (((((_e554 * 0.975f) + 3f) + (18f * _e578)) + -5.4f) + select(_e617, _e602, (_e602 > _e617)));
            if (_e639 != _e639) {
                phi_17530_ = true;
            } else {
                phi_17530_ = (_e641 >= _e639);
            }
            let _e645 = phi_17530_;
            let _e656 = (_e628 + (f32((_e496 & 1u)) * (select(_e631, _e632, _e636) - _e628)));
            let _e657 = (_e629 + (f32((_e496 >> bitcast<u32>(1i))) * (select(_e639, _e641, _e645) - _e629)));
            let _e662 = frame.member[0u].screen_size[0u];
            let _e667 = frame.member[0u].screen_size[1u];
            out_position = vec4<f32>((((_e656 / _e662) * 2f) - 1f), (((_e657 / _e667) * 2f) - 1f), 0f, 1f);
            out_pixel_pos[0u] = _e656;
            out_pixel_pos[1u] = _e657;
            out_pill_idx = _e497;
            break;
        }
    }
    return;
}

fn cantus_render_text_edge_distance(param: render_text_Edge, param_1: f32, param_2: vec2<f32>, param_3: f32) -> u0028_f32_u0020_i32_u0029_ {
    var phi_3823_: bool;
    var phi_3836_: bool;
    var phi_3841_: bool;
    var phi_3858_: i32;
    var phi_3859_: i32;
    var phi_21493_: bool;
    var phi_21508_: bool;
    var phi_21523_: bool;
    var phi_3932_: u0028_f32_u0020_i32_u0029_;

    let _e511 = (param.start.x + (param.start_delta.x * param_1));
    let _e512 = (param.start.y + (param.start_delta.y * param_1));
    let _e523 = (param.end.x + (param.end_delta.x * param_1));
    let _e524 = (param.end.y + (param.end_delta.y * param_1));
    let _e525 = (_e523 - _e511);
    let _e526 = (_e524 - _e512);
    if (_e512 <= param_2.y) {
        phi_3823_ = select(true, false, (param_2.y < _e524));
    } else {
        phi_3823_ = true;
    }
    let _e531 = phi_3823_;
    if _e531 {
        if (_e524 <= param_2.y) {
            phi_3836_ = select(true, false, (param_2.y < _e512));
        } else {
            phi_3836_ = true;
        }
        let _e536 = phi_3836_;
        phi_3841_ = select(true, false, _e536);
    } else {
        phi_3841_ = true;
    }
    let _e539 = phi_3841_;
    if _e539 {
        if ((_e511 + (((param_2.y - _e512) * _e525) / _e526)) > param_2.x) {
            phi_3858_ = select(-1i, 1i, (_e526 > 0f));
        } else {
            phi_3858_ = 0i;
        }
        let _e548 = phi_3858_;
        phi_3859_ = _e548;
    } else {
        phi_3859_ = 0i;
    }
    let _e550 = phi_3859_;
    let _e552 = select(_e523, _e511, (_e511 < _e523));
    let _e554 = select(_e524, _e512, (_e512 < _e524));
    let _e556 = select(_e523, _e511, (_e511 > _e523));
    let _e558 = select(_e524, _e512, (_e512 > _e524));
    let _e560 = select(_e552, param_2.x, (param_2.x > _e552));
    let _e562 = select(_e554, param_2.y, (param_2.y > _e554));
    let _e567 = (param_2.x - select(_e556, _e560, (_e560 < _e556)));
    let _e568 = (param_2.y - select(_e558, _e562, (_e562 < _e558)));
    if (((_e567 * _e567) + (_e568 * _e568)) >= param_3) {
        phi_3932_ = u0028_f32_u0020_i32_u0029_(param_3, _e550);
    } else {
        let _e580 = ((_e525 * _e525) + (_e526 * _e526));
        if (_e580 != _e580) {
            phi_21493_ = true;
        } else {
            phi_21493_ = (0.00000001f >= _e580);
        }
        let _e584 = phi_21493_;
        let _e586 = ((((param_2.x - _e511) * _e525) + ((param_2.y - _e512) * _e526)) / select(_e580, 0.00000001f, _e584));
        if (_e586 != _e586) {
            phi_21508_ = true;
        } else {
            phi_21508_ = (0f >= _e586);
        }
        let _e590 = phi_21508_;
        let _e591 = select(_e586, 0f, _e590);
        if (_e591 != _e591) {
            phi_21523_ = true;
        } else {
            phi_21523_ = (1f <= _e591);
        }
        let _e595 = phi_21523_;
        let _e596 = select(_e591, 1f, _e595);
        let _e601 = (param_2.x - (_e511 + (_e525 * _e596)));
        let _e602 = (param_2.y - (_e512 + (_e526 * _e596)));
        phi_3932_ = u0028_f32_u0020_i32_u0029_(((_e601 * _e601) + (_e602 * _e602)), _e550);
    }
    let _e609 = phi_3932_;
    return _e609;
}

fn cantus_render_shader_hash(param_4: vec2<f32>) -> vec2<f32> {
    let _e517 = ((bitcast<u32>(select(0i, select(select(i32(param_4.y), i32(-2147483648), (param_4.y < -2147483600f)), 2147483647i, (param_4.y > 2147483500f)), (param_4.y == param_4.y))) * 1664525u) + 1013904223u);
    let _e519 = (((bitcast<u32>(select(0i, select(select(i32(param_4.x), i32(-2147483648), (param_4.x < -2147483600f)), 2147483647i, (param_4.x > 2147483500f)), (param_4.x == param_4.x))) * 1664525u) + 1013904223u) + (_e517 * 1664525u));
    let _e521 = (_e517 + (_e519 * 1664525u));
    let _e527 = (_e521 ^ (_e521 >> bitcast<u32>(16i)));
    let _e529 = ((_e519 ^ (_e519 >> bitcast<u32>(16i))) + (_e527 * 1664525u));
    let _e531 = (_e527 + (_e529 * 1664525u));
    return vec2<f32>((f32((_e529 ^ (_e529 >> bitcast<u32>(16i)))) * 0.00000000023283064f), (f32((_e531 ^ (_e531 >> bitcast<u32>(16i)))) * 0.00000000023283064f));
}

fn cantus_render_shader_simplex_noise(param_5: vec2<f32>) -> f32 {
    var phi_21448_: bool;
    var phi_21463_: bool;
    var phi_21478_: bool;

    let _e499 = ((param_5.x + param_5.y) * 0.36602542f);
    let _e502 = floor((param_5.x + _e499));
    let _e503 = floor((param_5.y + _e499));
    let _e507 = ((_e502 + _e503) * 0.21132487f);
    let _e508 = ((param_5.x - _e502) + _e507);
    let _e509 = ((param_5.y - _e503) + _e507);
    let _e512 = select(vec2<f32>(0f, 1f), vec2<f32>(1f, 0f), vec2((_e508 > _e509)));
    let _e517 = ((_e508 - _e512.x) + 0.21132487f);
    let _e518 = ((_e509 - _e512.y) + 0.21132487f);
    let _e519 = (_e508 + -0.57735026f);
    let _e520 = (_e509 + -0.57735026f);
    let _e524 = (0.5f - ((_e508 * _e508) + (_e509 * _e509)));
    if (_e524 != _e524) {
        phi_21448_ = true;
    } else {
        phi_21448_ = (0f >= _e524);
    }
    let _e528 = phi_21448_;
    let _e529 = select(_e524, 0f, _e528);
    let _e534 = cantus_render_shader_hash(vec2<f32>(_e502, _e503));
    let _e548 = (0.5f - ((_e517 * _e517) + (_e518 * _e518)));
    if (_e548 != _e548) {
        phi_21463_ = true;
    } else {
        phi_21463_ = (0f >= _e548);
    }
    let _e552 = phi_21463_;
    let _e553 = select(_e548, 0f, _e552);
    let _e560 = cantus_render_shader_hash(vec2<f32>((_e502 + _e512.x), (_e503 + _e512.y)));
    let _e575 = (0.5f - ((_e519 * _e519) + (_e520 * _e520)));
    if (_e575 != _e575) {
        phi_21478_ = true;
    } else {
        phi_21478_ = (0f >= _e575);
    }
    let _e579 = phi_21478_;
    let _e580 = select(_e575, 0f, _e579);
    let _e587 = cantus_render_shader_hash(vec2<f32>((_e502 + 1f), (_e503 + 1f)));
    return (70f * ((((((_e529 * _e529) * _e529) * _e529) * ((_e508 * ((_e534.x * 2f) - 1f)) + (_e509 * ((_e534.y * 2f) - 1f)))) + ((((_e553 * _e553) * _e553) * _e553) * ((_e517 * ((_e560.x * 2f) - 1f)) + (_e518 * ((_e560.y * 2f) - 1f))))) + ((((_e580 * _e580) * _e580) * _e580) * ((_e519 * ((_e587.x * 2f) - 1f)) + (_e520 * ((_e587.y * 2f) - 1f))))));
}

fn cantus_render_track_plasma_field(param_6: vec2<f32>, param_7: vec4<f32>, param_8: f32, param_9: f32, param_10: f32) -> vec4<f32> {
    let _e508 = ((sin((((param_6.x * param_8) + (param_6.y * param_9)) + param_10)) * 0.5f) + 0.5f);
    let _e514 = ((0.12f + (_e508 * _e508)) * (0.25f + (param_7.w * 3f)));
    return vec4<f32>((param_7.x * _e514), (param_7.y * _e514), (param_7.z * _e514), _e514);
}

fn cantus_render_shader_sd_capsule_box(param_11: vec2<f32>, param_12: f32, param_13: f32) -> f32 {
    var phi_21418_: bool;
    var phi_21433_: bool;

    let _e501 = abs(param_11.y);
    let _e502 = (abs(param_11.x) - param_12);
    let _e504 = select(0f, _e502, (_e502 > 0f));
    let _e506 = select(0f, _e501, (_e501 > 0f));
    if (_e502 != _e502) {
        phi_21418_ = true;
    } else {
        phi_21418_ = (_e501 >= _e502);
    }
    let _e514 = phi_21418_;
    let _e515 = select(_e502, _e501, _e514);
    if (_e515 != _e515) {
        phi_21433_ = true;
    } else {
        phi_21433_ = (0f <= _e515);
    }
    let _e519 = phi_21433_;
    return ((sqrt(((_e504 * _e504) + (_e506 * _e506))) + select(_e515, 0f, _e519)) - param_13);
}

fn function_1() {
    var phi_988_: f32;
    var phi_991_: vec2<f32>;
    var phi_994_: f32;
    var phi_996_: u32;
    var phi_17579_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_17590_: bool;
    var phi_1101_: vec2<f32>;
    var phi_1102_: f32;
    var phi_1103_: vec2<f32>;
    var phi_1104_: f32;
    var phi_992_: vec2<f32>;
    var phi_995_: f32;
    var phi_997_: u32;
    var phi_21619_: bool;
    var phi_1148_: f32;
    var local_1: vec2<f32>;
    var local_2: vec2<f32>;
    var phi_1160_: bool;
    var local_3: vec2<f32>;
    var phi_1171_: f32;
    var local_4: vec2<f32>;
    var phi_17648_: bool;
    var phi_17672_: bool;
    var phi_17729_: bool;
    var phi_17753_: bool;
    var phi_17778_: bool;
    var phi_17793_: bool;
    var phi_17808_: bool;
    var phi_17825_: bool;
    var phi_17866_: bool;
    var phi_17881_: bool;
    var phi_17920_: bool;
    var phi_17935_: bool;
    var phi_18150_: bool;
    var phi_18177_: bool;
    var phi_18192_: bool;
    var phi_18127_: f32;
    var phi_2143_: vec3<f32>;
    var phi_2144_: vec3<f32>;
    var local_5: f32;
    var local_6: f32;
    var local_7: f32;
    var local_8: f32;
    var phi_2228_: vec4<f32>;
    var phi_2231_: u32;
    var phi_18231_: bool;
    var phi_18266_: bool;
    var phi_18281_: bool;
    var phi_18296_: bool;
    var phi_18311_: bool;
    var phi_2528_: vec4<f32>;
    var phi_2229_: vec4<f32>;
    var phi_2232_: u32;
    var phi_2530_: vec4<f32>;
    var phi_2531_: vec4<f32>;
    var phi_2543_: vec4<f32>;
    var phi_2546_: u32;
    var phi_2586_: render_RipplePulse;
    var phi_2588_: f32;
    var phi_18341_: bool;
    var phi_2707_: bool;
    var phi_2712_: bool;
    var phi_18378_: bool;
    var phi_18393_: bool;
    var phi_2814_: vec4<f32>;
    var phi_2815_: vec4<f32>;
    var phi_2816_: vec4<f32>;
    var phi_2817_: vec4<f32>;
    var phi_2544_: vec4<f32>;
    var phi_2547_: u32;
    var phi_21749_: bool;
    var phi_2833_: u32;
    var phi_2836_: u32;
    var phi_2864_: u32;
    var phi_2834_: u32;
    var phi_2837_: u32;
    var local_9: u32;
    var phi_2876_: u32;
    var phi_2879_: f32;
    var phi_2959_: f32;
    var phi_2962_: u32;
    var phi_2964_: i32;
    var phi_2960_: f32;
    var phi_2963_: u32;
    var phi_2965_: i32;
    var local_10: f32;
    var phi_2997_: f32;
    var local_11: i32;
    var phi_18408_: bool;
    var phi_3005_: f32;
    var phi_3006_: f32;
    var phi_3007_: f32;
    var phi_3008_: f32;
    var phi_3009_: f32;
    var phi_2877_: u32;
    var phi_2880_: f32;
    var phi_3011_: bool;
    var local_12: f32;
    var phi_3050_: u32;
    var phi_3053_: u32;
    var phi_3081_: u32;
    var phi_3051_: u32;
    var phi_3054_: u32;
    var local_13: u32;
    var phi_3093_: u32;
    var phi_3096_: f32;
    var phi_3176_: f32;
    var phi_3179_: u32;
    var phi_3181_: i32;
    var phi_3177_: f32;
    var phi_3180_: u32;
    var phi_3182_: i32;
    var local_14: f32;
    var phi_3214_: f32;
    var local_15: i32;
    var phi_18423_: bool;
    var phi_3222_: f32;
    var phi_3223_: f32;
    var phi_3224_: f32;
    var phi_3225_: f32;
    var phi_3226_: f32;
    var phi_3094_: u32;
    var phi_3097_: f32;
    var phi_3228_: bool;
    var local_16: f32;
    var phi_18438_: bool;
    var local_17: vec4<f32>;
    var local_18: vec4<f32>;
    var local_19: vec4<f32>;
    var local_20: vec4<f32>;
    var local_21: vec4<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = pixel_pos_1;
            let _e497 = pill_idx_1;
            let _e503 = frame.member[0u].launcher_open;
            if (_e503 > 0.5f) {
                discard;
            }
            let _e508 = pill.member[_e497].x;
            let _e512 = pill.member[_e497].width;
            let _e516 = frame.member[0u].panel_height;
            let _e517 = (_e496.x - _e508);
            let _e518 = (_e496.y - 6f);
            let _e519 = (_e512 * 0.5f);
            let _e520 = (_e516 * 0.5f);
            let _e522 = (_e518 - _e520);
            let _e523 = (_e512 - _e516);
            let _e524 = (_e523 * 0.5f);
            let _e526 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e517 - _e519), _e522), _e524, _e520);
            let _e530 = frame.member[0u].mouse_pressure;
            let _e531 = (_e530 > 0f);
            if _e531 {
                let _e536 = frame.member[0u].mouse_pos[0u];
                let _e541 = frame.member[0u].mouse_pos[1u];
                let _e547 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e536 - _e508) - _e519), ((_e541 - 6f) - _e520)), _e524, _e520);
                phi_988_ = _e547;
            } else {
                phi_988_ = 1f;
            }
            let _e549 = phi_988_;
            phi_991_ = vec2<f32>(0f, 0f);
            phi_994_ = 0f;
            phi_996_ = 0u;
            loop {
                let _e551 = phi_991_;
                let _e553 = phi_994_;
                let _e555 = phi_996_;
                local_1 = _e551;
                local_2 = _e551;
                local_3 = _e551;
                local_4 = _e551;
                local_5 = _e553;
                local_6 = _e553;
                local_7 = _e553;
                local_8 = _e553;
                let _e556 = (_e555 < 4u);
                if _e556 {
                    if _e556 {
                    } else {
                        phi_21619_ = true;
                        break;
                    }
                    let _e563 = frame.member[0u].ripples[_e555].origin[0u];
                    let _e570 = frame.member[0u].ripples[_e555].origin[1u];
                    let _e576 = frame.member[0u].ripples[_e555].start_time;
                    let _e582 = frame.member[0u].ripples[_e555].strength;
                    let _e586 = frame.member[0u].time;
                    let _e588 = ((_e586 - _e576) * 1.2f);
                    let _e590 = select(_e588, 0f, (_e588 < 0f));
                    let _e592 = select(_e590, 1f, (_e590 > 1f));
                    if (_e582 > 0f) {
                        if (_e592 < 1f) {
                            let _e595 = (_e496.x - _e563);
                            let _e596 = (_e496.y - _e570);
                            let _e600 = sqrt(((_e595 * _e595) + (_e596 * _e596)));
                            if (_e600 > 0.001f) {
                                phi_17579_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e595 / _e600), (_e596 / _e600)), _e600);
                            } else {
                                phi_17579_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e600);
                            }
                            let _e608 = phi_17579_;
                            let _e618 = ((abs((_e608.unnamed_1 - (_e592 * 600f))) - 80f) * -0.0125f);
                            let _e620 = select(_e618, 0f, (_e618 < 0f));
                            let _e622 = select(_e620, 1f, (_e620 > 1f));
                            let _e628 = (1f - _e592);
                            let _e629 = ((((_e622 * _e622) * (3f - (2f * _e622))) * _e582) * _e628);
                            let _e642 = (_e553 + (_e629 * 0.5f));
                            if (_e642 != _e642) {
                                phi_17590_ = true;
                            } else {
                                phi_17590_ = (1f <= _e642);
                            }
                            let _e646 = phi_17590_;
                            phi_1101_ = vec2<f32>((_e551.x + (((_e608.unnamed.x * _e629) * _e628) * 0.5f)), (_e551.y + (((_e608.unnamed.y * _e629) * _e628) * 0.5f)));
                            phi_1102_ = select(_e642, 1f, _e646);
                        } else {
                            phi_1101_ = _e551;
                            phi_1102_ = _e553;
                        }
                        let _e649 = phi_1101_;
                        let _e651 = phi_1102_;
                        phi_1103_ = _e649;
                        phi_1104_ = _e651;
                    } else {
                        phi_1103_ = _e551;
                        phi_1104_ = _e553;
                    }
                    let _e653 = phi_1103_;
                    let _e655 = phi_1104_;
                    phi_992_ = _e653;
                    phi_995_ = _e655;
                    phi_997_ = (_e555 + 1u);
                } else {
                    phi_992_ = vec2<f32>();
                    phi_995_ = f32();
                    phi_997_ = u32();
                }
                let _e658 = phi_992_;
                let _e660 = phi_995_;
                let _e662 = phi_997_;
                continue;
                continuing {
                    phi_991_ = _e658;
                    phi_994_ = _e660;
                    phi_996_ = _e662;
                    phi_21619_ = false;
                    break if !(_e556);
                }
            }
            let _e665 = phi_21619_;
            if _e665 {
                break;
            }
            if _e531 {
                let _e670 = frame.member[0u].mouse_pos[0u];
                let _e675 = frame.member[0u].mouse_pos[1u];
                let _e676 = (_e496.x - _e670);
                let _e677 = (_e496.y - _e675);
                let _e683 = ((sqrt(((_e676 * _e676) + (_e677 * _e677))) - 150f) * -0.006666667f);
                let _e685 = select(_e683, 0f, (_e683 < 0f));
                let _e687 = select(_e685, 1f, (_e685 > 1f));
                phi_1148_ = ((((_e687 * _e687) * (3f - (2f * _e687))) * _e530) * 8f);
            } else {
                phi_1148_ = 0f;
            }
            let _e695 = phi_1148_;
            let _e697 = local_1;
            let _e700 = global[0u];
            if (_e697.x == _e700) {
                let _e703 = local_2;
                let _e706 = global[1u];
                phi_1160_ = (_e703.y == _e706);
            } else {
                phi_1160_ = false;
            }
            let _e709 = phi_1160_;
            if _e709 {
                phi_1171_ = 0f;
            } else {
                let _e711 = local_3;
                phi_1171_ = (sqrt(((_e697.x * _e697.x) + (_e711.y * _e711.y))) * 22f);
            }
            let _e719 = phi_1171_;
            let _e721 = local_4;
            let _e723 = (_e517 / _e512);
            let _e724 = (_e518 / _e516);
            let _e725 = (_e723 - 0.5f);
            let _e726 = (_e724 - 0.5f);
            let _e727 = (_e508 + _e519);
            let _e728 = (_e516 * 0.975f);
            let _e729 = (_e728 + 3f);
            let _e733 = pill.member[_e497].rating;
            let _e734 = (_e733 >= 0i);
            let _e735 = select(0f, 5f, _e734);
            let _e739 = pill.member[_e497].primary_playlist_count;
            let _e741 = (_e735 + f32(_e739));
            let _e747 = pill.member[_e497].secondary_expansion;
            let _e749 = (_e729 + (18f * _e747));
            let _e753 = pill.member[_e497].secondary_playlist_count;
            let _e754 = f32(_e753);
            let _e761 = frame.member[0u].mouse_pos[0u];
            let _e766 = frame.member[0u].mouse_pos[1u];
            let _e767 = vec2<f32>(_e761, _e766);
            let _e769 = (_e741 - 1f);
            let _e770 = (_e769 != _e769);
            if _e770 {
                phi_17648_ = true;
            } else {
                phi_17648_ = (0f >= _e769);
            }
            let _e773 = phi_17648_;
            let _e776 = vec2<f32>(_e727, (_e728 + -4.4f));
            let _e778 = cantus_render_shader_sd_capsule_box((_e496 - _e776), (select(_e769, 0f, _e773) * 9f), 9f);
            if _e770 {
                phi_17672_ = true;
            } else {
                phi_17672_ = (0f >= _e769);
            }
            let _e781 = phi_17672_;
            let _e785 = cantus_render_shader_sd_capsule_box((_e767 - _e776), (select(_e769, 0f, _e781) * 9f), 9f);
            let _e786 = (10.5f * _e747);
            let _e788 = (_e754 - 1f);
            let _e789 = (_e788 != _e788);
            if _e789 {
                phi_17729_ = true;
            } else {
                phi_17729_ = (0f >= _e788);
            }
            let _e792 = phi_17729_;
            let _e797 = vec2<f32>(_e727, (_e749 + -5.4f));
            let _e799 = cantus_render_shader_sd_capsule_box((_e496 - _e797), (((select(_e788, 0f, _e792) * 18f) * _e747) * 0.5f), _e786);
            if _e789 {
                phi_17753_ = true;
            } else {
                phi_17753_ = (0f >= _e788);
            }
            let _e802 = phi_17753_;
            let _e808 = cantus_render_shader_sd_capsule_box((_e767 - _e797), (((select(_e788, 0f, _e802) * 18f) * _e747) * 0.5f), _e786);
            let _e812 = pill.member[_e497].primary_alpha;
            let _e815 = (0.5f + ((_e778 - _e526) * 0.05f));
            let _e817 = select(_e815, 0f, (_e815 < 0f));
            let _e819 = select(_e817, 1f, (_e817 > 1f));
            let _e829 = (_e526 + ((((_e778 + ((_e526 - _e778) * _e819)) - ((10f * _e819) * (1f - _e819))) - _e526) * _e812));
            let _e832 = (0.5f + ((_e785 - _e549) * 0.05f));
            let _e834 = select(_e832, 0f, (_e832 < 0f));
            let _e836 = select(_e834, 1f, (_e834 > 1f));
            let _e846 = (_e549 + ((((_e785 + ((_e549 - _e785) * _e836)) - ((10f * _e836) * (1f - _e836))) - _e549) * _e812));
            let _e848 = select(0f, 1f, (_e747 > 0f));
            let _e851 = (0.5f + ((_e799 - _e829) * 0.046296295f));
            let _e853 = select(_e851, 0f, (_e851 < 0f));
            let _e855 = select(_e853, 1f, (_e853 > 1f));
            let _e868 = (0.5f + ((_e808 - _e846) * 0.046296295f));
            let _e870 = select(_e868, 0f, (_e868 < 0f));
            let _e872 = select(_e870, 1f, (_e870 > 1f));
            let _e884 = (((_e846 + ((((_e808 + ((_e846 - _e808) * _e872)) - ((10.8f * _e872) * (1f - _e872))) - _e846) * _e848)) - 0.5f) * -1f);
            let _e886 = select(_e884, 0f, (_e884 < 0f));
            let _e888 = select(_e886, 1f, (_e886 > 1f));
            let _e895 = (((_e695 * ((_e888 * _e888) * (3f - (2f * _e888)))) + _e719) * 0.5f);
            let _e896 = ((_e829 + ((((_e799 + ((_e829 - _e799) * _e855)) - ((10.8f * _e855) * (1f - _e855))) - _e829) * _e848)) - _e895);
            let _e897 = fwidth(_e896);
            if (_e897 != _e897) {
                phi_17778_ = true;
            } else {
                phi_17778_ = (0.55f >= _e897);
            }
            let _e901 = phi_17778_;
            let _e902 = select(_e897, 0.55f, _e901);
            let _e906 = ((_e896 - _e902) / (-(_e902) - _e902));
            let _e908 = select(_e906, 0f, (_e906 < 0f));
            let _e910 = select(_e908, 1f, (_e908 > 1f));
            let _e914 = ((_e910 * _e910) * (3f - (2f * _e910)));
            let _e915 = (_e896 != _e896);
            if _e915 {
                phi_17793_ = true;
            } else {
                phi_17793_ = (0f >= _e896);
            }
            let _e918 = phi_17793_;
            let _e922 = (exp((select(_e896, 0f, _e918) * -0.3f)) * 0.16f);
            if (_e914 != _e914) {
                phi_17808_ = true;
            } else {
                phi_17808_ = (_e922 >= _e914);
            }
            let _e926 = phi_17808_;
            let _e927 = select(_e914, _e922, _e926);
            let _e931 = pill.member[_e497].visibility;
            if ((_e927 * _e931) <= 0.0009765625f) {
                discard;
            }
            if _e915 {
                phi_17825_ = true;
            } else {
                phi_17825_ = (0f <= _e896);
            }
            let _e936 = phi_17825_;
            let _e939 = (1f + (select(_e896, 0f, _e936) * 0.008333334f));
            let _e941 = select(_e939, 0f, (_e939 < 0f));
            let _e943 = select(_e941, 0.6f, (_e941 > 0.6f));
            let _e953 = ((_e724 - ((_e726 * _e943) * 0.08f)) - (_e721.y * 0.04f));
            let _e954 = (((_e723 - ((_e725 * _e943) * 0.08f)) - (_e697.x * 0.04f)) * _e512);
            let _e955 = (_e953 * _e516);
            let _e959 = pill.member[_e497].effects;
            let _e963 = frame.member[0u].time;
            let _e967 = pill.member[_e497].seed;
            let _e970 = ((_e959.tempo - 0.2f) * 2.5f);
            let _e972 = select(_e970, 0f, (_e970 < 0f));
            let _e981 = ((_e963 * ((0.12f + (_e959.energy * 0.25f)) + (select(_e972, 1f, (_e972 > 1f)) * 0.12f))) + _e967);
            let _e986 = ((sin(((_e963 * _e959.tempo) * 31.415928f)) * 0.5f) + 0.5f);
            let _e992 = (((_e986 * _e986) * _e959.danceability) * (0.025f + (_e959.energy * 0.055f)));
            let _e993 = (_e959.energy * 0.55f);
            let _e998 = ((_e993 + (_e959.danceability * 0.25f)) + (_e959.loudness * 0.2f));
            if _e915 {
                phi_17866_ = true;
            } else {
                phi_17866_ = (0f <= _e896);
            }
            let _e1001 = phi_17866_;
            let _e1004 = (1f + (select(_e896, 0f, _e1001) * 0.008333334f));
            let _e1006 = select(_e1004, 0f, (_e1004 < 0f));
            let _e1008 = select(_e1006, 1f, (_e1006 > 1f));
            let _e1019 = (_e967 - trunc(_e967));
            let _e1024 = ((_e512 / _e516) * ((0.5f + (_e1019 * 0.12f)) + (_e998 * 0.18f)));
            if (_e1024 != _e1024) {
                phi_17881_ = true;
            } else {
                phi_17881_ = (1.7f >= _e1024);
            }
            let _e1028 = phi_17881_;
            let _e1031 = select(0f, _e723, (_e723 > 0f));
            let _e1033 = select(0f, _e724, (_e724 > 0f));
            let _e1041 = (select(1f, _e1033, (_e1033 < 1f)) - (((((_e726 * _e1008) * _e1008) * 0.6f) + _e721.y) * 0.08f));
            let _e1042 = ((select(1f, _e1031, (_e1031 < 1f)) - (((((_e725 * _e1008) * _e1008) * 0.6f) + _e697.x) * 0.08f)) * select(_e1024, 1.7f, _e1028));
            let _e1053 = (_e981 * 0.8f);
            let _e1063 = ((0.14f + (_e998 * 0.2f)) + _e992);
            let _e1068 = (_e967 + 1.5707964f);
            let _e1073 = pill.member[_e497].colors[0u];
            let _e1075 = vec2<f32>((_e1042 + ((sin(((_e1041 * 4.32f) + _e981)) + cos(((_e1042 * 1.3f) - (_e981 * 0.7f)))) * _e1063)), ((_e1041 * 1.6f) + ((cos(((_e1042 * 2.3f) - _e1053)) + sin(((_e1041 * 2.72f) + (_e981 * 0.6f)))) * _e1063)));
            let _e1076 = cantus_render_track_plasma_field(_e1075, unpack4x8unorm(_e1073), 2.1f, 0.7f, _e981);
            let _e1081 = pill.member[_e497].colors[1u];
            let _e1084 = cantus_render_track_plasma_field(_e1075, unpack4x8unorm(_e1081), 0.6f, -2.4f, (_e1068 - _e1053));
            let _e1101 = pill.member[_e497].colors[2u];
            let _e1105 = cantus_render_track_plasma_field(_e1075, unpack4x8unorm(_e1101), -1.5f, 1.9f, ((_e981 * 0.65f) + 2f));
            let _e1118 = pill.member[_e497].colors[3u];
            let _e1119 = unpack4x8unorm(_e1118);
            let _e1122 = cantus_render_track_plasma_field(_e1075, _e1119, 2.4f, 1.6f, (_e1068 - (_e981 * 0.55f)));
            let _e1130 = (((_e1076.w + _e1084.w) + _e1105.w) + _e1122.w);
            let _e1131 = ((((_e1076.x + _e1084.x) + _e1105.x) + _e1122.x) / _e1130);
            let _e1132 = ((((_e1076.y + _e1084.y) + _e1105.y) + _e1122.y) / _e1130);
            let _e1133 = ((((_e1076.z + _e1084.z) + _e1105.z) + _e1122.z) / _e1130);
            let _e1138 = (((_e1131 * 0.2126f) + (_e1132 * 0.7152f)) + (_e1133 * 0.0722f));
            let _e1142 = frame.member[0u].playhead_x;
            let _e1143 = (_e1142 + 3f);
            let _e1147 = ((_e496.x - _e1143) / ((_e1142 - 3f) - _e1143));
            let _e1149 = select(_e1147, 0f, (_e1147 < 0f));
            let _e1151 = select(_e1149, 1f, (_e1149 > 1f));
            let _e1160 = pill.member[_e497].effects.valence;
            let _e1161 = (_e1160 * 0.4f);
            let _e1162 = (1.55f + _e1161);
            let _e1164 = (_e1138 * (-0.54999995f - _e1161));
            let _e1168 = (_e1164 + (_e1131 * _e1162));
            let _e1169 = (_e1164 + (_e1132 * _e1162));
            let _e1170 = (_e1164 + (_e1133 * _e1162));
            let _e1172 = select(0.035f, _e1168, (_e1168 > 0.035f));
            let _e1174 = select(0.035f, _e1169, (_e1169 > 0.035f));
            let _e1176 = select(0.035f, _e1170, (_e1170 > 0.035f));
            if (_e1138 != _e1138) {
                phi_17920_ = true;
            } else {
                phi_17920_ = (0.001f >= _e1138);
            }
            let _e1186 = phi_17920_;
            let _e1188 = (0.52f / select(_e1138, 0.001f, _e1186));
            if (_e1188 != _e1188) {
                phi_17935_ = true;
            } else {
                phi_17935_ = (1f <= _e1188);
            }
            let _e1192 = phi_17935_;
            let _e1193 = select(_e1188, 1f, _e1192);
            let _e1200 = ((0.96f + (_e1160 * 0.06f)) + (_e992 * 0.5f));
            let _e1205 = ((_e953 - 0.45f) * 1.8181818f);
            let _e1207 = select(_e1205, 0f, (_e1205 < 0f));
            let _e1209 = select(_e1207, 1f, (_e1207 > 1f));
            let _e1215 = (0.84f + (((_e1209 * _e1209) * (3f - (2f * _e1209))) * 0.1f));
            let _e1220 = (1f - (0.4f * ((_e1151 * _e1151) * (3f - (2f * _e1151)))));
            let _e1240 = (8f - _e959.acousticness);
            let _e1244 = (_e963 * (0.35f + _e993));
            let _e1247 = ((_e517 / _e1240) + (_e1244 * (0.16f + (_e1019 * 0.08f))));
            let _e1248 = ((_e518 / _e1240) + (_e1244 * (0.055f + (sin((_e967 * 0.7f)) * 0.025f))));
            let _e1249 = floor(_e1247);
            let _e1250 = floor(_e1248);
            let _e1259 = bitcast<u32>(select(0i, select(select(i32(_e1250), i32(-2147483648), (_e1250 < -2147483600f)), 2147483647i, (_e1250 > 2147483500f)), (_e1250 == _e1250)));
            let _e1267 = bitcast<u32>(select(0i, select(select(i32(_e1249), i32(-2147483648), (_e1249 < -2147483600f)), 2147483647i, (_e1249 > 2147483500f)), (_e1249 == _e1249)));
            let _e1269 = (bitcast<u32>((_e967 + 2.71f)) * 2654435761u);
            let _e1275 = (((_e1267 ^ _e1269) * 1664525u) + 1013904223u);
            let _e1277 = ((((_e1259 ^ _e1269) * 1664525u) + 1013904223u) + (_e1275 * 1664525u));
            let _e1279 = (_e1275 + (_e1277 * 1664525u));
            let _e1287 = ((_e1277 ^ (_e1277 >> bitcast<u32>(16i))) + ((_e1279 ^ (_e1279 >> bitcast<u32>(16i))) * 1664525u));
            let _e1291 = f32((_e1287 ^ (_e1287 >> bitcast<u32>(16i))));
            let _e1292 = (_e1291 * 0.0000000016600825f);
            let _e1306 = (_e959.acousticness * 0.09f);
            let _e1309 = (bitcast<u32>(_e967) * 2654435761u);
            let _e1315 = (((_e1259 ^ _e1309) * 1664525u) + 1013904223u);
            let _e1317 = ((((_e1267 ^ _e1309) * 1664525u) + 1013904223u) + (_e1315 * 1664525u));
            let _e1319 = (_e1315 + (_e1317 * 1664525u));
            let _e1327 = ((_e1317 ^ (_e1317 >> bitcast<u32>(16i))) + ((_e1319 ^ (_e1319 >> bitcast<u32>(16i))) * 1664525u));
            let _e1335 = (((f32((_e1327 ^ (_e1327 >> bitcast<u32>(16i)))) * 0.00000000023283064f) - (0.985f - _e1306)) / (_e1306 + 0.014999986f));
            let _e1337 = select(_e1335, 0f, (_e1335 < 0f));
            let _e1339 = select(_e1337, 1f, (_e1337 > 1f));
            let _e1348 = (((_e1247 - _e1249) - 0.5f) - ((_e1291 * 0.00000000013038516f) - 0.28f));
            let _e1349 = (((_e1248 - _e1250) - 0.5f) - (((_e1292 - trunc(_e1292)) * 0.56f) - 0.28f));
            let _e1355 = ((sqrt(((_e1348 * _e1348) + (_e1349 * _e1349))) - 0.06f) * 4.5454545f);
            let _e1357 = select(_e1355, 0f, (_e1355 < 0f));
            let _e1359 = select(_e1357, 1f, (_e1357 > 1f));
            let _e1372 = (((((_e1339 * _e1339) * (3f - (2f * _e1339))) * (1f - ((_e1359 * _e1359) * (3f - (2f * _e1359))))) * ((sin(((_e963 * ((0.7f + (_e1291 * 0.00000000020954757f)) + (_e959.energy * 0.8f))) + (_e1291 * 0.0000000014629181f))) * 0.5f) + 0.5f)) * (0.12f + (_e959.acousticness * 0.48f)));
            let _e1376 = (((((select(0.92f, _e1172, (_e1172 < 0.92f)) * _e1193) * _e1200) * _e1215) * _e1220) + (((_e1119.x * 0.75f) + 0.25f) * _e1372));
            let _e1377 = (((((select(0.92f, _e1174, (_e1174 < 0.92f)) * _e1193) * _e1200) * _e1215) * _e1220) + (((_e1119.y * 0.75f) + 0.25f) * _e1372));
            let _e1378 = (((((select(0.92f, _e1176, (_e1176 < 0.92f)) * _e1193) * _e1200) * _e1215) * _e1220) + (((_e1119.z * 0.75f) + 0.25f) * _e1372));
            let _e1385 = (_e517 / _e516);
            if (_e959.instrumentalness <= 0.00390625f) {
                phi_18127_ = 0f;
            } else {
                let _e1390 = (_e963 * (0.5f + (_e959.energy * 0.35f)));
                let _e1398 = (sin(((_e724 * 1.9f) + _e1390)) * 0.35f);
                let _e1399 = (sin(((_e1385 * 1.5f) - (_e1390 * 0.8f))) * 0.35f);
                let _e1402 = ((_e1390 * 0.05f) + _e967);
                let _e1403 = ((_e1390 * -0.04f) + _e967);
                let _e1411 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1385 * 0.7f) + _e1398) + _e1402), (((_e724 * 0.7f) + _e1399) + _e1403)));
                let _e1414 = (1f - (abs(_e1411) * 2f));
                if (_e1414 != _e1414) {
                    phi_18150_ = true;
                } else {
                    phi_18150_ = (0f >= _e1414);
                }
                let _e1418 = phi_18150_;
                let _e1419 = select(_e1414, 0f, _e1418);
                let _e1421 = ((_e1419 * _e1419) * _e1419);
                let _e1431 = cantus_render_shader_simplex_noise(vec2<f32>((((_e1385 * 1.1f) - _e1398) - (_e1402 * 0.8f)), (((_e724 * 1.1f) - _e1399) - (_e1403 * 0.8f))));
                let _e1434 = (1f - (abs(_e1431) * 2f));
                if (_e1434 != _e1434) {
                    phi_18177_ = true;
                } else {
                    phi_18177_ = (0f >= _e1434);
                }
                let _e1438 = phi_18177_;
                let _e1439 = select(_e1434, 0f, _e1438);
                let _e1441 = ((_e1439 * _e1439) * _e1439);
                if (_e1421 != _e1421) {
                    phi_18192_ = true;
                } else {
                    phi_18192_ = (_e1441 >= _e1421);
                }
                let _e1445 = phi_18192_;
                phi_18127_ = ((select(_e1421, _e1441, _e1445) * _e959.instrumentalness) * 0.06f);
            }
            let _e1450 = phi_18127_;
            let _e1454 = (_e1376 + (((_e1376 * 0.25f) + 0.75f) * _e1450));
            let _e1455 = (_e1377 + (((_e1377 * 0.25f) + 0.75f) * _e1450));
            let _e1456 = (_e1378 + (((_e1378 * 0.25f) + 0.75f) * _e1450));
            let _e1457 = vec3<f32>(_e1454, _e1455, _e1456);
            let _e1458 = (_e523 + _e520);
            let _e1462 = pill.member[_e497].image_index;
            if (_e1462 >= 0i) {
                let _e1464 = (_e517 - _e1458);
                let _e1465 = abs(_e1464);
                let _e1466 = abs(_e522);
                if (select(_e1466, _e1465, (_e1465 > _e1466)) < _e516) {
                    let _e1470 = (_e520 + _e895);
                    let _e1476 = (_e1470 * 2f);
                    let _e1482 = vec3<f32>(((_e1464 / _e1476) + 0.5f), ((_e522 / _e1476) + 0.5f), f32(_e1462));
                    let _e1488 = textureSample(images, sampler_, vec2<f32>(_e1482.x, _e1482.y), i32(_e1482.z));
                    let _e1490 = (((sqrt(((_e1464 * _e1464) + (_e522 * _e522))) - _e1470) - -4f) * 0.25f);
                    let _e1492 = select(_e1490, 0f, (_e1490 < 0f));
                    let _e1494 = select(_e1492, 1f, (_e1492 > 1f));
                    let _e1501 = ((_e549 - 0.5f) * -1f);
                    let _e1503 = select(_e1501, 0f, (_e1501 < 0f));
                    let _e1505 = select(_e1503, 1f, (_e1503 > 1f));
                    let _e1514 = ((_e526 - (((_e695 * ((_e1505 * _e1505) * (3f - (2f * _e1505)))) + _e719) * 0.5f)) - -0.5f);
                    let _e1516 = select(_e1514, 0f, (_e1514 < 0f));
                    let _e1518 = select(_e1516, 1f, (_e1516 > 1f));
                    let _e1529 = (((1f - ((_e1494 * _e1494) * (3f - (2f * _e1494)))) * (1f - ((_e1518 * _e1518) * (3f - (2f * _e1518))))) * _e1488.w);
                    let _e1530 = (1f - _e1529);
                    phi_2143_ = vec3<f32>(((_e1454 * _e1530) + (_e1488.x * _e1529)), ((_e1455 * _e1530) + (_e1488.y * _e1529)), ((_e1456 * _e1530) + (_e1488.z * _e1529)));
                } else {
                    phi_2143_ = _e1457;
                }
                let _e1542 = phi_2143_;
                phi_2144_ = _e1542;
            } else {
                phi_2144_ = _e1457;
            }
            let _e1544 = phi_2144_;
            let _e1555 = ((_e896 - 5f) * -0.125f);
            let _e1557 = select(_e1555, 0f, (_e1555 < 0f));
            let _e1559 = select(_e1557, 1f, (_e1557 > 1f));
            let _e1564 = (((_e1559 * _e1559) * (3f - (2f * _e1559))) * 0.14f);
            let _e1568 = (_e1544.x + (((_e1544.x * 0.68f) + 0.32f) * _e1564));
            let _e1569 = (_e1544.y + (((_e1544.y * 0.68f) + 0.32f) * _e1564));
            let _e1570 = (_e1544.z + (((_e1544.z * 0.68f) + 0.32f) * _e1564));
            let _e1578 = local_5;
            let _e1579 = (1f - _e1578);
            let _e1584 = local_6;
            let _e1587 = local_7;
            let _e1590 = local_8;
            let _e1598 = vec4<f32>((((_e1568 * _e1579) + (((_e1568 * 1.5f) + 0.1f) * _e1584)) * _e914), (((_e1569 * _e1579) + (((_e1569 * 1.5f) + 0.1f) * _e1587)) * _e914), (((_e1570 * _e1579) + (((_e1570 * 1.5f) + 0.1f) * _e1590)) * _e914), _e927);
            if _e734 {
                if (_e812 > 0f) {
                    phi_2228_ = _e1598;
                    phi_2231_ = 0u;
                    loop {
                        let _e1601 = phi_2228_;
                        let _e1603 = phi_2231_;
                        local_21 = _e1601;
                        let _e1604 = (_e1603 < 5u);
                        if _e1604 {
                            let _e1605 = f32(_e1603);
                            if _e770 {
                                phi_18231_ = true;
                            } else {
                                phi_18231_ = (0f >= _e769);
                            }
                            let _e1608 = phi_18231_;
                            let _e1613 = (_e727 + ((_e1605 - (select(_e769, 0f, _e1608) * 0.5f)) * 18f));
                            let _e1614 = (_e728 + 5f);
                            let _e1615 = (_e496.x - _e1613);
                            let _e1616 = (_e496.y - _e1614);
                            let _e1617 = abs(_e1615);
                            let _e1618 = abs(_e1616);
                            if (select(_e1618, _e1617, (_e1617 > _e1618)) < 38.88f) {
                                let _e1625 = ((f32(_e733) - (_e1605 * 2f)) * 0.5f);
                                let _e1627 = select(_e1625, 0f, (_e1625 < 0f));
                                let _e1630 = (_e1613 - _e761);
                                let _e1631 = (_e1614 - _e766);
                                let _e1637 = ((sqrt(((_e1630 * _e1630) + (_e1631 * _e1631))) - 11.3f) * -1f);
                                let _e1639 = select(_e1637, 0f, (_e1637 < 0f));
                                let _e1641 = select(_e1639, 1f, (_e1639 > 1f));
                                let _e1647 = select(_e530, 0f, (_e530 < 0f));
                                let _e1650 = (((_e1641 * _e1641) * (3f - (2f * _e1641))) * select(_e1647, 1f, (_e1647 > 1f)));
                                let _e1652 = (1.05f + (0.63f * _e1650));
                                let _e1653 = (_e1630 * _e1650);
                                let _e1655 = (_e1615 - (_e1653 * 0.5f));
                                let _e1656 = (_e1653 * -0.005f);
                                let _e1657 = sin(_e1656);
                                let _e1658 = cos(_e1656);
                                let _e1661 = ((_e1658 * _e1655) - (_e1657 * _e1616));
                                let _e1664 = ((_e1657 * _e1655) + (_e1658 * _e1616));
                                let _e1668 = (_e1652 * 5.4f);
                                let _e1669 = abs(_e1661);
                                let _e1673 = ((0.809017f * _e1669) + (_e1664 * 0.58778524f));
                                if (_e1673 != _e1673) {
                                    phi_18266_ = true;
                                } else {
                                    phi_18266_ = (0f >= _e1673);
                                }
                                let _e1677 = phi_18266_;
                                let _e1678 = select(_e1673, 0f, _e1677);
                                let _e1681 = (_e1669 - (_e1678 * 1.618034f));
                                let _e1682 = (-(_e1664) - (_e1678 * -1.1755705f));
                                let _e1685 = ((-0.809017f * _e1681) + (-0.58778524f * _e1682));
                                if (_e1685 != _e1685) {
                                    phi_18281_ = true;
                                } else {
                                    phi_18281_ = (0f >= _e1685);
                                }
                                let _e1689 = phi_18281_;
                                let _e1690 = select(_e1685, 0f, _e1689);
                                let _e1695 = abs((_e1681 - (_e1690 * -1.618034f)));
                                let _e1696 = ((_e1682 - (_e1690 * -1.1755705f)) - _e1668);
                                let _e1697 = (_e1652 * 2.031386f);
                                let _e1699 = ((_e1652 * 2.7959628f) - _e1668);
                                let _e1706 = (((_e1695 * _e1697) + (_e1696 * _e1699)) / ((_e1697 * _e1697) + (_e1699 * _e1699)));
                                let _e1708 = select(_e1706, 0f, (_e1706 < 0f));
                                let _e1710 = select(_e1708, 1f, (_e1708 > 1f));
                                let _e1716 = (_e1695 - (_e1697 * _e1710));
                                let _e1717 = (_e1696 - (_e1699 * _e1710));
                                let _e1726 = ((sqrt(((_e1716 * _e1716) + (_e1717 * _e1717))) * select(1f, -1f, (((_e1696 * _e1697) - (_e1695 * _e1699)) < 0f))) - (_e1652 * 1.08f));
                                let _e1727 = (((_e1661 / (_e1652 * 21.6f)) + 0.5f) - select(_e1627, 1f, (_e1627 > 1f)));
                                let _e1728 = fwidth(_e1727);
                                let _e1730 = ((_e1727 / _e1728) + 0.5f);
                                let _e1732 = select(_e1730, 0f, (_e1730 < 0f));
                                let _e1734 = select(_e1732, 1f, (_e1732 > 1f));
                                let _e1735 = (1f - _e1734);
                                let _e1738 = (0.33f * _e1734);
                                let _e1742 = (0.5f - _e1726);
                                let _e1744 = select(_e1742, 0f, (_e1742 < 0f));
                                let _e1746 = select(_e1744, 1f, (_e1744 > 1f));
                                if (_e1726 != _e1726) {
                                    phi_18296_ = true;
                                } else {
                                    phi_18296_ = (0f >= _e1726);
                                }
                                let _e1750 = phi_18296_;
                                let _e1753 = exp((select(_e1726, 0f, _e1750) * -0.5f));
                                let _e1754 = (_e1726 * -0.2f);
                                let _e1756 = select(_e1754, 0f, (_e1754 < 0f));
                                let _e1758 = select(_e1756, 1f, (_e1756 > 1f));
                                let _e1763 = (1f - ((_e1758 * _e1758) * (3f - (2f * _e1758))));
                                let _e1765 = ((_e1763 * _e1763) * 0.045f);
                                let _e1776 = ((_e1753 * _e1753) * 0.2f);
                                if (_e1746 != _e1746) {
                                    phi_18311_ = true;
                                } else {
                                    phi_18311_ = (_e1776 >= _e1746);
                                }
                                let _e1780 = phi_18311_;
                                let _e1782 = (select(_e1746, _e1776, _e1780) * _e812);
                                let _e1783 = (1f - _e1782);
                                phi_2528_ = vec4<f32>(((_e1601.x * _e1783) + ((((_e1735 + _e1738) + _e1765) * _e1746) * _e812)), ((_e1601.y * _e1783) + (((((0.85f * _e1735) + _e1738) + _e1765) * _e1746) * _e812)), ((_e1601.z * _e1783) + (((((0.2f * _e1735) + _e1738) + _e1765) * _e1746) * _e812)), ((_e1601.w * _e1783) + _e1782));
                            } else {
                                phi_2528_ = _e1601;
                            }
                            let _e1798 = phi_2528_;
                            phi_2229_ = _e1798;
                            phi_2232_ = (_e1603 + 1u);
                        } else {
                            phi_2229_ = vec4<f32>();
                            phi_2232_ = u32();
                        }
                        let _e1801 = phi_2229_;
                        let _e1803 = phi_2232_;
                        continue;
                        continuing {
                            phi_2228_ = _e1801;
                            phi_2231_ = _e1803;
                            break if !(_e1604);
                        }
                    }
                    if _e665 {
                        break;
                    }
                    let _e2452 = local_21;
                    phi_2530_ = _e2452;
                } else {
                    phi_2530_ = _e1598;
                }
                let _e1806 = phi_2530_;
                phi_2531_ = _e1806;
            } else {
                phi_2531_ = _e1598;
            }
            let _e1808 = phi_2531_;
            let _e1809 = (_e739 + _e753);
            phi_2543_ = _e1808;
            phi_2546_ = 0u;
            loop {
                let _e1813 = phi_2543_;
                let _e1815 = phi_2546_;
                local_17 = _e1813;
                local_18 = _e1813;
                local_19 = _e1813;
                local_20 = _e1813;
                let _e1816 = (_e1815 < select(_e1809, 8u, (8u < _e1809)));
                if _e1816 {
                    if (_e1815 < 8u) {
                    } else {
                        phi_21749_ = true;
                        break;
                    }
                    let _e1822 = pill.member[_e497].playlist_images[_e1815];
                    if (_e1822 >= 0i) {
                        let _e1824 = (_e1815 < _e739);
                        if _e1824 {
                            phi_2586_ = render_RipplePulse(vec2<f32>(_e727, _e729), _e741, 1f);
                            phi_2588_ = (f32(_e1815) + _e735);
                        } else {
                            phi_2586_ = render_RipplePulse(vec2<f32>(_e727, _e749), _e754, _e747);
                            phi_2588_ = f32((_e1815 - _e739));
                        }
                        let _e1830 = phi_2586_;
                        let _e1832 = phi_2588_;
                        let _e1833 = select(_e747, _e812, _e1824);
                        let _e1835 = (_e1830.start_time - 1f);
                        if (_e1835 != _e1835) {
                            phi_18341_ = true;
                        } else {
                            phi_18341_ = (0f >= _e1835);
                        }
                        let _e1839 = phi_18341_;
                        let _e1848 = (_e1830.origin.x + (((_e1832 - (select(_e1835, 0f, _e1839) * 0.5f)) * 18f) * _e1830.strength));
                        let _e1851 = (_e1830.origin.y + 2f);
                        if (_e1833 > 0f) {
                            let _e1853 = (_e496.x - _e1848);
                            let _e1854 = (_e496.y - _e1851);
                            let _e1855 = abs(_e1853);
                            let _e1856 = abs(_e1854);
                            if (select(_e1856, _e1855, (_e1855 > _e1856)) < 38.88f) {
                                let _e1860 = (_e1848 - _e761);
                                let _e1861 = (_e1851 - _e766);
                                let _e1865 = sqrt(((_e1860 * _e1860) + (_e1861 * _e1861)));
                                let _e1867 = ((_e1865 - 11.3f) * -1f);
                                let _e1869 = select(_e1867, 0f, (_e1867 < 0f));
                                let _e1871 = select(_e1869, 1f, (_e1869 > 1f));
                                let _e1877 = select(_e530, 0f, (_e530 < 0f));
                                let _e1880 = (((_e1871 * _e1871) * (3f - (2f * _e1871))) * select(_e1877, 1f, (_e1877 > 1f)));
                                let _e1882 = (1.05f + (0.63f * _e1880));
                                let _e1883 = (_e1860 * _e1880);
                                let _e1885 = (_e1853 - (_e1883 * 0.5f));
                                let _e1886 = (_e1883 * -0.005f);
                                let _e1887 = sin(_e1886);
                                let _e1888 = cos(_e1886);
                                let _e1891 = ((_e1888 * _e1885) - (_e1887 * _e1854));
                                let _e1894 = ((_e1887 * _e1885) + (_e1888 * _e1854));
                                let _e1895 = (_e1882 * 21.6f);
                                if _e1824 {
                                    phi_2712_ = true;
                                } else {
                                    if _e531 {
                                        phi_2707_ = select(true, false, (_e1865 <= 10.8f));
                                    } else {
                                        phi_2707_ = true;
                                    }
                                    let _e1903 = phi_2707_;
                                    phi_2712_ = select(true, false, _e1903);
                                }
                                let _e1906 = phi_2712_;
                                let _e1907 = select(0.2f, 0f, _e1906);
                                let _e1910 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1891, _e1894), 0f, (_e1882 * 6.4800005f));
                                if (_e1910 <= 7f) {
                                    let _e1913 = vec3<f32>(((_e1891 / _e1895) + 0.5f), ((_e1894 / _e1895) + 0.5f), f32(_e1822));
                                    let _e1919 = textureSample(images, sampler_, vec2<f32>(_e1913.x, _e1913.y), i32(_e1913.z));
                                    let _e1923 = (1f - _e1907);
                                    let _e1927 = (0.24f * _e1907);
                                    let _e1931 = (0.5f - _e1910);
                                    let _e1933 = select(_e1931, 0f, (_e1931 < 0f));
                                    let _e1935 = select(_e1933, 1f, (_e1933 > 1f));
                                    if (_e1910 != _e1910) {
                                        phi_18378_ = true;
                                    } else {
                                        phi_18378_ = (0f >= _e1910);
                                    }
                                    let _e1939 = phi_18378_;
                                    let _e1942 = exp((select(_e1910, 0f, _e1939) * -0.5f));
                                    let _e1943 = (_e1910 * -0.2f);
                                    let _e1945 = select(_e1943, 0f, (_e1943 < 0f));
                                    let _e1947 = select(_e1945, 1f, (_e1945 > 1f));
                                    let _e1952 = (1f - ((_e1947 * _e1947) * (3f - (2f * _e1947))));
                                    let _e1954 = ((_e1952 * _e1952) * 0.045f);
                                    let _e1965 = ((_e1942 * _e1942) * 0.2f);
                                    if (_e1935 != _e1935) {
                                        phi_18393_ = true;
                                    } else {
                                        phi_18393_ = (_e1965 >= _e1935);
                                    }
                                    let _e1969 = phi_18393_;
                                    let _e1971 = (select(_e1935, _e1965, _e1969) * _e1833);
                                    let _e1972 = (1f - _e1971);
                                    phi_2814_ = vec4<f32>(((_e1813.x * _e1972) + (((((_e1919.x * _e1923) + _e1927) + _e1954) * _e1935) * _e1833)), ((_e1813.y * _e1972) + (((((_e1919.y * _e1923) + _e1927) + _e1954) * _e1935) * _e1833)), ((_e1813.z * _e1972) + (((((_e1919.z * _e1923) + _e1927) + _e1954) * _e1935) * _e1833)), ((_e1813.w * _e1972) + _e1971));
                                } else {
                                    phi_2814_ = _e1813;
                                }
                                let _e1987 = phi_2814_;
                                phi_2815_ = _e1987;
                            } else {
                                phi_2815_ = _e1813;
                            }
                            let _e1989 = phi_2815_;
                            phi_2816_ = _e1989;
                        } else {
                            phi_2816_ = _e1813;
                        }
                        let _e1991 = phi_2816_;
                        phi_2817_ = _e1991;
                    } else {
                        phi_2817_ = _e1813;
                    }
                    let _e1993 = phi_2817_;
                    phi_2544_ = _e1993;
                    phi_2547_ = (_e1815 + 1u);
                } else {
                    phi_2544_ = vec4<f32>();
                    phi_2547_ = u32();
                }
                let _e1996 = phi_2544_;
                let _e1998 = phi_2547_;
                continue;
                continuing {
                    phi_2543_ = _e1996;
                    phi_2546_ = _e1998;
                    phi_21749_ = _e665;
                    break if !(_e1816);
                }
            }
            let _e2001 = phi_21749_;
            if _e2001 {
                break;
            }
            let _e2006 = pill.member[_e497].lines[0u];
            let _e2008 = (1f / _e2006.size);
            let _e2015 = ((_e954 - _e2006.origin.x) * _e2008);
            phi_2833_ = 0u;
            phi_2836_ = _e2006.count;
            loop {
                let _e2020 = phi_2833_;
                let _e2022 = phi_2836_;
                local_9 = _e2020;
                let _e2023 = (_e2020 < _e2022);
                if _e2023 {
                    let _e2026 = (_e2020 + ((_e2022 - _e2020) / 2u));
                    let _e2031 = placed_glyphs.member[(_e2006.first + _e2026)].x;
                    let _e2032 = (_e2031 <= _e2015);
                    if _e2032 {
                        phi_2864_ = (_e2026 + 1u);
                    } else {
                        phi_2864_ = _e2020;
                    }
                    let _e2035 = phi_2864_;
                    phi_2834_ = _e2035;
                    phi_2837_ = select(_e2026, _e2022, _e2032);
                } else {
                    phi_2834_ = u32();
                    phi_2837_ = u32();
                }
                let _e2038 = phi_2834_;
                let _e2040 = phi_2837_;
                continue;
                continuing {
                    phi_2833_ = _e2038;
                    phi_2836_ = _e2040;
                    break if !(_e2023);
                }
            }
            let _e2042 = (3.5f / _e2006.size);
            let _e2044 = local_9;
            let _e2045 = (_e2044 + 1u);
            phi_2876_ = select(_e2045, _e2006.count, (_e2006.count < _e2045));
            phi_2879_ = -1000000f;
            loop {
                let _e2049 = phi_2876_;
                let _e2051 = phi_2879_;
                local_12 = _e2051;
                if (_e2049 > 0u) {
                    let _e2053 = (_e2049 - 1u);
                    let _e2054 = (_e2006.first + _e2053);
                    let _e2058 = placed_glyphs.member[_e2054].x;
                    let _e2062 = placed_glyphs.member[_e2054].glyph;
                    let _e2067 = glyphs.member[_e2062].min[0u];
                    let _e2072 = glyphs.member[_e2062].min[1u];
                    let _e2077 = glyphs.member[_e2062].max[0u];
                    let _e2082 = glyphs.member[_e2062].max[1u];
                    let _e2086 = glyphs.member[_e2062].start;
                    let _e2090 = glyphs.member[_e2062].count;
                    let _e2091 = (_e2015 - _e2058);
                    let _e2092 = -(((_e955 - _e2006.origin.y) * _e2008));
                    let _e2093 = (_e2077 + _e2042);
                    let _e2094 = (_e2091 > _e2093);
                    if _e2094 {
                        phi_3009_ = f32();
                    } else {
                        if (_e2091 >= (_e2067 - _e2042)) {
                            if (_e2092 >= (_e2072 - _e2042)) {
                                if (_e2091 <= _e2093) {
                                    if (_e2092 <= (_e2082 + _e2042)) {
                                        phi_2959_ = 340282350000000000000000000000000000000f;
                                        phi_2962_ = 0u;
                                        phi_2964_ = 0i;
                                        loop {
                                            let _e2104 = phi_2959_;
                                            let _e2106 = phi_2962_;
                                            let _e2108 = phi_2964_;
                                            local_10 = _e2104;
                                            local_11 = _e2108;
                                            let _e2109 = (_e2106 < _e2090);
                                            if _e2109 {
                                                let _e2113 = edges.member[(_e2086 + _e2106)];
                                                let _e2115 = cantus_render_text_edge_distance(_e2113, _e2006.weight, vec2<f32>(_e2091, _e2092), _e2104);
                                                phi_2960_ = _e2115.member;
                                                phi_2963_ = (_e2106 + 1u);
                                                phi_2965_ = (_e2108 + _e2115.member_1);
                                            } else {
                                                phi_2960_ = f32();
                                                phi_2963_ = u32();
                                                phi_2965_ = i32();
                                            }
                                            let _e2121 = phi_2960_;
                                            let _e2123 = phi_2963_;
                                            let _e2125 = phi_2965_;
                                            continue;
                                            continuing {
                                                phi_2959_ = _e2121;
                                                phi_2962_ = _e2123;
                                                phi_2964_ = _e2125;
                                                break if !(_e2109);
                                            }
                                        }
                                        let _e2128 = local_10;
                                        let _e2130 = ((_e2128 * _e2006.size) * _e2006.size);
                                        if (_e2130 >= 12.25f) {
                                            phi_2997_ = 3.5f;
                                        } else {
                                            phi_2997_ = sqrt(_e2130);
                                        }
                                        let _e2134 = phi_2997_;
                                        let _e2136 = local_11;
                                        let _e2139 = (_e2134 * select(1f, -1f, (_e2136 == 0i)));
                                        if (_e2051 != _e2051) {
                                            phi_18408_ = true;
                                        } else {
                                            phi_18408_ = (_e2139 >= _e2051);
                                        }
                                        let _e2143 = phi_18408_;
                                        phi_3005_ = select(_e2051, _e2139, _e2143);
                                    } else {
                                        phi_3005_ = _e2051;
                                    }
                                    let _e2146 = phi_3005_;
                                    phi_3006_ = _e2146;
                                } else {
                                    phi_3006_ = _e2051;
                                }
                                let _e2148 = phi_3006_;
                                phi_3007_ = _e2148;
                            } else {
                                phi_3007_ = _e2051;
                            }
                            let _e2150 = phi_3007_;
                            phi_3008_ = _e2150;
                        } else {
                            phi_3008_ = _e2051;
                        }
                        let _e2152 = phi_3008_;
                        phi_3009_ = _e2152;
                    }
                    let _e2154 = phi_3009_;
                    phi_2877_ = _e2053;
                    phi_2880_ = _e2154;
                    phi_3011_ = select(true, false, _e2094);
                } else {
                    phi_2877_ = u32();
                    phi_2880_ = f32();
                    phi_3011_ = false;
                }
                let _e2157 = phi_2877_;
                let _e2159 = phi_2880_;
                let _e2161 = phi_3011_;
                continue;
                continuing {
                    phi_2876_ = _e2157;
                    phi_2879_ = _e2159;
                    break if !(_e2161);
                }
            }
            let _e2164 = local_12;
            let _e2166 = ((_e2164 * 1.25f) + 0.5f);
            let _e2168 = select(_e2166, 0f, (_e2166 < 0f));
            let _e2170 = select(_e2168, 1f, (_e2168 > 1f));
            let _e2174 = ((_e2170 * _e2170) * (3f - (2f * _e2170)));
            let _e2179 = pill.member[_e497].lines[1u];
            let _e2181 = (1f / _e2179.size);
            let _e2188 = ((_e954 - _e2179.origin.x) * _e2181);
            phi_3050_ = 0u;
            phi_3053_ = _e2179.count;
            loop {
                let _e2193 = phi_3050_;
                let _e2195 = phi_3053_;
                local_13 = _e2193;
                let _e2196 = (_e2193 < _e2195);
                if _e2196 {
                    let _e2199 = (_e2193 + ((_e2195 - _e2193) / 2u));
                    let _e2204 = placed_glyphs.member[(_e2179.first + _e2199)].x;
                    let _e2205 = (_e2204 <= _e2188);
                    if _e2205 {
                        phi_3081_ = (_e2199 + 1u);
                    } else {
                        phi_3081_ = _e2193;
                    }
                    let _e2208 = phi_3081_;
                    phi_3051_ = _e2208;
                    phi_3054_ = select(_e2199, _e2195, _e2205);
                } else {
                    phi_3051_ = u32();
                    phi_3054_ = u32();
                }
                let _e2211 = phi_3051_;
                let _e2213 = phi_3054_;
                continue;
                continuing {
                    phi_3050_ = _e2211;
                    phi_3053_ = _e2213;
                    break if !(_e2196);
                }
            }
            let _e2215 = (3.5f / _e2179.size);
            let _e2217 = local_13;
            let _e2218 = (_e2217 + 1u);
            phi_3093_ = select(_e2218, _e2179.count, (_e2179.count < _e2218));
            phi_3096_ = -1000000f;
            loop {
                let _e2222 = phi_3093_;
                let _e2224 = phi_3096_;
                local_16 = _e2224;
                if (_e2222 > 0u) {
                    let _e2226 = (_e2222 - 1u);
                    let _e2227 = (_e2179.first + _e2226);
                    let _e2231 = placed_glyphs.member[_e2227].x;
                    let _e2235 = placed_glyphs.member[_e2227].glyph;
                    let _e2240 = glyphs.member[_e2235].min[0u];
                    let _e2245 = glyphs.member[_e2235].min[1u];
                    let _e2250 = glyphs.member[_e2235].max[0u];
                    let _e2255 = glyphs.member[_e2235].max[1u];
                    let _e2259 = glyphs.member[_e2235].start;
                    let _e2263 = glyphs.member[_e2235].count;
                    let _e2264 = (_e2188 - _e2231);
                    let _e2265 = -(((_e955 - _e2179.origin.y) * _e2181));
                    let _e2266 = (_e2250 + _e2215);
                    let _e2267 = (_e2264 > _e2266);
                    if _e2267 {
                        phi_3226_ = f32();
                    } else {
                        if (_e2264 >= (_e2240 - _e2215)) {
                            if (_e2265 >= (_e2245 - _e2215)) {
                                if (_e2264 <= _e2266) {
                                    if (_e2265 <= (_e2255 + _e2215)) {
                                        phi_3176_ = 340282350000000000000000000000000000000f;
                                        phi_3179_ = 0u;
                                        phi_3181_ = 0i;
                                        loop {
                                            let _e2277 = phi_3176_;
                                            let _e2279 = phi_3179_;
                                            let _e2281 = phi_3181_;
                                            local_14 = _e2277;
                                            local_15 = _e2281;
                                            let _e2282 = (_e2279 < _e2263);
                                            if _e2282 {
                                                let _e2286 = edges.member[(_e2259 + _e2279)];
                                                let _e2288 = cantus_render_text_edge_distance(_e2286, _e2179.weight, vec2<f32>(_e2264, _e2265), _e2277);
                                                phi_3177_ = _e2288.member;
                                                phi_3180_ = (_e2279 + 1u);
                                                phi_3182_ = (_e2281 + _e2288.member_1);
                                            } else {
                                                phi_3177_ = f32();
                                                phi_3180_ = u32();
                                                phi_3182_ = i32();
                                            }
                                            let _e2294 = phi_3177_;
                                            let _e2296 = phi_3180_;
                                            let _e2298 = phi_3182_;
                                            continue;
                                            continuing {
                                                phi_3176_ = _e2294;
                                                phi_3179_ = _e2296;
                                                phi_3181_ = _e2298;
                                                break if !(_e2282);
                                            }
                                        }
                                        let _e2301 = local_14;
                                        let _e2303 = ((_e2301 * _e2179.size) * _e2179.size);
                                        if (_e2303 >= 12.25f) {
                                            phi_3214_ = 3.5f;
                                        } else {
                                            phi_3214_ = sqrt(_e2303);
                                        }
                                        let _e2307 = phi_3214_;
                                        let _e2309 = local_15;
                                        let _e2312 = (_e2307 * select(1f, -1f, (_e2309 == 0i)));
                                        if (_e2224 != _e2224) {
                                            phi_18423_ = true;
                                        } else {
                                            phi_18423_ = (_e2312 >= _e2224);
                                        }
                                        let _e2316 = phi_18423_;
                                        phi_3222_ = select(_e2224, _e2312, _e2316);
                                    } else {
                                        phi_3222_ = _e2224;
                                    }
                                    let _e2319 = phi_3222_;
                                    phi_3223_ = _e2319;
                                } else {
                                    phi_3223_ = _e2224;
                                }
                                let _e2321 = phi_3223_;
                                phi_3224_ = _e2321;
                            } else {
                                phi_3224_ = _e2224;
                            }
                            let _e2323 = phi_3224_;
                            phi_3225_ = _e2323;
                        } else {
                            phi_3225_ = _e2224;
                        }
                        let _e2325 = phi_3225_;
                        phi_3226_ = _e2325;
                    }
                    let _e2327 = phi_3226_;
                    phi_3094_ = _e2226;
                    phi_3097_ = _e2327;
                    phi_3228_ = select(true, false, _e2267);
                } else {
                    phi_3094_ = u32();
                    phi_3097_ = f32();
                    phi_3228_ = false;
                }
                let _e2330 = phi_3094_;
                let _e2332 = phi_3097_;
                let _e2334 = phi_3228_;
                continue;
                continuing {
                    phi_3093_ = _e2330;
                    phi_3096_ = _e2332;
                    break if !(_e2334);
                }
            }
            let _e2337 = local_16;
            let _e2339 = ((_e2337 * 1.25f) + 0.5f);
            let _e2341 = select(_e2339, 0f, (_e2339 < 0f));
            let _e2343 = select(_e2341, 1f, (_e2341 > 1f));
            let _e2347 = ((_e2343 * _e2343) * (3f - (2f * _e2343)));
            if (_e2174 != _e2174) {
                phi_18438_ = true;
            } else {
                phi_18438_ = (_e2347 >= _e2174);
            }
            let _e2351 = phi_18438_;
            let _e2356 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e954 - _e1458), (_e955 - _e520)), 0f, _e520);
            let _e2358 = ((_e2356 - 2f) * 0.0625f);
            let _e2360 = select(_e2358, 0f, (_e2358 < 0f));
            let _e2362 = select(_e2360, 1f, (_e2360 > 1f));
            let _e2368 = ((select(_e2174, _e2347, _e2351) * ((_e2362 * _e2362) * (3f - (2f * _e2362)))) * _e914);
            let _e2369 = (1f - _e2368);
            let _e2371 = local_17;
            let _e2375 = local_18;
            let _e2379 = local_19;
            let _e2383 = local_20;
            let _e2386 = (0.94f * _e2368);
            let _e2394 = (((_e2383.w * _e2369) + _e2368) * _e931);
            if (_e2394 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e2371.x * _e2369) + _e2386) * _e931), (((_e2375.y * _e2369) + _e2386) * _e931), (((_e2379.z * _e2369) + _e2386) * _e931), _e2394);
            break;
        }
    }
    return;
}

fn function_2() {
    let _e495 = vertex_7;
    let _e496 = _isthmus_instance_index_9;
    let _e499 = line.member[_e496];
    let _e502 = ((_e499.size * 0.20000005f) + 1f);
    let _e507 = (_e499.min.x - _e502);
    let _e508 = (_e499.min.y - _e502);
    let _e524 = (_e507 + (f32((_e495 & 1u)) * ((_e499.max.x + _e502) - _e507)));
    let _e525 = (_e508 + (f32((_e495 >> bitcast<u32>(1i))) * ((_e499.max.y + _e502) - _e508)));
    let _e530 = frame.member[0u].screen_size[0u];
    let _e535 = frame.member[0u].screen_size[1u];
    out_position = vec4<f32>((((_e524 / _e530) * 2f) - 1f), (((_e525 / _e535) * 2f) - 1f), 0f, 1f);
    out_pixel[0u] = _e524;
    out_pixel[1u] = _e525;
    out_isthmus_instance_index = _e496;
    return;
}

fn function_3() {
    var phi_18492_: bool;
    var phi_4131_: u32;
    var phi_4134_: u32;
    var phi_4162_: u32;
    var phi_4132_: u32;
    var phi_4135_: u32;
    var local_22: u32;
    var phi_4174_: u32;
    var phi_4177_: f32;
    var phi_4256_: f32;
    var phi_4259_: u32;
    var phi_4261_: i32;
    var phi_4257_: f32;
    var phi_4260_: u32;
    var phi_4262_: i32;
    var local_23: f32;
    var phi_4294_: f32;
    var local_24: i32;
    var phi_18507_: bool;
    var phi_4302_: f32;
    var phi_4303_: f32;
    var phi_4304_: f32;
    var phi_4305_: f32;
    var phi_4306_: f32;
    var phi_4175_: u32;
    var phi_4178_: f32;
    var phi_4308_: bool;
    var local_25: f32;
    var local_26: f32;

    let _e495 = pixel_4;
    let _e496 = _isthmus_instance_index_10;
    let _e504 = frame.member[0u].launcher_open;
    if (_e504 > 0.5f) {
        discard;
    }
    let _e506 = (_e495.x * 0.03125f);
    let _e508 = select(_e506, 0f, (_e506 < 0f));
    let _e510 = select(_e508, 1f, (_e508 > 1f));
    let _e519 = frame.member[0u].screen_size[0u];
    let _e523 = ((_e495.x - _e519) / ((_e519 - 32f) - _e519));
    let _e525 = select(_e523, 0f, (_e523 < 0f));
    let _e527 = select(_e525, 1f, (_e525 > 1f));
    let _e532 = (((_e510 * _e510) * (3f - (2f * _e510))) * ((_e527 * _e527) * (3f - (2f * _e527))));
    let _e536 = frame.member[0u].playhead_x;
    let _e540 = ((abs((_e495.x - _e536)) - 110f) * -0.009090909f);
    let _e542 = select(_e540, 0f, (_e540 < 0f));
    let _e544 = select(_e542, 1f, (_e542 > 1f));
    let _e548 = ((_e544 * _e544) * (3f - (2f * _e544)));
    let _e549 = line.member[_e496];
    let _e552 = (_e549.weight + (_e548 * 0.15f));
    if (_e552 != _e552) {
        phi_18492_ = true;
    } else {
        phi_18492_ = (1f <= _e552);
    }
    let _e556 = phi_18492_;
    let _e559 = (1f + (_e548 * 0.2f));
    let _e561 = (1f / _e549.size);
    let _e568 = ((_e495.x - _e549.origin.x) * _e561);
    phi_4131_ = 0u;
    phi_4134_ = _e549.count;
    loop {
        let _e573 = phi_4131_;
        let _e575 = phi_4134_;
        local_22 = _e573;
        let _e576 = (_e573 < _e575);
        if _e576 {
            let _e579 = (_e573 + ((_e575 - _e573) / 2u));
            let _e584 = placed_glyphs_1.member[(_e549.first + _e579)].x;
            let _e585 = (_e584 <= _e568);
            if _e585 {
                phi_4162_ = (_e579 + 1u);
            } else {
                phi_4162_ = _e573;
            }
            let _e588 = phi_4162_;
            phi_4132_ = _e588;
            phi_4135_ = select(_e579, _e575, _e585);
        } else {
            phi_4132_ = u32();
            phi_4135_ = u32();
        }
        let _e591 = phi_4132_;
        let _e593 = phi_4135_;
        continue;
        continuing {
            phi_4131_ = _e591;
            phi_4134_ = _e593;
            break if !(_e576);
        }
    }
    let _e596 = ((3.5f / _e549.size) / _e559);
    let _e598 = local_22;
    let _e599 = (_e598 + 1u);
    phi_4174_ = select(_e599, _e549.count, (_e549.count < _e599));
    phi_4177_ = -1000000f;
    loop {
        let _e603 = phi_4174_;
        let _e605 = phi_4177_;
        local_25 = _e605;
        local_26 = _e605;
        if (_e603 > 0u) {
            let _e607 = (_e603 - 1u);
            let _e608 = (_e549.first + _e607);
            let _e612 = placed_glyphs_1.member[_e608].x;
            let _e616 = placed_glyphs_1.member[_e608].glyph;
            let _e621 = glyphs_1.member[_e616].min[0u];
            let _e626 = glyphs_1.member[_e616].min[1u];
            let _e631 = glyphs_1.member[_e616].max[0u];
            let _e636 = glyphs_1.member[_e616].max[1u];
            let _e640 = glyphs_1.member[_e616].start;
            let _e644 = glyphs_1.member[_e616].count;
            let _e647 = ((_e568 - _e612) / _e559);
            let _e648 = (-(((_e495.y - _e549.origin.y) * _e561)) / _e559);
            let _e649 = (_e631 + _e596);
            let _e650 = (_e647 > _e649);
            if _e650 {
                phi_4306_ = f32();
            } else {
                if (_e647 >= (_e621 - _e596)) {
                    if (_e648 >= (_e626 - _e596)) {
                        if (_e647 <= _e649) {
                            if (_e648 <= (_e636 + _e596)) {
                                let _e658 = (_e549.size * _e559);
                                phi_4256_ = 340282350000000000000000000000000000000f;
                                phi_4259_ = 0u;
                                phi_4261_ = 0i;
                                loop {
                                    let _e660 = phi_4256_;
                                    let _e662 = phi_4259_;
                                    let _e664 = phi_4261_;
                                    local_23 = _e660;
                                    local_24 = _e664;
                                    let _e665 = (_e662 < _e644);
                                    if _e665 {
                                        let _e669 = edges_1.member[(_e640 + _e662)];
                                        let _e671 = cantus_render_text_edge_distance(_e669, select(_e552, 1f, _e556), vec2<f32>(_e647, _e648), _e660);
                                        phi_4257_ = _e671.member;
                                        phi_4260_ = (_e662 + 1u);
                                        phi_4262_ = (_e664 + _e671.member_1);
                                    } else {
                                        phi_4257_ = f32();
                                        phi_4260_ = u32();
                                        phi_4262_ = i32();
                                    }
                                    let _e677 = phi_4257_;
                                    let _e679 = phi_4260_;
                                    let _e681 = phi_4262_;
                                    continue;
                                    continuing {
                                        phi_4256_ = _e677;
                                        phi_4259_ = _e679;
                                        phi_4261_ = _e681;
                                        break if !(_e665);
                                    }
                                }
                                let _e684 = local_23;
                                let _e686 = ((_e684 * _e658) * _e658);
                                if (_e686 >= 12.25f) {
                                    phi_4294_ = 3.5f;
                                } else {
                                    phi_4294_ = sqrt(_e686);
                                }
                                let _e690 = phi_4294_;
                                let _e692 = local_24;
                                let _e695 = (_e690 * select(1f, -1f, (_e692 == 0i)));
                                if (_e605 != _e605) {
                                    phi_18507_ = true;
                                } else {
                                    phi_18507_ = (_e695 >= _e605);
                                }
                                let _e699 = phi_18507_;
                                phi_4302_ = select(_e605, _e695, _e699);
                            } else {
                                phi_4302_ = _e605;
                            }
                            let _e702 = phi_4302_;
                            phi_4303_ = _e702;
                        } else {
                            phi_4303_ = _e605;
                        }
                        let _e704 = phi_4303_;
                        phi_4304_ = _e704;
                    } else {
                        phi_4304_ = _e605;
                    }
                    let _e706 = phi_4304_;
                    phi_4305_ = _e706;
                } else {
                    phi_4305_ = _e605;
                }
                let _e708 = phi_4305_;
                phi_4306_ = _e708;
            }
            let _e710 = phi_4306_;
            phi_4175_ = _e607;
            phi_4178_ = _e710;
            phi_4308_ = select(true, false, _e650);
        } else {
            phi_4175_ = u32();
            phi_4178_ = f32();
            phi_4308_ = false;
        }
        let _e713 = phi_4175_;
        let _e715 = phi_4178_;
        let _e717 = phi_4308_;
        continue;
        continuing {
            phi_4174_ = _e713;
            phi_4177_ = _e715;
            break if !(_e717);
        }
    }
    let _e720 = local_25;
    let _e722 = ((_e720 * 1.25f) + 0.5f);
    let _e724 = select(_e722, 0f, (_e722 < 0f));
    let _e726 = select(_e724, 1f, (_e724 > 1f));
    let _e730 = ((_e726 * _e726) * (3f - (2f * _e726)));
    let _e732 = local_26;
    let _e735 = (((_e732 + 0.9f) * 1.25f) + 0.5f);
    let _e737 = select(_e735, 0f, (_e735 < 0f));
    let _e739 = select(_e737, 1f, (_e737 > 1f));
    let _e748 = (_e536 + 4f);
    let _e752 = ((_e495.x - _e748) / ((_e536 - 4f) - _e748));
    let _e754 = select(_e752, 0f, (_e752 < 0f));
    let _e756 = select(_e754, 1f, (_e754 > 1f));
    let _e760 = ((_e756 * _e756) * (3f - (2f * _e756)));
    let _e764 = line.member[_e496].color;
    let _e765 = unpack4x8unorm(_e764);
    let _e772 = (1f - _e760);
    out_color = vec4<f32>(((((_e765.x * _e772) + ((_e765.x * 0.42f) * _e760)) * _e730) * _e532), ((((_e765.y * _e772) + ((_e765.y * 0.42f) * _e760)) * _e730) * _e532), ((((_e765.z * _e772) + ((_e765.z * 0.42f) * _e760)) * _e730) * _e532), ((_e730 + ((((_e739 * _e739) * (3f - (2f * _e739))) * 0.4f) * (1f - _e730))) * _e532));
    return;
}

fn function_4() {
    var phi_4452_: bool;
    var phi_4478_: u32;
    var phi_4481_: f32;
    var phi_4479_: u32;
    var phi_4482_: f32;
    var phi_21781_: bool;
    var local_27: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = vertex_7;
            let _e497 = _isthmus_instance_index_9;
            let _e501 = pill_1.member[_e497].battery_level;
            if (_e501 >= -1f) {
                phi_4452_ = (_e501 <= 1f);
            } else {
                phi_4452_ = false;
            }
            let _e505 = phi_4452_;
            let _e507 = (select(0f, 40f, _e505) + 296f);
            let _e512 = frame.member[0u].screen_size[0u];
            let _e518 = frame.member[0u].mouse_pressure;
            phi_4478_ = 0u;
            phi_4481_ = (_e518 * 8f);
            loop {
                let _e521 = phi_4478_;
                let _e523 = phi_4481_;
                local_27 = _e523;
                let _e524 = (_e521 < 4u);
                if _e524 {
                    if _e524 {
                    } else {
                        phi_21781_ = true;
                        break;
                    }
                    let _e530 = frame.member[0u].ripples[_e521].start_time;
                    let _e536 = frame.member[0u].ripples[_e521].strength;
                    let _e540 = frame.member[0u].time;
                    let _e542 = ((_e540 - _e530) * 1.2f);
                    let _e544 = select(_e542, 0f, (_e542 < 0f));
                    let _e547 = (1f - select(_e544, 1f, (_e544 > 1f)));
                    phi_4479_ = (_e521 + 1u);
                    phi_4482_ = (_e523 + (((_e536 * _e547) * _e547) * 11f));
                } else {
                    phi_4479_ = u32();
                    phi_4482_ = f32();
                }
                let _e554 = phi_4479_;
                let _e556 = phi_4482_;
                continue;
                continuing {
                    phi_4478_ = _e554;
                    phi_4481_ = _e556;
                    phi_21781_ = false;
                    break if !(_e524);
                }
            }
            let _e559 = phi_21781_;
            if _e559 {
                break;
            }
            let _e561 = local_27;
            let _e562 = (_e561 * 0.5f);
            let _e563 = (18f + _e562);
            let _e574 = frame.member[0u].panel_height;
            let _e581 = ((((_e512 - _e507) - 8f) - _e563) + (f32((_e496 & 1u)) * (_e507 + (_e563 * 2f))));
            let _e582 = ((-12f - _e562) + (f32((_e496 >> bitcast<u32>(1i))) * ((_e574 + _e563) * 2f)));
            let _e587 = frame.member[0u].screen_size[1u];
            out_position = vec4<f32>((((_e581 / _e512) * 2f) - 1f), (((_e582 / _e587) * 2f) - 1f), 0f, 1f);
            out_pixel[0u] = _e581;
            out_pixel[1u] = _e582;
            out_isthmus_instance_index = _e497;
            break;
        }
    }
    return;
}

fn cantus_render_shader_sd_rounded_box(param_14: vec2<f32>, param_15: vec2<f32>, param_16: f32) -> f32 {
    var phi_21567_: bool;
    var phi_21582_: bool;

    let _e506 = ((abs(param_14.x) - param_15.x) + param_16);
    let _e507 = ((abs(param_14.y) - param_15.y) + param_16);
    let _e509 = select(0f, _e506, (_e506 > 0f));
    let _e511 = select(0f, _e507, (_e507 > 0f));
    if (_e506 != _e506) {
        phi_21567_ = true;
    } else {
        phi_21567_ = (_e507 >= _e506);
    }
    let _e519 = phi_21567_;
    let _e520 = select(_e506, _e507, _e519);
    if (_e520 != _e520) {
        phi_21582_ = true;
    } else {
        phi_21582_ = (0f <= _e520);
    }
    let _e524 = phi_21582_;
    return ((sqrt(((_e509 * _e509) + (_e511 * _e511))) + select(_e520, 0f, _e524)) - param_16);
}

fn function_5() {
    var phi_4632_: bool;
    var phi_4689_: f32;
    var phi_4692_: vec2<f32>;
    var phi_4695_: f32;
    var phi_4697_: u32;
    var phi_18588_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_18599_: bool;
    var phi_4801_: vec2<f32>;
    var phi_4802_: f32;
    var phi_4803_: vec2<f32>;
    var phi_4804_: f32;
    var phi_4693_: vec2<f32>;
    var phi_4696_: f32;
    var phi_4698_: u32;
    var phi_21788_: bool;
    var phi_4848_: f32;
    var local_28: vec2<f32>;
    var local_29: vec2<f32>;
    var phi_4860_: bool;
    var local_30: vec2<f32>;
    var phi_4871_: f32;
    var local_31: vec2<f32>;
    var phi_18617_: bool;
    var phi_18632_: bool;
    var phi_18647_: bool;
    var phi_18664_: bool;
    var phi_18688_: i32;
    var phi_18689_: f32;
    var phi_18690_: f32;
    var phi_18691_: vec2<f32>;
    var phi_18716_: i32;
    var phi_18717_: f32;
    var phi_18718_: f32;
    var phi_18719_: vec2<f32>;
    var local_32: f32;
    var phi_18730_: i32;
    var phi_18731_: f32;
    var phi_18732_: f32;
    var phi_18733_: vec2<f32>;
    var phi_18758_: i32;
    var phi_18759_: f32;
    var phi_18760_: f32;
    var phi_18761_: vec2<f32>;
    var local_33: f32;
    var local_34: f32;
    var phi_5417_: vec3<f32>;
    var phi_5624_: vec3<f32>;
    var phi_5818_: vec3<f32>;
    var phi_6012_: vec3<f32>;
    var phi_18772_: i32;
    var phi_18773_: f32;
    var phi_18774_: f32;
    var phi_18775_: vec2<f32>;
    var phi_18800_: i32;
    var phi_18801_: f32;
    var phi_18802_: f32;
    var phi_18803_: vec2<f32>;
    var local_35: f32;
    var phi_6103_: vec3<f32>;
    var phi_6209_: bool;
    var phi_6248_: bool;
    var phi_6261_: bool;
    var phi_6295_: bool;
    var phi_6321_: u32;
    var phi_6322_: u32;
    var phi_6323_: u32;
    var phi_6324_: u32;
    var phi_6334_: bool;
    var phi_6350_: f32;
    var phi_18953_: bool;
    var phi_18954_: bool;
    var phi_18955_: bool;
    var phi_6362_: vec2<f32>;
    var phi_6363_: bool;
    var phi_19016_: i32;
    var phi_19017_: f32;
    var phi_19018_: f32;
    var phi_19019_: vec2<f32>;
    var phi_19044_: i32;
    var phi_19045_: f32;
    var phi_19046_: f32;
    var phi_19047_: vec2<f32>;
    var local_36: f32;
    var phi_6509_: vec2<f32>;
    var phi_19098_: i32;
    var phi_19099_: f32;
    var phi_19100_: f32;
    var phi_19101_: vec2<f32>;
    var phi_19126_: i32;
    var phi_19127_: f32;
    var phi_19128_: f32;
    var phi_19129_: vec2<f32>;
    var local_37: f32;
    var phi_6654_: vec2<f32>;
    var phi_6668_: vec2<f32>;
    var phi_19144_: bool;
    var phi_19159_: bool;
    var phi_19160_: bool;
    var phi_19161_: bool;
    var phi_19186_: bool;
    var phi_19201_: bool;
    var phi_19216_: bool;
    var phi_19231_: bool;
    var phi_19232_: bool;
    var phi_19233_: bool;
    var phi_19337_: bool;
    var phi_19352_: bool;
    var phi_19367_: bool;
    var phi_19382_: bool;
    var phi_19397_: bool;
    var phi_19412_: bool;
    var phi_19427_: bool;
    var phi_21803_: bool;
    var phi_8759_: vec3<f32>;
    var phi_8760_: bool;
    var phi_19442_: bool;
    var phi_19487_: bool;
    var phi_19502_: bool;
    var phi_19517_: bool;
    var phi_9158_: f32;
    var phi_19532_: bool;
    var phi_9185_: vec3<f32>;
    var local_38: f32;
    var local_39: f32;
    var phi_9222_: render_text_Line;
    var phi_9227_: bool;
    var phi_9242_: u32;
    var phi_9245_: u32;
    var phi_9273_: u32;
    var phi_9243_: u32;
    var phi_9246_: u32;
    var local_40: u32;
    var phi_9285_: u32;
    var phi_9288_: f32;
    var phi_9368_: f32;
    var phi_9371_: u32;
    var phi_9373_: i32;
    var phi_9369_: f32;
    var phi_9372_: u32;
    var phi_9374_: i32;
    var local_41: f32;
    var phi_9406_: f32;
    var local_42: i32;
    var phi_19547_: bool;
    var phi_9414_: f32;
    var phi_9415_: f32;
    var phi_9416_: f32;
    var phi_9417_: f32;
    var phi_9418_: f32;
    var phi_9286_: u32;
    var phi_9289_: f32;
    var phi_9420_: bool;
    var local_43: f32;
    var phi_9445_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = pixel_4;
            let _e497 = _isthmus_instance_index_10;
            let _e503 = frame.member[0u].launcher_open;
            if (_e503 > 0.5f) {
                discard;
            }
            let _e508 = pill_1.member[_e497].battery_level;
            let _e509 = (_e508 >= -1f);
            if _e509 {
                phi_4632_ = (_e508 <= 1f);
            } else {
                phi_4632_ = false;
            }
            let _e512 = phi_4632_;
            let _e514 = (select(0f, 40f, _e512) + 296f);
            let _e519 = frame.member[0u].screen_size[0u];
            let _e521 = ((_e519 - _e514) - 8f);
            let _e525 = frame.member[0u].panel_height;
            let _e526 = (_e496.x - _e521);
            let _e527 = (_e496.y - 6f);
            let _e528 = (_e514 * 0.5f);
            let _e529 = (_e525 * 0.5f);
            let _e533 = ((_e514 - _e525) * 0.5f);
            let _e535 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e526 - _e528), (_e527 - _e529)), _e533, _e529);
            let _e539 = frame.member[0u].mouse_pressure;
            let _e540 = (_e539 > 0f);
            if _e540 {
                let _e545 = frame.member[0u].mouse_pos[0u];
                let _e550 = frame.member[0u].mouse_pos[1u];
                let _e556 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e545 - _e521) - _e528), ((_e550 - 6f) - _e529)), _e533, _e529);
                phi_4689_ = _e556;
            } else {
                phi_4689_ = 1f;
            }
            let _e558 = phi_4689_;
            phi_4692_ = vec2<f32>(0f, 0f);
            phi_4695_ = 0f;
            phi_4697_ = 0u;
            loop {
                let _e560 = phi_4692_;
                let _e562 = phi_4695_;
                let _e564 = phi_4697_;
                local_28 = _e560;
                local_29 = _e560;
                local_30 = _e560;
                local_31 = _e560;
                local_38 = _e562;
                local_39 = _e562;
                let _e565 = (_e564 < 4u);
                if _e565 {
                    if _e565 {
                    } else {
                        phi_21788_ = true;
                        break;
                    }
                    let _e572 = frame.member[0u].ripples[_e564].origin[0u];
                    let _e579 = frame.member[0u].ripples[_e564].origin[1u];
                    let _e585 = frame.member[0u].ripples[_e564].start_time;
                    let _e591 = frame.member[0u].ripples[_e564].strength;
                    let _e595 = frame.member[0u].time;
                    let _e597 = ((_e595 - _e585) * 1.2f);
                    let _e599 = select(_e597, 0f, (_e597 < 0f));
                    let _e601 = select(_e599, 1f, (_e599 > 1f));
                    if (_e591 > 0f) {
                        if (_e601 < 1f) {
                            let _e605 = (_e496 - vec2<f32>(_e572, _e579));
                            let _e611 = sqrt(((_e605.x * _e605.x) + (_e605.y * _e605.y)));
                            if (_e611 > 0.001f) {
                                phi_18588_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e605.x / _e611), (_e605.y / _e611)), _e611);
                            } else {
                                phi_18588_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e611);
                            }
                            let _e619 = phi_18588_;
                            let _e629 = ((abs((_e619.unnamed_1 - (_e601 * 600f))) - 80f) * -0.0125f);
                            let _e631 = select(_e629, 0f, (_e629 < 0f));
                            let _e633 = select(_e631, 1f, (_e631 > 1f));
                            let _e639 = (1f - _e601);
                            let _e640 = ((((_e633 * _e633) * (3f - (2f * _e633))) * _e591) * _e639);
                            let _e653 = (_e562 + (_e640 * 0.5f));
                            if (_e653 != _e653) {
                                phi_18599_ = true;
                            } else {
                                phi_18599_ = (1f <= _e653);
                            }
                            let _e657 = phi_18599_;
                            phi_4801_ = vec2<f32>((_e560.x + (((_e619.unnamed.x * _e640) * _e639) * 0.5f)), (_e560.y + (((_e619.unnamed.y * _e640) * _e639) * 0.5f)));
                            phi_4802_ = select(_e653, 1f, _e657);
                        } else {
                            phi_4801_ = _e560;
                            phi_4802_ = _e562;
                        }
                        let _e660 = phi_4801_;
                        let _e662 = phi_4802_;
                        phi_4803_ = _e660;
                        phi_4804_ = _e662;
                    } else {
                        phi_4803_ = _e560;
                        phi_4804_ = _e562;
                    }
                    let _e664 = phi_4803_;
                    let _e666 = phi_4804_;
                    phi_4693_ = _e664;
                    phi_4696_ = _e666;
                    phi_4698_ = (_e564 + 1u);
                } else {
                    phi_4693_ = vec2<f32>();
                    phi_4696_ = f32();
                    phi_4698_ = u32();
                }
                let _e669 = phi_4693_;
                let _e671 = phi_4696_;
                let _e673 = phi_4698_;
                continue;
                continuing {
                    phi_4692_ = _e669;
                    phi_4695_ = _e671;
                    phi_4697_ = _e673;
                    phi_21788_ = false;
                    break if !(_e565);
                }
            }
            let _e676 = phi_21788_;
            if _e676 {
                break;
            }
            if _e540 {
                let _e681 = frame.member[0u].mouse_pos[0u];
                let _e686 = frame.member[0u].mouse_pos[1u];
                let _e687 = (_e496.x - _e681);
                let _e688 = (_e496.y - _e686);
                let _e694 = ((sqrt(((_e687 * _e687) + (_e688 * _e688))) - 150f) * -0.006666667f);
                let _e696 = select(_e694, 0f, (_e694 < 0f));
                let _e698 = select(_e696, 1f, (_e696 > 1f));
                phi_4848_ = ((((_e698 * _e698) * (3f - (2f * _e698))) * _e539) * 8f);
            } else {
                phi_4848_ = 0f;
            }
            let _e706 = phi_4848_;
            let _e708 = local_28;
            let _e711 = global[0u];
            if (_e708.x == _e711) {
                let _e714 = local_29;
                let _e717 = global[1u];
                phi_4860_ = (_e714.y == _e717);
            } else {
                phi_4860_ = false;
            }
            let _e720 = phi_4860_;
            if _e720 {
                phi_4871_ = 0f;
            } else {
                let _e722 = local_30;
                phi_4871_ = (sqrt(((_e708.x * _e708.x) + (_e722.y * _e722.y))) * 22f);
            }
            let _e730 = phi_4871_;
            let _e732 = local_31;
            let _e735 = ((_e558 - 0.5f) * -1f);
            let _e737 = select(_e735, 0f, (_e735 < 0f));
            let _e739 = select(_e737, 1f, (_e737 > 1f));
            let _e747 = (_e535 - (((_e706 * ((_e739 * _e739) * (3f - (2f * _e739)))) + _e730) * 0.5f));
            let _e748 = fwidth(_e747);
            if (_e748 != _e748) {
                phi_18617_ = true;
            } else {
                phi_18617_ = (0.55f >= _e748);
            }
            let _e752 = phi_18617_;
            let _e753 = select(_e748, 0.55f, _e752);
            let _e757 = ((_e747 - _e753) / (-(_e753) - _e753));
            let _e759 = select(_e757, 0f, (_e757 < 0f));
            let _e761 = select(_e759, 1f, (_e759 > 1f));
            let _e765 = ((_e761 * _e761) * (3f - (2f * _e761)));
            let _e766 = (_e747 != _e747);
            if _e766 {
                phi_18632_ = true;
            } else {
                phi_18632_ = (0f >= _e747);
            }
            let _e769 = phi_18632_;
            let _e773 = (exp((select(_e747, 0f, _e769) * -0.3f)) * 0.16f);
            if (_e765 != _e765) {
                phi_18647_ = true;
            } else {
                phi_18647_ = (_e773 >= _e765);
            }
            let _e777 = phi_18647_;
            let _e778 = select(_e765, _e773, _e777);
            if (_e778 <= 0.0009765625f) {
                discard;
            }
            let _e780 = (_e526 / _e514);
            let _e781 = (_e527 / _e525);
            if _e766 {
                phi_18664_ = true;
            } else {
                phi_18664_ = (0f <= _e747);
            }
            let _e786 = phi_18664_;
            let _e789 = (1f + (select(_e747, 0f, _e786) * 0.008333334f));
            let _e791 = select(_e789, 0f, (_e789 < 0f));
            let _e793 = select(_e791, 0.6f, (_e791 > 0.6f));
            let _e802 = ((_e780 - (((_e780 - 0.5f) * _e793) * 0.08f)) - (_e708.x * 0.04f));
            let _e803 = ((_e781 - (((_e781 - 0.5f) * _e793) * 0.08f)) - (_e732.y * 0.04f));
            let _e804 = (_e802 * _e514);
            let _e805 = (_e803 * _e525);
            let _e809 = pill_1.member[_e497].sun_height;
            let _e811 = ((_e809 - -0.04f) * 4.1666665f);
            let _e813 = select(_e811, 0f, (_e811 < 0f));
            let _e815 = select(_e813, 1f, (_e813 > 1f));
            let _e819 = ((_e815 * _e815) * (3f - (2f * _e815)));
            let _e821 = ((_e809 - -0.32f) * 4.166667f);
            let _e823 = select(_e821, 0f, (_e821 < 0f));
            let _e825 = select(_e823, 1f, (_e823 > 1f));
            let _e830 = (1f - _e819);
            let _e833 = ((_e809 - -0.18f) * 5.5555553f);
            let _e835 = select(_e833, 0f, (_e833 < 0f));
            let _e837 = select(_e835, 1f, (_e835 > 1f));
            let _e843 = ((_e809 - 0.2f) * -5.5555553f);
            let _e845 = select(_e843, 0f, (_e843 < 0f));
            let _e847 = select(_e845, 1f, (_e845 > 1f));
            let _e852 = (((_e837 * _e837) * (3f - (2f * _e837))) * ((_e847 * _e847) * (3f - (2f * _e847))));
            let _e856 = pill_1.member[_e497].conditions;
            let _e860 = frame.member[0u].time;
            let _e862 = ((_e803 - 1f) * -1f);
            let _e864 = select(_e862, 0f, (_e862 < 0f));
            let _e866 = select(_e864, 1f, (_e864 > 1f));
            let _e870 = ((_e866 * _e866) * (3f - (2f * _e866)));
            let _e871 = (1f - _e870);
            let _e901 = (0.3f * _e871);
            let _e902 = (0.22f * _e870);
            let _e908 = ((((_e825 * _e825) * (3f - (2f * _e825))) * _e830) * 0.8f);
            let _e909 = (1f - _e908);
            let _e926 = (_e852 * 0.9f);
            let _e927 = (1f - _e926);
            let _e939 = floor((_e804 * 0.055555556f));
            let _e940 = floor((_e805 * 0.055555556f));
            let _e944 = cantus_render_shader_hash(vec2<f32>(_e939, _e940));
            let _e953 = (_e804 - (((_e939 + 0.2f) + (_e944.x * 0.6f)) * 18f));
            let _e954 = (_e805 - (((_e940 + 0.2f) + (_e944.y * 0.6f)) * 18f));
            let _e960 = ((sqrt(((_e953 * _e953) + (_e954 * _e954))) - 1f) * -1.6666666f);
            let _e962 = select(_e960, 0f, (_e960 < 0f));
            let _e964 = select(_e962, 1f, (_e962 > 1f));
            let _e972 = cantus_render_shader_hash(vec2<f32>((_e939 + 31.7f), (_e940 + 31.7f)));
            let _e975 = ((_e972.x - 0.75f) * 4f);
            let _e977 = select(_e975, 0f, (_e975 < 0f));
            let _e979 = select(_e977, 1f, (_e977 > 1f));
            let _e991 = ((((((_e964 * _e964) * (3f - (2f * _e964))) * ((_e979 * _e979) * (3f - (2f * _e979)))) * _e830) * (1f - _e856.cloud)) * (0.3f + (_e870 * 0.7f)));
            let _e992 = (((((((((0.006f * _e871) + (0.025f * _e870)) * _e830) + (((0.08f * _e871) + (0.32f * _e870)) * _e819)) * _e909) + (((0.1f * _e871) + _e902) * _e908)) * _e927) + (((0.78f * _e871) + (0.38f * _e870)) * _e926)) + _e991);
            let _e993 = (((((((((0.012f * _e871) + (0.04f * _e870)) * _e830) + (((0.34f * _e871) + (0.67f * _e870)) * _e819)) * _e909) + (((0.16f * _e871) + (0.25f * _e870)) * _e908)) * _e927) + ((_e901 + _e902) * _e926)) + _e991);
            let _e994 = (((((((((0.035f * _e871) + (0.095f * _e870)) * _e830) + (((0.62f * _e871) + (0.87f * _e870)) * _e819)) * _e909) + ((_e901 + (0.45f * _e870)) * _e908)) * _e927) + (((0.2f * _e871) + (0.42f * _e870)) * _e926)) + _e991);
            if (_e856.cloud > 0.0009765625f) {
                let _e997 = (_e804 / _e525);
                phi_18688_ = 0i;
                phi_18689_ = 0.5f;
                phi_18690_ = 0f;
                phi_18691_ = vec2<f32>(((_e997 * 0.14f) + (_e860 * 0.012f)), ((_e803 * 0.14f) + 6.1f));
                loop {
                    let _e1005 = phi_18688_;
                    let _e1007 = phi_18689_;
                    let _e1009 = phi_18690_;
                    let _e1011 = phi_18691_;
                    local_32 = _e1009;
                    let _e1012 = (_e1005 < 4i);
                    if _e1012 {
                        let _e1015 = cantus_render_shader_simplex_noise(_e1011);
                        phi_18716_ = (_e1005 + 1i);
                        phi_18717_ = (_e1007 * 0.5f);
                        phi_18718_ = (_e1009 + (_e1015 * _e1007));
                        phi_18719_ = vec2<f32>(((_e1011.x * 1.6f) + (_e1011.y * 1.2f)), ((_e1011.y * 1.6f) - (_e1011.x * 1.2f)));
                    } else {
                        phi_18716_ = i32();
                        phi_18717_ = f32();
                        phi_18718_ = f32();
                        phi_18719_ = vec2<f32>();
                    }
                    let _e1028 = phi_18716_;
                    let _e1030 = phi_18717_;
                    let _e1032 = phi_18718_;
                    let _e1034 = phi_18719_;
                    continue;
                    continuing {
                        phi_18688_ = _e1028;
                        phi_18689_ = _e1030;
                        phi_18690_ = _e1032;
                        phi_18691_ = _e1034;
                        break if !(_e1012);
                    }
                }
                let _e1037 = local_32;
                let _e1038 = (_e1037 * 0.5f);
                phi_18730_ = 0i;
                phi_18731_ = 0.5f;
                phi_18732_ = 0f;
                phi_18733_ = vec2<f32>(((_e997 * 0.287f) + (_e860 * 0.018f)), ((_e803 * 0.287f) + -3.7f));
                loop {
                    let _e1047 = phi_18730_;
                    let _e1049 = phi_18731_;
                    let _e1051 = phi_18732_;
                    let _e1053 = phi_18733_;
                    local_33 = _e1051;
                    local_34 = _e1051;
                    let _e1054 = (_e1047 < 4i);
                    if _e1054 {
                        let _e1057 = cantus_render_shader_simplex_noise(_e1053);
                        phi_18758_ = (_e1047 + 1i);
                        phi_18759_ = (_e1049 * 0.5f);
                        phi_18760_ = (_e1051 + (_e1057 * _e1049));
                        phi_18761_ = vec2<f32>(((_e1053.x * 1.6f) + (_e1053.y * 1.2f)), ((_e1053.y * 1.6f) - (_e1053.x * 1.2f)));
                    } else {
                        phi_18758_ = i32();
                        phi_18759_ = f32();
                        phi_18760_ = f32();
                        phi_18761_ = vec2<f32>();
                    }
                    let _e1070 = phi_18758_;
                    let _e1072 = phi_18759_;
                    let _e1074 = phi_18760_;
                    let _e1076 = phi_18761_;
                    continue;
                    continuing {
                        phi_18730_ = _e1070;
                        phi_18731_ = _e1072;
                        phi_18732_ = _e1074;
                        phi_18733_ = _e1076;
                        break if !(_e1054);
                    }
                }
                let _e1079 = local_33;
                let _e1082 = local_34;
                let _e1086 = ((((0.5f + _e1038) + (_e1082 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1088 = select(_e1086, 0f, (_e1086 < 0f));
                let _e1090 = select(_e1088, 1f, (_e1088 > 1f));
                let _e1096 = (((_e1079 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1098 = select(_e1096, 0f, (_e1096 < 0f));
                let _e1100 = select(_e1098, 1f, (_e1098 > 1f));
                let _e1107 = ((_e1038 + 0.02000001f) * 4.5454545f);
                let _e1109 = select(_e1107, 0f, (_e1107 < 0f));
                let _e1111 = select(_e1109, 1f, (_e1109 > 1f));
                let _e1117 = ((((_e1100 * _e1100) * (3f - (2f * _e1100))) * 0.55f) + (((_e1111 * _e1111) * (3f - (2f * _e1111))) * 0.45f));
                let _e1118 = (1f - _e1117);
                let _e1155 = (_e852 * 0.45f);
                let _e1156 = (1f - _e1155);
                let _e1168 = (_e856.cloud * (0.12f + (((_e1090 * _e1090) * (3f - (2f * _e1090))) * 0.7f)));
                let _e1169 = (1f - _e1168);
                phi_5417_ = vec3<f32>(((_e992 * _e1169) + (((((((0.16f * _e1118) + (0.32f * _e1117)) * _e830) + (((0.62f * _e1118) + (0.92f * _e1117)) * _e819)) * _e1156) + (((0.5f * _e1118) + (0.76f * _e1117)) * _e1155)) * _e1168)), ((_e993 * _e1169) + (((((((0.2f * _e1118) + (0.36f * _e1117)) * _e830) + (((0.7f * _e1118) + (0.94f * _e1117)) * _e819)) * _e1156) + (((0.36f * _e1118) + (0.59f * _e1117)) * _e1155)) * _e1168)), ((_e994 * _e1169) + (((((((0.28f * _e1118) + (0.43f * _e1117)) * _e830) + (((0.78f * _e1118) + (0.96f * _e1117)) * _e819)) * _e1156) + (((0.4f * _e1118) + (0.56f * _e1117)) * _e1155)) * _e1168)));
            } else {
                phi_5417_ = vec3<f32>(_e992, _e993, _e994);
            }
            let _e1181 = phi_5417_;
            let _e1184 = (1f - (_e856.rain * 0.2f));
            let _e1194 = ((_e1181.x * _e1184) + (_e856.rain * 0.020000001f));
            let _e1195 = ((_e1181.y * _e1184) + (_e856.rain * 0.034f));
            let _e1196 = ((_e1181.z * _e1184) + (_e856.rain * 0.05f));
            if (_e856.rain > 0.0009765625f) {
                let _e1201 = (_e804 - (20f * _e860));
                let _e1202 = (_e805 - (110f * _e860));
                let _e1205 = floor((_e1201 * 0.06666667f));
                let _e1206 = floor((_e1202 * 0.04f));
                let _e1208 = cantus_render_shader_hash(vec2<f32>(_e1205, _e1206));
                let _e1219 = (_e1201 - (((_e1205 + 0.15f) + (_e1208.x * 0.7f)) * 15f));
                let _e1220 = (_e1202 - (((_e1206 + 0.15f) + (_e1208.y * 0.7f)) * 25f));
                let _e1224 = (((_e1219 * 1.8000001f) + (_e1220 * 9f)) * 0.011870845f);
                let _e1226 = select(_e1224, 0f, (_e1224 < 0f));
                let _e1228 = select(_e1226, 1f, (_e1226 > 1f));
                let _e1231 = (_e1219 - (1.8000001f * _e1228));
                let _e1232 = (_e1220 - (9f * _e1228));
                let _e1238 = ((sqrt(((_e1231 * _e1231) + (_e1232 * _e1232))) - 1.0999999f) * -1.666667f);
                let _e1240 = select(_e1238, 0f, (_e1238 < 0f));
                let _e1242 = select(_e1240, 1f, (_e1240 > 1f));
                let _e1250 = cantus_render_shader_hash(vec2<f32>((_e1205 + 19.3f), (_e1206 + 19.3f)));
                let _e1253 = ((_e1250.x - 0.22000003f) * 1.2820513f);
                let _e1255 = select(_e1253, 0f, (_e1253 < 0f));
                let _e1257 = select(_e1255, 1f, (_e1255 > 1f));
                let _e1264 = (((((_e1242 * _e1242) * (3f - (2f * _e1242))) * ((_e1257 * _e1257) * (3f - (2f * _e1257)))) * _e856.rain) * 0.7f);
                let _e1266 = select(_e1264, 0f, (_e1264 < 0f));
                let _e1268 = select(_e1266, 1f, (_e1266 > 1f));
                let _e1269 = (1f - _e1268);
                phi_5624_ = vec3<f32>(((_e1194 * _e1269) + (0.52f * _e1268)), ((_e1195 * _e1269) + (0.72f * _e1268)), ((_e1196 * _e1269) + (0.9f * _e1268)));
            } else {
                phi_5624_ = vec3<f32>(_e1194, _e1195, _e1196);
            }
            let _e1281 = phi_5624_;
            if (_e856.snow > 0.0009765625f) {
                let _e1286 = (_e804 - (5f * _e860));
                let _e1287 = (_e805 - (14f * _e860));
                let _e1290 = floor((_e1286 * 0.05f));
                let _e1291 = floor((_e1287 * 0.05f));
                let _e1295 = cantus_render_shader_hash(vec2<f32>((_e1290 + 31.7f), (_e1291 + 31.7f)));
                let _e1306 = (_e1286 - (((_e1290 + 0.15f) + (_e1295.x * 0.7f)) * 20f));
                let _e1307 = (_e1287 - (((_e1291 + 0.15f) + (_e1295.y * 0.7f)) * 20f));
                let _e1311 = (((_e1306 * 0.080000006f) + (_e1307 * 0.4f)) * 6.009615f);
                let _e1313 = select(_e1311, 0f, (_e1311 < 0f));
                let _e1315 = select(_e1313, 1f, (_e1313 > 1f));
                let _e1318 = (_e1306 - (0.080000006f * _e1315));
                let _e1319 = (_e1307 - (0.4f * _e1315));
                let _e1325 = ((sqrt(((_e1318 * _e1318) + (_e1319 * _e1319))) - 1.5999999f) * -1.666667f);
                let _e1327 = select(_e1325, 0f, (_e1325 < 0f));
                let _e1329 = select(_e1327, 1f, (_e1327 > 1f));
                let _e1337 = cantus_render_shader_hash(vec2<f32>((_e1290 + 19.3f), (_e1291 + 19.3f)));
                let _e1340 = ((_e1337.x - 0.3f) * 1.4285715f);
                let _e1342 = select(_e1340, 0f, (_e1340 < 0f));
                let _e1344 = select(_e1342, 1f, (_e1342 > 1f));
                let _e1351 = (((((_e1329 * _e1329) * (3f - (2f * _e1329))) * ((_e1344 * _e1344) * (3f - (2f * _e1344)))) * _e856.snow) * 0.92f);
                let _e1353 = select(_e1351, 0f, (_e1351 < 0f));
                let _e1355 = select(_e1353, 1f, (_e1353 > 1f));
                let _e1356 = (1f - _e1355);
                let _e1363 = (0.96f * _e1355);
                phi_5818_ = vec3<f32>(((_e1281.x * _e1356) + _e1363), ((_e1281.y * _e1356) + _e1363), ((_e1281.z * _e1356) + _e1363));
            } else {
                phi_5818_ = _e1281;
            }
            let _e1369 = phi_5818_;
            if (_e856.hail > 0.0009765625f) {
                let _e1374 = (_e804 - (18f * _e860));
                let _e1375 = (_e805 - (85f * _e860));
                let _e1378 = floor((_e1374 * 0.04347826f));
                let _e1379 = floor((_e1375 * 0.04347826f));
                let _e1383 = cantus_render_shader_hash(vec2<f32>((_e1378 + 63.4f), (_e1379 + 63.4f)));
                let _e1394 = (_e1374 - (((_e1378 + 0.15f) + (_e1383.x * 0.7f)) * 23f));
                let _e1395 = (_e1375 - (((_e1379 + 0.15f) + (_e1383.y * 0.7f)) * 23f));
                let _e1399 = (((_e1394 * 0.24000001f) + (_e1395 * 1.2f)) * 0.667735f);
                let _e1401 = select(_e1399, 0f, (_e1399 < 0f));
                let _e1403 = select(_e1401, 1f, (_e1401 > 1f));
                let _e1406 = (_e1394 - (0.24000001f * _e1403));
                let _e1407 = (_e1395 - (1.2f * _e1403));
                let _e1413 = ((sqrt(((_e1406 * _e1406) + (_e1407 * _e1407))) - 0.79999995f) * -1.6666667f);
                let _e1415 = select(_e1413, 0f, (_e1413 < 0f));
                let _e1417 = select(_e1415, 1f, (_e1415 > 1f));
                let _e1425 = cantus_render_shader_hash(vec2<f32>((_e1378 + 19.3f), (_e1379 + 19.3f)));
                let _e1428 = ((_e1425.x - 0.7f) * 3.3333333f);
                let _e1430 = select(_e1428, 0f, (_e1428 < 0f));
                let _e1432 = select(_e1430, 1f, (_e1430 > 1f));
                let _e1439 = (((((_e1417 * _e1417) * (3f - (2f * _e1417))) * ((_e1432 * _e1432) * (3f - (2f * _e1432)))) * _e856.hail) * 0.7f);
                let _e1441 = select(_e1439, 0f, (_e1439 < 0f));
                let _e1443 = select(_e1441, 1f, (_e1441 > 1f));
                let _e1444 = (1f - _e1443);
                phi_6012_ = vec3<f32>(((_e1369.x * _e1444) + (0.75f * _e1443)), ((_e1369.y * _e1444) + (0.86f * _e1443)), ((_e1369.z * _e1444) + (0.94f * _e1443)));
            } else {
                phi_6012_ = _e1369;
            }
            let _e1459 = phi_6012_;
            let _e1463 = ((sin((_e860 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1465 = select(_e1463, 0f, (_e1463 < 0f));
            let _e1467 = select(_e1465, 1f, (_e1465 > 1f));
            let _e1473 = (((_e1467 * _e1467) * (3f - (2f * _e1467))) * _e856.lightning);
            let _e1475 = (1f - (_e1473 * 0.55f));
            let _e1485 = ((_e1459.x * _e1475) + (_e1473 * 0.3575f));
            let _e1486 = ((_e1459.y * _e1475) + (_e1473 * 0.407f));
            let _e1487 = ((_e1459.z * _e1475) + (_e1473 * 0.528f));
            if (_e856.fog > 0.0009765625f) {
                phi_18772_ = 0i;
                phi_18773_ = 0.5f;
                phi_18774_ = 0f;
                phi_18775_ = vec2<f32>(((_e802 * 0.9f) + (_e860 * 0.008f)), ((_e803 * 0.32f) + 12f));
                loop {
                    let _e1498 = phi_18772_;
                    let _e1500 = phi_18773_;
                    let _e1502 = phi_18774_;
                    let _e1504 = phi_18775_;
                    local_35 = _e1502;
                    let _e1505 = (_e1498 < 4i);
                    if _e1505 {
                        let _e1508 = cantus_render_shader_simplex_noise(_e1504);
                        phi_18800_ = (_e1498 + 1i);
                        phi_18801_ = (_e1500 * 0.5f);
                        phi_18802_ = (_e1502 + (_e1508 * _e1500));
                        phi_18803_ = vec2<f32>(((_e1504.x * 1.6f) + (_e1504.y * 1.2f)), ((_e1504.y * 1.6f) - (_e1504.x * 1.2f)));
                    } else {
                        phi_18800_ = i32();
                        phi_18801_ = f32();
                        phi_18802_ = f32();
                        phi_18803_ = vec2<f32>();
                    }
                    let _e1521 = phi_18800_;
                    let _e1523 = phi_18801_;
                    let _e1525 = phi_18802_;
                    let _e1527 = phi_18803_;
                    continue;
                    continuing {
                        phi_18772_ = _e1521;
                        phi_18773_ = _e1523;
                        phi_18774_ = _e1525;
                        phi_18775_ = _e1527;
                        break if !(_e1505);
                    }
                }
                let _e1530 = local_35;
                let _e1533 = (((_e1530 * 0.5f) + 0.15f) * 2.857143f);
                let _e1535 = select(_e1533, 0f, (_e1533 < 0f));
                let _e1537 = select(_e1535, 1f, (_e1535 > 1f));
                let _e1544 = (_e856.fog * (0.58f + (((_e1537 * _e1537) * (3f - (2f * _e1537))) * 0.18f)));
                let _e1545 = (1f - _e1544);
                phi_6103_ = vec3<f32>(((_e1485 * _e1545) + (0.63f * _e1544)), ((_e1486 * _e1545) + (0.69f * _e1544)), ((_e1487 * _e1545) + (0.73f * _e1544)));
            } else {
                phi_6103_ = vec3<f32>(_e1485, _e1486, _e1487);
            }
            let _e1557 = phi_6103_;
            let _e1559 = ((_e747 - 5f) * -0.125f);
            let _e1561 = select(_e1559, 0f, (_e1559 < 0f));
            let _e1563 = select(_e1561, 1f, (_e1561 > 1f));
            let _e1568 = (((_e1563 * _e1563) * (3f - (2f * _e1563))) * 0.14f);
            if (_e804 < 96f) {
                phi_6324_ = 0u;
            } else {
                if (_e804 < 184f) {
                    phi_6323_ = 1u;
                } else {
                    if _e509 {
                        phi_6209_ = (_e508 <= 1f);
                    } else {
                        phi_6209_ = false;
                    }
                    let _e1579 = phi_6209_;
                    if _e1579 {
                        phi_6248_ = select(true, false, (_e804 < 224f));
                    } else {
                        phi_6248_ = true;
                    }
                    let _e1583 = phi_6248_;
                    if _e1583 {
                        if _e509 {
                            phi_6261_ = (_e508 <= 1f);
                        } else {
                            phi_6261_ = false;
                        }
                        let _e1586 = phi_6261_;
                        if (_e804 < (select(0f, 40f, _e1586) + 224f)) {
                            phi_6321_ = 3u;
                        } else {
                            if _e509 {
                                phi_6295_ = (_e508 <= 1f);
                            } else {
                                phi_6295_ = false;
                            }
                            let _e1592 = phi_6295_;
                            phi_6321_ = select(5u, 4u, (_e804 < (select(0f, 40f, _e1592) + 256f)));
                        }
                        let _e1598 = phi_6321_;
                        phi_6322_ = _e1598;
                    } else {
                        phi_6322_ = 2u;
                    }
                    let _e1600 = phi_6322_;
                    phi_6323_ = _e1600;
                }
                let _e1602 = phi_6323_;
                phi_6324_ = _e1602;
            }
            let _e1604 = phi_6324_;
            if _e509 {
                phi_6334_ = (_e508 <= 1f);
            } else {
                phi_6334_ = false;
            }
            let _e1607 = phi_6334_;
            let _e1608 = select(0f, 40f, _e1607);
            switch bitcast<i32>(_e1604) {
                case 0: {
                    phi_6350_ = 12f;
                    break;
                }
                case 1: {
                    phi_6350_ = 100f;
                    break;
                }
                case 2: {
                    phi_6350_ = 188f;
                    break;
                }
                case 3: {
                    phi_6350_ = (188f + _e1608);
                    break;
                }
                case 4: {
                    phi_6350_ = (228f + _e1608);
                    break;
                }
                case 5: {
                    phi_6350_ = (260f + _e1608);
                    break;
                }
                default: {
                    phi_6350_ = f32();
                    break;
                }
            }
            let _e1614 = phi_6350_;
            switch bitcast<i32>(_e1604) {
                case 0: {
                    phi_18953_ = true;
                    phi_18954_ = false;
                    phi_18955_ = false;
                    break;
                }
                case 1: {
                    phi_18953_ = true;
                    phi_18954_ = false;
                    phi_18955_ = false;
                    break;
                }
                case 2: {
                    phi_18953_ = false;
                    phi_18954_ = true;
                    phi_18955_ = false;
                    break;
                }
                case 3: {
                    phi_18953_ = false;
                    phi_18954_ = true;
                    phi_18955_ = false;
                    break;
                }
                case 4: {
                    phi_18953_ = false;
                    phi_18954_ = false;
                    phi_18955_ = true;
                    break;
                }
                case 5: {
                    phi_18953_ = false;
                    phi_18954_ = false;
                    phi_18955_ = true;
                    break;
                }
                default: {
                    phi_18953_ = bool();
                    phi_18954_ = bool();
                    phi_18955_ = bool();
                    break;
                }
            }
            let _e1617 = phi_18953_;
            let _e1619 = phi_18954_;
            let _e1621 = phi_18955_;
            let _e1622 = select(_e1619, false, _e1617);
            let _e1629 = (_e804 - (_e1614 + (select(select(80f, 32f, _e1622), 24f, select(select(_e1621, false, _e1617), false, _e1622)) * 0.5f)));
            let _e1630 = (_e805 - _e529);
            switch bitcast<i32>(_e1604) {
                case 0: {
                    phi_6362_ = vec2<f32>();
                    phi_6363_ = true;
                    break;
                }
                case 1: {
                    phi_6362_ = vec2<f32>();
                    phi_6363_ = true;
                    break;
                }
                default: {
                    phi_6362_ = vec2<f32>(0f, 0f);
                    phi_6363_ = false;
                    break;
                }
            }
            let _e1633 = phi_6362_;
            let _e1635 = phi_6363_;
            if _e1635 {
                let _e1636 = (_e804 - 52f);
                let _e1641 = pill_1.member[_e497].cpu.temperature;
                if (_e1641 <= 62f) {
                    phi_6509_ = vec2<f32>(0f, 0f);
                } else {
                    let _e1644 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1636, _e1630), 13f, 13f);
                    phi_19016_ = 0i;
                    phi_19017_ = 0.5f;
                    phi_19018_ = 0f;
                    phi_19019_ = vec2<f32>(((_e1636 + (_e860 * 1.8f)) * 0.035f), (((_e1630 + -(_e860)) * 0.035f) + 6.1f));
                    loop {
                        let _e1654 = phi_19016_;
                        let _e1656 = phi_19017_;
                        let _e1658 = phi_19018_;
                        let _e1660 = phi_19019_;
                        local_36 = _e1658;
                        let _e1661 = (_e1654 < 4i);
                        if _e1661 {
                            let _e1664 = cantus_render_shader_simplex_noise(_e1660);
                            phi_19044_ = (_e1654 + 1i);
                            phi_19045_ = (_e1656 * 0.5f);
                            phi_19046_ = (_e1658 + (_e1664 * _e1656));
                            phi_19047_ = vec2<f32>(((_e1660.x * 1.6f) + (_e1660.y * 1.2f)), ((_e1660.y * 1.6f) - (_e1660.x * 1.2f)));
                        } else {
                            phi_19044_ = i32();
                            phi_19045_ = f32();
                            phi_19046_ = f32();
                            phi_19047_ = vec2<f32>();
                        }
                        let _e1677 = phi_19044_;
                        let _e1679 = phi_19045_;
                        let _e1681 = phi_19046_;
                        let _e1683 = phi_19047_;
                        continue;
                        continuing {
                            phi_19016_ = _e1677;
                            phi_19017_ = _e1679;
                            phi_19018_ = _e1681;
                            phi_19019_ = _e1683;
                            break if !(_e1661);
                        }
                    }
                    let _e1686 = local_36;
                    let _e1687 = (_e1686 * 0.5f);
                    let _e1690 = ((_e1644 - -0.5f) * 0.5f);
                    let _e1692 = select(_e1690, 0f, (_e1690 < 0f));
                    let _e1694 = select(_e1692, 1f, (_e1692 > 1f));
                    let _e1700 = ((_e1644 - 14f) * -0.083333336f);
                    let _e1702 = select(_e1700, 0f, (_e1700 < 0f));
                    let _e1704 = select(_e1702, 1f, (_e1702 > 1f));
                    let _e1709 = (((_e1694 * _e1694) * (3f - (2f * _e1694))) * ((_e1704 * _e1704) * (3f - (2f * _e1704))));
                    let _e1714 = ((_e1687 + 0.19999999f) * 3.125f);
                    let _e1716 = select(_e1714, 0f, (_e1714 < 0f));
                    let _e1718 = select(_e1716, 1f, (_e1716 > 1f));
                    let _e1725 = ((_e1641 - 62f) * 0.045454547f);
                    let _e1727 = select(_e1725, 0f, (_e1725 < 0f));
                    let _e1729 = select(_e1727, 1f, (_e1727 > 1f));
                    let _e1733 = ((_e1729 * _e1729) * (3f - (2f * _e1729)));
                    phi_6509_ = vec2<f32>(((_e1709 * (0.18f + ((0.5f + _e1687) * 0.34f))) * _e1733), ((_e1709 * ((_e1718 * _e1718) * (3f - (2f * _e1718)))) * _e1733));
                }
                let _e1738 = phi_6509_;
                let _e1741 = (_e804 - 140f);
                let _e1746 = pill_1.member[_e497].gpu.temperature;
                if (_e1746 <= 62f) {
                    phi_6654_ = vec2<f32>(0f, 0f);
                } else {
                    let _e1749 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1741, _e1630), 13f, 13f);
                    phi_19098_ = 0i;
                    phi_19099_ = 0.5f;
                    phi_19100_ = 0f;
                    phi_19101_ = vec2<f32>(((_e1741 + (_e860 * 1.8f)) * 0.035f), (((_e1630 + -(_e860)) * 0.035f) + 6.1f));
                    loop {
                        let _e1759 = phi_19098_;
                        let _e1761 = phi_19099_;
                        let _e1763 = phi_19100_;
                        let _e1765 = phi_19101_;
                        local_37 = _e1763;
                        let _e1766 = (_e1759 < 4i);
                        if _e1766 {
                            let _e1769 = cantus_render_shader_simplex_noise(_e1765);
                            phi_19126_ = (_e1759 + 1i);
                            phi_19127_ = (_e1761 * 0.5f);
                            phi_19128_ = (_e1763 + (_e1769 * _e1761));
                            phi_19129_ = vec2<f32>(((_e1765.x * 1.6f) + (_e1765.y * 1.2f)), ((_e1765.y * 1.6f) - (_e1765.x * 1.2f)));
                        } else {
                            phi_19126_ = i32();
                            phi_19127_ = f32();
                            phi_19128_ = f32();
                            phi_19129_ = vec2<f32>();
                        }
                        let _e1782 = phi_19126_;
                        let _e1784 = phi_19127_;
                        let _e1786 = phi_19128_;
                        let _e1788 = phi_19129_;
                        continue;
                        continuing {
                            phi_19098_ = _e1782;
                            phi_19099_ = _e1784;
                            phi_19100_ = _e1786;
                            phi_19101_ = _e1788;
                            break if !(_e1766);
                        }
                    }
                    let _e1791 = local_37;
                    let _e1792 = (_e1791 * 0.5f);
                    let _e1795 = ((_e1749 - -0.5f) * 0.5f);
                    let _e1797 = select(_e1795, 0f, (_e1795 < 0f));
                    let _e1799 = select(_e1797, 1f, (_e1797 > 1f));
                    let _e1805 = ((_e1749 - 14f) * -0.083333336f);
                    let _e1807 = select(_e1805, 0f, (_e1805 < 0f));
                    let _e1809 = select(_e1807, 1f, (_e1807 > 1f));
                    let _e1814 = (((_e1799 * _e1799) * (3f - (2f * _e1799))) * ((_e1809 * _e1809) * (3f - (2f * _e1809))));
                    let _e1819 = ((_e1792 + 0.19999999f) * 3.125f);
                    let _e1821 = select(_e1819, 0f, (_e1819 < 0f));
                    let _e1823 = select(_e1821, 1f, (_e1821 > 1f));
                    let _e1830 = ((_e1746 - 62f) * 0.045454547f);
                    let _e1832 = select(_e1830, 0f, (_e1830 < 0f));
                    let _e1834 = select(_e1832, 1f, (_e1832 > 1f));
                    let _e1838 = ((_e1834 * _e1834) * (3f - (2f * _e1834)));
                    phi_6654_ = vec2<f32>(((_e1814 * (0.18f + ((0.5f + _e1792) * 0.34f))) * _e1838), ((_e1814 * ((_e1823 * _e1823) * (3f - (2f * _e1823)))) * _e1838));
                }
                let _e1843 = phi_6654_;
                phi_6668_ = vec2<f32>(select(_e1843.x, _e1738.x, (_e1738.x > _e1843.x)), select(_e1843.y, _e1738.y, (_e1738.y > _e1843.y)));
            } else {
                phi_6668_ = _e1633;
            }
            let _e1852 = phi_6668_;
            let _e1857 = pill_1.member[_e497].cpu.temperature;
            let _e1862 = pill_1.member[_e497].gpu.temperature;
            if (_e1857 != _e1857) {
                phi_19144_ = true;
            } else {
                phi_19144_ = (_e1862 >= _e1857);
            }
            let _e1866 = phi_19144_;
            let _e1867 = select(_e1857, _e1862, _e1866);
            let _e1869 = ((_e1867 - 60f) * 0.083333336f);
            let _e1871 = select(_e1869, 0f, (_e1869 < 0f));
            let _e1873 = select(_e1871, 1f, (_e1871 > 1f));
            let _e1877 = ((_e1873 * _e1873) * (3f - (2f * _e1873)));
            let _e1878 = (1f - _e1877);
            let _e1887 = ((_e1867 - 72f) * 0.0625f);
            let _e1889 = select(_e1887, 0f, (_e1887 < 0f));
            let _e1891 = select(_e1889, 1f, (_e1889 > 1f));
            let _e1895 = ((_e1891 * _e1891) * (3f - (2f * _e1891)));
            let _e1896 = (1f - _e1895);
            let _e1906 = (_e1852.y * 0.12f);
            let _e1907 = (0.24f + _e1906);
            let _e1908 = (0.76f - _e1906);
            let _e1920 = (1f - (_e1852.x * 0.46f));
            let _e1930 = (_e1852.y * 0.64f);
            let _e1931 = (1f - _e1930);
            let _e1938 = (((((_e1557.x + _e1568) * _e1920) + (_e1852.x * 0.0009200001f)) * _e1931) + (((0.07f * _e1908) + (((((0.22f * _e1878) + _e1877) * _e1896) + _e1895) * _e1907)) * _e1930));
            let _e1939 = (((((_e1557.y + _e1568) * _e1920) + (_e1852.x * 0.00276f)) * _e1931) + (((0.12f * _e1908) + (((((0.62f * _e1878) + (0.38f * _e1877)) * _e1896) + (0.08f * _e1895)) * _e1907)) * _e1930));
            let _e1940 = (((((_e1557.z + _e1568) * _e1920) + (_e1852.x * 0.00552f)) * _e1931) + (((0.18f * _e1908) + ((((_e1878 + (0.08f * _e1877)) * _e1896) + (0.035f * _e1895)) * _e1907)) * _e1930));
            switch bitcast<i32>(_e1604) {
                case 0: {
                    let _e2656 = pill_1.member[_e497].history_scroll;
                    switch bitcast<i32>(_e1604) {
                        case 0: {
                            phi_19231_ = true;
                            phi_19232_ = false;
                            phi_19233_ = false;
                            break;
                        }
                        case 1: {
                            phi_19231_ = true;
                            phi_19232_ = false;
                            phi_19233_ = false;
                            break;
                        }
                        case 2: {
                            phi_19231_ = false;
                            phi_19232_ = true;
                            phi_19233_ = false;
                            break;
                        }
                        case 3: {
                            phi_19231_ = false;
                            phi_19232_ = true;
                            phi_19233_ = false;
                            break;
                        }
                        case 4: {
                            phi_19231_ = false;
                            phi_19232_ = false;
                            phi_19233_ = true;
                            break;
                        }
                        case 5: {
                            phi_19231_ = false;
                            phi_19232_ = false;
                            phi_19233_ = true;
                            break;
                        }
                        default: {
                            phi_19231_ = bool();
                            phi_19232_ = bool();
                            phi_19233_ = bool();
                            break;
                        }
                    }
                    let _e2659 = phi_19231_;
                    let _e2661 = phi_19232_;
                    let _e2663 = phi_19233_;
                    let _e2664 = select(_e2661, false, _e2659);
                    let _e2670 = ((select(select(80f, 32f, _e2664), 24f, select(select(_e2663, false, _e2659), false, _e2664)) * 0.5f) - 4f);
                    let _e2671 = (_e529 - 8f);
                    let _e2672 = (_e2670 - _e2671);
                    let _e2674 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1629, _e1630), _e2672, _e2671);
                    let _e2675 = abs(_e1629);
                    let _e2676 = abs(_e1630);
                    let _e2679 = (round((_e2675 * 0.11111111f)) * 9f);
                    if (_e2679 != _e2679) {
                        phi_19337_ = true;
                    } else {
                        phi_19337_ = (_e2670 <= _e2679);
                    }
                    let _e2683 = phi_19337_;
                    let _e2684 = select(_e2679, _e2670, _e2683);
                    let _e2685 = (_e2684 - _e2672);
                    if (_e2685 != _e2685) {
                        phi_19352_ = true;
                    } else {
                        phi_19352_ = (0f >= _e2685);
                    }
                    let _e2689 = phi_19352_;
                    let _e2690 = select(_e2685, 0f, _e2689);
                    let _e2691 = (_e2671 * _e2671);
                    let _e2694 = sqrt((_e2691 - (_e2690 * _e2690)));
                    let _e2695 = (_e2690 / _e2671);
                    let _e2696 = (_e2694 / _e2671);
                    let _e2701 = ((_e2675 - _e2684) - (_e2695 * 0.9f));
                    let _e2702 = ((_e2676 - _e2694) - (_e2696 * 0.9f));
                    let _e2711 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e2701 * -(_e2696)) + (_e2702 * _e2695)), ((_e2701 * _e2695) + (_e2702 * _e2696))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e2713 = round((_e2676 * 0.125f));
                    if (_e2713 != _e2713) {
                        phi_19367_ = true;
                    } else {
                        phi_19367_ = (1f <= _e2713);
                    }
                    let _e2717 = phi_19367_;
                    let _e2719 = (select(_e2713, 1f, _e2717) * 8f);
                    let _e2722 = sqrt((_e2691 - (_e2719 * _e2719)));
                    let _e2724 = (_e2722 / _e2671);
                    let _e2725 = (_e2719 / _e2671);
                    let _e2730 = ((_e2675 - (_e2672 + _e2722)) - (_e2724 * 0.9f));
                    let _e2731 = ((_e2676 - _e2719) - (_e2725 * 0.9f));
                    let _e2740 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e2730 * -(_e2725)) + (_e2731 * _e2724)), ((_e2730 * _e2724) + (_e2731 * _e2725))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e2711 != _e2711) {
                        phi_19382_ = true;
                    } else {
                        phi_19382_ = (_e2740 <= _e2711);
                    }
                    let _e2744 = phi_19382_;
                    let _e2745 = select(_e2711, _e2740, _e2744);
                    let _e2748 = (0.5f + ((_e2745 - _e2674) * 0.3125f));
                    let _e2750 = select(_e2748, 0f, (_e2748 < 0f));
                    let _e2752 = select(_e2750, 1f, (_e2750 > 1f));
                    let _e2761 = ((_e2674 - 0.55f) * -0.9090909f);
                    let _e2763 = select(_e2761, 0f, (_e2761 < 0f));
                    let _e2765 = select(_e2763, 1f, (_e2763 > 1f));
                    let _e2769 = ((_e2765 * _e2765) * (3f - (2f * _e2765)));
                    let _e2770 = (_e2670 * 0.051282052f);
                    let _e2771 = (_e1629 + _e2670);
                    let _e2773 = ((_e2771 / _e2770) + _e2656);
                    let _e2775 = select(_e2773, 0f, (_e2773 < 0f));
                    let _e2777 = select(_e2775, 39f, (_e2775 > 39f));
                    let _e2778 = floor(_e2777);
                    let _e2783 = select(select(u32(_e2778), 0u, (_e2778 < 0f)), 4294967295u, (_e2778 > 4294967000f));
                    let _e2784 = (_e529 - 10f);
                    let _e2788 = (((f32(_e2783) - _e2656) * _e2770) - _e2670);
                    let _e2790 = select(_e2783, 39u, (39u < _e2783));
                    let _e2791 = (_e2790 < 40u);
                    if _e2791 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2798 = pill_1.member[_e497].cpu.usage.samples[_e2790];
                    let _e2801 = (_e2784 * (1f - (_e2798 * 2f)));
                    let _e2802 = (_e2783 + 1u);
                    let _e2808 = select(_e2802, 39u, (39u < _e2802));
                    let _e2809 = (_e2808 < 40u);
                    if _e2809 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2816 = pill_1.member[_e497].cpu.usage.samples[_e2808];
                    let _e2820 = ((((f32(_e2802) - _e2656) * _e2770) - _e2670) - _e2788);
                    let _e2821 = ((_e2784 * (1f - (_e2816 * 2f))) - _e2801);
                    let _e2822 = (_e1629 - _e2788);
                    let _e2823 = (_e1630 - _e2801);
                    let _e2824 = (_e2822 * _e2820);
                    let _e2827 = (_e2820 * _e2820);
                    let _e2829 = (_e2827 + (_e2821 * _e2821));
                    if (_e2829 != _e2829) {
                        phi_19397_ = true;
                    } else {
                        phi_19397_ = (0.001f >= _e2829);
                    }
                    let _e2833 = phi_19397_;
                    let _e2835 = ((_e2824 + (_e2823 * _e2821)) / select(_e2829, 0.001f, _e2833));
                    let _e2837 = select(_e2835, 0f, (_e2835 < 0f));
                    let _e2839 = select(_e2837, 1f, (_e2837 > 1f));
                    let _e2842 = (_e2822 - (_e2820 * _e2839));
                    let _e2843 = (_e2823 - (_e2821 * _e2839));
                    let _e2850 = ((abs(sqrt(((_e2842 * _e2842) + (_e2843 * _e2843)))) - 1.4000001f) * -0.9090908f);
                    let _e2852 = select(_e2850, 0f, (_e2850 < 0f));
                    let _e2854 = select(_e2852, 1f, (_e2852 > 1f));
                    let _e2860 = (_e2777 - trunc(_e2777));
                    let _e2862 = select(_e2860, 0f, (_e2860 < 0f));
                    let _e2864 = select(_e2862, 1f, (_e2862 > 1f));
                    let _e2868 = ((_e2864 * _e2864) * (3f - (2f * _e2864)));
                    let _e2875 = ((((_e2801 + (_e2821 * _e2868)) - _e1630) - 0.55f) * -0.9090909f);
                    let _e2877 = select(_e2875, 0f, (_e2875 < 0f));
                    let _e2879 = select(_e2877, 1f, (_e2877 > 1f));
                    let _e2885 = ((((_e2879 * _e2879) * (3f - (2f * _e2879))) * 0.156f) + ((_e2854 * _e2854) * (3f - (2f * _e2854))));
                    if _e2791 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2894 = pill_1.member[_e497].cpu.memory.samples[_e2790];
                    let _e2897 = (_e2784 * (1f - (_e2894 * 2f)));
                    if _e2809 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2904 = pill_1.member[_e497].cpu.memory.samples[_e2808];
                    let _e2908 = ((_e2784 * (1f - (_e2904 * 2f))) - _e2897);
                    let _e2909 = (_e1630 - _e2897);
                    let _e2913 = (_e2827 + (_e2908 * _e2908));
                    if (_e2913 != _e2913) {
                        phi_19412_ = true;
                    } else {
                        phi_19412_ = (0.001f >= _e2913);
                    }
                    let _e2917 = phi_19412_;
                    let _e2919 = ((_e2824 + (_e2909 * _e2908)) / select(_e2913, 0.001f, _e2917));
                    let _e2921 = select(_e2919, 0f, (_e2919 < 0f));
                    let _e2923 = select(_e2921, 1f, (_e2921 > 1f));
                    let _e2926 = (_e2822 - (_e2820 * _e2923));
                    let _e2927 = (_e2909 - (_e2908 * _e2923));
                    let _e2934 = ((abs(sqrt(((_e2926 * _e2926) + (_e2927 * _e2927)))) - 1.4000001f) * -0.9090908f);
                    let _e2936 = select(_e2934, 0f, (_e2934 < 0f));
                    let _e2938 = select(_e2936, 1f, (_e2936 > 1f));
                    let _e2949 = ((((_e2897 + (_e2908 * _e2868)) - _e1630) - 0.55f) * -0.9090909f);
                    let _e2951 = select(_e2949, 0f, (_e2949 < 0f));
                    let _e2953 = select(_e2951, 1f, (_e2951 > 1f));
                    let _e2959 = ((((_e2953 * _e2953) * (3f - (2f * _e2953))) * 0.084f) + ((_e2938 * _e2938) * (3f - (2f * _e2938))));
                    let _e2967 = (_e2771 * 0.14285715f);
                    let _e2968 = ((_e1630 + _e2671) * 0.16393442f);
                    let _e2978 = ((abs(((_e2967 - trunc(_e2967)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2980 = select(_e2978, 0f, (_e2978 < 0f));
                    let _e2982 = select(_e2980, 1f, (_e2980 > 1f));
                    let _e2986 = ((_e2982 * _e2982) * (3f - (2f * _e2982)));
                    let _e2988 = ((abs(((_e2968 - trunc(_e2968)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2990 = select(_e2988, 0f, (_e2988 < 0f));
                    let _e2992 = select(_e2990, 1f, (_e2990 > 1f));
                    let _e2996 = ((_e2992 * _e2992) * (3f - (2f * _e2992)));
                    if (_e2986 != _e2986) {
                        phi_19427_ = true;
                    } else {
                        phi_19427_ = (_e2996 >= _e2986);
                    }
                    let _e3000 = phi_19427_;
                    let _e3008 = pill_1.member[_e497].cpu.usage.samples[39u];
                    let _e3009 = (_e3008 * 0.24f);
                    let _e3010 = (0.18f + _e3009);
                    let _e3011 = (0.82f - _e3009);
                    let _e3020 = (_e1857 - 60f);
                    let _e3021 = (_e3020 * 0.083333336f);
                    let _e3023 = select(_e3021, 0f, (_e3021 < 0f));
                    let _e3025 = select(_e3023, 1f, (_e3023 > 1f));
                    let _e3029 = ((_e3025 * _e3025) * (3f - (2f * _e3025)));
                    let _e3030 = (1f - _e3029);
                    let _e3039 = ((_e1857 - 72f) * 0.0625f);
                    let _e3041 = select(_e3039, 0f, (_e3039 < 0f));
                    let _e3043 = select(_e3041, 1f, (_e3041 > 1f));
                    let _e3047 = ((_e3043 * _e3043) * (3f - (2f * _e3043)));
                    let _e3048 = (1f - _e3047);
                    let _e3057 = (_e3020 * 0.03846154f);
                    let _e3059 = select(_e3057, 0f, (_e3057 < 0f));
                    let _e3061 = select(_e3059, 1f, (_e3059 > 1f));
                    let _e3066 = (((_e3061 * _e3061) * (3f - (2f * _e3061))) * 0.9f);
                    let _e3067 = (1f - _e3066);
                    let _e3074 = ((((0.025f * _e3011) + (0.32f * _e3010)) * _e3067) + (((((0.22f * _e3030) + _e3029) * _e3048) + _e3047) * _e3066));
                    let _e3075 = ((((0.09f * _e3011) + (0.68f * _e3010)) * _e3067) + (((((0.62f * _e3030) + (0.38f * _e3029)) * _e3048) + (0.08f * _e3047)) * _e3066));
                    let _e3076 = ((((0.15f * _e3011) + _e3010) * _e3067) + ((((_e3030 + (0.08f * _e3029)) * _e3048) + (0.035f * _e3047)) * _e3066));
                    let _e3078 = ((((_e2745 + ((_e2674 - _e2745) * _e2752)) - ((1.6f * _e2752) * (1f - _e2752))) - 0.55f) * -0.9090909f);
                    let _e3080 = select(_e3078, 0f, (_e3078 < 0f));
                    let _e3082 = select(_e3080, 1f, (_e3080 > 1f));
                    let _e3086 = ((_e3082 * _e3082) * (3f - (2f * _e3082)));
                    let _e3088 = (1f - (_e3086 * 0.82f));
                    let _e3100 = ((abs(_e2674) - 2.1f) * -0.909091f);
                    let _e3102 = select(_e3100, 0f, (_e3100 < 0f));
                    let _e3104 = select(_e3102, 1f, (_e3102 > 1f));
                    let _e3109 = (((_e3104 * _e3104) * (3f - (2f * _e3104))) * 0.92f);
                    let _e3110 = (1f - _e3109);
                    let _e3121 = ((_e2745 - 0.55f) * -0.9090909f);
                    let _e3123 = select(_e3121, 0f, (_e3121 < 0f));
                    let _e3125 = select(_e3123, 1f, (_e3123 > 1f));
                    let _e3130 = (((_e3125 * _e3125) * (3f - (2f * _e3125))) * 0.78f);
                    let _e3131 = (1f - _e3130);
                    let _e3142 = ((_e2769 * select(_e2986, _e2996, _e3000)) * 0.045f);
                    phi_21803_ = _e676;
                    phi_8759_ = vec3<f32>(((((((((_e1938 * _e3088) + (_e3086 * 0.00328f)) * _e3110) + (_e3074 * _e3109)) * _e3131) + (_e3074 * _e3130)) + _e3142) + (((0.32f * _e2769) * _e2885) + ((0.78f * _e2769) * _e2959))), ((((((((_e1939 * _e3088) + (_e3086 * 0.00984f)) * _e3110) + (_e3075 * _e3109)) * _e3131) + (_e3075 * _e3130)) + _e3142) + (((0.68f * _e2769) * _e2885) + ((0.3f * _e2769) * _e2959))), ((((((((_e1940 * _e3088) + (_e3086 * 0.02132f)) * _e3110) + (_e3076 * _e3109)) * _e3131) + (_e3076 * _e3130)) + _e3142) + (_e2769 * (_e2885 + _e2959))));
                    phi_8760_ = false;
                    break;
                }
                case 1: {
                    let _e2275 = pill_1.member[_e497].history_scroll;
                    switch bitcast<i32>(_e1604) {
                        case 0: {
                            phi_19159_ = true;
                            phi_19160_ = false;
                            phi_19161_ = false;
                            break;
                        }
                        case 1: {
                            phi_19159_ = true;
                            phi_19160_ = false;
                            phi_19161_ = false;
                            break;
                        }
                        case 2: {
                            phi_19159_ = false;
                            phi_19160_ = true;
                            phi_19161_ = false;
                            break;
                        }
                        case 3: {
                            phi_19159_ = false;
                            phi_19160_ = true;
                            phi_19161_ = false;
                            break;
                        }
                        case 4: {
                            phi_19159_ = false;
                            phi_19160_ = false;
                            phi_19161_ = true;
                            break;
                        }
                        case 5: {
                            phi_19159_ = false;
                            phi_19160_ = false;
                            phi_19161_ = true;
                            break;
                        }
                        default: {
                            phi_19159_ = bool();
                            phi_19160_ = bool();
                            phi_19161_ = bool();
                            break;
                        }
                    }
                    let _e2278 = phi_19159_;
                    let _e2280 = phi_19160_;
                    let _e2282 = phi_19161_;
                    let _e2283 = select(_e2280, false, _e2278);
                    let _e2289 = ((select(select(80f, 32f, _e2283), 24f, select(select(_e2282, false, _e2278), false, _e2283)) * 0.5f) - 4f);
                    let _e2290 = (_e529 - 8f);
                    let _e2293 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1629, _e1630), (_e2289 - _e2290), _e2290);
                    let _e2295 = ((_e2293 - 0.55f) * -0.9090909f);
                    let _e2297 = select(_e2295, 0f, (_e2295 < 0f));
                    let _e2299 = select(_e2297, 1f, (_e2297 > 1f));
                    let _e2303 = ((_e2299 * _e2299) * (3f - (2f * _e2299)));
                    let _e2304 = (_e2289 * 0.051282052f);
                    let _e2305 = (_e1629 + _e2289);
                    let _e2307 = ((_e2305 / _e2304) + _e2275);
                    let _e2309 = select(_e2307, 0f, (_e2307 < 0f));
                    let _e2311 = select(_e2309, 39f, (_e2309 > 39f));
                    let _e2312 = floor(_e2311);
                    let _e2317 = select(select(u32(_e2312), 0u, (_e2312 < 0f)), 4294967295u, (_e2312 > 4294967000f));
                    let _e2318 = (_e529 - 10f);
                    let _e2322 = (((f32(_e2317) - _e2275) * _e2304) - _e2289);
                    let _e2324 = select(_e2317, 39u, (39u < _e2317));
                    let _e2325 = (_e2324 < 40u);
                    if _e2325 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2332 = pill_1.member[_e497].gpu.usage.samples[_e2324];
                    let _e2335 = (_e2318 * (1f - (_e2332 * 2f)));
                    let _e2336 = (_e2317 + 1u);
                    let _e2342 = select(_e2336, 39u, (39u < _e2336));
                    let _e2343 = (_e2342 < 40u);
                    if _e2343 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2350 = pill_1.member[_e497].gpu.usage.samples[_e2342];
                    let _e2354 = ((((f32(_e2336) - _e2275) * _e2304) - _e2289) - _e2322);
                    let _e2355 = ((_e2318 * (1f - (_e2350 * 2f))) - _e2335);
                    let _e2356 = (_e1629 - _e2322);
                    let _e2357 = (_e1630 - _e2335);
                    let _e2358 = (_e2356 * _e2354);
                    let _e2361 = (_e2354 * _e2354);
                    let _e2363 = (_e2361 + (_e2355 * _e2355));
                    if (_e2363 != _e2363) {
                        phi_19186_ = true;
                    } else {
                        phi_19186_ = (0.001f >= _e2363);
                    }
                    let _e2367 = phi_19186_;
                    let _e2369 = ((_e2358 + (_e2357 * _e2355)) / select(_e2363, 0.001f, _e2367));
                    let _e2371 = select(_e2369, 0f, (_e2369 < 0f));
                    let _e2373 = select(_e2371, 1f, (_e2371 > 1f));
                    let _e2376 = (_e2356 - (_e2354 * _e2373));
                    let _e2377 = (_e2357 - (_e2355 * _e2373));
                    let _e2384 = ((abs(sqrt(((_e2376 * _e2376) + (_e2377 * _e2377)))) - 1.4000001f) * -0.9090908f);
                    let _e2386 = select(_e2384, 0f, (_e2384 < 0f));
                    let _e2388 = select(_e2386, 1f, (_e2386 > 1f));
                    let _e2394 = (_e2311 - trunc(_e2311));
                    let _e2396 = select(_e2394, 0f, (_e2394 < 0f));
                    let _e2398 = select(_e2396, 1f, (_e2396 > 1f));
                    let _e2402 = ((_e2398 * _e2398) * (3f - (2f * _e2398)));
                    let _e2409 = ((((_e2335 + (_e2355 * _e2402)) - _e1630) - 0.55f) * -0.9090909f);
                    let _e2411 = select(_e2409, 0f, (_e2409 < 0f));
                    let _e2413 = select(_e2411, 1f, (_e2411 > 1f));
                    let _e2419 = ((((_e2413 * _e2413) * (3f - (2f * _e2413))) * 0.156f) + ((_e2388 * _e2388) * (3f - (2f * _e2388))));
                    if _e2325 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2428 = pill_1.member[_e497].gpu.memory.samples[_e2324];
                    let _e2431 = (_e2318 * (1f - (_e2428 * 2f)));
                    if _e2343 {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e2438 = pill_1.member[_e497].gpu.memory.samples[_e2342];
                    let _e2442 = ((_e2318 * (1f - (_e2438 * 2f))) - _e2431);
                    let _e2443 = (_e1630 - _e2431);
                    let _e2447 = (_e2361 + (_e2442 * _e2442));
                    if (_e2447 != _e2447) {
                        phi_19201_ = true;
                    } else {
                        phi_19201_ = (0.001f >= _e2447);
                    }
                    let _e2451 = phi_19201_;
                    let _e2453 = ((_e2358 + (_e2443 * _e2442)) / select(_e2447, 0.001f, _e2451));
                    let _e2455 = select(_e2453, 0f, (_e2453 < 0f));
                    let _e2457 = select(_e2455, 1f, (_e2455 > 1f));
                    let _e2460 = (_e2356 - (_e2354 * _e2457));
                    let _e2461 = (_e2443 - (_e2442 * _e2457));
                    let _e2468 = ((abs(sqrt(((_e2460 * _e2460) + (_e2461 * _e2461)))) - 1.4000001f) * -0.9090908f);
                    let _e2470 = select(_e2468, 0f, (_e2468 < 0f));
                    let _e2472 = select(_e2470, 1f, (_e2470 > 1f));
                    let _e2483 = ((((_e2431 + (_e2442 * _e2402)) - _e1630) - 0.55f) * -0.9090909f);
                    let _e2485 = select(_e2483, 0f, (_e2483 < 0f));
                    let _e2487 = select(_e2485, 1f, (_e2485 > 1f));
                    let _e2493 = ((((_e2487 * _e2487) * (3f - (2f * _e2487))) * 0.084f) + ((_e2472 * _e2472) * (3f - (2f * _e2472))));
                    let _e2501 = (_e2305 * 0.14285715f);
                    let _e2502 = ((_e1630 + _e2290) * 0.16393442f);
                    let _e2512 = ((abs(((_e2501 - trunc(_e2501)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2514 = select(_e2512, 0f, (_e2512 < 0f));
                    let _e2516 = select(_e2514, 1f, (_e2514 > 1f));
                    let _e2520 = ((_e2516 * _e2516) * (3f - (2f * _e2516)));
                    let _e2522 = ((abs(((_e2502 - trunc(_e2502)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2524 = select(_e2522, 0f, (_e2522 < 0f));
                    let _e2526 = select(_e2524, 1f, (_e2524 > 1f));
                    let _e2530 = ((_e2526 * _e2526) * (3f - (2f * _e2526)));
                    if (_e2520 != _e2520) {
                        phi_19216_ = true;
                    } else {
                        phi_19216_ = (_e2530 >= _e2520);
                    }
                    let _e2534 = phi_19216_;
                    let _e2542 = pill_1.member[_e497].gpu.usage.samples[39u];
                    let _e2543 = (_e2542 * 0.24f);
                    let _e2544 = (0.18f + _e2543);
                    let _e2545 = (0.82f - _e2543);
                    let _e2554 = (_e1862 - 60f);
                    let _e2555 = (_e2554 * 0.083333336f);
                    let _e2557 = select(_e2555, 0f, (_e2555 < 0f));
                    let _e2559 = select(_e2557, 1f, (_e2557 > 1f));
                    let _e2563 = ((_e2559 * _e2559) * (3f - (2f * _e2559)));
                    let _e2564 = (1f - _e2563);
                    let _e2573 = ((_e1862 - 72f) * 0.0625f);
                    let _e2575 = select(_e2573, 0f, (_e2573 < 0f));
                    let _e2577 = select(_e2575, 1f, (_e2575 > 1f));
                    let _e2581 = ((_e2577 * _e2577) * (3f - (2f * _e2577)));
                    let _e2582 = (1f - _e2581);
                    let _e2591 = (_e2554 * 0.03846154f);
                    let _e2593 = select(_e2591, 0f, (_e2591 < 0f));
                    let _e2595 = select(_e2593, 1f, (_e2593 > 1f));
                    let _e2600 = (((_e2595 * _e2595) * (3f - (2f * _e2595))) * 0.9f);
                    let _e2601 = (1f - _e2600);
                    let _e2612 = (1f - (_e2303 * 0.82f));
                    let _e2624 = ((abs(_e2293) - 2.1f) * -0.909091f);
                    let _e2626 = select(_e2624, 0f, (_e2624 < 0f));
                    let _e2628 = select(_e2626, 1f, (_e2626 > 1f));
                    let _e2633 = (((_e2628 * _e2628) * (3f - (2f * _e2628))) * 0.92f);
                    let _e2634 = (1f - _e2633);
                    let _e2645 = ((_e2303 * select(_e2520, _e2530, _e2534)) * 0.045f);
                    phi_21803_ = _e676;
                    phi_8759_ = vec3<f32>(((((((_e1938 * _e2612) + (_e2303 * 0.00328f)) * _e2634) + (((((0.025f * _e2545) + (0.32f * _e2544)) * _e2601) + (((((0.22f * _e2564) + _e2563) * _e2582) + _e2581) * _e2600)) * _e2633)) + _e2645) + (((0.32f * _e2303) * _e2419) + ((0.78f * _e2303) * _e2493))), ((((((_e1939 * _e2612) + (_e2303 * 0.00984f)) * _e2634) + (((((0.09f * _e2545) + (0.68f * _e2544)) * _e2601) + (((((0.62f * _e2564) + (0.38f * _e2563)) * _e2582) + (0.08f * _e2581)) * _e2600)) * _e2633)) + _e2645) + (((0.68f * _e2303) * _e2419) + ((0.3f * _e2303) * _e2493))), ((((((_e1940 * _e2612) + (_e2303 * 0.02132f)) * _e2634) + (((((0.15f * _e2545) + _e2544) * _e2601) + ((((_e2564 + (0.08f * _e2563)) * _e2582) + (0.035f * _e2581)) * _e2600)) * _e2633)) + _e2645) + (_e2303 * (_e2419 + _e2493))));
                    phi_8760_ = false;
                    break;
                }
                case 2: {
                    let _e2067 = (_e1629 * 1.25f);
                    let _e2068 = (_e1630 * 1.25f);
                    let _e2070 = select(0f, 1f, (_e508 < 0f));
                    let _e2071 = abs(_e508);
                    let _e2072 = (_e2068 - 1f);
                    let _e2073 = vec2<f32>(_e2067, _e2072);
                    let _e2074 = cantus_render_shader_sd_rounded_box(_e2073, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e2077 = ((abs(_e2074) - 2.425f) * -0.909091f);
                    let _e2079 = select(_e2077, 0f, (_e2077 < 0f));
                    let _e2081 = select(_e2079, 1f, (_e2079 > 1f));
                    let _e2088 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2067, (_e2068 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e2090 = ((_e2088 - 0.55f) * -0.9090909f);
                    let _e2092 = select(_e2090, 0f, (_e2090 < 0f));
                    let _e2094 = select(_e2092, 1f, (_e2092 > 1f));
                    let _e2099 = cantus_render_shader_sd_rounded_box(_e2073, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e2101 = ((_e2099 - 0.55f) * -0.9090909f);
                    let _e2103 = select(_e2101, 0f, (_e2101 < 0f));
                    let _e2105 = select(_e2103, 1f, (_e2103 > 1f));
                    let _e2109 = ((_e2105 * _e2105) * (3f - (2f * _e2105)));
                    let _e2111 = select(_e2071, 0f, (_e2071 < 0f));
                    let _e2129 = ((12f - (select(_e2111, 1f, (_e2111 > 1f)) * 24f)) + ((sin(((_e1629 * 0.775f) + (_e860 * (1.4f + (_e2070 * 1.2f))))) * 1.15f) + (sin(((_e1629 * 0.3375f) - (_e860 * 0.8f))) * 0.45f)));
                    let _e2130 = (_e2129 - 0.7f);
                    let _e2134 = ((_e2072 - _e2130) / ((_e2129 + 0.7f) - _e2130));
                    let _e2136 = select(_e2134, 0f, (_e2134 < 0f));
                    let _e2138 = select(_e2136, 1f, (_e2136 > 1f));
                    let _e2143 = (_e2109 * ((_e2138 * _e2138) * (3f - (2f * _e2138))));
                    let _e2145 = ((_e2071 - 0.08f) * 5f);
                    let _e2147 = select(_e2145, 0f, (_e2145 < 0f));
                    let _e2149 = select(_e2147, 1f, (_e2147 > 1f));
                    let _e2153 = ((_e2149 * _e2149) * (3f - (2f * _e2149)));
                    let _e2154 = (1f - _e2153);
                    let _e2162 = ((_e2071 - 0.18f) * 1.8518518f);
                    let _e2164 = select(_e2162, 0f, (_e2162 < 0f));
                    let _e2166 = select(_e2164, 1f, (_e2164 > 1f));
                    let _e2170 = ((_e2166 * _e2166) * (3f - (2f * _e2166)));
                    let _e2171 = (1f - _e2170);
                    let _e2177 = (_e2171 + (0.22f * _e2170));
                    let _e2178 = ((((0.18f * _e2154) + (0.72f * _e2153)) * _e2171) + (0.95f * _e2170));
                    let _e2179 = ((((0.1f * _e2154) + (0.12f * _e2153)) * _e2171) + (0.55f * _e2170));
                    let _e2181 = floor((_e1629 * 0.4166667f));
                    let _e2183 = cantus_render_shader_hash(vec2<f32>(_e2181, 0f));
                    let _e2186 = (_e2183.y * 0.5f);
                    let _e2190 = ((_e860 * (0.35f + _e2186)) + (_e2183.x * 7f));
                    let _e2192 = (_e2190 - trunc(_e2190));
                    let _e2199 = (_e2067 - (((_e2181 + 0.2f) + (_e2183.x * 0.6f)) * 3f));
                    let _e2200 = (_e2068 - (13f - (_e2192 * 24f)));
                    let _e2207 = (_e2192 * 4f);
                    let _e2209 = select(_e2207, 0f, (_e2207 < 0f));
                    let _e2211 = select(_e2209, 1f, (_e2209 > 1f));
                    let _e2217 = ((_e2192 - 1f) * -3.3333333f);
                    let _e2219 = select(_e2217, 0f, (_e2217 < 0f));
                    let _e2221 = select(_e2219, 1f, (_e2219 > 1f));
                    let _e2229 = ((abs((sqrt(((_e2199 * _e2199) + (_e2200 * _e2200))) - (0.4f + _e2186))) - 1f) * -0.9090909f);
                    let _e2231 = select(_e2229, 0f, (_e2229 < 0f));
                    let _e2233 = select(_e2231, 1f, (_e2231 > 1f));
                    let _e2240 = (((((_e2233 * _e2233) * (3f - (2f * _e2233))) * (((_e2211 * _e2211) * (3f - (2f * _e2211))) * ((_e2221 * _e2221) * (3f - (2f * _e2221))))) * _e2109) * _e2070);
                    let _e2243 = ((((_e2081 * _e2081) * (3f - (2f * _e2081))) * 0.43f) + (((_e2094 * _e2094) * (3f - (2f * _e2094))) * 0.38f));
                    phi_21803_ = _e676;
                    phi_8759_ = vec3<f32>((_e1938 + ((_e2243 + ((_e2177 * _e2143) * 0.78f)) + ((((_e2177 * 0.27999997f) + 0.72f) * _e2240) * 0.9f))), (_e1939 + ((_e2243 + ((_e2178 * _e2143) * 0.78f)) + ((((_e2178 * 0.27999997f) + 0.72f) * _e2240) * 0.9f))), (_e1940 + ((_e2243 + ((_e2179 * _e2143) * 0.78f)) + ((((_e2179 * 0.27999997f) + 0.72f) * _e2240) * 0.9f))));
                    phi_8760_ = false;
                    break;
                }
                case 3: {
                    let _e1945 = pill_1.member[_e497].volume;
                    let _e1947 = select(0f, 1f, (_e1945 < 0f));
                    let _e1948 = abs(_e1945);
                    let _e1951 = round(((_e1629 + 12f) * 0.25f));
                    let _e1953 = select(_e1951, 0f, (_e1951 < 0f));
                    let _e1955 = select(_e1953, 6f, (_e1953 > 6f));
                    let _e1960 = select(select(u32(_e1955), 0u, (_e1955 < 0f)), 4294967295u, (_e1955 > 4294967000f));
                    if (_e1960 < 7u) {
                    } else {
                        phi_21803_ = true;
                        phi_8759_ = vec3<f32>();
                        phi_8760_ = bool();
                        break;
                    }
                    let _e1966 = pill_1.member[_e497].audio_spectrum[_e1960];
                    let _e1967 = (1f - _e1947);
                    let _e1968 = (_e1966 * _e1967);
                    let _e1977 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e1629 - (-12f + (_e1955 * 4f))), (_e1630 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1968))), 1.25f);
                    let _e1980 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1629, (_e1630 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1982 = ((_e1980 - 0.55f) * -0.9090909f);
                    let _e1984 = select(_e1982, 0f, (_e1982 < 0f));
                    let _e1986 = select(_e1984, 1f, (_e1984 > 1f));
                    let _e1990 = ((_e1986 * _e1986) * (3f - (2f * _e1986)));
                    let _e1992 = select(_e1948, 0f, (_e1948 < 0f));
                    let _e1995 = (select(_e1992, 1f, (_e1992 > 1f)) * 28f);
                    let _e1996 = (_e1995 + -13.2f);
                    let _e2000 = ((_e1629 - _e1996) / ((_e1995 + -14.8f) - _e1996));
                    let _e2002 = select(_e2000, 0f, (_e2000 < 0f));
                    let _e2004 = select(_e2002, 1f, (_e2002 > 1f));
                    let _e2009 = (_e1990 * ((_e2004 * _e2004) * (3f - (2f * _e2004))));
                    let _e2011 = (1f - (_e1948 * 0.65f));
                    let _e2016 = ((0.08f * _e2011) + (_e1948 * 0.42249995f));
                    let _e2017 = ((0.88f * _e2011) + (_e1948 * 0.221f));
                    let _e2019 = ((_e1977 - 0.7f) * -0.71428573f);
                    let _e2021 = select(_e2019, 0f, (_e2019 < 0f));
                    let _e2023 = select(_e2021, 1f, (_e2021 > 1f));
                    let _e2032 = ((_e1977 - 3.2f) * -0.3125f);
                    let _e2034 = select(_e2032, 0f, (_e2032 < 0f));
                    let _e2036 = select(_e2034, 1f, (_e2034 > 1f));
                    let _e2043 = ((((_e2023 * _e2023) * (3f - (2f * _e2023))) * (0.58f + (_e1968 * 0.35f))) + ((((_e2036 * _e2036) * (3f - (2f * _e2036))) * _e1968) * 0.12f));
                    let _e2056 = (_e2009 + ((_e1990 * (1f - _e2009)) * 0.22f));
                    phi_21803_ = _e676;
                    phi_8759_ = vec3<f32>((_e1938 + ((_e2016 * _e2043) + (((_e2016 * _e1967) + _e1947) * _e2056))), (_e1939 + ((_e2017 * _e2043) + (((_e2017 * _e1967) + (0.24f * _e1947)) * _e2056))), (_e1940 + (_e2043 + ((_e1967 + (0.3f * _e1947)) * _e2056))));
                    phi_8760_ = false;
                    break;
                }
                case 4: {
                    phi_21803_ = _e676;
                    phi_8759_ = vec3<f32>();
                    phi_8760_ = true;
                    break;
                }
                case 5: {
                    phi_21803_ = _e676;
                    phi_8759_ = vec3<f32>();
                    phi_8760_ = true;
                    break;
                }
                default: {
                    phi_21803_ = _e676;
                    phi_8759_ = vec3<f32>();
                    phi_8760_ = bool();
                    break;
                }
            }
            let _e3151 = phi_21803_;
            let _e3153 = phi_8759_;
            let _e3155 = phi_8760_;
            if _e3151 {
                break;
            }
            if _e3155 {
                let _e3157 = select(1f, 0f, (_e1604 == 5u));
                let _e3161 = pill_1.member[_e497].power_hover;
                let _e3166 = ((abs((f32(_e3161) - _e3157)) - 0.4f) * -2.857143f);
                let _e3168 = select(_e3166, 0f, (_e3166 < 0f));
                let _e3170 = select(_e3168, 1f, (_e3168 > 1f));
                let _e3174 = ((_e3170 * _e3170) * (3f - (2f * _e3170)));
                let _e3176 = (1f + (_e3174 * 0.07f));
                let _e3177 = (_e1629 / _e3176);
                let _e3178 = (_e1630 / _e3176);
                let _e3182 = pill_1.member[_e497].power_action;
                let _e3187 = ((abs((f32(_e3182) - _e3157)) - 0.4f) * -2.857143f);
                let _e3189 = select(_e3187, 0f, (_e3187 < 0f));
                let _e3191 = select(_e3189, 1f, (_e3189 > 1f));
                let _e3195 = ((_e3191 * _e3191) * (3f - (2f * _e3191)));
                let _e3199 = pill_1.member[_e497].power_progress;
                let _e3200 = (_e3199 * _e3195);
                if (_e3157 < 0.5f) {
                    let _e3324 = select(_e3200, 0f, (_e3200 < 0f));
                    let _e3326 = select(_e3324, 1f, (_e3324 > 1f));
                    let _e3330 = ((_e3326 * _e3326) * (3f - (2f * _e3326)));
                    let _e3336 = (1f - _e3200);
                    let _e3345 = (_e3330 * 0.7f);
                    let _e3346 = (_e3345 + 1.5999999f);
                    let _e3351 = ((abs((sqrt(((_e3177 * _e3177) + (_e3178 * _e3178))) - ((7.5f - (_e3200 * 4.6f)) + (((sin((_e860 * 8f)) * _e3200) * _e3336) * 0.16f)))) - _e3346) / ((_e3345 + 0.49999994f) - _e3346));
                    let _e3353 = select(_e3351, 0f, (_e3351 < 0f));
                    let _e3355 = select(_e3353, 1f, (_e3353 > 1f));
                    let _e3364 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e3177, (_e3178 - -7f)), vec2<f32>((3f * _e3336), 3f), 0.5f);
                    let _e3366 = ((_e3364 - 0.55f) * -0.9090909f);
                    let _e3368 = select(_e3366, 0f, (_e3366 < 0f));
                    let _e3370 = select(_e3368, 1f, (_e3368 > 1f));
                    let _e3384 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e3177, (_e3178 - (-5f + (_e3200 * 3.5f)))), vec2<f32>((1.05f + (_e3330 * 0.45f)), (4.6f - (_e3200 * 3f))), 0.7f);
                    let _e3386 = ((_e3384 - 0.55f) * -0.9090909f);
                    let _e3388 = select(_e3386, 0f, (_e3386 < 0f));
                    let _e3390 = select(_e3388, 1f, (_e3388 > 1f));
                    let _e3394 = ((_e3390 * _e3390) * (3f - (2f * _e3390)));
                    let _e3396 = (((_e3355 * _e3355) * (3f - (2f * _e3355))) * (1f - ((_e3370 * _e3370) * (3f - (2f * _e3370)))));
                    if (_e3396 != _e3396) {
                        phi_19517_ = true;
                    } else {
                        phi_19517_ = (_e3394 >= _e3396);
                    }
                    let _e3400 = phi_19517_;
                    phi_9158_ = select(_e3396, _e3394, _e3400);
                } else {
                    let _e3203 = ((1f - _e3195) + _e3200);
                    let _e3207 = (((atan2(_e3178, _e3177) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e3211 = ((_e3203 * 0.82f) - 0.045f);
                    if (_e3211 != _e3211) {
                        phi_19442_ = true;
                    } else {
                        phi_19442_ = (0f >= _e3211);
                    }
                    let _e3215 = phi_19442_;
                    let _e3216 = select(_e3211, 0f, _e3215);
                    let _e3224 = ((abs((sqrt(((_e3177 * _e3177) + (_e3178 * _e3178))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e3226 = select(_e3224, 0f, (_e3224 < 0f));
                    let _e3228 = select(_e3226, 1f, (_e3226 > 1f));
                    let _e3233 = (_e3216 + 0.008f);
                    let _e3237 = (((_e3207 - trunc(_e3207)) - _e3233) / ((_e3216 - 0.008f) - _e3233));
                    let _e3239 = select(_e3237, 0f, (_e3237 < 0f));
                    let _e3241 = select(_e3239, 1f, (_e3239 > 1f));
                    let _e3247 = (_e3203 * 50f);
                    let _e3249 = select(_e3247, 0f, (_e3247 < 0f));
                    let _e3251 = select(_e3249, 1f, (_e3249 > 1f));
                    let _e3256 = ((((_e3228 * _e3228) * (3f - (2f * _e3228))) * ((_e3241 * _e3241) * (3f - (2f * _e3241)))) * ((_e3251 * _e3251) * (3f - (2f * _e3251))));
                    let _e3258 = (0.50265485f + (5.152212f * _e3203));
                    let _e3259 = cos(_e3258);
                    let _e3260 = sin(_e3258);
                    let _e3264 = (_e3177 - (_e3259 * 7.1f));
                    let _e3265 = (_e3178 - (_e3260 * 7.1f));
                    let _e3268 = ((_e3264 * -(_e3260)) + (_e3265 * _e3259));
                    let _e3271 = ((_e3264 * _e3259) + (_e3265 * _e3260));
                    let _e3272 = (_e3268 * -3.2f);
                    let _e3275 = ((_e3272 + (_e3271 * 2.1f)) * 0.06825939f);
                    let _e3277 = select(_e3275, 0f, (_e3275 < 0f));
                    let _e3279 = select(_e3277, 1f, (_e3277 > 1f));
                    let _e3282 = (_e3268 - (-3.2f * _e3279));
                    let _e3283 = (_e3271 - (2.1f * _e3279));
                    let _e3287 = sqrt(((_e3282 * _e3282) + (_e3283 * _e3283)));
                    let _e3290 = ((_e3272 + (_e3271 * -2.1f)) * 0.06825939f);
                    let _e3292 = select(_e3290, 0f, (_e3290 < 0f));
                    let _e3294 = select(_e3292, 1f, (_e3292 > 1f));
                    let _e3297 = (_e3268 - (-3.2f * _e3294));
                    let _e3298 = (_e3271 - (-2.1f * _e3294));
                    let _e3302 = sqrt(((_e3297 * _e3297) + (_e3298 * _e3298)));
                    if (_e3287 != _e3287) {
                        phi_19487_ = true;
                    } else {
                        phi_19487_ = (_e3302 <= _e3287);
                    }
                    let _e3306 = phi_19487_;
                    let _e3309 = ((select(_e3287, _e3302, _e3306) - 1.7f) * -0.71428573f);
                    let _e3311 = select(_e3309, 0f, (_e3309 < 0f));
                    let _e3313 = select(_e3311, 1f, (_e3311 > 1f));
                    let _e3317 = ((_e3313 * _e3313) * (3f - (2f * _e3313)));
                    if (_e3256 != _e3256) {
                        phi_19502_ = true;
                    } else {
                        phi_19502_ = (_e3317 >= _e3256);
                    }
                    let _e3321 = phi_19502_;
                    phi_9158_ = select(_e3256, _e3317, _e3321);
                }
                let _e3403 = phi_9158_;
                let _e3406 = (_e3195 * (0.5f + (_e3200 * 0.5f)));
                if (_e3174 != _e3174) {
                    phi_19532_ = true;
                } else {
                    phi_19532_ = (_e3406 >= _e3174);
                }
                let _e3410 = phi_19532_;
                let _e3411 = select(_e3174, _e3406, _e3410);
                let _e3413 = (0.48f * (1f - _e3411));
                let _e3424 = (1f + (_e3200 * 0.45f));
                phi_9185_ = vec3<f32>((_e1938 + (((_e3413 + (0.78f * _e3411)) * _e3403) * _e3424)), (_e1939 + (((_e3413 + (0.3f * _e3411)) * _e3403) * _e3424)), (_e1940 + (((_e3413 + (0.28f * _e3411)) * _e3403) * _e3424)));
            } else {
                phi_9185_ = _e3153;
            }
            let _e3433 = phi_9185_;
            let _e3435 = local_38;
            let _e3437 = (1f - (_e3435 * 0.35f));
            let _e3445 = local_39;
            let _e3446 = (_e3445 * 0.33249998f);
            switch bitcast<i32>(_e1604) {
                case 0: {
                    let _e3460 = pill_1.member[_e497].labels[0u];
                    phi_9222_ = _e3460;
                    break;
                }
                case 1: {
                    let _e3455 = pill_1.member[_e497].labels[1u];
                    phi_9222_ = _e3455;
                    break;
                }
                default: {
                    phi_9222_ = render_text_Line(vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f), 0f, 0f, 0u, 0u, 0u);
                    break;
                }
            }
            let _e3462 = phi_9222_;
            switch bitcast<i32>(_e1604) {
                case 0: {
                    phi_9227_ = true;
                    break;
                }
                case 1: {
                    phi_9227_ = true;
                    break;
                }
                default: {
                    phi_9227_ = false;
                    break;
                }
            }
            let _e3465 = phi_9227_;
            if _e3465 {
                let _e3467 = (1f / _e3462.size);
                let _e3474 = ((_e804 - _e3462.origin.x) * _e3467);
                phi_9242_ = 0u;
                phi_9245_ = _e3462.count;
                loop {
                    let _e3479 = phi_9242_;
                    let _e3481 = phi_9245_;
                    local_40 = _e3479;
                    let _e3482 = (_e3479 < _e3481);
                    if _e3482 {
                        let _e3485 = (_e3479 + ((_e3481 - _e3479) / 2u));
                        let _e3490 = placed_glyphs_1.member[(_e3462.first + _e3485)].x;
                        let _e3491 = (_e3490 <= _e3474);
                        if _e3491 {
                            phi_9273_ = (_e3485 + 1u);
                        } else {
                            phi_9273_ = _e3479;
                        }
                        let _e3494 = phi_9273_;
                        phi_9243_ = _e3494;
                        phi_9246_ = select(_e3485, _e3481, _e3491);
                    } else {
                        phi_9243_ = u32();
                        phi_9246_ = u32();
                    }
                    let _e3497 = phi_9243_;
                    let _e3499 = phi_9246_;
                    continue;
                    continuing {
                        phi_9242_ = _e3497;
                        phi_9245_ = _e3499;
                        break if !(_e3482);
                    }
                }
                let _e3501 = (3.5f / _e3462.size);
                let _e3503 = local_40;
                let _e3504 = (_e3503 + 1u);
                phi_9285_ = select(_e3504, _e3462.count, (_e3462.count < _e3504));
                phi_9288_ = -1000000f;
                loop {
                    let _e3508 = phi_9285_;
                    let _e3510 = phi_9288_;
                    local_43 = _e3510;
                    if (_e3508 > 0u) {
                        let _e3512 = (_e3508 - 1u);
                        let _e3513 = (_e3462.first + _e3512);
                        let _e3517 = placed_glyphs_1.member[_e3513].x;
                        let _e3521 = placed_glyphs_1.member[_e3513].glyph;
                        let _e3526 = glyphs_1.member[_e3521].min[0u];
                        let _e3531 = glyphs_1.member[_e3521].min[1u];
                        let _e3536 = glyphs_1.member[_e3521].max[0u];
                        let _e3541 = glyphs_1.member[_e3521].max[1u];
                        let _e3545 = glyphs_1.member[_e3521].start;
                        let _e3549 = glyphs_1.member[_e3521].count;
                        let _e3550 = (_e3474 - _e3517);
                        let _e3551 = -(((_e805 - _e3462.origin.y) * _e3467));
                        let _e3552 = (_e3536 + _e3501);
                        let _e3553 = (_e3550 > _e3552);
                        if _e3553 {
                            phi_9418_ = f32();
                        } else {
                            if (_e3550 >= (_e3526 - _e3501)) {
                                if (_e3551 >= (_e3531 - _e3501)) {
                                    if (_e3550 <= _e3552) {
                                        if (_e3551 <= (_e3541 + _e3501)) {
                                            phi_9368_ = 340282350000000000000000000000000000000f;
                                            phi_9371_ = 0u;
                                            phi_9373_ = 0i;
                                            loop {
                                                let _e3563 = phi_9368_;
                                                let _e3565 = phi_9371_;
                                                let _e3567 = phi_9373_;
                                                local_41 = _e3563;
                                                local_42 = _e3567;
                                                let _e3568 = (_e3565 < _e3549);
                                                if _e3568 {
                                                    let _e3572 = edges_1.member[(_e3545 + _e3565)];
                                                    let _e3574 = cantus_render_text_edge_distance(_e3572, _e3462.weight, vec2<f32>(_e3550, _e3551), _e3563);
                                                    phi_9369_ = _e3574.member;
                                                    phi_9372_ = (_e3565 + 1u);
                                                    phi_9374_ = (_e3567 + _e3574.member_1);
                                                } else {
                                                    phi_9369_ = f32();
                                                    phi_9372_ = u32();
                                                    phi_9374_ = i32();
                                                }
                                                let _e3580 = phi_9369_;
                                                let _e3582 = phi_9372_;
                                                let _e3584 = phi_9374_;
                                                continue;
                                                continuing {
                                                    phi_9368_ = _e3580;
                                                    phi_9371_ = _e3582;
                                                    phi_9373_ = _e3584;
                                                    break if !(_e3568);
                                                }
                                            }
                                            let _e3587 = local_41;
                                            let _e3589 = ((_e3587 * _e3462.size) * _e3462.size);
                                            if (_e3589 >= 12.25f) {
                                                phi_9406_ = 3.5f;
                                            } else {
                                                phi_9406_ = sqrt(_e3589);
                                            }
                                            let _e3593 = phi_9406_;
                                            let _e3595 = local_42;
                                            let _e3598 = (_e3593 * select(1f, -1f, (_e3595 == 0i)));
                                            if (_e3510 != _e3510) {
                                                phi_19547_ = true;
                                            } else {
                                                phi_19547_ = (_e3598 >= _e3510);
                                            }
                                            let _e3602 = phi_19547_;
                                            phi_9414_ = select(_e3510, _e3598, _e3602);
                                        } else {
                                            phi_9414_ = _e3510;
                                        }
                                        let _e3605 = phi_9414_;
                                        phi_9415_ = _e3605;
                                    } else {
                                        phi_9415_ = _e3510;
                                    }
                                    let _e3607 = phi_9415_;
                                    phi_9416_ = _e3607;
                                } else {
                                    phi_9416_ = _e3510;
                                }
                                let _e3609 = phi_9416_;
                                phi_9417_ = _e3609;
                            } else {
                                phi_9417_ = _e3510;
                            }
                            let _e3611 = phi_9417_;
                            phi_9418_ = _e3611;
                        }
                        let _e3613 = phi_9418_;
                        phi_9286_ = _e3512;
                        phi_9289_ = _e3613;
                        phi_9420_ = select(true, false, _e3553);
                    } else {
                        phi_9286_ = u32();
                        phi_9289_ = f32();
                        phi_9420_ = false;
                    }
                    let _e3616 = phi_9286_;
                    let _e3618 = phi_9289_;
                    let _e3620 = phi_9420_;
                    continue;
                    continuing {
                        phi_9285_ = _e3616;
                        phi_9288_ = _e3618;
                        break if !(_e3620);
                    }
                }
                let _e3623 = local_43;
                let _e3625 = ((_e3623 * 1.25f) + 0.5f);
                let _e3627 = select(_e3625, 0f, (_e3625 < 0f));
                let _e3629 = select(_e3627, 1f, (_e3627 > 1f));
                phi_9445_ = ((_e3629 * _e3629) * (3f - (2f * _e3629)));
            } else {
                phi_9445_ = 0f;
            }
            let _e3635 = phi_9445_;
            let _e3636 = (1f - _e3635);
            let _e3640 = (0.94f * _e3635);
            out_color = vec4<f32>((((((_e3433.x * _e3437) + _e3446) * _e3636) + _e3640) * _e765), (((((_e3433.y * _e3437) + _e3446) * _e3636) + _e3640) * _e765), (((((_e3433.z * _e3437) + _e3446) * _e3636) + _e3640) * _e765), _e778);
            break;
        }
    }
    return;
}

fn function_6() {
    var phi_9739_: u32;
    var phi_9742_: f32;
    var phi_9740_: u32;
    var phi_9743_: f32;
    var phi_22124_: bool;
    var local_44: f32;
    var phi_9826_: isthmus_Vertex_render_text_Varyings;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = vertex_7;
            let _e497 = instance_2;
            let _e501 = row.member[_e497].icon;
            if (_e501 == -3i) {
                let _e601 = frame.member[0u].screen_size[0u];
                let _e606 = frame.member[0u].screen_size[1u];
                let _e610 = frame.member[0u].panel_height;
                let _e614 = (((_e610 + 36f) + (8f * _e610)) + 56f);
                let _e626 = (((_e601 - 520f) * 0.5f) + (f32((_e496 & 1u)) * 520f));
                let _e627 = (((_e606 - _e614) * 0.5f) + (f32((_e496 >> bitcast<u32>(1i))) * _e614));
                phi_9826_ = isthmus_Vertex_render_text_Varyings(vec4<f32>((((_e626 / _e601) * 2f) - 1f), (((_e627 / _e606) * 2f) - 1f), 0f, 1f), vec2<f32>(_e626, _e627));
            } else {
                let _e507 = frame.member[0u].screen_size[0u];
                let _e512 = frame.member[0u].screen_size[1u];
                let _e516 = frame.member[0u].panel_height;
                let _e523 = row.member[_e497].y;
                let _e527 = frame.member[0u].mouse_pressure;
                phi_9739_ = 0u;
                phi_9742_ = (_e527 * 8f);
                loop {
                    let _e530 = phi_9739_;
                    let _e532 = phi_9742_;
                    local_44 = _e532;
                    let _e533 = (_e530 < 4u);
                    if _e533 {
                        if _e533 {
                        } else {
                            phi_22124_ = true;
                            break;
                        }
                        let _e539 = frame.member[0u].ripples[_e530].start_time;
                        let _e545 = frame.member[0u].ripples[_e530].strength;
                        let _e549 = frame.member[0u].time;
                        let _e551 = ((_e549 - _e539) * 1.2f);
                        let _e553 = select(_e551, 0f, (_e551 < 0f));
                        let _e556 = (1f - select(_e553, 1f, (_e553 > 1f)));
                        phi_9740_ = (_e530 + 1u);
                        phi_9743_ = (_e532 + (((_e545 * _e556) * _e556) * 11f));
                    } else {
                        phi_9740_ = u32();
                        phi_9743_ = f32();
                    }
                    let _e563 = phi_9740_;
                    let _e565 = phi_9743_;
                    continue;
                    continuing {
                        phi_9739_ = _e563;
                        phi_9742_ = _e565;
                        phi_22124_ = false;
                        break if !(_e533);
                    }
                }
                let _e568 = phi_22124_;
                if _e568 {
                    break;
                }
                let _e570 = local_44;
                let _e572 = (18f + (_e570 * 0.5f));
                let _e586 = (((((_e507 - 520f) * 0.5f) + 12f) - _e572) + (f32((_e496 & 1u)) * (496f + (_e572 * 2f))));
                let _e587 = ((_e523 - _e572) + (f32((_e496 >> bitcast<u32>(1i))) * ((_e516 + _e572) * 2f)));
                phi_9826_ = isthmus_Vertex_render_text_Varyings(vec4<f32>((((_e586 / _e507) * 2f) - 1f), (((_e587 / _e512) * 2f) - 1f), 0f, 1f), vec2<f32>(_e586, _e587));
            }
            let _e638 = phi_9826_;
            out_position = _e638.position;
            out_pixel[0u] = _e638.varyings.x;
            out_pixel[1u] = _e638.varyings.y;
            out_row_idx = _e497;
            break;
        }
    }
    return;
}

fn function_7() {
    var phi_10702_: f32;
    var phi_10705_: vec2<f32>;
    var phi_10710_: u32;
    var phi_19673_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_10815_: vec2<f32>;
    var phi_10817_: vec2<f32>;
    var phi_10706_: vec2<f32>;
    var phi_10711_: u32;
    var phi_22129_: bool;
    var phi_10862_: f32;
    var local_45: vec2<f32>;
    var local_46: vec2<f32>;
    var phi_10874_: bool;
    var local_47: vec2<f32>;
    var phi_10885_: f32;
    var local_48: vec2<f32>;
    var phi_19702_: bool;
    var phi_19717_: bool;
    var phi_19732_: bool;
    var phi_19749_: bool;
    var phi_19771_: bool;
    var phi_11217_: vec3<f32>;
    var phi_11218_: vec3<f32>;
    var phi_19795_: bool;
    var phi_19810_: bool;
    var phi_11219_: vec3<f32>;
    var phi_11221_: u32;
    var phi_11224_: vec3<f32>;
    var phi_19855_: bool;
    var phi_19885_: bool;
    var phi_19915_: bool;
    var phi_19960_: bool;
    var phi_19990_: bool;
    var phi_20035_: bool;
    var phi_20065_: bool;
    var phi_20095_: bool;
    var phi_20110_: bool;
    var phi_12167_: f32;
    var phi_20125_: bool;
    var phi_20140_: bool;
    var phi_12197_: vec4<f32>;
    var phi_11222_: u32;
    var phi_11225_: vec3<f32>;
    var phi_22142_: bool;
    var phi_12218_: u32;
    var phi_12221_: vec3<f32>;
    var phi_12248_: u32;
    var phi_12251_: u32;
    var phi_12279_: u32;
    var phi_12249_: u32;
    var phi_12252_: u32;
    var local_49: u32;
    var phi_12291_: u32;
    var phi_12294_: f32;
    var phi_12374_: f32;
    var phi_12377_: u32;
    var phi_12379_: i32;
    var phi_12375_: f32;
    var phi_12378_: u32;
    var phi_12380_: i32;
    var local_50: f32;
    var phi_12412_: f32;
    var local_51: i32;
    var phi_20155_: bool;
    var phi_12420_: f32;
    var phi_12421_: f32;
    var phi_12422_: f32;
    var phi_12423_: f32;
    var phi_12424_: f32;
    var phi_12292_: u32;
    var phi_12295_: f32;
    var phi_12426_: bool;
    var local_52: f32;
    var phi_12219_: u32;
    var phi_12222_: vec3<f32>;
    var phi_22261_: bool;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var local_55: vec3<f32>;
    var phi_10010_: f32;
    var phi_10012_: u32;
    var phi_20215_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_20226_: bool;
    var phi_10118_: f32;
    var phi_10120_: f32;
    var phi_10011_: f32;
    var phi_10013_: u32;
    var phi_22283_: bool;
    var local_56: f32;
    var local_57: f32;
    var phi_20256_: bool;
    var phi_10315_: f32;
    var phi_10317_: f32;
    var phi_10318_: bool;
    var phi_10325_: f32;
    var phi_10429_: u32;
    var phi_10432_: u32;
    var phi_10460_: u32;
    var phi_10430_: u32;
    var phi_10433_: u32;
    var local_58: u32;
    var phi_10472_: u32;
    var phi_10475_: f32;
    var phi_10555_: f32;
    var phi_10558_: u32;
    var phi_10560_: i32;
    var phi_10556_: f32;
    var phi_10559_: u32;
    var phi_10561_: i32;
    var local_59: f32;
    var phi_10593_: f32;
    var local_60: i32;
    var phi_20278_: bool;
    var phi_10601_: f32;
    var phi_10602_: f32;
    var phi_10603_: f32;
    var phi_10604_: f32;
    var phi_10605_: f32;
    var phi_10473_: u32;
    var phi_10476_: f32;
    var phi_10607_: bool;
    var local_61: f32;
    var phi_12478_: vec4<f32>;
    var local_62: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = pixel_4;
            let _e497 = row_idx_1;
            let _e503 = row.member[_e497].icon;
            if (_e503 == -3i) {
                let _e1464 = frame.member[0u].screen_size[0u];
                let _e1469 = frame.member[0u].screen_size[1u];
                let _e1473 = frame.member[0u].panel_height;
                let _e1477 = (((_e1473 + 36f) + (8f * _e1473)) + 56f);
                let _e1482 = (_e496.x - ((_e1464 - 520f) * 0.5f));
                let _e1483 = (_e496.y - ((_e1469 - _e1477) * 0.5f));
                let _e1484 = (_e1477 * 0.5f);
                let _e1485 = (_e1482 - 260f);
                let _e1489 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1485, (_e1483 - _e1484)), vec2<f32>(260f, _e1484), 16f);
                let _e1491 = ((_e1489 - 0.55f) * -0.9090909f);
                let _e1493 = select(_e1491, 0f, (_e1491 < 0f));
                let _e1495 = select(_e1493, 1f, (_e1493 > 1f));
                let _e1499 = ((_e1495 * _e1495) * (3f - (2f * _e1495)));
                if (_e1499 <= 0f) {
                    discard;
                }
                let _e1505 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1485, (_e1483 - (_e1473 + 11.5f))), vec2<f32>(260f, 0.5f), 0f);
                let _e1507 = ((_e1505 - 0.55f) * -0.9090909f);
                let _e1509 = select(_e1507, 0f, (_e1507 < 0f));
                let _e1511 = select(_e1509, 1f, (_e1509 > 1f));
                let _e1515 = ((_e1511 * _e1511) * (3f - (2f * _e1511)));
                let _e1519 = ((0.09f * (1f - _e1515)) + (0.17f * _e1515));
                phi_10010_ = 0f;
                phi_10012_ = 0u;
                loop {
                    let _e1523 = phi_10010_;
                    let _e1525 = phi_10012_;
                    local_56 = _e1523;
                    local_57 = _e1523;
                    let _e1526 = (_e1525 < 4u);
                    if _e1526 {
                        if _e1526 {
                        } else {
                            phi_22283_ = true;
                            break;
                        }
                        let _e1533 = frame.member[0u].ripples[_e1525].origin[0u];
                        let _e1540 = frame.member[0u].ripples[_e1525].origin[1u];
                        let _e1546 = frame.member[0u].ripples[_e1525].start_time;
                        let _e1552 = frame.member[0u].ripples[_e1525].strength;
                        let _e1556 = frame.member[0u].time;
                        let _e1558 = ((_e1556 - _e1546) * 1.2f);
                        let _e1560 = select(_e1558, 0f, (_e1558 < 0f));
                        let _e1562 = select(_e1560, 1f, (_e1560 > 1f));
                        if (_e1552 > 0f) {
                            if (_e1562 < 1f) {
                                let _e1565 = (_e496.x - _e1533);
                                let _e1566 = (_e496.y - _e1540);
                                let _e1570 = sqrt(((_e1565 * _e1565) + (_e1566 * _e1566)));
                                if (_e1570 > 0.001f) {
                                    phi_20215_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e1565 / _e1570), (_e1566 / _e1570)), _e1570);
                                } else {
                                    phi_20215_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e1570);
                                }
                                let _e1578 = phi_20215_;
                                let _e1584 = ((abs((_e1578.unnamed_1 - (_e1562 * 600f))) - 80f) * -0.0125f);
                                let _e1586 = select(_e1584, 0f, (_e1584 < 0f));
                                let _e1588 = select(_e1586, 1f, (_e1586 > 1f));
                                let _e1597 = (_e1523 + (((((_e1588 * _e1588) * (3f - (2f * _e1588))) * _e1552) * (1f - _e1562)) * 0.5f));
                                if (_e1597 != _e1597) {
                                    phi_20226_ = true;
                                } else {
                                    phi_20226_ = (1f <= _e1597);
                                }
                                let _e1601 = phi_20226_;
                                phi_10118_ = select(_e1597, 1f, _e1601);
                            } else {
                                phi_10118_ = _e1523;
                            }
                            let _e1604 = phi_10118_;
                            phi_10120_ = _e1604;
                        } else {
                            phi_10120_ = _e1523;
                        }
                        let _e1606 = phi_10120_;
                        phi_10011_ = _e1606;
                        phi_10013_ = (_e1525 + 1u);
                    } else {
                        phi_10011_ = f32();
                        phi_10013_ = u32();
                    }
                    let _e1609 = phi_10011_;
                    let _e1611 = phi_10013_;
                    continue;
                    continuing {
                        phi_10010_ = _e1609;
                        phi_10012_ = _e1611;
                        phi_22283_ = false;
                        break if !(_e1526);
                    }
                }
                let _e1614 = phi_22283_;
                if _e1614 {
                    break;
                }
                let _e1616 = local_56;
                let _e1620 = local_57;
                let _e1624 = (_e1482 - 23f);
                let _e1625 = (_e1483 - ((_e1473 + 12f) * 0.5f));
                let _e1633 = ((abs((sqrt(((_e1624 * _e1624) + (_e1625 * _e1625))) - 6.2f)) - 1.5999999f) * -0.909091f);
                let _e1635 = select(_e1633, 0f, (_e1633 < 0f));
                let _e1637 = select(_e1635, 1f, (_e1635 > 1f));
                let _e1641 = ((_e1637 * _e1637) * (3f - (2f * _e1637)));
                let _e1642 = (_e1482 - 27.6f);
                let _e1643 = (_e1625 - 4.6f);
                let _e1645 = ((_e1642 + _e1643) * 0.119047605f);
                let _e1647 = select(_e1645, 0f, (_e1645 < 0f));
                let _e1650 = (4.2000003f * select(_e1647, 1f, (_e1647 > 1f)));
                let _e1651 = (_e1642 - _e1650);
                let _e1652 = (_e1643 - _e1650);
                let _e1659 = ((abs(sqrt(((_e1651 * _e1651) + (_e1652 * _e1652)))) - 1.5999999f) * -0.909091f);
                let _e1661 = select(_e1659, 0f, (_e1659 < 0f));
                let _e1663 = select(_e1661, 1f, (_e1661 > 1f));
                let _e1667 = ((_e1663 * _e1663) * (3f - (2f * _e1663)));
                if (_e1641 != _e1641) {
                    phi_20256_ = true;
                } else {
                    phi_20256_ = (_e1667 >= _e1641);
                }
                let _e1671 = phi_20256_;
                let _e1672 = select(_e1641, _e1667, _e1671);
                let _e1681 = row.member[_e497].selection[1u];
                let _e1686 = row.member[_e497].selection[0u];
                let _e1687 = (_e1681 - _e1686);
                if (abs(_e1686) <= 170141170000000000000000000000000000000f) {
                    let _e1691 = (abs(_e1681) <= 170141170000000000000000000000000000000f);
                    if _e1691 {
                        phi_10315_ = ((_e1686 + _e1681) * 0.5f);
                    } else {
                        phi_10315_ = f32();
                    }
                    let _e1695 = phi_10315_;
                    phi_10317_ = _e1695;
                    phi_10318_ = select(true, false, _e1691);
                } else {
                    phi_10317_ = f32();
                    phi_10318_ = true;
                }
                let _e1698 = phi_10317_;
                let _e1700 = phi_10318_;
                if _e1700 {
                    phi_10325_ = (0.5f * (_e1686 + _e1681));
                } else {
                    phi_10325_ = _e1698;
                }
                let _e1704 = phi_10325_;
                let _e1709 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e1482 - _e1704), _e1625), vec2<f32>((_e1687 * 0.5f), 13f), 3f);
                let _e1711 = ((_e1709 - 0.55f) * -0.9090909f);
                let _e1713 = select(_e1711, 0f, (_e1711 < 0f));
                let _e1715 = select(_e1713, 1f, (_e1713 > 1f));
                let _e1722 = (((_e1715 * _e1715) * (3f - (2f * _e1715))) * select(0f, 1f, (_e1687 > 0f)));
                let _e1724 = (((((_e1519 * (1f - _e1616)) + (((_e1519 * 1.5f) + 0.1f) * _e1620)) * (1f - _e1672)) + (0.58f * _e1672)) * (1f - _e1722));
                let _e1735 = row.member[_e497].caret[0u];
                let _e1738 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e1482 - _e1735), _e1625), vec2<f32>(0.9f, 12f), 0.9f);
                let _e1740 = ((_e1738 - 0.55f) * -0.9090909f);
                let _e1742 = select(_e1740, 0f, (_e1740 < 0f));
                let _e1744 = select(_e1742, 1f, (_e1742 > 1f));
                let _e1753 = row.member[_e497].caret[1u];
                let _e1754 = (((_e1744 * _e1744) * (3f - (2f * _e1744))) * _e1753);
                let _e1755 = (1f - _e1754);
                let _e1759 = (0.94f * _e1754);
                let _e1767 = row.member[_e497].lines[0u];
                let _e1769 = (1f / _e1767.size);
                let _e1776 = ((_e1482 - _e1767.origin.x) * _e1769);
                phi_10429_ = 0u;
                phi_10432_ = _e1767.count;
                loop {
                    let _e1781 = phi_10429_;
                    let _e1783 = phi_10432_;
                    local_58 = _e1781;
                    let _e1784 = (_e1781 < _e1783);
                    if _e1784 {
                        let _e1787 = (_e1781 + ((_e1783 - _e1781) / 2u));
                        let _e1792 = placed_glyphs.member[(_e1767.first + _e1787)].x;
                        let _e1793 = (_e1792 <= _e1776);
                        if _e1793 {
                            phi_10460_ = (_e1787 + 1u);
                        } else {
                            phi_10460_ = _e1781;
                        }
                        let _e1796 = phi_10460_;
                        phi_10430_ = _e1796;
                        phi_10433_ = select(_e1787, _e1783, _e1793);
                    } else {
                        phi_10430_ = u32();
                        phi_10433_ = u32();
                    }
                    let _e1799 = phi_10430_;
                    let _e1801 = phi_10433_;
                    continue;
                    continuing {
                        phi_10429_ = _e1799;
                        phi_10432_ = _e1801;
                        break if !(_e1784);
                    }
                }
                let _e1803 = (3.5f / _e1767.size);
                let _e1805 = local_58;
                let _e1806 = (_e1805 + 1u);
                phi_10472_ = select(_e1806, _e1767.count, (_e1767.count < _e1806));
                phi_10475_ = -1000000f;
                loop {
                    let _e1810 = phi_10472_;
                    let _e1812 = phi_10475_;
                    local_61 = _e1812;
                    if (_e1810 > 0u) {
                        let _e1814 = (_e1810 - 1u);
                        let _e1815 = (_e1767.first + _e1814);
                        let _e1819 = placed_glyphs.member[_e1815].x;
                        let _e1823 = placed_glyphs.member[_e1815].glyph;
                        let _e1828 = glyphs.member[_e1823].min[0u];
                        let _e1833 = glyphs.member[_e1823].min[1u];
                        let _e1838 = glyphs.member[_e1823].max[0u];
                        let _e1843 = glyphs.member[_e1823].max[1u];
                        let _e1847 = glyphs.member[_e1823].start;
                        let _e1851 = glyphs.member[_e1823].count;
                        let _e1852 = (_e1776 - _e1819);
                        let _e1853 = -(((_e1483 - _e1767.origin.y) * _e1769));
                        let _e1854 = (_e1838 + _e1803);
                        let _e1855 = (_e1852 > _e1854);
                        if _e1855 {
                            phi_10605_ = f32();
                        } else {
                            if (_e1852 >= (_e1828 - _e1803)) {
                                if (_e1853 >= (_e1833 - _e1803)) {
                                    if (_e1852 <= _e1854) {
                                        if (_e1853 <= (_e1843 + _e1803)) {
                                            phi_10555_ = 340282350000000000000000000000000000000f;
                                            phi_10558_ = 0u;
                                            phi_10560_ = 0i;
                                            loop {
                                                let _e1865 = phi_10555_;
                                                let _e1867 = phi_10558_;
                                                let _e1869 = phi_10560_;
                                                local_59 = _e1865;
                                                local_60 = _e1869;
                                                let _e1870 = (_e1867 < _e1851);
                                                if _e1870 {
                                                    let _e1874 = edges.member[(_e1847 + _e1867)];
                                                    let _e1876 = cantus_render_text_edge_distance(_e1874, _e1767.weight, vec2<f32>(_e1852, _e1853), _e1865);
                                                    phi_10556_ = _e1876.member;
                                                    phi_10559_ = (_e1867 + 1u);
                                                    phi_10561_ = (_e1869 + _e1876.member_1);
                                                } else {
                                                    phi_10556_ = f32();
                                                    phi_10559_ = u32();
                                                    phi_10561_ = i32();
                                                }
                                                let _e1882 = phi_10556_;
                                                let _e1884 = phi_10559_;
                                                let _e1886 = phi_10561_;
                                                continue;
                                                continuing {
                                                    phi_10555_ = _e1882;
                                                    phi_10558_ = _e1884;
                                                    phi_10560_ = _e1886;
                                                    break if !(_e1870);
                                                }
                                            }
                                            let _e1889 = local_59;
                                            let _e1891 = ((_e1889 * _e1767.size) * _e1767.size);
                                            if (_e1891 >= 12.25f) {
                                                phi_10593_ = 3.5f;
                                            } else {
                                                phi_10593_ = sqrt(_e1891);
                                            }
                                            let _e1895 = phi_10593_;
                                            let _e1897 = local_60;
                                            let _e1900 = (_e1895 * select(1f, -1f, (_e1897 == 0i)));
                                            if (_e1812 != _e1812) {
                                                phi_20278_ = true;
                                            } else {
                                                phi_20278_ = (_e1900 >= _e1812);
                                            }
                                            let _e1904 = phi_20278_;
                                            phi_10601_ = select(_e1812, _e1900, _e1904);
                                        } else {
                                            phi_10601_ = _e1812;
                                        }
                                        let _e1907 = phi_10601_;
                                        phi_10602_ = _e1907;
                                    } else {
                                        phi_10602_ = _e1812;
                                    }
                                    let _e1909 = phi_10602_;
                                    phi_10603_ = _e1909;
                                } else {
                                    phi_10603_ = _e1812;
                                }
                                let _e1911 = phi_10603_;
                                phi_10604_ = _e1911;
                            } else {
                                phi_10604_ = _e1812;
                            }
                            let _e1913 = phi_10604_;
                            phi_10605_ = _e1913;
                        }
                        let _e1915 = phi_10605_;
                        phi_10473_ = _e1814;
                        phi_10476_ = _e1915;
                        phi_10607_ = select(true, false, _e1855);
                    } else {
                        phi_10473_ = u32();
                        phi_10476_ = f32();
                        phi_10607_ = false;
                    }
                    let _e1918 = phi_10473_;
                    let _e1920 = phi_10476_;
                    let _e1922 = phi_10607_;
                    continue;
                    continuing {
                        phi_10472_ = _e1918;
                        phi_10475_ = _e1920;
                        break if !(_e1922);
                    }
                }
                let _e1925 = local_61;
                let _e1927 = ((_e1925 * 1.25f) + 0.5f);
                let _e1929 = select(_e1927, 0f, (_e1927 < 0f));
                let _e1931 = select(_e1929, 1f, (_e1929 > 1f));
                let _e1935 = ((_e1931 * _e1931) * (3f - (2f * _e1931)));
                let _e1936 = (_e1499 * 0.82f);
                let _e1938 = unpack4x8unorm(_e1767.color);
                let _e1942 = (1f - _e1935);
                phi_12478_ = vec4<f32>(((((((_e1724 + (0.24f * _e1722)) * _e1755) + _e1759) * _e1942) + (_e1938.x * _e1935)) * _e1936), ((((((_e1724 + (0.28f * _e1722)) * _e1755) + _e1759) * _e1942) + (_e1938.y * _e1935)) * _e1936), ((((((_e1724 + (0.52f * _e1722)) * _e1755) + _e1759) * _e1942) + (_e1938.z * _e1935)) * _e1936), _e1936);
            } else {
                let _e509 = frame.member[0u].screen_size[0u];
                let _e513 = frame.member[0u].panel_height;
                let _e516 = (((_e509 - 520f) * 0.5f) + 12f);
                let _e520 = row.member[_e497].y;
                let _e521 = (_e496.x - _e516);
                let _e522 = (_e496.y - _e520);
                let _e523 = (_e513 * 0.5f);
                let _e525 = (_e522 - _e523);
                let _e527 = ((496f - _e513) * 0.5f);
                let _e529 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e521 - 248f), _e525), _e527, _e523);
                let _e533 = frame.member[0u].mouse_pressure;
                let _e534 = (_e533 > 0f);
                if _e534 {
                    let _e539 = frame.member[0u].mouse_pos[0u];
                    let _e544 = frame.member[0u].mouse_pos[1u];
                    let _e550 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e539 - _e516) - 248f), ((_e544 - _e520) - _e523)), _e527, _e523);
                    phi_10702_ = _e550;
                } else {
                    phi_10702_ = 1f;
                }
                let _e552 = phi_10702_;
                phi_10705_ = vec2<f32>(0f, 0f);
                phi_10710_ = 0u;
                loop {
                    let _e554 = phi_10705_;
                    let _e556 = phi_10710_;
                    local_45 = _e554;
                    local_46 = _e554;
                    local_47 = _e554;
                    local_48 = _e554;
                    let _e557 = (_e556 < 4u);
                    if _e557 {
                        if _e557 {
                        } else {
                            phi_22129_ = true;
                            break;
                        }
                        let _e564 = frame.member[0u].ripples[_e556].origin[0u];
                        let _e571 = frame.member[0u].ripples[_e556].origin[1u];
                        let _e577 = frame.member[0u].ripples[_e556].start_time;
                        let _e583 = frame.member[0u].ripples[_e556].strength;
                        let _e587 = frame.member[0u].time;
                        let _e589 = ((_e587 - _e577) * 1.2f);
                        let _e591 = select(_e589, 0f, (_e589 < 0f));
                        let _e593 = select(_e591, 1f, (_e591 > 1f));
                        if (_e583 > 0f) {
                            if (_e593 < 1f) {
                                let _e596 = (_e496.x - _e564);
                                let _e597 = (_e496.y - _e571);
                                let _e601 = sqrt(((_e596 * _e596) + (_e597 * _e597)));
                                if (_e601 > 0.001f) {
                                    phi_19673_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e596 / _e601), (_e597 / _e601)), _e601);
                                } else {
                                    phi_19673_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e601);
                                }
                                let _e609 = phi_19673_;
                                let _e619 = ((abs((_e609.unnamed_1 - (_e593 * 600f))) - 80f) * -0.0125f);
                                let _e621 = select(_e619, 0f, (_e619 < 0f));
                                let _e623 = select(_e621, 1f, (_e621 > 1f));
                                let _e629 = (1f - _e593);
                                let _e630 = ((((_e623 * _e623) * (3f - (2f * _e623))) * _e583) * _e629);
                                phi_10815_ = vec2<f32>((_e554.x + (((_e609.unnamed.x * _e630) * _e629) * 0.5f)), (_e554.y + (((_e609.unnamed.y * _e630) * _e629) * 0.5f)));
                            } else {
                                phi_10815_ = _e554;
                            }
                            let _e643 = phi_10815_;
                            phi_10817_ = _e643;
                        } else {
                            phi_10817_ = _e554;
                        }
                        let _e645 = phi_10817_;
                        phi_10706_ = _e645;
                        phi_10711_ = (_e556 + 1u);
                    } else {
                        phi_10706_ = vec2<f32>();
                        phi_10711_ = u32();
                    }
                    let _e648 = phi_10706_;
                    let _e650 = phi_10711_;
                    continue;
                    continuing {
                        phi_10705_ = _e648;
                        phi_10710_ = _e650;
                        phi_22129_ = false;
                        break if !(_e557);
                    }
                }
                let _e653 = phi_22129_;
                if _e653 {
                    break;
                }
                if _e534 {
                    let _e658 = frame.member[0u].mouse_pos[0u];
                    let _e663 = frame.member[0u].mouse_pos[1u];
                    let _e664 = (_e496.x - _e658);
                    let _e665 = (_e496.y - _e663);
                    let _e671 = ((sqrt(((_e664 * _e664) + (_e665 * _e665))) - 150f) * -0.006666667f);
                    let _e673 = select(_e671, 0f, (_e671 < 0f));
                    let _e675 = select(_e673, 1f, (_e673 > 1f));
                    phi_10862_ = ((((_e675 * _e675) * (3f - (2f * _e675))) * _e533) * 8f);
                } else {
                    phi_10862_ = 0f;
                }
                let _e683 = phi_10862_;
                let _e685 = local_45;
                let _e688 = global[0u];
                if (_e685.x == _e688) {
                    let _e691 = local_46;
                    let _e694 = global[1u];
                    phi_10874_ = (_e691.y == _e694);
                } else {
                    phi_10874_ = false;
                }
                let _e697 = phi_10874_;
                if _e697 {
                    phi_10885_ = 0f;
                } else {
                    let _e699 = local_47;
                    phi_10885_ = (sqrt(((_e685.x * _e685.x) + (_e699.y * _e699.y))) * 22f);
                }
                let _e707 = phi_10885_;
                let _e709 = local_48;
                let _e712 = ((_e552 - 0.5f) * -1f);
                let _e714 = select(_e712, 0f, (_e712 < 0f));
                let _e716 = select(_e714, 1f, (_e714 > 1f));
                let _e722 = ((_e683 * ((_e716 * _e716) * (3f - (2f * _e716)))) + _e707);
                let _e724 = (_e529 - (_e722 * 0.5f));
                let _e725 = fwidth(_e724);
                if (_e725 != _e725) {
                    phi_19702_ = true;
                } else {
                    phi_19702_ = (0.55f >= _e725);
                }
                let _e729 = phi_19702_;
                let _e730 = select(_e725, 0.55f, _e729);
                let _e734 = ((_e724 - _e730) / (-(_e730) - _e730));
                let _e736 = select(_e734, 0f, (_e734 < 0f));
                let _e738 = select(_e736, 1f, (_e736 > 1f));
                let _e742 = ((_e738 * _e738) * (3f - (2f * _e738)));
                let _e743 = (_e724 != _e724);
                if _e743 {
                    phi_19717_ = true;
                } else {
                    phi_19717_ = (0f >= _e724);
                }
                let _e746 = phi_19717_;
                let _e750 = (exp((select(_e724, 0f, _e746) * -0.3f)) * 0.16f);
                if (_e742 != _e742) {
                    phi_19732_ = true;
                } else {
                    phi_19732_ = (_e750 >= _e742);
                }
                let _e754 = phi_19732_;
                let _e755 = select(_e742, _e750, _e754);
                if (_e755 <= 0.0009765625f) {
                    discard;
                }
                let _e757 = (_e521 * 0.002016129f);
                let _e758 = (_e522 / _e513);
                if _e743 {
                    phi_19749_ = true;
                } else {
                    phi_19749_ = (0f <= _e724);
                }
                let _e763 = phi_19749_;
                let _e766 = (1f + (select(_e724, 0f, _e763) * 0.008333334f));
                let _e768 = select(_e766, 0f, (_e766 < 0f));
                let _e770 = select(_e768, 0.6f, (_e768 > 0.6f));
                let _e781 = (((_e757 - (((_e757 - 0.5f) * _e770) * 0.08f)) - (_e685.x * 0.04f)) * 496f);
                let _e782 = (((_e758 - (((_e758 - 0.5f) * _e770) * 0.08f)) - (_e709.y * 0.04f)) * _e513);
                let _e788 = row.member[_e497].badges[0u][1u];
                let _e790 = select(0f, 1f, (_e788 > 0f));
                let _e795 = (_e722 * 0.125f);
                if (_e795 != _e795) {
                    phi_19771_ = true;
                } else {
                    phi_19771_ = (1f <= _e795);
                }
                let _e799 = phi_19771_;
                let _e800 = select(_e795, 1f, _e799);
                let _e804 = ((((0.15f * (1f - _e790)) + (0.235f * _e790)) * (1f - _e800)) + (0.3f * _e800));
                let _e805 = vec3(_e804);
                let _e806 = (_e521 - _e523);
                if (_e503 == -2i) {
                    let _e844 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e806, _e525), vec2<f32>(13f, 13f), 9f);
                    let _e846 = ((_e844 - 0.55f) * -0.9090909f);
                    let _e848 = select(_e846, 0f, (_e846 < 0f));
                    let _e850 = select(_e848, 1f, (_e848 > 1f));
                    let _e854 = ((_e850 * _e850) * (3f - (2f * _e850)));
                    let _e857 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e806, (_e525 - -3.1f)), vec2<f32>(5.4f, 1.1f), 1.1f);
                    let _e859 = ((_e857 - 0.55f) * -0.9090909f);
                    let _e861 = select(_e859, 0f, (_e859 < 0f));
                    let _e863 = select(_e861, 1f, (_e861 > 1f));
                    let _e867 = ((_e863 * _e863) * (3f - (2f * _e863)));
                    let _e870 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e806, (_e525 - 3.1f)), vec2<f32>(5.4f, 1.1f), 1.1f);
                    let _e872 = ((_e870 - 0.55f) * -0.9090909f);
                    let _e874 = select(_e872, 0f, (_e872 < 0f));
                    let _e876 = select(_e874, 1f, (_e874 > 1f));
                    let _e880 = ((_e876 * _e876) * (3f - (2f * _e876)));
                    if (_e867 != _e867) {
                        phi_19795_ = true;
                    } else {
                        phi_19795_ = (_e880 >= _e867);
                    }
                    let _e884 = phi_19795_;
                    let _e885 = select(_e867, _e880, _e884);
                    let _e886 = (1f - _e885);
                    let _e890 = (0.96f * _e885);
                    let _e894 = (_e885 * _e854);
                    if (_e854 != _e854) {
                        phi_19810_ = true;
                    } else {
                        phi_19810_ = (_e894 >= _e854);
                    }
                    let _e898 = phi_19810_;
                    let _e899 = select(_e854, _e894, _e898);
                    let _e901 = (_e804 * (1f - _e899));
                    phi_11219_ = vec3<f32>((_e901 + (((0.44f * _e886) + _e890) * _e899)), (_e901 + (((0.4f * _e886) + _e890) * _e899)), (_e901 + (((0.8f * _e886) + _e890) * _e899)));
                } else {
                    if (_e503 >= 0i) {
                        let _e809 = abs(_e806);
                        let _e810 = abs(_e525);
                        if (select(_e810, _e809, (_e809 > _e810)) < 16f) {
                            let _e819 = vec3<f32>(((_e806 * 0.03125f) + 0.5f), ((_e525 * 0.03125f) + 0.5f), f32(_e503));
                            let _e825 = textureSample(icons, sampler_, vec2<f32>(_e819.x, _e819.y), i32(_e819.z));
                            let _e831 = (_e804 * (1f - _e825.w));
                            phi_11217_ = vec3<f32>((_e831 + (_e825.x * _e825.w)), (_e831 + (_e825.y * _e825.w)), (_e831 + (_e825.z * _e825.w)));
                        } else {
                            phi_11217_ = _e805;
                        }
                        let _e840 = phi_11217_;
                        phi_11218_ = _e840;
                    } else {
                        phi_11218_ = _e805;
                    }
                    let _e842 = phi_11218_;
                    phi_11219_ = _e842;
                }
                let _e910 = phi_11219_;
                phi_11221_ = 0u;
                phi_11224_ = _e910;
                loop {
                    let _e912 = phi_11221_;
                    let _e914 = phi_11224_;
                    local_62 = _e914;
                    let _e915 = (_e912 < 2u);
                    if _e915 {
                        if _e915 {
                        } else {
                            phi_22142_ = true;
                            break;
                        }
                        let _e921 = row.member[_e497].badges[_e912][0u];
                        let _e927 = row.member[_e497].badges[_e912][1u];
                        let _e928 = (_e781 - _e921);
                        let _e929 = (_e782 - _e523);
                        if (_e927 <= 0f) {
                            phi_12197_ = vec4<f32>(0f, 0f, 0f, 0f);
                        } else {
                            let _e934 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e928, _e929), vec2<f32>(_e927, 10.5f), 6f);
                            let _e936 = ((_e934 - 0.55f) * -0.9090909f);
                            let _e938 = select(_e936, 0f, (_e936 < 0f));
                            let _e940 = select(_e938, 1f, (_e938 > 1f));
                            let _e944 = ((_e940 * _e940) * (3f - (2f * _e940)));
                            let _e947 = ((abs(_e934) - 1.2f) * -0.9090908f);
                            let _e949 = select(_e947, 0f, (_e947 < 0f));
                            let _e951 = select(_e949, 1f, (_e949 > 1f));
                            let _e955 = ((_e951 * _e951) * (3f - (2f * _e951)));
                            if (_e912 == 1u) {
                                let _e1039 = (_e928 + 8.5f);
                                let _e1040 = (_e929 - -4f);
                                let _e1042 = (_e1040 * 4.2f);
                                let _e1044 = (((_e1039 * -3.4f) + _e1042) * 0.03424658f);
                                let _e1046 = select(_e1044, 0f, (_e1044 < 0f));
                                let _e1048 = select(_e1046, 1f, (_e1046 > 1f));
                                let _e1051 = (_e1039 - (-3.4f * _e1048));
                                let _e1052 = (_e1040 - (4.2f * _e1048));
                                let _e1056 = sqrt(((_e1051 * _e1051) + (_e1052 * _e1052)));
                                let _e1059 = (((_e1039 * 3.4f) + _e1042) * 0.03424658f);
                                let _e1061 = select(_e1059, 0f, (_e1059 < 0f));
                                let _e1063 = select(_e1061, 1f, (_e1061 > 1f));
                                let _e1066 = (_e1039 - (3.4f * _e1063));
                                let _e1067 = (_e1040 - (4.2f * _e1063));
                                let _e1071 = sqrt(((_e1066 * _e1066) + (_e1067 * _e1067)));
                                if (_e1056 != _e1056) {
                                    phi_19960_ = true;
                                } else {
                                    phi_19960_ = (_e1071 <= _e1056);
                                }
                                let _e1075 = phi_19960_;
                                let _e1076 = select(_e1056, _e1071, _e1075);
                                let _e1077 = (_e929 - -0.6f);
                                let _e1078 = (_e1077 * 0.21739131f);
                                let _e1080 = select(_e1078, 0f, (_e1078 < 0f));
                                let _e1084 = (_e1077 - (4.6f * select(_e1080, 1f, (_e1080 > 1f))));
                                let _e1088 = sqrt(((_e1039 * _e1039) + (_e1084 * _e1084)));
                                if (_e1076 != _e1076) {
                                    phi_19990_ = true;
                                } else {
                                    phi_19990_ = (_e1088 <= _e1076);
                                }
                                let _e1092 = phi_19990_;
                                let _e1096 = ((abs(select(_e1076, _e1088, _e1092)) - 1.35f) * -0.9090909f);
                                let _e1098 = select(_e1096, 0f, (_e1096 < 0f));
                                let _e1100 = select(_e1098, 1f, (_e1098 > 1f));
                                let _e1104 = ((_e1100 * _e1100) * (3f - (2f * _e1100)));
                                let _e1105 = (_e928 - 10.9f);
                                let _e1106 = (_e929 - -3.6f);
                                let _e1107 = (_e1106 * 0.18518521f);
                                let _e1109 = select(_e1107, 0f, (_e1107 < 0f));
                                let _e1113 = (_e1106 - (5.3999996f * select(_e1109, 1f, (_e1109 > 1f))));
                                let _e1117 = sqrt(((_e1105 * _e1105) + (_e1113 * _e1113)));
                                let _e1118 = (_e929 - 1.8f);
                                let _e1119 = (_e1105 * -0.16666667f);
                                let _e1121 = select(_e1119, 0f, (_e1119 < 0f));
                                let _e1125 = (_e1105 - (-6f * select(_e1121, 1f, (_e1121 > 1f))));
                                let _e1129 = sqrt(((_e1125 * _e1125) + (_e1118 * _e1118)));
                                if (_e1117 != _e1117) {
                                    phi_20035_ = true;
                                } else {
                                    phi_20035_ = (_e1129 <= _e1117);
                                }
                                let _e1133 = phi_20035_;
                                let _e1134 = select(_e1117, _e1129, _e1133);
                                let _e1135 = (_e928 - 4.9f);
                                let _e1136 = (_e1135 * 2.8f);
                                let _e1139 = ((_e1136 + (_e1118 * -2.6f)) * 0.06849316f);
                                let _e1141 = select(_e1139, 0f, (_e1139 < 0f));
                                let _e1143 = select(_e1141, 1f, (_e1141 > 1f));
                                let _e1146 = (_e1135 - (2.8f * _e1143));
                                let _e1147 = (_e1118 - (-2.6f * _e1143));
                                let _e1151 = sqrt(((_e1146 * _e1146) + (_e1147 * _e1147)));
                                if (_e1134 != _e1134) {
                                    phi_20065_ = true;
                                } else {
                                    phi_20065_ = (_e1151 <= _e1134);
                                }
                                let _e1155 = phi_20065_;
                                let _e1156 = select(_e1134, _e1151, _e1155);
                                let _e1159 = ((_e1136 + (_e1118 * 2.6000001f)) * 0.06849315f);
                                let _e1161 = select(_e1159, 0f, (_e1159 < 0f));
                                let _e1163 = select(_e1161, 1f, (_e1161 > 1f));
                                let _e1166 = (_e1135 - (2.8f * _e1163));
                                let _e1167 = (_e1118 - (2.6000001f * _e1163));
                                let _e1171 = sqrt(((_e1166 * _e1166) + (_e1167 * _e1167)));
                                if (_e1156 != _e1156) {
                                    phi_20095_ = true;
                                } else {
                                    phi_20095_ = (_e1171 <= _e1156);
                                }
                                let _e1175 = phi_20095_;
                                let _e1179 = ((abs(select(_e1156, _e1171, _e1175)) - 1.35f) * -0.9090909f);
                                let _e1181 = select(_e1179, 0f, (_e1179 < 0f));
                                let _e1183 = select(_e1181, 1f, (_e1181 > 1f));
                                let _e1187 = ((_e1183 * _e1183) * (3f - (2f * _e1183)));
                                if (_e1104 != _e1104) {
                                    phi_20110_ = true;
                                } else {
                                    phi_20110_ = (_e1187 >= _e1104);
                                }
                                let _e1191 = phi_20110_;
                                phi_12167_ = select(_e1104, _e1187, _e1191);
                            } else {
                                let _e956 = (_e928 - 3.4f);
                                let _e957 = (_e929 - -3.6f);
                                let _e958 = (_e957 * 0.18518521f);
                                let _e960 = select(_e958, 0f, (_e958 < 0f));
                                let _e964 = (_e957 - (5.3999996f * select(_e960, 1f, (_e960 > 1f))));
                                let _e968 = sqrt(((_e956 * _e956) + (_e964 * _e964)));
                                let _e969 = (_e929 - 1.8f);
                                let _e970 = (_e956 * -0.16666667f);
                                let _e972 = select(_e970, 0f, (_e970 < 0f));
                                let _e976 = (_e956 - (-6f * select(_e972, 1f, (_e972 > 1f))));
                                let _e980 = sqrt(((_e976 * _e976) + (_e969 * _e969)));
                                if (_e968 != _e968) {
                                    phi_19855_ = true;
                                } else {
                                    phi_19855_ = (_e980 <= _e968);
                                }
                                let _e984 = phi_19855_;
                                let _e985 = select(_e968, _e980, _e984);
                                let _e986 = (_e928 - -2.6f);
                                let _e987 = (_e986 * 2.8f);
                                let _e990 = ((_e987 + (_e969 * -2.6f)) * 0.06849316f);
                                let _e992 = select(_e990, 0f, (_e990 < 0f));
                                let _e994 = select(_e992, 1f, (_e992 > 1f));
                                let _e997 = (_e986 - (2.8f * _e994));
                                let _e998 = (_e969 - (-2.6f * _e994));
                                let _e1002 = sqrt(((_e997 * _e997) + (_e998 * _e998)));
                                if (_e985 != _e985) {
                                    phi_19885_ = true;
                                } else {
                                    phi_19885_ = (_e1002 <= _e985);
                                }
                                let _e1006 = phi_19885_;
                                let _e1007 = select(_e985, _e1002, _e1006);
                                let _e1010 = ((_e987 + (_e969 * 2.6000001f)) * 0.06849315f);
                                let _e1012 = select(_e1010, 0f, (_e1010 < 0f));
                                let _e1014 = select(_e1012, 1f, (_e1012 > 1f));
                                let _e1017 = (_e986 - (2.8f * _e1014));
                                let _e1018 = (_e969 - (2.6000001f * _e1014));
                                let _e1022 = sqrt(((_e1017 * _e1017) + (_e1018 * _e1018)));
                                if (_e1007 != _e1007) {
                                    phi_19915_ = true;
                                } else {
                                    phi_19915_ = (_e1022 <= _e1007);
                                }
                                let _e1026 = phi_19915_;
                                let _e1030 = ((abs(select(_e1007, _e1022, _e1026)) - 1.35f) * -0.9090909f);
                                let _e1032 = select(_e1030, 0f, (_e1030 < 0f));
                                let _e1034 = select(_e1032, 1f, (_e1032 > 1f));
                                phi_12167_ = ((_e1034 * _e1034) * (3f - (2f * _e1034)));
                            }
                            let _e1194 = phi_12167_;
                            let _e1202 = ((((0.27f * (1f - _e955)) + (0.58f * _e955)) * (1f - _e1194)) + (0.94f * _e1194));
                            if (_e944 != _e944) {
                                phi_20125_ = true;
                            } else {
                                phi_20125_ = (_e955 >= _e944);
                            }
                            let _e1206 = phi_20125_;
                            let _e1207 = select(_e944, _e955, _e1206);
                            if (_e1207 != _e1207) {
                                phi_20140_ = true;
                            } else {
                                phi_20140_ = (_e1194 >= _e1207);
                            }
                            let _e1211 = phi_20140_;
                            phi_12197_ = vec4<f32>(_e1202, _e1202, _e1202, select(_e1207, _e1194, _e1211));
                        }
                        let _e1215 = phi_12197_;
                        let _e1220 = (1f - _e1215.w);
                        phi_11222_ = (_e912 + 1u);
                        phi_11225_ = vec3<f32>(((_e914.x * _e1220) + (_e1215.x * _e1215.w)), ((_e914.y * _e1220) + (_e1215.y * _e1215.w)), ((_e914.z * _e1220) + (_e1215.z * _e1215.w)));
                    } else {
                        phi_11222_ = u32();
                        phi_11225_ = vec3<f32>();
                    }
                    let _e1236 = phi_11222_;
                    let _e1238 = phi_11225_;
                    continue;
                    continuing {
                        phi_11221_ = _e1236;
                        phi_11224_ = _e1238;
                        phi_22142_ = _e653;
                        break if !(_e915);
                    }
                }
                let _e1241 = phi_22142_;
                if _e1241 {
                    break;
                }
                phi_12218_ = 0u;
                let _e2005 = local_62;
                phi_12221_ = _e2005;
                loop {
                    let _e1243 = phi_12218_;
                    let _e1245 = phi_12221_;
                    local_53 = _e1245;
                    local_54 = _e1245;
                    local_55 = _e1245;
                    let _e1246 = (_e1243 < 4u);
                    if _e1246 {
                        if _e1246 {
                        } else {
                            phi_22261_ = true;
                            break;
                        }
                        let _e1251 = row.member[_e497].lines[_e1243];
                        let _e1253 = (1f / _e1251.size);
                        let _e1260 = ((_e781 - _e1251.origin.x) * _e1253);
                        phi_12248_ = 0u;
                        phi_12251_ = _e1251.count;
                        loop {
                            let _e1265 = phi_12248_;
                            let _e1267 = phi_12251_;
                            local_49 = _e1265;
                            let _e1268 = (_e1265 < _e1267);
                            if _e1268 {
                                let _e1271 = (_e1265 + ((_e1267 - _e1265) / 2u));
                                let _e1276 = placed_glyphs.member[(_e1251.first + _e1271)].x;
                                let _e1277 = (_e1276 <= _e1260);
                                if _e1277 {
                                    phi_12279_ = (_e1271 + 1u);
                                } else {
                                    phi_12279_ = _e1265;
                                }
                                let _e1280 = phi_12279_;
                                phi_12249_ = _e1280;
                                phi_12252_ = select(_e1271, _e1267, _e1277);
                            } else {
                                phi_12249_ = u32();
                                phi_12252_ = u32();
                            }
                            let _e1283 = phi_12249_;
                            let _e1285 = phi_12252_;
                            continue;
                            continuing {
                                phi_12248_ = _e1283;
                                phi_12251_ = _e1285;
                                break if !(_e1268);
                            }
                        }
                        let _e1287 = (3.5f / _e1251.size);
                        let _e1289 = local_49;
                        let _e1290 = (_e1289 + 1u);
                        phi_12291_ = select(_e1290, _e1251.count, (_e1251.count < _e1290));
                        phi_12294_ = -1000000f;
                        loop {
                            let _e1294 = phi_12291_;
                            let _e1296 = phi_12294_;
                            local_52 = _e1296;
                            if (_e1294 > 0u) {
                                let _e1298 = (_e1294 - 1u);
                                let _e1299 = (_e1251.first + _e1298);
                                let _e1303 = placed_glyphs.member[_e1299].x;
                                let _e1307 = placed_glyphs.member[_e1299].glyph;
                                let _e1312 = glyphs.member[_e1307].min[0u];
                                let _e1317 = glyphs.member[_e1307].min[1u];
                                let _e1322 = glyphs.member[_e1307].max[0u];
                                let _e1327 = glyphs.member[_e1307].max[1u];
                                let _e1331 = glyphs.member[_e1307].start;
                                let _e1335 = glyphs.member[_e1307].count;
                                let _e1336 = (_e1260 - _e1303);
                                let _e1337 = -(((_e782 - _e1251.origin.y) * _e1253));
                                let _e1338 = (_e1322 + _e1287);
                                let _e1339 = (_e1336 > _e1338);
                                if _e1339 {
                                    phi_12424_ = f32();
                                } else {
                                    if (_e1336 >= (_e1312 - _e1287)) {
                                        if (_e1337 >= (_e1317 - _e1287)) {
                                            if (_e1336 <= _e1338) {
                                                if (_e1337 <= (_e1327 + _e1287)) {
                                                    phi_12374_ = 340282350000000000000000000000000000000f;
                                                    phi_12377_ = 0u;
                                                    phi_12379_ = 0i;
                                                    loop {
                                                        let _e1349 = phi_12374_;
                                                        let _e1351 = phi_12377_;
                                                        let _e1353 = phi_12379_;
                                                        local_50 = _e1349;
                                                        local_51 = _e1353;
                                                        let _e1354 = (_e1351 < _e1335);
                                                        if _e1354 {
                                                            let _e1358 = edges.member[(_e1331 + _e1351)];
                                                            let _e1360 = cantus_render_text_edge_distance(_e1358, _e1251.weight, vec2<f32>(_e1336, _e1337), _e1349);
                                                            phi_12375_ = _e1360.member;
                                                            phi_12378_ = (_e1351 + 1u);
                                                            phi_12380_ = (_e1353 + _e1360.member_1);
                                                        } else {
                                                            phi_12375_ = f32();
                                                            phi_12378_ = u32();
                                                            phi_12380_ = i32();
                                                        }
                                                        let _e1366 = phi_12375_;
                                                        let _e1368 = phi_12378_;
                                                        let _e1370 = phi_12380_;
                                                        continue;
                                                        continuing {
                                                            phi_12374_ = _e1366;
                                                            phi_12377_ = _e1368;
                                                            phi_12379_ = _e1370;
                                                            break if !(_e1354);
                                                        }
                                                    }
                                                    let _e1373 = local_50;
                                                    let _e1375 = ((_e1373 * _e1251.size) * _e1251.size);
                                                    if (_e1375 >= 12.25f) {
                                                        phi_12412_ = 3.5f;
                                                    } else {
                                                        phi_12412_ = sqrt(_e1375);
                                                    }
                                                    let _e1379 = phi_12412_;
                                                    let _e1381 = local_51;
                                                    let _e1384 = (_e1379 * select(1f, -1f, (_e1381 == 0i)));
                                                    if (_e1296 != _e1296) {
                                                        phi_20155_ = true;
                                                    } else {
                                                        phi_20155_ = (_e1384 >= _e1296);
                                                    }
                                                    let _e1388 = phi_20155_;
                                                    phi_12420_ = select(_e1296, _e1384, _e1388);
                                                } else {
                                                    phi_12420_ = _e1296;
                                                }
                                                let _e1391 = phi_12420_;
                                                phi_12421_ = _e1391;
                                            } else {
                                                phi_12421_ = _e1296;
                                            }
                                            let _e1393 = phi_12421_;
                                            phi_12422_ = _e1393;
                                        } else {
                                            phi_12422_ = _e1296;
                                        }
                                        let _e1395 = phi_12422_;
                                        phi_12423_ = _e1395;
                                    } else {
                                        phi_12423_ = _e1296;
                                    }
                                    let _e1397 = phi_12423_;
                                    phi_12424_ = _e1397;
                                }
                                let _e1399 = phi_12424_;
                                phi_12292_ = _e1298;
                                phi_12295_ = _e1399;
                                phi_12426_ = select(true, false, _e1339);
                            } else {
                                phi_12292_ = u32();
                                phi_12295_ = f32();
                                phi_12426_ = false;
                            }
                            let _e1402 = phi_12292_;
                            let _e1404 = phi_12295_;
                            let _e1406 = phi_12426_;
                            continue;
                            continuing {
                                phi_12291_ = _e1402;
                                phi_12294_ = _e1404;
                                break if !(_e1406);
                            }
                        }
                        let _e1409 = local_52;
                        let _e1411 = ((_e1409 * 1.25f) + 0.5f);
                        let _e1413 = select(_e1411, 0f, (_e1411 < 0f));
                        let _e1415 = select(_e1413, 1f, (_e1413 > 1f));
                        let _e1419 = ((_e1415 * _e1415) * (3f - (2f * _e1415)));
                        let _e1421 = unpack4x8unorm(_e1251.color);
                        let _e1425 = (1f - _e1419);
                        phi_12219_ = (_e1243 + 1u);
                        phi_12222_ = vec3<f32>(((_e1245.x * _e1425) + (_e1421.x * _e1419)), ((_e1245.y * _e1425) + (_e1421.y * _e1419)), ((_e1245.z * _e1425) + (_e1421.z * _e1419)));
                    } else {
                        phi_12219_ = u32();
                        phi_12222_ = vec3<f32>();
                    }
                    let _e1441 = phi_12219_;
                    let _e1443 = phi_12222_;
                    continue;
                    continuing {
                        phi_12218_ = _e1441;
                        phi_12221_ = _e1443;
                        phi_22261_ = _e1241;
                        break if !(_e1246);
                    }
                }
                let _e1446 = phi_22261_;
                if _e1446 {
                    break;
                }
                let _e1448 = local_53;
                let _e1452 = local_54;
                let _e1456 = local_55;
                phi_12478_ = vec4<f32>((_e1448.x * _e742), (_e1452.y * _e742), (_e1456.z * _e742), _e755);
            }
            let _e1957 = phi_12478_;
            out_color = _e1957;
            break;
        }
    }
    return;
}

fn function_8() {
    let _e495 = vertex_7;
    let _e496 = _isthmus_instance_index_9;
    let _e505 = frame.member[0u].playhead_x;
    let _e511 = frame.member[0u].panel_height;
    let _e514 = (_e505 + ((((f32((_e495 & 1u)) * 2f) - 1f) * _e511) * 0.4f));
    let _e517 = (1f + (f32((_e495 >> bitcast<u32>(1i))) * (_e511 + 10f)));
    let _e522 = frame.member[0u].screen_size[0u];
    let _e527 = frame.member[0u].screen_size[1u];
    out_position = vec4<f32>((((_e514 / _e522) * 2f) - 1f), (((_e517 / _e527) * 2f) - 1f), 0f, 1f);
    out_world_pos[0u] = _e514;
    out_world_pos[1u] = _e517;
    out_isthmus_instance_index = _e496;
    return;
}

fn function_9() {
    var phi_20328_: bool;
    var phi_20343_: bool;
    var phi_20358_: bool;
    var phi_20375_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = world_pos_1;
            let _e497 = _isthmus_instance_index_10;
            let _e503 = frame.member[0u].launcher_open;
            if (_e503 > 0.5f) {
                discard;
            }
            let _e508 = frame.member[0u].playhead_x;
            let _e512 = frame.member[0u].panel_height;
            let _e515 = (_e496.x - _e508);
            let _e516 = (_e496.y - (6f + (_e512 * 0.5f)));
            let _e517 = abs(_e515);
            let _e518 = abs(_e516);
            let _e522 = state.member[_e497].bar_split;
            let _e525 = (_e512 * (0.5f - (0.375f * _e522)));
            let _e531 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e518 - ((_e512 - _e525) * 0.5f)), _e517), (_e525 * 0.5f), 4.5f);
            let _e534 = abs((_e517 - (4f * _e522)));
            let _e536 = (_e518 - (_e512 * 0.1f));
            if (_e536 != _e536) {
                phi_20328_ = true;
            } else {
                phi_20328_ = (0f >= _e536);
            }
            let _e540 = phi_20328_;
            let _e541 = select(_e536, 0f, _e540);
            let _e546 = (sqrt(((_e534 * _e534) + (_e541 * _e541))) - 3.5f);
            let _e551 = state.member[_e497].icon_morph;
            let _e555 = state.member[_e497].icon_presence;
            let _e559 = ((_e512 * 0.18f) * (1f + (_e551 * (1f - _e555))));
            let _e561 = (_e559 * 0.5f);
            let _e562 = abs(-(_e516));
            let _e564 = (_e562 + (1.7320508f * _e515));
            if (_e564 != _e564) {
                phi_20343_ = true;
            } else {
                phi_20343_ = (0f >= _e564);
            }
            let _e568 = phi_20343_;
            let _e569 = select(_e564, 0f, _e568);
            let _e572 = (_e562 - (0.5f * _e569));
            let _e574 = (_e559 - _e561);
            let _e576 = (_e574 * -0.8660254f);
            let _e577 = (_e574 * 0.8660254f);
            if (_e576 <= _e577) {
            } else {
                break;
            }
            let _e580 = select(_e572, _e576, (_e572 < _e576));
            let _e583 = (_e572 - select(_e580, _e577, (_e580 > _e577)));
            let _e584 = ((_e515 - (_e569 * 0.8660254f)) - (-0.5f * _e574));
            let _e595 = (_e546 + ((((sqrt(((_e583 * _e583) + (_e584 * _e584))) * select(1f, -1f, (_e584 > 0f))) - _e561) - _e546) * _e551));
            let _e596 = (_e531 - -0.8f);
            let _e598 = select(_e596, 0f, (_e596 < 0f));
            let _e600 = select(_e598, 1f, (_e598 > 1f));
            let _e605 = (1f - ((_e600 * _e600) * (3f - (2f * _e600))));
            let _e606 = (_e595 - -0.8f);
            let _e608 = select(_e606, 0f, (_e606 < 0f));
            let _e610 = select(_e608, 1f, (_e608 > 1f));
            let _e616 = ((1f - ((_e610 * _e610) * (3f - (2f * _e610)))) * _e555);
            if (_e616 != _e616) {
                phi_20358_ = true;
            } else {
                phi_20358_ = (_e605 >= _e616);
            }
            let _e620 = phi_20358_;
            let _e621 = select(_e616, _e605, _e620);
            if (_e621 <= 0f) {
                discard;
            }
            if (_e531 != _e531) {
                phi_20375_ = true;
            } else {
                phi_20375_ = (_e595 <= _e531);
            }
            let _e626 = phi_20375_;
            let _e629 = ((select(_e531, _e595, _e626) - -2.5f) * 0.6666667f);
            let _e631 = select(_e629, 0f, (_e629 < 0f));
            let _e633 = select(_e631, 1f, (_e631 > 1f));
            let _e637 = ((_e633 * _e633) * (3f - (2f * _e633)));
            let _e638 = (1f - _e637);
            let _e641 = (0.15f * _e637);
            out_color = vec4<f32>((_e638 + _e641), ((0.878f * _e638) + _e641), ((0.824f * _e638) + _e641), _e621);
            break;
        }
    }
    return;
}

fn function_10() {
    var phi_20400_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_12925_: isthmus_Vertex_render_particles_Varyings;
    var phi_12926_: isthmus_Vertex_render_particles_Varyings;
    var phi_12927_: bool;
    var phi_12935_: isthmus_Vertex_render_particles_Varyings;
    var phi_12936_: isthmus_Vertex_render_particles_Varyings;

    let _e495 = vertex_7;
    let _e496 = _isthmus_instance_index_9;
    let _e500 = frame.member[0u].launcher_open;
    if (_e500 > 0.5f) {
        phi_12936_ = isthmus_Vertex_render_particles_Varyings(isthmus_Vertex_render_text_Varyings(vec4<f32>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f)), vec4<f32>(0f, 0f, 0f, 0f));
    } else {
        let _e505 = frame.member[0u].time;
        let _e509 = particle.member[_e496].end_time;
        let _e513 = particle.member[_e496].duration;
        let _e515 = (_e505 - (_e509 - _e513));
        if (_e515 < 0f) {
            phi_12926_ = isthmus_Vertex_render_particles_Varyings();
            phi_12927_ = true;
        } else {
            let _e517 = (_e515 > _e513);
            if _e517 {
                phi_12925_ = isthmus_Vertex_render_particles_Varyings();
            } else {
                let _e518 = (_e515 / _e513);
                let _e523 = particle.member[_e496].spawn_vel[0u];
                let _e528 = particle.member[_e496].spawn_vel[1u];
                let _e532 = sqrt(((_e523 * _e523) + (_e528 * _e528)));
                if (_e532 > 0.001f) {
                    phi_20400_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e523 / _e532), (_e528 / _e532)), _e532);
                } else {
                    phi_20400_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e532);
                }
                let _e540 = phi_20400_;
                let _e552 = ((f32((_e495 & 1u)) * 2f) - 1f);
                let _e553 = ((f32((_e495 >> bitcast<u32>(1i))) * 2f) - 1f);
                let _e556 = (_e518 + 0.5f);
                let _e557 = ((_e552 * 5f) * _e556);
                let _e558 = ((_e553 * 2.5f) * _e556);
                let _e563 = particle.member[_e496].spawn_pos[0u];
                let _e568 = particle.member[_e496].spawn_pos[1u];
                let _e585 = particle.member[_e496].rgb;
                let _e586 = unpack4x8unorm(_e585);
                let _e595 = ((((_e586.x * 0.299f) + (_e586.y * 0.587f)) + (_e586.z * 0.114f)) * -1f);
                let _e615 = frame.member[0u].screen_size[0u];
                let _e620 = frame.member[0u].screen_size[1u];
                let _e629 = (_e515 * 6.6666665f);
                let _e631 = select(_e629, 0f, (_e629 < 0f));
                let _e633 = select(_e631, 1f, (_e631 > 1f));
                phi_12925_ = isthmus_Vertex_render_particles_Varyings(isthmus_Vertex_render_text_Varyings(vec4<f32>(((((_e595 + (_e586.x * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e595 + (_e586.y * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e595 + (_e586.z * 2f)) * 0.8f) + 0.2f) * 2f), (((1f - _e518) * ((_e633 * _e633) * (3f - (2f * _e633)))) * 0.3f)), vec2<f32>(_e552, _e553)), vec4<f32>(((((((_e563 + (_e523 * _e515)) + (_e540.unnamed.x * _e557)) + (-(_e540.unnamed.y) * _e558)) / _e615) * 2f) - 1f), ((((((_e568 + (_e528 * _e515)) + (_e540.unnamed.y * _e557)) + (_e540.unnamed.x * _e558)) / _e620) * 2f) - 1f), 0f, 1f));
            }
            let _e645 = phi_12925_;
            phi_12926_ = _e645;
            phi_12927_ = _e517;
        }
        let _e647 = phi_12926_;
        let _e649 = phi_12927_;
        if _e649 {
            phi_12935_ = isthmus_Vertex_render_particles_Varyings(isthmus_Vertex_render_text_Varyings(vec4<f32>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f)), vec4<f32>(0f, 0f, 0f, 0f));
        } else {
            phi_12935_ = _e647;
        }
        let _e651 = phi_12935_;
        phi_12936_ = _e651;
    }
    let _e653 = phi_12936_;
    out_position = _e653.position;
    out_color = _e653.varyings.position;
    out_uv[0u] = _e653.varyings.varyings.x;
    out_uv[1u] = _e653.varyings.varyings.y;
    return;
}

fn function_11() {
    let _e495 = color_1;
    let _e496 = uv_1;
    let _e500 = (_e496.x * 0.8f);
    let _e506 = ((sqrt(((_e500 * _e500) + (_e496.y * _e496.y))) - 1f) * -1.25f);
    let _e508 = select(_e506, 0f, (_e506 < 0f));
    let _e510 = select(_e508, 1f, (_e508 > 1f));
    let _e515 = (_e495.w * ((_e510 * _e510) * (3f - (2f * _e510))));
    if (_e515 <= 0f) {
        discard;
    }
    out_color = vec4<f32>((_e495.x * _e515), (_e495.y * _e515), (_e495.z * _e515), _e515);
    return;
}

fn function_12() {
    var phi_20453_: array<f32, 2>;
    var phi_20456_: array<f32, 2>;
    var phi_20457_: bool;
    var phi_20470_: f32;
    var phi_20480_: array<f32, 2>;
    var phi_20505_: array<f32, 2>;
    var phi_20508_: array<f32, 2>;
    var phi_20509_: bool;
    var phi_20522_: f32;
    var phi_20532_: array<f32, 2>;
    var phi_13065_: u32;
    var phi_13068_: f32;
    var phi_13066_: u32;
    var phi_13069_: f32;
    var phi_22299_: bool;
    var local_63: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = vertex_7;
            let _e497 = _isthmus_instance_index_9;
            let _e501 = pill_2.member[_e497].calendar_expansion;
            let _e503 = select(_e501, 0f, (_e501 < 0f));
            let _e505 = select(_e503, 1f, (_e503 > 1f));
            let _e509 = ((_e505 * _e505) * (3f - (2f * _e505)));
            let _e513 = frame.member[0u].weather_hour;
            let _e517 = pill_2.member[_e497].sun_hours;
            let _e520 = (_e517[1] - _e517[0]);
            if (_e513 >= _e517[0]) {
                let _e522 = (_e513 <= _e517[1]);
                if _e522 {
                    let _e524 = ((_e513 - _e517[0]) / _e520);
                    phi_20453_ = array<f32, 2>(_e524, sin((_e524 * 3.1415927f)));
                } else {
                    phi_20453_ = array<f32, 2>();
                }
                let _e529 = phi_20453_;
                phi_20456_ = _e529;
                phi_20457_ = select(true, false, _e522);
            } else {
                phi_20456_ = array<f32, 2>();
                phi_20457_ = true;
            }
            let _e532 = phi_20456_;
            let _e534 = phi_20457_;
            if _e534 {
                let _e535 = (24f - _e520);
                if (_e513 < _e517[0]) {
                    phi_20470_ = (((_e513 + 24f) - _e517[1]) / _e535);
                } else {
                    phi_20470_ = ((_e513 - _e517[1]) / _e535);
                }
                let _e543 = phi_20470_;
                phi_20480_ = array<f32, 2>(select(0f, 1f, (_e513 >= _e517[1])), -(sin((_e543 * 3.1415927f))));
            } else {
                phi_20480_ = _e532;
            }
            let _e551 = phi_20480_;
            if (12f >= _e517[0]) {
                let _e555 = (12f <= _e517[1]);
                if _e555 {
                    let _e557 = ((12f - _e517[0]) / _e520);
                    phi_20505_ = array<f32, 2>(_e557, sin((_e557 * 3.1415927f)));
                } else {
                    phi_20505_ = array<f32, 2>();
                }
                let _e562 = phi_20505_;
                phi_20508_ = _e562;
                phi_20509_ = select(true, false, _e555);
            } else {
                phi_20508_ = array<f32, 2>();
                phi_20509_ = true;
            }
            let _e565 = phi_20508_;
            let _e567 = phi_20509_;
            if _e567 {
                let _e568 = (24f - _e520);
                if (12f < _e517[0]) {
                    phi_20522_ = ((36f - _e517[1]) / _e568);
                } else {
                    phi_20522_ = ((12f - _e517[1]) / _e568);
                }
                let _e575 = phi_20522_;
                phi_20532_ = array<f32, 2>(select(0f, 1f, (12f >= _e517[1])), -(sin((_e575 * 3.1415927f))));
            } else {
                phi_20532_ = _e565;
            }
            let _e583 = phi_20532_;
            let _e589 = pill_2.member[_e497].x;
            let _e598 = frame.member[0u].mouse_pressure;
            phi_13065_ = 0u;
            phi_13068_ = (_e598 * 8f);
            loop {
                let _e601 = phi_13065_;
                let _e603 = phi_13068_;
                local_63 = _e603;
                let _e604 = (_e601 < 4u);
                if _e604 {
                    if _e604 {
                    } else {
                        phi_22299_ = true;
                        break;
                    }
                    let _e610 = frame.member[0u].ripples[_e601].start_time;
                    let _e616 = frame.member[0u].ripples[_e601].strength;
                    let _e620 = frame.member[0u].time;
                    let _e622 = ((_e620 - _e610) * 1.2f);
                    let _e624 = select(_e622, 0f, (_e622 < 0f));
                    let _e627 = (1f - select(_e624, 1f, (_e624 > 1f)));
                    phi_13066_ = (_e601 + 1u);
                    phi_13069_ = (_e603 + (((_e616 * _e627) * _e627) * 11f));
                } else {
                    phi_13066_ = u32();
                    phi_13069_ = f32();
                }
                let _e634 = phi_13066_;
                let _e636 = phi_13069_;
                continue;
                continuing {
                    phi_13065_ = _e634;
                    phi_13068_ = _e636;
                    phi_22299_ = false;
                    break if !(_e604);
                }
            }
            let _e639 = phi_22299_;
            if _e639 {
                break;
            }
            let _e641 = local_63;
            let _e642 = (_e641 * 0.5f);
            let _e643 = (18f + _e642);
            let _e654 = frame.member[0u].panel_height;
            let _e662 = (((_e589 - (_e509 * 158f)) - _e643) + (f32((_e496 & 1u)) * ((308f + (316f * _e509)) + (_e643 * 2f))));
            let _e663 = ((-12f - _e642) + (f32((_e496 >> bitcast<u32>(1i))) * ((244f * _e509) + ((_e654 + _e643) * 2f))));
            let _e668 = frame.member[0u].screen_size[0u];
            let _e673 = frame.member[0u].screen_size[1u];
            out_position = vec4<f32>((((_e662 / _e668) * 2f) - 1f), (((_e663 / _e673) * 2f) - 1f), 0f, 1f);
            out_pixel[0u] = _e662;
            out_pixel[1u] = _e663;
            out_weather = vec4<f32>(_e551[0], _e551[1], _e583[1], _e509);
            out_isthmus_instance_index_1 = _e497;
            break;
        }
    }
    return;
}

fn function_13() {
    var phi_13316_: f32;
    var phi_13324_: u32;
    var phi_13325_: u32;
    var phi_22316_: bool;
    var phi_13525_: vec2<f32>;
    var phi_13528_: f32;
    var phi_13530_: u32;
    var phi_20633_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_20644_: bool;
    var phi_13635_: vec2<f32>;
    var phi_13636_: f32;
    var phi_13637_: vec2<f32>;
    var phi_13638_: f32;
    var phi_13526_: vec2<f32>;
    var phi_13529_: f32;
    var phi_13531_: u32;
    var phi_22329_: bool;
    var phi_13682_: f32;
    var local_64: vec2<f32>;
    var local_65: vec2<f32>;
    var phi_13694_: bool;
    var local_66: vec2<f32>;
    var phi_13705_: f32;
    var local_67: vec2<f32>;
    var phi_20670_: bool;
    var phi_20685_: bool;
    var phi_20700_: bool;
    var phi_20718_: bool;
    var phi_20733_: bool;
    var phi_20748_: bool;
    var phi_14027_: f32;
    var phi_14028_: render_tempestas_WeatherCondition;
    var phi_14029_: render_tempestas_WeatherCondition;
    var phi_20902_: array<f32, 2>;
    var phi_20905_: array<f32, 2>;
    var phi_20906_: bool;
    var phi_20919_: f32;
    var phi_20929_: array<f32, 2>;
    var phi_21014_: bool;
    var phi_21029_: bool;
    var phi_21044_: bool;
    var phi_14415_: vec3<f32>;
    var phi_14417_: vec2<f32>;
    var phi_14418_: render_tempestas_WeatherCondition;
    var phi_21068_: i32;
    var phi_21069_: f32;
    var phi_21070_: f32;
    var phi_21071_: vec2<f32>;
    var phi_21096_: i32;
    var phi_21097_: f32;
    var phi_21098_: f32;
    var phi_21099_: vec2<f32>;
    var local_68: f32;
    var phi_21110_: i32;
    var phi_21111_: f32;
    var phi_21112_: f32;
    var phi_21113_: vec2<f32>;
    var phi_21138_: i32;
    var phi_21139_: f32;
    var phi_21140_: f32;
    var phi_21141_: vec2<f32>;
    var local_69: f32;
    var local_70: f32;
    var phi_14760_: vec3<f32>;
    var phi_14967_: vec3<f32>;
    var phi_15161_: vec3<f32>;
    var phi_15355_: vec3<f32>;
    var phi_21152_: i32;
    var phi_21153_: f32;
    var phi_21154_: f32;
    var phi_21155_: vec2<f32>;
    var phi_21180_: i32;
    var phi_21181_: f32;
    var phi_21182_: f32;
    var phi_21183_: vec2<f32>;
    var local_71: f32;
    var phi_15446_: vec3<f32>;
    var phi_21207_: i32;
    var phi_21208_: f32;
    var phi_21209_: f32;
    var phi_21210_: vec2<f32>;
    var phi_21235_: i32;
    var phi_21236_: f32;
    var phi_21237_: f32;
    var phi_21238_: vec2<f32>;
    var local_72: f32;
    var phi_21249_: i32;
    var phi_21250_: f32;
    var phi_21251_: f32;
    var phi_21252_: vec2<f32>;
    var phi_21277_: i32;
    var phi_21278_: f32;
    var phi_21279_: f32;
    var phi_21280_: vec2<f32>;
    var local_73: f32;
    var local_74: f32;
    var phi_15905_: vec3<f32>;
    var phi_16112_: vec3<f32>;
    var phi_16306_: vec3<f32>;
    var phi_16500_: vec3<f32>;
    var phi_21291_: i32;
    var phi_21292_: f32;
    var phi_21293_: f32;
    var phi_21294_: vec2<f32>;
    var phi_21319_: i32;
    var phi_21320_: f32;
    var phi_21321_: f32;
    var phi_21322_: vec2<f32>;
    var local_75: f32;
    var phi_16591_: vec3<f32>;
    var phi_16631_: vec3<f32>;
    var phi_16632_: vec3<f32>;
    var phi_21346_: i32;
    var phi_21347_: f32;
    var phi_21348_: f32;
    var phi_21349_: vec2<f32>;
    var phi_21374_: i32;
    var phi_21375_: f32;
    var phi_21376_: f32;
    var phi_21377_: vec2<f32>;
    var local_76: f32;
    var phi_16721_: f32;
    var phi_16847_: vec3<f32>;
    var local_77: f32;
    var local_78: f32;
    var local_79: f32;
    var local_80: f32;
    var phi_16941_: vec3<f32>;
    var phi_16944_: i32;
    var phi_16979_: u32;
    var phi_16982_: u32;
    var phi_17010_: u32;
    var phi_16980_: u32;
    var phi_16983_: u32;
    var local_81: u32;
    var phi_17022_: u32;
    var phi_17025_: f32;
    var phi_17105_: f32;
    var phi_17108_: u32;
    var phi_17110_: i32;
    var phi_17106_: f32;
    var phi_17109_: u32;
    var phi_17111_: i32;
    var local_82: f32;
    var phi_17143_: f32;
    var local_83: i32;
    var phi_21403_: bool;
    var phi_17151_: f32;
    var phi_17152_: f32;
    var phi_17153_: f32;
    var phi_17154_: f32;
    var phi_17155_: f32;
    var phi_17023_: u32;
    var phi_17026_: f32;
    var phi_17157_: bool;
    var local_84: f32;
    var phi_16942_: vec3<f32>;
    var phi_16945_: i32;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e496 = pixel_4;
            let _e497 = weather_1;
            let _e498 = _isthmus_instance_index_11;
            let _e507 = frame.member[0u].launcher_open;
            if (_e507 > 0.5f) {
                discard;
            }
            let _e512 = pill_2.member[_e498].x;
            let _e516 = frame.member[0u].panel_height;
            let _e517 = (_e496.x - _e512);
            let _e518 = (_e496.y - 6f);
            let _e519 = (_e516 * 0.5f);
            let _e523 = ((308f - _e516) * 0.5f);
            let _e525 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e517 - 154f), (_e518 - _e519)), _e523, _e519);
            let _e529 = frame.member[0u].mouse_pressure;
            let _e530 = (_e529 > 0f);
            if _e530 {
                let _e535 = frame.member[0u].mouse_pos[0u];
                let _e540 = frame.member[0u].mouse_pos[1u];
                let _e546 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e535 - _e512) - 154f), ((_e540 - 6f) - _e519)), _e523, _e519);
                phi_13316_ = _e546;
            } else {
                phi_13316_ = 1f;
            }
            let _e548 = phi_13316_;
            phi_13324_ = 0u;
            loop {
                let _e550 = phi_13324_;
                let _e551 = (_e550 < 4u);
                if _e551 {
                    if _e551 {
                    } else {
                        phi_22316_ = true;
                        break;
                    }
                    phi_13325_ = (_e550 + 1u);
                } else {
                    phi_13325_ = u32();
                }
                let _e554 = phi_13325_;
                continue;
                continuing {
                    phi_13324_ = _e554;
                    phi_22316_ = false;
                    break if !(_e551);
                }
            }
            let _e557 = phi_22316_;
            if _e557 {
                break;
            }
            phi_13525_ = vec2<f32>(0f, 0f);
            phi_13528_ = 0f;
            phi_13530_ = 0u;
            loop {
                let _e560 = phi_13525_;
                let _e562 = phi_13528_;
                let _e564 = phi_13530_;
                local_64 = _e560;
                local_65 = _e560;
                local_66 = _e560;
                local_67 = _e560;
                local_77 = _e562;
                local_78 = _e562;
                local_79 = _e562;
                local_80 = _e562;
                let _e565 = (_e564 < 4u);
                if _e565 {
                    if _e565 {
                    } else {
                        phi_22329_ = true;
                        break;
                    }
                    let _e572 = frame.member[0u].ripples[_e564].origin[0u];
                    let _e579 = frame.member[0u].ripples[_e564].origin[1u];
                    let _e585 = frame.member[0u].ripples[_e564].start_time;
                    let _e591 = frame.member[0u].ripples[_e564].strength;
                    let _e595 = frame.member[0u].time;
                    let _e597 = ((_e595 - _e585) * 1.2f);
                    let _e599 = select(_e597, 0f, (_e597 < 0f));
                    let _e601 = select(_e599, 1f, (_e599 > 1f));
                    if (_e591 > 0f) {
                        if (_e601 < 1f) {
                            let _e604 = (_e496.x - _e572);
                            let _e605 = (_e496.y - _e579);
                            let _e609 = sqrt(((_e604 * _e604) + (_e605 * _e605)));
                            if (_e609 > 0.001f) {
                                phi_20633_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e604 / _e609), (_e605 / _e609)), _e609);
                            } else {
                                phi_20633_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e609);
                            }
                            let _e617 = phi_20633_;
                            let _e627 = ((abs((_e617.unnamed_1 - (_e601 * 600f))) - 80f) * -0.0125f);
                            let _e629 = select(_e627, 0f, (_e627 < 0f));
                            let _e631 = select(_e629, 1f, (_e629 > 1f));
                            let _e637 = (1f - _e601);
                            let _e638 = ((((_e631 * _e631) * (3f - (2f * _e631))) * _e591) * _e637);
                            let _e651 = (_e562 + (_e638 * 0.5f));
                            if (_e651 != _e651) {
                                phi_20644_ = true;
                            } else {
                                phi_20644_ = (1f <= _e651);
                            }
                            let _e655 = phi_20644_;
                            phi_13635_ = vec2<f32>((_e560.x + (((_e617.unnamed.x * _e638) * _e637) * 0.5f)), (_e560.y + (((_e617.unnamed.y * _e638) * _e637) * 0.5f)));
                            phi_13636_ = select(_e651, 1f, _e655);
                        } else {
                            phi_13635_ = _e560;
                            phi_13636_ = _e562;
                        }
                        let _e658 = phi_13635_;
                        let _e660 = phi_13636_;
                        phi_13637_ = _e658;
                        phi_13638_ = _e660;
                    } else {
                        phi_13637_ = _e560;
                        phi_13638_ = _e562;
                    }
                    let _e662 = phi_13637_;
                    let _e664 = phi_13638_;
                    phi_13526_ = _e662;
                    phi_13529_ = _e664;
                    phi_13531_ = (_e564 + 1u);
                } else {
                    phi_13526_ = vec2<f32>();
                    phi_13529_ = f32();
                    phi_13531_ = u32();
                }
                let _e667 = phi_13526_;
                let _e669 = phi_13529_;
                let _e671 = phi_13531_;
                continue;
                continuing {
                    phi_13525_ = _e667;
                    phi_13528_ = _e669;
                    phi_13530_ = _e671;
                    phi_22329_ = _e557;
                    break if !(_e565);
                }
            }
            let _e674 = phi_22329_;
            if _e674 {
                break;
            }
            if _e530 {
                let _e679 = frame.member[0u].mouse_pos[0u];
                let _e684 = frame.member[0u].mouse_pos[1u];
                let _e685 = (_e496.x - _e679);
                let _e686 = (_e496.y - _e684);
                let _e692 = ((sqrt(((_e685 * _e685) + (_e686 * _e686))) - 150f) * -0.006666667f);
                let _e694 = select(_e692, 0f, (_e692 < 0f));
                let _e696 = select(_e694, 1f, (_e694 > 1f));
                phi_13682_ = ((((_e696 * _e696) * (3f - (2f * _e696))) * _e529) * 8f);
            } else {
                phi_13682_ = 0f;
            }
            let _e704 = phi_13682_;
            let _e706 = local_64;
            let _e708 = global[0u];
            if (_e706.x == _e708) {
                let _e711 = local_65;
                let _e714 = global[1u];
                phi_13694_ = (_e711.y == _e714);
            } else {
                phi_13694_ = false;
            }
            let _e717 = phi_13694_;
            if _e717 {
                phi_13705_ = 0f;
            } else {
                let _e719 = local_66;
                phi_13705_ = (sqrt(((_e706.x * _e706.x) + (_e719.y * _e719.y))) * 22f);
            }
            let _e727 = phi_13705_;
            let _e729 = local_67;
            let _e735 = (_e512 - (_e497.w * 158f));
            let _e736 = (6f + _e516);
            let _e737 = (8f * _e497.w);
            let _e738 = ((244f * _e497.w) - _e737);
            if (_e738 != _e738) {
                phi_20670_ = true;
            } else {
                phi_20670_ = (0f >= _e738);
            }
            let _e742 = phi_20670_;
            let _e748 = frame.member[0u].mouse_pos[0u];
            let _e753 = frame.member[0u].mouse_pos[1u];
            let _e756 = ((308f + (316f * _e497.w)) * 0.5f);
            let _e757 = (select(_e738, 0f, _e742) * 0.5f);
            let _e758 = (_e737 + _e757);
            let _e761 = (_e757 != _e757);
            if _e761 {
                phi_20685_ = true;
            } else {
                phi_20685_ = (18f <= _e757);
            }
            let _e764 = phi_20685_;
            let _e767 = vec2<f32>(_e756, _e757);
            let _e768 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e496.x - _e735) - _e756), ((_e496.y - _e736) - _e758)), _e767, select(_e757, 18f, _e764));
            if _e761 {
                phi_20700_ = true;
            } else {
                phi_20700_ = (18f <= _e757);
            }
            let _e775 = phi_20700_;
            let _e778 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e748 - _e735) - _e756), ((_e753 - _e736) - _e758)), _e767, select(_e757, 18f, _e775));
            let _e781 = (0.5f + ((_e768 - _e525) * 0.008928572f));
            let _e783 = select(_e781, 0f, (_e781 < 0f));
            let _e785 = select(_e783, 1f, (_e783 > 1f));
            let _e798 = (0.5f + ((_e778 - _e548) * 0.008928572f));
            let _e800 = select(_e798, 0f, (_e798 < 0f));
            let _e802 = select(_e800, 1f, (_e800 > 1f));
            let _e814 = (((_e548 + ((((_e778 + ((_e548 - _e778) * _e802)) - ((56f * _e802) * (1f - _e802))) - _e548) * _e497.w)) - 0.5f) * -1f);
            let _e816 = select(_e814, 0f, (_e814 < 0f));
            let _e818 = select(_e816, 1f, (_e816 > 1f));
            let _e826 = ((_e525 + ((((_e768 + ((_e525 - _e768) * _e785)) - ((56f * _e785) * (1f - _e785))) - _e525) * _e497.w)) - (((_e704 * ((_e818 * _e818) * (3f - (2f * _e818)))) + _e727) * 0.5f));
            let _e827 = fwidth(_e826);
            if (_e827 != _e827) {
                phi_20718_ = true;
            } else {
                phi_20718_ = (0.55f >= _e827);
            }
            let _e831 = phi_20718_;
            let _e832 = select(_e827, 0.55f, _e831);
            let _e836 = ((_e826 - _e832) / (-(_e832) - _e832));
            let _e838 = select(_e836, 0f, (_e836 < 0f));
            let _e840 = select(_e838, 1f, (_e838 > 1f));
            let _e844 = ((_e840 * _e840) * (3f - (2f * _e840)));
            let _e845 = (_e826 != _e826);
            if _e845 {
                phi_20733_ = true;
            } else {
                phi_20733_ = (0f >= _e826);
            }
            let _e848 = phi_20733_;
            let _e852 = (exp((select(_e826, 0f, _e848) * -0.3f)) * 0.16f);
            if (_e844 != _e844) {
                phi_20748_ = true;
            } else {
                phi_20748_ = (_e852 >= _e844);
            }
            let _e856 = phi_20748_;
            let _e857 = select(_e844, _e852, _e856);
            if (_e857 <= 0.0009765625f) {
                discard;
            }
            let _e861 = ((_e518 - _e516) > (_e516 + 60f));
            let _e869 = (_e512 + 166f);
            let _e870 = (_e736 + (((56f + _e519) + (select(0f, 1f, _e861) * (_e516 + 8f))) - _e519));
            let _e871 = (_e496.x - _e869);
            let _e872 = (_e496.y - _e870);
            let _e873 = select(6u, 5u, _e861);
            let _e874 = (_e871 * 0.0034246575f);
            let _e877 = ((_e874 * f32(_e873)) - 0.5f);
            let _e879 = f32((_e873 - 1u));
            if (0f <= _e879) {
            } else {
                break;
            }
            let _e882 = select(_e877, 0f, (_e877 < 0f));
            let _e884 = select(_e882, _e879, (_e882 > _e879));
            let _e885 = floor(_e884);
            let _e890 = select(select(u32(_e885), 0u, (_e885 < 0f)), 4294967295u, (_e885 > 4294967000f));
            let _e892 = (_e884 - trunc(_e884));
            let _e894 = select(_e892, 0f, (_e892 < 0f));
            let _e896 = select(_e894, 1f, (_e894 > 1f));
            let _e900 = ((_e896 * _e896) * (3f - (2f * _e896)));
            if _e861 {
                if (_e890 < 5u) {
                } else {
                    break;
                }
                let _e928 = pill_2.member[_e498].daily_conditions[_e890];
                let _e929 = (_e890 + 1u);
                let _e931 = select(_e929, 4u, (4u < _e929));
                if (_e931 < 5u) {
                } else {
                    break;
                }
                let _e937 = pill_2.member[_e498].daily_conditions[_e931];
                phi_14027_ = 12f;
                phi_14028_ = _e937;
                phi_14029_ = _e928;
            } else {
                if (_e890 < 6u) {
                } else {
                    break;
                }
                let _e906 = pill_2.member[_e498].hourly_conditions[_e890];
                let _e907 = (_e890 + 1u);
                let _e909 = select(_e907, 5u, (5u < _e907));
                if (_e909 < 6u) {
                } else {
                    break;
                }
                let _e915 = pill_2.member[_e498].hourly_conditions[_e909];
                let _e919 = pill_2.member[_e498].hourly_start;
                phi_14027_ = ((_e919 + (_e884 * 4f)) % 24f);
                phi_14028_ = _e915;
                phi_14029_ = _e906;
            }
            let _e939 = phi_14027_;
            let _e941 = phi_14028_;
            let _e943 = phi_14029_;
            let _e947 = ((292f - _e516) * 0.5f);
            let _e949 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e871 - 146f), (_e872 - _e519)), _e947, _e519);
            let _e955 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e748 - _e869) - 146f), ((_e753 - _e870) - _e519)), _e947, _e519);
            let _e990 = pill_2.member[_e498].sun_hours;
            let _e993 = (_e990[1] - _e990[0]);
            if (_e939 >= _e990[0]) {
                let _e995 = (_e939 <= _e990[1]);
                if _e995 {
                    let _e997 = ((_e939 - _e990[0]) / _e993);
                    phi_20902_ = array<f32, 2>(_e997, sin((_e997 * 3.1415927f)));
                } else {
                    phi_20902_ = array<f32, 2>();
                }
                let _e1002 = phi_20902_;
                phi_20905_ = _e1002;
                phi_20906_ = select(true, false, _e995);
            } else {
                phi_20905_ = array<f32, 2>();
                phi_20906_ = true;
            }
            let _e1005 = phi_20905_;
            let _e1007 = phi_20906_;
            if _e1007 {
                let _e1008 = (24f - _e993);
                if (_e939 < _e990[0]) {
                    phi_20919_ = (((_e939 + 24f) - _e990[1]) / _e1008);
                } else {
                    phi_20919_ = ((_e939 - _e990[1]) / _e1008);
                }
                let _e1016 = phi_20919_;
                phi_20929_ = array<f32, 2>(select(0f, 1f, (_e939 >= _e990[1])), -(sin((_e1016 * 3.1415927f))));
            } else {
                phi_20929_ = _e1005;
            }
            let _e1024 = phi_20929_;
            let _e1027 = ((_e955 - 0.5f) * -1f);
            let _e1029 = select(_e1027, 0f, (_e1027 < 0f));
            let _e1031 = select(_e1029, 1f, (_e1029 > 1f));
            let _e1039 = (_e949 - (((_e704 * ((_e1031 * _e1031) * (3f - (2f * _e1031)))) + _e727) * 0.5f));
            let _e1044 = pill_2.member[_e498].hourly_conditions[0u];
            let _e1045 = (_e517 * 0.0032467532f);
            let _e1047 = select(_e1045, 0f, (_e1045 < 0f));
            let _e1056 = pill_2.member[_e498].hourly_conditions[1u];
            let _e1058 = ((abs((select(_e1047, 1f, (_e1047 > 1f)) - 0.5f)) - 0.05f) * 5f);
            let _e1060 = select(_e1058, 0f, (_e1058 < 0f));
            let _e1062 = select(_e1060, 1f, (_e1060 > 1f));
            let _e1066 = ((_e1062 * _e1062) * (3f - (2f * _e1062)));
            let _e1071 = (_e1044.fog + ((_e1056.fog - _e1044.fog) * _e1066));
            let _e1076 = (_e1044.cloud + ((_e1056.cloud - _e1044.cloud) * _e1066));
            let _e1081 = (_e1044.rain + ((_e1056.rain - _e1044.rain) * _e1066));
            let _e1086 = (_e1044.snow + ((_e1056.snow - _e1044.snow) * _e1066));
            let _e1091 = (_e1044.lightning + ((_e1056.lightning - _e1044.lightning) * _e1066));
            let _e1096 = (_e1044.hail + ((_e1056.hail - _e1044.hail) * _e1066));
            let _e1099 = (_e1071 + ((_e1044.fog - _e1071) * _e497.w));
            let _e1102 = (_e1076 + ((_e1044.cloud - _e1076) * _e497.w));
            let _e1105 = (_e1081 + ((_e1044.rain - _e1081) * _e497.w));
            let _e1108 = (_e1086 + ((_e1044.snow - _e1086) * _e497.w));
            let _e1111 = (_e1091 + ((_e1044.lightning - _e1091) * _e497.w));
            let _e1114 = (_e1096 + ((_e1044.hail - _e1096) * _e497.w));
            let _e1116 = (_e518 / _e516);
            if _e845 {
                phi_21014_ = true;
            } else {
                phi_21014_ = (0f <= _e826);
            }
            let _e1121 = phi_21014_;
            let _e1124 = (1f + (select(_e826, 0f, _e1121) * 0.008333334f));
            let _e1126 = select(_e1124, 0f, (_e1124 < 0f));
            let _e1128 = select(_e1126, 0.6f, (_e1126 > 0.6f));
            let _e1135 = (_e706.x * 0.04f);
            let _e1136 = (_e729.y * 0.04f);
            let _e1137 = ((_e1045 - (((_e1045 - 0.5f) * _e1128) * 0.08f)) - _e1135);
            let _e1138 = ((_e1116 - (((_e1116 - 0.5f) * _e1128) * 0.08f)) - _e1136);
            let _e1139 = (_e872 / _e516);
            if (_e1039 != _e1039) {
                phi_21029_ = true;
            } else {
                phi_21029_ = (0f <= _e1039);
            }
            let _e1145 = phi_21029_;
            let _e1148 = (1f + (select(_e1039, 0f, _e1145) * 0.008333334f));
            let _e1150 = select(_e1148, 0f, (_e1148 < 0f));
            let _e1152 = select(_e1150, 0.6f, (_e1150 > 0.6f));
            let _e1161 = fwidth(_e1039);
            if (_e1161 != _e1161) {
                phi_21044_ = true;
            } else {
                phi_21044_ = (0.55f >= _e1161);
            }
            let _e1165 = phi_21044_;
            let _e1166 = select(_e1161, 0.55f, _e1165);
            let _e1170 = ((_e1039 - _e1166) / (-(_e1166) - _e1166));
            let _e1172 = select(_e1170, 0f, (_e1170 < 0f));
            let _e1174 = select(_e1172, 1f, (_e1172 > 1f));
            let _e1178 = ((_e1174 * _e1174) * (3f - (2f * _e1174)));
            let _e1179 = (_e1178 > 0.001f);
            if _e1179 {
                let _e1231 = ((_e1024[1] - -0.04f) * 4.1666665f);
                let _e1233 = select(_e1231, 0f, (_e1231 < 0f));
                let _e1235 = select(_e1233, 1f, (_e1233 > 1f));
                let _e1239 = ((_e1235 * _e1235) * (3f - (2f * _e1235)));
                let _e1241 = ((_e1024[1] - -0.32f) * 4.166667f);
                let _e1243 = select(_e1241, 0f, (_e1241 < 0f));
                let _e1245 = select(_e1243, 1f, (_e1243 > 1f));
                let _e1253 = ((_e1024[1] - -0.18f) * 5.5555553f);
                let _e1255 = select(_e1253, 0f, (_e1253 < 0f));
                let _e1257 = select(_e1255, 1f, (_e1255 > 1f));
                let _e1263 = ((_e1024[1] - 0.2f) * -5.5555553f);
                let _e1265 = select(_e1263, 0f, (_e1263 < 0f));
                let _e1267 = select(_e1265, 1f, (_e1265 > 1f));
                phi_14415_ = vec3<f32>(_e1239, (((_e1245 * _e1245) * (3f - (2f * _e1245))) * (1f - _e1239)), (((_e1257 * _e1257) * (3f - (2f * _e1257))) * ((_e1267 * _e1267) * (3f - (2f * _e1267)))));
                phi_14417_ = vec2<f32>((((_e874 - (((_e874 - 0.5f) * _e1152) * 0.08f)) - _e1135) * 292f), (((_e1139 - (((_e1139 - 0.5f) * _e1152) * 0.08f)) - _e1136) * _e516));
                phi_14418_ = render_tempestas_WeatherCondition((_e943.fog + ((_e941.fog - _e943.fog) * _e900)), (_e943.cloud + ((_e941.cloud - _e943.cloud) * _e900)), (_e943.rain + ((_e941.rain - _e943.rain) * _e900)), (_e943.snow + ((_e941.snow - _e943.snow) * _e900)), (_e943.lightning + ((_e941.lightning - _e943.lightning) * _e900)), (_e943.hail + ((_e941.hail - _e943.hail) * _e900)));
            } else {
                let _e1184 = ((_e497.y - -0.04f) * 4.1666665f);
                let _e1186 = select(_e1184, 0f, (_e1184 < 0f));
                let _e1188 = select(_e1186, 1f, (_e1186 > 1f));
                let _e1192 = ((_e1188 * _e1188) * (3f - (2f * _e1188)));
                let _e1194 = ((_e497.y - -0.32f) * 4.166667f);
                let _e1196 = select(_e1194, 0f, (_e1194 < 0f));
                let _e1198 = select(_e1196, 1f, (_e1196 > 1f));
                let _e1206 = ((_e497.y - -0.18f) * 5.5555553f);
                let _e1208 = select(_e1206, 0f, (_e1206 < 0f));
                let _e1210 = select(_e1208, 1f, (_e1208 > 1f));
                let _e1216 = ((_e497.y - 0.2f) * -5.5555553f);
                let _e1218 = select(_e1216, 0f, (_e1216 < 0f));
                let _e1220 = select(_e1218, 1f, (_e1218 > 1f));
                phi_14415_ = vec3<f32>(_e1192, (((_e1198 * _e1198) * (3f - (2f * _e1198))) * (1f - _e1192)), (((_e1210 * _e1210) * (3f - (2f * _e1210))) * ((_e1220 * _e1220) * (3f - (2f * _e1220)))));
                phi_14417_ = vec2<f32>((_e1137 * 308f), (_e1138 * _e516));
                phi_14418_ = render_tempestas_WeatherCondition(_e1099, _e1102, _e1105, _e1108, _e1111, _e1114);
            }
            let _e1275 = phi_14415_;
            let _e1277 = phi_14417_;
            let _e1279 = phi_14418_;
            let _e1287 = frame.member[0u].time;
            let _e1288 = (_e1277.y / _e516);
            let _e1290 = ((_e1288 - 1f) * -1f);
            let _e1292 = select(_e1290, 0f, (_e1290 < 0f));
            let _e1294 = select(_e1292, 1f, (_e1292 > 1f));
            let _e1298 = ((_e1294 * _e1294) * (3f - (2f * _e1294)));
            let _e1299 = (1f - _e1298);
            let _e1319 = (1f - _e1275.x);
            let _e1331 = (0.3f * _e1299);
            let _e1332 = (0.22f * _e1298);
            let _e1339 = (_e1275.y * 0.8f);
            let _e1340 = (1f - _e1339);
            let _e1358 = (_e1275.z * 0.9f);
            let _e1359 = (1f - _e1358);
            let _e1371 = floor((_e1277.x * 0.055555556f));
            let _e1372 = floor((_e1277.y * 0.055555556f));
            let _e1376 = cantus_render_shader_hash(vec2<f32>(_e1371, _e1372));
            let _e1385 = (_e1277.x - (((_e1371 + 0.2f) + (_e1376.x * 0.6f)) * 18f));
            let _e1386 = (_e1277.y - (((_e1372 + 0.2f) + (_e1376.y * 0.6f)) * 18f));
            let _e1392 = ((sqrt(((_e1385 * _e1385) + (_e1386 * _e1386))) - 1f) * -1.6666666f);
            let _e1394 = select(_e1392, 0f, (_e1392 < 0f));
            let _e1396 = select(_e1394, 1f, (_e1394 > 1f));
            let _e1404 = cantus_render_shader_hash(vec2<f32>((_e1371 + 31.7f), (_e1372 + 31.7f)));
            let _e1407 = ((_e1404.x - 0.75f) * 4f);
            let _e1409 = select(_e1407, 0f, (_e1407 < 0f));
            let _e1411 = select(_e1409, 1f, (_e1409 > 1f));
            let _e1423 = ((((((_e1396 * _e1396) * (3f - (2f * _e1396))) * ((_e1411 * _e1411) * (3f - (2f * _e1411)))) * _e1319) * (1f - _e1279.cloud)) * (0.3f + (_e1298 * 0.7f)));
            let _e1424 = (((((((((0.006f * _e1299) + (0.025f * _e1298)) * _e1319) + (((0.08f * _e1299) + (0.32f * _e1298)) * _e1275.x)) * _e1340) + (((0.1f * _e1299) + _e1332) * _e1339)) * _e1359) + (((0.78f * _e1299) + (0.38f * _e1298)) * _e1358)) + _e1423);
            let _e1425 = (((((((((0.012f * _e1299) + (0.04f * _e1298)) * _e1319) + (((0.34f * _e1299) + (0.67f * _e1298)) * _e1275.x)) * _e1340) + (((0.16f * _e1299) + (0.25f * _e1298)) * _e1339)) * _e1359) + ((_e1331 + _e1332) * _e1358)) + _e1423);
            let _e1426 = (((((((((0.035f * _e1299) + (0.095f * _e1298)) * _e1319) + (((0.62f * _e1299) + (0.87f * _e1298)) * _e1275.x)) * _e1340) + ((_e1331 + (0.45f * _e1298)) * _e1339)) * _e1359) + (((0.2f * _e1299) + (0.42f * _e1298)) * _e1358)) + _e1423);
            if (_e1279.cloud > 0.0009765625f) {
                let _e1429 = (_e1277.x / _e516);
                phi_21068_ = 0i;
                phi_21069_ = 0.5f;
                phi_21070_ = 0f;
                phi_21071_ = vec2<f32>(((_e1429 * 0.14f) + (_e1287 * 0.012f)), ((_e1288 * 0.14f) + 6.1f));
                loop {
                    let _e1437 = phi_21068_;
                    let _e1439 = phi_21069_;
                    let _e1441 = phi_21070_;
                    let _e1443 = phi_21071_;
                    local_68 = _e1441;
                    let _e1444 = (_e1437 < 4i);
                    if _e1444 {
                        let _e1447 = cantus_render_shader_simplex_noise(_e1443);
                        phi_21096_ = (_e1437 + 1i);
                        phi_21097_ = (_e1439 * 0.5f);
                        phi_21098_ = (_e1441 + (_e1447 * _e1439));
                        phi_21099_ = vec2<f32>(((_e1443.x * 1.6f) + (_e1443.y * 1.2f)), ((_e1443.y * 1.6f) - (_e1443.x * 1.2f)));
                    } else {
                        phi_21096_ = i32();
                        phi_21097_ = f32();
                        phi_21098_ = f32();
                        phi_21099_ = vec2<f32>();
                    }
                    let _e1460 = phi_21096_;
                    let _e1462 = phi_21097_;
                    let _e1464 = phi_21098_;
                    let _e1466 = phi_21099_;
                    continue;
                    continuing {
                        phi_21068_ = _e1460;
                        phi_21069_ = _e1462;
                        phi_21070_ = _e1464;
                        phi_21071_ = _e1466;
                        break if !(_e1444);
                    }
                }
                let _e1469 = local_68;
                let _e1470 = (_e1469 * 0.5f);
                phi_21110_ = 0i;
                phi_21111_ = 0.5f;
                phi_21112_ = 0f;
                phi_21113_ = vec2<f32>(((_e1429 * 0.287f) + (_e1287 * 0.018f)), ((_e1288 * 0.287f) + -3.7f));
                loop {
                    let _e1479 = phi_21110_;
                    let _e1481 = phi_21111_;
                    let _e1483 = phi_21112_;
                    let _e1485 = phi_21113_;
                    local_69 = _e1483;
                    local_70 = _e1483;
                    let _e1486 = (_e1479 < 4i);
                    if _e1486 {
                        let _e1489 = cantus_render_shader_simplex_noise(_e1485);
                        phi_21138_ = (_e1479 + 1i);
                        phi_21139_ = (_e1481 * 0.5f);
                        phi_21140_ = (_e1483 + (_e1489 * _e1481));
                        phi_21141_ = vec2<f32>(((_e1485.x * 1.6f) + (_e1485.y * 1.2f)), ((_e1485.y * 1.6f) - (_e1485.x * 1.2f)));
                    } else {
                        phi_21138_ = i32();
                        phi_21139_ = f32();
                        phi_21140_ = f32();
                        phi_21141_ = vec2<f32>();
                    }
                    let _e1502 = phi_21138_;
                    let _e1504 = phi_21139_;
                    let _e1506 = phi_21140_;
                    let _e1508 = phi_21141_;
                    continue;
                    continuing {
                        phi_21110_ = _e1502;
                        phi_21111_ = _e1504;
                        phi_21112_ = _e1506;
                        phi_21113_ = _e1508;
                        break if !(_e1486);
                    }
                }
                let _e1511 = local_69;
                let _e1514 = local_70;
                let _e1518 = ((((0.5f + _e1470) + (_e1514 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1520 = select(_e1518, 0f, (_e1518 < 0f));
                let _e1522 = select(_e1520, 1f, (_e1520 > 1f));
                let _e1528 = (((_e1511 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1530 = select(_e1528, 0f, (_e1528 < 0f));
                let _e1532 = select(_e1530, 1f, (_e1530 > 1f));
                let _e1539 = ((_e1470 + 0.02000001f) * 4.5454545f);
                let _e1541 = select(_e1539, 0f, (_e1539 < 0f));
                let _e1543 = select(_e1541, 1f, (_e1541 > 1f));
                let _e1549 = ((((_e1532 * _e1532) * (3f - (2f * _e1532))) * 0.55f) + (((_e1543 * _e1543) * (3f - (2f * _e1543))) * 0.45f));
                let _e1550 = (1f - _e1549);
                let _e1587 = (_e1275.z * 0.45f);
                let _e1588 = (1f - _e1587);
                let _e1600 = (_e1279.cloud * (0.12f + (((_e1522 * _e1522) * (3f - (2f * _e1522))) * 0.7f)));
                let _e1601 = (1f - _e1600);
                phi_14760_ = vec3<f32>(((_e1424 * _e1601) + (((((((0.16f * _e1550) + (0.32f * _e1549)) * _e1319) + (((0.62f * _e1550) + (0.92f * _e1549)) * _e1275.x)) * _e1588) + (((0.5f * _e1550) + (0.76f * _e1549)) * _e1587)) * _e1600)), ((_e1425 * _e1601) + (((((((0.2f * _e1550) + (0.36f * _e1549)) * _e1319) + (((0.7f * _e1550) + (0.94f * _e1549)) * _e1275.x)) * _e1588) + (((0.36f * _e1550) + (0.59f * _e1549)) * _e1587)) * _e1600)), ((_e1426 * _e1601) + (((((((0.28f * _e1550) + (0.43f * _e1549)) * _e1319) + (((0.78f * _e1550) + (0.96f * _e1549)) * _e1275.x)) * _e1588) + (((0.4f * _e1550) + (0.56f * _e1549)) * _e1587)) * _e1600)));
            } else {
                phi_14760_ = vec3<f32>(_e1424, _e1425, _e1426);
            }
            let _e1613 = phi_14760_;
            let _e1616 = (1f - (_e1279.rain * 0.2f));
            let _e1626 = ((_e1613.x * _e1616) + (_e1279.rain * 0.020000001f));
            let _e1627 = ((_e1613.y * _e1616) + (_e1279.rain * 0.034f));
            let _e1628 = ((_e1613.z * _e1616) + (_e1279.rain * 0.05f));
            if (_e1279.rain > 0.0009765625f) {
                let _e1633 = (_e1277.x - (20f * _e1287));
                let _e1634 = (_e1277.y - (110f * _e1287));
                let _e1637 = floor((_e1633 * 0.06666667f));
                let _e1638 = floor((_e1634 * 0.04f));
                let _e1640 = cantus_render_shader_hash(vec2<f32>(_e1637, _e1638));
                let _e1651 = (_e1633 - (((_e1637 + 0.15f) + (_e1640.x * 0.7f)) * 15f));
                let _e1652 = (_e1634 - (((_e1638 + 0.15f) + (_e1640.y * 0.7f)) * 25f));
                let _e1656 = (((_e1651 * 1.8000001f) + (_e1652 * 9f)) * 0.011870845f);
                let _e1658 = select(_e1656, 0f, (_e1656 < 0f));
                let _e1660 = select(_e1658, 1f, (_e1658 > 1f));
                let _e1663 = (_e1651 - (1.8000001f * _e1660));
                let _e1664 = (_e1652 - (9f * _e1660));
                let _e1670 = ((sqrt(((_e1663 * _e1663) + (_e1664 * _e1664))) - 1.0999999f) * -1.666667f);
                let _e1672 = select(_e1670, 0f, (_e1670 < 0f));
                let _e1674 = select(_e1672, 1f, (_e1672 > 1f));
                let _e1682 = cantus_render_shader_hash(vec2<f32>((_e1637 + 19.3f), (_e1638 + 19.3f)));
                let _e1685 = ((_e1682.x - 0.22000003f) * 1.2820513f);
                let _e1687 = select(_e1685, 0f, (_e1685 < 0f));
                let _e1689 = select(_e1687, 1f, (_e1687 > 1f));
                let _e1696 = (((((_e1674 * _e1674) * (3f - (2f * _e1674))) * ((_e1689 * _e1689) * (3f - (2f * _e1689)))) * _e1279.rain) * 0.7f);
                let _e1698 = select(_e1696, 0f, (_e1696 < 0f));
                let _e1700 = select(_e1698, 1f, (_e1698 > 1f));
                let _e1701 = (1f - _e1700);
                phi_14967_ = vec3<f32>(((_e1626 * _e1701) + (0.52f * _e1700)), ((_e1627 * _e1701) + (0.72f * _e1700)), ((_e1628 * _e1701) + (0.9f * _e1700)));
            } else {
                phi_14967_ = vec3<f32>(_e1626, _e1627, _e1628);
            }
            let _e1713 = phi_14967_;
            if (_e1279.snow > 0.0009765625f) {
                let _e1718 = (_e1277.x - (5f * _e1287));
                let _e1719 = (_e1277.y - (14f * _e1287));
                let _e1722 = floor((_e1718 * 0.05f));
                let _e1723 = floor((_e1719 * 0.05f));
                let _e1727 = cantus_render_shader_hash(vec2<f32>((_e1722 + 31.7f), (_e1723 + 31.7f)));
                let _e1738 = (_e1718 - (((_e1722 + 0.15f) + (_e1727.x * 0.7f)) * 20f));
                let _e1739 = (_e1719 - (((_e1723 + 0.15f) + (_e1727.y * 0.7f)) * 20f));
                let _e1743 = (((_e1738 * 0.080000006f) + (_e1739 * 0.4f)) * 6.009615f);
                let _e1745 = select(_e1743, 0f, (_e1743 < 0f));
                let _e1747 = select(_e1745, 1f, (_e1745 > 1f));
                let _e1750 = (_e1738 - (0.080000006f * _e1747));
                let _e1751 = (_e1739 - (0.4f * _e1747));
                let _e1757 = ((sqrt(((_e1750 * _e1750) + (_e1751 * _e1751))) - 1.5999999f) * -1.666667f);
                let _e1759 = select(_e1757, 0f, (_e1757 < 0f));
                let _e1761 = select(_e1759, 1f, (_e1759 > 1f));
                let _e1769 = cantus_render_shader_hash(vec2<f32>((_e1722 + 19.3f), (_e1723 + 19.3f)));
                let _e1772 = ((_e1769.x - 0.3f) * 1.4285715f);
                let _e1774 = select(_e1772, 0f, (_e1772 < 0f));
                let _e1776 = select(_e1774, 1f, (_e1774 > 1f));
                let _e1783 = (((((_e1761 * _e1761) * (3f - (2f * _e1761))) * ((_e1776 * _e1776) * (3f - (2f * _e1776)))) * _e1279.snow) * 0.92f);
                let _e1785 = select(_e1783, 0f, (_e1783 < 0f));
                let _e1787 = select(_e1785, 1f, (_e1785 > 1f));
                let _e1788 = (1f - _e1787);
                let _e1795 = (0.96f * _e1787);
                phi_15161_ = vec3<f32>(((_e1713.x * _e1788) + _e1795), ((_e1713.y * _e1788) + _e1795), ((_e1713.z * _e1788) + _e1795));
            } else {
                phi_15161_ = _e1713;
            }
            let _e1801 = phi_15161_;
            if (_e1279.hail > 0.0009765625f) {
                let _e1806 = (_e1277.x - (18f * _e1287));
                let _e1807 = (_e1277.y - (85f * _e1287));
                let _e1810 = floor((_e1806 * 0.04347826f));
                let _e1811 = floor((_e1807 * 0.04347826f));
                let _e1815 = cantus_render_shader_hash(vec2<f32>((_e1810 + 63.4f), (_e1811 + 63.4f)));
                let _e1826 = (_e1806 - (((_e1810 + 0.15f) + (_e1815.x * 0.7f)) * 23f));
                let _e1827 = (_e1807 - (((_e1811 + 0.15f) + (_e1815.y * 0.7f)) * 23f));
                let _e1831 = (((_e1826 * 0.24000001f) + (_e1827 * 1.2f)) * 0.667735f);
                let _e1833 = select(_e1831, 0f, (_e1831 < 0f));
                let _e1835 = select(_e1833, 1f, (_e1833 > 1f));
                let _e1838 = (_e1826 - (0.24000001f * _e1835));
                let _e1839 = (_e1827 - (1.2f * _e1835));
                let _e1845 = ((sqrt(((_e1838 * _e1838) + (_e1839 * _e1839))) - 0.79999995f) * -1.6666667f);
                let _e1847 = select(_e1845, 0f, (_e1845 < 0f));
                let _e1849 = select(_e1847, 1f, (_e1847 > 1f));
                let _e1857 = cantus_render_shader_hash(vec2<f32>((_e1810 + 19.3f), (_e1811 + 19.3f)));
                let _e1860 = ((_e1857.x - 0.7f) * 3.3333333f);
                let _e1862 = select(_e1860, 0f, (_e1860 < 0f));
                let _e1864 = select(_e1862, 1f, (_e1862 > 1f));
                let _e1871 = (((((_e1849 * _e1849) * (3f - (2f * _e1849))) * ((_e1864 * _e1864) * (3f - (2f * _e1864)))) * _e1279.hail) * 0.7f);
                let _e1873 = select(_e1871, 0f, (_e1871 < 0f));
                let _e1875 = select(_e1873, 1f, (_e1873 > 1f));
                let _e1876 = (1f - _e1875);
                phi_15355_ = vec3<f32>(((_e1801.x * _e1876) + (0.75f * _e1875)), ((_e1801.y * _e1876) + (0.86f * _e1875)), ((_e1801.z * _e1876) + (0.94f * _e1875)));
            } else {
                phi_15355_ = _e1801;
            }
            let _e1891 = phi_15355_;
            let _e1895 = ((sin((_e1287 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1897 = select(_e1895, 0f, (_e1895 < 0f));
            let _e1899 = select(_e1897, 1f, (_e1897 > 1f));
            let _e1903 = ((_e1899 * _e1899) * (3f - (2f * _e1899)));
            let _e1905 = (_e1903 * _e1279.lightning);
            let _e1907 = (1f - (_e1905 * 0.55f));
            let _e1917 = ((_e1891.x * _e1907) + (_e1905 * 0.3575f));
            let _e1918 = ((_e1891.y * _e1907) + (_e1905 * 0.407f));
            let _e1919 = ((_e1891.z * _e1907) + (_e1905 * 0.528f));
            if (_e1279.fog > 0.0009765625f) {
                phi_21152_ = 0i;
                phi_21153_ = 0.5f;
                phi_21154_ = 0f;
                phi_21155_ = vec2<f32>((((_e1277.x / select(308f, 292f, _e1179)) * 0.9f) + (_e1287 * 0.008f)), ((_e1288 * 0.32f) + 12f));
                loop {
                    let _e1931 = phi_21152_;
                    let _e1933 = phi_21153_;
                    let _e1935 = phi_21154_;
                    let _e1937 = phi_21155_;
                    local_71 = _e1935;
                    let _e1938 = (_e1931 < 4i);
                    if _e1938 {
                        let _e1941 = cantus_render_shader_simplex_noise(_e1937);
                        phi_21180_ = (_e1931 + 1i);
                        phi_21181_ = (_e1933 * 0.5f);
                        phi_21182_ = (_e1935 + (_e1941 * _e1933));
                        phi_21183_ = vec2<f32>(((_e1937.x * 1.6f) + (_e1937.y * 1.2f)), ((_e1937.y * 1.6f) - (_e1937.x * 1.2f)));
                    } else {
                        phi_21180_ = i32();
                        phi_21181_ = f32();
                        phi_21182_ = f32();
                        phi_21183_ = vec2<f32>();
                    }
                    let _e1954 = phi_21180_;
                    let _e1956 = phi_21181_;
                    let _e1958 = phi_21182_;
                    let _e1960 = phi_21183_;
                    continue;
                    continuing {
                        phi_21152_ = _e1954;
                        phi_21153_ = _e1956;
                        phi_21154_ = _e1958;
                        phi_21155_ = _e1960;
                        break if !(_e1938);
                    }
                }
                let _e1963 = local_71;
                let _e1966 = (((_e1963 * 0.5f) + 0.15f) * 2.857143f);
                let _e1968 = select(_e1966, 0f, (_e1966 < 0f));
                let _e1970 = select(_e1968, 1f, (_e1968 > 1f));
                let _e1977 = (_e1279.fog * (0.58f + (((_e1970 * _e1970) * (3f - (2f * _e1970))) * 0.18f)));
                let _e1978 = (1f - _e1977);
                phi_15446_ = vec3<f32>(((_e1917 * _e1978) + (0.63f * _e1977)), ((_e1918 * _e1978) + (0.69f * _e1977)), ((_e1919 * _e1978) + (0.73f * _e1977)));
            } else {
                phi_15446_ = vec3<f32>(_e1917, _e1918, _e1919);
            }
            let _e1990 = phi_15446_;
            let _e1992 = ((select(_e826, _e1039, _e1179) - 5f) * -0.125f);
            let _e1994 = select(_e1992, 0f, (_e1992 < 0f));
            let _e1996 = select(_e1994, 1f, (_e1994 > 1f));
            let _e2001 = (((_e1996 * _e1996) * (3f - (2f * _e1996))) * 0.14f);
            let _e2009 = (_e1990 + vec3(_e2001));
            if _e1179 {
                if (_e1178 < 0.999f) {
                    let _e2011 = (_e1137 * 308f);
                    let _e2012 = (_e1138 * _e516);
                    let _e2014 = ((_e497.y - -0.04f) * 4.1666665f);
                    let _e2016 = select(_e2014, 0f, (_e2014 < 0f));
                    let _e2018 = select(_e2016, 1f, (_e2016 > 1f));
                    let _e2022 = ((_e2018 * _e2018) * (3f - (2f * _e2018)));
                    let _e2024 = ((_e497.y - -0.32f) * 4.166667f);
                    let _e2026 = select(_e2024, 0f, (_e2024 < 0f));
                    let _e2028 = select(_e2026, 1f, (_e2026 > 1f));
                    let _e2033 = (1f - _e2022);
                    let _e2036 = ((_e497.y - -0.18f) * 5.5555553f);
                    let _e2038 = select(_e2036, 0f, (_e2036 < 0f));
                    let _e2040 = select(_e2038, 1f, (_e2038 > 1f));
                    let _e2046 = ((_e497.y - 0.2f) * -5.5555553f);
                    let _e2048 = select(_e2046, 0f, (_e2046 < 0f));
                    let _e2050 = select(_e2048, 1f, (_e2048 > 1f));
                    let _e2055 = (((_e2040 * _e2040) * (3f - (2f * _e2040))) * ((_e2050 * _e2050) * (3f - (2f * _e2050))));
                    let _e2057 = ((_e1138 - 1f) * -1f);
                    let _e2059 = select(_e2057, 0f, (_e2057 < 0f));
                    let _e2061 = select(_e2059, 1f, (_e2059 > 1f));
                    let _e2065 = ((_e2061 * _e2061) * (3f - (2f * _e2061)));
                    let _e2066 = (1f - _e2065);
                    let _e2096 = (0.3f * _e2066);
                    let _e2097 = (0.22f * _e2065);
                    let _e2103 = ((((_e2028 * _e2028) * (3f - (2f * _e2028))) * _e2033) * 0.8f);
                    let _e2104 = (1f - _e2103);
                    let _e2121 = (_e2055 * 0.9f);
                    let _e2122 = (1f - _e2121);
                    let _e2134 = floor((_e1137 * 17.11111f));
                    let _e2135 = floor((_e2012 * 0.055555556f));
                    let _e2139 = cantus_render_shader_hash(vec2<f32>(_e2134, _e2135));
                    let _e2148 = (_e2011 - (((_e2134 + 0.2f) + (_e2139.x * 0.6f)) * 18f));
                    let _e2149 = (_e2012 - (((_e2135 + 0.2f) + (_e2139.y * 0.6f)) * 18f));
                    let _e2155 = ((sqrt(((_e2148 * _e2148) + (_e2149 * _e2149))) - 1f) * -1.6666666f);
                    let _e2157 = select(_e2155, 0f, (_e2155 < 0f));
                    let _e2159 = select(_e2157, 1f, (_e2157 > 1f));
                    let _e2167 = cantus_render_shader_hash(vec2<f32>((_e2134 + 31.7f), (_e2135 + 31.7f)));
                    let _e2170 = ((_e2167.x - 0.75f) * 4f);
                    let _e2172 = select(_e2170, 0f, (_e2170 < 0f));
                    let _e2174 = select(_e2172, 1f, (_e2172 > 1f));
                    let _e2185 = ((((((_e2159 * _e2159) * (3f - (2f * _e2159))) * ((_e2174 * _e2174) * (3f - (2f * _e2174)))) * _e2033) * (1f - _e1102)) * (0.3f + (_e2065 * 0.7f)));
                    let _e2186 = (((((((((0.006f * _e2066) + (0.025f * _e2065)) * _e2033) + (((0.08f * _e2066) + (0.32f * _e2065)) * _e2022)) * _e2104) + (((0.1f * _e2066) + _e2097) * _e2103)) * _e2122) + (((0.78f * _e2066) + (0.38f * _e2065)) * _e2121)) + _e2185);
                    let _e2187 = (((((((((0.012f * _e2066) + (0.04f * _e2065)) * _e2033) + (((0.34f * _e2066) + (0.67f * _e2065)) * _e2022)) * _e2104) + (((0.16f * _e2066) + (0.25f * _e2065)) * _e2103)) * _e2122) + ((_e2096 + _e2097) * _e2121)) + _e2185);
                    let _e2188 = (((((((((0.035f * _e2066) + (0.095f * _e2065)) * _e2033) + (((0.62f * _e2066) + (0.87f * _e2065)) * _e2022)) * _e2104) + ((_e2096 + (0.45f * _e2065)) * _e2103)) * _e2122) + (((0.2f * _e2066) + (0.42f * _e2065)) * _e2121)) + _e2185);
                    if (_e1102 > 0.0009765625f) {
                        let _e2191 = (_e2011 / _e516);
                        phi_21207_ = 0i;
                        phi_21208_ = 0.5f;
                        phi_21209_ = 0f;
                        phi_21210_ = vec2<f32>(((_e2191 * 0.14f) + (_e1287 * 0.012f)), ((_e1138 * 0.14f) + 6.1f));
                        loop {
                            let _e2199 = phi_21207_;
                            let _e2201 = phi_21208_;
                            let _e2203 = phi_21209_;
                            let _e2205 = phi_21210_;
                            local_72 = _e2203;
                            let _e2206 = (_e2199 < 4i);
                            if _e2206 {
                                let _e2209 = cantus_render_shader_simplex_noise(_e2205);
                                phi_21235_ = (_e2199 + 1i);
                                phi_21236_ = (_e2201 * 0.5f);
                                phi_21237_ = (_e2203 + (_e2209 * _e2201));
                                phi_21238_ = vec2<f32>(((_e2205.x * 1.6f) + (_e2205.y * 1.2f)), ((_e2205.y * 1.6f) - (_e2205.x * 1.2f)));
                            } else {
                                phi_21235_ = i32();
                                phi_21236_ = f32();
                                phi_21237_ = f32();
                                phi_21238_ = vec2<f32>();
                            }
                            let _e2222 = phi_21235_;
                            let _e2224 = phi_21236_;
                            let _e2226 = phi_21237_;
                            let _e2228 = phi_21238_;
                            continue;
                            continuing {
                                phi_21207_ = _e2222;
                                phi_21208_ = _e2224;
                                phi_21209_ = _e2226;
                                phi_21210_ = _e2228;
                                break if !(_e2206);
                            }
                        }
                        let _e2231 = local_72;
                        let _e2232 = (_e2231 * 0.5f);
                        phi_21249_ = 0i;
                        phi_21250_ = 0.5f;
                        phi_21251_ = 0f;
                        phi_21252_ = vec2<f32>(((_e2191 * 0.287f) + (_e1287 * 0.018f)), ((_e1138 * 0.287f) + -3.7f));
                        loop {
                            let _e2241 = phi_21249_;
                            let _e2243 = phi_21250_;
                            let _e2245 = phi_21251_;
                            let _e2247 = phi_21252_;
                            local_73 = _e2245;
                            local_74 = _e2245;
                            let _e2248 = (_e2241 < 4i);
                            if _e2248 {
                                let _e2251 = cantus_render_shader_simplex_noise(_e2247);
                                phi_21277_ = (_e2241 + 1i);
                                phi_21278_ = (_e2243 * 0.5f);
                                phi_21279_ = (_e2245 + (_e2251 * _e2243));
                                phi_21280_ = vec2<f32>(((_e2247.x * 1.6f) + (_e2247.y * 1.2f)), ((_e2247.y * 1.6f) - (_e2247.x * 1.2f)));
                            } else {
                                phi_21277_ = i32();
                                phi_21278_ = f32();
                                phi_21279_ = f32();
                                phi_21280_ = vec2<f32>();
                            }
                            let _e2264 = phi_21277_;
                            let _e2266 = phi_21278_;
                            let _e2268 = phi_21279_;
                            let _e2270 = phi_21280_;
                            continue;
                            continuing {
                                phi_21249_ = _e2264;
                                phi_21250_ = _e2266;
                                phi_21251_ = _e2268;
                                phi_21252_ = _e2270;
                                break if !(_e2248);
                            }
                        }
                        let _e2273 = local_73;
                        let _e2276 = local_74;
                        let _e2280 = ((((0.5f + _e2232) + (_e2276 * 0.12f)) - 0.35f) * 3.9999995f);
                        let _e2282 = select(_e2280, 0f, (_e2280 < 0f));
                        let _e2284 = select(_e2282, 1f, (_e2282 > 1f));
                        let _e2290 = (((_e2273 * 0.5f) + 0.08000001f) * 3.3333328f);
                        let _e2292 = select(_e2290, 0f, (_e2290 < 0f));
                        let _e2294 = select(_e2292, 1f, (_e2292 > 1f));
                        let _e2301 = ((_e2232 + 0.02000001f) * 4.5454545f);
                        let _e2303 = select(_e2301, 0f, (_e2301 < 0f));
                        let _e2305 = select(_e2303, 1f, (_e2303 > 1f));
                        let _e2311 = ((((_e2294 * _e2294) * (3f - (2f * _e2294))) * 0.55f) + (((_e2305 * _e2305) * (3f - (2f * _e2305))) * 0.45f));
                        let _e2312 = (1f - _e2311);
                        let _e2349 = (_e2055 * 0.45f);
                        let _e2350 = (1f - _e2349);
                        let _e2362 = (_e1102 * (0.12f + (((_e2284 * _e2284) * (3f - (2f * _e2284))) * 0.7f)));
                        let _e2363 = (1f - _e2362);
                        phi_15905_ = vec3<f32>(((_e2186 * _e2363) + (((((((0.16f * _e2312) + (0.32f * _e2311)) * _e2033) + (((0.62f * _e2312) + (0.92f * _e2311)) * _e2022)) * _e2350) + (((0.5f * _e2312) + (0.76f * _e2311)) * _e2349)) * _e2362)), ((_e2187 * _e2363) + (((((((0.2f * _e2312) + (0.36f * _e2311)) * _e2033) + (((0.7f * _e2312) + (0.94f * _e2311)) * _e2022)) * _e2350) + (((0.36f * _e2312) + (0.59f * _e2311)) * _e2349)) * _e2362)), ((_e2188 * _e2363) + (((((((0.28f * _e2312) + (0.43f * _e2311)) * _e2033) + (((0.78f * _e2312) + (0.96f * _e2311)) * _e2022)) * _e2350) + (((0.4f * _e2312) + (0.56f * _e2311)) * _e2349)) * _e2362)));
                    } else {
                        phi_15905_ = vec3<f32>(_e2186, _e2187, _e2188);
                    }
                    let _e2375 = phi_15905_;
                    let _e2377 = (1f - (_e1105 * 0.2f));
                    let _e2387 = ((_e2375.x * _e2377) + (_e1105 * 0.020000001f));
                    let _e2388 = ((_e2375.y * _e2377) + (_e1105 * 0.034f));
                    let _e2389 = ((_e2375.z * _e2377) + (_e1105 * 0.05f));
                    if (_e1105 > 0.0009765625f) {
                        let _e2394 = (_e2011 - (20f * _e1287));
                        let _e2395 = (_e2012 - (110f * _e1287));
                        let _e2398 = floor((_e2394 * 0.06666667f));
                        let _e2399 = floor((_e2395 * 0.04f));
                        let _e2401 = cantus_render_shader_hash(vec2<f32>(_e2398, _e2399));
                        let _e2412 = (_e2394 - (((_e2398 + 0.15f) + (_e2401.x * 0.7f)) * 15f));
                        let _e2413 = (_e2395 - (((_e2399 + 0.15f) + (_e2401.y * 0.7f)) * 25f));
                        let _e2417 = (((_e2412 * 1.8000001f) + (_e2413 * 9f)) * 0.011870845f);
                        let _e2419 = select(_e2417, 0f, (_e2417 < 0f));
                        let _e2421 = select(_e2419, 1f, (_e2419 > 1f));
                        let _e2424 = (_e2412 - (1.8000001f * _e2421));
                        let _e2425 = (_e2413 - (9f * _e2421));
                        let _e2431 = ((sqrt(((_e2424 * _e2424) + (_e2425 * _e2425))) - 1.0999999f) * -1.666667f);
                        let _e2433 = select(_e2431, 0f, (_e2431 < 0f));
                        let _e2435 = select(_e2433, 1f, (_e2433 > 1f));
                        let _e2443 = cantus_render_shader_hash(vec2<f32>((_e2398 + 19.3f), (_e2399 + 19.3f)));
                        let _e2446 = ((_e2443.x - 0.22000003f) * 1.2820513f);
                        let _e2448 = select(_e2446, 0f, (_e2446 < 0f));
                        let _e2450 = select(_e2448, 1f, (_e2448 > 1f));
                        let _e2457 = (((((_e2435 * _e2435) * (3f - (2f * _e2435))) * ((_e2450 * _e2450) * (3f - (2f * _e2450)))) * _e1105) * 0.7f);
                        let _e2459 = select(_e2457, 0f, (_e2457 < 0f));
                        let _e2461 = select(_e2459, 1f, (_e2459 > 1f));
                        let _e2462 = (1f - _e2461);
                        phi_16112_ = vec3<f32>(((_e2387 * _e2462) + (0.52f * _e2461)), ((_e2388 * _e2462) + (0.72f * _e2461)), ((_e2389 * _e2462) + (0.9f * _e2461)));
                    } else {
                        phi_16112_ = vec3<f32>(_e2387, _e2388, _e2389);
                    }
                    let _e2474 = phi_16112_;
                    if (_e1108 > 0.0009765625f) {
                        let _e2478 = (_e2011 - (5f * _e1287));
                        let _e2479 = (_e2012 - (14f * _e1287));
                        let _e2482 = floor((_e2478 * 0.05f));
                        let _e2483 = floor((_e2479 * 0.05f));
                        let _e2487 = cantus_render_shader_hash(vec2<f32>((_e2482 + 31.7f), (_e2483 + 31.7f)));
                        let _e2498 = (_e2478 - (((_e2482 + 0.15f) + (_e2487.x * 0.7f)) * 20f));
                        let _e2499 = (_e2479 - (((_e2483 + 0.15f) + (_e2487.y * 0.7f)) * 20f));
                        let _e2503 = (((_e2498 * 0.080000006f) + (_e2499 * 0.4f)) * 6.009615f);
                        let _e2505 = select(_e2503, 0f, (_e2503 < 0f));
                        let _e2507 = select(_e2505, 1f, (_e2505 > 1f));
                        let _e2510 = (_e2498 - (0.080000006f * _e2507));
                        let _e2511 = (_e2499 - (0.4f * _e2507));
                        let _e2517 = ((sqrt(((_e2510 * _e2510) + (_e2511 * _e2511))) - 1.5999999f) * -1.666667f);
                        let _e2519 = select(_e2517, 0f, (_e2517 < 0f));
                        let _e2521 = select(_e2519, 1f, (_e2519 > 1f));
                        let _e2529 = cantus_render_shader_hash(vec2<f32>((_e2482 + 19.3f), (_e2483 + 19.3f)));
                        let _e2532 = ((_e2529.x - 0.3f) * 1.4285715f);
                        let _e2534 = select(_e2532, 0f, (_e2532 < 0f));
                        let _e2536 = select(_e2534, 1f, (_e2534 > 1f));
                        let _e2543 = (((((_e2521 * _e2521) * (3f - (2f * _e2521))) * ((_e2536 * _e2536) * (3f - (2f * _e2536)))) * _e1108) * 0.92f);
                        let _e2545 = select(_e2543, 0f, (_e2543 < 0f));
                        let _e2547 = select(_e2545, 1f, (_e2545 > 1f));
                        let _e2548 = (1f - _e2547);
                        let _e2555 = (0.96f * _e2547);
                        phi_16306_ = vec3<f32>(((_e2474.x * _e2548) + _e2555), ((_e2474.y * _e2548) + _e2555), ((_e2474.z * _e2548) + _e2555));
                    } else {
                        phi_16306_ = _e2474;
                    }
                    let _e2561 = phi_16306_;
                    if (_e1114 > 0.0009765625f) {
                        let _e2565 = (_e2011 - (18f * _e1287));
                        let _e2566 = (_e2012 - (85f * _e1287));
                        let _e2569 = floor((_e2565 * 0.04347826f));
                        let _e2570 = floor((_e2566 * 0.04347826f));
                        let _e2574 = cantus_render_shader_hash(vec2<f32>((_e2569 + 63.4f), (_e2570 + 63.4f)));
                        let _e2585 = (_e2565 - (((_e2569 + 0.15f) + (_e2574.x * 0.7f)) * 23f));
                        let _e2586 = (_e2566 - (((_e2570 + 0.15f) + (_e2574.y * 0.7f)) * 23f));
                        let _e2590 = (((_e2585 * 0.24000001f) + (_e2586 * 1.2f)) * 0.667735f);
                        let _e2592 = select(_e2590, 0f, (_e2590 < 0f));
                        let _e2594 = select(_e2592, 1f, (_e2592 > 1f));
                        let _e2597 = (_e2585 - (0.24000001f * _e2594));
                        let _e2598 = (_e2586 - (1.2f * _e2594));
                        let _e2604 = ((sqrt(((_e2597 * _e2597) + (_e2598 * _e2598))) - 0.79999995f) * -1.6666667f);
                        let _e2606 = select(_e2604, 0f, (_e2604 < 0f));
                        let _e2608 = select(_e2606, 1f, (_e2606 > 1f));
                        let _e2616 = cantus_render_shader_hash(vec2<f32>((_e2569 + 19.3f), (_e2570 + 19.3f)));
                        let _e2619 = ((_e2616.x - 0.7f) * 3.3333333f);
                        let _e2621 = select(_e2619, 0f, (_e2619 < 0f));
                        let _e2623 = select(_e2621, 1f, (_e2621 > 1f));
                        let _e2630 = (((((_e2608 * _e2608) * (3f - (2f * _e2608))) * ((_e2623 * _e2623) * (3f - (2f * _e2623)))) * _e1114) * 0.7f);
                        let _e2632 = select(_e2630, 0f, (_e2630 < 0f));
                        let _e2634 = select(_e2632, 1f, (_e2632 > 1f));
                        let _e2635 = (1f - _e2634);
                        phi_16500_ = vec3<f32>(((_e2561.x * _e2635) + (0.75f * _e2634)), ((_e2561.y * _e2635) + (0.86f * _e2634)), ((_e2561.z * _e2635) + (0.94f * _e2634)));
                    } else {
                        phi_16500_ = _e2561;
                    }
                    let _e2650 = phi_16500_;
                    let _e2651 = (_e1903 * _e1111);
                    let _e2653 = (1f - (_e2651 * 0.55f));
                    let _e2663 = ((_e2650.x * _e2653) + (_e2651 * 0.3575f));
                    let _e2664 = ((_e2650.y * _e2653) + (_e2651 * 0.407f));
                    let _e2665 = ((_e2650.z * _e2653) + (_e2651 * 0.528f));
                    if (_e1099 > 0.0009765625f) {
                        phi_21291_ = 0i;
                        phi_21292_ = 0.5f;
                        phi_21293_ = 0f;
                        phi_21294_ = vec2<f32>(((_e1137 * 0.9f) + (_e1287 * 0.008f)), ((_e1138 * 0.32f) + 12f));
                        loop {
                            let _e2675 = phi_21291_;
                            let _e2677 = phi_21292_;
                            let _e2679 = phi_21293_;
                            let _e2681 = phi_21294_;
                            local_75 = _e2679;
                            let _e2682 = (_e2675 < 4i);
                            if _e2682 {
                                let _e2685 = cantus_render_shader_simplex_noise(_e2681);
                                phi_21319_ = (_e2675 + 1i);
                                phi_21320_ = (_e2677 * 0.5f);
                                phi_21321_ = (_e2679 + (_e2685 * _e2677));
                                phi_21322_ = vec2<f32>(((_e2681.x * 1.6f) + (_e2681.y * 1.2f)), ((_e2681.y * 1.6f) - (_e2681.x * 1.2f)));
                            } else {
                                phi_21319_ = i32();
                                phi_21320_ = f32();
                                phi_21321_ = f32();
                                phi_21322_ = vec2<f32>();
                            }
                            let _e2698 = phi_21319_;
                            let _e2700 = phi_21320_;
                            let _e2702 = phi_21321_;
                            let _e2704 = phi_21322_;
                            continue;
                            continuing {
                                phi_21291_ = _e2698;
                                phi_21292_ = _e2700;
                                phi_21293_ = _e2702;
                                phi_21294_ = _e2704;
                                break if !(_e2682);
                            }
                        }
                        let _e2707 = local_75;
                        let _e2710 = (((_e2707 * 0.5f) + 0.15f) * 2.857143f);
                        let _e2712 = select(_e2710, 0f, (_e2710 < 0f));
                        let _e2714 = select(_e2712, 1f, (_e2712 > 1f));
                        let _e2721 = (_e1099 * (0.58f + (((_e2714 * _e2714) * (3f - (2f * _e2714))) * 0.18f)));
                        let _e2722 = (1f - _e2721);
                        phi_16591_ = vec3<f32>(((_e2663 * _e2722) + (0.63f * _e2721)), ((_e2664 * _e2722) + (0.69f * _e2721)), ((_e2665 * _e2722) + (0.73f * _e2721)));
                    } else {
                        phi_16591_ = vec3<f32>(_e2663, _e2664, _e2665);
                    }
                    let _e2734 = phi_16591_;
                    let _e2736 = ((_e826 - 5f) * -0.125f);
                    let _e2738 = select(_e2736, 0f, (_e2736 < 0f));
                    let _e2740 = select(_e2738, 1f, (_e2738 > 1f));
                    let _e2745 = (((_e2740 * _e2740) * (3f - (2f * _e2740))) * 0.14f);
                    let _e2752 = (1f - _e1178);
                    phi_16631_ = vec3<f32>((((_e2734.x + _e2745) * _e2752) + ((_e1990.x + _e2001) * _e1178)), (((_e2734.y + _e2745) * _e2752) + ((_e1990.y + _e2001) * _e1178)), (((_e2734.z + _e2745) * _e2752) + ((_e1990.z + _e2001) * _e1178)));
                } else {
                    phi_16631_ = _e2009;
                }
                let _e2764 = phi_16631_;
                phi_16632_ = _e2764;
            } else {
                phi_16632_ = _e2009;
            }
            let _e2766 = phi_16632_;
            if (_e525 < 1f) {
                let _e2769 = (16f + (_e497.x * 276f));
                let _e2771 = select(_e497.y, 0f, (_e497.y < 0f));
                let _e2775 = (0.72f - (select(_e2771, 1f, (_e2771 > 1f)) * 0.45f));
                let _e2778 = ((_e497.y - 0.55f) * -1.8867923f);
                let _e2780 = select(_e2778, 0f, (_e2778 < 0f));
                let _e2782 = select(_e2780, 1f, (_e2780 > 1f));
                let _e2786 = ((_e2782 * _e2782) * (3f - (2f * _e2782)));
                let _e2787 = (1f - _e2786);
                if (_e1076 > 0.0009765625f) {
                    phi_21346_ = 0i;
                    phi_21347_ = 0.5f;
                    phi_21348_ = 0f;
                    phi_21349_ = vec2<f32>((((_e2769 / _e516) * 0.14f) + (_e1287 * 0.012f)), ((_e2775 * 0.14f) + 6.1f));
                    loop {
                        let _e2805 = phi_21346_;
                        let _e2807 = phi_21347_;
                        let _e2809 = phi_21348_;
                        let _e2811 = phi_21349_;
                        local_76 = _e2809;
                        let _e2812 = (_e2805 < 4i);
                        if _e2812 {
                            let _e2815 = cantus_render_shader_simplex_noise(_e2811);
                            phi_21374_ = (_e2805 + 1i);
                            phi_21375_ = (_e2807 * 0.5f);
                            phi_21376_ = (_e2809 + (_e2815 * _e2807));
                            phi_21377_ = vec2<f32>(((_e2811.x * 1.6f) + (_e2811.y * 1.2f)), ((_e2811.y * 1.6f) - (_e2811.x * 1.2f)));
                        } else {
                            phi_21374_ = i32();
                            phi_21375_ = f32();
                            phi_21376_ = f32();
                            phi_21377_ = vec2<f32>();
                        }
                        let _e2828 = phi_21374_;
                        let _e2830 = phi_21375_;
                        let _e2832 = phi_21376_;
                        let _e2834 = phi_21377_;
                        continue;
                        continuing {
                            phi_21346_ = _e2828;
                            phi_21347_ = _e2830;
                            phi_21348_ = _e2832;
                            phi_21349_ = _e2834;
                            break if !(_e2812);
                        }
                    }
                    let _e2837 = local_76;
                    let _e2840 = (((_e2837 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e2842 = select(_e2840, 0f, (_e2840 < 0f));
                    let _e2844 = select(_e2842, 1f, (_e2842 > 1f));
                    phi_16721_ = ((((_e2844 * _e2844) * (3f - (2f * _e2844))) * _e1076) * 0.82f);
                } else {
                    phi_16721_ = 0f;
                }
                let _e2852 = phi_16721_;
                let _e2854 = ((_e497.y - -0.02f) * 16.666668f);
                let _e2856 = select(_e2854, 0f, (_e2854 < 0f));
                let _e2858 = select(_e2856, 1f, (_e2856 > 1f));
                let _e2865 = (_e517 - _e2769);
                let _e2866 = (_e518 - (_e516 * _e2775));
                let _e2870 = sqrt(((_e2865 * _e2865) + (_e2866 * _e2866)));
                let _e2872 = ((_e2870 - 62f) * -0.01724138f);
                let _e2874 = select(_e2872, 0f, (_e2872 < 0f));
                let _e2876 = select(_e2874, 1f, (_e2874 > 1f));
                let _e2883 = ((_e2870 - 11f) * -0.1f);
                let _e2885 = select(_e2883, 0f, (_e2883 < 0f));
                let _e2887 = select(_e2885, 1f, (_e2885 > 1f));
                let _e2894 = (((((_e2876 * _e2876) * (3f - (2f * _e2876))) * 0.24f) + (((_e2887 * _e2887) * (3f - (2f * _e2887))) * 0.7f)) * (((_e2858 * _e2858) * (3f - (2f * _e2858))) * (1f - _e2852)));
                let _e2895 = (1f - _e2894);
                let _e2911 = ((_e525 - 1f) / ((_e516 * -0.25f) - 1f));
                let _e2913 = select(_e2911, 0f, (_e2911 < 0f));
                let _e2915 = select(_e2913, 1f, (_e2913 > 1f));
                let _e2919 = ((_e2915 * _e2915) * (3f - (2f * _e2915)));
                let _e2920 = (1f - _e2919);
                phi_16847_ = vec3<f32>(((_e2766.x * _e2920) + (((_e2766.x * _e2895) + (((0.96f * _e2787) + (0.98f * _e2786)) * _e2894)) * _e2919)), ((_e2766.y * _e2920) + (((_e2766.y * _e2895) + (((0.98f * _e2787) + (0.74f * _e2786)) * _e2894)) * _e2919)), ((_e2766.z * _e2920) + (((_e2766.z * _e2895) + ((_e2787 + (0.66f * _e2786)) * _e2894)) * _e2919)));
            } else {
                phi_16847_ = _e2766;
            }
            let _e2932 = phi_16847_;
            let _e2943 = local_77;
            let _e2944 = (1f - _e2943);
            let _e2949 = local_78;
            let _e2952 = local_79;
            let _e2955 = local_80;
            let _e2966 = floor(((_e496.x - (_e512 - 158f)) * 0.03846154f));
            let _e2967 = floor((_e518 / ((_e516 + 244f) * 0.027777778f)));
            let _e2969 = select(0f, _e2966, (_e2966 > 0f));
            let _e2971 = select(0f, _e2967, (_e2967 > 0f));
            let _e2977 = ((select(35f, _e2971, (_e2971 < 35f)) * 24f) + select(23f, _e2969, (_e2969 < 23f)));
            let _e2985 = text_cells.member[select(select(u32(_e2977), 0u, (_e2977 < 0f)), 4294967295u, (_e2977 > 4294967000f))];
            phi_16941_ = vec3<f32>(((_e2932.x * _e2944) + (((_e2932.x * 1.5f) + 0.1f) * _e2949)), ((_e2932.y * _e2944) + (((_e2932.y * 1.5f) + 0.1f) * _e2952)), ((_e2932.z * _e2944) + (((_e2932.z * 1.5f) + 0.1f) * _e2955)));
            phi_16944_ = 0i;
            loop {
                let _e2987 = phi_16941_;
                let _e2989 = phi_16944_;
                local_85 = _e2987;
                local_86 = _e2987;
                local_87 = _e2987;
                let _e2990 = (_e2989 < 2i);
                if _e2990 {
                    let _e2998 = text_lines.member[((_e2985 >> bitcast<u32>(((_e2989 * 16i) & 31i))) & 65535u)];
                    let _e3000 = unpack4x8unorm(_e2998.color);
                    let _e3002 = (1f / _e2998.size);
                    let _e3009 = ((_e496.x - _e2998.origin.x) * _e3002);
                    phi_16979_ = 0u;
                    phi_16982_ = _e2998.count;
                    loop {
                        let _e3014 = phi_16979_;
                        let _e3016 = phi_16982_;
                        local_81 = _e3014;
                        let _e3017 = (_e3014 < _e3016);
                        if _e3017 {
                            let _e3020 = (_e3014 + ((_e3016 - _e3014) / 2u));
                            let _e3025 = placed_glyphs.member[(_e2998.first + _e3020)].x;
                            let _e3026 = (_e3025 <= _e3009);
                            if _e3026 {
                                phi_17010_ = (_e3020 + 1u);
                            } else {
                                phi_17010_ = _e3014;
                            }
                            let _e3029 = phi_17010_;
                            phi_16980_ = _e3029;
                            phi_16983_ = select(_e3020, _e3016, _e3026);
                        } else {
                            phi_16980_ = u32();
                            phi_16983_ = u32();
                        }
                        let _e3032 = phi_16980_;
                        let _e3034 = phi_16983_;
                        continue;
                        continuing {
                            phi_16979_ = _e3032;
                            phi_16982_ = _e3034;
                            break if !(_e3017);
                        }
                    }
                    let _e3036 = (3.5f / _e2998.size);
                    let _e3038 = local_81;
                    let _e3039 = (_e3038 + 1u);
                    phi_17022_ = select(_e3039, _e2998.count, (_e2998.count < _e3039));
                    phi_17025_ = -1000000f;
                    loop {
                        let _e3043 = phi_17022_;
                        let _e3045 = phi_17025_;
                        local_84 = _e3045;
                        if (_e3043 > 0u) {
                            let _e3047 = (_e3043 - 1u);
                            let _e3048 = (_e2998.first + _e3047);
                            let _e3052 = placed_glyphs.member[_e3048].x;
                            let _e3056 = placed_glyphs.member[_e3048].glyph;
                            let _e3061 = glyphs.member[_e3056].min[0u];
                            let _e3066 = glyphs.member[_e3056].min[1u];
                            let _e3071 = glyphs.member[_e3056].max[0u];
                            let _e3076 = glyphs.member[_e3056].max[1u];
                            let _e3080 = glyphs.member[_e3056].start;
                            let _e3084 = glyphs.member[_e3056].count;
                            let _e3085 = (_e3009 - _e3052);
                            let _e3086 = -(((_e496.y - _e2998.origin.y) * _e3002));
                            let _e3087 = (_e3071 + _e3036);
                            let _e3088 = (_e3085 > _e3087);
                            if _e3088 {
                                phi_17155_ = f32();
                            } else {
                                if (_e3085 >= (_e3061 - _e3036)) {
                                    if (_e3086 >= (_e3066 - _e3036)) {
                                        if (_e3085 <= _e3087) {
                                            if (_e3086 <= (_e3076 + _e3036)) {
                                                phi_17105_ = 340282350000000000000000000000000000000f;
                                                phi_17108_ = 0u;
                                                phi_17110_ = 0i;
                                                loop {
                                                    let _e3098 = phi_17105_;
                                                    let _e3100 = phi_17108_;
                                                    let _e3102 = phi_17110_;
                                                    local_82 = _e3098;
                                                    local_83 = _e3102;
                                                    let _e3103 = (_e3100 < _e3084);
                                                    if _e3103 {
                                                        let _e3107 = edges.member[(_e3080 + _e3100)];
                                                        let _e3109 = cantus_render_text_edge_distance(_e3107, _e2998.weight, vec2<f32>(_e3085, _e3086), _e3098);
                                                        phi_17106_ = _e3109.member;
                                                        phi_17109_ = (_e3100 + 1u);
                                                        phi_17111_ = (_e3102 + _e3109.member_1);
                                                    } else {
                                                        phi_17106_ = f32();
                                                        phi_17109_ = u32();
                                                        phi_17111_ = i32();
                                                    }
                                                    let _e3115 = phi_17106_;
                                                    let _e3117 = phi_17109_;
                                                    let _e3119 = phi_17111_;
                                                    continue;
                                                    continuing {
                                                        phi_17105_ = _e3115;
                                                        phi_17108_ = _e3117;
                                                        phi_17110_ = _e3119;
                                                        break if !(_e3103);
                                                    }
                                                }
                                                let _e3122 = local_82;
                                                let _e3124 = ((_e3122 * _e2998.size) * _e2998.size);
                                                if (_e3124 >= 12.25f) {
                                                    phi_17143_ = 3.5f;
                                                } else {
                                                    phi_17143_ = sqrt(_e3124);
                                                }
                                                let _e3128 = phi_17143_;
                                                let _e3130 = local_83;
                                                let _e3133 = (_e3128 * select(1f, -1f, (_e3130 == 0i)));
                                                if (_e3045 != _e3045) {
                                                    phi_21403_ = true;
                                                } else {
                                                    phi_21403_ = (_e3133 >= _e3045);
                                                }
                                                let _e3137 = phi_21403_;
                                                phi_17151_ = select(_e3045, _e3133, _e3137);
                                            } else {
                                                phi_17151_ = _e3045;
                                            }
                                            let _e3140 = phi_17151_;
                                            phi_17152_ = _e3140;
                                        } else {
                                            phi_17152_ = _e3045;
                                        }
                                        let _e3142 = phi_17152_;
                                        phi_17153_ = _e3142;
                                    } else {
                                        phi_17153_ = _e3045;
                                    }
                                    let _e3144 = phi_17153_;
                                    phi_17154_ = _e3144;
                                } else {
                                    phi_17154_ = _e3045;
                                }
                                let _e3146 = phi_17154_;
                                phi_17155_ = _e3146;
                            }
                            let _e3148 = phi_17155_;
                            phi_17023_ = _e3047;
                            phi_17026_ = _e3148;
                            phi_17157_ = select(true, false, _e3088);
                        } else {
                            phi_17023_ = u32();
                            phi_17026_ = f32();
                            phi_17157_ = false;
                        }
                        let _e3151 = phi_17023_;
                        let _e3153 = phi_17026_;
                        let _e3155 = phi_17157_;
                        continue;
                        continuing {
                            phi_17022_ = _e3151;
                            phi_17025_ = _e3153;
                            break if !(_e3155);
                        }
                    }
                    let _e3158 = local_84;
                    let _e3160 = ((_e3158 * 1.25f) + 0.5f);
                    let _e3162 = select(_e3160, 0f, (_e3160 < 0f));
                    let _e3164 = select(_e3162, 1f, (_e3162 > 1f));
                    let _e3170 = (((_e3164 * _e3164) * (3f - (2f * _e3164))) * _e3000.w);
                    let _e3171 = (1f - _e3170);
                    phi_16942_ = vec3<f32>(((_e2987.x * _e3171) + (_e3000.x * _e3170)), ((_e2987.y * _e3171) + (_e3000.y * _e3170)), ((_e2987.z * _e3171) + (_e3000.z * _e3170)));
                    phi_16945_ = (_e2989 + 1i);
                } else {
                    phi_16942_ = vec3<f32>();
                    phi_16945_ = i32();
                }
                let _e3190 = phi_16942_;
                let _e3192 = phi_16945_;
                continue;
                continuing {
                    phi_16941_ = _e3190;
                    phi_16944_ = _e3192;
                    break if !(_e2990);
                }
            }
            if _e674 {
                break;
            }
            let _e3195 = local_85;
            let _e3199 = local_86;
            let _e3203 = local_87;
            out_color = vec4<f32>((_e3195.x * _e844), (_e3199.y * _e844), (_e3203.z * _e844), _e857);
            break;
        }
    }
    return;
}

@vertex
fn render_track_isthmus_trackpass_vertex(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    vertex_7 = vertex;
    instance_2 = instance;
    function_();
    let _e8 = out_position.y;
    out_position.y = -(_e8);
    let _e10 = out_position;
    let _e11 = out_pixel_pos;
    let _e12 = out_pill_idx;
    return VertexOutput(_e10, _e11, _e12);
}

@fragment
fn render_track_isthmus_trackpass_fragment(@location(0) pixel_pos: vec2<f32>, @location(1) @interpolate(flat) pill_idx: u32) -> @location(0) vec4<f32> {
    pixel_pos_1 = pixel_pos;
    pill_idx_1 = pill_idx;
    function_1();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_lyrics_isthmus_lyricspass_vertex(@builtin(vertex_index) vertex_1: u32, @builtin(instance_index) _isthmus_instance_index: u32) -> VertexOutput {
    vertex_7 = vertex_1;
    _isthmus_instance_index_9 = _isthmus_instance_index;
    function_2();
    let _e8 = out_position.y;
    out_position.y = -(_e8);
    let _e10 = out_position;
    let _e11 = out_pixel;
    let _e12 = out_isthmus_instance_index;
    return VertexOutput(_e10, _e11, _e12);
}

@fragment
fn render_lyrics_isthmus_lyricspass_fragment(@location(0) pixel: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_1: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel;
    _isthmus_instance_index_10 = _isthmus_instance_index_1;
    function_3();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_status_isthmus_statuspass_vertex(@builtin(vertex_index) vertex_2: u32, @builtin(instance_index) _isthmus_instance_index_2: u32) -> VertexOutput {
    vertex_7 = vertex_2;
    _isthmus_instance_index_9 = _isthmus_instance_index_2;
    function_4();
    let _e8 = out_position.y;
    out_position.y = -(_e8);
    let _e10 = out_position;
    let _e11 = out_pixel;
    let _e12 = out_isthmus_instance_index;
    return VertexOutput(_e10, _e11, _e12);
}

@fragment
fn render_status_isthmus_statuspass_fragment(@location(0) pixel_1: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_3: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel_1;
    _isthmus_instance_index_10 = _isthmus_instance_index_3;
    function_5();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_launcher_isthmus_launcherpass_vertex(@builtin(vertex_index) vertex_3: u32, @builtin(instance_index) instance_1: u32) -> VertexOutput {
    vertex_7 = vertex_3;
    instance_2 = instance_1;
    function_6();
    let _e8 = out_position.y;
    out_position.y = -(_e8);
    let _e10 = out_position;
    let _e11 = out_pixel;
    let _e12 = out_row_idx;
    return VertexOutput(_e10, _e11, _e12);
}

@fragment
fn render_launcher_isthmus_launcherpass_fragment(@location(0) pixel_2: vec2<f32>, @location(1) @interpolate(flat) row_idx: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel_2;
    row_idx_1 = row_idx;
    function_7();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_playhead_isthmus_playheadpass_vertex(@builtin(vertex_index) vertex_4: u32, @builtin(instance_index) _isthmus_instance_index_4: u32) -> VertexOutput {
    vertex_7 = vertex_4;
    _isthmus_instance_index_9 = _isthmus_instance_index_4;
    function_8();
    let _e8 = out_position.y;
    out_position.y = -(_e8);
    let _e10 = out_position;
    let _e11 = out_world_pos;
    let _e12 = out_isthmus_instance_index;
    return VertexOutput(_e10, _e11, _e12);
}

@fragment
fn render_playhead_isthmus_playheadpass_fragment(@location(0) world_pos: vec2<f32>, @location(1) @interpolate(flat) _isthmus_instance_index_5: u32) -> @location(0) vec4<f32> {
    world_pos_1 = world_pos;
    _isthmus_instance_index_10 = _isthmus_instance_index_5;
    function_9();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_particles_isthmus_particlepass_vertex(@builtin(vertex_index) vertex_5: u32, @builtin(instance_index) _isthmus_instance_index_6: u32) -> VertexOutput_1 {
    vertex_7 = vertex_5;
    _isthmus_instance_index_9 = _isthmus_instance_index_6;
    function_10();
    let _e8 = out_position.y;
    out_position.y = -(_e8);
    let _e10 = out_position;
    let _e11 = out_color;
    let _e12 = out_uv;
    return VertexOutput_1(_e10, _e11, _e12);
}

@fragment
fn render_particles_isthmus_particlepass_fragment(@location(0) color: vec4<f32>, @location(1) uv: vec2<f32>) -> @location(0) vec4<f32> {
    color_1 = color;
    uv_1 = uv;
    function_11();
    let _e5 = out_color;
    return _e5;
}

@vertex
fn render_tempestas_isthmus_tempestaspass_vertex(@builtin(vertex_index) vertex_6: u32, @builtin(instance_index) _isthmus_instance_index_7: u32) -> VertexOutput_2 {
    vertex_7 = vertex_6;
    _isthmus_instance_index_9 = _isthmus_instance_index_7;
    function_12();
    let _e9 = out_position.y;
    out_position.y = -(_e9);
    let _e11 = out_position;
    let _e12 = out_pixel;
    let _e13 = out_weather;
    let _e14 = out_isthmus_instance_index_1;
    return VertexOutput_2(_e11, _e12, _e13, _e14);
}

@fragment
fn render_tempestas_isthmus_tempestaspass_fragment(@location(0) pixel_3: vec2<f32>, @location(1) @interpolate(flat) weather: vec4<f32>, @location(2) @interpolate(flat) _isthmus_instance_index_8: u32) -> @location(0) vec4<f32> {
    pixel_4 = pixel_3;
    weather_1 = weather;
    _isthmus_instance_index_11 = _isthmus_instance_index_8;
    function_13();
    let _e7 = out_color;
    return _e7;
}
