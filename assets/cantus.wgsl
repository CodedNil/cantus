struct render_shared_RipplePulse {
    origin: vec2<f32>,
    start_time: f32,
    strength: f32,
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

struct type_13 {
    member: array<render_track_TrackPill>,
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
    conditions: render_tempestas_WeatherCondition,
    text: render_text_Text_2_u0020_32_,
}

struct type_28 {
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

struct type_40 {
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
var<storage> glyphs: type_17;
@group(0) @binding(3)
var<storage> edges: type_19;
@group(0) @binding(5)
var sampler_: sampler;
@group(0) @binding(4)
var images: texture_2d_array<f32>;
var<private> out_color: vec4<f32>;
@group(0) @binding(1)
var<storage> pill_1: type_28;
var<private> _isthmus_instance_index_7: u32;
var<private> out_pixel: vec2<f32>;
var<private> out_isthmus_instance_index: u32;
var<private> pixel_2: vec2<f32>;
var<private> _isthmus_instance_index_8: u32;
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
var<storage> pill_2: type_40;
var<private> out_weather: vec4<f32>;
var<private> out_isthmus_instance_index_1: u32;
var<private> weather_1: vec4<f32>;
var<private> _isthmus_instance_index_9: u32;

fn cantus_render_shader_pixel_to_ndc(param: vec2<f32>, param_1: vec2<f32>) -> vec4<f32> {
    return vec4<f32>((((param.x / param_1.x) * 2f) - 1f), (1f - ((param.y / param_1.y) * 2f)), 0f, 1f);
}

fn render_track_vertex_impl() {
    var phi_11763_: bool;
    var phi_11799_: bool;
    var phi_11821_: bool;
    var phi_11836_: bool;
    var phi_11860_: bool;

    let _e23 = vertex_5;
    let _e24 = instance_1;
    let _e28 = pill.member[_e24].width;
    let _e32 = frame.member[0u].panel_height;
    let _e36 = pill.member[_e24].x;
    let _e40 = frame.member[0u].panel_top;
    let _e42 = (_e36 + (_e28 * 0.5f));
    let _e49 = pill.member[_e24].secondary_expansion;
    let _e53 = pill.member[_e24].rating;
    let _e59 = pill.member[_e24].primary_playlist_count;
    let _e61 = (select(0f, 5f, (_e53 >= 0i)) + f32(_e59));
    let _e67 = pill.member[_e24].secondary_playlist_count;
    let _e68 = f32(_e67);
    let _e72 = pill.member[_e24].primary_alpha;
    let _e73 = (_e61 - 1f);
    if (_e73 != _e73) {
        phi_11763_ = true;
    } else {
        phi_11763_ = (0f >= _e73);
    }
    let _e77 = phi_11763_;
    let _e83 = select(0f, 1f, ((_e61 * _e72) > 0f));
    let _e84 = (((select(_e73, 0f, _e77) * 9f) + 32.4f) * _e83);
    let _e85 = (32.4f * _e83);
    let _e86 = (_e68 - 1f);
    if (_e86 != _e86) {
        phi_11799_ = true;
    } else {
        phi_11799_ = (0f >= _e86);
    }
    let _e90 = phi_11799_;
    let _e98 = select(0f, 1f, ((_e68 * _e49) > 0f));
    let _e99 = (((((select(_e86, 0f, _e90) * 18f) * _e49) * 0.5f) + 32.4f) * _e98);
    let _e100 = (32.4f * _e98);
    let _e102 = select(_e99, _e84, (_e84 > _e99));
    let _e105 = (_e36 - 48f);
    let _e106 = (_e42 - _e102);
    if (_e105 != _e105) {
        phi_11821_ = true;
    } else {
        phi_11821_ = (_e106 <= _e105);
    }
    let _e110 = phi_11821_;
    let _e111 = select(_e105, _e106, _e110);
    let _e112 = (_e40 - 48f);
    let _e114 = ((_e36 + _e28) + 48f);
    let _e115 = (_e42 + _e102);
    if (_e114 != _e114) {
        phi_11836_ = true;
    } else {
        phi_11836_ = (_e115 >= _e114);
    }
    let _e119 = phi_11836_;
    let _e122 = ((_e40 + _e32) + 48f);
    let _e124 = (((((_e40 + (_e32 * 0.975f)) - 3f) + (18f * _e49)) + -5.4f) + select(_e100, _e85, (_e85 > _e100)));
    if (_e122 != _e122) {
        phi_11860_ = true;
    } else {
        phi_11860_ = (_e124 >= _e122);
    }
    let _e128 = phi_11860_;
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

fn cantus_render_text_edge_distance(param_2: render_text_Edge, param_3: f32, param_4: vec2<f32>) -> u0028_f32_u0020_i32_u0029_ {
    var phi_14700_: bool;
    var phi_14715_: bool;
    var phi_14730_: bool;
    var phi_3257_: f32;
    var phi_3260_: i32;
    var phi_14771_: bool;
    var phi_14786_: bool;
    var phi_14801_: bool;
    var phi_3258_: f32;
    var phi_3261_: i32;
    var phi_14816_: bool;
    var local: f32;
    var local_1: f32;
    var local_2: f32;
    var local_3: f32;
    var local_4: f32;
    var local_5: f32;
    var phi_14857_: bool;
    var phi_3337_: i32;
    var phi_3340_: vec2<f32>;
    var phi_3342_: i32;
    var phi_3394_: i32;
    var phi_3395_: i32;
    var phi_3383_: i32;
    var phi_3384_: i32;
    var phi_3396_: i32;
    var phi_3338_: i32;
    var phi_3341_: vec2<f32>;
    var phi_3343_: i32;
    var local_6: i32;

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
        phi_14700_ = true;
    } else {
        phi_14700_ = (0.00000001f >= _e71);
    }
    let _e75 = phi_14700_;
    let _e77 = (((_e64 * _e62) + (_e65 * _e63)) / select(_e71, 0.00000001f, _e75));
    if (_e77 != _e77) {
        phi_14715_ = true;
    } else {
        phi_14715_ = (0f >= _e77);
    }
    let _e81 = phi_14715_;
    let _e82 = select(_e77, 0f, _e81);
    if (_e82 != _e82) {
        phi_14730_ = true;
    } else {
        phi_14730_ = (1f <= _e82);
    }
    let _e86 = phi_14730_;
    phi_3257_ = select(_e82, 1f, _e86);
    phi_3260_ = 0i;
    loop {
        let _e97 = phi_3257_;
        let _e99 = phi_3260_;
        local = _e97;
        local_1 = _e97;
        local_2 = _e97;
        local_3 = _e97;
        local_4 = _e97;
        local_5 = _e97;
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
                phi_14771_ = true;
            } else {
                phi_14771_ = (0.00000001f >= _e139);
            }
            let _e143 = phi_14771_;
            let _e155 = (_e97 - (((_e130 * _e128) + (_e131 * _e129)) / bitcast<f32>(((bitcast<u32>(select(_e139, 0.00000001f, _e143)) & 2147483647u) | (bitcast<u32>(_e138) & 2147483648u)))));
            if (_e155 != _e155) {
                phi_14786_ = true;
            } else {
                phi_14786_ = (0f >= _e155);
            }
            let _e159 = phi_14786_;
            let _e160 = select(_e155, 0f, _e159);
            if (_e160 != _e160) {
                phi_14801_ = true;
            } else {
                phi_14801_ = (1f <= _e160);
            }
            let _e164 = phi_14801_;
            phi_3258_ = select(_e160, 1f, _e164);
            phi_3261_ = (_e99 + 1i);
        } else {
            phi_3258_ = f32();
            phi_3261_ = i32();
        }
        let _e168 = phi_3258_;
        let _e170 = phi_3261_;
        continue;
        continuing {
            phi_3257_ = _e168;
            phi_3260_ = _e170;
            break if !(_e100);
        }
    }
    let _e174 = ((_e64 * _e64) + (_e65 * _e65));
    let _e175 = (param_4.x - _e60);
    let _e176 = (param_4.y - _e61);
    let _e179 = ((_e175 * _e175) + (_e176 * _e176));
    if (_e174 != _e174) {
        phi_14816_ = true;
    } else {
        phi_14816_ = (_e179 <= _e174);
    }
    let _e183 = phi_14816_;
    let _e184 = select(_e174, _e179, _e183);
    let _e187 = local;
    let _e188 = (1f - _e187);
    let _e195 = local_1;
    let _e196 = ((2f * _e188) * _e195);
    let _e202 = local_2;
    let _e205 = local_3;
    let _e208 = local_4;
    let _e211 = local_5;
    let _e215 = (param_4.x - ((((_e32 * _e188) * _e188) + (_e46 * _e196)) + ((_e60 * _e202) * _e208)));
    let _e216 = (param_4.y - ((((_e33 * _e188) * _e188) + (_e47 * _e196)) + ((_e61 * _e205) * _e211)));
    let _e219 = ((_e215 * _e215) + (_e216 * _e216));
    if (_e184 != _e184) {
        phi_14857_ = true;
    } else {
        phi_14857_ = (_e219 <= _e184);
    }
    let _e223 = phi_14857_;
    phi_3337_ = 1i;
    phi_3340_ = vec2<f32>(_e32, _e33);
    phi_3342_ = 0i;
    loop {
        let _e226 = phi_3337_;
        let _e228 = phi_3340_;
        let _e230 = phi_3342_;
        local_6 = _e230;
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
                        phi_3383_ = (_e230 + 1i);
                    } else {
                        phi_3383_ = _e230;
                    }
                    let _e272 = phi_3383_;
                    phi_3384_ = _e272;
                } else {
                    phi_3384_ = _e230;
                }
                let _e274 = phi_3384_;
                phi_3396_ = _e274;
            } else {
                if (_e250 <= param_4.y) {
                    if (_e259 < 0f) {
                        phi_3394_ = (_e230 - 1i);
                    } else {
                        phi_3394_ = _e230;
                    }
                    let _e265 = phi_3394_;
                    phi_3395_ = _e265;
                } else {
                    phi_3395_ = _e230;
                }
                let _e267 = phi_3395_;
                phi_3396_ = _e267;
            }
            let _e276 = phi_3396_;
            phi_3338_ = (_e226 + 1i);
            phi_3341_ = vec2<f32>(_e249, _e250);
            phi_3343_ = _e276;
        } else {
            phi_3338_ = i32();
            phi_3341_ = vec2<f32>();
            phi_3343_ = i32();
        }
        let _e280 = phi_3338_;
        let _e282 = phi_3341_;
        let _e284 = phi_3343_;
        continue;
        continuing {
            phi_3337_ = _e280;
            phi_3340_ = _e282;
            phi_3342_ = _e284;
            break if !(_e231);
        }
    }
    let _e287 = local_6;
    return u0028_f32_u0020_i32_u0029_(select(_e184, _e219, _e223), _e287);
}

fn cantus_render_track_plasma_field(param_5: vec2<f32>, param_6: render_track_PaletteColor, param_7: f32, param_8: f32, param_9: f32) -> vec4<f32> {
    let _e17 = ((sin((((param_5.x * param_7) + (param_5.y * param_8)) + param_9)) * 0.5f) + 0.5f);
    let _e23 = ((0.12f + (_e17 * _e17)) * (0.25f + (param_6.weight * 3f)));
    let _e25 = unpack4x8unorm(param_6.rgb);
    return vec4<f32>((_e25.x * _e23), (_e25.y * _e23), (_e25.z * _e23), _e23);
}

fn cantus_render_shader_sd_capsule_box(param_10: vec2<f32>, param_11: f32, param_12: f32) -> f32 {
    var phi_14657_: bool;
    var phi_14672_: bool;

    let _e8 = abs(param_10.y);
    let _e9 = (abs(param_10.x) - param_11);
    let _e11 = select(0f, _e9, (_e9 > 0f));
    let _e13 = select(0f, _e8, (_e8 > 0f));
    if (_e9 != _e9) {
        phi_14657_ = true;
    } else {
        phi_14657_ = (_e8 >= _e9);
    }
    let _e21 = phi_14657_;
    let _e22 = select(_e9, _e8, _e21);
    if (_e22 != _e22) {
        phi_14672_ = true;
    } else {
        phi_14672_ = (0f <= _e22);
    }
    let _e26 = phi_14672_;
    return ((sqrt(((_e11 * _e11) + (_e13 * _e13))) + select(_e22, 0f, _e26)) - param_12);
}

fn render_track_fragment_impl() {
    var local_7: array<u32, 2>;
    var phi_874_: vec2<f32>;
    var phi_877_: f32;
    var phi_879_: u32;
    var phi_11893_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_11904_: bool;
    var phi_875_: vec2<f32>;
    var phi_878_: f32;
    var phi_880_: u32;
    var phi_14992_: bool;
    var phi_1019_: f32;
    var local_8: vec2<f32>;
    var local_9: vec2<f32>;
    var phi_11962_: bool;
    var phi_11986_: bool;
    var phi_12043_: bool;
    var phi_12067_: bool;
    var phi_12092_: bool;
    var phi_12107_: bool;
    var phi_12122_: bool;
    var phi_12139_: bool;
    var phi_12154_: bool;
    var phi_12169_: bool;
    var phi_12184_: bool;
    var phi_12199_: bool;
    var phi_1915_: vec3<f32>;
    var phi_1916_: vec3<f32>;
    var local_10: f32;
    var local_11: f32;
    var local_12: f32;
    var local_13: f32;
    var phi_2022_: vec4<f32>;
    var phi_2025_: i32;
    var phi_12378_: bool;
    var phi_12413_: bool;
    var phi_12428_: bool;
    var phi_12443_: bool;
    var phi_12458_: bool;
    var phi_2320_: vec4<f32>;
    var phi_2023_: vec4<f32>;
    var phi_2026_: i32;
    var phi_2322_: vec4<f32>;
    var phi_2323_: vec4<f32>;
    var phi_2335_: vec4<f32>;
    var phi_2338_: u32;
    var phi_2374_: render_shared_RipplePulse;
    var phi_2375_: f32;
    var phi_12488_: bool;
    var phi_2494_: bool;
    var phi_2499_: bool;
    var phi_12525_: bool;
    var phi_12540_: bool;
    var phi_2601_: vec4<f32>;
    var phi_2602_: vec4<f32>;
    var phi_2603_: vec4<f32>;
    var phi_2604_: vec4<f32>;
    var phi_2336_: vec4<f32>;
    var phi_2339_: u32;
    var phi_15110_: bool;
    var phi_15145_: bool;
    var phi_2612_: u32;
    var phi_2615_: f32;
    var phi_2695_: u32;
    var phi_2698_: u32;
    var phi_2732_: u32;
    var phi_2696_: u32;
    var phi_2699_: u32;
    var phi_15142_: bool;
    var local_14: u32;
    var phi_15150_: bool;
    var phi_2740_: u32;
    var phi_2743_: f32;
    var phi_2830_: u32;
    var phi_2833_: i32;
    var phi_2835_: f32;
    var phi_12555_: bool;
    var phi_2831_: u32;
    var phi_2834_: i32;
    var phi_2836_: f32;
    var phi_15147_: bool;
    var local_15: f32;
    var local_16: i32;
    var phi_12570_: bool;
    var phi_15160_: bool;
    var phi_2867_: f32;
    var phi_15159_: bool;
    var phi_2868_: f32;
    var phi_15158_: bool;
    var phi_2869_: f32;
    var phi_15157_: bool;
    var phi_2870_: f32;
    var phi_15156_: bool;
    var phi_2871_: f32;
    var phi_15155_: bool;
    var phi_2741_: u32;
    var phi_2744_: f32;
    var phi_2873_: bool;
    var phi_15154_: bool;
    var phi_15172_: bool;
    var phi_2878_: f32;
    var phi_15171_: bool;
    var phi_2879_: f32;
    var phi_2880_: bool;
    var phi_15170_: bool;
    var phi_2881_: f32;
    var phi_2882_: bool;
    var phi_15169_: bool;
    var phi_2883_: f32;
    var phi_2884_: bool;
    var phi_12585_: bool;
    var phi_15165_: bool;
    var phi_2613_: u32;
    var phi_2616_: f32;
    var phi_15164_: bool;
    var local_17: f32;
    var local_18: vec4<f32>;
    var local_19: vec4<f32>;
    var local_20: vec4<f32>;
    var local_21: vec4<f32>;
    var local_22: vec4<f32>;
    var local_23: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e162 = pixel_pos_1;
            let _e163 = pill_idx_1;
            let _e173 = pill.member[_e163].x;
            let _e177 = pill.member[_e163].width;
            let _e181 = frame.member[0u].panel_height;
            let _e185 = frame.member[0u].panel_top;
            let _e186 = (_e162.x - _e173);
            let _e187 = (_e162.y - _e185);
            let _e188 = (_e177 * 0.5f);
            let _e189 = (_e181 * 0.5f);
            let _e191 = (_e187 - _e189);
            let _e192 = (_e177 - _e181);
            let _e193 = (_e192 * 0.5f);
            let _e195 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e186 - _e188), _e191), _e193, _e189);
            let _e200 = frame.member[0u].mouse_pos[0u];
            let _e205 = frame.member[0u].mouse_pos[1u];
            let _e211 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e200 - _e173) - _e188), ((_e205 - _e185) - _e189)), _e193, _e189);
            phi_874_ = vec2<f32>(0f, 0f);
            phi_877_ = 0f;
            phi_879_ = 0u;
            loop {
                let _e213 = phi_874_;
                let _e215 = phi_877_;
                let _e217 = phi_879_;
                local_8 = _e213;
                local_9 = _e213;
                local_10 = _e215;
                local_11 = _e215;
                local_12 = _e215;
                local_13 = _e215;
                let _e218 = (_e217 < 4u);
                if _e218 {
                    if _e218 {
                    } else {
                        phi_14992_ = true;
                        break;
                    }
                    let _e225 = frame.member[0u].ripples[_e217].origin[0u];
                    let _e232 = frame.member[0u].ripples[_e217].origin[1u];
                    let _e238 = frame.member[0u].ripples[_e217].start_time;
                    let _e244 = frame.member[0u].ripples[_e217].strength;
                    let _e248 = frame.member[0u].time;
                    let _e250 = ((_e248 - _e238) * 1.2f);
                    let _e252 = select(_e250, 0f, (_e250 < 0f));
                    let _e254 = select(_e252, 1f, (_e252 > 1f));
                    let _e255 = (_e162.x - _e225);
                    let _e256 = (_e162.y - _e232);
                    let _e260 = sqrt(((_e255 * _e255) + (_e256 * _e256)));
                    if (_e260 > 0.001f) {
                        phi_11893_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e255 / _e260), (_e256 / _e260)), _e260);
                    } else {
                        phi_11893_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e260);
                    }
                    let _e268 = phi_11893_;
                    let _e278 = ((abs((_e268.unnamed_1 - (_e254 * 600f))) - 80f) * -0.0125f);
                    let _e280 = select(_e278, 0f, (_e278 < 0f));
                    let _e282 = select(_e280, 1f, (_e280 > 1f));
                    let _e288 = (1f - _e254);
                    let _e289 = ((((_e282 * _e282) * (3f - (2f * _e282))) * _e244) * _e288);
                    let _e302 = (_e215 + (_e289 * 0.5f));
                    if (_e302 != _e302) {
                        phi_11904_ = true;
                    } else {
                        phi_11904_ = (1f <= _e302);
                    }
                    let _e306 = phi_11904_;
                    phi_875_ = vec2<f32>((_e213.x + (((_e268.unnamed.x * _e289) * _e288) * 0.5f)), (_e213.y + (((_e268.unnamed.y * _e289) * _e288) * 0.5f)));
                    phi_878_ = select(_e302, 1f, _e306);
                    phi_880_ = (_e217 + 1u);
                } else {
                    phi_875_ = vec2<f32>();
                    phi_878_ = f32();
                    phi_880_ = u32();
                }
                let _e310 = phi_875_;
                let _e312 = phi_878_;
                let _e314 = phi_880_;
                continue;
                continuing {
                    phi_874_ = _e310;
                    phi_877_ = _e312;
                    phi_879_ = _e314;
                    phi_14992_ = false;
                    break if !(_e218);
                }
            }
            let _e317 = phi_14992_;
            if _e317 {
                break;
            }
            let _e321 = frame.member[0u].mouse_pressure;
            let _e322 = (_e321 > 0f);
            if _e322 {
                let _e323 = (_e162.x - _e200);
                let _e324 = (_e162.y - _e205);
                let _e330 = ((sqrt(((_e323 * _e323) + (_e324 * _e324))) - 150f) * -0.006666667f);
                let _e332 = select(_e330, 0f, (_e330 < 0f));
                let _e334 = select(_e332, 1f, (_e332 > 1f));
                phi_1019_ = ((((_e334 * _e334) * (3f - (2f * _e334))) * _e321) * 8f);
            } else {
                phi_1019_ = 0f;
            }
            let _e342 = phi_1019_;
            let _e344 = local_8;
            let _e347 = local_9;
            let _e349 = (_e186 / _e177);
            let _e350 = (_e187 / _e181);
            let _e351 = (_e349 - 0.5f);
            let _e352 = (_e350 - 0.5f);
            let _e353 = (_e173 + _e188);
            let _e355 = (_e185 + (_e181 * 0.975f));
            let _e356 = (_e355 - 3f);
            let _e360 = pill.member[_e163].secondary_expansion;
            let _e364 = pill.member[_e163].rating;
            let _e365 = (_e364 >= 0i);
            let _e366 = select(0f, 5f, _e365);
            let _e370 = pill.member[_e163].primary_playlist_count;
            let _e372 = (_e366 + f32(_e370));
            let _e376 = (_e356 + (18f * _e360));
            let _e380 = pill.member[_e163].secondary_playlist_count;
            let _e381 = f32(_e380);
            let _e387 = pill.member[_e163].primary_alpha;
            let _e388 = vec2<f32>(_e200, _e205);
            let _e390 = (_e372 - 1f);
            let _e391 = (_e390 != _e390);
            if _e391 {
                phi_11962_ = true;
            } else {
                phi_11962_ = (0f >= _e390);
            }
            let _e394 = phi_11962_;
            let _e397 = vec2<f32>(_e353, (_e355 + -10.4f));
            let _e399 = cantus_render_shader_sd_capsule_box((_e162 - _e397), (select(_e390, 0f, _e394) * 9f), 9f);
            if _e391 {
                phi_11986_ = true;
            } else {
                phi_11986_ = (0f >= _e390);
            }
            let _e402 = phi_11986_;
            let _e406 = cantus_render_shader_sd_capsule_box((_e388 - _e397), (select(_e390, 0f, _e402) * 9f), 9f);
            let _e407 = (10.5f * _e360);
            let _e409 = (_e381 - 1f);
            let _e410 = (_e409 != _e409);
            if _e410 {
                phi_12043_ = true;
            } else {
                phi_12043_ = (0f >= _e409);
            }
            let _e413 = phi_12043_;
            let _e418 = vec2<f32>(_e353, (_e376 + -5.4f));
            let _e420 = cantus_render_shader_sd_capsule_box((_e162 - _e418), (((select(_e409, 0f, _e413) * 18f) * _e360) * 0.5f), _e407);
            if _e410 {
                phi_12067_ = true;
            } else {
                phi_12067_ = (0f >= _e409);
            }
            let _e423 = phi_12067_;
            let _e429 = cantus_render_shader_sd_capsule_box((_e388 - _e418), (((select(_e409, 0f, _e423) * 18f) * _e360) * 0.5f), _e407);
            let _e432 = (0.5f + ((_e399 - _e195) * 0.05f));
            let _e434 = select(_e432, 0f, (_e432 < 0f));
            let _e436 = select(_e434, 1f, (_e434 > 1f));
            let _e446 = (_e195 + ((((_e399 + ((_e195 - _e399) * _e436)) - ((10f * _e436) * (1f - _e436))) - _e195) * _e387));
            let _e449 = (0.5f + ((_e406 - _e211) * 0.05f));
            let _e451 = select(_e449, 0f, (_e449 < 0f));
            let _e453 = select(_e451, 1f, (_e451 > 1f));
            let _e463 = (_e211 + ((((_e406 + ((_e211 - _e406) * _e453)) - ((10f * _e453) * (1f - _e453))) - _e211) * _e387));
            let _e465 = select(0f, 1f, (_e360 > 0f));
            let _e468 = (0.5f + ((_e420 - _e446) * 0.046296295f));
            let _e470 = select(_e468, 0f, (_e468 < 0f));
            let _e472 = select(_e470, 1f, (_e470 > 1f));
            let _e485 = (0.5f + ((_e429 - _e463) * 0.046296295f));
            let _e487 = select(_e485, 0f, (_e485 < 0f));
            let _e489 = select(_e487, 1f, (_e487 > 1f));
            let _e501 = (((_e463 + ((((_e429 + ((_e463 - _e429) * _e489)) - ((10.8f * _e489) * (1f - _e489))) - _e463) * _e465)) - 0.5f) * -1f);
            let _e503 = select(_e501, 0f, (_e501 < 0f));
            let _e505 = select(_e503, 1f, (_e503 > 1f));
            let _e515 = (sqrt(((_e344.x * _e344.x) + (_e347.y * _e347.y))) * 22f);
            let _e517 = (((_e342 * ((_e505 * _e505) * (3f - (2f * _e505)))) + _e515) * 0.5f);
            let _e518 = ((_e446 + ((((_e420 + ((_e446 - _e420) * _e472)) - ((10.8f * _e472) * (1f - _e472))) - _e446) * _e465)) - _e517);
            let _e519 = fwidth(_e518);
            if (_e519 != _e519) {
                phi_12092_ = true;
            } else {
                phi_12092_ = (0.55f >= _e519);
            }
            let _e523 = phi_12092_;
            let _e524 = select(_e519, 0.55f, _e523);
            let _e528 = ((_e518 - _e524) / (-(_e524) - _e524));
            let _e530 = select(_e528, 0f, (_e528 < 0f));
            let _e532 = select(_e530, 1f, (_e530 > 1f));
            let _e536 = ((_e532 * _e532) * (3f - (2f * _e532)));
            let _e537 = (_e518 != _e518);
            if _e537 {
                phi_12107_ = true;
            } else {
                phi_12107_ = (0f >= _e518);
            }
            let _e540 = phi_12107_;
            let _e544 = (exp((select(_e518, 0f, _e540) * -0.3f)) * 0.16f);
            if (_e536 != _e536) {
                phi_12122_ = true;
            } else {
                phi_12122_ = (_e544 >= _e536);
            }
            let _e548 = phi_12122_;
            let _e549 = select(_e536, _e544, _e548);
            let _e553 = pill.member[_e163].visibility;
            if ((_e549 * _e553) <= 0.0009765625f) {
                discard;
            }
            if _e537 {
                phi_12139_ = true;
            } else {
                phi_12139_ = (0f <= _e518);
            }
            let _e558 = phi_12139_;
            let _e561 = (1f + (select(_e518, 0f, _e558) * 0.008333334f));
            let _e563 = select(_e561, 0f, (_e561 < 0f));
            let _e565 = select(_e563, 0.6f, (_e563 > 0.6f));
            let _e575 = ((_e350 - ((_e352 * _e565) * 0.08f)) - (_e347.y * 0.04f));
            let _e576 = (((_e349 - ((_e351 * _e565) * 0.08f)) - (_e344.x * 0.04f)) * _e177);
            let _e577 = (_e575 * _e181);
            let _e581 = pill.member[_e163].effects;
            if _e537 {
                phi_12154_ = true;
            } else {
                phi_12154_ = (0f <= _e518);
            }
            let _e587 = phi_12154_;
            let _e590 = (1f + (select(_e518, 0f, _e587) * 0.008333334f));
            let _e592 = select(_e590, 0f, (_e590 < 0f));
            let _e594 = select(_e592, 1f, (_e592 > 1f));
            let _e607 = (_e581.seed - trunc(_e581.seed));
            let _e612 = ((_e177 / _e181) * ((0.5f + (_e607 * 0.12f)) + (_e581.turbulence * 0.18f)));
            if (_e612 != _e612) {
                phi_12169_ = true;
            } else {
                phi_12169_ = (1.7f >= _e612);
            }
            let _e616 = phi_12169_;
            let _e619 = select(0f, _e349, (_e349 > 0f));
            let _e621 = select(0f, _e350, (_e350 > 0f));
            let _e629 = (select(1f, _e621, (_e621 < 1f)) - (((((_e352 * _e594) * _e594) * 0.6f) + _e347.y) * 0.08f));
            let _e630 = ((select(1f, _e619, (_e619 < 1f)) - (((((_e351 * _e594) * _e594) * 0.6f) + _e344.x) * 0.08f)) * select(_e612, 1.7f, _e616));
            let _e641 = (_e581.flow_time * 0.8f);
            let _e651 = ((0.14f + (_e581.turbulence * 0.2f)) + _e581.beat);
            let _e656 = (_e581.seed + 1.5707964f);
            let _e661 = pill.member[_e163].colors[0u];
            let _e662 = vec2<f32>((_e630 + ((sin(((_e629 * 4.32f) + _e581.flow_time)) + cos(((_e630 * 1.3f) - (_e581.flow_time * 0.7f)))) * _e651)), ((_e629 * 1.6f) + ((cos(((_e630 * 2.3f) - _e641)) + sin(((_e629 * 2.72f) + (_e581.flow_time * 0.6f)))) * _e651)));
            let _e663 = cantus_render_track_plasma_field(_e662, _e661, 2.1f, 0.7f, _e581.flow_time);
            let _e668 = pill.member[_e163].colors[1u];
            let _e670 = cantus_render_track_plasma_field(_e662, _e668, 0.6f, -2.4f, (_e656 - _e641));
            let _e687 = pill.member[_e163].colors[2u];
            let _e690 = cantus_render_track_plasma_field(_e662, _e687, -1.5f, 1.9f, ((_e581.flow_time * 0.65f) + 2f));
            let _e703 = pill.member[_e163].colors[3u];
            let _e706 = cantus_render_track_plasma_field(_e662, _e703, 2.4f, 1.6f, (_e656 - (_e581.flow_time * 0.55f)));
            let _e714 = (((_e663.w + _e670.w) + _e690.w) + _e706.w);
            let _e715 = ((((_e663.x + _e670.x) + _e690.x) + _e706.x) / _e714);
            let _e716 = ((((_e663.y + _e670.y) + _e690.y) + _e706.y) / _e714);
            let _e717 = ((((_e663.z + _e670.z) + _e690.z) + _e706.z) / _e714);
            let _e722 = (((_e715 * 0.2126f) + (_e716 * 0.7152f)) + (_e717 * 0.0722f));
            let _e726 = frame.member[0u].playhead_x;
            let _e727 = (_e726 + 3f);
            let _e731 = ((_e162.x - _e727) / ((_e726 - 3f) - _e727));
            let _e733 = select(_e731, 0f, (_e731 < 0f));
            let _e735 = select(_e733, 1f, (_e733 > 1f));
            let _e740 = (_e581.valence * 0.4f);
            let _e741 = (1.55f + _e740);
            let _e743 = (_e722 * (-0.54999995f - _e740));
            let _e747 = (_e743 + (_e715 * _e741));
            let _e748 = (_e743 + (_e716 * _e741));
            let _e749 = (_e743 + (_e717 * _e741));
            let _e751 = select(0.035f, _e747, (_e747 > 0.035f));
            let _e753 = select(0.035f, _e748, (_e748 > 0.035f));
            let _e755 = select(0.035f, _e749, (_e749 > 0.035f));
            if (_e722 != _e722) {
                phi_12184_ = true;
            } else {
                phi_12184_ = (0.001f >= _e722);
            }
            let _e765 = phi_12184_;
            let _e767 = (0.52f / select(_e722, 0.001f, _e765));
            if (_e767 != _e767) {
                phi_12199_ = true;
            } else {
                phi_12199_ = (1f <= _e767);
            }
            let _e771 = phi_12199_;
            let _e772 = select(_e767, 1f, _e771);
            let _e779 = ((0.96f + (_e581.valence * 0.06f)) + (_e581.beat * 0.5f));
            let _e784 = ((_e575 - 0.45f) * 1.8181818f);
            let _e786 = select(_e784, 0f, (_e784 < 0f));
            let _e788 = select(_e786, 1f, (_e786 > 1f));
            let _e794 = (0.84f + (((_e788 * _e788) * (3f - (2f * _e788))) * 0.1f));
            let _e799 = (1f - (0.4f * ((_e735 * _e735) * (3f - (2f * _e735)))));
            let _e808 = pill.member[_e163].colors[3u].rgb;
            let _e809 = unpack4x8unorm(_e808);
            let _e822 = frame.member[0u].time;
            let _e827 = ((_e581.acousticness * 0.7f) + (_e581.instrumentalness * 0.3f));
            let _e834 = (8f - _e827);
            let _e839 = (_e822 * (0.35f + (_e581.instrumentalness * 0.55f)));
            let _e842 = ((_e186 / _e834) + (_e839 * (0.16f + (_e607 * 0.08f))));
            let _e843 = ((_e187 / _e834) + (_e839 * (0.055f + (sin((_e581.seed * 0.7f)) * 0.025f))));
            let _e844 = floor(_e842);
            let _e845 = floor(_e843);
            let _e854 = bitcast<u32>(select(0i, select(select(i32(_e845), i32(-2147483648), (_e845 < -2147483600f)), 2147483647i, (_e845 > 2147483500f)), (_e845 == _e845)));
            let _e862 = bitcast<u32>(select(0i, select(select(i32(_e844), i32(-2147483648), (_e844 < -2147483600f)), 2147483647i, (_e844 > 2147483500f)), (_e844 == _e844)));
            let _e864 = (bitcast<u32>((_e581.seed + 2.71f)) * 2654435761u);
            let _e870 = (((_e862 ^ _e864) * 1664525u) + 1013904223u);
            let _e872 = ((((_e854 ^ _e864) * 1664525u) + 1013904223u) + (_e870 * 1664525u));
            let _e874 = (_e870 + (_e872 * 1664525u));
            let _e882 = ((_e872 ^ (_e872 >> bitcast<u32>(16i))) + ((_e874 ^ (_e874 >> bitcast<u32>(16i))) * 1664525u));
            let _e886 = f32((_e882 ^ (_e882 >> bitcast<u32>(16i))));
            let _e887 = (_e886 * 0.0000000016600825f);
            let _e901 = (_e827 * 0.09f);
            let _e904 = (bitcast<u32>(_e581.seed) * 2654435761u);
            let _e910 = (((_e854 ^ _e904) * 1664525u) + 1013904223u);
            let _e912 = ((((_e862 ^ _e904) * 1664525u) + 1013904223u) + (_e910 * 1664525u));
            let _e914 = (_e910 + (_e912 * 1664525u));
            let _e922 = ((_e912 ^ (_e912 >> bitcast<u32>(16i))) + ((_e914 ^ (_e914 >> bitcast<u32>(16i))) * 1664525u));
            let _e930 = (((f32((_e922 ^ (_e922 >> bitcast<u32>(16i)))) * 0.00000000023283064f) - (0.985f - _e901)) / (_e901 + 0.014999986f));
            let _e932 = select(_e930, 0f, (_e930 < 0f));
            let _e934 = select(_e932, 1f, (_e932 > 1f));
            let _e943 = (((_e842 - _e844) - 0.5f) - ((_e886 * 0.00000000013038516f) - 0.28f));
            let _e944 = (((_e843 - _e845) - 0.5f) - (((_e887 - trunc(_e887)) * 0.56f) - 0.28f));
            let _e950 = ((sqrt(((_e943 * _e943) + (_e944 * _e944))) - 0.06f) * 4.5454545f);
            let _e952 = select(_e950, 0f, (_e950 < 0f));
            let _e954 = select(_e952, 1f, (_e952 > 1f));
            let _e967 = (((((_e934 * _e934) * (3f - (2f * _e934))) * (1f - ((_e954 * _e954) * (3f - (2f * _e954))))) * ((sin(((_e822 * ((0.7f + (_e886 * 0.00000000020954757f)) + (_e581.instrumentalness * 0.8f))) + (_e886 * 0.0000000014629181f))) * 0.5f) + 0.5f)) * (0.12f + (_e827 * 0.48f)));
            let _e971 = (((((select(0.92f, _e751, (_e751 < 0.92f)) * _e772) * _e779) * _e794) * _e799) + (((_e809.x * 0.75f) + 0.25f) * _e967));
            let _e972 = (((((select(0.92f, _e753, (_e753 < 0.92f)) * _e772) * _e779) * _e794) * _e799) + (((_e809.y * 0.75f) + 0.25f) * _e967));
            let _e973 = (((((select(0.92f, _e755, (_e755 < 0.92f)) * _e772) * _e779) * _e794) * _e799) + (((_e809.z * 0.75f) + 0.25f) * _e967));
            let _e974 = vec3<f32>(_e971, _e972, _e973);
            let _e975 = (_e192 + _e189);
            let _e979 = pill.member[_e163].image_index;
            if (_e979 >= 0i) {
                let _e981 = (_e186 - _e975);
                let _e982 = abs(_e981);
                let _e983 = abs(_e191);
                if (select(_e983, _e982, (_e982 > _e983)) < _e181) {
                    let _e987 = (_e189 + _e517);
                    let _e993 = (_e987 * 2f);
                    let _e999 = vec3<f32>(((_e981 / _e993) + 0.5f), ((_e191 / _e993) + 0.5f), f32(_e979));
                    let _e1005 = textureSample(images, sampler_, vec2<f32>(_e999.x, _e999.y), i32(_e999.z));
                    let _e1007 = (((sqrt(((_e981 * _e981) + (_e191 * _e191))) - _e987) - -4f) * 0.25f);
                    let _e1009 = select(_e1007, 0f, (_e1007 < 0f));
                    let _e1011 = select(_e1009, 1f, (_e1009 > 1f));
                    let _e1018 = ((_e211 - 0.5f) * -1f);
                    let _e1020 = select(_e1018, 0f, (_e1018 < 0f));
                    let _e1022 = select(_e1020, 1f, (_e1020 > 1f));
                    let _e1031 = ((_e195 - (((_e342 * ((_e1022 * _e1022) * (3f - (2f * _e1022)))) + _e515) * 0.5f)) - -0.5f);
                    let _e1033 = select(_e1031, 0f, (_e1031 < 0f));
                    let _e1035 = select(_e1033, 1f, (_e1033 > 1f));
                    let _e1046 = (((1f - ((_e1011 * _e1011) * (3f - (2f * _e1011)))) * (1f - ((_e1035 * _e1035) * (3f - (2f * _e1035))))) * _e1005.w);
                    let _e1047 = (1f - _e1046);
                    phi_1915_ = vec3<f32>(((_e971 * _e1047) + (_e1005.x * _e1046)), ((_e972 * _e1047) + (_e1005.y * _e1046)), ((_e973 * _e1047) + (_e1005.z * _e1046)));
                } else {
                    phi_1915_ = _e974;
                }
                let _e1059 = phi_1915_;
                phi_1916_ = _e1059;
            } else {
                phi_1916_ = _e974;
            }
            let _e1061 = phi_1916_;
            let _e1072 = ((_e575 - 0.12f) * -8.333334f);
            let _e1074 = select(_e1072, 0f, (_e1072 < 0f));
            let _e1076 = select(_e1074, 1f, (_e1074 > 1f));
            let _e1083 = ((_e518 - 5f) * -0.125f);
            let _e1085 = select(_e1083, 0f, (_e1083 < 0f));
            let _e1087 = select(_e1085, 1f, (_e1085 > 1f));
            let _e1093 = ((((_e1076 * _e1076) * (3f - (2f * _e1076))) * 0.12f) + (((_e1087 * _e1087) * (3f - (2f * _e1087))) * 0.08f));
            let _e1097 = (_e1061.x + (((_e1061.x * 0.68f) + 0.32f) * _e1093));
            let _e1098 = (_e1061.y + (((_e1061.y * 0.68f) + 0.32f) * _e1093));
            let _e1099 = (_e1061.z + (((_e1061.z * 0.68f) + 0.32f) * _e1093));
            let _e1107 = local_10;
            let _e1108 = (1f - _e1107);
            let _e1113 = local_11;
            let _e1116 = local_12;
            let _e1119 = local_13;
            let _e1127 = vec4<f32>((((_e1097 * _e1108) + (((_e1097 * 1.5f) + 0.1f) * _e1113)) * _e536), (((_e1098 * _e1108) + (((_e1098 * 1.5f) + 0.1f) * _e1116)) * _e536), (((_e1099 * _e1108) + (((_e1099 * 1.5f) + 0.1f) * _e1119)) * _e536), _e549);
            if _e365 {
                if (_e387 > 0f) {
                    phi_2022_ = _e1127;
                    phi_2025_ = 0i;
                    loop {
                        let _e1130 = phi_2022_;
                        let _e1132 = phi_2025_;
                        local_22 = _e1130;
                        let _e1133 = (_e1132 < 5i);
                        if _e1133 {
                            let _e1134 = f32(_e1132);
                            if _e391 {
                                phi_12378_ = true;
                            } else {
                                phi_12378_ = (0f >= _e390);
                            }
                            let _e1137 = phi_12378_;
                            let _e1142 = (_e353 + ((_e1134 - (select(_e390, 0f, _e1137) * 0.5f)) * 18f));
                            let _e1143 = (_e355 + -1f);
                            let _e1144 = (_e162.x - _e1142);
                            let _e1145 = (_e162.y - _e1143);
                            let _e1146 = abs(_e1144);
                            let _e1147 = abs(_e1145);
                            if (select(_e1147, _e1146, (_e1146 > _e1147)) < 38.88f) {
                                let _e1154 = ((f32(_e364) - (_e1134 * 2f)) * 0.5f);
                                let _e1156 = select(_e1154, 0f, (_e1154 < 0f));
                                let _e1159 = (_e1142 - _e200);
                                let _e1160 = (_e1143 - _e205);
                                let _e1166 = ((sqrt(((_e1159 * _e1159) + (_e1160 * _e1160))) - 11.3f) * -1f);
                                let _e1168 = select(_e1166, 0f, (_e1166 < 0f));
                                let _e1170 = select(_e1168, 1f, (_e1168 > 1f));
                                let _e1176 = select(_e321, 0f, (_e321 < 0f));
                                let _e1179 = (((_e1170 * _e1170) * (3f - (2f * _e1170))) * select(_e1176, 1f, (_e1176 > 1f)));
                                let _e1181 = (1.05f + (0.63f * _e1179));
                                let _e1182 = (_e1159 * _e1179);
                                let _e1184 = (_e1144 - (_e1182 * 0.5f));
                                let _e1185 = (_e1182 * -0.005f);
                                let _e1186 = sin(_e1185);
                                let _e1187 = cos(_e1185);
                                let _e1190 = ((_e1187 * _e1184) - (_e1186 * _e1145));
                                let _e1193 = ((_e1186 * _e1184) + (_e1187 * _e1145));
                                let _e1197 = (_e1181 * 5.4f);
                                let _e1198 = abs(_e1190);
                                let _e1202 = ((0.809017f * _e1198) + (_e1193 * 0.58778524f));
                                if (_e1202 != _e1202) {
                                    phi_12413_ = true;
                                } else {
                                    phi_12413_ = (0f >= _e1202);
                                }
                                let _e1206 = phi_12413_;
                                let _e1207 = select(_e1202, 0f, _e1206);
                                let _e1210 = (_e1198 - (_e1207 * 1.618034f));
                                let _e1211 = (-(_e1193) - (_e1207 * -1.1755705f));
                                let _e1214 = ((-0.809017f * _e1210) + (-0.58778524f * _e1211));
                                if (_e1214 != _e1214) {
                                    phi_12428_ = true;
                                } else {
                                    phi_12428_ = (0f >= _e1214);
                                }
                                let _e1218 = phi_12428_;
                                let _e1219 = select(_e1214, 0f, _e1218);
                                let _e1224 = abs((_e1210 - (_e1219 * -1.618034f)));
                                let _e1225 = ((_e1211 - (_e1219 * -1.1755705f)) - _e1197);
                                let _e1226 = (_e1181 * 2.031386f);
                                let _e1228 = ((_e1181 * 2.7959628f) - _e1197);
                                let _e1235 = (((_e1224 * _e1226) + (_e1225 * _e1228)) / ((_e1226 * _e1226) + (_e1228 * _e1228)));
                                let _e1237 = select(_e1235, 0f, (_e1235 < 0f));
                                let _e1239 = select(_e1237, 1f, (_e1237 > 1f));
                                let _e1245 = (_e1224 - (_e1226 * _e1239));
                                let _e1246 = (_e1225 - (_e1228 * _e1239));
                                let _e1255 = ((sqrt(((_e1245 * _e1245) + (_e1246 * _e1246))) * select(1f, -1f, (((_e1225 * _e1226) - (_e1224 * _e1228)) < 0f))) - (_e1181 * 1.08f));
                                let _e1256 = (((_e1190 / (_e1181 * 21.6f)) + 0.5f) - select(_e1156, 1f, (_e1156 > 1f)));
                                let _e1257 = fwidth(_e1256);
                                let _e1259 = ((_e1256 / _e1257) + 0.5f);
                                let _e1261 = select(_e1259, 0f, (_e1259 < 0f));
                                let _e1263 = select(_e1261, 1f, (_e1261 > 1f));
                                let _e1264 = (1f - _e1263);
                                let _e1267 = (0.33f * _e1263);
                                let _e1271 = (0.5f - _e1255);
                                let _e1273 = select(_e1271, 0f, (_e1271 < 0f));
                                let _e1275 = select(_e1273, 1f, (_e1273 > 1f));
                                if (_e1255 != _e1255) {
                                    phi_12443_ = true;
                                } else {
                                    phi_12443_ = (0f >= _e1255);
                                }
                                let _e1279 = phi_12443_;
                                let _e1282 = exp((select(_e1255, 0f, _e1279) * -0.5f));
                                let _e1283 = (_e1255 * -0.2f);
                                let _e1285 = select(_e1283, 0f, (_e1283 < 0f));
                                let _e1287 = select(_e1285, 1f, (_e1285 > 1f));
                                let _e1292 = (1f - ((_e1287 * _e1287) * (3f - (2f * _e1287))));
                                let _e1294 = ((_e1292 * _e1292) * 0.045f);
                                let _e1305 = ((_e1282 * _e1282) * 0.2f);
                                if (_e1275 != _e1275) {
                                    phi_12458_ = true;
                                } else {
                                    phi_12458_ = (_e1305 >= _e1275);
                                }
                                let _e1309 = phi_12458_;
                                let _e1311 = (select(_e1275, _e1305, _e1309) * _e387);
                                let _e1312 = (1f - _e1311);
                                phi_2320_ = vec4<f32>(((_e1130.x * _e1312) + ((((_e1264 + _e1267) + _e1294) * _e1275) * _e387)), ((_e1130.y * _e1312) + (((((0.85f * _e1264) + _e1267) + _e1294) * _e1275) * _e387)), ((_e1130.z * _e1312) + (((((0.2f * _e1264) + _e1267) + _e1294) * _e1275) * _e387)), ((_e1130.w * _e1312) + _e1311));
                            } else {
                                phi_2320_ = _e1130;
                            }
                            let _e1327 = phi_2320_;
                            phi_2023_ = _e1327;
                            phi_2026_ = (_e1132 + 1i);
                        } else {
                            phi_2023_ = vec4<f32>();
                            phi_2026_ = i32();
                        }
                        let _e1330 = phi_2023_;
                        let _e1332 = phi_2026_;
                        continue;
                        continuing {
                            phi_2022_ = _e1330;
                            phi_2025_ = _e1332;
                            break if !(_e1133);
                        }
                    }
                    if _e317 {
                        break;
                    }
                    let _e1939 = local_22;
                    phi_2322_ = _e1939;
                } else {
                    phi_2322_ = _e1127;
                }
                let _e1335 = phi_2322_;
                phi_2323_ = _e1335;
            } else {
                phi_2323_ = _e1127;
            }
            let _e1337 = phi_2323_;
            let _e1338 = (_e370 + _e380);
            phi_2335_ = _e1337;
            phi_2338_ = 0u;
            loop {
                let _e1342 = phi_2335_;
                let _e1344 = phi_2338_;
                local_18 = _e1342;
                local_19 = _e1342;
                local_20 = _e1342;
                local_21 = _e1342;
                let _e1345 = (_e1344 < select(_e1338, 8u, (8u < _e1338)));
                if _e1345 {
                    if (_e1344 < 8u) {
                    } else {
                        phi_15110_ = true;
                        break;
                    }
                    let _e1351 = pill.member[_e163].playlist_images[_e1344];
                    if (_e1351 >= 0i) {
                        let _e1353 = (_e1344 < _e370);
                        if _e1353 {
                            phi_2374_ = render_shared_RipplePulse(vec2<f32>(_e353, _e356), _e372, 1f);
                            phi_2375_ = (f32(_e1344) + _e366);
                        } else {
                            phi_2374_ = render_shared_RipplePulse(vec2<f32>(_e353, _e376), _e381, _e360);
                            phi_2375_ = f32((_e1344 - _e370));
                        }
                        let _e1359 = phi_2374_;
                        let _e1361 = phi_2375_;
                        let _e1362 = select(_e360, _e387, _e1353);
                        let _e1364 = (_e1359.start_time - 1f);
                        if (_e1364 != _e1364) {
                            phi_12488_ = true;
                        } else {
                            phi_12488_ = (0f >= _e1364);
                        }
                        let _e1368 = phi_12488_;
                        let _e1377 = (_e1359.origin.x + (((_e1361 - (select(_e1364, 0f, _e1368) * 0.5f)) * 18f) * _e1359.strength));
                        let _e1380 = (_e1359.origin.y + 2f);
                        if (_e1362 > 0f) {
                            let _e1382 = (_e162.x - _e1377);
                            let _e1383 = (_e162.y - _e1380);
                            let _e1384 = abs(_e1382);
                            let _e1385 = abs(_e1383);
                            if (select(_e1385, _e1384, (_e1384 > _e1385)) < 38.88f) {
                                let _e1389 = (_e1377 - _e200);
                                let _e1390 = (_e1380 - _e205);
                                let _e1394 = sqrt(((_e1389 * _e1389) + (_e1390 * _e1390)));
                                let _e1396 = ((_e1394 - 11.3f) * -1f);
                                let _e1398 = select(_e1396, 0f, (_e1396 < 0f));
                                let _e1400 = select(_e1398, 1f, (_e1398 > 1f));
                                let _e1406 = select(_e321, 0f, (_e321 < 0f));
                                let _e1409 = (((_e1400 * _e1400) * (3f - (2f * _e1400))) * select(_e1406, 1f, (_e1406 > 1f)));
                                let _e1411 = (1.05f + (0.63f * _e1409));
                                let _e1412 = (_e1389 * _e1409);
                                let _e1414 = (_e1382 - (_e1412 * 0.5f));
                                let _e1415 = (_e1412 * -0.005f);
                                let _e1416 = sin(_e1415);
                                let _e1417 = cos(_e1415);
                                let _e1420 = ((_e1417 * _e1414) - (_e1416 * _e1383));
                                let _e1423 = ((_e1416 * _e1414) + (_e1417 * _e1383));
                                let _e1424 = (_e1411 * 21.6f);
                                if _e1353 {
                                    phi_2499_ = true;
                                } else {
                                    if _e322 {
                                        phi_2494_ = select(true, false, (_e1394 <= 10.8f));
                                    } else {
                                        phi_2494_ = true;
                                    }
                                    let _e1432 = phi_2494_;
                                    phi_2499_ = select(true, false, _e1432);
                                }
                                let _e1435 = phi_2499_;
                                let _e1436 = select(0.2f, 0f, _e1435);
                                let _e1439 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e1420, _e1423), 0f, (_e1411 * 6.4800005f));
                                if (_e1439 <= 7f) {
                                    let _e1442 = vec3<f32>(((_e1420 / _e1424) + 0.5f), ((_e1423 / _e1424) + 0.5f), f32(_e1351));
                                    let _e1448 = textureSample(images, sampler_, vec2<f32>(_e1442.x, _e1442.y), i32(_e1442.z));
                                    let _e1452 = (1f - _e1436);
                                    let _e1456 = (0.24f * _e1436);
                                    let _e1460 = (0.5f - _e1439);
                                    let _e1462 = select(_e1460, 0f, (_e1460 < 0f));
                                    let _e1464 = select(_e1462, 1f, (_e1462 > 1f));
                                    if (_e1439 != _e1439) {
                                        phi_12525_ = true;
                                    } else {
                                        phi_12525_ = (0f >= _e1439);
                                    }
                                    let _e1468 = phi_12525_;
                                    let _e1471 = exp((select(_e1439, 0f, _e1468) * -0.5f));
                                    let _e1472 = (_e1439 * -0.2f);
                                    let _e1474 = select(_e1472, 0f, (_e1472 < 0f));
                                    let _e1476 = select(_e1474, 1f, (_e1474 > 1f));
                                    let _e1481 = (1f - ((_e1476 * _e1476) * (3f - (2f * _e1476))));
                                    let _e1483 = ((_e1481 * _e1481) * 0.045f);
                                    let _e1494 = ((_e1471 * _e1471) * 0.2f);
                                    if (_e1464 != _e1464) {
                                        phi_12540_ = true;
                                    } else {
                                        phi_12540_ = (_e1494 >= _e1464);
                                    }
                                    let _e1498 = phi_12540_;
                                    let _e1500 = (select(_e1464, _e1494, _e1498) * _e1362);
                                    let _e1501 = (1f - _e1500);
                                    phi_2601_ = vec4<f32>(((_e1342.x * _e1501) + (((((_e1448.x * _e1452) + _e1456) + _e1483) * _e1464) * _e1362)), ((_e1342.y * _e1501) + (((((_e1448.y * _e1452) + _e1456) + _e1483) * _e1464) * _e1362)), ((_e1342.z * _e1501) + (((((_e1448.z * _e1452) + _e1456) + _e1483) * _e1464) * _e1362)), ((_e1342.w * _e1501) + _e1500));
                                } else {
                                    phi_2601_ = _e1342;
                                }
                                let _e1516 = phi_2601_;
                                phi_2602_ = _e1516;
                            } else {
                                phi_2602_ = _e1342;
                            }
                            let _e1518 = phi_2602_;
                            phi_2603_ = _e1518;
                        } else {
                            phi_2603_ = _e1342;
                        }
                        let _e1520 = phi_2603_;
                        phi_2604_ = _e1520;
                    } else {
                        phi_2604_ = _e1342;
                    }
                    let _e1522 = phi_2604_;
                    phi_2336_ = _e1522;
                    phi_2339_ = (_e1344 + 1u);
                } else {
                    phi_2336_ = vec4<f32>();
                    phi_2339_ = u32();
                }
                let _e1525 = phi_2336_;
                let _e1527 = phi_2339_;
                continue;
                continuing {
                    phi_2335_ = _e1525;
                    phi_2338_ = _e1527;
                    phi_15110_ = _e317;
                    break if !(_e1345);
                }
            }
            let _e1530 = phi_15110_;
            if _e1530 {
                break;
            }
            local_7 = array<u32, 2>(0u, 1u);
            phi_15145_ = _e1530;
            phi_2612_ = 0u;
            phi_2615_ = -1000000f;
            loop {
                let _e1532 = phi_15145_;
                let _e1534 = phi_2612_;
                let _e1536 = phi_2615_;
                local_17 = _e1536;
                let _e1537 = (_e1534 < 2u);
                if _e1537 {
                    if _e1537 {
                    } else {
                        phi_15164_ = true;
                        break;
                    }
                    let _e1539 = local_7[_e1534];
                    if (_e1539 < 2u) {
                    } else {
                        phi_15164_ = true;
                        break;
                    }
                    let _e1548 = pill.member[_e163].text.lines[_e1539].min[0u];
                    let _e1556 = pill.member[_e163].text.lines[_e1539].min[1u];
                    let _e1564 = pill.member[_e163].text.lines[_e1539].max[0u];
                    let _e1572 = pill.member[_e163].text.lines[_e1539].max[1u];
                    let _e1580 = pill.member[_e163].text.lines[_e1539].origin[0u];
                    let _e1588 = pill.member[_e163].text.lines[_e1539].origin[1u];
                    let _e1595 = pill.member[_e163].text.lines[_e1539].size;
                    let _e1602 = pill.member[_e163].text.lines[_e1539].weight;
                    let _e1609 = pill.member[_e163].text.lines[_e1539].count;
                    let _e1616 = pill.member[_e163].text.lines[_e1539].first;
                    if (_e576 < _e1548) {
                        phi_15169_ = _e1532;
                        phi_2883_ = f32();
                        phi_2884_ = true;
                    } else {
                        if (_e576 > _e1564) {
                            phi_15170_ = _e1532;
                            phi_2881_ = f32();
                            phi_2882_ = true;
                        } else {
                            if (_e577 < _e1556) {
                                phi_15171_ = _e1532;
                                phi_2879_ = f32();
                                phi_2880_ = true;
                            } else {
                                let _e1620 = (_e577 > _e1572);
                                if _e1620 {
                                    phi_15172_ = _e1532;
                                    phi_2878_ = f32();
                                } else {
                                    phi_2695_ = _e1609;
                                    phi_2698_ = 0u;
                                    loop {
                                        let _e1622 = phi_2695_;
                                        let _e1624 = phi_2698_;
                                        local_14 = _e1624;
                                        let _e1625 = (_e1624 < _e1622);
                                        if _e1625 {
                                            let _e1628 = (_e1624 + ((_e1622 - _e1624) / 2u));
                                            let _e1629 = (_e1616 + _e1628);
                                            if (_e1629 < 128u) {
                                            } else {
                                                phi_15142_ = true;
                                                break;
                                            }
                                            let _e1637 = pill.member[_e163].text.glyphs[_e1629].x;
                                            let _e1640 = (_e1637 <= ((_e576 - _e1580) / _e1595));
                                            if _e1640 {
                                                phi_2732_ = (_e1628 + 1u);
                                            } else {
                                                phi_2732_ = _e1624;
                                            }
                                            let _e1643 = phi_2732_;
                                            phi_2696_ = select(_e1628, _e1622, _e1640);
                                            phi_2699_ = _e1643;
                                        } else {
                                            phi_2696_ = u32();
                                            phi_2699_ = u32();
                                        }
                                        let _e1646 = phi_2696_;
                                        let _e1648 = phi_2699_;
                                        continue;
                                        continuing {
                                            phi_2695_ = _e1646;
                                            phi_2698_ = _e1648;
                                            phi_15142_ = _e1532;
                                            break if !(_e1625);
                                        }
                                    }
                                    let _e1651 = phi_15142_;
                                    phi_15164_ = _e1651;
                                    if _e1651 {
                                        break;
                                    }
                                    let _e1653 = local_14;
                                    let _e1654 = (_e1653 + 1u);
                                    phi_15150_ = _e1651;
                                    phi_2740_ = select(_e1654, _e1609, (_e1609 < _e1654));
                                    phi_2743_ = -1000000f;
                                    loop {
                                        let _e1658 = phi_15150_;
                                        let _e1660 = phi_2740_;
                                        let _e1662 = phi_2743_;
                                        local_23 = _e1662;
                                        if (_e1660 > 0u) {
                                            let _e1664 = (_e1660 - 1u);
                                            let _e1665 = (_e1616 + _e1664);
                                            if (_e1665 < 128u) {
                                            } else {
                                                phi_15154_ = true;
                                                break;
                                            }
                                            let _e1673 = pill.member[_e163].text.glyphs[_e1665].x;
                                            let _e1680 = pill.member[_e163].text.glyphs[_e1665].glyph;
                                            if (_e1680 < arrayLength((&glyphs.member))) {
                                            } else {
                                                phi_15154_ = true;
                                                break;
                                            }
                                            let _e1686 = glyphs.member[_e1680].min[0u];
                                            let _e1691 = glyphs.member[_e1680].min[1u];
                                            let _e1696 = glyphs.member[_e1680].max[0u];
                                            let _e1701 = glyphs.member[_e1680].max[1u];
                                            let _e1705 = glyphs.member[_e1680].start;
                                            let _e1709 = glyphs.member[_e1680].count;
                                            let _e1712 = (((_e576 - _e1580) / _e1595) - _e1673);
                                            let _e1715 = (-((_e577 - _e1588)) / _e1595);
                                            let _e1716 = (3.5f / _e1595);
                                            let _e1717 = (_e1696 + _e1716);
                                            let _e1718 = (_e1712 > _e1717);
                                            if _e1718 {
                                                phi_15156_ = _e1658;
                                                phi_2871_ = f32();
                                            } else {
                                                if (_e1712 >= (_e1686 - _e1716)) {
                                                    if (_e1715 >= (_e1691 - _e1716)) {
                                                        if (_e1712 <= _e1717) {
                                                            if (_e1715 <= (_e1701 + _e1716)) {
                                                                phi_2830_ = 0u;
                                                                phi_2833_ = 0i;
                                                                phi_2835_ = 340282350000000000000000000000000000000f;
                                                                loop {
                                                                    let _e1727 = phi_2830_;
                                                                    let _e1729 = phi_2833_;
                                                                    let _e1731 = phi_2835_;
                                                                    local_15 = _e1731;
                                                                    local_16 = _e1729;
                                                                    let _e1732 = (_e1727 < _e1709);
                                                                    if _e1732 {
                                                                        let _e1733 = (_e1705 + _e1727);
                                                                        if (_e1733 < arrayLength((&edges.member))) {
                                                                        } else {
                                                                            phi_15147_ = true;
                                                                            break;
                                                                        }
                                                                        let _e1737 = edges.member[_e1733];
                                                                        let _e1739 = cantus_render_text_edge_distance(_e1737, _e1602, vec2<f32>(_e1712, _e1715));
                                                                        if (_e1731 != _e1731) {
                                                                            phi_12555_ = true;
                                                                        } else {
                                                                            phi_12555_ = (_e1739.member <= _e1731);
                                                                        }
                                                                        let _e1745 = phi_12555_;
                                                                        phi_2831_ = (_e1727 + 1u);
                                                                        phi_2834_ = (_e1729 + _e1739.member_1);
                                                                        phi_2836_ = select(_e1731, _e1739.member, _e1745);
                                                                    } else {
                                                                        phi_2831_ = u32();
                                                                        phi_2834_ = i32();
                                                                        phi_2836_ = f32();
                                                                    }
                                                                    let _e1750 = phi_2831_;
                                                                    let _e1752 = phi_2834_;
                                                                    let _e1754 = phi_2836_;
                                                                    continue;
                                                                    continuing {
                                                                        phi_2830_ = _e1750;
                                                                        phi_2833_ = _e1752;
                                                                        phi_2835_ = _e1754;
                                                                        phi_15147_ = _e1658;
                                                                        break if !(_e1732);
                                                                    }
                                                                }
                                                                let _e1757 = phi_15147_;
                                                                phi_15154_ = _e1757;
                                                                if _e1757 {
                                                                    break;
                                                                }
                                                                let _e1759 = local_15;
                                                                let _e1763 = local_16;
                                                                let _e1766 = ((sqrt(_e1759) * _e1595) * select(1f, -1f, (_e1763 == 0i)));
                                                                if (_e1662 != _e1662) {
                                                                    phi_12570_ = true;
                                                                } else {
                                                                    phi_12570_ = (_e1766 >= _e1662);
                                                                }
                                                                let _e1770 = phi_12570_;
                                                                phi_15160_ = _e1757;
                                                                phi_2867_ = select(_e1662, _e1766, _e1770);
                                                            } else {
                                                                phi_15160_ = _e1658;
                                                                phi_2867_ = _e1662;
                                                            }
                                                            let _e1773 = phi_15160_;
                                                            let _e1775 = phi_2867_;
                                                            phi_15159_ = _e1773;
                                                            phi_2868_ = _e1775;
                                                        } else {
                                                            phi_15159_ = _e1658;
                                                            phi_2868_ = _e1662;
                                                        }
                                                        let _e1777 = phi_15159_;
                                                        let _e1779 = phi_2868_;
                                                        phi_15158_ = _e1777;
                                                        phi_2869_ = _e1779;
                                                    } else {
                                                        phi_15158_ = _e1658;
                                                        phi_2869_ = _e1662;
                                                    }
                                                    let _e1781 = phi_15158_;
                                                    let _e1783 = phi_2869_;
                                                    phi_15157_ = _e1781;
                                                    phi_2870_ = _e1783;
                                                } else {
                                                    phi_15157_ = _e1658;
                                                    phi_2870_ = _e1662;
                                                }
                                                let _e1785 = phi_15157_;
                                                let _e1787 = phi_2870_;
                                                phi_15156_ = _e1785;
                                                phi_2871_ = _e1787;
                                            }
                                            let _e1789 = phi_15156_;
                                            let _e1791 = phi_2871_;
                                            phi_15155_ = _e1789;
                                            phi_2741_ = _e1664;
                                            phi_2744_ = _e1791;
                                            phi_2873_ = select(true, false, _e1718);
                                        } else {
                                            phi_15155_ = _e1658;
                                            phi_2741_ = u32();
                                            phi_2744_ = f32();
                                            phi_2873_ = false;
                                        }
                                        let _e1794 = phi_15155_;
                                        let _e1796 = phi_2741_;
                                        let _e1798 = phi_2744_;
                                        let _e1800 = phi_2873_;
                                        continue;
                                        continuing {
                                            phi_15150_ = _e1794;
                                            phi_2740_ = _e1796;
                                            phi_2743_ = _e1798;
                                            phi_15154_ = _e1794;
                                            break if !(_e1800);
                                        }
                                    }
                                    let _e1803 = phi_15154_;
                                    phi_15164_ = _e1803;
                                    if _e1803 {
                                        break;
                                    }
                                    phi_15172_ = _e1803;
                                    let _e1999 = local_23;
                                    phi_2878_ = _e1999;
                                }
                                let _e1805 = phi_15172_;
                                let _e1807 = phi_2878_;
                                phi_15171_ = _e1805;
                                phi_2879_ = _e1807;
                                phi_2880_ = _e1620;
                            }
                            let _e1809 = phi_15171_;
                            let _e1811 = phi_2879_;
                            let _e1813 = phi_2880_;
                            phi_15170_ = _e1809;
                            phi_2881_ = _e1811;
                            phi_2882_ = _e1813;
                        }
                        let _e1815 = phi_15170_;
                        let _e1817 = phi_2881_;
                        let _e1819 = phi_2882_;
                        phi_15169_ = _e1815;
                        phi_2883_ = _e1817;
                        phi_2884_ = _e1819;
                    }
                    let _e1821 = phi_15169_;
                    let _e1823 = phi_2883_;
                    let _e1825 = phi_2884_;
                    let _e1826 = select(_e1823, -1000000f, _e1825);
                    if (_e1536 != _e1536) {
                        phi_12585_ = true;
                    } else {
                        phi_12585_ = (_e1826 >= _e1536);
                    }
                    let _e1830 = phi_12585_;
                    phi_15165_ = _e1821;
                    phi_2613_ = (_e1534 + 1u);
                    phi_2616_ = select(_e1536, _e1826, _e1830);
                } else {
                    phi_15165_ = _e1532;
                    phi_2613_ = u32();
                    phi_2616_ = f32();
                }
                let _e1834 = phi_15165_;
                let _e1836 = phi_2613_;
                let _e1838 = phi_2616_;
                continue;
                continuing {
                    phi_15145_ = _e1834;
                    phi_2612_ = _e1836;
                    phi_2615_ = _e1838;
                    phi_15164_ = _e1834;
                    break if !(_e1537);
                }
            }
            let _e1841 = phi_15164_;
            if _e1841 {
                break;
            }
            let _e1845 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e576 - _e975), (_e577 - _e189)), 0f, _e189);
            let _e1846 = (_e1845 * 0.125f);
            let _e1848 = select(_e1846, 0f, (_e1846 < 0f));
            let _e1850 = select(_e1848, 1f, (_e1848 > 1f));
            let _e1856 = local_17;
            let _e1858 = ((_e1856 * 1.25f) + 0.5f);
            let _e1860 = select(_e1858, 0f, (_e1858 < 0f));
            let _e1862 = select(_e1860, 1f, (_e1860 > 1f));
            let _e1868 = ((((_e1862 * _e1862) * (3f - (2f * _e1862))) * ((_e1850 * _e1850) * (3f - (2f * _e1850)))) * _e536);
            let _e1869 = (1f - _e1868);
            let _e1871 = local_18;
            let _e1875 = local_19;
            let _e1879 = local_20;
            let _e1883 = local_21;
            let _e1886 = (0.94f * _e1868);
            let _e1894 = (((_e1883.w * _e1869) + _e1868) * _e553);
            if (_e1894 <= 0f) {
                discard;
            }
            out_color = vec4<f32>((((_e1871.x * _e1869) + _e1886) * _e553), (((_e1875.y * _e1869) + _e1886) * _e553), (((_e1879.z * _e1869) + _e1886) * _e553), _e1894);
            break;
        }
    }
    return;
}

fn render_status_vertex_impl() {
    var local_24: array<u32, 6>;
    var phi_3456_: u32;
    var phi_3459_: f32;
    var phi_3483_: bool;
    var phi_3492_: bool;
    var phi_12602_: bool;
    var phi_12603_: bool;
    var phi_12604_: bool;
    var phi_3499_: f32;
    var phi_3457_: u32;
    var phi_3460_: f32;
    var phi_15173_: bool;
    var local_25: f32;
    var local_26: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e33 = vertex_5;
            let _e34 = _isthmus_instance_index_7;
            phi_3456_ = 0u;
            phi_3459_ = 12f;
            loop {
                let _e36 = phi_3456_;
                let _e38 = phi_3459_;
                local_25 = _e38;
                local_26 = _e38;
                let _e39 = (_e36 < 5u);
                if _e39 {
                    local_24 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                    if (_e36 < 6u) {
                    } else {
                        phi_15173_ = true;
                        break;
                    }
                    let _e42 = local_24[_e36];
                    let _e46 = pill_1.member[_e34].battery_level;
                    if (_e46 >= -1f) {
                        phi_3483_ = (_e46 <= 1f);
                    } else {
                        phi_3483_ = false;
                    }
                    let _e50 = phi_3483_;
                    if _e50 {
                        phi_3492_ = true;
                    } else {
                        phi_3492_ = select(true, false, (_e42 == 2u));
                    }
                    let _e54 = phi_3492_;
                    if _e54 {
                        switch bitcast<i32>(_e42) {
                            case 0: {
                                phi_12602_ = true;
                                phi_12603_ = false;
                                phi_12604_ = false;
                                break;
                            }
                            case 1: {
                                phi_12602_ = true;
                                phi_12603_ = false;
                                phi_12604_ = false;
                                break;
                            }
                            case 2: {
                                phi_12602_ = false;
                                phi_12603_ = true;
                                phi_12604_ = false;
                                break;
                            }
                            case 3: {
                                phi_12602_ = false;
                                phi_12603_ = true;
                                phi_12604_ = false;
                                break;
                            }
                            case 4: {
                                phi_12602_ = false;
                                phi_12603_ = false;
                                phi_12604_ = true;
                                break;
                            }
                            case 5: {
                                phi_12602_ = false;
                                phi_12603_ = false;
                                phi_12604_ = true;
                                break;
                            }
                            default: {
                                phi_12602_ = bool();
                                phi_12603_ = bool();
                                phi_12604_ = bool();
                                break;
                            }
                        }
                        let _e57 = phi_12602_;
                        let _e59 = phi_12603_;
                        let _e61 = phi_12604_;
                        let _e62 = select(_e59, false, _e57);
                        phi_3499_ = (_e38 + (select(select(80f, 32f, _e62), 24f, select(select(_e61, false, _e57), false, _e62)) + 8f));
                    } else {
                        phi_3499_ = _e38;
                    }
                    let _e70 = phi_3499_;
                    phi_3457_ = (_e36 + 1u);
                    phi_3460_ = _e70;
                } else {
                    phi_3457_ = u32();
                    phi_3460_ = f32();
                }
                let _e73 = phi_3457_;
                let _e75 = phi_3460_;
                continue;
                continuing {
                    phi_3456_ = _e73;
                    phi_3459_ = _e75;
                    phi_15173_ = false;
                    break if !(_e39);
                }
            }
            let _e78 = phi_15173_;
            if _e78 {
                break;
            }
            let _e80 = local_25;
            let _e86 = frame.member[0u].screen_size[0u];
            let _e92 = frame.member[0u].panel_top;
            let _e102 = frame.member[0u].panel_height;
            let _e105 = local_26;
            let _e109 = (((_e86 - (_e80 + 36f)) - 56f) + (f32((_e33 & 1u)) * (_e105 + 132f)));
            let _e110 = ((_e92 - 48f) + (f32((_e33 >> bitcast<u32>(1i))) * (_e102 + 96f)));
            let _e115 = frame.member[0u].screen_size[1u];
            let _e118 = cantus_render_shader_pixel_to_ndc(vec2<f32>(_e109, _e110), vec2<f32>(_e86, _e115));
            out_position = _e118;
            out_pixel[0u] = _e109;
            out_pixel[1u] = _e110;
            out_isthmus_instance_index = _e34;
            break;
        }
    }
    return;
}

fn cantus_render_shader_hash(param_13: vec2<f32>) -> vec2<f32> {
    let _e31 = ((bitcast<u32>(select(0i, select(select(i32(param_13.y), i32(-2147483648), (param_13.y < -2147483600f)), 2147483647i, (param_13.y > 2147483500f)), (param_13.y == param_13.y))) * 1664525u) + 1013904223u);
    let _e33 = (((bitcast<u32>(select(0i, select(select(i32(param_13.x), i32(-2147483648), (param_13.x < -2147483600f)), 2147483647i, (param_13.x > 2147483500f)), (param_13.x == param_13.x))) * 1664525u) + 1013904223u) + (_e31 * 1664525u));
    let _e35 = (_e31 + (_e33 * 1664525u));
    let _e41 = (_e35 ^ (_e35 >> bitcast<u32>(16i)));
    let _e43 = ((_e33 ^ (_e33 >> bitcast<u32>(16i))) + (_e41 * 1664525u));
    let _e45 = (_e41 + (_e43 * 1664525u));
    return vec2<f32>((f32((_e43 ^ (_e43 >> bitcast<u32>(16i)))) * 0.00000000023283064f), (f32((_e45 ^ (_e45 >> bitcast<u32>(16i)))) * 0.00000000023283064f));
}

fn cantus_render_shader_sd_rounded_box(param_14: vec2<f32>, param_15: vec2<f32>, param_16: f32) -> f32 {
    var phi_14943_: bool;
    var phi_14958_: bool;

    let _e13 = ((abs(param_14.x) - param_15.x) + param_16);
    let _e14 = ((abs(param_14.y) - param_15.y) + param_16);
    let _e16 = select(0f, _e13, (_e13 > 0f));
    let _e18 = select(0f, _e14, (_e14 > 0f));
    if (_e13 != _e13) {
        phi_14943_ = true;
    } else {
        phi_14943_ = (_e14 >= _e13);
    }
    let _e26 = phi_14943_;
    let _e27 = select(_e13, _e14, _e26);
    if (_e27 != _e27) {
        phi_14958_ = true;
    } else {
        phi_14958_ = (0f <= _e27);
    }
    let _e31 = phi_14958_;
    return ((sqrt(((_e16 * _e16) + (_e18 * _e18))) + select(_e27, 0f, _e31)) - param_16);
}

fn cantus_render_shader_simplex_noise(param_17: vec2<f32>) -> f32 {
    var phi_14898_: bool;
    var phi_14913_: bool;
    var phi_14928_: bool;

    let _e15 = ((param_17.x + param_17.y) * 0.36602542f);
    let _e18 = floor((param_17.x + _e15));
    let _e19 = floor((param_17.y + _e15));
    let _e23 = ((_e18 + _e19) * 0.21132487f);
    let _e24 = ((param_17.x - _e18) + _e23);
    let _e25 = ((param_17.y - _e19) + _e23);
    let _e28 = select(vec2<f32>(0f, 1f), vec2<f32>(1f, 0f), vec2((_e24 > _e25)));
    let _e33 = ((_e24 - _e28.x) + 0.21132487f);
    let _e34 = ((_e25 - _e28.y) + 0.21132487f);
    let _e35 = (_e24 + -0.57735026f);
    let _e36 = (_e25 + -0.57735026f);
    let _e40 = (0.5f - ((_e24 * _e24) + (_e25 * _e25)));
    if (_e40 != _e40) {
        phi_14898_ = true;
    } else {
        phi_14898_ = (0f >= _e40);
    }
    let _e44 = phi_14898_;
    let _e45 = select(_e40, 0f, _e44);
    let _e50 = cantus_render_shader_hash(vec2<f32>(_e18, _e19));
    let _e64 = (0.5f - ((_e33 * _e33) + (_e34 * _e34)));
    if (_e64 != _e64) {
        phi_14913_ = true;
    } else {
        phi_14913_ = (0f >= _e64);
    }
    let _e68 = phi_14913_;
    let _e69 = select(_e64, 0f, _e68);
    let _e76 = cantus_render_shader_hash(vec2<f32>((_e18 + _e28.x), (_e19 + _e28.y)));
    let _e91 = (0.5f - ((_e35 * _e35) + (_e36 * _e36)));
    if (_e91 != _e91) {
        phi_14928_ = true;
    } else {
        phi_14928_ = (0f >= _e91);
    }
    let _e95 = phi_14928_;
    let _e96 = select(_e91, 0f, _e95);
    let _e103 = cantus_render_shader_hash(vec2<f32>((_e18 + 1f), (_e19 + 1f)));
    return (70f * ((((((_e45 * _e45) * _e45) * _e45) * ((_e24 * ((_e50.x * 2f) - 1f)) + (_e25 * ((_e50.y * 2f) - 1f)))) + ((((_e69 * _e69) * _e69) * _e69) * ((_e33 * ((_e76.x * 2f) - 1f)) + (_e34 * ((_e76.y * 2f) - 1f))))) + ((((_e96 * _e96) * _e96) * _e96) * ((_e35 * ((_e103.x * 2f) - 1f)) + (_e36 * ((_e103.y * 2f) - 1f))))));
}

fn render_status_fragment_impl() {
    var local_27: array<u32, 6>;
    var local_28: array<u32, 6>;
    var local_29: array<u32, 6>;
    var phi_3618_: u32;
    var phi_3621_: f32;
    var phi_3645_: bool;
    var phi_3654_: bool;
    var phi_12694_: bool;
    var phi_12695_: bool;
    var phi_12696_: bool;
    var phi_3661_: f32;
    var phi_3619_: u32;
    var phi_3622_: f32;
    var phi_15184_: bool;
    var local_30: f32;
    var phi_3708_: vec2<f32>;
    var phi_3711_: f32;
    var phi_3713_: u32;
    var phi_12789_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_12800_: bool;
    var phi_3709_: vec2<f32>;
    var phi_3712_: f32;
    var phi_3714_: u32;
    var phi_15195_: bool;
    var phi_3852_: f32;
    var local_31: vec2<f32>;
    var local_32: vec2<f32>;
    var phi_12818_: bool;
    var phi_12833_: bool;
    var phi_12848_: bool;
    var phi_12865_: bool;
    var phi_4228_: u32;
    var phi_4231_: f32;
    var phi_4233_: f32;
    var phi_4261_: bool;
    var phi_4265_: bool;
    var phi_12880_: bool;
    var phi_12881_: bool;
    var phi_12882_: bool;
    var phi_4282_: f32;
    var phi_4283_: f32;
    var phi_4284_: f32;
    var phi_4285_: bool;
    var phi_4290_: u32;
    var phi_4229_: u32;
    var phi_4232_: f32;
    var phi_4234_: f32;
    var phi_4291_: u32;
    var phi_4292_: f32;
    var phi_4293_: bool;
    var phi_15213_: bool;
    var phi_11710_: bool;
    var phi_11709_: f32;
    var phi_11707_: u32;
    var local_33: f32;
    var phi_4299_: render_track_PaletteColor;
    var phi_4305_: render_track_PaletteColor;
    var phi_4316_: vec2<f32>;
    var phi_4317_: bool;
    var phi_12970_: i32;
    var phi_12971_: f32;
    var phi_12972_: f32;
    var phi_12973_: vec2<f32>;
    var phi_12998_: i32;
    var phi_12999_: f32;
    var phi_13000_: f32;
    var phi_13001_: vec2<f32>;
    var local_34: f32;
    var phi_4483_: vec2<f32>;
    var phi_4487_: u32;
    var phi_4490_: f32;
    var phi_4514_: bool;
    var phi_4523_: bool;
    var phi_13016_: bool;
    var phi_13017_: bool;
    var phi_13018_: bool;
    var phi_4530_: f32;
    var phi_4488_: u32;
    var phi_4491_: f32;
    var phi_15268_: bool;
    var local_35: f32;
    var phi_13079_: i32;
    var phi_13080_: f32;
    var phi_13081_: f32;
    var phi_13082_: vec2<f32>;
    var phi_13107_: i32;
    var phi_13108_: f32;
    var phi_13109_: f32;
    var phi_13110_: vec2<f32>;
    var local_36: f32;
    var phi_4648_: vec2<f32>;
    var phi_15301_: bool;
    var phi_4662_: vec2<f32>;
    var phi_13125_: bool;
    var phi_13140_: bool;
    var phi_13141_: bool;
    var phi_13142_: bool;
    var phi_13167_: bool;
    var phi_13182_: bool;
    var phi_13197_: bool;
    var phi_13212_: bool;
    var phi_13213_: bool;
    var phi_13214_: bool;
    var phi_13318_: bool;
    var phi_13333_: bool;
    var phi_13348_: bool;
    var phi_13363_: bool;
    var phi_13378_: bool;
    var phi_13393_: bool;
    var phi_13408_: bool;
    var phi_15294_: bool;
    var phi_6759_: vec3<f32>;
    var phi_6760_: bool;
    var phi_13423_: bool;
    var phi_13468_: bool;
    var phi_13483_: bool;
    var phi_13498_: bool;
    var phi_7160_: f32;
    var phi_13513_: bool;
    var phi_7187_: vec3<f32>;
    var phi_7197_: bool;
    var phi_7267_: u32;
    var phi_7270_: u32;
    var phi_7304_: u32;
    var phi_7268_: u32;
    var phi_7271_: u32;
    var phi_15465_: bool;
    var local_37: u32;
    var phi_15515_: bool;
    var phi_7312_: u32;
    var phi_7315_: f32;
    var phi_7402_: u32;
    var phi_7405_: i32;
    var phi_7407_: f32;
    var phi_13528_: bool;
    var phi_7403_: u32;
    var phi_7406_: i32;
    var phi_7408_: f32;
    var phi_15512_: bool;
    var local_38: f32;
    var local_39: i32;
    var phi_13543_: bool;
    var phi_15525_: bool;
    var phi_7439_: f32;
    var phi_15524_: bool;
    var phi_7440_: f32;
    var phi_15523_: bool;
    var phi_7441_: f32;
    var phi_15522_: bool;
    var phi_7442_: f32;
    var phi_15521_: bool;
    var phi_7443_: f32;
    var phi_15520_: bool;
    var phi_7313_: u32;
    var phi_7316_: f32;
    var phi_7445_: bool;
    var phi_15519_: bool;
    var phi_7450_: f32;
    var phi_7451_: f32;
    var phi_7452_: bool;
    var phi_7453_: f32;
    var phi_7454_: bool;
    var phi_7455_: f32;
    var phi_7456_: bool;
    var phi_7461_: f32;
    var local_40: f32;
    var local_41: f32;
    var local_42: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e228 = pixel_2;
            let _e229 = _isthmus_instance_index_8;
            phi_3618_ = 0u;
            phi_3621_ = 12f;
            loop {
                let _e237 = phi_3618_;
                let _e239 = phi_3621_;
                local_30 = _e239;
                let _e240 = (_e237 < 5u);
                if _e240 {
                    local_29 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                    if (_e237 < 6u) {
                    } else {
                        phi_15184_ = true;
                        break;
                    }
                    let _e243 = local_29[_e237];
                    let _e247 = pill_1.member[_e229].battery_level;
                    if (_e247 >= -1f) {
                        phi_3645_ = (_e247 <= 1f);
                    } else {
                        phi_3645_ = false;
                    }
                    let _e251 = phi_3645_;
                    if _e251 {
                        phi_3654_ = true;
                    } else {
                        phi_3654_ = select(true, false, (_e243 == 2u));
                    }
                    let _e255 = phi_3654_;
                    if _e255 {
                        switch bitcast<i32>(_e243) {
                            case 0: {
                                phi_12694_ = true;
                                phi_12695_ = false;
                                phi_12696_ = false;
                                break;
                            }
                            case 1: {
                                phi_12694_ = true;
                                phi_12695_ = false;
                                phi_12696_ = false;
                                break;
                            }
                            case 2: {
                                phi_12694_ = false;
                                phi_12695_ = true;
                                phi_12696_ = false;
                                break;
                            }
                            case 3: {
                                phi_12694_ = false;
                                phi_12695_ = true;
                                phi_12696_ = false;
                                break;
                            }
                            case 4: {
                                phi_12694_ = false;
                                phi_12695_ = false;
                                phi_12696_ = true;
                                break;
                            }
                            case 5: {
                                phi_12694_ = false;
                                phi_12695_ = false;
                                phi_12696_ = true;
                                break;
                            }
                            default: {
                                phi_12694_ = bool();
                                phi_12695_ = bool();
                                phi_12696_ = bool();
                                break;
                            }
                        }
                        let _e258 = phi_12694_;
                        let _e260 = phi_12695_;
                        let _e262 = phi_12696_;
                        let _e263 = select(_e260, false, _e258);
                        phi_3661_ = (_e239 + (select(select(80f, 32f, _e263), 24f, select(select(_e262, false, _e258), false, _e263)) + 8f));
                    } else {
                        phi_3661_ = _e239;
                    }
                    let _e271 = phi_3661_;
                    phi_3619_ = (_e237 + 1u);
                    phi_3622_ = _e271;
                } else {
                    phi_3619_ = u32();
                    phi_3622_ = f32();
                }
                let _e274 = phi_3619_;
                let _e276 = phi_3622_;
                continue;
                continuing {
                    phi_3618_ = _e274;
                    phi_3621_ = _e276;
                    phi_15184_ = false;
                    break if !(_e240);
                }
            }
            let _e279 = phi_15184_;
            if _e279 {
                break;
            }
            let _e281 = local_30;
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
            phi_3708_ = vec2<f32>(0f, 0f);
            phi_3711_ = 0f;
            phi_3713_ = 0u;
            loop {
                let _e325 = phi_3708_;
                let _e327 = phi_3711_;
                let _e329 = phi_3713_;
                local_31 = _e325;
                local_32 = _e325;
                local_40 = _e327;
                local_41 = _e327;
                let _e330 = (_e329 < 4u);
                if _e330 {
                    if _e330 {
                    } else {
                        phi_15195_ = true;
                        break;
                    }
                    let _e337 = frame.member[0u].ripples[_e329].origin[0u];
                    let _e344 = frame.member[0u].ripples[_e329].origin[1u];
                    let _e350 = frame.member[0u].ripples[_e329].start_time;
                    let _e356 = frame.member[0u].ripples[_e329].strength;
                    let _e360 = frame.member[0u].time;
                    let _e362 = ((_e360 - _e350) * 1.2f);
                    let _e364 = select(_e362, 0f, (_e362 < 0f));
                    let _e366 = select(_e364, 1f, (_e364 > 1f));
                    let _e368 = (_e228 - vec2<f32>(_e337, _e344));
                    let _e374 = sqrt(((_e368.x * _e368.x) + (_e368.y * _e368.y)));
                    if (_e374 > 0.001f) {
                        phi_12789_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e368.x / _e374), (_e368.y / _e374)), _e374);
                    } else {
                        phi_12789_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e374);
                    }
                    let _e382 = phi_12789_;
                    let _e392 = ((abs((_e382.unnamed_1 - (_e366 * 600f))) - 80f) * -0.0125f);
                    let _e394 = select(_e392, 0f, (_e392 < 0f));
                    let _e396 = select(_e394, 1f, (_e394 > 1f));
                    let _e402 = (1f - _e366);
                    let _e403 = ((((_e396 * _e396) * (3f - (2f * _e396))) * _e356) * _e402);
                    let _e416 = (_e327 + (_e403 * 0.5f));
                    if (_e416 != _e416) {
                        phi_12800_ = true;
                    } else {
                        phi_12800_ = (1f <= _e416);
                    }
                    let _e420 = phi_12800_;
                    phi_3709_ = vec2<f32>((_e325.x + (((_e382.unnamed.x * _e403) * _e402) * 0.5f)), (_e325.y + (((_e382.unnamed.y * _e403) * _e402) * 0.5f)));
                    phi_3712_ = select(_e416, 1f, _e420);
                    phi_3714_ = (_e329 + 1u);
                } else {
                    phi_3709_ = vec2<f32>();
                    phi_3712_ = f32();
                    phi_3714_ = u32();
                }
                let _e424 = phi_3709_;
                let _e426 = phi_3712_;
                let _e428 = phi_3714_;
                continue;
                continuing {
                    phi_3708_ = _e424;
                    phi_3711_ = _e426;
                    phi_3713_ = _e428;
                    phi_15195_ = _e279;
                    break if !(_e330);
                }
            }
            let _e431 = phi_15195_;
            if _e431 {
                break;
            }
            let _e435 = frame.member[0u].mouse_pressure;
            if (_e435 > 0f) {
                let _e437 = (_e228.x - _e312);
                let _e438 = (_e228.y - _e317);
                let _e444 = ((sqrt(((_e437 * _e437) + (_e438 * _e438))) - 150f) * -0.006666667f);
                let _e446 = select(_e444, 0f, (_e444 < 0f));
                let _e448 = select(_e446, 1f, (_e446 > 1f));
                phi_3852_ = ((((_e448 * _e448) * (3f - (2f * _e448))) * _e435) * 8f);
            } else {
                phi_3852_ = 0f;
            }
            let _e456 = phi_3852_;
            let _e458 = local_31;
            let _e461 = local_32;
            let _e464 = ((_e323 - 0.5f) * -1f);
            let _e466 = select(_e464, 0f, (_e464 < 0f));
            let _e468 = select(_e466, 1f, (_e466 > 1f));
            let _e481 = (_e307 - (((_e456 * ((_e468 * _e468) * (3f - (2f * _e468)))) + (sqrt(((_e458.x * _e458.x) + (_e461.y * _e461.y))) * 22f)) * 0.5f));
            let _e482 = fwidth(_e481);
            if (_e482 != _e482) {
                phi_12818_ = true;
            } else {
                phi_12818_ = (0.55f >= _e482);
            }
            let _e486 = phi_12818_;
            let _e487 = select(_e482, 0.55f, _e486);
            let _e491 = ((_e481 - _e487) / (-(_e487) - _e487));
            let _e493 = select(_e491, 0f, (_e491 < 0f));
            let _e495 = select(_e493, 1f, (_e493 > 1f));
            let _e499 = ((_e495 * _e495) * (3f - (2f * _e495)));
            let _e500 = (_e481 != _e481);
            if _e500 {
                phi_12833_ = true;
            } else {
                phi_12833_ = (0f >= _e481);
            }
            let _e503 = phi_12833_;
            let _e507 = (exp((select(_e481, 0f, _e503) * -0.3f)) * 0.16f);
            if (_e499 != _e499) {
                phi_12848_ = true;
            } else {
                phi_12848_ = (_e507 >= _e499);
            }
            let _e511 = phi_12848_;
            let _e512 = select(_e499, _e507, _e511);
            if (_e512 <= 0.0009765625f) {
                discard;
            }
            let _e514 = (_e298 / _e282);
            let _e515 = (_e299 / _e293);
            if _e500 {
                phi_12865_ = true;
            } else {
                phi_12865_ = (0f <= _e481);
            }
            let _e520 = phi_12865_;
            let _e523 = (1f + (select(_e481, 0f, _e520) * 0.008333334f));
            let _e525 = select(_e523, 0f, (_e523 < 0f));
            let _e527 = select(_e525, 0.6f, (_e525 > 0.6f));
            let _e537 = ((_e515 - (((_e515 - 0.5f) * _e527) * 0.08f)) - (_e461.y * 0.04f));
            let _e541 = pill_1.member[_e229].sun_height;
            let _e545 = pill_1.member[_e229].conditions;
            let _e549 = frame.member[0u].time;
            let _e557 = ((_e537 - 1f) * -1f);
            let _e559 = select(_e557, 0f, (_e557 < 0f));
            let _e561 = select(_e559, 1f, (_e559 > 1f));
            let _e565 = ((_e561 * _e561) * (3f - (2f * _e561)));
            let _e567 = ((_e541 - -0.04f) * 4.1666665f);
            let _e569 = select(_e567, 0f, (_e567 < 0f));
            let _e571 = select(_e569, 1f, (_e569 > 1f));
            let _e575 = ((_e571 * _e571) * (3f - (2f * _e571)));
            let _e577 = ((_e541 - -0.2f) * 4.5454545f);
            let _e579 = select(_e577, 0f, (_e577 < 0f));
            let _e581 = select(_e579, 1f, (_e579 > 1f));
            let _e586 = (1f - _e575);
            let _e587 = (((_e581 * _e581) * (3f - (2f * _e581))) * _e586);
            let _e588 = (1f - _e565);
            let _e600 = (0.65f * _e588);
            let _e624 = (1f - _e587);
            let _e638 = (((_e545.cloud * 0.34f) + (_e545.rain * 0.16f)) + (_e545.hail * 0.08f));
            let _e639 = (1f - _e638);
            let _e650 = (1f - (_e545.snow * 0.16f));
            let _e654 = (_e545.snow * 0.1312f);
            let _e659 = (1f - (_e545.fog * 0.62f));
            let _e672 = ((sin((_e549 * 2.7f)) - 0.92f) * 12.500003f);
            let _e674 = select(_e672, 0f, (_e672 < 0f));
            let _e676 = select(_e674, 1f, (_e674 > 1f));
            let _e681 = (((_e676 * _e676) * (3f - (2f * _e676))) * _e545.lightning);
            let _e683 = (1f - (_e681 * 0.45f));
            let _e694 = ((_e537 - 0.12f) * -8.333334f);
            let _e696 = select(_e694, 0f, (_e694 < 0f));
            let _e698 = select(_e696, 1f, (_e696 > 1f));
            let _e705 = ((_e481 - 5f) * -0.125f);
            let _e707 = select(_e705, 0f, (_e705 < 0f));
            let _e709 = select(_e707, 1f, (_e707 > 1f));
            let _e715 = ((((_e698 * _e698) * (3f - (2f * _e698))) * 0.12f) + (((_e709 * _e709) * (3f - (2f * _e709))) * 0.08f));
            let _e719 = (((_e514 - (((_e514 - 0.5f) * _e527) * 0.08f)) - (_e458.x * 0.04f)) * _e282);
            phi_4228_ = 0u;
            phi_4231_ = 0f;
            phi_4233_ = 12f;
            loop {
                let _e722 = phi_4228_;
                let _e724 = phi_4231_;
                let _e726 = phi_4233_;
                local_33 = _e724;
                let _e727 = (_e722 < 6u);
                if _e727 {
                    local_28 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                    if _e727 {
                    } else {
                        phi_15213_ = true;
                        phi_11710_ = bool();
                        phi_11709_ = f32();
                        phi_11707_ = u32();
                        break;
                    }
                    let _e729 = local_28[_e722];
                    if (_e729 == 2u) {
                        let _e734 = pill_1.member[_e229].battery_level;
                        if (_e734 >= -1f) {
                            phi_4261_ = (_e734 <= 1f);
                        } else {
                            phi_4261_ = false;
                        }
                        let _e738 = phi_4261_;
                        phi_4265_ = _e738;
                    } else {
                        phi_4265_ = true;
                    }
                    let _e740 = phi_4265_;
                    if _e740 {
                        switch bitcast<i32>(_e729) {
                            case 0: {
                                phi_12880_ = true;
                                phi_12881_ = false;
                                phi_12882_ = false;
                                break;
                            }
                            case 1: {
                                phi_12880_ = true;
                                phi_12881_ = false;
                                phi_12882_ = false;
                                break;
                            }
                            case 2: {
                                phi_12880_ = false;
                                phi_12881_ = true;
                                phi_12882_ = false;
                                break;
                            }
                            case 3: {
                                phi_12880_ = false;
                                phi_12881_ = true;
                                phi_12882_ = false;
                                break;
                            }
                            case 4: {
                                phi_12880_ = false;
                                phi_12881_ = false;
                                phi_12882_ = true;
                                break;
                            }
                            case 5: {
                                phi_12880_ = false;
                                phi_12881_ = false;
                                phi_12882_ = true;
                                break;
                            }
                            default: {
                                phi_12880_ = bool();
                                phi_12881_ = bool();
                                phi_12882_ = bool();
                                break;
                            }
                        }
                        let _e743 = phi_12880_;
                        let _e745 = phi_12881_;
                        let _e747 = phi_12882_;
                        let _e748 = select(_e745, false, _e743);
                        let _e752 = select(select(80f, 32f, _e748), 24f, select(select(_e747, false, _e743), false, _e748));
                        let _e754 = (_e726 + (_e752 + 8f));
                        let _e757 = ((_e754 - 8f) - (_e752 * 0.5f));
                        phi_4282_ = _e757;
                        phi_4283_ = _e757;
                        phi_4284_ = _e754;
                        phi_4285_ = select(true, false, (_e719 < (_e754 - 4f)));
                    } else {
                        phi_4282_ = f32();
                        phi_4283_ = _e724;
                        phi_4284_ = _e726;
                        phi_4285_ = true;
                    }
                    let _e762 = phi_4282_;
                    let _e764 = phi_4283_;
                    let _e766 = phi_4284_;
                    let _e768 = phi_4285_;
                    if _e768 {
                        phi_4290_ = (_e722 + 1u);
                    } else {
                        phi_4290_ = u32();
                    }
                    let _e771 = phi_4290_;
                    phi_4229_ = _e771;
                    phi_4232_ = _e764;
                    phi_4234_ = _e766;
                    phi_4291_ = _e729;
                    phi_4292_ = _e762;
                    phi_4293_ = _e768;
                } else {
                    phi_4229_ = u32();
                    phi_4232_ = f32();
                    phi_4234_ = f32();
                    phi_4291_ = u32();
                    phi_4292_ = f32();
                    phi_4293_ = false;
                }
                let _e773 = phi_4229_;
                let _e775 = phi_4232_;
                let _e777 = phi_4234_;
                let _e779 = phi_4291_;
                let _e781 = phi_4292_;
                let _e783 = phi_4293_;
                continue;
                continuing {
                    phi_4228_ = _e773;
                    phi_4231_ = _e775;
                    phi_4233_ = _e777;
                    phi_15213_ = _e431;
                    phi_11710_ = select(true, false, _e727);
                    phi_11709_ = _e781;
                    phi_11707_ = _e779;
                    break if !(_e783);
                }
            }
            let _e787 = phi_15213_;
            let _e789 = phi_11710_;
            let _e791 = phi_11709_;
            let _e793 = phi_11707_;
            if _e787 {
                break;
            }
            if _e789 {
                let _e795 = local_33;
                phi_4299_ = render_track_PaletteColor(5u, _e795);
            } else {
                phi_4299_ = render_track_PaletteColor();
            }
            let _e798 = phi_4299_;
            if select(true, false, _e789) {
                phi_4305_ = render_track_PaletteColor(_e793, _e791);
            } else {
                phi_4305_ = _e798;
            }
            let _e802 = phi_4305_;
            let _e805 = (_e719 - _e802.weight);
            let _e806 = ((_e537 * _e293) - _e301);
            switch bitcast<i32>(_e802.rgb) {
                case 0: {
                    phi_4316_ = vec2<f32>();
                    phi_4317_ = true;
                    break;
                }
                case 1: {
                    phi_4316_ = vec2<f32>();
                    phi_4317_ = true;
                    break;
                }
                default: {
                    phi_4316_ = vec2<f32>(0f, 0f);
                    phi_4317_ = false;
                    break;
                }
            }
            let _e809 = phi_4316_;
            let _e811 = phi_4317_;
            if _e811 {
                if _e787 {
                    break;
                }
                let _e812 = (_e719 - 52f);
                let _e817 = pill_1.member[_e229].cpu.temperature;
                if (_e817 <= 62f) {
                    phi_4483_ = vec2<f32>(0f, 0f);
                } else {
                    let _e820 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e812, _e806), 13f, 13f);
                    phi_12970_ = 0i;
                    phi_12971_ = 0.5f;
                    phi_12972_ = 0f;
                    phi_12973_ = vec2<f32>(((_e812 + (_e549 * 1.8f)) * 0.035f), (((_e806 + -(_e549)) * 0.035f) + 6.1f));
                    loop {
                        let _e830 = phi_12970_;
                        let _e832 = phi_12971_;
                        let _e834 = phi_12972_;
                        let _e836 = phi_12973_;
                        local_34 = _e834;
                        let _e837 = (_e830 < 4i);
                        if _e837 {
                            let _e840 = cantus_render_shader_simplex_noise(_e836);
                            phi_12998_ = (_e830 + 1i);
                            phi_12999_ = (_e832 * 0.5f);
                            phi_13000_ = (_e834 + (_e840 * _e832));
                            phi_13001_ = vec2<f32>(((_e836.x * 1.6f) + (_e836.y * 1.2f)), ((_e836.y * 1.6f) - (_e836.x * 1.2f)));
                        } else {
                            phi_12998_ = i32();
                            phi_12999_ = f32();
                            phi_13000_ = f32();
                            phi_13001_ = vec2<f32>();
                        }
                        let _e853 = phi_12998_;
                        let _e855 = phi_12999_;
                        let _e857 = phi_13000_;
                        let _e859 = phi_13001_;
                        continue;
                        continuing {
                            phi_12970_ = _e853;
                            phi_12971_ = _e855;
                            phi_12972_ = _e857;
                            phi_12973_ = _e859;
                            break if !(_e837);
                        }
                    }
                    let _e862 = local_34;
                    let _e863 = (_e862 * 0.5f);
                    let _e866 = ((_e820 - -0.5f) * 0.5f);
                    let _e868 = select(_e866, 0f, (_e866 < 0f));
                    let _e870 = select(_e868, 1f, (_e868 > 1f));
                    let _e876 = ((_e820 - 14f) * -0.083333336f);
                    let _e878 = select(_e876, 0f, (_e876 < 0f));
                    let _e880 = select(_e878, 1f, (_e878 > 1f));
                    let _e885 = (((_e870 * _e870) * (3f - (2f * _e870))) * ((_e880 * _e880) * (3f - (2f * _e880))));
                    let _e890 = ((_e863 + 0.19999999f) * 3.125f);
                    let _e892 = select(_e890, 0f, (_e890 < 0f));
                    let _e894 = select(_e892, 1f, (_e892 > 1f));
                    let _e901 = ((_e817 - 62f) * 0.045454547f);
                    let _e903 = select(_e901, 0f, (_e901 < 0f));
                    let _e905 = select(_e903, 1f, (_e903 > 1f));
                    let _e909 = ((_e905 * _e905) * (3f - (2f * _e905)));
                    phi_4483_ = vec2<f32>(((_e885 * (0.18f + ((0.5f + _e863) * 0.34f))) * _e909), ((_e885 * ((_e894 * _e894) * (3f - (2f * _e894)))) * _e909));
                }
                let _e914 = phi_4483_;
                phi_4487_ = 0u;
                phi_4490_ = 12f;
                loop {
                    let _e918 = phi_4487_;
                    let _e920 = phi_4490_;
                    local_35 = _e920;
                    let _e921 = (_e918 < 1u);
                    if _e921 {
                        local_27 = array<u32, 6>(0u, 1u, 2u, 3u, 4u, 5u);
                        if (_e918 < 6u) {
                        } else {
                            phi_15268_ = true;
                            break;
                        }
                        let _e924 = local_27[_e918];
                        let _e928 = pill_1.member[_e229].battery_level;
                        if (_e928 >= -1f) {
                            phi_4514_ = (_e928 <= 1f);
                        } else {
                            phi_4514_ = false;
                        }
                        let _e932 = phi_4514_;
                        if _e932 {
                            phi_4523_ = true;
                        } else {
                            phi_4523_ = select(true, false, (_e924 == 2u));
                        }
                        let _e936 = phi_4523_;
                        if _e936 {
                            switch bitcast<i32>(_e924) {
                                case 0: {
                                    phi_13016_ = true;
                                    phi_13017_ = false;
                                    phi_13018_ = false;
                                    break;
                                }
                                case 1: {
                                    phi_13016_ = true;
                                    phi_13017_ = false;
                                    phi_13018_ = false;
                                    break;
                                }
                                case 2: {
                                    phi_13016_ = false;
                                    phi_13017_ = true;
                                    phi_13018_ = false;
                                    break;
                                }
                                case 3: {
                                    phi_13016_ = false;
                                    phi_13017_ = true;
                                    phi_13018_ = false;
                                    break;
                                }
                                case 4: {
                                    phi_13016_ = false;
                                    phi_13017_ = false;
                                    phi_13018_ = true;
                                    break;
                                }
                                case 5: {
                                    phi_13016_ = false;
                                    phi_13017_ = false;
                                    phi_13018_ = true;
                                    break;
                                }
                                default: {
                                    phi_13016_ = bool();
                                    phi_13017_ = bool();
                                    phi_13018_ = bool();
                                    break;
                                }
                            }
                            let _e939 = phi_13016_;
                            let _e941 = phi_13017_;
                            let _e943 = phi_13018_;
                            let _e944 = select(_e941, false, _e939);
                            phi_4530_ = (_e920 + (select(select(80f, 32f, _e944), 24f, select(select(_e943, false, _e939), false, _e944)) + 8f));
                        } else {
                            phi_4530_ = _e920;
                        }
                        let _e952 = phi_4530_;
                        phi_4488_ = (_e918 + 1u);
                        phi_4491_ = _e952;
                    } else {
                        phi_4488_ = u32();
                        phi_4491_ = f32();
                    }
                    let _e955 = phi_4488_;
                    let _e957 = phi_4491_;
                    continue;
                    continuing {
                        phi_4487_ = _e955;
                        phi_4490_ = _e957;
                        phi_15268_ = _e787;
                        break if !(_e921);
                    }
                }
                let _e960 = phi_15268_;
                if _e960 {
                    break;
                }
                let _e962 = local_35;
                let _e964 = (_e719 - (_e962 + 40f));
                let _e969 = pill_1.member[_e229].gpu.temperature;
                if (_e969 <= 62f) {
                    phi_4648_ = vec2<f32>(0f, 0f);
                } else {
                    let _e972 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e964, _e806), 13f, 13f);
                    phi_13079_ = 0i;
                    phi_13080_ = 0.5f;
                    phi_13081_ = 0f;
                    phi_13082_ = vec2<f32>(((_e964 + (_e549 * 1.8f)) * 0.035f), (((_e806 + -(_e549)) * 0.035f) + 6.1f));
                    loop {
                        let _e982 = phi_13079_;
                        let _e984 = phi_13080_;
                        let _e986 = phi_13081_;
                        let _e988 = phi_13082_;
                        local_36 = _e986;
                        let _e989 = (_e982 < 4i);
                        if _e989 {
                            let _e992 = cantus_render_shader_simplex_noise(_e988);
                            phi_13107_ = (_e982 + 1i);
                            phi_13108_ = (_e984 * 0.5f);
                            phi_13109_ = (_e986 + (_e992 * _e984));
                            phi_13110_ = vec2<f32>(((_e988.x * 1.6f) + (_e988.y * 1.2f)), ((_e988.y * 1.6f) - (_e988.x * 1.2f)));
                        } else {
                            phi_13107_ = i32();
                            phi_13108_ = f32();
                            phi_13109_ = f32();
                            phi_13110_ = vec2<f32>();
                        }
                        let _e1005 = phi_13107_;
                        let _e1007 = phi_13108_;
                        let _e1009 = phi_13109_;
                        let _e1011 = phi_13110_;
                        continue;
                        continuing {
                            phi_13079_ = _e1005;
                            phi_13080_ = _e1007;
                            phi_13081_ = _e1009;
                            phi_13082_ = _e1011;
                            break if !(_e989);
                        }
                    }
                    let _e1014 = local_36;
                    let _e1015 = (_e1014 * 0.5f);
                    let _e1018 = ((_e972 - -0.5f) * 0.5f);
                    let _e1020 = select(_e1018, 0f, (_e1018 < 0f));
                    let _e1022 = select(_e1020, 1f, (_e1020 > 1f));
                    let _e1028 = ((_e972 - 14f) * -0.083333336f);
                    let _e1030 = select(_e1028, 0f, (_e1028 < 0f));
                    let _e1032 = select(_e1030, 1f, (_e1030 > 1f));
                    let _e1037 = (((_e1022 * _e1022) * (3f - (2f * _e1022))) * ((_e1032 * _e1032) * (3f - (2f * _e1032))));
                    let _e1042 = ((_e1015 + 0.19999999f) * 3.125f);
                    let _e1044 = select(_e1042, 0f, (_e1042 < 0f));
                    let _e1046 = select(_e1044, 1f, (_e1044 > 1f));
                    let _e1053 = ((_e969 - 62f) * 0.045454547f);
                    let _e1055 = select(_e1053, 0f, (_e1053 < 0f));
                    let _e1057 = select(_e1055, 1f, (_e1055 > 1f));
                    let _e1061 = ((_e1057 * _e1057) * (3f - (2f * _e1057)));
                    phi_4648_ = vec2<f32>(((_e1037 * (0.18f + ((0.5f + _e1015) * 0.34f))) * _e1061), ((_e1037 * ((_e1046 * _e1046) * (3f - (2f * _e1046)))) * _e1061));
                }
                let _e1066 = phi_4648_;
                phi_15301_ = _e960;
                phi_4662_ = vec2<f32>(select(_e1066.x, _e914.x, (_e914.x > _e1066.x)), select(_e1066.y, _e914.y, (_e914.y > _e1066.y)));
            } else {
                phi_15301_ = _e787;
                phi_4662_ = _e809;
            }
            let _e1075 = phi_15301_;
            let _e1077 = phi_4662_;
            let _e1082 = pill_1.member[_e229].cpu.temperature;
            let _e1087 = pill_1.member[_e229].gpu.temperature;
            if (_e1082 != _e1082) {
                phi_13125_ = true;
            } else {
                phi_13125_ = (_e1087 >= _e1082);
            }
            let _e1091 = phi_13125_;
            let _e1092 = select(_e1082, _e1087, _e1091);
            let _e1094 = ((_e1092 - 60f) * 0.083333336f);
            let _e1096 = select(_e1094, 0f, (_e1094 < 0f));
            let _e1098 = select(_e1096, 1f, (_e1096 > 1f));
            let _e1102 = ((_e1098 * _e1098) * (3f - (2f * _e1098)));
            let _e1103 = (1f - _e1102);
            let _e1112 = ((_e1092 - 72f) * 0.0625f);
            let _e1114 = select(_e1112, 0f, (_e1112 < 0f));
            let _e1116 = select(_e1114, 1f, (_e1114 > 1f));
            let _e1120 = ((_e1116 * _e1116) * (3f - (2f * _e1116)));
            let _e1121 = (1f - _e1120);
            let _e1131 = (_e1077.y * 0.12f);
            let _e1132 = (0.24f + _e1131);
            let _e1133 = (0.76f - _e1131);
            let _e1145 = (1f - (_e1077.x * 0.46f));
            let _e1155 = (_e1077.y * 0.64f);
            let _e1156 = (1f - _e1155);
            let _e1163 = (((((((((((((((((((0.008f * _e588) + (0.03f * _e565)) * _e586) + (((0.09f * _e588) + (0.34f * _e565)) * _e575)) * _e624) + ((_e600 + (0.3f * _e565)) * _e587)) * _e639) + (0.16f * _e638)) * _e650) + _e654) * _e659) + (_e545.fog * 0.3844f)) * _e683) + (_e681 * 0.2925f)) + _e715) * _e1145) + (_e1077.x * 0.0009200001f)) * _e1156) + (((0.07f * _e1133) + (((((0.22f * _e1103) + _e1102) * _e1121) + _e1120) * _e1132)) * _e1155));
            let _e1164 = (((((((((((((((((((0.015f * _e588) + (0.06f * _e565)) * _e586) + (((0.37f * _e588) + (0.7f * _e565)) * _e575)) * _e624) + (((0.25f * _e588) + (0.2f * _e565)) * _e587)) * _e639) + (0.2f * _e638)) * _e650) + _e654) * _e659) + (_e545.fog * 0.4216f)) * _e683) + (_e681 * 0.333f)) + _e715) * _e1145) + (_e1077.x * 0.00276f)) * _e1156) + (((0.12f * _e1133) + (((((0.62f * _e1103) + (0.38f * _e1102)) * _e1121) + (0.08f * _e1120)) * _e1132)) * _e1155));
            let _e1165 = (((((((((((((((((((0.04f * _e588) + (0.13f * _e565)) * _e586) + ((_e600 + (0.9f * _e565)) * _e575)) * _e624) + (((0.2f * _e588) + (0.4f * _e565)) * _e587)) * _e639) + (0.27f * _e638)) * _e650) + _e654) * _e659) + (_e545.fog * 0.44640002f)) * _e683) + (_e681 * 0.43199998f)) + _e715) * _e1145) + (_e1077.x * 0.00552f)) * _e1156) + (((0.18f * _e1133) + ((((_e1103 + (0.08f * _e1102)) * _e1121) + (0.035f * _e1120)) * _e1132)) * _e1155));
            switch bitcast<i32>(_e802.rgb) {
                case 0: {
                    let _e1891 = pill_1.member[_e229].history_scroll;
                    switch bitcast<i32>(_e802.rgb) {
                        case 0: {
                            phi_13212_ = true;
                            phi_13213_ = false;
                            phi_13214_ = false;
                            break;
                        }
                        case 1: {
                            phi_13212_ = true;
                            phi_13213_ = false;
                            phi_13214_ = false;
                            break;
                        }
                        case 2: {
                            phi_13212_ = false;
                            phi_13213_ = true;
                            phi_13214_ = false;
                            break;
                        }
                        case 3: {
                            phi_13212_ = false;
                            phi_13213_ = true;
                            phi_13214_ = false;
                            break;
                        }
                        case 4: {
                            phi_13212_ = false;
                            phi_13213_ = false;
                            phi_13214_ = true;
                            break;
                        }
                        case 5: {
                            phi_13212_ = false;
                            phi_13213_ = false;
                            phi_13214_ = true;
                            break;
                        }
                        default: {
                            phi_13212_ = bool();
                            phi_13213_ = bool();
                            phi_13214_ = bool();
                            break;
                        }
                    }
                    let _e1894 = phi_13212_;
                    let _e1896 = phi_13213_;
                    let _e1898 = phi_13214_;
                    let _e1899 = select(_e1896, false, _e1894);
                    let _e1905 = ((select(select(80f, 32f, _e1899), 24f, select(select(_e1898, false, _e1894), false, _e1899)) * 0.5f) - 4f);
                    let _e1906 = (_e301 - 8f);
                    let _e1907 = (_e1905 - _e1906);
                    let _e1909 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e805, _e806), _e1907, _e1906);
                    let _e1910 = abs(_e805);
                    let _e1911 = abs(_e806);
                    let _e1914 = (round((_e1910 * 0.11111111f)) * 9f);
                    if (_e1914 != _e1914) {
                        phi_13318_ = true;
                    } else {
                        phi_13318_ = (_e1905 <= _e1914);
                    }
                    let _e1918 = phi_13318_;
                    let _e1919 = select(_e1914, _e1905, _e1918);
                    let _e1920 = (_e1919 - _e1907);
                    if (_e1920 != _e1920) {
                        phi_13333_ = true;
                    } else {
                        phi_13333_ = (0f >= _e1920);
                    }
                    let _e1924 = phi_13333_;
                    let _e1925 = select(_e1920, 0f, _e1924);
                    let _e1926 = (_e1906 * _e1906);
                    let _e1929 = sqrt((_e1926 - (_e1925 * _e1925)));
                    let _e1930 = (_e1925 / _e1906);
                    let _e1931 = (_e1929 / _e1906);
                    let _e1936 = ((_e1910 - _e1919) - (_e1930 * 0.9f));
                    let _e1937 = ((_e1911 - _e1929) - (_e1931 * 0.9f));
                    let _e1946 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1936 * -(_e1931)) + (_e1937 * _e1930)), ((_e1936 * _e1930) + (_e1937 * _e1931))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    let _e1948 = round((_e1911 * 0.125f));
                    if (_e1948 != _e1948) {
                        phi_13348_ = true;
                    } else {
                        phi_13348_ = (1f <= _e1948);
                    }
                    let _e1952 = phi_13348_;
                    let _e1954 = (select(_e1948, 1f, _e1952) * 8f);
                    let _e1957 = sqrt((_e1926 - (_e1954 * _e1954)));
                    let _e1959 = (_e1957 / _e1906);
                    let _e1960 = (_e1954 / _e1906);
                    let _e1965 = ((_e1910 - (_e1907 + _e1957)) - (_e1959 * 0.9f));
                    let _e1966 = ((_e1911 - _e1954) - (_e1960 * 0.9f));
                    let _e1975 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e1965 * -(_e1960)) + (_e1966 * _e1959)), ((_e1965 * _e1959) + (_e1966 * _e1960))), vec2<f32>(1.55f, 2.05f), 0.65f);
                    if (_e1946 != _e1946) {
                        phi_13363_ = true;
                    } else {
                        phi_13363_ = (_e1975 <= _e1946);
                    }
                    let _e1979 = phi_13363_;
                    let _e1980 = select(_e1946, _e1975, _e1979);
                    let _e1983 = (0.5f + ((_e1980 - _e1909) * 0.3125f));
                    let _e1985 = select(_e1983, 0f, (_e1983 < 0f));
                    let _e1987 = select(_e1985, 1f, (_e1985 > 1f));
                    let _e1996 = ((_e1909 - 0.55f) * -0.9090909f);
                    let _e1998 = select(_e1996, 0f, (_e1996 < 0f));
                    let _e2000 = select(_e1998, 1f, (_e1998 > 1f));
                    let _e2004 = ((_e2000 * _e2000) * (3f - (2f * _e2000)));
                    let _e2005 = (_e1905 * 0.051282052f);
                    let _e2006 = (_e805 + _e1905);
                    let _e2008 = ((_e2006 / _e2005) + _e1891);
                    let _e2010 = select(_e2008, 0f, (_e2008 < 0f));
                    let _e2012 = select(_e2010, 39f, (_e2010 > 39f));
                    let _e2013 = floor(_e2012);
                    let _e2018 = select(select(u32(_e2013), 0u, (_e2013 < 0f)), 4294967295u, (_e2013 > 4294967000f));
                    let _e2019 = (_e301 - 10f);
                    let _e2023 = (((f32(_e2018) - _e1891) * _e2005) - _e1905);
                    let _e2025 = select(_e2018, 39u, (39u < _e2018));
                    let _e2026 = (_e2025 < 40u);
                    if _e2026 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e2033 = pill_1.member[_e229].cpu.usage.samples[_e2025];
                    let _e2036 = (_e2019 * (1f - (_e2033 * 2f)));
                    let _e2037 = (_e2018 + 1u);
                    let _e2043 = select(_e2037, 39u, (39u < _e2037));
                    let _e2044 = (_e2043 < 40u);
                    if _e2044 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e2051 = pill_1.member[_e229].cpu.usage.samples[_e2043];
                    let _e2055 = ((((f32(_e2037) - _e1891) * _e2005) - _e1905) - _e2023);
                    let _e2056 = ((_e2019 * (1f - (_e2051 * 2f))) - _e2036);
                    let _e2057 = (_e805 - _e2023);
                    let _e2058 = (_e806 - _e2036);
                    let _e2059 = (_e2057 * _e2055);
                    let _e2062 = (_e2055 * _e2055);
                    let _e2064 = (_e2062 + (_e2056 * _e2056));
                    if (_e2064 != _e2064) {
                        phi_13378_ = true;
                    } else {
                        phi_13378_ = (0.001f >= _e2064);
                    }
                    let _e2068 = phi_13378_;
                    let _e2070 = ((_e2059 + (_e2058 * _e2056)) / select(_e2064, 0.001f, _e2068));
                    let _e2072 = select(_e2070, 0f, (_e2070 < 0f));
                    let _e2074 = select(_e2072, 1f, (_e2072 > 1f));
                    let _e2077 = (_e2057 - (_e2055 * _e2074));
                    let _e2078 = (_e2058 - (_e2056 * _e2074));
                    let _e2085 = ((abs(sqrt(((_e2077 * _e2077) + (_e2078 * _e2078)))) - 1.4000001f) * -0.9090908f);
                    let _e2087 = select(_e2085, 0f, (_e2085 < 0f));
                    let _e2089 = select(_e2087, 1f, (_e2087 > 1f));
                    let _e2095 = (_e2012 - trunc(_e2012));
                    let _e2097 = select(_e2095, 0f, (_e2095 < 0f));
                    let _e2099 = select(_e2097, 1f, (_e2097 > 1f));
                    let _e2103 = ((_e2099 * _e2099) * (3f - (2f * _e2099)));
                    let _e2110 = ((((_e2036 + (_e2056 * _e2103)) - _e806) - 0.55f) * -0.9090909f);
                    let _e2112 = select(_e2110, 0f, (_e2110 < 0f));
                    let _e2114 = select(_e2112, 1f, (_e2112 > 1f));
                    let _e2120 = ((((_e2114 * _e2114) * (3f - (2f * _e2114))) * 0.156f) + ((_e2089 * _e2089) * (3f - (2f * _e2089))));
                    if _e2026 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e2129 = pill_1.member[_e229].cpu.memory.samples[_e2025];
                    let _e2132 = (_e2019 * (1f - (_e2129 * 2f)));
                    if _e2044 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e2139 = pill_1.member[_e229].cpu.memory.samples[_e2043];
                    let _e2143 = ((_e2019 * (1f - (_e2139 * 2f))) - _e2132);
                    let _e2144 = (_e806 - _e2132);
                    let _e2148 = (_e2062 + (_e2143 * _e2143));
                    if (_e2148 != _e2148) {
                        phi_13393_ = true;
                    } else {
                        phi_13393_ = (0.001f >= _e2148);
                    }
                    let _e2152 = phi_13393_;
                    let _e2154 = ((_e2059 + (_e2144 * _e2143)) / select(_e2148, 0.001f, _e2152));
                    let _e2156 = select(_e2154, 0f, (_e2154 < 0f));
                    let _e2158 = select(_e2156, 1f, (_e2156 > 1f));
                    let _e2161 = (_e2057 - (_e2055 * _e2158));
                    let _e2162 = (_e2144 - (_e2143 * _e2158));
                    let _e2169 = ((abs(sqrt(((_e2161 * _e2161) + (_e2162 * _e2162)))) - 1.4000001f) * -0.9090908f);
                    let _e2171 = select(_e2169, 0f, (_e2169 < 0f));
                    let _e2173 = select(_e2171, 1f, (_e2171 > 1f));
                    let _e2184 = ((((_e2132 + (_e2143 * _e2103)) - _e806) - 0.55f) * -0.9090909f);
                    let _e2186 = select(_e2184, 0f, (_e2184 < 0f));
                    let _e2188 = select(_e2186, 1f, (_e2186 > 1f));
                    let _e2194 = ((((_e2188 * _e2188) * (3f - (2f * _e2188))) * 0.084f) + ((_e2173 * _e2173) * (3f - (2f * _e2173))));
                    let _e2202 = (_e2006 * 0.14285715f);
                    let _e2203 = ((_e806 + _e1906) * 0.16393442f);
                    let _e2213 = ((abs(((_e2202 - trunc(_e2202)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e2215 = select(_e2213, 0f, (_e2213 < 0f));
                    let _e2217 = select(_e2215, 1f, (_e2215 > 1f));
                    let _e2221 = ((_e2217 * _e2217) * (3f - (2f * _e2217)));
                    let _e2223 = ((abs(((_e2203 - trunc(_e2203)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e2225 = select(_e2223, 0f, (_e2223 < 0f));
                    let _e2227 = select(_e2225, 1f, (_e2225 > 1f));
                    let _e2231 = ((_e2227 * _e2227) * (3f - (2f * _e2227)));
                    if (_e2221 != _e2221) {
                        phi_13408_ = true;
                    } else {
                        phi_13408_ = (_e2231 >= _e2221);
                    }
                    let _e2235 = phi_13408_;
                    let _e2243 = pill_1.member[_e229].cpu.usage.samples[39u];
                    let _e2244 = (_e2243 * 0.24f);
                    let _e2245 = (0.18f + _e2244);
                    let _e2246 = (0.82f - _e2244);
                    let _e2255 = (_e1082 - 60f);
                    let _e2256 = (_e2255 * 0.083333336f);
                    let _e2258 = select(_e2256, 0f, (_e2256 < 0f));
                    let _e2260 = select(_e2258, 1f, (_e2258 > 1f));
                    let _e2264 = ((_e2260 * _e2260) * (3f - (2f * _e2260)));
                    let _e2265 = (1f - _e2264);
                    let _e2274 = ((_e1082 - 72f) * 0.0625f);
                    let _e2276 = select(_e2274, 0f, (_e2274 < 0f));
                    let _e2278 = select(_e2276, 1f, (_e2276 > 1f));
                    let _e2282 = ((_e2278 * _e2278) * (3f - (2f * _e2278)));
                    let _e2283 = (1f - _e2282);
                    let _e2292 = (_e2255 * 0.03846154f);
                    let _e2294 = select(_e2292, 0f, (_e2292 < 0f));
                    let _e2296 = select(_e2294, 1f, (_e2294 > 1f));
                    let _e2301 = (((_e2296 * _e2296) * (3f - (2f * _e2296))) * 0.9f);
                    let _e2302 = (1f - _e2301);
                    let _e2309 = ((((0.025f * _e2246) + (0.32f * _e2245)) * _e2302) + (((((0.22f * _e2265) + _e2264) * _e2283) + _e2282) * _e2301));
                    let _e2310 = ((((0.09f * _e2246) + (0.68f * _e2245)) * _e2302) + (((((0.62f * _e2265) + (0.38f * _e2264)) * _e2283) + (0.08f * _e2282)) * _e2301));
                    let _e2311 = ((((0.15f * _e2246) + _e2245) * _e2302) + ((((_e2265 + (0.08f * _e2264)) * _e2283) + (0.035f * _e2282)) * _e2301));
                    let _e2313 = ((((_e1980 + ((_e1909 - _e1980) * _e1987)) - ((1.6f * _e1987) * (1f - _e1987))) - 0.55f) * -0.9090909f);
                    let _e2315 = select(_e2313, 0f, (_e2313 < 0f));
                    let _e2317 = select(_e2315, 1f, (_e2315 > 1f));
                    let _e2321 = ((_e2317 * _e2317) * (3f - (2f * _e2317)));
                    let _e2323 = (1f - (_e2321 * 0.82f));
                    let _e2335 = ((abs(_e1909) - 2.1f) * -0.909091f);
                    let _e2337 = select(_e2335, 0f, (_e2335 < 0f));
                    let _e2339 = select(_e2337, 1f, (_e2337 > 1f));
                    let _e2344 = (((_e2339 * _e2339) * (3f - (2f * _e2339))) * 0.92f);
                    let _e2345 = (1f - _e2344);
                    let _e2356 = ((_e1980 - 0.55f) * -0.9090909f);
                    let _e2358 = select(_e2356, 0f, (_e2356 < 0f));
                    let _e2360 = select(_e2358, 1f, (_e2358 > 1f));
                    let _e2365 = (((_e2360 * _e2360) * (3f - (2f * _e2360))) * 0.78f);
                    let _e2366 = (1f - _e2365);
                    let _e2377 = ((_e2004 * select(_e2221, _e2231, _e2235)) * 0.045f);
                    phi_15294_ = _e1075;
                    phi_6759_ = vec3<f32>(((((((((_e1163 * _e2323) + (_e2321 * 0.00328f)) * _e2345) + (_e2309 * _e2344)) * _e2366) + (_e2309 * _e2365)) + _e2377) + (((0.32f * _e2004) * _e2120) + ((0.78f * _e2004) * _e2194))), ((((((((_e1164 * _e2323) + (_e2321 * 0.00984f)) * _e2345) + (_e2310 * _e2344)) * _e2366) + (_e2310 * _e2365)) + _e2377) + (((0.68f * _e2004) * _e2120) + ((0.3f * _e2004) * _e2194))), ((((((((_e1165 * _e2323) + (_e2321 * 0.02132f)) * _e2345) + (_e2311 * _e2344)) * _e2366) + (_e2311 * _e2365)) + _e2377) + (_e2004 * (_e2120 + _e2194))));
                    phi_6760_ = false;
                    break;
                }
                case 1: {
                    let _e1510 = pill_1.member[_e229].history_scroll;
                    switch bitcast<i32>(_e802.rgb) {
                        case 0: {
                            phi_13140_ = true;
                            phi_13141_ = false;
                            phi_13142_ = false;
                            break;
                        }
                        case 1: {
                            phi_13140_ = true;
                            phi_13141_ = false;
                            phi_13142_ = false;
                            break;
                        }
                        case 2: {
                            phi_13140_ = false;
                            phi_13141_ = true;
                            phi_13142_ = false;
                            break;
                        }
                        case 3: {
                            phi_13140_ = false;
                            phi_13141_ = true;
                            phi_13142_ = false;
                            break;
                        }
                        case 4: {
                            phi_13140_ = false;
                            phi_13141_ = false;
                            phi_13142_ = true;
                            break;
                        }
                        case 5: {
                            phi_13140_ = false;
                            phi_13141_ = false;
                            phi_13142_ = true;
                            break;
                        }
                        default: {
                            phi_13140_ = bool();
                            phi_13141_ = bool();
                            phi_13142_ = bool();
                            break;
                        }
                    }
                    let _e1513 = phi_13140_;
                    let _e1515 = phi_13141_;
                    let _e1517 = phi_13142_;
                    let _e1518 = select(_e1515, false, _e1513);
                    let _e1524 = ((select(select(80f, 32f, _e1518), 24f, select(select(_e1517, false, _e1513), false, _e1518)) * 0.5f) - 4f);
                    let _e1525 = (_e301 - 8f);
                    let _e1528 = cantus_render_shader_sd_capsule_box(vec2<f32>(_e805, _e806), (_e1524 - _e1525), _e1525);
                    let _e1530 = ((_e1528 - 0.55f) * -0.9090909f);
                    let _e1532 = select(_e1530, 0f, (_e1530 < 0f));
                    let _e1534 = select(_e1532, 1f, (_e1532 > 1f));
                    let _e1538 = ((_e1534 * _e1534) * (3f - (2f * _e1534)));
                    let _e1539 = (_e1524 * 0.051282052f);
                    let _e1540 = (_e805 + _e1524);
                    let _e1542 = ((_e1540 / _e1539) + _e1510);
                    let _e1544 = select(_e1542, 0f, (_e1542 < 0f));
                    let _e1546 = select(_e1544, 39f, (_e1544 > 39f));
                    let _e1547 = floor(_e1546);
                    let _e1552 = select(select(u32(_e1547), 0u, (_e1547 < 0f)), 4294967295u, (_e1547 > 4294967000f));
                    let _e1553 = (_e301 - 10f);
                    let _e1557 = (((f32(_e1552) - _e1510) * _e1539) - _e1524);
                    let _e1559 = select(_e1552, 39u, (39u < _e1552));
                    let _e1560 = (_e1559 < 40u);
                    if _e1560 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e1567 = pill_1.member[_e229].gpu.usage.samples[_e1559];
                    let _e1570 = (_e1553 * (1f - (_e1567 * 2f)));
                    let _e1571 = (_e1552 + 1u);
                    let _e1577 = select(_e1571, 39u, (39u < _e1571));
                    let _e1578 = (_e1577 < 40u);
                    if _e1578 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e1585 = pill_1.member[_e229].gpu.usage.samples[_e1577];
                    let _e1589 = ((((f32(_e1571) - _e1510) * _e1539) - _e1524) - _e1557);
                    let _e1590 = ((_e1553 * (1f - (_e1585 * 2f))) - _e1570);
                    let _e1591 = (_e805 - _e1557);
                    let _e1592 = (_e806 - _e1570);
                    let _e1593 = (_e1591 * _e1589);
                    let _e1596 = (_e1589 * _e1589);
                    let _e1598 = (_e1596 + (_e1590 * _e1590));
                    if (_e1598 != _e1598) {
                        phi_13167_ = true;
                    } else {
                        phi_13167_ = (0.001f >= _e1598);
                    }
                    let _e1602 = phi_13167_;
                    let _e1604 = ((_e1593 + (_e1592 * _e1590)) / select(_e1598, 0.001f, _e1602));
                    let _e1606 = select(_e1604, 0f, (_e1604 < 0f));
                    let _e1608 = select(_e1606, 1f, (_e1606 > 1f));
                    let _e1611 = (_e1591 - (_e1589 * _e1608));
                    let _e1612 = (_e1592 - (_e1590 * _e1608));
                    let _e1619 = ((abs(sqrt(((_e1611 * _e1611) + (_e1612 * _e1612)))) - 1.4000001f) * -0.9090908f);
                    let _e1621 = select(_e1619, 0f, (_e1619 < 0f));
                    let _e1623 = select(_e1621, 1f, (_e1621 > 1f));
                    let _e1629 = (_e1546 - trunc(_e1546));
                    let _e1631 = select(_e1629, 0f, (_e1629 < 0f));
                    let _e1633 = select(_e1631, 1f, (_e1631 > 1f));
                    let _e1637 = ((_e1633 * _e1633) * (3f - (2f * _e1633)));
                    let _e1644 = ((((_e1570 + (_e1590 * _e1637)) - _e806) - 0.55f) * -0.9090909f);
                    let _e1646 = select(_e1644, 0f, (_e1644 < 0f));
                    let _e1648 = select(_e1646, 1f, (_e1646 > 1f));
                    let _e1654 = ((((_e1648 * _e1648) * (3f - (2f * _e1648))) * 0.156f) + ((_e1623 * _e1623) * (3f - (2f * _e1623))));
                    if _e1560 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e1663 = pill_1.member[_e229].gpu.memory.samples[_e1559];
                    let _e1666 = (_e1553 * (1f - (_e1663 * 2f)));
                    if _e1578 {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e1673 = pill_1.member[_e229].gpu.memory.samples[_e1577];
                    let _e1677 = ((_e1553 * (1f - (_e1673 * 2f))) - _e1666);
                    let _e1678 = (_e806 - _e1666);
                    let _e1682 = (_e1596 + (_e1677 * _e1677));
                    if (_e1682 != _e1682) {
                        phi_13182_ = true;
                    } else {
                        phi_13182_ = (0.001f >= _e1682);
                    }
                    let _e1686 = phi_13182_;
                    let _e1688 = ((_e1593 + (_e1678 * _e1677)) / select(_e1682, 0.001f, _e1686));
                    let _e1690 = select(_e1688, 0f, (_e1688 < 0f));
                    let _e1692 = select(_e1690, 1f, (_e1690 > 1f));
                    let _e1695 = (_e1591 - (_e1589 * _e1692));
                    let _e1696 = (_e1678 - (_e1677 * _e1692));
                    let _e1703 = ((abs(sqrt(((_e1695 * _e1695) + (_e1696 * _e1696)))) - 1.4000001f) * -0.9090908f);
                    let _e1705 = select(_e1703, 0f, (_e1703 < 0f));
                    let _e1707 = select(_e1705, 1f, (_e1705 > 1f));
                    let _e1718 = ((((_e1666 + (_e1677 * _e1637)) - _e806) - 0.55f) * -0.9090909f);
                    let _e1720 = select(_e1718, 0f, (_e1718 < 0f));
                    let _e1722 = select(_e1720, 1f, (_e1720 > 1f));
                    let _e1728 = ((((_e1722 * _e1722) * (3f - (2f * _e1722))) * 0.084f) + ((_e1707 * _e1707) * (3f - (2f * _e1707))));
                    let _e1736 = (_e1540 * 0.14285715f);
                    let _e1737 = ((_e806 + _e1525) * 0.16393442f);
                    let _e1747 = ((abs(((_e1736 - trunc(_e1736)) - 0.5f)) - 0.49f) * -33.333332f);
                    let _e1749 = select(_e1747, 0f, (_e1747 < 0f));
                    let _e1751 = select(_e1749, 1f, (_e1749 > 1f));
                    let _e1755 = ((_e1751 * _e1751) * (3f - (2f * _e1751)));
                    let _e1757 = ((abs(((_e1737 - trunc(_e1737)) - 0.5f)) - 0.49f) * -24.999987f);
                    let _e1759 = select(_e1757, 0f, (_e1757 < 0f));
                    let _e1761 = select(_e1759, 1f, (_e1759 > 1f));
                    let _e1765 = ((_e1761 * _e1761) * (3f - (2f * _e1761)));
                    if (_e1755 != _e1755) {
                        phi_13197_ = true;
                    } else {
                        phi_13197_ = (_e1765 >= _e1755);
                    }
                    let _e1769 = phi_13197_;
                    let _e1777 = pill_1.member[_e229].gpu.usage.samples[39u];
                    let _e1778 = (_e1777 * 0.24f);
                    let _e1779 = (0.18f + _e1778);
                    let _e1780 = (0.82f - _e1778);
                    let _e1789 = (_e1087 - 60f);
                    let _e1790 = (_e1789 * 0.083333336f);
                    let _e1792 = select(_e1790, 0f, (_e1790 < 0f));
                    let _e1794 = select(_e1792, 1f, (_e1792 > 1f));
                    let _e1798 = ((_e1794 * _e1794) * (3f - (2f * _e1794)));
                    let _e1799 = (1f - _e1798);
                    let _e1808 = ((_e1087 - 72f) * 0.0625f);
                    let _e1810 = select(_e1808, 0f, (_e1808 < 0f));
                    let _e1812 = select(_e1810, 1f, (_e1810 > 1f));
                    let _e1816 = ((_e1812 * _e1812) * (3f - (2f * _e1812)));
                    let _e1817 = (1f - _e1816);
                    let _e1826 = (_e1789 * 0.03846154f);
                    let _e1828 = select(_e1826, 0f, (_e1826 < 0f));
                    let _e1830 = select(_e1828, 1f, (_e1828 > 1f));
                    let _e1835 = (((_e1830 * _e1830) * (3f - (2f * _e1830))) * 0.9f);
                    let _e1836 = (1f - _e1835);
                    let _e1847 = (1f - (_e1538 * 0.82f));
                    let _e1859 = ((abs(_e1528) - 2.1f) * -0.909091f);
                    let _e1861 = select(_e1859, 0f, (_e1859 < 0f));
                    let _e1863 = select(_e1861, 1f, (_e1861 > 1f));
                    let _e1868 = (((_e1863 * _e1863) * (3f - (2f * _e1863))) * 0.92f);
                    let _e1869 = (1f - _e1868);
                    let _e1880 = ((_e1538 * select(_e1755, _e1765, _e1769)) * 0.045f);
                    phi_15294_ = _e1075;
                    phi_6759_ = vec3<f32>(((((((_e1163 * _e1847) + (_e1538 * 0.00328f)) * _e1869) + (((((0.025f * _e1780) + (0.32f * _e1779)) * _e1836) + (((((0.22f * _e1799) + _e1798) * _e1817) + _e1816) * _e1835)) * _e1868)) + _e1880) + (((0.32f * _e1538) * _e1654) + ((0.78f * _e1538) * _e1728))), ((((((_e1164 * _e1847) + (_e1538 * 0.00984f)) * _e1869) + (((((0.09f * _e1780) + (0.68f * _e1779)) * _e1836) + (((((0.62f * _e1799) + (0.38f * _e1798)) * _e1817) + (0.08f * _e1816)) * _e1835)) * _e1868)) + _e1880) + (((0.68f * _e1538) * _e1654) + ((0.3f * _e1538) * _e1728))), ((((((_e1165 * _e1847) + (_e1538 * 0.02132f)) * _e1869) + (((((0.15f * _e1780) + _e1779) * _e1836) + ((((_e1799 + (0.08f * _e1798)) * _e1817) + (0.035f * _e1816)) * _e1835)) * _e1868)) + _e1880) + (_e1538 * (_e1654 + _e1728))));
                    phi_6760_ = false;
                    break;
                }
                case 2: {
                    let _e1292 = (_e805 * 1.25f);
                    let _e1293 = (_e806 * 1.25f);
                    let _e1297 = pill_1.member[_e229].battery_level;
                    let _e1299 = select(0f, 1f, (_e1297 < 0f));
                    let _e1300 = abs(_e1297);
                    let _e1301 = (_e1293 - 1f);
                    let _e1302 = vec2<f32>(_e1292, _e1301);
                    let _e1303 = cantus_render_shader_sd_rounded_box(_e1302, vec2<f32>(11.5f, 15f), 3.2f);
                    let _e1306 = ((abs(_e1303) - 2.425f) * -0.909091f);
                    let _e1308 = select(_e1306, 0f, (_e1306 < 0f));
                    let _e1310 = select(_e1308, 1f, (_e1308 > 1f));
                    let _e1317 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e1292, (_e1293 - -15.6f)), vec2<f32>(4f, 1.8f), 0.8f);
                    let _e1319 = ((_e1317 - 0.55f) * -0.9090909f);
                    let _e1321 = select(_e1319, 0f, (_e1319 < 0f));
                    let _e1323 = select(_e1321, 1f, (_e1321 > 1f));
                    let _e1328 = cantus_render_shader_sd_rounded_box(_e1302, vec2<f32>(8.5f, 12f), 1.7f);
                    let _e1330 = ((_e1328 - 0.55f) * -0.9090909f);
                    let _e1332 = select(_e1330, 0f, (_e1330 < 0f));
                    let _e1334 = select(_e1332, 1f, (_e1332 > 1f));
                    let _e1340 = select(_e1300, 0f, (_e1300 < 0f));
                    let _e1358 = ((12f - (select(_e1340, 1f, (_e1340 > 1f)) * 24f)) + ((sin(((_e805 * 0.775f) + (_e549 * (1.4f + (_e1299 * 1.2f))))) * 1.15f) + (sin(((_e805 * 0.3375f) - (_e549 * 0.8f))) * 0.45f)));
                    let _e1359 = (_e1358 - 0.7f);
                    let _e1363 = ((_e1301 - _e1359) / ((_e1358 + 0.7f) - _e1359));
                    let _e1365 = select(_e1363, 0f, (_e1363 < 0f));
                    let _e1367 = select(_e1365, 1f, (_e1365 > 1f));
                    let _e1372 = (((_e1334 * _e1334) * (3f - (2f * _e1334))) * ((_e1367 * _e1367) * (3f - (2f * _e1367))));
                    let _e1374 = ((_e1300 - 0.08f) * 5f);
                    let _e1376 = select(_e1374, 0f, (_e1374 < 0f));
                    let _e1378 = select(_e1376, 1f, (_e1376 > 1f));
                    let _e1382 = ((_e1378 * _e1378) * (3f - (2f * _e1378)));
                    let _e1383 = (1f - _e1382);
                    let _e1391 = ((_e1300 - 0.18f) * 1.8518518f);
                    let _e1393 = select(_e1391, 0f, (_e1391 < 0f));
                    let _e1395 = select(_e1393, 1f, (_e1393 > 1f));
                    let _e1399 = ((_e1395 * _e1395) * (3f - (2f * _e1395)));
                    let _e1400 = (1f - _e1399);
                    let _e1406 = (_e1400 + (0.22f * _e1399));
                    let _e1407 = ((((0.18f * _e1383) + (0.72f * _e1382)) * _e1400) + (0.95f * _e1399));
                    let _e1408 = ((((0.1f * _e1383) + (0.12f * _e1382)) * _e1400) + (0.55f * _e1399));
                    let _e1411 = floor((_e805 * 0.4166667f));
                    let _e1412 = floor((_e806 * 0.36764705f));
                    let _e1414 = cantus_render_shader_hash(vec2<f32>(_e1411, _e1412));
                    let _e1428 = ((_e549 * (0.5f + _e1414.y)) + (_e1414.x * 11f));
                    let _e1430 = (_e1428 - trunc(_e1428));
                    let _e1431 = (_e1292 - (((_e1411 + 0.2f) + (_e1414.x * 0.6f)) * 3f));
                    let _e1434 = ((_e1293 - (((_e1412 + 0.2f) + (_e1414.y * 0.6f)) * 3.4f)) + (_e1430 * 5f));
                    let _e1442 = (_e1430 * 4f);
                    let _e1444 = select(_e1442, 0f, (_e1442 < 0f));
                    let _e1446 = select(_e1444, 1f, (_e1444 > 1f));
                    let _e1452 = ((_e1430 - 1f) * -3.3333333f);
                    let _e1454 = select(_e1452, 0f, (_e1452 < 0f));
                    let _e1456 = select(_e1454, 1f, (_e1454 > 1f));
                    let _e1464 = ((abs((sqrt(((_e1431 * _e1431) + (_e1434 * _e1434))) - (0.4f + (_e1414.y * 0.5f)))) - 1f) * -0.9090909f);
                    let _e1466 = select(_e1464, 0f, (_e1464 < 0f));
                    let _e1468 = select(_e1466, 1f, (_e1466 > 1f));
                    let _e1475 = (((((_e1468 * _e1468) * (3f - (2f * _e1468))) * (((_e1446 * _e1446) * (3f - (2f * _e1446))) * ((_e1456 * _e1456) * (3f - (2f * _e1456))))) * _e1372) * _e1299);
                    let _e1478 = ((((_e1310 * _e1310) * (3f - (2f * _e1310))) * 0.43f) + (((_e1323 * _e1323) * (3f - (2f * _e1323))) * 0.38f));
                    phi_15294_ = _e1075;
                    phi_6759_ = vec3<f32>((_e1163 + ((_e1478 + ((_e1406 * _e1372) * 0.78f)) + ((((_e1406 * 0.27999997f) + 0.72f) * _e1475) * 0.9f))), (_e1164 + ((_e1478 + ((_e1407 * _e1372) * 0.78f)) + ((((_e1407 * 0.27999997f) + 0.72f) * _e1475) * 0.9f))), (_e1165 + ((_e1478 + ((_e1408 * _e1372) * 0.78f)) + ((((_e1408 * 0.27999997f) + 0.72f) * _e1475) * 0.9f))));
                    phi_6760_ = false;
                    break;
                }
                case 3: {
                    let _e1170 = pill_1.member[_e229].volume;
                    let _e1172 = select(0f, 1f, (_e1170 < 0f));
                    let _e1173 = abs(_e1170);
                    let _e1176 = round(((_e805 + 12f) * 0.25f));
                    let _e1178 = select(_e1176, 0f, (_e1176 < 0f));
                    let _e1180 = select(_e1178, 6f, (_e1178 > 6f));
                    let _e1185 = select(select(u32(_e1180), 0u, (_e1180 < 0f)), 4294967295u, (_e1180 > 4294967000f));
                    if (_e1185 < 7u) {
                    } else {
                        phi_15294_ = true;
                        phi_6759_ = vec3<f32>();
                        phi_6760_ = bool();
                        break;
                    }
                    let _e1191 = pill_1.member[_e229].audio_spectrum[_e1185];
                    let _e1192 = (1f - _e1172);
                    let _e1193 = (_e1191 * _e1192);
                    let _e1202 = cantus_render_shader_sd_rounded_box(vec2<f32>((_e805 - (-12f + (_e1180 * 4f))), (_e806 - -1.5f)), vec2<f32>(1.25f, (1.2f + (7.7f * _e1193))), 1.25f);
                    let _e1205 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e805, (_e806 - 11.5f)), vec2<f32>(14f, 1.25f), 1.25f);
                    let _e1207 = ((_e1205 - 0.55f) * -0.9090909f);
                    let _e1209 = select(_e1207, 0f, (_e1207 < 0f));
                    let _e1211 = select(_e1209, 1f, (_e1209 > 1f));
                    let _e1215 = ((_e1211 * _e1211) * (3f - (2f * _e1211)));
                    let _e1217 = select(_e1173, 0f, (_e1173 < 0f));
                    let _e1220 = (select(_e1217, 1f, (_e1217 > 1f)) * 28f);
                    let _e1221 = (_e1220 + -13.2f);
                    let _e1225 = ((_e805 - _e1221) / ((_e1220 + -14.8f) - _e1221));
                    let _e1227 = select(_e1225, 0f, (_e1225 < 0f));
                    let _e1229 = select(_e1227, 1f, (_e1227 > 1f));
                    let _e1234 = (_e1215 * ((_e1229 * _e1229) * (3f - (2f * _e1229))));
                    let _e1236 = (1f - (_e1173 * 0.65f));
                    let _e1241 = ((0.08f * _e1236) + (_e1173 * 0.42249995f));
                    let _e1242 = ((0.88f * _e1236) + (_e1173 * 0.221f));
                    let _e1244 = ((_e1202 - 0.7f) * -0.71428573f);
                    let _e1246 = select(_e1244, 0f, (_e1244 < 0f));
                    let _e1248 = select(_e1246, 1f, (_e1246 > 1f));
                    let _e1257 = ((_e1202 - 3.2f) * -0.3125f);
                    let _e1259 = select(_e1257, 0f, (_e1257 < 0f));
                    let _e1261 = select(_e1259, 1f, (_e1259 > 1f));
                    let _e1268 = ((((_e1248 * _e1248) * (3f - (2f * _e1248))) * (0.58f + (_e1193 * 0.35f))) + ((((_e1261 * _e1261) * (3f - (2f * _e1261))) * _e1193) * 0.12f));
                    let _e1281 = (_e1234 + ((_e1215 * (1f - _e1234)) * 0.22f));
                    phi_15294_ = _e1075;
                    phi_6759_ = vec3<f32>((_e1163 + ((_e1241 * _e1268) + (((_e1241 * _e1192) + _e1172) * _e1281))), (_e1164 + ((_e1242 * _e1268) + (((_e1242 * _e1192) + (0.24f * _e1172)) * _e1281))), (_e1165 + (_e1268 + ((_e1192 + (0.3f * _e1172)) * _e1281))));
                    phi_6760_ = false;
                    break;
                }
                case 4: {
                    phi_15294_ = _e1075;
                    phi_6759_ = vec3<f32>();
                    phi_6760_ = true;
                    break;
                }
                case 5: {
                    phi_15294_ = _e1075;
                    phi_6759_ = vec3<f32>();
                    phi_6760_ = true;
                    break;
                }
                default: {
                    phi_15294_ = _e1075;
                    phi_6759_ = vec3<f32>();
                    phi_6760_ = bool();
                    break;
                }
            }
            let _e2386 = phi_15294_;
            let _e2388 = phi_6759_;
            let _e2390 = phi_6760_;
            if _e2386 {
                break;
            }
            if _e2390 {
                let _e2392 = select(1f, 0f, (_e802.rgb == 5u));
                let _e2396 = pill_1.member[_e229].power_hover;
                let _e2402 = ((abs(((f32(_e2396) - _e2392) - 1f)) - 0.4f) * -2.857143f);
                let _e2404 = select(_e2402, 0f, (_e2402 < 0f));
                let _e2406 = select(_e2404, 1f, (_e2404 > 1f));
                let _e2410 = ((_e2406 * _e2406) * (3f - (2f * _e2406)));
                let _e2412 = (1f + (_e2410 * 0.07f));
                let _e2413 = (_e805 / _e2412);
                let _e2414 = (_e806 / _e2412);
                let _e2418 = pill_1.member[_e229].power_state;
                let _e2424 = ((abs(((floor(_e2418) - _e2392) - 1f)) - 0.4f) * -2.857143f);
                let _e2426 = select(_e2424, 0f, (_e2424 < 0f));
                let _e2428 = select(_e2426, 1f, (_e2426 > 1f));
                let _e2432 = ((_e2428 * _e2428) * (3f - (2f * _e2428)));
                let _e2435 = ((_e2418 - trunc(_e2418)) * _e2432);
                if (_e2392 < 0.5f) {
                    let _e2559 = select(_e2435, 0f, (_e2435 < 0f));
                    let _e2561 = select(_e2559, 1f, (_e2559 > 1f));
                    let _e2565 = ((_e2561 * _e2561) * (3f - (2f * _e2561)));
                    let _e2571 = (1f - _e2435);
                    let _e2580 = (_e2565 * 0.7f);
                    let _e2581 = (_e2580 + 1.5999999f);
                    let _e2586 = ((abs((sqrt(((_e2413 * _e2413) + (_e2414 * _e2414))) - ((7.5f - (_e2435 * 4.6f)) + (((sin((_e549 * 8f)) * _e2435) * _e2571) * 0.16f)))) - _e2581) / ((_e2580 + 0.49999994f) - _e2581));
                    let _e2588 = select(_e2586, 0f, (_e2586 < 0f));
                    let _e2590 = select(_e2588, 1f, (_e2588 > 1f));
                    let _e2599 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2413, (_e2414 - -7f)), vec2<f32>((3f * _e2571), 3f), 0.5f);
                    let _e2601 = ((_e2599 - 0.55f) * -0.9090909f);
                    let _e2603 = select(_e2601, 0f, (_e2601 < 0f));
                    let _e2605 = select(_e2603, 1f, (_e2603 > 1f));
                    let _e2619 = cantus_render_shader_sd_rounded_box(vec2<f32>(_e2413, (_e2414 - (-5f + (_e2435 * 3.5f)))), vec2<f32>((1.05f + (_e2565 * 0.45f)), (4.6f - (_e2435 * 3f))), 0.7f);
                    let _e2621 = ((_e2619 - 0.55f) * -0.9090909f);
                    let _e2623 = select(_e2621, 0f, (_e2621 < 0f));
                    let _e2625 = select(_e2623, 1f, (_e2623 > 1f));
                    let _e2629 = ((_e2625 * _e2625) * (3f - (2f * _e2625)));
                    let _e2631 = (((_e2590 * _e2590) * (3f - (2f * _e2590))) * (1f - ((_e2605 * _e2605) * (3f - (2f * _e2605)))));
                    if (_e2631 != _e2631) {
                        phi_13498_ = true;
                    } else {
                        phi_13498_ = (_e2629 >= _e2631);
                    }
                    let _e2635 = phi_13498_;
                    phi_7160_ = select(_e2631, _e2629, _e2635);
                } else {
                    let _e2438 = ((1f - _e2432) + _e2435);
                    let _e2442 = (((atan2(_e2414, _e2413) - 0.50265485f) * 0.15915494f) + 1f);
                    let _e2446 = ((_e2438 * 0.82f) - 0.045f);
                    if (_e2446 != _e2446) {
                        phi_13423_ = true;
                    } else {
                        phi_13423_ = (0f >= _e2446);
                    }
                    let _e2450 = phi_13423_;
                    let _e2451 = select(_e2446, 0f, _e2450);
                    let _e2459 = ((abs((sqrt(((_e2413 * _e2413) + (_e2414 * _e2414))) - 7.1f)) - 1.5999999f) * -0.909091f);
                    let _e2461 = select(_e2459, 0f, (_e2459 < 0f));
                    let _e2463 = select(_e2461, 1f, (_e2461 > 1f));
                    let _e2468 = (_e2451 + 0.008f);
                    let _e2472 = (((_e2442 - trunc(_e2442)) - _e2468) / ((_e2451 - 0.008f) - _e2468));
                    let _e2474 = select(_e2472, 0f, (_e2472 < 0f));
                    let _e2476 = select(_e2474, 1f, (_e2474 > 1f));
                    let _e2482 = (_e2438 * 50f);
                    let _e2484 = select(_e2482, 0f, (_e2482 < 0f));
                    let _e2486 = select(_e2484, 1f, (_e2484 > 1f));
                    let _e2491 = ((((_e2463 * _e2463) * (3f - (2f * _e2463))) * ((_e2476 * _e2476) * (3f - (2f * _e2476)))) * ((_e2486 * _e2486) * (3f - (2f * _e2486))));
                    let _e2493 = (0.50265485f + (5.152212f * _e2438));
                    let _e2494 = cos(_e2493);
                    let _e2495 = sin(_e2493);
                    let _e2499 = (_e2413 - (_e2494 * 7.1f));
                    let _e2500 = (_e2414 - (_e2495 * 7.1f));
                    let _e2503 = ((_e2499 * -(_e2495)) + (_e2500 * _e2494));
                    let _e2506 = ((_e2499 * _e2494) + (_e2500 * _e2495));
                    let _e2507 = (_e2503 * -3.2f);
                    let _e2510 = ((_e2507 + (_e2506 * 2.1f)) * 0.06825939f);
                    let _e2512 = select(_e2510, 0f, (_e2510 < 0f));
                    let _e2514 = select(_e2512, 1f, (_e2512 > 1f));
                    let _e2517 = (_e2503 - (-3.2f * _e2514));
                    let _e2518 = (_e2506 - (2.1f * _e2514));
                    let _e2522 = sqrt(((_e2517 * _e2517) + (_e2518 * _e2518)));
                    let _e2525 = ((_e2507 + (_e2506 * -2.1f)) * 0.06825939f);
                    let _e2527 = select(_e2525, 0f, (_e2525 < 0f));
                    let _e2529 = select(_e2527, 1f, (_e2527 > 1f));
                    let _e2532 = (_e2503 - (-3.2f * _e2529));
                    let _e2533 = (_e2506 - (-2.1f * _e2529));
                    let _e2537 = sqrt(((_e2532 * _e2532) + (_e2533 * _e2533)));
                    if (_e2522 != _e2522) {
                        phi_13468_ = true;
                    } else {
                        phi_13468_ = (_e2537 <= _e2522);
                    }
                    let _e2541 = phi_13468_;
                    let _e2544 = ((select(_e2522, _e2537, _e2541) - 1.7f) * -0.71428573f);
                    let _e2546 = select(_e2544, 0f, (_e2544 < 0f));
                    let _e2548 = select(_e2546, 1f, (_e2546 > 1f));
                    let _e2552 = ((_e2548 * _e2548) * (3f - (2f * _e2548)));
                    if (_e2491 != _e2491) {
                        phi_13483_ = true;
                    } else {
                        phi_13483_ = (_e2552 >= _e2491);
                    }
                    let _e2556 = phi_13483_;
                    phi_7160_ = select(_e2491, _e2552, _e2556);
                }
                let _e2638 = phi_7160_;
                let _e2641 = (_e2432 * (0.5f + (_e2435 * 0.5f)));
                if (_e2410 != _e2410) {
                    phi_13513_ = true;
                } else {
                    phi_13513_ = (_e2641 >= _e2410);
                }
                let _e2645 = phi_13513_;
                let _e2646 = select(_e2410, _e2641, _e2645);
                let _e2648 = (0.48f * (1f - _e2646));
                let _e2659 = (1f + (_e2435 * 0.45f));
                phi_7187_ = vec3<f32>((_e1163 + (((_e2648 + (0.78f * _e2646)) * _e2638) * _e2659)), (_e1164 + (((_e2648 + (0.3f * _e2646)) * _e2638) * _e2659)), (_e1165 + (((_e2648 + (0.28f * _e2646)) * _e2638) * _e2659)));
            } else {
                phi_7187_ = _e2388;
            }
            let _e2668 = phi_7187_;
            let _e2670 = select(1u, 0u, (_e802.rgb == 0u));
            switch bitcast<i32>(_e802.rgb) {
                case 0: {
                    phi_7197_ = true;
                    break;
                }
                case 1: {
                    phi_7197_ = true;
                    break;
                }
                default: {
                    phi_7197_ = false;
                    break;
                }
            }
            let _e2673 = phi_7197_;
            if _e2673 {
                if (_e2670 < 2u) {
                } else {
                    break;
                }
                let _e2682 = pill_1.member[_e229].text.lines[_e2670].min[0u];
                let _e2690 = pill_1.member[_e229].text.lines[_e2670].min[1u];
                let _e2698 = pill_1.member[_e229].text.lines[_e2670].max[0u];
                let _e2706 = pill_1.member[_e229].text.lines[_e2670].max[1u];
                let _e2714 = pill_1.member[_e229].text.lines[_e2670].origin[0u];
                let _e2722 = pill_1.member[_e229].text.lines[_e2670].origin[1u];
                let _e2729 = pill_1.member[_e229].text.lines[_e2670].size;
                let _e2736 = pill_1.member[_e229].text.lines[_e2670].weight;
                let _e2743 = pill_1.member[_e229].text.lines[_e2670].count;
                let _e2750 = pill_1.member[_e229].text.lines[_e2670].first;
                if (_e298 < _e2682) {
                    phi_7455_ = f32();
                    phi_7456_ = true;
                } else {
                    if (_e298 > _e2698) {
                        phi_7453_ = f32();
                        phi_7454_ = true;
                    } else {
                        if (_e299 < _e2690) {
                            phi_7451_ = f32();
                            phi_7452_ = true;
                        } else {
                            let _e2754 = (_e299 > _e2706);
                            if _e2754 {
                                phi_7450_ = f32();
                            } else {
                                phi_7267_ = _e2743;
                                phi_7270_ = 0u;
                                loop {
                                    let _e2756 = phi_7267_;
                                    let _e2758 = phi_7270_;
                                    local_37 = _e2758;
                                    let _e2759 = (_e2758 < _e2756);
                                    if _e2759 {
                                        let _e2762 = (_e2758 + ((_e2756 - _e2758) / 2u));
                                        let _e2763 = (_e2750 + _e2762);
                                        if (_e2763 < 32u) {
                                        } else {
                                            phi_15465_ = true;
                                            break;
                                        }
                                        let _e2771 = pill_1.member[_e229].text.glyphs[_e2763].x;
                                        let _e2774 = (_e2771 <= ((_e298 - _e2714) / _e2729));
                                        if _e2774 {
                                            phi_7304_ = (_e2762 + 1u);
                                        } else {
                                            phi_7304_ = _e2758;
                                        }
                                        let _e2777 = phi_7304_;
                                        phi_7268_ = select(_e2762, _e2756, _e2774);
                                        phi_7271_ = _e2777;
                                    } else {
                                        phi_7268_ = u32();
                                        phi_7271_ = u32();
                                    }
                                    let _e2780 = phi_7268_;
                                    let _e2782 = phi_7271_;
                                    continue;
                                    continuing {
                                        phi_7267_ = _e2780;
                                        phi_7270_ = _e2782;
                                        phi_15465_ = _e2386;
                                        break if !(_e2759);
                                    }
                                }
                                let _e2785 = phi_15465_;
                                if _e2785 {
                                    break;
                                }
                                let _e2787 = local_37;
                                let _e2788 = (_e2787 + 1u);
                                phi_15515_ = _e2785;
                                phi_7312_ = select(_e2788, _e2743, (_e2743 < _e2788));
                                phi_7315_ = -1000000f;
                                loop {
                                    let _e2792 = phi_15515_;
                                    let _e2794 = phi_7312_;
                                    let _e2796 = phi_7315_;
                                    local_42 = _e2796;
                                    if (_e2794 > 0u) {
                                        let _e2798 = (_e2794 - 1u);
                                        let _e2799 = (_e2750 + _e2798);
                                        if (_e2799 < 32u) {
                                        } else {
                                            phi_15519_ = true;
                                            break;
                                        }
                                        let _e2807 = pill_1.member[_e229].text.glyphs[_e2799].x;
                                        let _e2814 = pill_1.member[_e229].text.glyphs[_e2799].glyph;
                                        if (_e2814 < arrayLength((&glyphs.member))) {
                                        } else {
                                            phi_15519_ = true;
                                            break;
                                        }
                                        let _e2820 = glyphs.member[_e2814].min[0u];
                                        let _e2825 = glyphs.member[_e2814].min[1u];
                                        let _e2830 = glyphs.member[_e2814].max[0u];
                                        let _e2835 = glyphs.member[_e2814].max[1u];
                                        let _e2839 = glyphs.member[_e2814].start;
                                        let _e2843 = glyphs.member[_e2814].count;
                                        let _e2846 = (((_e298 - _e2714) / _e2729) - _e2807);
                                        let _e2849 = (-((_e299 - _e2722)) / _e2729);
                                        let _e2850 = (3.5f / _e2729);
                                        let _e2851 = (_e2830 + _e2850);
                                        let _e2852 = (_e2846 > _e2851);
                                        if _e2852 {
                                            phi_15521_ = _e2792;
                                            phi_7443_ = f32();
                                        } else {
                                            if (_e2846 >= (_e2820 - _e2850)) {
                                                if (_e2849 >= (_e2825 - _e2850)) {
                                                    if (_e2846 <= _e2851) {
                                                        if (_e2849 <= (_e2835 + _e2850)) {
                                                            phi_7402_ = 0u;
                                                            phi_7405_ = 0i;
                                                            phi_7407_ = 340282350000000000000000000000000000000f;
                                                            loop {
                                                                let _e2861 = phi_7402_;
                                                                let _e2863 = phi_7405_;
                                                                let _e2865 = phi_7407_;
                                                                local_38 = _e2865;
                                                                local_39 = _e2863;
                                                                let _e2866 = (_e2861 < _e2843);
                                                                if _e2866 {
                                                                    let _e2867 = (_e2839 + _e2861);
                                                                    if (_e2867 < arrayLength((&edges.member))) {
                                                                    } else {
                                                                        phi_15512_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2871 = edges.member[_e2867];
                                                                    let _e2873 = cantus_render_text_edge_distance(_e2871, _e2736, vec2<f32>(_e2846, _e2849));
                                                                    if (_e2865 != _e2865) {
                                                                        phi_13528_ = true;
                                                                    } else {
                                                                        phi_13528_ = (_e2873.member <= _e2865);
                                                                    }
                                                                    let _e2879 = phi_13528_;
                                                                    phi_7403_ = (_e2861 + 1u);
                                                                    phi_7406_ = (_e2863 + _e2873.member_1);
                                                                    phi_7408_ = select(_e2865, _e2873.member, _e2879);
                                                                } else {
                                                                    phi_7403_ = u32();
                                                                    phi_7406_ = i32();
                                                                    phi_7408_ = f32();
                                                                }
                                                                let _e2884 = phi_7403_;
                                                                let _e2886 = phi_7406_;
                                                                let _e2888 = phi_7408_;
                                                                continue;
                                                                continuing {
                                                                    phi_7402_ = _e2884;
                                                                    phi_7405_ = _e2886;
                                                                    phi_7407_ = _e2888;
                                                                    phi_15512_ = _e2792;
                                                                    break if !(_e2866);
                                                                }
                                                            }
                                                            let _e2891 = phi_15512_;
                                                            phi_15519_ = _e2891;
                                                            if _e2891 {
                                                                break;
                                                            }
                                                            let _e2893 = local_38;
                                                            let _e2897 = local_39;
                                                            let _e2900 = ((sqrt(_e2893) * _e2729) * select(1f, -1f, (_e2897 == 0i)));
                                                            if (_e2796 != _e2796) {
                                                                phi_13543_ = true;
                                                            } else {
                                                                phi_13543_ = (_e2900 >= _e2796);
                                                            }
                                                            let _e2904 = phi_13543_;
                                                            phi_15525_ = _e2891;
                                                            phi_7439_ = select(_e2796, _e2900, _e2904);
                                                        } else {
                                                            phi_15525_ = _e2792;
                                                            phi_7439_ = _e2796;
                                                        }
                                                        let _e2907 = phi_15525_;
                                                        let _e2909 = phi_7439_;
                                                        phi_15524_ = _e2907;
                                                        phi_7440_ = _e2909;
                                                    } else {
                                                        phi_15524_ = _e2792;
                                                        phi_7440_ = _e2796;
                                                    }
                                                    let _e2911 = phi_15524_;
                                                    let _e2913 = phi_7440_;
                                                    phi_15523_ = _e2911;
                                                    phi_7441_ = _e2913;
                                                } else {
                                                    phi_15523_ = _e2792;
                                                    phi_7441_ = _e2796;
                                                }
                                                let _e2915 = phi_15523_;
                                                let _e2917 = phi_7441_;
                                                phi_15522_ = _e2915;
                                                phi_7442_ = _e2917;
                                            } else {
                                                phi_15522_ = _e2792;
                                                phi_7442_ = _e2796;
                                            }
                                            let _e2919 = phi_15522_;
                                            let _e2921 = phi_7442_;
                                            phi_15521_ = _e2919;
                                            phi_7443_ = _e2921;
                                        }
                                        let _e2923 = phi_15521_;
                                        let _e2925 = phi_7443_;
                                        phi_15520_ = _e2923;
                                        phi_7313_ = _e2798;
                                        phi_7316_ = _e2925;
                                        phi_7445_ = select(true, false, _e2852);
                                    } else {
                                        phi_15520_ = _e2792;
                                        phi_7313_ = u32();
                                        phi_7316_ = f32();
                                        phi_7445_ = false;
                                    }
                                    let _e2928 = phi_15520_;
                                    let _e2930 = phi_7313_;
                                    let _e2932 = phi_7316_;
                                    let _e2934 = phi_7445_;
                                    continue;
                                    continuing {
                                        phi_15515_ = _e2928;
                                        phi_7312_ = _e2930;
                                        phi_7315_ = _e2932;
                                        phi_15519_ = _e2928;
                                        break if !(_e2934);
                                    }
                                }
                                let _e2937 = phi_15519_;
                                if _e2937 {
                                    break;
                                }
                                let _e3149 = local_42;
                                phi_7450_ = _e3149;
                            }
                            let _e2939 = phi_7450_;
                            phi_7451_ = _e2939;
                            phi_7452_ = _e2754;
                        }
                        let _e2941 = phi_7451_;
                        let _e2943 = phi_7452_;
                        phi_7453_ = _e2941;
                        phi_7454_ = _e2943;
                    }
                    let _e2945 = phi_7453_;
                    let _e2947 = phi_7454_;
                    phi_7455_ = _e2945;
                    phi_7456_ = _e2947;
                }
                let _e2949 = phi_7455_;
                let _e2951 = phi_7456_;
                phi_7461_ = select(_e2949, -1000000f, _e2951);
            } else {
                phi_7461_ = -1000000f;
            }
            let _e2954 = phi_7461_;
            let _e2956 = ((_e2954 * 1.25f) + 0.5f);
            let _e2958 = select(_e2956, 0f, (_e2956 < 0f));
            let _e2960 = select(_e2958, 1f, (_e2958 > 1f));
            let _e2964 = ((_e2960 * _e2960) * (3f - (2f * _e2960)));
            let _e2965 = (1f - _e2964);
            let _e2972 = (0.94f * _e2964);
            let _e2977 = local_40;
            let _e2979 = (1f - (_e2977 * 0.35f));
            let _e2984 = local_41;
            let _e2985 = (_e2984 * 0.33249998f);
            out_color = vec4<f32>((((((_e2668.x * _e2965) + _e2972) * _e2979) + _e2985) * _e499), (((((_e2668.y * _e2965) + _e2972) * _e2979) + _e2985) * _e499), (((((_e2668.z * _e2965) + _e2972) * _e2979) + _e2985) * _e499), _e512);
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
    out_isthmus_instance_index = _e15;
    return;
}

fn render_playhead_fragment_impl() {
    var phi_13565_: bool;
    var phi_13580_: bool;
    var phi_13595_: bool;
    var phi_13612_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e30 = world_pos_1;
            let _e31 = _isthmus_instance_index_8;
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
                phi_13565_ = true;
            } else {
                phi_13565_ = (0f >= _e69);
            }
            let _e73 = phi_13565_;
            let _e74 = select(_e69, 0f, _e73);
            let _e79 = (sqrt(((_e67 * _e67) + (_e74 * _e74))) - 3.5f);
            let _e84 = state.member[_e31].icon_morph;
            let _e88 = state.member[_e31].icon_presence;
            let _e92 = ((_e45 * 0.18f) * (1f + (_e84 * (1f - _e88))));
            let _e94 = (_e92 * 0.5f);
            let _e95 = abs(-(_e49));
            let _e97 = (_e95 + (1.7320508f * _e48));
            if (_e97 != _e97) {
                phi_13580_ = true;
            } else {
                phi_13580_ = (0f >= _e97);
            }
            let _e101 = phi_13580_;
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
                phi_13595_ = true;
            } else {
                phi_13595_ = (_e138 >= _e149);
            }
            let _e153 = phi_13595_;
            let _e154 = select(_e149, _e138, _e153);
            if (_e154 <= 0f) {
                discard;
            }
            if (_e64 != _e64) {
                phi_13612_ = true;
            } else {
                phi_13612_ = (_e128 <= _e64);
            }
            let _e159 = phi_13612_;
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
    var phi_13637_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_8279_: isthmus_Vertex_render_particles_Varyings;
    var phi_8280_: isthmus_Vertex_render_particles_Varyings;
    var phi_8281_: bool;
    var phi_8289_: isthmus_Vertex_render_particles_Varyings;

    let _e30 = vertex_5;
    let _e31 = _isthmus_instance_index_7;
    let _e35 = frame.member[0u].time;
    let _e39 = particle.member[_e31].end_time;
    let _e43 = particle.member[_e31].duration;
    let _e45 = (_e35 - (_e39 - _e43));
    if (_e45 < 0f) {
        phi_8280_ = isthmus_Vertex_render_particles_Varyings();
        phi_8281_ = true;
    } else {
        let _e47 = (_e45 > _e43);
        if _e47 {
            phi_8279_ = isthmus_Vertex_render_particles_Varyings();
        } else {
            let _e48 = (_e45 / _e43);
            let _e53 = particle.member[_e31].spawn_vel[0u];
            let _e58 = particle.member[_e31].spawn_vel[1u];
            let _e62 = sqrt(((_e53 * _e53) + (_e58 * _e58)));
            if (_e62 > 0.001f) {
                phi_13637_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e53 / _e62), (_e58 / _e62)), _e62);
            } else {
                phi_13637_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e62);
            }
            let _e70 = phi_13637_;
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
            phi_8279_ = isthmus_Vertex_render_particles_Varyings(u0028_isthmus_glam_Vec4_u0020_isthmus_glam_Vec2_u0029_(vec4<f32>(((((_e125 + (_e116.x * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.y * 2f)) * 0.8f) + 0.2f) * 2f), ((((_e125 + (_e116.z * 2f)) * 0.8f) + 0.2f) * 2f), (((1f - _e48) * ((_e159 * _e159) * (3f - (2f * _e159)))) * 0.3f)), vec2<f32>(_e82, _e83)), _e153);
        }
        let _e171 = phi_8279_;
        phi_8280_ = _e171;
        phi_8281_ = _e47;
    }
    let _e173 = phi_8280_;
    let _e175 = phi_8281_;
    if _e175 {
        phi_8289_ = isthmus_Vertex_render_particles_Varyings(u0028_isthmus_glam_Vec4_u0020_isthmus_glam_Vec2_u0029_(vec4<f32>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f)), vec4<f32>(0f, 0f, 0f, 0f));
    } else {
        phi_8289_ = _e173;
    }
    let _e177 = phi_8289_;
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
    var phi_13677_: array<f32, 2>;
    var phi_13680_: array<f32, 2>;
    var phi_13681_: bool;
    var phi_13694_: f32;
    var phi_13704_: array<f32, 2>;
    var phi_13729_: array<f32, 2>;
    var phi_13732_: array<f32, 2>;
    var phi_13733_: bool;
    var phi_13746_: f32;
    var phi_13756_: array<f32, 2>;

    let _e28 = vertex_5;
    let _e29 = _isthmus_instance_index_7;
    let _e33 = pill_2.member[_e29].x;
    let _e37 = pill_2.member[_e29].calendar_expansion;
    let _e39 = select(_e37, 0f, (_e37 < 0f));
    let _e41 = select(_e39, 1f, (_e39 > 1f));
    let _e45 = ((_e41 * _e41) * (3f - (2f * _e41)));
    let _e49 = frame.member[0u].weather_hour;
    let _e53 = pill_2.member[_e29].sun_hours;
    let _e56 = (_e53[1] - _e53[0]);
    if (_e49 >= _e53[0]) {
        let _e58 = (_e49 <= _e53[1]);
        if _e58 {
            let _e60 = ((_e49 - _e53[0]) / _e56);
            phi_13677_ = array<f32, 2>(_e60, sin((_e60 * 3.1415927f)));
        } else {
            phi_13677_ = array<f32, 2>();
        }
        let _e65 = phi_13677_;
        phi_13680_ = _e65;
        phi_13681_ = select(true, false, _e58);
    } else {
        phi_13680_ = array<f32, 2>();
        phi_13681_ = true;
    }
    let _e68 = phi_13680_;
    let _e70 = phi_13681_;
    if _e70 {
        let _e71 = (24f - _e56);
        if (_e49 < _e53[0]) {
            phi_13694_ = (((_e49 + 24f) - _e53[1]) / _e71);
        } else {
            phi_13694_ = ((_e49 - _e53[1]) / _e71);
        }
        let _e79 = phi_13694_;
        phi_13704_ = array<f32, 2>(select(0f, 1f, (_e49 >= _e53[1])), -(sin((_e79 * 3.1415927f))));
    } else {
        phi_13704_ = _e68;
    }
    let _e87 = phi_13704_;
    if (12f >= _e53[0]) {
        let _e91 = (12f <= _e53[1]);
        if _e91 {
            let _e93 = ((12f - _e53[0]) / _e56);
            phi_13729_ = array<f32, 2>(_e93, sin((_e93 * 3.1415927f)));
        } else {
            phi_13729_ = array<f32, 2>();
        }
        let _e98 = phi_13729_;
        phi_13732_ = _e98;
        phi_13733_ = select(true, false, _e91);
    } else {
        phi_13732_ = array<f32, 2>();
        phi_13733_ = true;
    }
    let _e101 = phi_13732_;
    let _e103 = phi_13733_;
    if _e103 {
        let _e104 = (24f - _e56);
        if (12f < _e53[0]) {
            phi_13746_ = ((36f - _e53[1]) / _e104);
        } else {
            phi_13746_ = ((12f - _e53[1]) / _e104);
        }
        let _e111 = phi_13746_;
        phi_13756_ = array<f32, 2>(select(0f, 1f, (12f >= _e53[1])), -(sin((_e111 * 3.1415927f))));
    } else {
        phi_13756_ = _e101;
    }
    let _e119 = phi_13756_;
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
    out_isthmus_instance_index_1 = _e29;
    return;
}

fn render_tempestas_fragment_impl() {
    var phi_8594_: u32;
    var phi_8595_: u32;
    var phi_15529_: bool;
    var phi_13848_: bool;
    var phi_13863_: bool;
    var phi_13878_: bool;
    var phi_8867_: vec2<f32>;
    var phi_8870_: f32;
    var phi_8872_: u32;
    var phi_13906_: u0028_isthmus_glam_Vec2_u0020_f32_u0029_;
    var phi_13917_: bool;
    var phi_8868_: vec2<f32>;
    var phi_8871_: f32;
    var phi_8873_: u32;
    var phi_15539_: bool;
    var phi_9012_: f32;
    var local_43: vec2<f32>;
    var local_44: vec2<f32>;
    var phi_13981_: bool;
    var phi_9168_: f32;
    var phi_9182_: f32;
    var phi_9205_: f32;
    var phi_13999_: bool;
    var phi_14014_: bool;
    var phi_14029_: bool;
    var phi_14044_: bool;
    var phi_14128_: bool;
    var phi_14143_: bool;
    var phi_9412_: vec2<f32>;
    var phi_14158_: bool;
    var phi_14212_: array<f32, 2>;
    var phi_14215_: array<f32, 2>;
    var phi_14216_: bool;
    var phi_14229_: f32;
    var phi_14239_: array<f32, 2>;
    var phi_14297_: array<f32, 2>;
    var phi_14300_: array<f32, 2>;
    var phi_14301_: bool;
    var phi_14314_: f32;
    var phi_14324_: array<f32, 2>;
    var phi_9557_: u0028_render_tempestas_WeatherCondition_u0020_f32_u0029_;
    var phi_14343_: bool;
    var phi_14400_: i32;
    var phi_14401_: f32;
    var phi_14402_: f32;
    var phi_14403_: vec2<f32>;
    var phi_14428_: i32;
    var phi_14429_: f32;
    var phi_14430_: f32;
    var phi_14431_: vec2<f32>;
    var local_45: f32;
    var phi_14442_: i32;
    var phi_14443_: f32;
    var phi_14444_: f32;
    var phi_14445_: vec2<f32>;
    var phi_14470_: i32;
    var phi_14471_: f32;
    var phi_14472_: f32;
    var phi_14473_: vec2<f32>;
    var local_46: f32;
    var local_47: f32;
    var phi_10008_: vec3<f32>;
    var phi_10218_: vec3<f32>;
    var phi_10416_: vec3<f32>;
    var phi_10614_: vec3<f32>;
    var phi_14484_: i32;
    var phi_14485_: f32;
    var phi_14486_: f32;
    var phi_14487_: vec2<f32>;
    var phi_14512_: i32;
    var phi_14513_: f32;
    var phi_14514_: f32;
    var phi_14515_: vec2<f32>;
    var local_48: f32;
    var phi_10702_: vec3<f32>;
    var phi_14539_: i32;
    var phi_14540_: f32;
    var phi_14541_: f32;
    var phi_14542_: vec2<f32>;
    var phi_14567_: i32;
    var phi_14568_: f32;
    var phi_14569_: f32;
    var phi_14570_: vec2<f32>;
    var local_49: f32;
    var phi_10844_: f32;
    var phi_10964_: vec3<f32>;
    var phi_11214_: f32;
    var phi_11215_: u32;
    var phi_11216_: f32;
    var phi_11217_: u32;
    var phi_11116_: bool;
    var phi_11121_: f32;
    var phi_11122_: u32;
    var phi_11123_: f32;
    var phi_11124_: vec3<f32>;
    var phi_11000_: u32;
    var phi_11125_: f32;
    var phi_11126_: u32;
    var phi_11127_: f32;
    var phi_11128_: vec3<f32>;
    var phi_11218_: f32;
    var phi_11219_: u32;
    var phi_11220_: f32;
    var phi_11221_: vec3<f32>;
    var phi_11250_: u32;
    var phi_11251_: f32;
    var phi_11252_: vec3<f32>;
    var phi_11253_: vec2<f32>;
    var phi_11255_: u32;
    var phi_11256_: f32;
    var phi_11257_: vec3<f32>;
    var phi_11258_: vec2<f32>;
    var phi_11259_: bool;
    var phi_11335_: u32;
    var phi_11338_: u32;
    var phi_11372_: u32;
    var phi_11336_: u32;
    var phi_11339_: u32;
    var phi_15562_: bool;
    var local_50: u32;
    var phi_15759_: bool;
    var phi_11380_: u32;
    var phi_11383_: f32;
    var phi_11470_: u32;
    var phi_11473_: i32;
    var phi_11475_: f32;
    var phi_14627_: bool;
    var phi_11471_: u32;
    var phi_11474_: i32;
    var phi_11476_: f32;
    var phi_15756_: bool;
    var local_51: f32;
    var local_52: i32;
    var phi_14642_: bool;
    var phi_15769_: bool;
    var phi_11507_: f32;
    var phi_15768_: bool;
    var phi_11508_: f32;
    var phi_15767_: bool;
    var phi_11509_: f32;
    var phi_15766_: bool;
    var phi_11510_: f32;
    var phi_15765_: bool;
    var phi_11511_: f32;
    var phi_15764_: bool;
    var phi_11381_: u32;
    var phi_11384_: f32;
    var phi_11513_: bool;
    var phi_15763_: bool;
    var phi_11518_: f32;
    var phi_11519_: f32;
    var phi_11520_: bool;
    var phi_11521_: f32;
    var phi_11522_: bool;
    var phi_11523_: f32;
    var phi_11524_: bool;
    var local_53: f32;
    var local_54: f32;
    var local_55: f32;
    var local_56: f32;
    var local_57: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e218 = pixel_2;
            let _e219 = weather_1;
            let _e220 = _isthmus_instance_index_9;
            let _e233 = pill_2.member[_e220].x;
            let _e237 = frame.member[0u].panel_height;
            let _e241 = frame.member[0u].panel_top;
            let _e242 = (_e218.x - _e233);
            let _e243 = (_e218.y - _e241);
            let _e244 = (_e237 * 0.5f);
            let _e248 = ((308f - _e237) * 0.5f);
            let _e250 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e242 - 154f), (_e243 - _e244)), _e248, _e244);
            let _e255 = frame.member[0u].mouse_pos[0u];
            let _e260 = frame.member[0u].mouse_pos[1u];
            let _e266 = cantus_render_shader_sd_capsule_box(vec2<f32>(((_e255 - _e233) - 154f), ((_e260 - _e241) - _e244)), _e248, _e244);
            phi_8594_ = 0u;
            loop {
                let _e268 = phi_8594_;
                let _e269 = (_e268 < 4u);
                if _e269 {
                    if _e269 {
                    } else {
                        phi_15529_ = true;
                        break;
                    }
                    phi_8595_ = (_e268 + 1u);
                } else {
                    phi_8595_ = u32();
                }
                let _e272 = phi_8595_;
                continue;
                continuing {
                    phi_8594_ = _e272;
                    phi_15529_ = false;
                    break if !(_e269);
                }
            }
            let _e275 = phi_15529_;
            if _e275 {
                break;
            }
            let _e279 = frame.member[0u].mouse_pressure;
            let _e282 = (_e241 + _e237);
            let _e287 = (_e233 - (_e219.w * 158f));
            let _e289 = (_e218.y - _e282);
            let _e290 = (_e233 - 158f);
            let _e291 = (_e218.x - _e290);
            let _e292 = (8f * _e219.w);
            let _e293 = ((244f * _e219.w) - _e292);
            if (_e293 != _e293) {
                phi_13848_ = true;
            } else {
                phi_13848_ = (0f >= _e293);
            }
            let _e297 = phi_13848_;
            let _e299 = ((308f + (316f * _e219.w)) * 0.5f);
            let _e300 = (select(_e293, 0f, _e297) * 0.5f);
            let _e301 = (_e292 + _e300);
            let _e304 = (_e300 != _e300);
            if _e304 {
                phi_13863_ = true;
            } else {
                phi_13863_ = (18f <= _e300);
            }
            let _e307 = phi_13863_;
            let _e310 = vec2<f32>(_e299, _e300);
            let _e311 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e218.x - _e287) - _e299), (_e289 - _e301)), _e310, select(_e300, 18f, _e307));
            let _e313 = (_e260 - _e282);
            if _e304 {
                phi_13878_ = true;
            } else {
                phi_13878_ = (18f <= _e300);
            }
            let _e318 = phi_13878_;
            let _e321 = cantus_render_shader_sd_rounded_box(vec2<f32>(((_e255 - _e287) - _e299), (_e313 - _e301)), _e310, select(_e300, 18f, _e318));
            let _e324 = (0.5f + ((_e311 - _e250) * 0.008928572f));
            let _e326 = select(_e324, 0f, (_e324 < 0f));
            let _e328 = select(_e326, 1f, (_e326 > 1f));
            let _e341 = (0.5f + ((_e321 - _e266) * 0.008928572f));
            let _e343 = select(_e341, 0f, (_e341 < 0f));
            let _e345 = select(_e343, 1f, (_e343 > 1f));
            phi_8867_ = vec2<f32>(0f, 0f);
            phi_8870_ = 0f;
            phi_8872_ = 0u;
            loop {
                let _e357 = phi_8867_;
                let _e359 = phi_8870_;
                let _e361 = phi_8872_;
                local_43 = _e357;
                local_44 = _e357;
                local_53 = _e359;
                local_54 = _e359;
                local_55 = _e359;
                local_56 = _e359;
                let _e362 = (_e361 < 4u);
                if _e362 {
                    if _e362 {
                    } else {
                        phi_15539_ = true;
                        break;
                    }
                    let _e369 = frame.member[0u].ripples[_e361].origin[0u];
                    let _e376 = frame.member[0u].ripples[_e361].origin[1u];
                    let _e382 = frame.member[0u].ripples[_e361].start_time;
                    let _e388 = frame.member[0u].ripples[_e361].strength;
                    let _e392 = frame.member[0u].time;
                    let _e394 = ((_e392 - _e382) * 1.2f);
                    let _e396 = select(_e394, 0f, (_e394 < 0f));
                    let _e398 = select(_e396, 1f, (_e396 > 1f));
                    let _e399 = (_e218.x - _e369);
                    let _e400 = (_e218.y - _e376);
                    let _e404 = sqrt(((_e399 * _e399) + (_e400 * _e400)));
                    if (_e404 > 0.001f) {
                        phi_13906_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>((_e399 / _e404), (_e400 / _e404)), _e404);
                    } else {
                        phi_13906_ = u0028_isthmus_glam_Vec2_u0020_f32_u0029_(vec2<f32>(0f, 0f), _e404);
                    }
                    let _e412 = phi_13906_;
                    let _e422 = ((abs((_e412.unnamed_1 - (_e398 * 600f))) - 80f) * -0.0125f);
                    let _e424 = select(_e422, 0f, (_e422 < 0f));
                    let _e426 = select(_e424, 1f, (_e424 > 1f));
                    let _e432 = (1f - _e398);
                    let _e433 = ((((_e426 * _e426) * (3f - (2f * _e426))) * _e388) * _e432);
                    let _e446 = (_e359 + (_e433 * 0.5f));
                    if (_e446 != _e446) {
                        phi_13917_ = true;
                    } else {
                        phi_13917_ = (1f <= _e446);
                    }
                    let _e450 = phi_13917_;
                    phi_8868_ = vec2<f32>((_e357.x + (((_e412.unnamed.x * _e433) * _e432) * 0.5f)), (_e357.y + (((_e412.unnamed.y * _e433) * _e432) * 0.5f)));
                    phi_8871_ = select(_e446, 1f, _e450);
                    phi_8873_ = (_e361 + 1u);
                } else {
                    phi_8868_ = vec2<f32>();
                    phi_8871_ = f32();
                    phi_8873_ = u32();
                }
                let _e454 = phi_8868_;
                let _e456 = phi_8871_;
                let _e458 = phi_8873_;
                continue;
                continuing {
                    phi_8867_ = _e454;
                    phi_8870_ = _e456;
                    phi_8872_ = _e458;
                    phi_15539_ = _e275;
                    break if !(_e362);
                }
            }
            let _e461 = phi_15539_;
            if _e461 {
                break;
            }
            if (_e279 > 0f) {
                let _e462 = (_e218.x - _e255);
                let _e463 = (_e218.y - _e260);
                let _e469 = ((sqrt(((_e462 * _e462) + (_e463 * _e463))) - 150f) * -0.006666667f);
                let _e471 = select(_e469, 0f, (_e469 < 0f));
                let _e473 = select(_e471, 1f, (_e471 > 1f));
                phi_9012_ = ((((_e473 * _e473) * (3f - (2f * _e473))) * _e279) * 8f);
            } else {
                phi_9012_ = 0f;
            }
            let _e481 = phi_9012_;
            let _e483 = local_43;
            let _e486 = local_44;
            let _e489 = (((_e266 + ((((_e321 + ((_e266 - _e321) * _e345)) - ((56f * _e345) * (1f - _e345))) - _e266) * _e219.w)) - 0.5f) * -1f);
            let _e491 = select(_e489, 0f, (_e489 < 0f));
            let _e493 = select(_e491, 1f, (_e491 > 1f));
            let _e503 = (sqrt(((_e483.x * _e483.x) + (_e486.y * _e486.y))) * 22f);
            let _e506 = ((_e250 + ((((_e311 + ((_e250 - _e311) * _e328)) - ((56f * _e328) * (1f - _e328))) - _e250) * _e219.w)) - (((_e481 * ((_e493 * _e493) * (3f - (2f * _e493)))) + _e503) * 0.5f));
            let _e507 = (56f + _e244);
            let _e508 = (_e237 + 8f);
            let _e512 = (_e289 > ((_e507 + (_e507 + _e508)) * 0.5f));
            let _e517 = pill_2.member[_e220].calendar_expansion;
            let _e519 = (_e507 + (select(0f, 1f, _e512) * _e508));
            let _e520 = (_e519 * 0.0007377049f);
            let _e521 = (0.5f + _e520);
            let _e525 = ((_e517 - _e521) / ((_e520 + 0.74f) - _e521));
            let _e527 = select(_e525, 0f, (_e525 < 0f));
            let _e529 = select(_e527, 1f, (_e527 > 1f));
            let _e533 = ((_e529 * _e529) * (3f - (2f * _e529)));
            let _e535 = (292f * _e533);
            let _e536 = (_e237 * _e533);
            let _e541 = (324f + ((292f - _e535) * 0.5f));
            let _e542 = ((_e519 - _e244) + ((_e237 - _e536) * 0.5f));
            let _e543 = (_e291 - _e541);
            let _e544 = (_e289 - _e542);
            if (_e535 != _e535) {
                phi_13981_ = true;
            } else {
                phi_13981_ = (0.001f >= _e535);
            }
            let _e548 = phi_13981_;
            let _e550 = (_e543 / select(_e535, 0.001f, _e548));
            if _e512 {
                let _e558 = ((_e550 * 5f) - 0.5f);
                let _e560 = select(_e558, 0f, (_e558 < 0f));
                phi_9168_ = select(_e560, 4f, (_e560 > 4f));
            } else {
                let _e552 = ((_e550 * 6f) - 0.5f);
                let _e554 = select(_e552, 0f, (_e552 < 0f));
                phi_9168_ = select(_e554, 5f, (_e554 > 5f));
            }
            let _e564 = phi_9168_;
            let _e565 = (_e533 <= 0.001f);
            if _e565 {
                phi_9182_ = 340282350000000000000000000000000000000f;
            } else {
                let _e567 = (_e536 * 0.5f);
                let _e573 = cantus_render_shader_sd_capsule_box(vec2<f32>((_e543 - (_e533 * 146f)), (_e544 - _e567)), ((_e535 - _e536) * 0.5f), _e567);
                phi_9182_ = _e573;
            }
            let _e575 = phi_9182_;
            if _e565 {
                phi_9205_ = 340282350000000000000000000000000000000f;
            } else {
                let _e580 = (_e536 * 0.5f);
                let _e586 = cantus_render_shader_sd_capsule_box(vec2<f32>((((_e255 - _e290) - _e541) - (_e533 * 146f)), ((_e313 - _e542) - _e580)), ((_e535 - _e536) * 0.5f), _e580);
                phi_9205_ = _e586;
            }
            let _e588 = phi_9205_;
            let _e590 = ((_e588 - 0.5f) * -1f);
            let _e592 = select(_e590, 0f, (_e590 < 0f));
            let _e594 = select(_e592, 1f, (_e592 > 1f));
            let _e602 = (_e575 - (((_e481 * ((_e594 * _e594) * (3f - (2f * _e594)))) + _e503) * 0.5f));
            let _e603 = (_e506 != _e506);
            if _e603 {
                phi_13999_ = true;
            } else {
                phi_13999_ = (_e602 <= _e506);
            }
            let _e606 = phi_13999_;
            let _e607 = select(_e506, _e602, _e606);
            let _e608 = fwidth(_e607);
            if (_e608 != _e608) {
                phi_14014_ = true;
            } else {
                phi_14014_ = (0.55f >= _e608);
            }
            let _e612 = phi_14014_;
            let _e613 = select(_e608, 0.55f, _e612);
            let _e617 = ((_e607 - _e613) / (-(_e613) - _e613));
            let _e619 = select(_e617, 0f, (_e617 < 0f));
            let _e621 = select(_e619, 1f, (_e619 > 1f));
            let _e625 = ((_e621 * _e621) * (3f - (2f * _e621)));
            if (_e607 != _e607) {
                phi_14029_ = true;
            } else {
                phi_14029_ = (0f >= _e607);
            }
            let _e629 = phi_14029_;
            let _e633 = (exp((select(_e607, 0f, _e629) * -0.3f)) * 0.16f);
            if (_e625 != _e625) {
                phi_14044_ = true;
            } else {
                phi_14044_ = (_e633 >= _e625);
            }
            let _e637 = phi_14044_;
            let _e638 = select(_e625, _e633, _e637);
            if (_e638 <= 0.0009765625f) {
                discard;
            }
            let _e644 = pill_2.member[_e220].hourly_conditions[0u];
            let _e645 = (_e242 * 0.0032467532f);
            let _e647 = select(_e645, 0f, (_e645 < 0f));
            let _e656 = pill_2.member[_e220].hourly_conditions[1u];
            let _e658 = ((abs((select(_e647, 1f, (_e647 > 1f)) - 0.5f)) - 0.2f) * 20.000002f);
            let _e660 = select(_e658, 0f, (_e658 < 0f));
            let _e662 = select(_e660, 1f, (_e660 > 1f));
            let _e666 = ((_e662 * _e662) * (3f - (2f * _e662)));
            let _e671 = (_e644.fog + ((_e656.fog - _e644.fog) * _e666));
            let _e676 = (_e644.cloud + ((_e656.cloud - _e644.cloud) * _e666));
            let _e681 = (_e644.rain + ((_e656.rain - _e644.rain) * _e666));
            let _e686 = (_e644.snow + ((_e656.snow - _e644.snow) * _e666));
            let _e691 = (_e644.lightning + ((_e656.lightning - _e644.lightning) * _e666));
            let _e696 = (_e644.hail + ((_e656.hail - _e644.hail) * _e666));
            let _e699 = (_e671 + ((_e644.fog - _e671) * _e219.w));
            let _e702 = (_e676 + ((_e644.cloud - _e676) * _e219.w));
            let _e705 = (_e681 + ((_e644.rain - _e681) * _e219.w));
            let _e708 = (_e686 + ((_e644.snow - _e686) * _e219.w));
            let _e711 = (_e691 + ((_e644.lightning - _e691) * _e219.w));
            let _e714 = (_e696 + ((_e644.hail - _e696) * _e219.w));
            let _e715 = (_e243 / _e237);
            if _e603 {
                phi_14128_ = true;
            } else {
                phi_14128_ = (0f <= _e506);
            }
            let _e720 = phi_14128_;
            let _e723 = (1f + (select(_e506, 0f, _e720) * 0.008333334f));
            let _e725 = select(_e723, 0f, (_e723 < 0f));
            let _e727 = select(_e725, 0.6f, (_e725 > 0.6f));
            let _e734 = (_e483.x * 0.04f);
            let _e735 = (_e486.y * 0.04f);
            let _e736 = ((_e645 - (((_e645 - 0.5f) * _e727) * 0.08f)) - _e734);
            let _e737 = ((_e715 - (((_e715 - 0.5f) * _e727) * 0.08f)) - _e735);
            if (_e533 > 0.001f) {
                let _e740 = (_e543 / _e535);
                let _e741 = (_e544 / _e536);
                if (_e602 != _e602) {
                    phi_14143_ = true;
                } else {
                    phi_14143_ = (0f <= _e602);
                }
                let _e747 = phi_14143_;
                let _e750 = (1f + (select(_e602, 0f, _e747) * 0.008333334f));
                let _e752 = select(_e750, 0f, (_e750 < 0f));
                let _e754 = select(_e752, 0.6f, (_e752 > 0.6f));
                phi_9412_ = vec2<f32>(((_e740 - (((_e740 - 0.5f) * _e754) * 0.08f)) - _e734), ((_e741 - (((_e741 - 0.5f) * _e754) * 0.08f)) - _e735));
            } else {
                phi_9412_ = vec2<f32>(_e736, _e737);
            }
            let _e765 = phi_9412_;
            let _e766 = fwidth(_e602);
            if (_e766 != _e766) {
                phi_14158_ = true;
            } else {
                phi_14158_ = (0.55f >= _e766);
            }
            let _e770 = phi_14158_;
            let _e771 = select(_e766, 0.55f, _e770);
            let _e775 = ((_e602 - _e771) / (-(_e771) - _e771));
            let _e777 = select(_e775, 0f, (_e775 < 0f));
            let _e779 = select(_e777, 1f, (_e777 > 1f));
            let _e784 = (((_e779 * _e779) * (3f - (2f * _e779))) * _e533);
            let _e785 = floor(_e564);
            let _e790 = select(select(u32(_e785), 0u, (_e785 < 0f)), 4294967295u, (_e785 > 4294967000f));
            if _e512 {
                if (_e790 < 5u) {
                } else {
                    break;
                }
                let _e899 = pill_2.member[_e220].daily_conditions[_e790];
                let _e900 = (_e790 + 1u);
                let _e902 = select(_e900, 4u, (4u < _e900));
                if (_e902 < 5u) {
                } else {
                    break;
                }
                let _e908 = pill_2.member[_e220].daily_conditions[_e902];
                let _e910 = (_e564 - trunc(_e564));
                let _e912 = select(_e910, 0f, (_e910 < 0f));
                let _e914 = select(_e912, 1f, (_e912 > 1f));
                let _e918 = ((_e914 * _e914) * (3f - (2f * _e914)));
                let _e953 = pill_2.member[_e220].sun_hours;
                let _e956 = (_e953[1] - _e953[0]);
                if (12f >= _e953[0]) {
                    let _e958 = (12f <= _e953[1]);
                    if _e958 {
                        let _e960 = ((12f - _e953[0]) / _e956);
                        phi_14297_ = array<f32, 2>(_e960, sin((_e960 * 3.1415927f)));
                    } else {
                        phi_14297_ = array<f32, 2>();
                    }
                    let _e965 = phi_14297_;
                    phi_14300_ = _e965;
                    phi_14301_ = select(true, false, _e958);
                } else {
                    phi_14300_ = array<f32, 2>();
                    phi_14301_ = true;
                }
                let _e968 = phi_14300_;
                let _e970 = phi_14301_;
                if _e970 {
                    let _e971 = (24f - _e956);
                    if (12f < _e953[0]) {
                        phi_14314_ = ((36f - _e953[1]) / _e971);
                    } else {
                        phi_14314_ = ((12f - _e953[1]) / _e971);
                    }
                    let _e978 = phi_14314_;
                    phi_14324_ = array<f32, 2>(select(0f, 1f, (12f >= _e953[1])), -(sin((_e978 * 3.1415927f))));
                } else {
                    phi_14324_ = _e968;
                }
                let _e986 = phi_14324_;
                phi_9557_ = u0028_render_tempestas_WeatherCondition_u0020_f32_u0029_(render_tempestas_WeatherCondition((_e899.fog + ((_e908.fog - _e899.fog) * _e918)), (_e899.cloud + ((_e908.cloud - _e899.cloud) * _e918)), (_e899.rain + ((_e908.rain - _e899.rain) * _e918)), (_e899.snow + ((_e908.snow - _e899.snow) * _e918)), (_e899.lightning + ((_e908.lightning - _e899.lightning) * _e918)), (_e899.hail + ((_e908.hail - _e899.hail) * _e918))), _e986[1]);
            } else {
                if (_e790 < 6u) {
                } else {
                    break;
                }
                let _e796 = pill_2.member[_e220].hourly_conditions[_e790];
                let _e797 = (_e790 + 1u);
                let _e799 = select(_e797, 5u, (5u < _e797));
                if (_e799 < 6u) {
                } else {
                    break;
                }
                let _e805 = pill_2.member[_e220].hourly_conditions[_e799];
                let _e807 = (_e564 - trunc(_e564));
                let _e809 = select(_e807, 0f, (_e807 < 0f));
                let _e811 = select(_e809, 1f, (_e809 > 1f));
                let _e815 = ((_e811 * _e811) * (3f - (2f * _e811)));
                let _e850 = pill_2.member[_e220].hourly_start;
                let _e853 = ((_e850 + (_e564 * 4f)) % 24f);
                let _e857 = pill_2.member[_e220].sun_hours;
                let _e860 = (_e857[1] - _e857[0]);
                if (_e853 >= _e857[0]) {
                    let _e862 = (_e853 <= _e857[1]);
                    if _e862 {
                        let _e864 = ((_e853 - _e857[0]) / _e860);
                        phi_14212_ = array<f32, 2>(_e864, sin((_e864 * 3.1415927f)));
                    } else {
                        phi_14212_ = array<f32, 2>();
                    }
                    let _e869 = phi_14212_;
                    phi_14215_ = _e869;
                    phi_14216_ = select(true, false, _e862);
                } else {
                    phi_14215_ = array<f32, 2>();
                    phi_14216_ = true;
                }
                let _e872 = phi_14215_;
                let _e874 = phi_14216_;
                if _e874 {
                    let _e875 = (24f - _e860);
                    if (_e853 < _e857[0]) {
                        phi_14229_ = (((_e853 + 24f) - _e857[1]) / _e875);
                    } else {
                        phi_14229_ = ((_e853 - _e857[1]) / _e875);
                    }
                    let _e883 = phi_14229_;
                    phi_14239_ = array<f32, 2>(select(0f, 1f, (_e853 >= _e857[1])), -(sin((_e883 * 3.1415927f))));
                } else {
                    phi_14239_ = _e872;
                }
                let _e891 = phi_14239_;
                phi_9557_ = u0028_render_tempestas_WeatherCondition_u0020_f32_u0029_(render_tempestas_WeatherCondition((_e796.fog + ((_e805.fog - _e796.fog) * _e815)), (_e796.cloud + ((_e805.cloud - _e796.cloud) * _e815)), (_e796.rain + ((_e805.rain - _e796.rain) * _e815)), (_e796.snow + ((_e805.snow - _e796.snow) * _e815)), (_e796.lightning + ((_e805.lightning - _e796.lightning) * _e815)), (_e796.hail + ((_e805.hail - _e796.hail) * _e815))), _e891[1]);
            }
            let _e990 = phi_9557_;
            let _e995 = (1f - _e784);
            let _e1000 = ((_e736 * _e995) + (_e765.x * _e784));
            if (_e602 != _e602) {
                phi_14343_ = true;
            } else {
                phi_14343_ = (1000f <= _e602);
            }
            let _e1011 = phi_14343_;
            let _e1019 = (_e699 + ((_e990.unnamed.fog - _e699) * _e784));
            let _e1023 = (_e702 + ((_e990.unnamed.cloud - _e702) * _e784));
            let _e1027 = (_e705 + ((_e990.unnamed.rain - _e705) * _e784));
            let _e1031 = (_e708 + ((_e990.unnamed.snow - _e708) * _e784));
            let _e1039 = (_e714 + ((_e990.unnamed.hail - _e714) * _e784));
            let _e1042 = (_e219.y + ((_e990.unnamed_1 - _e219.y) * _e784));
            let _e1043 = (_e1000 * ((308f * _e995) + (_e535 * _e784)));
            let _e1044 = (((_e737 * _e995) + (_e765.y * _e784)) * ((_e237 * _e995) + (_e536 * _e784)));
            let _e1048 = frame.member[0u].time;
            let _e1049 = (_e1044 / _e237);
            let _e1051 = ((_e1042 - -0.04f) * 4.1666665f);
            let _e1053 = select(_e1051, 0f, (_e1051 < 0f));
            let _e1055 = select(_e1053, 1f, (_e1053 > 1f));
            let _e1059 = ((_e1055 * _e1055) * (3f - (2f * _e1055)));
            let _e1061 = ((_e1042 - -0.32f) * 4.166667f);
            let _e1063 = select(_e1061, 0f, (_e1061 < 0f));
            let _e1065 = select(_e1063, 1f, (_e1063 > 1f));
            let _e1070 = (1f - _e1059);
            let _e1073 = ((_e1042 - -0.18f) * 5.5555553f);
            let _e1075 = select(_e1073, 0f, (_e1073 < 0f));
            let _e1077 = select(_e1075, 1f, (_e1075 > 1f));
            let _e1083 = ((_e1042 - 0.2f) * -5.5555553f);
            let _e1085 = select(_e1083, 0f, (_e1083 < 0f));
            let _e1087 = select(_e1085, 1f, (_e1085 > 1f));
            let _e1092 = (((_e1077 * _e1077) * (3f - (2f * _e1077))) * ((_e1087 * _e1087) * (3f - (2f * _e1087))));
            let _e1094 = ((_e1049 - 1f) * -1f);
            let _e1096 = select(_e1094, 0f, (_e1094 < 0f));
            let _e1098 = select(_e1096, 1f, (_e1096 > 1f));
            let _e1102 = ((_e1098 * _e1098) * (3f - (2f * _e1098)));
            let _e1103 = (1f - _e1102);
            let _e1133 = (0.3f * _e1103);
            let _e1134 = (0.22f * _e1102);
            let _e1140 = ((((_e1065 * _e1065) * (3f - (2f * _e1065))) * _e1070) * 0.8f);
            let _e1141 = (1f - _e1140);
            let _e1158 = (_e1092 * 0.9f);
            let _e1159 = (1f - _e1158);
            let _e1171 = floor((_e1043 * 0.055555556f));
            let _e1172 = floor((_e1044 * 0.055555556f));
            let _e1176 = cantus_render_shader_hash(vec2<f32>(_e1171, _e1172));
            let _e1185 = (_e1043 - (((_e1171 + 0.2f) + (_e1176.x * 0.6f)) * 18f));
            let _e1186 = (_e1044 - (((_e1172 + 0.2f) + (_e1176.y * 0.6f)) * 18f));
            let _e1192 = ((sqrt(((_e1185 * _e1185) + (_e1186 * _e1186))) - 1f) * -1.6666666f);
            let _e1194 = select(_e1192, 0f, (_e1192 < 0f));
            let _e1196 = select(_e1194, 1f, (_e1194 > 1f));
            let _e1204 = cantus_render_shader_hash(vec2<f32>((_e1171 + 31.7f), (_e1172 + 31.7f)));
            let _e1207 = ((_e1204.x - 0.75f) * 4f);
            let _e1209 = select(_e1207, 0f, (_e1207 < 0f));
            let _e1211 = select(_e1209, 1f, (_e1209 > 1f));
            let _e1222 = ((((((_e1196 * _e1196) * (3f - (2f * _e1196))) * ((_e1211 * _e1211) * (3f - (2f * _e1211)))) * _e1070) * (1f - _e1023)) * (0.3f + (_e1102 * 0.7f)));
            let _e1223 = (((((((((0.006f * _e1103) + (0.025f * _e1102)) * _e1070) + (((0.08f * _e1103) + (0.32f * _e1102)) * _e1059)) * _e1141) + (((0.1f * _e1103) + _e1134) * _e1140)) * _e1159) + (((0.78f * _e1103) + (0.38f * _e1102)) * _e1158)) + _e1222);
            let _e1224 = (((((((((0.012f * _e1103) + (0.04f * _e1102)) * _e1070) + (((0.34f * _e1103) + (0.67f * _e1102)) * _e1059)) * _e1141) + (((0.16f * _e1103) + (0.25f * _e1102)) * _e1140)) * _e1159) + ((_e1133 + _e1134) * _e1158)) + _e1222);
            let _e1225 = (((((((((0.035f * _e1103) + (0.095f * _e1102)) * _e1070) + (((0.62f * _e1103) + (0.87f * _e1102)) * _e1059)) * _e1141) + ((_e1133 + (0.45f * _e1102)) * _e1140)) * _e1159) + (((0.2f * _e1103) + (0.42f * _e1102)) * _e1158)) + _e1222);
            if (_e1023 > 0.0009765625f) {
                let _e1228 = (_e1043 / _e237);
                phi_14400_ = 0i;
                phi_14401_ = 0.5f;
                phi_14402_ = 0f;
                phi_14403_ = vec2<f32>(((_e1228 * 0.14f) + (_e1048 * 0.012f)), ((_e1049 * 0.14f) + 6.1f));
                loop {
                    let _e1236 = phi_14400_;
                    let _e1238 = phi_14401_;
                    let _e1240 = phi_14402_;
                    let _e1242 = phi_14403_;
                    local_45 = _e1240;
                    let _e1243 = (_e1236 < 4i);
                    if _e1243 {
                        let _e1246 = cantus_render_shader_simplex_noise(_e1242);
                        phi_14428_ = (_e1236 + 1i);
                        phi_14429_ = (_e1238 * 0.5f);
                        phi_14430_ = (_e1240 + (_e1246 * _e1238));
                        phi_14431_ = vec2<f32>(((_e1242.x * 1.6f) + (_e1242.y * 1.2f)), ((_e1242.y * 1.6f) - (_e1242.x * 1.2f)));
                    } else {
                        phi_14428_ = i32();
                        phi_14429_ = f32();
                        phi_14430_ = f32();
                        phi_14431_ = vec2<f32>();
                    }
                    let _e1259 = phi_14428_;
                    let _e1261 = phi_14429_;
                    let _e1263 = phi_14430_;
                    let _e1265 = phi_14431_;
                    continue;
                    continuing {
                        phi_14400_ = _e1259;
                        phi_14401_ = _e1261;
                        phi_14402_ = _e1263;
                        phi_14403_ = _e1265;
                        break if !(_e1243);
                    }
                }
                let _e1268 = local_45;
                let _e1269 = (_e1268 * 0.5f);
                phi_14442_ = 0i;
                phi_14443_ = 0.5f;
                phi_14444_ = 0f;
                phi_14445_ = vec2<f32>(((_e1228 * 0.287f) + (_e1048 * 0.018f)), ((_e1049 * 0.287f) + -3.7f));
                loop {
                    let _e1278 = phi_14442_;
                    let _e1280 = phi_14443_;
                    let _e1282 = phi_14444_;
                    let _e1284 = phi_14445_;
                    local_46 = _e1282;
                    local_47 = _e1282;
                    let _e1285 = (_e1278 < 4i);
                    if _e1285 {
                        let _e1288 = cantus_render_shader_simplex_noise(_e1284);
                        phi_14470_ = (_e1278 + 1i);
                        phi_14471_ = (_e1280 * 0.5f);
                        phi_14472_ = (_e1282 + (_e1288 * _e1280));
                        phi_14473_ = vec2<f32>(((_e1284.x * 1.6f) + (_e1284.y * 1.2f)), ((_e1284.y * 1.6f) - (_e1284.x * 1.2f)));
                    } else {
                        phi_14470_ = i32();
                        phi_14471_ = f32();
                        phi_14472_ = f32();
                        phi_14473_ = vec2<f32>();
                    }
                    let _e1301 = phi_14470_;
                    let _e1303 = phi_14471_;
                    let _e1305 = phi_14472_;
                    let _e1307 = phi_14473_;
                    continue;
                    continuing {
                        phi_14442_ = _e1301;
                        phi_14443_ = _e1303;
                        phi_14444_ = _e1305;
                        phi_14445_ = _e1307;
                        break if !(_e1285);
                    }
                }
                let _e1310 = local_46;
                let _e1313 = local_47;
                let _e1317 = ((((0.5f + _e1269) + (_e1313 * 0.12f)) - 0.35f) * 3.9999995f);
                let _e1319 = select(_e1317, 0f, (_e1317 < 0f));
                let _e1321 = select(_e1319, 1f, (_e1319 > 1f));
                let _e1327 = (((_e1310 * 0.5f) + 0.08000001f) * 3.3333328f);
                let _e1329 = select(_e1327, 0f, (_e1327 < 0f));
                let _e1331 = select(_e1329, 1f, (_e1329 > 1f));
                let _e1338 = ((_e1269 + 0.02000001f) * 4.5454545f);
                let _e1340 = select(_e1338, 0f, (_e1338 < 0f));
                let _e1342 = select(_e1340, 1f, (_e1340 > 1f));
                let _e1348 = ((((_e1331 * _e1331) * (3f - (2f * _e1331))) * 0.55f) + (((_e1342 * _e1342) * (3f - (2f * _e1342))) * 0.45f));
                let _e1349 = (1f - _e1348);
                let _e1386 = (_e1092 * 0.45f);
                let _e1387 = (1f - _e1386);
                let _e1399 = (_e1023 * (0.12f + (((_e1321 * _e1321) * (3f - (2f * _e1321))) * 0.7f)));
                let _e1400 = (1f - _e1399);
                phi_10008_ = vec3<f32>(((_e1223 * _e1400) + (((((((0.16f * _e1349) + (0.32f * _e1348)) * _e1070) + (((0.62f * _e1349) + (0.92f * _e1348)) * _e1059)) * _e1387) + (((0.5f * _e1349) + (0.76f * _e1348)) * _e1386)) * _e1399)), ((_e1224 * _e1400) + (((((((0.2f * _e1349) + (0.36f * _e1348)) * _e1070) + (((0.7f * _e1349) + (0.94f * _e1348)) * _e1059)) * _e1387) + (((0.36f * _e1349) + (0.59f * _e1348)) * _e1386)) * _e1399)), ((_e1225 * _e1400) + (((((((0.28f * _e1349) + (0.43f * _e1348)) * _e1070) + (((0.78f * _e1349) + (0.96f * _e1348)) * _e1059)) * _e1387) + (((0.4f * _e1349) + (0.56f * _e1348)) * _e1386)) * _e1399)));
            } else {
                phi_10008_ = vec3<f32>(_e1223, _e1224, _e1225);
            }
            let _e1412 = phi_10008_;
            let _e1414 = (1f - (_e1027 * 0.2f));
            let _e1424 = ((_e1412.x * _e1414) + (_e1027 * 0.020000001f));
            let _e1425 = ((_e1412.y * _e1414) + (_e1027 * 0.034f));
            let _e1426 = ((_e1412.z * _e1414) + (_e1027 * 0.05f));
            if (_e1027 > 0.0009765625f) {
                let _e1431 = (_e1043 - (20f * _e1048));
                let _e1432 = (_e1044 - (110f * _e1048));
                let _e1435 = floor((_e1431 * 0.06666667f));
                let _e1436 = floor((_e1432 * 0.04f));
                let _e1438 = cantus_render_shader_hash(vec2<f32>(_e1435, _e1436));
                let _e1449 = (_e1431 - (((_e1435 + 0.15f) + (_e1438.x * 0.7f)) * 15f));
                let _e1450 = (_e1432 - (((_e1436 + 0.15f) + (_e1438.y * 0.7f)) * 25f));
                let _e1454 = (((_e1449 * 1.8000001f) + (_e1450 * 9f)) * 0.011870845f);
                let _e1456 = select(_e1454, 0f, (_e1454 < 0f));
                let _e1458 = select(_e1456, 1f, (_e1456 > 1f));
                let _e1461 = (_e1449 - (1.8000001f * _e1458));
                let _e1462 = (_e1450 - (9f * _e1458));
                let _e1468 = ((sqrt(((_e1461 * _e1461) + (_e1462 * _e1462))) - 1.0999999f) * -1.666667f);
                let _e1470 = select(_e1468, 0f, (_e1468 < 0f));
                let _e1472 = select(_e1470, 1f, (_e1470 > 1f));
                let _e1480 = cantus_render_shader_hash(vec2<f32>((_e1435 + 19.3f), (_e1436 + 19.3f)));
                let _e1483 = ((_e1480.x - 0.22000003f) * 1.2820513f);
                let _e1485 = select(_e1483, 0f, (_e1483 < 0f));
                let _e1487 = select(_e1485, 1f, (_e1485 > 1f));
                let _e1494 = (((((_e1472 * _e1472) * (3f - (2f * _e1472))) * ((_e1487 * _e1487) * (3f - (2f * _e1487)))) * _e1027) * 0.7f);
                let _e1496 = select(_e1494, 0f, (_e1494 < 0f));
                let _e1498 = select(_e1496, 1f, (_e1496 > 1f));
                let _e1499 = (1f - _e1498);
                phi_10218_ = vec3<f32>(((_e1424 * _e1499) + (0.52f * _e1498)), ((_e1425 * _e1499) + (0.72f * _e1498)), ((_e1426 * _e1499) + (0.9f * _e1498)));
            } else {
                phi_10218_ = vec3<f32>(_e1424, _e1425, _e1426);
            }
            let _e1511 = phi_10218_;
            if (_e1031 > 0.0009765625f) {
                let _e1515 = (_e1043 - (5f * _e1048));
                let _e1516 = (_e1044 - (14f * _e1048));
                let _e1519 = floor((_e1515 * 0.05f));
                let _e1520 = floor((_e1516 * 0.05f));
                let _e1524 = cantus_render_shader_hash(vec2<f32>((_e1519 + 31.7f), (_e1520 + 31.7f)));
                let _e1535 = (_e1515 - (((_e1519 + 0.15f) + (_e1524.x * 0.7f)) * 20f));
                let _e1536 = (_e1516 - (((_e1520 + 0.15f) + (_e1524.y * 0.7f)) * 20f));
                let _e1540 = (((_e1535 * 0.080000006f) + (_e1536 * 0.4f)) * 6.009615f);
                let _e1542 = select(_e1540, 0f, (_e1540 < 0f));
                let _e1544 = select(_e1542, 1f, (_e1542 > 1f));
                let _e1547 = (_e1535 - (0.080000006f * _e1544));
                let _e1548 = (_e1536 - (0.4f * _e1544));
                let _e1554 = ((sqrt(((_e1547 * _e1547) + (_e1548 * _e1548))) - 1.5999999f) * -1.666667f);
                let _e1556 = select(_e1554, 0f, (_e1554 < 0f));
                let _e1558 = select(_e1556, 1f, (_e1556 > 1f));
                let _e1566 = cantus_render_shader_hash(vec2<f32>((_e1519 + 19.3f), (_e1520 + 19.3f)));
                let _e1569 = ((_e1566.x - 0.3f) * 1.4285715f);
                let _e1571 = select(_e1569, 0f, (_e1569 < 0f));
                let _e1573 = select(_e1571, 1f, (_e1571 > 1f));
                let _e1580 = (((((_e1558 * _e1558) * (3f - (2f * _e1558))) * ((_e1573 * _e1573) * (3f - (2f * _e1573)))) * _e1031) * 0.92f);
                let _e1582 = select(_e1580, 0f, (_e1580 < 0f));
                let _e1584 = select(_e1582, 1f, (_e1582 > 1f));
                let _e1585 = (1f - _e1584);
                let _e1592 = (0.96f * _e1584);
                phi_10416_ = vec3<f32>(((_e1511.x * _e1585) + _e1592), ((_e1511.y * _e1585) + _e1592), ((_e1511.z * _e1585) + _e1592));
            } else {
                phi_10416_ = _e1511;
            }
            let _e1598 = phi_10416_;
            if (_e1039 > 0.0009765625f) {
                let _e1602 = (_e1043 - (18f * _e1048));
                let _e1603 = (_e1044 - (85f * _e1048));
                let _e1606 = floor((_e1602 * 0.04347826f));
                let _e1607 = floor((_e1603 * 0.04347826f));
                let _e1611 = cantus_render_shader_hash(vec2<f32>((_e1606 + 63.4f), (_e1607 + 63.4f)));
                let _e1622 = (_e1602 - (((_e1606 + 0.15f) + (_e1611.x * 0.7f)) * 23f));
                let _e1623 = (_e1603 - (((_e1607 + 0.15f) + (_e1611.y * 0.7f)) * 23f));
                let _e1627 = (((_e1622 * 0.24000001f) + (_e1623 * 1.2f)) * 0.667735f);
                let _e1629 = select(_e1627, 0f, (_e1627 < 0f));
                let _e1631 = select(_e1629, 1f, (_e1629 > 1f));
                let _e1634 = (_e1622 - (0.24000001f * _e1631));
                let _e1635 = (_e1623 - (1.2f * _e1631));
                let _e1641 = ((sqrt(((_e1634 * _e1634) + (_e1635 * _e1635))) - 0.79999995f) * -1.6666667f);
                let _e1643 = select(_e1641, 0f, (_e1641 < 0f));
                let _e1645 = select(_e1643, 1f, (_e1643 > 1f));
                let _e1653 = cantus_render_shader_hash(vec2<f32>((_e1606 + 19.3f), (_e1607 + 19.3f)));
                let _e1656 = ((_e1653.x - 0.7f) * 3.3333333f);
                let _e1658 = select(_e1656, 0f, (_e1656 < 0f));
                let _e1660 = select(_e1658, 1f, (_e1658 > 1f));
                let _e1667 = (((((_e1645 * _e1645) * (3f - (2f * _e1645))) * ((_e1660 * _e1660) * (3f - (2f * _e1660)))) * _e1039) * 0.7f);
                let _e1669 = select(_e1667, 0f, (_e1667 < 0f));
                let _e1671 = select(_e1669, 1f, (_e1669 > 1f));
                let _e1672 = (1f - _e1671);
                phi_10614_ = vec3<f32>(((_e1598.x * _e1672) + (0.75f * _e1671)), ((_e1598.y * _e1672) + (0.86f * _e1671)), ((_e1598.z * _e1672) + (0.94f * _e1671)));
            } else {
                phi_10614_ = _e1598;
            }
            let _e1687 = phi_10614_;
            let _e1691 = ((sin((_e1048 * 2.7f)) - 0.92f) * 12.500003f);
            let _e1693 = select(_e1691, 0f, (_e1691 < 0f));
            let _e1695 = select(_e1693, 1f, (_e1693 > 1f));
            let _e1700 = (((_e1695 * _e1695) * (3f - (2f * _e1695))) * (_e711 + ((_e990.unnamed.lightning - _e711) * _e784)));
            let _e1702 = (1f - (_e1700 * 0.55f));
            let _e1712 = ((_e1687.x * _e1702) + (_e1700 * 0.3575f));
            let _e1713 = ((_e1687.y * _e1702) + (_e1700 * 0.407f));
            let _e1714 = ((_e1687.z * _e1702) + (_e1700 * 0.528f));
            if (_e1019 > 0.0009765625f) {
                phi_14484_ = 0i;
                phi_14485_ = 0.5f;
                phi_14486_ = 0f;
                phi_14487_ = vec2<f32>(((_e1000 * 0.9f) + (_e1048 * 0.008f)), ((_e1049 * 0.32f) + 12f));
                loop {
                    let _e1724 = phi_14484_;
                    let _e1726 = phi_14485_;
                    let _e1728 = phi_14486_;
                    let _e1730 = phi_14487_;
                    local_48 = _e1728;
                    let _e1731 = (_e1724 < 4i);
                    if _e1731 {
                        let _e1734 = cantus_render_shader_simplex_noise(_e1730);
                        phi_14512_ = (_e1724 + 1i);
                        phi_14513_ = (_e1726 * 0.5f);
                        phi_14514_ = (_e1728 + (_e1734 * _e1726));
                        phi_14515_ = vec2<f32>(((_e1730.x * 1.6f) + (_e1730.y * 1.2f)), ((_e1730.y * 1.6f) - (_e1730.x * 1.2f)));
                    } else {
                        phi_14512_ = i32();
                        phi_14513_ = f32();
                        phi_14514_ = f32();
                        phi_14515_ = vec2<f32>();
                    }
                    let _e1747 = phi_14512_;
                    let _e1749 = phi_14513_;
                    let _e1751 = phi_14514_;
                    let _e1753 = phi_14515_;
                    continue;
                    continuing {
                        phi_14484_ = _e1747;
                        phi_14485_ = _e1749;
                        phi_14486_ = _e1751;
                        phi_14487_ = _e1753;
                        break if !(_e1731);
                    }
                }
                let _e1756 = local_48;
                let _e1759 = (((_e1756 * 0.5f) + 0.15f) * 2.857143f);
                let _e1761 = select(_e1759, 0f, (_e1759 < 0f));
                let _e1763 = select(_e1761, 1f, (_e1761 > 1f));
                let _e1770 = (_e1019 * (0.58f + (((_e1763 * _e1763) * (3f - (2f * _e1763))) * 0.18f)));
                let _e1771 = (1f - _e1770);
                phi_10702_ = vec3<f32>(((_e1712 * _e1771) + (0.63f * _e1770)), ((_e1713 * _e1771) + (0.69f * _e1770)), ((_e1714 * _e1771) + (0.73f * _e1770)));
            } else {
                phi_10702_ = vec3<f32>(_e1712, _e1713, _e1714);
            }
            let _e1783 = phi_10702_;
            let _e1785 = ((_e1049 - 0.12f) * -8.333334f);
            let _e1787 = select(_e1785, 0f, (_e1785 < 0f));
            let _e1789 = select(_e1787, 1f, (_e1787 > 1f));
            let _e1796 = (((_e506 + ((select(_e602, 1000f, _e1011) - _e506) * _e784)) - 5f) * -0.125f);
            let _e1798 = select(_e1796, 0f, (_e1796 < 0f));
            let _e1800 = select(_e1798, 1f, (_e1798 > 1f));
            let _e1806 = ((((_e1789 * _e1789) * (3f - (2f * _e1789))) * 0.12f) + (((_e1800 * _e1800) * (3f - (2f * _e1800))) * 0.08f));
            let _e1808 = (_e1783.x + _e1806);
            let _e1810 = (_e1783.y + _e1806);
            let _e1812 = (_e1783.z + _e1806);
            if (_e250 < 1f) {
                let _e1817 = (16f + (_e219.x * 276f));
                let _e1819 = select(_e219.y, 0f, (_e219.y < 0f));
                let _e1823 = (0.72f - (select(_e1819, 1f, (_e1819 > 1f)) * 0.45f));
                let _e1826 = ((_e219.y - 0.55f) * -1.8867923f);
                let _e1828 = select(_e1826, 0f, (_e1826 < 0f));
                let _e1830 = select(_e1828, 1f, (_e1828 > 1f));
                let _e1834 = ((_e1830 * _e1830) * (3f - (2f * _e1830)));
                let _e1835 = (1f - _e1834);
                if (_e676 > 0.0009765625f) {
                    phi_14539_ = 0i;
                    phi_14540_ = 0.5f;
                    phi_14541_ = 0f;
                    phi_14542_ = vec2<f32>((((_e1817 / _e237) * 0.14f) + (_e1048 * 0.012f)), ((_e1823 * 0.14f) + 6.1f));
                    loop {
                        let _e1853 = phi_14539_;
                        let _e1855 = phi_14540_;
                        let _e1857 = phi_14541_;
                        let _e1859 = phi_14542_;
                        local_49 = _e1857;
                        let _e1860 = (_e1853 < 4i);
                        if _e1860 {
                            let _e1863 = cantus_render_shader_simplex_noise(_e1859);
                            phi_14567_ = (_e1853 + 1i);
                            phi_14568_ = (_e1855 * 0.5f);
                            phi_14569_ = (_e1857 + (_e1863 * _e1855));
                            phi_14570_ = vec2<f32>(((_e1859.x * 1.6f) + (_e1859.y * 1.2f)), ((_e1859.y * 1.6f) - (_e1859.x * 1.2f)));
                        } else {
                            phi_14567_ = i32();
                            phi_14568_ = f32();
                            phi_14569_ = f32();
                            phi_14570_ = vec2<f32>();
                        }
                        let _e1876 = phi_14567_;
                        let _e1878 = phi_14568_;
                        let _e1880 = phi_14569_;
                        let _e1882 = phi_14570_;
                        continue;
                        continuing {
                            phi_14539_ = _e1876;
                            phi_14540_ = _e1878;
                            phi_14541_ = _e1880;
                            phi_14542_ = _e1882;
                            break if !(_e1860);
                        }
                    }
                    let _e1885 = local_49;
                    let _e1888 = (((_e1885 * 0.5f) + 0.06999999f) * 3.846154f);
                    let _e1890 = select(_e1888, 0f, (_e1888 < 0f));
                    let _e1892 = select(_e1890, 1f, (_e1890 > 1f));
                    phi_10844_ = ((((_e1892 * _e1892) * (3f - (2f * _e1892))) * _e676) * 0.82f);
                } else {
                    phi_10844_ = 0f;
                }
                let _e1900 = phi_10844_;
                let _e1902 = ((_e219.y - -0.02f) * 16.666668f);
                let _e1904 = select(_e1902, 0f, (_e1902 < 0f));
                let _e1906 = select(_e1904, 1f, (_e1904 > 1f));
                let _e1913 = (_e242 - _e1817);
                let _e1914 = (_e243 - (_e237 * _e1823));
                let _e1918 = sqrt(((_e1913 * _e1913) + (_e1914 * _e1914)));
                let _e1920 = ((_e1918 - 62f) * -0.01724138f);
                let _e1922 = select(_e1920, 0f, (_e1920 < 0f));
                let _e1924 = select(_e1922, 1f, (_e1922 > 1f));
                let _e1931 = ((_e1918 - 11f) * -0.1f);
                let _e1933 = select(_e1931, 0f, (_e1931 < 0f));
                let _e1935 = select(_e1933, 1f, (_e1933 > 1f));
                let _e1942 = (((((_e1924 * _e1924) * (3f - (2f * _e1924))) * 0.24f) + (((_e1935 * _e1935) * (3f - (2f * _e1935))) * 0.7f)) * (((_e1906 * _e1906) * (3f - (2f * _e1906))) * (1f - _e1900)));
                let _e1943 = (1f - _e1942);
                let _e1956 = ((_e250 - 1f) / ((_e237 * -0.25f) - 1f));
                let _e1958 = select(_e1956, 0f, (_e1956 < 0f));
                let _e1960 = select(_e1958, 1f, (_e1958 > 1f));
                let _e1964 = ((_e1960 * _e1960) * (3f - (2f * _e1960)));
                let _e1965 = (1f - _e1964);
                phi_10964_ = vec3<f32>(((_e1808 * _e1965) + (((_e1808 * _e1943) + (((0.96f * _e1835) + (0.98f * _e1834)) * _e1942)) * _e1964)), ((_e1810 * _e1965) + (((_e1810 * _e1943) + (((0.98f * _e1835) + (0.74f * _e1834)) * _e1942)) * _e1964)), ((_e1812 * _e1965) + (((_e1812 * _e1943) + ((_e1835 + (0.66f * _e1834)) * _e1942)) * _e1964)));
            } else {
                phi_10964_ = (_e1783 + vec3(_e1806));
            }
            let _e1977 = phi_10964_;
            if (_e517 > 0f) {
                let _e1979 = (_e289 >= 0f);
                if _e1979 {
                    if (_e291 < 308f) {
                        if (_e289 < 54f) {
                            let _e2097 = (_e291 - 154f);
                            if (_e2097 < -60f) {
                                phi_11000_ = 2u;
                            } else {
                                phi_11000_ = select(1u, 3u, (_e2097 > 60f));
                            }
                            let _e2102 = phi_11000_;
                            phi_11125_ = 40f;
                            phi_11126_ = _e2102;
                            phi_11127_ = 1f;
                            phi_11128_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                        } else {
                            if (_e289 < 82f) {
                                let _e2078 = floor((_e291 * 0.022727273f));
                                let _e2080 = select(_e2078, 0f, (_e2078 < 0f));
                                let _e2082 = select(_e2080, 6f, (_e2080 > 6f));
                                phi_11121_ = 68f;
                                phi_11122_ = (27u + select(select(u32(_e2082), 0u, (_e2082 < 0f)), 4294967295u, (_e2082 > 4294967000f)));
                                phi_11123_ = 0.75f;
                                phi_11124_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                            } else {
                                let _e2026 = floor((((_e289 - 96f) * 0.041666668f) + 0.5f));
                                let _e2028 = select(_e2026, 0f, (_e2026 < 0f));
                                let _e2030 = select(_e2028, 5f, (_e2028 > 5f));
                                let _e2038 = floor((_e291 * 0.022727273f));
                                let _e2040 = select(_e2038, 0f, (_e2038 < 0f));
                                let _e2042 = select(_e2040, 6f, (_e2040 > 6f));
                                let _e2048 = ((select(select(u32(_e2030), 0u, (_e2030 < 0f)), 4294967295u, (_e2030 > 4294967000f)) * 7u) + select(select(u32(_e2042), 0u, (_e2042 < 0f)), 4294967295u, (_e2042 > 4294967000f)));
                                let _e2058 = pill_2.member[_e220].today_index;
                                let _e2066 = pill_2.member[_e220].month_range[0u];
                                if (_e2048 < _e2066) {
                                    phi_11116_ = true;
                                } else {
                                    let _e2072 = pill_2.member[_e220].month_range[1u];
                                    phi_11116_ = (_e2048 >= _e2072);
                                }
                                let _e2075 = phi_11116_;
                                phi_11121_ = (96f + (f32((_e2048 / 7u)) * 24f));
                                phi_11122_ = (34u + _e2048);
                                phi_11123_ = select(1f, 0.32f, _e2075);
                                phi_11124_ = select(vec3<f32>(0.94f, 0.94f, 0.94f), vec3<f32>(1f, 0.68f, 0.68f), vec3((bitcast<i32>(_e2048) == _e2058)));
                            }
                            let _e2090 = phi_11121_;
                            let _e2092 = phi_11122_;
                            let _e2094 = phi_11123_;
                            let _e2096 = phi_11124_;
                            phi_11125_ = _e2090;
                            phi_11126_ = _e2092;
                            phi_11127_ = _e2094;
                            phi_11128_ = _e2096;
                        }
                        let _e2104 = phi_11125_;
                        let _e2106 = phi_11126_;
                        let _e2108 = phi_11127_;
                        let _e2110 = phi_11128_;
                        phi_11218_ = _e2104;
                        phi_11219_ = _e2106;
                        phi_11220_ = _e2108;
                        phi_11221_ = _e2110;
                    } else {
                        if (_e291 >= 316f) {
                            if (_e289 >= 56f) {
                                let _e1984 = select(6u, 5u, _e512);
                                let _e1989 = ((((_e291 - 316f) * 0.0032467532f) * f32(_e1984)) - 0.5f);
                                let _e1991 = f32((_e1984 - 1u));
                                let _e1992 = (0f <= _e1991);
                                if _e1992 {
                                } else {
                                    break;
                                }
                                let _e1994 = select(_e1989, 0f, (_e1989 < 0f));
                                let _e1997 = round(select(_e1994, _e1991, (_e1994 > _e1991)));
                                if _e1992 {
                                } else {
                                    break;
                                }
                                let _e1999 = select(_e1997, 0f, (_e1997 < 0f));
                                let _e2001 = select(_e1999, _e1991, (_e1999 > _e1991));
                                phi_11214_ = _e519;
                                phi_11215_ = (select(5u, 17u, _e512) + ((select(select(u32(_e2001), 0u, (_e2001 < 0f)), 4294967295u, (_e2001 > 4294967000f)) * 2u) + select(0u, 1u, (_e289 >= _e519))));
                            } else {
                                phi_11214_ = 40f;
                                phi_11215_ = 4u;
                            }
                            let _e2014 = phi_11214_;
                            let _e2016 = phi_11215_;
                            phi_11216_ = _e2014;
                            phi_11217_ = _e2016;
                        } else {
                            phi_11216_ = 40f;
                            phi_11217_ = 4u;
                        }
                        let _e2018 = phi_11216_;
                        let _e2020 = phi_11217_;
                        phi_11218_ = _e2018;
                        phi_11219_ = _e2020;
                        phi_11220_ = 1f;
                        phi_11221_ = vec3<f32>(0.94f, 0.94f, 0.94f);
                    }
                    let _e2112 = phi_11218_;
                    let _e2114 = phi_11219_;
                    let _e2116 = phi_11220_;
                    let _e2118 = phi_11221_;
                    let _e2119 = (_e2112 * 0.0007377049f);
                    let _e2120 = (0.5f + _e2119);
                    let _e2124 = ((_e517 - _e2120) / ((_e2119 + 0.74f) - _e2120));
                    let _e2126 = select(_e2124, 0f, (_e2124 < 0f));
                    let _e2128 = select(_e2126, 1f, (_e2126 > 1f));
                    phi_11250_ = _e2114;
                    phi_11251_ = (((_e2128 * _e2128) * (3f - (2f * _e2128))) * _e2116);
                    phi_11252_ = _e2118;
                    phi_11253_ = vec2<f32>(_e291, _e289);
                } else {
                    phi_11250_ = u32();
                    phi_11251_ = f32();
                    phi_11252_ = vec3<f32>();
                    phi_11253_ = vec2<f32>();
                }
                let _e2135 = phi_11250_;
                let _e2137 = phi_11251_;
                let _e2139 = phi_11252_;
                let _e2141 = phi_11253_;
                phi_11255_ = _e2135;
                phi_11256_ = _e2137;
                phi_11257_ = _e2139;
                phi_11258_ = _e2141;
                phi_11259_ = select(true, false, _e1979);
            } else {
                phi_11255_ = u32();
                phi_11256_ = f32();
                phi_11257_ = vec3<f32>();
                phi_11258_ = vec2<f32>();
                phi_11259_ = true;
            }
            let _e2144 = phi_11255_;
            let _e2146 = phi_11256_;
            let _e2148 = phi_11257_;
            let _e2150 = phi_11258_;
            let _e2152 = phi_11259_;
            let _e2153 = select(_e2144, 0u, _e2152);
            let _e2156 = select(_e2148, vec3<f32>(0.94f, 0.94f, 0.94f), vec3(_e2152));
            let _e2158 = select(_e2150, vec2<f32>(_e242, _e243), vec2(_e2152));
            if (_e2153 < 76u) {
            } else {
                break;
            }
            let _e2169 = pill_2.member[_e220].text.lines[_e2153].min[0u];
            let _e2177 = pill_2.member[_e220].text.lines[_e2153].min[1u];
            let _e2185 = pill_2.member[_e220].text.lines[_e2153].max[0u];
            let _e2193 = pill_2.member[_e220].text.lines[_e2153].max[1u];
            let _e2201 = pill_2.member[_e220].text.lines[_e2153].origin[0u];
            let _e2209 = pill_2.member[_e220].text.lines[_e2153].origin[1u];
            let _e2216 = pill_2.member[_e220].text.lines[_e2153].size;
            let _e2223 = pill_2.member[_e220].text.lines[_e2153].weight;
            let _e2230 = pill_2.member[_e220].text.lines[_e2153].count;
            let _e2237 = pill_2.member[_e220].text.lines[_e2153].first;
            if (_e2158.x < _e2169) {
                phi_11523_ = f32();
                phi_11524_ = true;
            } else {
                if (_e2158.x > _e2185) {
                    phi_11521_ = f32();
                    phi_11522_ = true;
                } else {
                    if (_e2158.y < _e2177) {
                        phi_11519_ = f32();
                        phi_11520_ = true;
                    } else {
                        let _e2241 = (_e2158.y > _e2193);
                        if _e2241 {
                            phi_11518_ = f32();
                        } else {
                            phi_11335_ = _e2230;
                            phi_11338_ = 0u;
                            loop {
                                let _e2243 = phi_11335_;
                                let _e2245 = phi_11338_;
                                local_50 = _e2245;
                                let _e2246 = (_e2245 < _e2243);
                                if _e2246 {
                                    let _e2249 = (_e2245 + ((_e2243 - _e2245) / 2u));
                                    let _e2250 = (_e2237 + _e2249);
                                    if (_e2250 < 512u) {
                                    } else {
                                        phi_15562_ = true;
                                        break;
                                    }
                                    let _e2258 = pill_2.member[_e220].text.glyphs[_e2250].x;
                                    let _e2261 = (_e2258 <= ((_e2158.x - _e2201) / _e2216));
                                    if _e2261 {
                                        phi_11372_ = (_e2249 + 1u);
                                    } else {
                                        phi_11372_ = _e2245;
                                    }
                                    let _e2264 = phi_11372_;
                                    phi_11336_ = select(_e2249, _e2243, _e2261);
                                    phi_11339_ = _e2264;
                                } else {
                                    phi_11336_ = u32();
                                    phi_11339_ = u32();
                                }
                                let _e2267 = phi_11336_;
                                let _e2269 = phi_11339_;
                                continue;
                                continuing {
                                    phi_11335_ = _e2267;
                                    phi_11338_ = _e2269;
                                    phi_15562_ = _e461;
                                    break if !(_e2246);
                                }
                            }
                            let _e2272 = phi_15562_;
                            if _e2272 {
                                break;
                            }
                            let _e2274 = local_50;
                            let _e2275 = (_e2274 + 1u);
                            phi_15759_ = _e2272;
                            phi_11380_ = select(_e2275, _e2230, (_e2230 < _e2275));
                            phi_11383_ = -1000000f;
                            loop {
                                let _e2279 = phi_15759_;
                                let _e2281 = phi_11380_;
                                let _e2283 = phi_11383_;
                                local_57 = _e2283;
                                if (_e2281 > 0u) {
                                    let _e2285 = (_e2281 - 1u);
                                    let _e2286 = (_e2237 + _e2285);
                                    if (_e2286 < 512u) {
                                    } else {
                                        phi_15763_ = true;
                                        break;
                                    }
                                    let _e2294 = pill_2.member[_e220].text.glyphs[_e2286].x;
                                    let _e2301 = pill_2.member[_e220].text.glyphs[_e2286].glyph;
                                    if (_e2301 < arrayLength((&glyphs.member))) {
                                    } else {
                                        phi_15763_ = true;
                                        break;
                                    }
                                    let _e2307 = glyphs.member[_e2301].min[0u];
                                    let _e2312 = glyphs.member[_e2301].min[1u];
                                    let _e2317 = glyphs.member[_e2301].max[0u];
                                    let _e2322 = glyphs.member[_e2301].max[1u];
                                    let _e2326 = glyphs.member[_e2301].start;
                                    let _e2330 = glyphs.member[_e2301].count;
                                    let _e2333 = (((_e2158.x - _e2201) / _e2216) - _e2294);
                                    let _e2336 = (-((_e2158.y - _e2209)) / _e2216);
                                    let _e2337 = (3.5f / _e2216);
                                    let _e2338 = (_e2317 + _e2337);
                                    let _e2339 = (_e2333 > _e2338);
                                    if _e2339 {
                                        phi_15765_ = _e2279;
                                        phi_11511_ = f32();
                                    } else {
                                        if (_e2333 >= (_e2307 - _e2337)) {
                                            if (_e2336 >= (_e2312 - _e2337)) {
                                                if (_e2333 <= _e2338) {
                                                    if (_e2336 <= (_e2322 + _e2337)) {
                                                        phi_11470_ = 0u;
                                                        phi_11473_ = 0i;
                                                        phi_11475_ = 340282350000000000000000000000000000000f;
                                                        loop {
                                                            let _e2348 = phi_11470_;
                                                            let _e2350 = phi_11473_;
                                                            let _e2352 = phi_11475_;
                                                            local_51 = _e2352;
                                                            local_52 = _e2350;
                                                            let _e2353 = (_e2348 < _e2330);
                                                            if _e2353 {
                                                                let _e2354 = (_e2326 + _e2348);
                                                                if (_e2354 < arrayLength((&edges.member))) {
                                                                } else {
                                                                    phi_15756_ = true;
                                                                    break;
                                                                }
                                                                let _e2358 = edges.member[_e2354];
                                                                let _e2360 = cantus_render_text_edge_distance(_e2358, _e2223, vec2<f32>(_e2333, _e2336));
                                                                if (_e2352 != _e2352) {
                                                                    phi_14627_ = true;
                                                                } else {
                                                                    phi_14627_ = (_e2360.member <= _e2352);
                                                                }
                                                                let _e2366 = phi_14627_;
                                                                phi_11471_ = (_e2348 + 1u);
                                                                phi_11474_ = (_e2350 + _e2360.member_1);
                                                                phi_11476_ = select(_e2352, _e2360.member, _e2366);
                                                            } else {
                                                                phi_11471_ = u32();
                                                                phi_11474_ = i32();
                                                                phi_11476_ = f32();
                                                            }
                                                            let _e2371 = phi_11471_;
                                                            let _e2373 = phi_11474_;
                                                            let _e2375 = phi_11476_;
                                                            continue;
                                                            continuing {
                                                                phi_11470_ = _e2371;
                                                                phi_11473_ = _e2373;
                                                                phi_11475_ = _e2375;
                                                                phi_15756_ = _e2279;
                                                                break if !(_e2353);
                                                            }
                                                        }
                                                        let _e2378 = phi_15756_;
                                                        phi_15763_ = _e2378;
                                                        if _e2378 {
                                                            break;
                                                        }
                                                        let _e2380 = local_51;
                                                        let _e2384 = local_52;
                                                        let _e2387 = ((sqrt(_e2380) * _e2216) * select(1f, -1f, (_e2384 == 0i)));
                                                        if (_e2283 != _e2283) {
                                                            phi_14642_ = true;
                                                        } else {
                                                            phi_14642_ = (_e2387 >= _e2283);
                                                        }
                                                        let _e2391 = phi_14642_;
                                                        phi_15769_ = _e2378;
                                                        phi_11507_ = select(_e2283, _e2387, _e2391);
                                                    } else {
                                                        phi_15769_ = _e2279;
                                                        phi_11507_ = _e2283;
                                                    }
                                                    let _e2394 = phi_15769_;
                                                    let _e2396 = phi_11507_;
                                                    phi_15768_ = _e2394;
                                                    phi_11508_ = _e2396;
                                                } else {
                                                    phi_15768_ = _e2279;
                                                    phi_11508_ = _e2283;
                                                }
                                                let _e2398 = phi_15768_;
                                                let _e2400 = phi_11508_;
                                                phi_15767_ = _e2398;
                                                phi_11509_ = _e2400;
                                            } else {
                                                phi_15767_ = _e2279;
                                                phi_11509_ = _e2283;
                                            }
                                            let _e2402 = phi_15767_;
                                            let _e2404 = phi_11509_;
                                            phi_15766_ = _e2402;
                                            phi_11510_ = _e2404;
                                        } else {
                                            phi_15766_ = _e2279;
                                            phi_11510_ = _e2283;
                                        }
                                        let _e2406 = phi_15766_;
                                        let _e2408 = phi_11510_;
                                        phi_15765_ = _e2406;
                                        phi_11511_ = _e2408;
                                    }
                                    let _e2410 = phi_15765_;
                                    let _e2412 = phi_11511_;
                                    phi_15764_ = _e2410;
                                    phi_11381_ = _e2285;
                                    phi_11384_ = _e2412;
                                    phi_11513_ = select(true, false, _e2339);
                                } else {
                                    phi_15764_ = _e2279;
                                    phi_11381_ = u32();
                                    phi_11384_ = f32();
                                    phi_11513_ = false;
                                }
                                let _e2415 = phi_15764_;
                                let _e2417 = phi_11381_;
                                let _e2419 = phi_11384_;
                                let _e2421 = phi_11513_;
                                continue;
                                continuing {
                                    phi_15759_ = _e2415;
                                    phi_11380_ = _e2417;
                                    phi_11383_ = _e2419;
                                    phi_15763_ = _e2415;
                                    break if !(_e2421);
                                }
                            }
                            let _e2424 = phi_15763_;
                            if _e2424 {
                                break;
                            }
                            let _e2646 = local_57;
                            phi_11518_ = _e2646;
                        }
                        let _e2426 = phi_11518_;
                        phi_11519_ = _e2426;
                        phi_11520_ = _e2241;
                    }
                    let _e2428 = phi_11519_;
                    let _e2430 = phi_11520_;
                    phi_11521_ = _e2428;
                    phi_11522_ = _e2430;
                }
                let _e2432 = phi_11521_;
                let _e2434 = phi_11522_;
                phi_11523_ = _e2432;
                phi_11524_ = _e2434;
            }
            let _e2436 = phi_11523_;
            let _e2438 = phi_11524_;
            let _e2441 = ((select(_e2436, -1000000f, _e2438) * 1.25f) + 0.5f);
            let _e2443 = select(_e2441, 0f, (_e2441 < 0f));
            let _e2445 = select(_e2443, 1f, (_e2443 > 1f));
            let _e2450 = (((_e2445 * _e2445) * (3f - (2f * _e2445))) * select(_e2146, 1f, _e2152));
            let _e2451 = (1f - _e2450);
            let _e2464 = ((_e1977.x * _e2451) + (_e2156.x * _e2450));
            let _e2465 = ((_e1977.y * _e2451) + (_e2156.y * _e2450));
            let _e2466 = ((_e1977.z * _e2451) + (_e2156.z * _e2450));
            let _e2474 = local_53;
            let _e2475 = (1f - _e2474);
            let _e2480 = local_54;
            let _e2483 = local_55;
            let _e2486 = local_56;
            out_color = vec4<f32>((((_e2464 * _e2475) + (((_e2464 * 1.5f) + 0.1f) * _e2480)) * _e625), (((_e2465 * _e2475) + (((_e2465 * 1.5f) + 0.1f) * _e2483)) * _e625), (((_e2466 * _e2475) + (((_e2466 * 1.5f) + 0.1f) * _e2486)) * _e625), _e638);
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
